//! In-process preflight-gate tests: a configurable hand-rolled JSON-RPC duplex stub stands in for
//! Pulse, each variant exercising one readiness leg (ready · wrong version · missing tool · empty
//! canary · canary call-error). Per `.claude/rules/testing.md`: mock Pulse over a stub; never fake
//! its reaction as a verdict. The real child-spawn path is `tests/preflight_spawn.rs`.

mod common;

use common::{StubConfig, serve_stub};
use conductor_core::ReportState;
use conductor_verify::{
    CanaryMarker, CanaryOutcome, CanaryPoll, ContractManifest, MARK_INCIDENT_RESOLVED,
    QUERY_INCIDENT_LIST, READBACK_TOOLS, RETRIEVE_REPORT, RETRIEVE_TELEMETRY_SLICE, ReadbackClient,
    ReadyState, ToolPresence, run_preflight,
};

fn manifest() -> ContractManifest {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/mcp-contract.toml");
    ContractManifest::load(&path).expect("pinned manifest loads")
}

async fn drive_with(config: StubConfig, manifest: ContractManifest) -> ReadyState {
    let (client_io, server_io) = tokio::io::duplex(4096);
    let canary = CanaryMarker::new(config.canary.clone(), config.canary_fingerprint.clone());
    let server = tokio::spawn(serve_stub(server_io, config));
    let client = ReadbackClient::connect_transport(client_io).await.expect("client connects");
    let ready = run_preflight(&client, &manifest, &canary, "/test/data-dir", CanaryPoll::immediate())
        .await
        .expect("preflight runs");
    drop(client);
    server.abort();
    ready
}

async fn drive(config: StubConfig) -> ReadyState {
    drive_with(config, manifest()).await
}

#[tokio::test(flavor = "current_thread")]
async fn healthy_pulse_is_ready() {
    let ready = drive(StubConfig::default()).await;
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
    // expects a version the (healthy) stub does not report must block.
    let manifest = ContractManifest {
        expected_protocol_version: "9999-12-31".to_string(),
        required_tools: READBACK_TOOLS.iter().map(|s| s.to_string()).collect(),
    };
    let ready = drive_with(StubConfig::default(), manifest).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("protocol version mismatch"), "{precondition}");
    assert!(precondition.contains("9999-12-31"), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn an_absent_required_tool_is_blocked() {
    let config = StubConfig {
        // drop mark_incident_resolved
        tools: vec![
            QUERY_INCIDENT_LIST.to_string(),
            RETRIEVE_REPORT.to_string(),
            RETRIEVE_TELEMETRY_SLICE.to_string(),
        ],
        ..StubConfig::default()
    };
    let ready = drive(config).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    assert_eq!(ready.required_tools[MARK_INCIDENT_RESOLVED], ToolPresence::Absent);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("absent"), "{precondition}");
    assert!(precondition.contains(MARK_INCIDENT_RESOLVED), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn ready_state_serializes_to_the_readiness_envelope() {
    let ready = drive(StubConfig::default()).await;
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
    let config = StubConfig { canary_in_corpus: false, ..StubConfig::default() };
    let ready = drive(config).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Failed);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("incident not found"), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn a_canary_call_error_is_blocked_distinctly_from_an_empty_corpus() {
    // The masked-error fix: a genuine query_incident_list error must surface its OWN precondition,
    // NOT the misleading "incident not found in corpus".
    let config = StubConfig { query_errors: true, ..StubConfig::default() };
    let ready = drive(config).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Failed);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("MCP read-back call failed"), "{precondition}");
    assert!(!precondition.contains("incident not found"), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn a_fingerprint_mismatch_is_blocked() {
    // An incident is present, but its telemetry slice carries a different fingerprint than the bridge
    // emitted — fidelity fails (no false pass on a stale / foreign incident; titles are scrubbed, so
    // only the fingerprint proves the round-trip).
    let (client_io, server_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(serve_stub(server_io, StubConfig::default()));
    let client = ReadbackClient::connect_transport(client_io).await.expect("client connects");
    let canary = CanaryMarker::new("ConductorCanary_x", "deadbeefdeadbeef");
    let ready = run_preflight(&client, &manifest(), &canary, "/test/data-dir", CanaryPoll::immediate())
        .await
        .expect("preflight runs");
    drop(client);
    server.abort();

    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Failed);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("fingerprint not found"), "{precondition}");
}
