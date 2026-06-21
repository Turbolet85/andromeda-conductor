//! The MCP read-back client — the `conductor-verify` transport foundation.
//!
//! [`ReadbackClient`] spawns the Pulse MCP sidecar over stdio (hardened, see [`crate::spawn`]),
//! negotiates the protocol version *down* to Pulse's hand-rolled `2024-11-05` (architecture
//! §Established Decisions [MCP Read-Back Client]), and exposes a typed surface over the four consumed
//! read-back tools. The preflight gate, the data-dir canary, and verdict classification build on this
//! handle in later Epoch-5 chunks.

use std::path::PathBuf;

use rmcp::ServiceExt;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, ClientCapabilities, ClientInfo, Implementation,
    JsonObject, ProtocolVersion, Tool,
};
use rmcp::service::{RoleClient, RunningService};
use rmcp::transport::{IntoTransport, TokioChildProcess};

use crate::error::VerifyError;
use crate::spawn;

/// The four Pulse read-back tools Conductor consumes (architecture §Occupied Resources).
pub const QUERY_INCIDENT_LIST: &str = "query_incident_list";
pub const RETRIEVE_REPORT: &str = "retrieve_report";
pub const RETRIEVE_TELEMETRY_SLICE: &str = "retrieve_telemetry_slice";
pub const MARK_INCIDENT_RESOLVED: &str = "mark_incident_resolved";

/// A live MCP read-back session against a Pulse sidecar.
pub struct ReadbackClient {
    service: RunningService<RoleClient, ClientInfo>,
}

impl ReadbackClient {
    /// The client identity + protocol pin sent in the `initialize` handshake — pinned to
    /// `2024-11-05` so negotiation lands on Pulse's hand-rolled server version rather than the rmcp
    /// default (`LATEST`), the silent mismatch the preflight gate exists to prevent.
    fn client_info() -> ClientInfo {
        ClientInfo::new(
            ClientCapabilities::default(),
            Implementation::new("conductor", env!("CARGO_PKG_VERSION")),
        )
        .with_protocol_version(ProtocolVersion::V_2024_11_05)
    }

    /// Spawn the fixed-path Pulse sidecar (data-dir via `.env` after metacharacter rejection) and
    /// establish the read-back session. `data_dir` falls back to the platform default when `None`.
    #[tracing::instrument(name = "verify.readback.connect", skip_all, err)]
    pub async fn connect(data_dir: Option<PathBuf>) -> Result<Self, VerifyError> {
        let dir = spawn::resolve_data_dir(data_dir)?;
        let command = spawn::build_command(&dir);
        let transport = TokioChildProcess::new(command).map_err(VerifyError::Spawn)?;
        Self::connect_transport(transport).await
    }

    /// Establish the read-back session over an arbitrary transport — the seam tests inject an
    /// in-process duplex stub here, and the preflight/boot chunk drives the real child process.
    #[tracing::instrument(name = "verify.readback.connect_transport", skip_all, err)]
    pub async fn connect_transport<T, E, A>(transport: T) -> Result<Self, VerifyError>
    where
        T: IntoTransport<RoleClient, E, A>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let service = Self::client_info()
            .serve(transport)
            .await
            .map_err(|e| VerifyError::Initialize(Box::new(e)))?;
        Ok(Self { service })
    }

    /// The protocol version negotiated with the server — the value the preflight gate asserts is
    /// `2024-11-05`. `None` before the handshake result is available.
    pub fn negotiated_protocol_version(&self) -> Option<&ProtocolVersion> {
        self.service.peer_info().map(|info| &info.protocol_version)
    }

    /// List every tool the server advertises (paginating to completion).
    #[tracing::instrument(name = "verify.readback.list_tools", skip_all, err)]
    pub async fn list_tools(&self) -> Result<Vec<Tool>, VerifyError> {
        self.service.list_all_tools().await.map_err(|e| VerifyError::Call(Box::new(e)))
    }

    /// Call a read-back tool by name with optional JSON arguments, returning the raw result for a
    /// later verdict chunk to interpret.
    #[tracing::instrument(name = "verify.readback.call_tool", skip(self, arguments), fields(mcp_tool = %name))]
    pub async fn call_tool(
        &self,
        name: &str,
        arguments: Option<JsonObject>,
    ) -> Result<CallToolResult, VerifyError> {
        let mut params = CallToolRequestParams::new(name.to_owned());
        if let Some(arguments) = arguments {
            params = params.with_arguments(arguments);
        }
        self.service.call_tool(params).await.map_err(|e| VerifyError::Call(Box::new(e)))
    }

    /// `query_incident_list` — the shared-corpus incident listing the canary + most scenarios read.
    pub async fn query_incident_list(
        &self,
        arguments: Option<JsonObject>,
    ) -> Result<CallToolResult, VerifyError> {
        self.call_tool(QUERY_INCIDENT_LIST, arguments).await
    }

    /// `retrieve_report` — a per-incident report (may carry `degraded_mode`, a later known-residual).
    pub async fn retrieve_report(
        &self,
        arguments: Option<JsonObject>,
    ) -> Result<CallToolResult, VerifyError> {
        self.call_tool(RETRIEVE_REPORT, arguments).await
    }

    /// `retrieve_telemetry_slice` — the raw telemetry window behind an incident.
    pub async fn retrieve_telemetry_slice(
        &self,
        arguments: Option<JsonObject>,
    ) -> Result<CallToolResult, VerifyError> {
        self.call_tool(RETRIEVE_TELEMETRY_SLICE, arguments).await
    }

    /// `mark_incident_resolved` — the lifecycle write used by resolution scenarios.
    pub async fn mark_incident_resolved(
        &self,
        arguments: Option<JsonObject>,
    ) -> Result<CallToolResult, VerifyError> {
        self.call_tool(MARK_INCIDENT_RESOLVED, arguments).await
    }
}
