//! In-process preflight-gate tests: a configurable rmcp stub stands in for Pulse over a duplex
//! stream, and each variant exercises one readiness leg (ready · wrong version · missing tool ·
//! empty canary). Per `.claude/rules/testing.md`: mock Pulse with an rmcp stub; never fake Pulse's
//! reaction as a verdict. The real `TokioChildProcess` child-spawn path is `tests/preflight_spawn.rs`.

use std::sync::Arc;

use rmcp::ErrorData as McpError;
use rmcp::ServerHandler;
use rmcp::ServiceExt;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, Content, Implementation, ListToolsResult,
    PaginatedRequestParams, ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::{RequestContext, RoleServer};

use conductor_core::ReportState;
use conductor_verify::{
    CanaryMarker, CanaryOutcome, ContractManifest, MARK_INCIDENT_RESOLVED, QUERY_INCIDENT_LIST,
    READBACK_TOOLS, ReadbackClient, ReadyState, RETRIEVE_REPORT, RETRIEVE_TELEMETRY_SLICE,
    ToolPresence, run_preflight,
};

const CANARY: &str = "conductor-canary-7f3a";

/// A configurable Pulse stand-in: it reports `version`, advertises `tools`, and `query_incident_list`
/// returns the canary incident only when `canary_in_corpus`.
#[derive(Clone)]
struct StubPulse {
    version: ProtocolVersion,
    tools: Vec<&'static str>,
    canary_in_corpus: bool,
}

impl StubPulse {
    fn healthy() -> Self {
        Self {
            version: ProtocolVersion::V_2024_11_05,
            tools: READBACK_TOOLS.to_vec(),
            canary_in_corpus: true,
        }
    }
}

impl ServerHandler for StubPulse {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(self.version.clone())
            .with_server_info(Implementation::new("stub-pulse", "0.0.0"))
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let schema = Arc::new(serde_json::Map::new());
        let tools = self.tools.iter().map(|n| Tool::new(*n, "stub", schema.clone())).collect();
        Ok(ListToolsResult::with_all_items(tools))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        if request.name == QUERY_INCIDENT_LIST {
            let body = if self.canary_in_corpus {
                format!("[{{\"incident\":\"{CANARY}\"}}]")
            } else {
                "[]".to_string()
            };
            Ok(CallToolResult::success(vec![Content::text(body)]))
        } else {
            Ok(CallToolResult::success(vec![Content::text("ok")]))
        }
    }
}

fn manifest() -> ContractManifest {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/mcp-contract.toml");
    ContractManifest::load(&path).expect("pinned manifest loads")
}

async fn drive_with(stub: StubPulse, manifest: ContractManifest) -> ReadyState {
    let (client_io, server_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(async move {
        let running = stub.serve(server_io).await.expect("stub server init");
        let _ = running.waiting().await;
    });
    let client = ReadbackClient::connect_transport(client_io).await.expect("client connects");
    let ready = run_preflight(&client, &manifest, &CanaryMarker::new(CANARY), "/test/data-dir")
        .await
        .expect("preflight runs");
    drop(client);
    server.abort();
    ready
}

async fn drive(stub: StubPulse) -> ReadyState {
    drive_with(stub, manifest()).await
}

#[tokio::test(flavor = "current_thread")]
async fn healthy_pulse_is_ready() {
    let ready = drive(StubPulse::healthy()).await;
    assert!(ready.ready, "blocked: {:?}", ready.blocked_precondition);
    assert_eq!(ready.report_state(), ReportState::Pass);
    assert_eq!(ready.negotiated_protocol_version.as_deref(), Some("2024-11-05"));
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Ok);
    assert!(ready.required_tools.values().all(|p| *p == ToolPresence::Present));
    assert!(ready.blocked_precondition.is_none());
}

#[tokio::test(flavor = "current_thread")]
async fn wrong_protocol_version_is_blocked() {
    // The gate compares the negotiated version against the manifest's pinned value; a manifest that
    // expects a version the (healthy) server does not negotiate must block. (rmcp negotiates a newer
    // client request DOWN to the server's version, so the mismatch is driven from the manifest side.)
    let manifest = ContractManifest {
        expected_protocol_version: "9999-12-31".to_string(),
        required_tools: READBACK_TOOLS.iter().map(|s| s.to_string()).collect(),
    };
    let ready = drive_with(StubPulse::healthy(), manifest).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("protocol version mismatch"), "{precondition}");
    assert!(precondition.contains("9999-12-31"), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn an_absent_required_tool_is_blocked() {
    let mut stub = StubPulse::healthy();
    stub.tools = vec![QUERY_INCIDENT_LIST, RETRIEVE_REPORT, RETRIEVE_TELEMETRY_SLICE]; // drop mark_incident_resolved
    let ready = drive(stub).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    assert_eq!(ready.required_tools[MARK_INCIDENT_RESOLVED], ToolPresence::Absent);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("absent"), "{precondition}");
    assert!(precondition.contains(MARK_INCIDENT_RESOLVED), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn ready_state_serializes_to_the_readiness_envelope() {
    let ready = drive(StubPulse::healthy()).await;
    let value = serde_json::to_value(&ready).expect("serialize ReadyState");
    let obj = value.as_object().expect("readiness envelope is an object");
    for key in [
        "ready",
        "negotiated_protocol_version",
        "expected_protocol_version",
        "required_tools",
        "data_dir",
        "canary_round_trip",
        "blocked_precondition",
        "checked_at",
    ] {
        assert!(obj.contains_key(key), "readiness envelope missing `{key}`");
    }
    assert_eq!(obj["ready"], serde_json::json!(true));
    assert_eq!(obj["canary_round_trip"], serde_json::json!("ok"));
    assert_eq!(obj["required_tools"][QUERY_INCIDENT_LIST], serde_json::json!("present"));
    assert_eq!(obj["blocked_precondition"], serde_json::Value::Null);
}

#[tokio::test(flavor = "current_thread")]
async fn an_empty_canary_is_blocked() {
    let mut stub = StubPulse::healthy();
    stub.canary_in_corpus = false;
    let ready = drive(stub).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Failed);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("canary"), "{precondition}");
}
