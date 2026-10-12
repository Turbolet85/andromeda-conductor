//! The suite driver `conductor suite` runs its catalogue through.
//!
//! [`drive_run`] walks the scenarios, reports progress after each and polls the caller's stop flag
//! between them, so a stopped run keeps — and persists — every scenario that finished.

use std::path::Path;

use conductor_core::{EnvelopeStatus, PauseResolver, RunRecord, Scenario};

use crate::canary::Preflight;
use crate::envelope::persist;
use crate::execute::execute_scenario;

/// What [`drive_run`] returns: the records of every scenario that completed, and whether the run was
/// stopped before the catalogue was exhausted (or during its last scenario).
#[derive(Debug)]
pub struct DriveOutcome {
    /// One record per completed scenario, in catalogue order.
    pub records: Vec<RunRecord>,
    /// `true` when `should_abort` was observed — the run ended on a stop, not on the last scenario.
    pub stopped: bool,
}

/// Drive `scenarios` to their [`RunRecord`]s through `resolver`, persisting the run once. After each
/// completed scenario `progress` receives the number completed so far; `should_abort` is polled
/// before each scenario and once after the last, so a scenario already running always completes and
/// a stop never discards a finished record. Generic over the resolver (`PauseResolver::resolve`
/// returns `impl Future`, so never `dyn`) — the CLI passes its interactive-or-headless resolver; the
/// test path passes a [`conductor_core::HeadlessResolver`] that never blocks.
#[allow(clippy::too_many_arguments)] // one parameter per distinct run input; bundling them would
// hide the run-level envelope standing behind a struct every caller would have to construct anyway
pub async fn drive_run<R, P, A>(
    pf: &Preflight,
    scenarios: &[Scenario],
    run_id: &str,
    runs_dir: &Path,
    envelope: &EnvelopeStatus,
    resolver: &R,
    mut progress: P,
    should_abort: A,
) -> anyhow::Result<DriveOutcome>
where
    R: PauseResolver,
    P: FnMut(usize),
    A: Fn() -> bool,
{
    let mut records = Vec::with_capacity(scenarios.len());
    let mut checks = Vec::new();
    let mut stopped = false;
    for scenario in scenarios {
        if should_abort() {
            stopped = true;
            break;
        }
        let outcome = execute_scenario(pf, scenario, run_id, resolver).await?;
        records.push(outcome.record);
        checks.extend(outcome.checks);
        progress(records.len());
    }
    // Polled again after the last scenario: the loop-head check alone cannot see a stop raised
    // DURING the final scenario, so such a run would report a completed suite.
    if !stopped && should_abort() {
        stopped = true;
    }
    persist(runs_dir, run_id, &records, &checks, envelope)?;
    if stopped {
        tracing::info!(run_id, count = records.len(), "run aborted by the operator");
    }
    Ok(DriveOutcome { records, stopped })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::*;
    use conductor_core::{HeadlessResolver, ReportState};

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_ticks_progress_and_persists_the_blocked_spine() {
        let dir = assert_fs::TempDir::new().unwrap();
        let mut ticks = Vec::new();
        let outcome = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-drive",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |done| ticks.push(done),
            || false,
        )
        .await
        .expect("drive_run is infallible on the blocked path");

        assert_eq!(outcome.records.len(), 1);
        assert!(matches!(outcome.records[0].state, ReportState::Blocked));
        assert!(!outcome.stopped, "an unstopped run is not reported stopped");
        assert_eq!(ticks, vec![1], "progress ticks once per completed scenario");
        let journal = std::fs::read_to_string(dir.path().join("run-drive.jsonl"))
            .expect("the JSONL journal was persisted");
        assert!(
            journal.contains("\"Blocked\""),
            "the journal carries the Blocked envelope"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_honors_abort_before_the_first_scenario() {
        let dir = assert_fs::TempDir::new().unwrap();
        let mut ticks = Vec::new();
        let outcome = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-abort",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |done| ticks.push(done),
            || true,
        )
        .await
        .unwrap();
        assert!(
            outcome.records.is_empty(),
            "an immediate abort runs no scenario"
        );
        assert!(outcome.stopped);
        assert!(
            ticks.is_empty(),
            "no scenario completed, so progress never ticked"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_reports_an_abort_raised_during_the_last_scenario() {
        let dir = assert_fs::TempDir::new().unwrap();
        // Stop is raised while the FINAL scenario is executing, so the flag is only set once that
        // scenario has completed — the loop-head poll can never observe it.
        let stop = std::cell::Cell::new(false);
        let outcome = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-abort-last",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |done| {
                if done == 1 {
                    stop.set(true);
                }
            },
            || stop.get(),
        )
        .await
        .unwrap();

        assert_eq!(
            outcome.records.len(),
            1,
            "the scenario already running still completes"
        );
        assert!(
            outcome.stopped,
            "a stop during the last scenario reports stopped, never a completed suite"
        );
        let journal = std::fs::read_to_string(dir.path().join("run-abort-last.jsonl")).unwrap();
        assert!(
            journal.contains("\"Blocked\""),
            "the completed scenario's envelope is persisted before the stop is reported"
        );
    }
}
