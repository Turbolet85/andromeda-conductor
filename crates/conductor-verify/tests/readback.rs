//! In-process integration test: a hand-rolled JSON-RPC duplex stub stands in for the Pulse MCP
//! server, so the read-back client's handshake + tool surface is exercised against Pulse's RAW
//! result shapes without a live Pulse or a child process (the real spawn is `tests/preflight_spawn.rs`).
//! Per `.claude/rules/testing.md`: mock Pulse over a stub; never fake Pulse's reaction as a verdict.

mod common;

use common::{StubConfig, serve_stub};
use conductor_core::ComparisonKind;
use conductor_verify::{
    QUERY_INCIDENT_LIST, ReadBackOutcome, ReadbackClient, observe,
};

/// Drive one read-back pass against a configured stub.
async fn observe_with(config: StubConfig) -> ReadBackOutcome {
    let (client_io, server_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(serve_stub(server_io, config));
    let client = ReadbackClient::connect_transport(client_io)
        .await
        .expect("client connects to the stub");
    let outcome = observe(&client).await;
    drop(client);
    server.abort();
    outcome
}

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

#[tokio::test(flavor = "current_thread")]
async fn observation_composes_list_fields_and_report_markdown() {
    let outcome = observe_with(StubConfig::default()).await;
    let ReadBackOutcome::Observed(o) = outcome else {
        panic!("a populated corpus observes: {outcome:?}");
    };

    // The list item's own fields and the report body both reach the text a Contains check grades.
    assert!(o.text.contains("active"), "list status: {}", o.text);
    assert!(o.text.contains("RetryStorm"), "report markdown: {}", o.text);
    assert!(!o.degraded);
    assert_eq!(o.fingerprints, vec!["0123456789abcdef".to_string()]);
}

#[tokio::test(flavor = "current_thread")]
async fn count_at_least_grades_the_evidence_count_not_the_text() {
    let config = StubConfig { span_ref_count: 12, ..StubConfig::default() };
    let ReadBackOutcome::Observed(o) = observe_with(config).await else {
        panic!("a populated corpus observes");
    };

    // The regression this guards: handing the composed markdown to a CountAtLeast check makes
    // `compare` fail to parse it, softening every sample floor to the calibration region forever.
    let counted = o.observed_for(ComparisonKind::CountAtLeast);
    assert_eq!(counted.trim().parse::<i64>().unwrap(), 12);
    assert!(o.observed_for(ComparisonKind::Contains).parse::<i64>().is_err());
}

#[tokio::test(flavor = "current_thread")]
async fn degraded_report_is_observed_as_the_pre_accepted_residual() {
    let config = StubConfig { report_degraded: true, ..StubConfig::default() };
    let ReadBackOutcome::Observed(o) = observe_with(config).await else {
        panic!("a populated corpus observes");
    };
    assert!(o.degraded, "retrieve_report reported degraded_mode: true");
}

#[tokio::test(flavor = "current_thread")]
async fn empty_corpus_is_never_a_gradable_observation() {
    // An Absent check against an empty observation would pass trivially — the false pass-as-empty
    // the read-back gate exists to prevent (security-plan §Anti-Patterns → Input).
    let config = StubConfig { canary_in_corpus: false, ..StubConfig::default() };
    assert_eq!(observe_with(config).await, ReadBackOutcome::EmptyCorpus);
}

#[tokio::test(flavor = "current_thread")]
async fn a_call_error_is_a_typed_value_never_a_panic() {
    let config = StubConfig { query_errors: true, ..StubConfig::default() };
    let outcome = observe_with(config).await;
    let ReadBackOutcome::CallFailed(reason) = outcome else {
        panic!("a JSON-RPC error surfaces as CallFailed: {outcome:?}");
    };
    assert!(reason.contains("corpus unavailable"), "redacted server reason: {reason}");
}

#[tokio::test(flavor = "current_thread")]
async fn a_malformed_result_shape_degrades_to_empty_rather_than_panicking() {
    let config = StubConfig { malformed_results: true, ..StubConfig::default() };
    // Every tool returns a well-formed JSON value of the wrong shape: the incident list has no
    // `items`, so there is nothing to grade and the pass is Blocked-bound, not a false pass.
    assert_eq!(observe_with(config).await, ReadBackOutcome::EmptyCorpus);
}
