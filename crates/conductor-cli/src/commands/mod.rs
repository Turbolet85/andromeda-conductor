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

use std::process::ExitCode;

use conductor_core::{Lamp, RunRecord};
use conductor_run::persist;

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
