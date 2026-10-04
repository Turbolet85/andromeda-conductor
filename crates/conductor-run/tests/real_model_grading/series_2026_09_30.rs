//! The 2026-09-30 series (plan steps 10-13): three drives on one fresh letters-only data dir, against
//! andromeda-pulse `fcc31b2`.

use crate::*;

use super::workspace_mask::KEY;
use super::{Series, pre_registered, v3_09_met};

const GRADING: Series = Series {
    drives: &SERIES_2026_09_30,
    evidence: EVIDENCE_2026_09_30,
    measured: measured_2026_09_30,
};

/// The digest of the series' contract section, recorded in the attempt ledger before `d1` fired.
const SERIES_2026_09_30_RULE_SHA256: &str =
    "0091fe6f876d05dfcaa4d454320a31426927d94cbcf13fe6d8095070a0753c19";

#[test]
fn the_2026_09_30_series_rule_was_fixed_before_d1() {
    // The section's digest now, the digest the ledger recorded before d1, and the pin agree: any edit
    // to the series' design after it was pre-registered fails here.
    pre_registered(
        EVIDENCE_2026_09_30,
        "## The 2026-09-30 series",
        SERIES_2026_09_30_RULE_SHA256,
    );
}

/// A 2026-09-30 drive's committed capture, digest-checked.
pub(crate) fn capture_2026_09_30(drive: &Drive) -> String {
    GRADING.capture(drive)
}

#[test]
fn each_2026_09_30_capture_matches_its_pinned_digest() {
    GRADING.captures_match_their_pins();
}

#[test]
fn each_2026_09_30_drive_recorded_the_current_rule_before_it_fired() {
    GRADING.drives_recorded_the_current_rule();
}

#[test]
fn the_2026_09_30_captures_carry_no_fingerprint_and_no_workspace_key() {
    // The capture's own elision and key mask, found to have left nothing in any capture. The ledger is
    // not held to the elision: it carries sha256 digests and a Pulse commit sha by design, and the
    // pre-registration digest must stay whole for `the_2026_09_30_series_rule_was_fixed_before_d1`.
    GRADING.captures_carry_no_fingerprint(Some(KEY));
}

/// What the rule measured on each 2026-09-30 drive: its route, its rank-1 grade, the three further
/// grades and the canary tokens the capture printed.
fn measured_2026_09_30(label: &str) -> (Route, Grade, [Outcome; 3], CanaryAttempts) {
    let none = Outcome::Blocked("no attributable incident");
    let tokens = |surfaced, dismissed, pipeline_fault| CanaryAttempts {
        surfaced,
        dismissed,
        pipeline_fault,
    };
    match label {
        "d1" => (
            Route::PreflightBlocked,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(0, 3, 0),
        ),
        "d2" => (
            Route::ReadBack,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(2, 1, 0),
        ),
        "d3" => (
            Route::ReadBack,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(2, 0, 0),
        ),
        other => panic!("no measurement recorded for {other}"),
    }
}

#[test]
fn each_2026_09_30_drive_grades_as_the_ledger_records() {
    GRADING.drives_grade_as_the_ledger_records();
    GRADING.drives_witness_the_real_model_and_a_clear_launch();
}

#[test]
fn the_2026_09_30_envelopes_carry_the_eleven_keys() {
    for drive in &SERIES_2026_09_30 {
        let committed = capture_2026_09_30(drive);
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
fn v3_09_is_not_met_by_the_2026_09_30_series() {
    // The pass condition (posture contract, The drive series (a), carried into The 2026-09-30 series):
    // met only if at least one drive is graded AND every graded drive reads Identified. No drive
    // attributed an incident on the read-back route, so none is graded and v3-09 is not met — recorded,
    // never replaced by a further drive.
    let graded = GRADING.graded();
    assert_eq!(graded, []);
    assert!(
        !v3_09_met(&graded),
        "the 2026-09-30 series does not meet v3-09"
    );
}
