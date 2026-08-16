//! In-process preflight-gate tests: a configurable hand-rolled JSON-RPC duplex stub stands in for
//! Pulse, each variant exercising one readiness leg (ready · wrong version · missing tool · empty
//! canary · canary call-error). Per `.claude/rules/testing.md`: mock Pulse over a stub; never fake
//! its reaction as a verdict. The real child-spawn path is `tests/preflight_spawn.rs`.

mod common;

use common::{StubConfig, serve_stub};
use conductor_core::{ReportState, RunContractStatus, UnmetTerm, redact_value};
use conductor_verify::{
    CanaryMarker, CanaryOutcome, CanaryPoll, ContractManifest, MARK_INCIDENT_RESOLVED,
    QUERY_INCIDENT_LIST, READBACK_TOOLS, RETRIEVE_REPORT, RETRIEVE_TELEMETRY_SLICE, ReadbackClient,
    ReadyState, ToolPresence, run_preflight,
};

/// The canary's emission instant for the stub legs. The default stub reports `i64::MAX`, so every
/// pre-existing leg sees a FRESH incident and the freshness carrier never masks the leg under test.
const CANARY_EMITTED_AT: i64 = 1_700_000_000_000_000_000;

/// A contract whose terms are all satisfied — the shape every pre-existing leg assumes, so the
/// run-contract arm never masks the leg under test.
fn satisfied() -> RunContractStatus {
    RunContractStatus::satisfied()
}

/// A contract with one unmet shell-declaration term, mirroring the committed `l4-deterministic`.
fn unmet_l4() -> RunContractStatus {
    RunContractStatus::from_unmet(vec![UnmetTerm {
        id: "l4-deterministic".to_string(),
        statement: "the launched pulse-app must run with ANDROMEDA_PULSE_L4_DETERMINISTIC=true"
            .to_string(),
        causes: "either the launching shell never declared it, or pulse-app was started from a \
                 different environment than Conductor's"
            .to_string(),
    }])
}

fn manifest() -> ContractManifest {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/mcp-contract.toml");
    ContractManifest::load(&path).expect("pinned manifest loads")
}

async fn drive_with(
    config: StubConfig,
    manifest: ContractManifest,
    contract: RunContractStatus,
) -> ReadyState {
    let (client_io, server_io) = tokio::io::duplex(4096);
    let canary =
        CanaryMarker::new(config.canary.clone(), config.canary_fingerprint.clone(), CANARY_EMITTED_AT);
    let server = tokio::spawn(serve_stub(server_io, config));
    let client = ReadbackClient::connect_transport(client_io).await.expect("client connects");
    let ready = run_preflight(
        &client,
        &manifest,
        &contract,
        &canary,
        "/test/data-dir",
        CanaryPoll::immediate(),
    )
    .await
    .expect("preflight runs");
    drop(client);
    server.abort();
    ready
}

async fn drive(config: StubConfig) -> ReadyState {
    drive_with(config, manifest(), satisfied()).await
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
    let ready = drive_with(StubConfig::default(), manifest, satisfied()).await;
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
async fn an_empty_canary_names_the_workspace_key_precondition() {
    // A corpus that returns zero incidents is byte-identical on the wire to an app/sidecar
    // workspace-key divergence, which returns zero rows forever — so the precondition names the key
    // agreement AND the no-incident cause, replacing the opaque "incident not found in corpus".
    let config = StubConfig { canary_in_corpus: false, ..StubConfig::default() };
    let ready = drive(config).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Failed);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("same incident workspace key"), "{precondition}");
    assert!(precondition.contains("ANDROMEDA_PULSE_DATA_DIR"), "{precondition}");
    assert!(precondition.contains("raised no incident"), "{precondition}");
    assert!(!precondition.contains("incident not found in corpus"), "{precondition}");
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
    assert!(!precondition.contains("workspace key"), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn a_corpus_of_only_older_incidents_is_blocked() {
    // Incidents ARE present, but every one predates the canary's emission — read-back would be grading
    // residue, so the gate blocks rather than passing on a stale/foreign incident.
    let config = StubConfig { opened_at_unix_nano: Some(CANARY_EMITTED_AT - 1), ..Default::default() };
    let ready = drive_with(config, manifest(), satisfied()).await;

    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Failed);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("predates"), "{precondition}");
    // The two not-found causes must stay distinguishable: incidents ARE present here, so this is a
    // staleness failure, never the workspace-key/no-incident precondition.
    assert!(!precondition.contains("workspace key"), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn an_incident_missing_its_open_stamp_is_blocked() {
    // The readers degrade to empty on shape, so an absent stamp must read as NOT-fresh. Reading it as
    // satisfied would be the false-green this carrier exists to remove.
    let config = StubConfig { opened_at_unix_nano: None, ..Default::default() };
    let ready = drive_with(config, manifest(), satisfied()).await;

    assert!(!ready.ready);
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Failed);
    assert!(ready.blocked_precondition.expect("a precondition").contains("predates"));
}

#[tokio::test(flavor = "current_thread")]
async fn an_unmet_run_contract_term_is_blocked_and_named_individually() {
    // The launch condition is reported as a condition to satisfy with its candidate causes — never
    // as a measurement of pulse-app, which Conductor does not launch and cannot inspect.
    let ready = drive_with(StubConfig::default(), manifest(), unmet_l4()).await;
    assert!(!ready.ready);
    assert_eq!(ready.report_state(), ReportState::Blocked);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(precondition.contains("unmet run-contract terms"), "{precondition}");
    assert!(precondition.contains("[l4-deterministic]"), "the term is named: {precondition}");
    assert!(precondition.contains("ANDROMEDA_PULSE_L4_DETERMINISTIC"), "{precondition}");
    assert!(precondition.contains("cannot inspect") || precondition.contains("environment"),
        "the candidate causes ride with the condition: {precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn an_unmet_term_skips_the_canary_rather_than_reporting_its_symptom() {
    // An unmet launch condition explains a failed canary, so the gate must not spend the poll budget
    // and then surface the downstream symptom — the failure mode the workspace-key probe hit.
    let ready = drive_with(StubConfig::default(), manifest(), unmet_l4()).await;
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Skipped);
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert!(!precondition.contains("workspace key"), "{precondition}");
    assert!(!precondition.contains("predates"), "{precondition}");
}

#[tokio::test(flavor = "current_thread")]
async fn a_satisfied_contract_leaves_the_other_legs_deciding() {
    // The new arm must not mask the legs beneath it: with every term satisfied the gate reaches the
    // canary exactly as before.
    let ready = drive_with(StubConfig::default(), manifest(), satisfied()).await;
    assert!(ready.ready, "blocked: {:?}", ready.blocked_precondition);
    assert_eq!(ready.canary_round_trip, CanaryOutcome::Ok);
}

#[tokio::test(flavor = "current_thread")]
async fn a_blocked_precondition_carries_no_absolute_host_path() {
    // The invariant is the value scrub, not a `::`-token filter: `redact_value` masks absolute
    // host-FILE paths, so a precondition it leaves byte-identical carries none (obs-plan §11
    // Anti-Patterns → Logs, PII Scrubbing).
    for config in [
        StubConfig { canary_in_corpus: false, ..StubConfig::default() },
        StubConfig { query_errors: true, ..StubConfig::default() },
    ] {
        let ready = drive(config).await;
        let precondition = ready.blocked_precondition.expect("a precondition");
        assert_eq!(redact_value(&precondition).as_ref(), precondition.as_str(), "{precondition}");
        assert!(!precondition.contains("/test/data-dir"), "{precondition}");
    }

    // The run-contract arm is held to the same invariant: its terms are operator-facing prose and
    // must never carry a host path into the readiness envelope.
    let ready = drive_with(StubConfig::default(), manifest(), unmet_l4()).await;
    let precondition = ready.blocked_precondition.expect("a precondition");
    assert_eq!(redact_value(&precondition).as_ref(), precondition.as_str(), "{precondition}");
    assert!(!precondition.contains("/test/data-dir"), "{precondition}");
}
