//! Pins the committed over-envelope fixture the webview `--e2e` arm's load-envelope banner is
//! seeded with, and seeds it.
//!
//! The subject's CONTENT is two committed artifacts — `tests/fixtures/over-envelope.toml` (the
//! breaching scenario) and `tests/fixtures/lamps-journal.jsonl` (the run's records, shared with
//! `lamps_fixture.rs`) — never a row hand-written into `wdio.conf.ts` or a committed binary
//! `runs.db`. The row itself is produced by [`conductor_run::persist`], the SAME production writer
//! both shells call, so nothing here hand-populates the database (test-plan §7).
//!
//! It asserts the fixture's SEMANTICS, not merely that it parses: a scenario edit that leaves the
//! TOML readable but no longer BREACHING fails HERE, loudly, instead of leaving the e2e green over a
//! fixture that stopped testing anything.
//!
//! The banner and the coverage lamps both read `latest_run_id`, so the subject is ONE run carrying
//! both: the lamps journal's records, persisted under the lamps run-id with an `EnvironmentSuspect`
//! envelope standing.

use std::path::{Path, PathBuf};

use conductor_core::{
    CheckRecord, ComparisonKind, EnvelopeStatus, LoadEnvelope, ReportState, Scenario, Verdict,
    read_run_journal,
};

/// The run-id the seed writes, shared with `lamps_fixture.rs` and `wdio.conf.ts`. One run carries
/// both subjects because `run_report` and `run_envelope` both default to the latest run.
const FIXTURE_RUN_ID: &str = "lamps-fixture";

/// Set by `wdio.conf.ts` to the repo-relative fixture runs dir. Unset — every ordinary
/// `cargo nextest` / `cargo test` run — the seeding test is a no-op, so the suite never writes
/// into the repository.
const SEED_DIR_ENV: &str = "CONDUCTOR_E2E_SEED_DIR";

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn fixture_scenario() -> Scenario {
    let toml = std::fs::read_to_string(format!(
        "{}/tests/fixtures/over-envelope.toml",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("the committed over-envelope fixture is readable");
    Scenario::from_toml_str(&toml).expect("the fixture parses and passes its load-path validation")
}

fn pinned_envelope() -> LoadEnvelope {
    LoadEnvelope::load(&workspace_root().join(LoadEnvelope::default_path()))
        .expect("the pinned SUT load envelope loads from its fixed path")
}

/// Classify the committed fixture against the committed envelope — the exact pair the seed persists.
fn classified() -> EnvelopeStatus {
    conductor_run::classify_run(
        &pinned_envelope(),
        std::slice::from_ref(&fixture_scenario()),
    )
}

/// The graded check behind the subject's passing scenario row.
///
/// Seeded so the subject carries BOTH registered journal shapes and a `run_check` row: without it
/// the journal is envelope-only, so the conformance gate's discrimination arm and the `cleanup`
/// count-zero verification would each assert over a table that was empty to begin with.
fn fixture_check() -> CheckRecord {
    CheckRecord {
        run_id: FIXTURE_RUN_ID.to_string(),
        scenario: "lamps-fixture-pass".to_string(),
        check_index: 0,
        kind: ComparisonKind::Contains,
        verdict: Verdict::Pass,
        state: ReportState::Pass,
        latency_ms: 2000,
        deadline_ms: 5000,
        budget_ms: None,
    }
}

#[test]
fn the_fixture_breaches_the_pinned_envelope() {
    let status = classified();
    assert!(
        status.is_suspect(),
        "the banner has no subject unless this fixture breaches: {status:?}"
    );
    assert_eq!(
        status.label(),
        "ENVIRONMENT-SUSPECT",
        "the label the banner renders"
    );

    let cause = status
        .cause()
        .expect("a suspect standing carries its cause");
    assert!(
        cause.contains("over-envelope") && cause.contains("burst"),
        "the cause names the breaching scenario and phase: {cause}"
    );
    assert!(
        cause.contains("faster than"),
        "the breach is on the sustained RATE term, not the storm window — a storm breach would need \
         a phase longer than ten minutes and would make the fixture expensive: {cause}"
    );
}

#[test]
fn the_breach_survives_neither_a_smaller_burst_nor_a_longer_window() {
    // phase_rate_exceeds is `occurrences * 1000 > max_rate * gap_ms`, so at the pinned 10000 the
    // condition is `occurrences > 10 * gap_ms`. The committed fixture clears it by ONE dispatch —
    // pin both directions so an edit that widens the window or trims the burst fails here.
    let envelope = pinned_envelope();
    let scenario = fixture_scenario();
    let phase = scenario
        .phases
        .first()
        .expect("the fixture declares its breaching phase");

    assert!(
        u64::from(phase.emission.occurrences) * 1_000
            > envelope.envelope.max_sustained_rate_spans_per_s * phase.gap_ms,
        "the committed shape must breach the rate term"
    );
    assert!(
        u64::from(phase.emission.occurrences - 1) * 1_000
            <= envelope.envelope.max_sustained_rate_spans_per_s * phase.gap_ms,
        "one dispatch fewer must NOT breach — otherwise the fixture is over-sized and stops pinning \
         the boundary it is supposed to sit on"
    );
    assert!(
        phase.gap_ms <= envelope.envelope.max_sustained_storm_ms,
        "the fixture must breach the RATE term only; a storm breach would cost ten minutes per run"
    );
}

#[test]
fn the_persisted_standing_round_trips_through_the_production_reader() {
    let dir = assert_fs::TempDir::new().unwrap();
    let records = seed_into(dir.path());

    assert!(
        !records.is_empty(),
        "the seed persists the lamps journal's records"
    );
    let standing = conductor_run::read_envelope(dir.path(), FIXTURE_RUN_ID)
        .expect("the envelope row reads back")
        .expect("the seeded run recorded an envelope row");
    assert!(
        standing.is_suspect(),
        "what persist wrote is what read_envelope returns — the path run_envelope single-sources"
    );

    let reread = read_run_journal(dir.path(), FIXTURE_RUN_ID)
        .expect("the persisted journal parses through the production reader");
    assert_eq!(
        reread.len(),
        records.len(),
        "the seed preserves the lamps subject the coverage rows are asserted on"
    );
}

/// Seed one runs dir with the fixture subject, through the production writer. Returns the records
/// persisted. Shared by the round-trip test and the `--e2e` seeding entry point below.
fn seed_into(runs_dir: &Path) -> Vec<conductor_core::RunRecord> {
    let journal = format!(
        "{}/tests/fixtures/lamps-journal.jsonl",
        env!("CARGO_MANIFEST_DIR")
    );
    let staged = assert_fs::TempDir::new().unwrap();
    std::fs::copy(
        &journal,
        staged.path().join(format!("{FIXTURE_RUN_ID}.jsonl")),
    )
    .expect("the committed lamps journal stages for reading");
    let records = read_run_journal(staged.path(), FIXTURE_RUN_ID)
        .expect("the committed lamps journal parses through the production reader");

    conductor_run::persist(
        runs_dir,
        FIXTURE_RUN_ID,
        &records,
        &[fixture_check()],
        &classified(),
    )
    .expect("the production writer persists the journal, the runs.db rows and the report");
    records
}

/// The `--e2e` seeding entry point. A NO-OP unless `CONDUCTOR_E2E_SEED_DIR` names the target dir, so
/// an ordinary suite run never writes into the repository; `wdio.conf.ts` sets it in `onPrepare`.
#[test]
fn seed_the_e2e_fixture_runs_dir() {
    let Ok(target) = std::env::var(SEED_DIR_ENV) else {
        return;
    };
    let dir = workspace_root().join(&target);
    std::fs::create_dir_all(&dir).expect("the fixture runs dir is creatable");
    let records = seed_into(&dir);
    println!(
        "seeded {} records + an ENVIRONMENT-SUSPECT envelope row into {target}",
        records.len()
    );
}
