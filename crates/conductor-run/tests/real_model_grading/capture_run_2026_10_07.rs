//! The 2026-10-07 capture run (plan step 12): three drives on one fresh letters-only data dir, against
//! andromeda-pulse `f70be92`, under the launch posture note of its contract section. A capture, never a
//! series: each grade here is an observation, and this module reads no pass condition.

use crate::*;

use super::{Series, pre_registered};

const GRADING: Series = Series {
    drives: &CAPTURE_RUN_2026_10_07,
    evidence: EVIDENCE_CAPTURE_RUN_2026_10_07,
    measured: measured_capture_run_2026_10_07,
};

/// A 2026-10-07 capture-run drive's committed capture, digest-checked.
pub(crate) fn capture_run_2026_10_07(drive: &Drive) -> String {
    GRADING.capture(drive)
}

/// The digest of the run's contract section, recorded in the attempt ledger before `d1` fired.
const CAPTURE_RUN_2026_10_07_RECORD_SHA256: &str =
    "513892694d7d0f16b51bf7d225b8d11986c16281d0647937768889fa763387da";

#[test]
fn the_2026_10_07_capture_run_record_was_fixed_before_d1() {
    pre_registered(
        EVIDENCE_CAPTURE_RUN_2026_10_07,
        "## The 2026-10-07 capture run",
        CAPTURE_RUN_2026_10_07_RECORD_SHA256,
    );
}

#[test]
fn each_2026_10_07_capture_run_capture_matches_its_pinned_digest() {
    GRADING.captures_match_their_pins();
}

#[test]
fn each_2026_10_07_capture_run_drive_recorded_the_current_rule_before_it_fired() {
    GRADING.drives_recorded_the_current_rule();
}

/// The leaf of the run's data dir, which is Pulse's workspace key under that launch.
const KEY_CAPTURE_RUN_2026_10_07: &str = "rm-recorded-run";

#[test]
fn the_2026_10_07_capture_run_captures_carry_no_fingerprint_and_no_workspace_key() {
    GRADING.captures_carry_no_fingerprint(Some(KEY_CAPTURE_RUN_2026_10_07));
}

/// What the rule measured on each drive of the run: its route, its rank-1 grade, the three further
/// grades and the canary tokens the capture printed.
fn measured_capture_run_2026_10_07(label: &str) -> (Route, Grade, [Outcome; 3], CanaryAttempts) {
    let surfaced_twice = CanaryAttempts {
        surfaced: 2,
        dismissed: 0,
        pipeline_fault: 0,
    };
    match label {
        "d1" => (
            Route::ReadBack,
            Grade::Identified,
            [
                Outcome::Pass,
                Outcome::Pass,
                Outcome::Blocked("no prior same-scope incident to retrieve"),
            ],
            surfaced_twice,
        ),
        "d2" => (
            Route::ReadBack,
            Grade::Identified,
            [Outcome::Pass; 3],
            surfaced_twice,
        ),
        "d3" => (
            Route::ReadBack,
            Grade::Identified,
            [Outcome::Pass; 3],
            surfaced_twice,
        ),
        other => panic!("no measurement recorded for {other}"),
    }
}

#[test]
fn each_2026_10_07_capture_run_drive_grades_as_the_ledger_records() {
    GRADING.drives_grade_as_the_ledger_records();
    GRADING.drives_witness_the_real_model_and_a_clear_launch();
}

#[test]
fn the_2026_10_07_capture_run_envelopes_carry_the_eleven_keys() {
    for drive in &CAPTURE_RUN_2026_10_07 {
        let committed = capture_run_2026_10_07(drive);
        let record = envelope_record(capture_block(&committed), drive.label)
            .expect("every drive printed its envelope");
        assert_eq!(record.scenario, "real-model-interpretation");
        assert_eq!(record.verdict, None, "{}: declare-only", drive.label);
        assert!(
            matches!(
                record.state,
                ReportState::ManualCheck | ReportState::Blocked
            ),
            "{}",
            drive.label
        );
    }
}
