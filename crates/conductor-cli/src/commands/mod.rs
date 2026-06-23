//! The three verb handlers + their shared persist / render / exit-code helpers.

mod coverage;
mod preflight;
mod report;
mod run;
mod suite;

pub use coverage::coverage;
pub use preflight::preflight;
pub use report::report;
pub use run::run;
pub use suite::suite;

use std::path::Path;
use std::process::ExitCode;

use conductor_core::{Lamp, RunRecord};
use conductor_report::{JournalWriter, RunReport, RunsDb};

/// Persist a run's records across the three artifacts: the JSONL journal (append), the `runs.db`
/// index (one row per scenario), and a single Markdown report for the run.
fn persist(runs_dir: &Path, run_id: &str, records: &[RunRecord]) -> anyhow::Result<()> {
    let mut journal = JournalWriter::create(runs_dir, run_id)?;
    let db = RunsDb::open(runs_dir)?;
    for record in records {
        journal.append(record)?;
        db.insert(record)?;
    }
    RunReport::write(runs_dir, run_id, records)?;
    Ok(())
}

/// Print one status line — the verdict-first lamp prefix (tty-gated color over it) + the scenario name.
fn print_record(record: &RunRecord) {
    println!("{}", crate::render::status_line(record));
}

/// Exit non-zero only on a hard `Fail`; every other reported state exits 0 (test-plan §1).
fn exit_code(records: &[RunRecord]) -> ExitCode {
    if records.iter().any(|r| Lamp::for_record(r) == Lamp::Fail) {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
