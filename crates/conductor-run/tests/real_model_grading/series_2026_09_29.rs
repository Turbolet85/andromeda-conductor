//! The 2026-09-29 drive series (step 10): six captures against andromeda-pulse `e98d838` under the
//! real-model posture on one long-lived data dir, recorded one row each in `evidence/attempt-ledger.md`.
//! Graded by the rule; nobody re-judges them.

use crate::*;

use super::{Series, v3_09_met};

const GRADING: Series = Series {
    drives: &SERIES,
    evidence: EVIDENCE,
    measured,
};

/// A series drive's committed capture, digest-checked.
pub(crate) fn series_capture(drive: &Drive) -> String {
    GRADING.capture(drive)
}

#[test]
fn each_series_capture_matches_its_pinned_digest() {
    GRADING.captures_match_their_pins();
}

#[test]
fn each_series_drive_recorded_the_current_rule_before_it_fired() {
    // The rule was not touched between the series and now, so every drive's pre-leg record equals it.
    GRADING.drives_recorded_the_current_rule();
}

#[test]
fn the_series_captures_carry_no_fingerprint() {
    // Elided once after the series by a mirror of the capture's own rule; this is the Rust rule
    // finding nothing left in any committed capture or the ledger.
    let ledger = committed(&format!("{EVIDENCE}/attempt-ledger.md"));
    assert_eq!(elide_fingerprints(&ledger), ledger, "the attempt ledger");
    GRADING.captures_carry_no_fingerprint(None);
}

/// What the rule measured on each drive: its route, its rank-1 grade, the three further grades and
/// the canary tokens the capture printed (b1's and b2's `pipeline-fault` are the pairing artifacts
/// the ledger corrects from Pulse's own log; the tokens are recorded as printed).
fn measured(label: &str) -> (Route, Grade, [Outcome; 3], CanaryAttempts) {
    let none = Outcome::Blocked("no attributable incident");
    let tokens = |surfaced, dismissed, pipeline_fault| CanaryAttempts {
        surfaced,
        dismissed,
        pipeline_fault,
    };
    match label {
        "a1-pipeline-fault" => (
            Route::PreflightBlocked,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(0, 0, 1),
        ),
        "a1" => (
            Route::PreflightBlocked,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(0, 1, 0),
        ),
        "a2" => (
            Route::EmittedNoReadBack,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(1, 0, 0),
        ),
        "a3" => (
            Route::ReadBack,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(1, 0, 0),
        ),
        "b1" => (
            Route::ReadBack,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(0, 1, 1),
        ),
        "b2" => (
            Route::ReadBack,
            Grade::NotIdentified,
            [Outcome::Pass; 3],
            tokens(1, 0, 1),
        ),
        other => panic!("no measurement recorded for {other}"),
    }
}

#[test]
fn each_series_drive_grades_as_the_ledger_records() {
    GRADING.drives_grade_as_the_ledger_records();
}

#[test]
fn every_series_drive_witnesses_the_real_model_and_a_clear_launch() {
    GRADING.drives_witness_the_real_model_and_a_clear_launch();
}

#[test]
fn the_series_envelopes_carry_the_eleven_keys() {
    for drive in &SERIES {
        let committed = series_capture(drive);
        let Some(record) = envelope_record(capture_block(&committed), drive.label) else {
            continue;
        };
        assert_eq!(record.scenario, "real-model-interpretation");
        assert_eq!(record.verdict, None, "{}: declare-only", drive.label);
    }
}

#[test]
fn v3_09_is_not_met_by_the_series() {
    // D1 (posture contract, The drive series (a)): met only if at least one drive is graded AND every
    // graded drive reads Identified. A drive is graded when it attributed an incident on the read-back
    // route. b2 alone did, and it reads NotIdentified — recorded, never replaced.
    let graded = GRADING.graded();
    assert_eq!(graded, [("b2", Grade::NotIdentified)]);
    assert_eq!(
        row(Grade::NotIdentified),
        (Some(Verdict::CalibrationRegion), ReportState::ManualCheck)
    );
    assert!(!v3_09_met(&graded), "the series does not meet v3-09");
}
