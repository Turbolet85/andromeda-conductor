//! `conductor suite [--filter] [--seed]` — run the catalog over one runtime + one preflight gate.
//!
//! The first Ctrl-C stops the suite gracefully: the running scenario finishes, every finished record
//! is persisted and reported, and the suite exits 130. A second Ctrl-C ends the process at once.

use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use conductor_core::RunRecord;
use conductor_run as pipeline;

use crate::paths::Paths;
use crate::pause::CliResolver;
use crate::render;

use super::exit_code;

/// The conventional exit status of a process ended by SIGINT (128 + 2).
const STOPPED_EXIT: u8 = 130;

pub async fn suite(
    filter: Option<&str>,
    seed: Option<u64>,
    paths: &Paths,
    run_id: &str,
    agent_mode: bool,
) -> anyhow::Result<ExitCode> {
    let scenarios = paths.load_all_scenarios(filter, seed)?;
    if scenarios.is_empty() {
        anyhow::bail!("no scenarios found to run");
    }

    let envelope = pipeline::classify_run(&paths.load_envelope()?, &scenarios);
    let preflight = pipeline::preflight(&paths.manifest_path).await?;
    let stop = install_stop_handler().await;
    let progress = render::spinner(scenarios.len());
    let resolver = CliResolver::select(Some(progress.clone()), agent_mode);
    let outcome = pipeline::drive_run(
        &preflight,
        &scenarios,
        run_id,
        &paths.runs_dir,
        &envelope,
        &resolver,
        |done| progress.set_position(done as u64),
        || stop.load(Ordering::SeqCst),
    )
    .await?;
    if outcome.stopped {
        // Stops in place at the count it reached — never cleared, never driven to its length.
        progress.abandon();
    } else {
        progress.finish_and_clear();
    }

    if let Some(caption) = render::envelope_caption(&envelope) {
        println!("{caption}");
    }
    println!("{}", render::results_table(&outcome.records));
    if outcome.stopped {
        eprintln!(
            "{}",
            render::suite_stopped_line(outcome.records.len(), scenarios.len())
        );
    }
    Ok(suite_exit(&outcome.records, outcome.stopped))
}

/// A stopped suite exits 130 whatever its kept records say; a completed one on the hard-Fail rule.
fn suite_exit(records: &[RunRecord], stopped: bool) -> ExitCode {
    if stopped {
        ExitCode::from(STOPPED_EXIT)
    } else {
        exit_code(records)
    }
}

/// Install the suite's Ctrl-C handler and return the stop flag it sets. The first press sets the
/// flag and says so on stderr; a second press ends the process with the SIGINT status.
///
/// `ctrl_c` registers its OS handler on first poll, so the yield lets the spawned task take that
/// poll before the caller proceeds — until then a Ctrl-C would still kill the process outright.
async fn install_stop_handler() -> Arc<AtomicBool> {
    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_err() {
            return;
        }
        flag.store(true, Ordering::SeqCst);
        eprintln!("{}", render::STOP_REQUESTED);
        if tokio::signal::ctrl_c().await.is_ok() {
            std::process::exit(i32::from(STOPPED_EXIT));
        }
    });
    tokio::task::yield_now().await;
    stop
}

#[cfg(test)]
mod tests {
    use super::*;
    use conductor_core::{PId, ReportState, SloTier, Verdict};

    fn failed() -> RunRecord {
        RunRecord::measured(
            "2026-10-12T00-00-00-abc",
            424242,
            "boom",
            vec![PId("P-009".to_string())],
            Verdict::Fail,
            ReportState::Fail,
            "2026-10-12T00:00:00Z",
            "2026-10-12T00:00:01Z",
            1000,
            SloTier::Tier5s,
            Vec::new(),
        )
    }

    #[test]
    fn a_stopped_suite_exits_130_whatever_its_records_say() {
        assert_eq!(suite_exit(&[], true), ExitCode::from(130));
        assert_eq!(suite_exit(&[failed()], true), ExitCode::from(130));
    }

    #[test]
    fn a_completed_suite_exits_on_the_hard_fail_rule_alone() {
        assert_eq!(suite_exit(&[], false), ExitCode::SUCCESS);
        assert_eq!(suite_exit(&[failed()], false), ExitCode::FAILURE);
    }

    /// The first SIGINT sets the stop flag and leaves the process running. The signal is sent by a
    /// fixed-argv `kill` spawn — no operator value, no shell.
    #[cfg(unix)]
    #[tokio::test(flavor = "current_thread")]
    async fn the_first_interrupt_sets_the_stop_flag_and_the_process_lives() {
        let stop = install_stop_handler().await;
        assert!(!stop.load(Ordering::SeqCst), "nothing has been pressed yet");

        let status = std::process::Command::new("kill")
            .args(["-INT", &std::process::id().to_string()])
            .status()
            .expect("the fixed kill program spawns");
        assert!(status.success(), "kill delivered the signal: {status}");

        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            while !stop.load(Ordering::SeqCst) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("the first Ctrl-C sets the stop flag within the bound");
        // Reaching this line is the liveness half: the default SIGINT disposition would have ended
        // the test process before the flag could be read.
    }
}
