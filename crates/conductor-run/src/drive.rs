//! The live-counter run driver both shells stream progress from.
//!
//! [`drive_run`] walks the scenarios, emits a [`RunEvent`] per stage and polls the caller's abort
//! flag between them, so the GUI's Tauri `Channel` and the CLI's spinner observe the same sequence.

use std::path::Path;

use serde::Serialize;

use conductor_core::{
    EnvelopeStatus, PauseResolver, ReportState, RunRecord, Scenario,
};



use crate::canary::Preflight;
use crate::envelope::persist;
use crate::execute::execute_scenario;

/// A live run-progress event streamed to the GUI (the Tauri `Channel` payload). `count` is the number
/// of scenarios completed so far — the live counter the titlebar reads; the faithful per-signal
/// emission count is the Epoch-10 bridge (a Blocked run emits no signals, so it ticks only per
/// scenario).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct RunEvent {
    /// The progress stage this event reports.
    pub stage: RunStage,
    /// Scenarios completed so far in this run.
    pub count: u64,
}

/// The stage a [`RunEvent`] reports. Serializes to its lowercase name; the GUI maps `progress` → live,
/// the terminal stages (`blocked`/`done`) → idle, and `aborted` → aborted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStage {
    /// A scenario completed (or the run just started, `count = 0`).
    Progress,
    /// Terminal — every record blocked (no live Pulse).
    Blocked,
    /// Terminal — at least one record was measured.
    Done,
    /// Terminal — the operator stopped the run.
    Aborted,
}

/// Drive `scenarios` to their [`RunRecord`]s through `resolver`, persisting the run once and
/// streaming a [`RunEvent`] per scenario through `emit`; `should_abort` is polled between scenarios
/// (the GUI's stop button). The terminal stage is `Blocked` when every record blocked (the no-live-
/// Pulse spine), else `Done`. Generic over the resolver (`PauseResolver::resolve` returns
/// `impl Future`, so never `dyn`) — the GUI shell passes its `TauriResolver` to gate the
/// operator-checklist holds on the live-Pulse path; the agent/test path passes a
/// [`conductor_core::HeadlessResolver`] that never blocks. The faithful per-emission counter is the
/// Epoch-10 bridge; `count` here ticks per scenario.
#[allow(clippy::too_many_arguments)] // one parameter per distinct run input; bundling them would
// hide the run-level envelope standing behind a struct both shells would have to construct anyway
pub async fn drive_run<R, E, A>(
    pf: &Preflight,
    scenarios: &[Scenario],
    run_id: &str,
    runs_dir: &Path,
    envelope: &EnvelopeStatus,
    resolver: &R,
    mut emit: E,
    should_abort: A,
) -> anyhow::Result<Vec<RunRecord>>
where
    R: PauseResolver,
    E: FnMut(RunEvent),
    A: Fn() -> bool,
{
    emit(RunEvent { stage: RunStage::Progress, count: 0 });
    let mut records = Vec::with_capacity(scenarios.len());
    let mut checks = Vec::new();
    let mut aborted = false;
    for scenario in scenarios {
        if should_abort() {
            aborted = true;
            break;
        }
        let outcome = execute_scenario(pf, scenario, run_id, resolver).await?;
        records.push(outcome.record);
        checks.extend(outcome.checks);
        emit(RunEvent { stage: RunStage::Progress, count: records.len() as u64 });
    }
    // Polled again after the last scenario: the loop-head check alone cannot see a stop pressed DURING
    // the final scenario, so such a run settled `Done` and the GUI's announced abort was overwritten
    // by `idle` (NVDA pass 2026-09-02, S3-05).
    if !aborted && should_abort() {
        aborted = true;
    }
    persist(runs_dir, run_id, &records, &checks, envelope)?;
    let stage = if aborted {
        tracing::info!(run_id, count = records.len(), "run aborted by the operator");
        RunStage::Aborted
    } else if records.iter().all(|r| matches!(r.state, ReportState::Blocked)) {
        RunStage::Blocked
    } else {
        RunStage::Done
    };
    emit(RunEvent { stage, count: records.len() as u64 });
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::*;
    use conductor_core::HeadlessResolver;

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_streams_progress_then_blocked_and_persists() {
        let dir = assert_fs::TempDir::new().unwrap();
        let mut events = Vec::new();
        let records = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-drive",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |ev| events.push(ev),
            || false,
        )
        .await
        .expect("drive_run is infallible on the blocked path");

        assert_eq!(records.len(), 1);
        assert!(matches!(records[0].state, ReportState::Blocked));
        assert_eq!(events.first().unwrap().stage, RunStage::Progress);
        assert_eq!(events.last().unwrap().stage, RunStage::Blocked);
        assert_eq!(events.last().unwrap().count, 1);
        assert!(
            dir.path().join("run-drive.jsonl").is_file(),
            "the JSONL journal was persisted"
        );
        let journal = std::fs::read_to_string(dir.path().join("run-drive.jsonl")).unwrap();
        assert!(journal.contains("\"Blocked\""), "the journal carries the Blocked envelope");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_honors_abort_before_the_first_scenario() {
        let dir = assert_fs::TempDir::new().unwrap();
        let mut events = Vec::new();
        let records = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-abort",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |ev| events.push(ev),
            || true,
        )
        .await
        .unwrap();
        assert!(records.is_empty(), "an immediate abort runs no scenario");
        assert_eq!(events.last().unwrap().stage, RunStage::Aborted);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_reports_an_abort_raised_during_the_last_scenario() {
        let dir = assert_fs::TempDir::new().unwrap();
        // Stop is pressed while the FINAL scenario is executing, so the flag is only set once that
        // scenario has completed — the loop-head poll can never observe it.
        let stop = std::cell::Cell::new(false);
        let mut events = Vec::new();
        let records = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-abort-last",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |ev| {
                if ev.stage == RunStage::Progress && ev.count == 1 {
                    stop.set(true);
                }
                events.push(ev);
            },
            || stop.get(),
        )
        .await
        .unwrap();

        assert_eq!(records.len(), 1, "the scenario already running still completes");
        assert_eq!(
            events.last().unwrap().stage,
            RunStage::Aborted,
            "a stop during the last scenario reports Aborted, never Done/Blocked"
        );
        assert_eq!(events.last().unwrap().count, 1);
        let journal = std::fs::read_to_string(dir.path().join("run-abort-last.jsonl")).unwrap();
        assert!(
            journal.contains("\"Blocked\""),
            "the completed scenario's envelope is persisted before the abort is reported"
        );
    }
}
