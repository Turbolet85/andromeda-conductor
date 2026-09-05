//! The composition root's PUBLIC surface, asserted from outside the crate.
//!
//! `conductor-run` was split into sibling modules along its named seams; every item stays reachable
//! at its original `conductor_run::` path through the crate root's re-exports. Both bins and the
//! other test binaries call these paths, and `architecture.md` §Standard Contracts cites
//! `conductor_run::observe_preconditions` by path in prose — so a module move that silently renamed
//! one would falsify a spec claim no compiler error would surface. An integration test sees only
//! what a downstream crate sees, which is exactly the surface under assertion.

use std::path::Path;

use conductor_core::{LoadEnvelope, PId, PhaseSpec, Scenario, SloTier};

/// Every public item, referenced by its crate-root path. A `pub use` that dropped or renamed one
/// stops this compiling — the split's byte-stability oracle, checked by the type system rather than
/// by a listing that can drift.
#[test]
fn public_api_paths_are_stable() {
    // Functions — named as values, so the reference is the signature, not a call.
    let _: fn(u64, u64) -> u64 = conductor_run::canary_storm_seed;
    let _: fn(u64, u32) -> u64 = conductor_run::canary_warmup_seed;
    let _: fn(&str) -> conductor_emit::ExceptionSpec = conductor_run::canary_spec;
    let _: fn(&[i64]) -> Option<(i64, i64)> = conductor_run::select_resolve_target;
    let _: fn(&conductor_run::LifecycleObservation, i64) -> conductor_run::LifecycleVerdict =
        conductor_run::evaluate_lifecycle;
    let _: fn(&LoadEnvelope, &[Scenario]) -> conductor_core::EnvelopeStatus =
        conductor_run::classify_run;
    let _: fn(&Path, &str) -> anyhow::Result<Option<conductor_core::EnvelopeStatus>> =
        conductor_run::read_envelope;

    // Consts.
    let _: u64 = conductor_run::CANARY_STORM_COUNT;
    let _: &str = conductor_run::CANARY_SERVICE_NAME;
    let _: f64 = conductor_run::AUTO_RESOLVE_IDLE_SECONDS;

    // Types the bins name.
    let _: Option<conductor_run::Preflight> = None;
    let _: Option<conductor_run::ScenarioOutcome> = None;
    let _: Option<conductor_run::Dispatcher> = None;
    let _: Option<conductor_run::DispatchError> = None;
    let _: Option<conductor_run::LifecycleObservation> = None;
    let _: Option<conductor_run::LifecycleVerdict> = None;
    let _: Option<conductor_run::RunEvent> = None;
    let _: conductor_run::RunStage = conductor_run::RunStage::Aborted;

    // The async entry points, referenced through a never-awaited binding so the paths are checked
    // without driving a runtime.
    #[allow(clippy::no_effect_underscore_binding)]
    let _unused = || async {
        let _ = conductor_run::preflight(Path::new("contracts/mcp-contract.toml")).await;
        let _ = conductor_run::readiness(Path::new("contracts/mcp-contract.toml")).await;
        let _ = conductor_run::observe_preconditions().await;
    };
}

/// `persist` and `attribute_by_liveness` carry the two remaining public signatures, exercised rather
/// than merely named: an envelope written through the production writer reads back through
/// `read_envelope`, which is the path the a11y fixtures and both shells depend on.
#[test]
fn persist_and_read_envelope_round_trip_through_the_public_paths() {
    let dir = assert_fs::TempDir::new().unwrap();
    let runs = dir.path().join("runs");
    std::fs::create_dir_all(&runs).unwrap();

    let scenario = Scenario {
        name: "composition-root".to_string(),
        p_ids: vec![PId("P-009".to_string())],
        seed: 424_242,
        slo_tier: SloTier::Tier5s,
        phases: vec![PhaseSpec {
            name: "quiet".to_string(),
            gap_ms: 10,
            emission: conductor_core::EmissionSpec::default(),
            fault: None,
        }],
        jitter_ms: 0,
        expected: Vec::new(),
        checklist: Vec::new(),
    };
    let record = conductor_core::RunRecord::blocked(
        "2026-09-05T00-00-00-abc",
        424_242,
        &scenario.name,
        scenario.p_ids.clone(),
        scenario.slo_tier,
    );
    let envelope = conductor_core::EnvelopeStatus::InEnvelope;

    conductor_run::persist(&runs, "2026-09-05T00-00-00-abc", &[record], &[], &envelope)
        .expect("the production writer persists a run");

    let read = conductor_run::read_envelope(&runs, "2026-09-05T00-00-00-abc")
        .expect("read_envelope reads what persist wrote");
    assert_eq!(
        read,
        Some(envelope),
        "the standing round-trips through runs.db"
    );
    assert_eq!(
        conductor_run::read_envelope(&runs, "no-such-run").expect("an unknown run is not an error"),
        None
    );
}
