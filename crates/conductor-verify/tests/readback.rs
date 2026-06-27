//! In-process integration test: a hand-rolled JSON-RPC duplex stub stands in for the Pulse MCP
//! server, so the read-back client's handshake + tool surface is exercised against Pulse's RAW
//! result shapes without a live Pulse or a child process (the real spawn is `tests/preflight_spawn.rs`).
//! Per `.claude/rules/testing.md`: mock Pulse over a stub; never fake Pulse's reaction as a verdict.

mod common;

use common::{StubConfig, serve_stub};
use conductor_verify::{QUERY_INCIDENT_LIST, ReadbackClient};

#[tokio::test(flavor = "current_thread")]
async fn negotiates_down_to_2024_11_05_and_round_trips_the_tool_surface() {
    let (client_io, server_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(serve_stub(server_io, StubConfig::default()));

    let client = ReadbackClient::connect_transport(client_io)
        .await
        .expect("client connects to the stub");

    // Negotiation landed on Pulse's hand-rolled version, read from the `initialize` result.
    assert_eq!(client.negotiated_protocol_version(), Some("2024-11-05"));

    // The tool surface lists the advertised tool by name...
    let tools = client.list_tools().await.expect("list tools");
    assert!(tools.iter().any(|t| t == QUERY_INCIDENT_LIST));

    // ...and a call round-trips Pulse's RAW result shape (an object with `items`), NOT an MCP
    // `CallToolResult` envelope — the regression that would have caught `UnexpectedResponse`.
    let result = client
        .query_incident_list(None)
        .await
        .expect("call query_incident_list");
    assert!(result.get("items").is_some(), "raw query_incident_list shape: {result}");
    assert!(result.get("content").is_none(), "must NOT be MCP-wrapped: {result}");

    drop(client);
    server.abort();
}
