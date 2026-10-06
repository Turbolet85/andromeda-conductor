//! The 2026-10-06 series (plan step 12): three drives on one fresh letters-only data dir, against
//! andromeda-pulse `5f77859` — the first reading of Pulse's shipped model.

use crate::*;

use super::{Series, pre_registered, v3_09_met};

const GRADING: Series = Series {
    drives: &SERIES_2026_10_06,
    evidence: EVIDENCE_2026_10_06,
    measured: measured_2026_10_06,
};

/// A 2026-10-06 drive's committed capture, digest-checked.
pub(crate) fn capture_2026_10_06(drive: &Drive) -> String {
    GRADING.capture(drive)
}

/// The digest of the series' contract section, recorded in the attempt ledger before `d1` fired.
const SERIES_2026_10_06_RULE_SHA256: &str =
    "00173912ffa684a4eebe120e45a9afb61c1766fff888bada4b5e713bb26f8097";

#[test]
fn the_2026_10_06_series_rule_was_fixed_before_d1() {
    pre_registered(
        EVIDENCE_2026_10_06,
        "## The 2026-10-06 series",
        SERIES_2026_10_06_RULE_SHA256,
    );
}

#[test]
fn each_2026_10_06_capture_matches_its_pinned_digest() {
    GRADING.captures_match_their_pins();
}

#[test]
fn each_2026_10_06_drive_recorded_the_current_rule_before_it_fired() {
    GRADING.drives_recorded_the_current_rule();
}

/// The leaf of the series' data dir, which is Pulse's workspace key under that launch.
const KEY_2026_10_06: &str = "rm-trigger-series";

#[test]
fn the_2026_10_06_captures_carry_no_fingerprint_and_no_workspace_key() {
    GRADING.captures_carry_no_fingerprint(Some(KEY_2026_10_06));
}

/// What the rule measured on each 2026-10-06 drive: its route, its rank-1 grade, the three further
/// grades and the canary tokens the capture printed.
fn measured_2026_10_06(label: &str) -> (Route, Grade, [Outcome; 3], CanaryAttempts) {
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
            Grade::NotIdentified,
            [Outcome::Pass; 3],
            surfaced_twice,
        ),
        other => panic!("no measurement recorded for {other}"),
    }
}

#[test]
fn each_2026_10_06_drive_grades_as_the_ledger_records() {
    GRADING.drives_grade_as_the_ledger_records();
    GRADING.drives_witness_the_real_model_and_a_clear_launch();
}

#[test]
fn the_2026_10_06_envelopes_carry_the_eleven_keys() {
    for drive in &SERIES_2026_10_06 {
        let committed = capture_2026_10_06(drive);
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

#[test]
fn v3_09_is_not_met_by_the_2026_10_06_series() {
    // The pass condition (posture contract, The drive series (a), carried into The 2026-10-06 series):
    // met only if at least one drive is graded AND every graded drive reads Identified. All three drives
    // attributed an incident on the read-back route; d3's rank 1 names the retry storm on the canary
    // identity alone, so it reads NotIdentified and v3-09 is not met — recorded, never replaced by a
    // further drive.
    let graded = GRADING.graded();
    assert_eq!(
        graded,
        [
            ("d1", Grade::Identified),
            ("d2", Grade::Identified),
            ("d3", Grade::NotIdentified)
        ]
    );
    assert_eq!(
        row(Grade::NotIdentified),
        (Some(Verdict::CalibrationRegion), ReportState::ManualCheck)
    );
    assert!(
        !v3_09_met(&graded),
        "the 2026-10-06 series does not meet v3-09"
    );
}
