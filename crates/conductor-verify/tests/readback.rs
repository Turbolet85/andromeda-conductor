//! In-process integration test: a duplex-stream rmcp stub stands in for the Pulse MCP server, so the
//! read-back client's negotiation + tool surface is exercised without a live Pulse or a child process
//! (the real `TokioChildProcess` spawn is the preflight/boot chunk's integration). Per
//! `.claude/rules/testing.md`: mock Pulse with an rmcp stub; never fake Pulse's reaction as a verdict.

use std::sync::Arc;

use rmcp::ErrorData as McpError;
use rmcp::ServerHandler;
use rmcp::ServiceExt;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, Content, Implementation, ListToolsResult,
    PaginatedRequestParams, ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::{RequestContext, RoleServer};

use conductor_verify::{QUERY_INCIDENT_LIST, ReadbackClient};

/// A minimal Pulse stand-in: pins protocol `2024-11-05`, advertises one read-back tool, and echoes
/// tool calls back as a success result.
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
        let tool = Tool::new(QUERY_INCIDENT_LIST, "stub incident listing", schema);
        Ok(ListToolsResult::with_all_items(vec![tool]))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(format!(
            "called {}",
            request.name
        ))]))
    }
}

#[tokio::test(flavor = "current_thread")]
async fn negotiates_down_to_2024_11_05_and_round_trips_the_tool_surface() {
    let (client_io, server_io) = tokio::io::duplex(4096);

    let server = tokio::spawn(async move {
        let running = StubPulse.serve(server_io).await.expect("stub server init");
        let _ = running.waiting().await;
    });

    let client = ReadbackClient::connect_transport(client_io)
        .await
        .expect("client connects to the stub");

    // Negotiation landed on Pulse's hand-rolled version, NOT the rmcp LATEST default.
    assert_eq!(
        client.negotiated_protocol_version(),
        Some(&ProtocolVersion::V_2024_11_05)
    );

    // The typed surface lists the advertised tool...
    let tools = client.list_tools().await.expect("list tools");
    assert!(tools.iter().any(|t| t.name == QUERY_INCIDENT_LIST));

    // ...and a call round-trips a non-error result.
    let result = client
        .query_incident_list(None)
        .await
        .expect("call query_incident_list");
    assert_eq!(result.is_error, Some(false));

    drop(client);
    server.abort();
}
