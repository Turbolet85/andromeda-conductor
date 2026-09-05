//! The run's persisted artifacts and its load-envelope standing.
//!
//! [`persist`] writes the three artifacts one run produces — the JSONL journal, the `runs.db` rows
//! and the Markdown report — and `report.generate` / `db.insert_run` stay run-scoped SIBLINGS of
//! `scenario.run`, correlated by `run_id` rather than nested beneath it (obs-plan §4).

use std::path::Path;


use conductor_core::{
    CheckRecord, EnvelopeStatus,
    LoadEnvelope, RunRecord, Scenario,
};
use conductor_report::{JournalWriter, RunReport, RunsDb};




/// Persist a run's records across the three artifacts: the JSONL journal (append), the `runs.db`
/// index (one row per scenario, plus one row per graded check), and a single Markdown report for the
/// run. Shared by both shells so the CLI and the GUI write an identical envelope for the same
/// scenario+seed (test-plan Path 7).
///
/// `checks` carries the per-check grain behind the rows; it is empty when nothing was graded (a
/// blocked run, or an all-declare-only suite).
pub fn persist(
    runs_dir: &Path,
    run_id: &str,
    records: &[RunRecord],
    checks: &[CheckRecord],
    envelope: &EnvelopeStatus,
) -> anyhow::Result<()> {
    let mut journal = JournalWriter::create(runs_dir, run_id)?;
    let db = RunsDb::open(runs_dir)?;
    for record in records {
        journal.append(record)?;
        db.insert(record)?;
    }
    for check in checks {
        journal.append_check(check)?;
        db.insert_check(check)?;
    }
    db.insert_envelope(run_id, envelope)?;
    RunReport::write(runs_dir, run_id, records, checks, envelope)?;
    Ok(())
}

/// Read back a run's load-envelope standing — the run-level qualifier the desktop run-report renders.
///
/// The read-side counterpart to [`persist`]'s `insert_envelope`. It lives here, not in the GUI shell,
/// because `conductor-tauri` has no `conductor-report` edge and the composition root already owns the
/// `RunsDb` seam (arch §Established Decisions [Module Boundaries]). A run that recorded no envelope row
/// yields `None` — an absence, never an `Err` (arch §Conventions — Error handling).
pub fn read_envelope(runs_dir: &Path, run_id: &str) -> anyhow::Result<Option<EnvelopeStatus>> {
    let db = RunsDb::open(runs_dir)?;
    let status = db.get_envelope(run_id)?;
    tracing::info!(
        run_id = %run_id,
        message = status.as_ref().map_or("no envelope row", |s| s.label()),
        "read run envelope"
    );
    Ok(status)
}

/// Judge a run against the pinned SUT load envelope, before it is driven.
///
/// A run is environment-suspect if ANY of its scenarios breaches the envelope: the qualifier is
/// about whether the run could be evidence at all, and one over-envelope scenario is enough to
/// stall the SUT's append path for the rest of it. Scenarios are judged in catalog order, so the
/// reported cause is deterministic.
///
/// This is a VALUE on every path — never an `Err`. Driving the SUT too hard is an outcome about the
/// run, not a Conductor fault (arch §Cross-cutting Patterns "Verdict/error wall").
pub fn classify_run(envelope: &LoadEnvelope, scenarios: &[Scenario]) -> EnvelopeStatus {
    scenarios
        .iter()
        .map(|s| envelope.classify(s))
        .find(EnvelopeStatus::is_suspect)
        .unwrap_or(EnvelopeStatus::InEnvelope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::*;
    use crate::drive::drive_run;
    use conductor_core::{HeadlessResolver, ReportState};

    #[test]
    fn read_envelope_round_trips_what_persist_wrote_and_is_none_for_an_unknown_run() {
        let dir = assert_fs::TempDir::new().unwrap();
        let suspect = EnvelopeStatus::EnvironmentSuspect("storm-scenario over the sustained bound".to_string());

        persist(dir.path(), "run-suspect", &[], &[], &suspect).unwrap();
        persist(dir.path(), "run-clean", &[], &[], &EnvelopeStatus::InEnvelope).unwrap();

        assert_eq!(read_envelope(dir.path(), "run-suspect").unwrap(), Some(suspect));
        assert_eq!(
            read_envelope(dir.path(), "run-clean").unwrap(),
            Some(EnvelopeStatus::InEnvelope),
            "an in-envelope run records a standing too — the banner's absence is the RENDERER's choice"
        );
        assert_eq!(
            read_envelope(dir.path(), "never-ran").unwrap(),
            None,
            "a run with no envelope row is an absence, never an Err"
        );
    }

    #[test]
    fn classify_run_flags_the_first_breaching_scenario_deterministically() {
        let envelope = test_envelope(1_000, &[("idle-long", "deliberate quiet")]);

        assert_eq!(
            classify_run(&envelope, &[named_fixture("a", 500), named_fixture("b", 900)]),
            EnvelopeStatus::InEnvelope
        );
        assert_eq!(
            classify_run(&envelope, &[named_fixture("idle-long", 9_000)]),
            EnvelopeStatus::InEnvelope,
            "a pinned exemption keeps the run in-envelope"
        );
        assert_eq!(classify_run(&envelope, &[]), EnvelopeStatus::InEnvelope);

        let suite = [named_fixture("ok", 500), named_fixture("first-breach", 5_000), named_fixture("second-breach", 9_000)];
        let status = classify_run(&envelope, &suite);
        let cause = status.cause().expect("a breaching suite names its cause");
        assert!(cause.contains("first-breach"), "catalog order decides the reported cause: {cause}");
        assert!(!cause.contains("second-breach"), "only the first breach is reported: {cause}");
    }

    /// `v2-07` — an over-envelope run is classified environment-suspect, distinct from `Fail`, and
    /// the classification round-trips through all three artifacts with the breach named as the cause.
    #[tokio::test(flavor = "current_thread")]
    async fn an_over_envelope_run_is_environment_suspect_not_fail() {
        let dir = assert_fs::TempDir::new().unwrap();
        let scenario = named_fixture("over-envelope-fixture", 900_000);
        let envelope = classify_run(&test_envelope(600_000, &[]), std::slice::from_ref(&scenario));
        assert!(envelope.is_suspect(), "the fixture must breach for this test to mean anything");

        let records = drive_run(
            &blocked_preflight(),
            std::slice::from_ref(&scenario),
            "run-envelope",
            dir.path(),
            &envelope,
            &HeadlessResolver::proceed(),
            |_| {},
            || false,
        )
        .await
        .expect("an envelope breach is never a harness Err");

        // distinct from Fail: the qualifier never rewrites a check state
        assert!(records.iter().all(|r| r.state != ReportState::Fail), "no row became a Fail");
        assert!(records.iter().all(|r| r.state == ReportState::Blocked), "states are unchanged");

        // the runs.db round-trip, read through a bound-parameter query
        let db = RunsDb::open(dir.path()).expect("runs.db opens");
        assert_eq!(db.get_envelope("run-envelope").unwrap(), Some(envelope.clone()));

        // the Markdown report names the breach as the cause
        let md = std::fs::read_to_string(dir.path().join("run-envelope.md")).unwrap();
        assert!(md.contains("[ENVIRONMENT-SUSPECT]"), "{md}");
        assert!(md.contains("over-envelope-fixture"), "the report names the breaching scenario: {md}");
        assert!(!md.contains("[FAIL]"), "an envelope breach never renders as Fail: {md}");

        // the JSONL journal is untouched by the run-level qualifier (eleven per-check fields)
        let journal = std::fs::read_to_string(dir.path().join("run-envelope.jsonl")).unwrap();
        assert!(journal.contains("\"Blocked\""), "the journal keeps its own envelope: {journal}");
        assert!(
            !journal.contains("ENVIRONMENT-SUSPECT"),
            "the run-level qualifier does not leak into the per-check journal: {journal}"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn an_in_envelope_run_records_its_standing_too() {
        let dir = assert_fs::TempDir::new().unwrap();
        drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-in-env",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |_| {},
            || false,
        )
        .await
        .unwrap();

        let db = RunsDb::open(dir.path()).unwrap();
        assert_eq!(
            db.get_envelope("run-in-env").unwrap(),
            Some(EnvelopeStatus::InEnvelope),
            "every run records a standing, so an absent row means a bug, not an in-envelope run"
        );
        let md = std::fs::read_to_string(dir.path().join("run-in-env.md")).unwrap();
        assert!(!md.contains("ENVIRONMENT-SUSPECT"), "{md}");
    }
}
