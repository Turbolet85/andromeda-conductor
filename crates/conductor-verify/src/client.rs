//! The MCP read-back client — the `conductor-verify` transport foundation.
//!
//! [`ReadbackClient`] spawns the Pulse MCP sidecar over stdio (hardened, see [`crate::spawn`]) and
//! speaks line-delimited JSON-RPC directly (see [`crate::jsonrpc`]), pinning the protocol version to
//! Pulse's hand-rolled `2024-11-05` in the `initialize` handshake. It exposes a typed surface over the
//! four consumed read-back tools whose calls return the RAW [`serde_json::Value`] result — because
//! Pulse returns un-enveloped tool payloads (no MCP `{content:[…]}`), which rmcp's typed client
//! rejects (architecture §Established Decisions [MCP Read-Back Client], reversed at the
//! 2026-06-27 read-back-result-shape-adapter chunk).

use std::path::PathBuf;
use std::process::Stdio;

use serde_json::{Value, json};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

use crate::error::VerifyError;
use crate::jsonrpc::JsonRpcSession;
use crate::spawn;

/// The four Pulse read-back tools Conductor consumes (architecture §Occupied Resources).
pub const QUERY_INCIDENT_LIST: &str = "query_incident_list";
pub const RETRIEVE_REPORT: &str = "retrieve_report";
pub const RETRIEVE_TELEMETRY_SLICE: &str = "retrieve_telemetry_slice";
pub const MARK_INCIDENT_RESOLVED: &str = "mark_incident_resolved";

/// The protocol version Conductor pins in the `initialize` handshake — Pulse's hand-rolled server
/// version. The preflight gate asserts the server negotiates this exact value (never the silent
/// mismatch a strict newer default would cause).
const CLIENT_PROTOCOL_VERSION: &str = "2024-11-05";

/// A live MCP read-back session against a Pulse sidecar (or an in-process stub in tests).
pub struct ReadbackClient {
    session: Mutex<JsonRpcSession>,
    negotiated_protocol_version: Option<String>,
    /// Kept alive so the spawned child is killed on drop (`kill_on_drop`); `None` for the in-process
    /// duplex transport used by the seam tests.
    _child: Option<Child>,
}

impl ReadbackClient {
    /// Spawn the fixed-path Pulse sidecar (data-dir via `.env` after metacharacter rejection) and
    /// establish the read-back session. `data_dir` falls back to the platform default when `None`.
    #[tracing::instrument(name = "verify.readback.connect", skip_all, err)]
    pub async fn connect(data_dir: Option<PathBuf>) -> Result<Self, VerifyError> {
        let dir = spawn::resolve_data_dir(data_dir)?;
        Self::connect_command(spawn::build_command(&dir)).await
    }

    /// Spawn an explicit command as the sidecar child (piped stdio, killed on drop) and establish the
    /// session. `connect` builds the hardened fixed-path command; the child-spawn test passes the stub
    /// binary directly.
    #[tracing::instrument(name = "verify.readback.connect_command", skip_all, err)]
    pub async fn connect_command(mut command: Command) -> Result<Self, VerifyError> {
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true);
        let mut child = command.spawn().map_err(VerifyError::Spawn)?;
        let stdin = child.stdin.take().ok_or_else(|| {
            VerifyError::Spawn(std::io::Error::other("sidecar stdin was not piped"))
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            VerifyError::Spawn(std::io::Error::other("sidecar stdout was not piped"))
        })?;
        let session = JsonRpcSession::new(Box::new(stdout), Box::new(stdin));
        Self::initialize(session, Some(child)).await
    }

    /// Establish the read-back session over an arbitrary transport — the seam tests inject an
    /// in-process duplex stub here; `connect`/`connect_command` drive the real child process.
    #[tracing::instrument(name = "verify.readback.connect_transport", skip_all, err)]
    pub async fn connect_transport<T>(transport: T) -> Result<Self, VerifyError>
    where
        T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
    {
        let (reader, writer) = tokio::io::split(transport);
        let session = JsonRpcSession::new(Box::new(reader), Box::new(writer));
        Self::initialize(session, None).await
    }

    /// Run the `initialize` handshake (pinning `2024-11-05`), capture the server's negotiated version,
    /// then send the `notifications/initialized` step. A transport/decode/JSON-RPC failure during init
    /// is a harness `Err`.
    async fn initialize(
        mut session: JsonRpcSession,
        child: Option<Child>,
    ) -> Result<Self, VerifyError> {
        let params = json!({
            "protocolVersion": CLIENT_PROTOCOL_VERSION,
            "capabilities": {},
            "clientInfo": { "name": "conductor", "version": env!("CARGO_PKG_VERSION") },
        });
        let result = session.request("initialize", params).await?;
        let negotiated_protocol_version = result
            .get("protocolVersion")
            .and_then(Value::as_str)
            .map(str::to_string);
        // Best-effort MCP handshake completion; Pulse tolerates its absence.
        let _ = session.notify("notifications/initialized", json!({})).await;
        Ok(Self {
            session: Mutex::new(session),
            negotiated_protocol_version,
            _child: child,
        })
    }

    /// The protocol version the server reported in `initialize` — the value the preflight gate asserts
    /// is `2024-11-05`. `None` if the server omitted it.
    pub fn negotiated_protocol_version(&self) -> Option<&str> {
        self.negotiated_protocol_version.as_deref()
    }

    /// The tool names the server advertises (`tools/list` → `result.tools[].name`).
    #[tracing::instrument(name = "verify.readback.list_tools", skip_all, err)]
    pub async fn list_tools(&self) -> Result<Vec<String>, VerifyError> {
        let result = self
            .session
            .lock()
            .await
            .request("tools/list", json!({}))
            .await?;
        let names = result
            .get("tools")
            .and_then(Value::as_array)
            .map(|tools| {
                tools
                    .iter()
                    .filter_map(|t| t.get("name").and_then(Value::as_str).map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        Ok(names)
    }

    /// Call a read-back tool by name, returning Pulse's RAW JSON-RPC result for the caller to read.
    #[tracing::instrument(name = "verify.readback.call_tool", skip(self, arguments), fields(mcp_tool = %name))]
    pub async fn call_tool(
        &self,
        name: &str,
        arguments: Option<Value>,
    ) -> Result<Value, VerifyError> {
        let params = json!({ "name": name, "arguments": arguments.unwrap_or_else(|| json!({})) });
        self.session
            .lock()
            .await
            .request("tools/call", params)
            .await
    }

    /// `query_incident_list` — the shared-corpus incident listing the canary + most scenarios read
    /// (raw shape `{items,total,next_cursor}`).
    pub async fn query_incident_list(
        &self,
        arguments: Option<Value>,
    ) -> Result<Value, VerifyError> {
        self.call_tool(QUERY_INCIDENT_LIST, arguments).await
    }

    /// `retrieve_report` — a per-incident report (raw `{markdown,degraded_mode}`).
    pub async fn retrieve_report(&self, arguments: Option<Value>) -> Result<Value, VerifyError> {
        self.call_tool(RETRIEVE_REPORT, arguments).await
    }

    /// `retrieve_telemetry_slice` — raw `{incident_id,span_refs,fingerprint_refs,timestamps_unix_nano}`.
    pub async fn retrieve_telemetry_slice(
        &self,
        arguments: Option<Value>,
    ) -> Result<Value, VerifyError> {
        self.call_tool(RETRIEVE_TELEMETRY_SLICE, arguments).await
    }

    /// `mark_incident_resolved` — the lifecycle write used by resolution scenarios (raw `{resolved,incident_id}`).
    pub async fn mark_incident_resolved(
        &self,
        arguments: Option<Value>,
    ) -> Result<Value, VerifyError> {
        self.call_tool(MARK_INCIDENT_RESOLVED, arguments).await
    }

    /// The lifecycle write by id — the tool's only argument shape (`IncidentIdArgs` on Pulse's side).
    ///
    /// Exists so a caller can drive the write without taking a `serde_json` dependency of its own to
    /// build the one-field object; the raw-`Value` form above stays for callers that already hold one.
    pub async fn resolve_incident(&self, incident_id: i64) -> Result<Value, VerifyError> {
        self.mark_incident_resolved(Some(json!({ "incident_id": incident_id })))
            .await
    }
}
