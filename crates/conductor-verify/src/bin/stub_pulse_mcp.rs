//! Test-support stub MCP server: protocol `2024-11-05`, the four read-back tools, and a canary
//! incident on `query_incident_list`, served over stdio. Spawned as a real child process by
//! `tests/preflight_spawn.rs` so the preflight gate's `TokioChildProcess` path is exercised end to
//! end. Built only under the `stub-server` feature — never part of a release build.

use std::sync::Arc;

use rmcp::ErrorData as McpError;
use rmcp::ServerHandler;
use rmcp::ServiceExt;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, Content, Implementation, ListToolsResult,
    PaginatedRequestParams, ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::{RequestContext, RoleServer};

use conductor_verify::{QUERY_INCIDENT_LIST, READBACK_TOOLS};

/// Must match `tests/preflight_spawn.rs`'s expected canary marker.
const CANARY: &str = "conductor-canary-7f3a";

#[derive(Clone)]
struct StubPulse;

impl ServerHandler for StubPulse {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2024_11_05)
            .with_server_info(Implementation::new("stub-pulse", "0.0.0"))
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let schema = Arc::new(serde_json::Map::new());
        let tools = READBACK_TOOLS.iter().map(|n| Tool::new(*n, "stub", schema.clone())).collect();
        Ok(ListToolsResult::with_all_items(tools))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let body = if request.name == QUERY_INCIDENT_LIST {
            format!("[{{\"incident\":\"{CANARY}\"}}]")
        } else {
            "ok".to_string()
        };
        Ok(CallToolResult::success(vec![Content::text(body)]))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let running = StubPulse.serve(rmcp::transport::stdio()).await.expect("stub server init");
    let _ = running.waiting().await;
}
