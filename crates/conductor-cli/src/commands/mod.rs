//! The three verb handlers + their shared persist / render / exit-code helpers.

mod coverage;
mod preconditions;
mod preflight;
mod report;
mod run;
mod suite;

pub use coverage::coverage;
pub use preconditions::preconditions;
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

#[cfg(test)]
mod tests {
    use super::*;
    use conductor_core::{PId, ReportState, SloTier, Verdict};

    fn measured(scenario: &str, verdict: Verdict, state: ReportState) -> RunRecord {
        RunRecord::measured(
            "2026-09-05T00-00-00-abc",
            424242,
            scenario,
            vec![PId("P-009".to_string())],
            verdict,
            state,
            "2026-09-05T00:00:00Z",
            "2026-09-05T00:00:01Z",
            1000,
            SloTier::Tier5s,
            Vec::new(),
        )
    }

    /// Only a hard `Fail` exits non-zero — a Blocked / ManualCheck / KnownResidual row is a REPORTED
    /// state, and exiting on one would make an unreachable SUT indistinguishable from a real defect
    /// (test-plan §1; `.claude/rules/testing.md` §Quality gates). No live Pulse can produce a Fail
    /// through the binary, so the contract is asserted here over constructed records.
    #[test]
    fn only_a_hard_fail_exits_non_zero() {
        assert_eq!(
            exit_code(&[measured("boom", Verdict::Fail, ReportState::Fail)]),
            ExitCode::FAILURE
        );
        assert_eq!(
            exit_code(&[]),
            ExitCode::SUCCESS,
            "an empty run is not a failure"
        );
        for (verdict, state) in [
            (Verdict::Pass, ReportState::Pass),
            (Verdict::CalibrationRegion, ReportState::ManualCheck),
            (Verdict::Pass, ReportState::KnownResidual),
        ] {
            assert_eq!(
                exit_code(&[measured("ok", verdict, state)]),
                ExitCode::SUCCESS,
                "{verdict:?}/{state:?} is a reported state, never an exit"
            );
        }
        assert_eq!(
            exit_code(&[
                measured("ok", Verdict::Pass, ReportState::Pass),
                measured("boom", Verdict::Fail, ReportState::Fail),
            ]),
            ExitCode::FAILURE,
            "one hard Fail anywhere in the run decides the exit"
        );
    }
}
