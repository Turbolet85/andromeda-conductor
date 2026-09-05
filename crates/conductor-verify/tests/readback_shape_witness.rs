//! The read-back key-set witness, alone in its own test binary.
//!
//! `conductor_core::init_observability` installs a PROCESS-GLOBAL, first-install-wins subscriber, so a
//! per-test temp sink does not isolate it and serializing the file would hide the shared state rather
//! than remove it (test-plan §11). One test per binary holds under both runners.
//!
//! `ShapeWitness::list` is one-shot, and its only effect is a single `tracing` line naming the tool's
//! observed key set (obs-plan §6) — no return value carries it, so the self-obs artifact is the only
//! witness. THREE answering attempts is the minimum that separates the three behaviours: recorded
//! once on the first answer (1 line), recorded on every answer but the first (2), never recorded (0).
//! At two attempts the first two are indistinguishable.

mod common;

use std::time::Duration;

use common::{StubConfig, bounded, serve_stub};
use conductor_core::{ObsSink, RunContractStatus, init_observability};
use conductor_verify::{CanaryMarker, CanaryPoll, ContractManifest, ReadbackClient, run_preflight};

/// The `run_id` this test stamps on its own self-obs stream — the discriminator the count filters on.
const WITNESS_RUN_ID: &str = "shape-witness";
const CANARY_EMITTED_AT: i64 = 1_700_000_000_000_000_000;
const ATTEMPTS: u32 = 3;

#[tokio::test(flavor = "current_thread")]
async fn the_read_back_key_set_witness_is_recorded_once_on_the_first_answering_attempt() {
    let dir = assert_fs::TempDir::new().expect("temp dir");
    let log = dir.path().join("obs.jsonl");
    // Deliberately no `RUST_LOG`: the subscriber's default directive is INFO and the witness rides
    // `info`, so this asserts the line an operator sees with no opt-in (obs-plan §6 / §3).
    init_observability("conductor", Some(WITNESS_RUN_ID.to_string()), ObsSink::File(log.clone()));

    // A corpus that ANSWERS every attempt and never goes fresh: the witness records on any answering
    // call, empty list included, so the poll spends its whole budget with three answers behind it.
    let config = StubConfig { canary_in_corpus: false, ..StubConfig::default() };
    let canary = CanaryMarker::new(
        config.canary.clone(),
        config.canary_fingerprint.clone(),
        CANARY_EMITTED_AT,
    );
    let (client_io, server_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(serve_stub(server_io, config));
    let client =
        bounded(ReadbackClient::connect_transport(client_io)).await.expect("client connects");

    let manifest_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/mcp-contract.toml");
    let manifest = ContractManifest::load(&manifest_path).expect("pinned manifest loads");

    let ready = bounded(run_preflight(
        &client,
        &manifest,
        &RunContractStatus::satisfied(),
        &canary,
        "/test/data-dir",
        CanaryPoll { attempts: ATTEMPTS, interval: Duration::ZERO },
    ))
    .await
    .expect("preflight runs");

    drop(client);
    server.abort();

    assert!(!ready.ready, "the corpus never goes fresh, so the gate must block");

    let contents = std::fs::read_to_string(&log).expect("self-obs artifact written");
    // Scoped by this test's own `run_id`, and by the witness's own message text rather than the tool
    // name alone — `query_incident_list` also rides the `mcp_tool` attribute of the call-tool span.
    let witness: Vec<&str> = contents
        .lines()
        .filter(|line| {
            line.contains(WITNESS_RUN_ID) && line.contains("query_incident_list returned keys")
        })
        .collect();

    assert_eq!(
        witness.len(),
        1,
        "the witness is one-shot across {ATTEMPTS} answering attempts — 0 means it never recorded, \
         {} means it records on every answer but the first:\n{contents}",
        ATTEMPTS - 1
    );

    let line = witness[0];
    for key in ["items", "next_cursor", "total"] {
        assert!(line.contains(key), "the witness names the observed key `{key}`: {line}");
    }
    for field in [
        "timestamp_ms",
        "level",
        "target",
        "service.name",
        "service.version",
        "deployment.environment",
        "run_id",
    ] {
        assert!(
            line.contains(&format!("\"{field}\"")),
            "the witness line is missing the self-obs base field {field}: {line}"
        );
    }
}
