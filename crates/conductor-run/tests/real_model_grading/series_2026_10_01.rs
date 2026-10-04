//! The 2026-10-01 series (plan step 12): three drives on one fresh letters-only data dir, against
//! andromeda-pulse `a2addb3`.

use crate::*;

use super::{Series, pre_registered, v3_09_met};

const GRADING: Series = Series {
    drives: &SERIES_2026_10_01,
    evidence: EVIDENCE_2026_10_01,
    measured: measured_2026_10_01,
};

/// A 2026-10-01 drive's committed capture, digest-checked.
pub(crate) fn capture_2026_10_01(drive: &Drive) -> String {
    GRADING.capture(drive)
}

/// The digest of the series' contract section, recorded in the attempt ledger before `d1` fired.
const SERIES_2026_10_01_RULE_SHA256: &str =
    "0232afb1c302c49e408c92246ebfb6090c64c06ef81dea782a4af7656422e841";

#[test]
fn the_2026_10_01_series_rule_was_fixed_before_d1() {
    pre_registered(
        EVIDENCE_2026_10_01,
        "## The 2026-10-01 series",
        SERIES_2026_10_01_RULE_SHA256,
    );
}

#[test]
fn each_2026_10_01_capture_matches_its_pinned_digest() {
    GRADING.captures_match_their_pins();
}

#[test]
fn each_2026_10_01_drive_recorded_the_current_rule_before_it_fired() {
    GRADING.drives_recorded_the_current_rule();
}

/// The leaf of the series' data dir, which is Pulse's workspace key under that launch.
const KEY_2026_10_01: &str = "rm-surfacing-series";

#[test]
fn the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key() {
    // The capture's own elision and key mask, found to have left nothing in any capture. d3 once kept an
    // all-digit `fingerprint_hex` prefix that the unkeyed rule passes as a stamp would be; the keyed rule
    // elides it, d3 was re-elided on 2026-10-02 under the founder's ruling, and no drive is an exception.
    GRADING.captures_carry_no_fingerprint(Some(KEY_2026_10_01));
    for drive in &SERIES_2026_10_01 {
        let committed = capture_2026_10_01(drive);
        let all_digit_prefixes = committed
            .split("fingerprint_hex=")
            .skip(1)
            .filter(|rest| {
                let run: String = rest
                    .chars()
                    .take_while(char::is_ascii_alphanumeric)
                    .collect();
                !run.is_empty() && run.chars().all(|c| c.is_ascii_digit())
            })
            .count();
        assert_eq!(all_digit_prefixes, 0, "{}", drive.label);
    }
}

/// What the rule measured on each 2026-10-01 drive: its route, its rank-1 grade, the three further
/// grades and the canary tokens the capture printed.
fn measured_2026_10_01(label: &str) -> (Route, Grade, [Outcome; 3], CanaryAttempts) {
    let none = Outcome::Blocked("no attributable incident");
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
            Grade::NoAttributableIncident,
            [none; 3],
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
fn each_2026_10_01_drive_grades_as_the_ledger_records() {
    GRADING.drives_grade_as_the_ledger_records();
    GRADING.drives_witness_the_real_model_and_a_clear_launch();
}

#[test]
fn the_2026_10_01_envelopes_carry_the_eleven_keys() {
    for drive in &SERIES_2026_10_01 {
        let committed = capture_2026_10_01(drive);
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
fn v3_09_is_not_met_by_the_2026_10_01_series() {
    // The pass condition (posture contract, The drive series (a), carried into The 2026-10-01 series):
    // met only if at least one drive is graded AND every graded drive reads Identified. d1 and d3
    // attributed an incident on the read-back route; d3 reads NotIdentified, so v3-09 is not met —
    // recorded, never replaced by a further drive.
    let graded = GRADING.graded();
    assert_eq!(
        graded,
        [("d1", Grade::Identified), ("d3", Grade::NotIdentified)]
    );
    assert_eq!(
        row(Grade::NotIdentified),
        (Some(Verdict::CalibrationRegion), ReportState::ManualCheck)
    );
    assert!(
        !v3_09_met(&graded),
        "the 2026-10-01 series does not meet v3-09"
    );
}
