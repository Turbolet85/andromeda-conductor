//! The 2026-10-07 sixth series (plan step 13): three drives on one fresh letters-only data dir, against
//! andromeda-pulse `9bfefb8` — the first live reading with the digest's corpus block narrowed to the
//! triggering scope, and the last series for `v3-09` in 0.3.0.

use crate::*;

use super::{Series, pre_registered, v3_09_met};

const GRADING: Series = Series {
    drives: &SERIES_2026_10_07_SIXTH,
    evidence: EVIDENCE_2026_10_07_SIXTH,
    measured: measured_2026_10_07_sixth,
};

/// A 2026-10-07 sixth-series drive's committed capture, digest-checked.
pub(crate) fn capture_2026_10_07_sixth(drive: &Drive) -> String {
    GRADING.capture(drive)
}

/// The digest of the series' contract section, recorded in the attempt ledger before `d1` fired.
const SERIES_2026_10_07_SIXTH_RULE_SHA256: &str =
    "01cf94c55845834391edd14c041152ad1c3fad6d4b2b7377221d5d8488f44fb5";

#[test]
fn the_2026_10_07_sixth_series_rule_was_fixed_before_d1() {
    pre_registered(
        EVIDENCE_2026_10_07_SIXTH,
        "## The 2026-10-07 sixth series",
        SERIES_2026_10_07_SIXTH_RULE_SHA256,
    );
}

#[test]
fn each_2026_10_07_sixth_capture_matches_its_pinned_digest() {
    GRADING.captures_match_their_pins();
}

#[test]
fn each_2026_10_07_sixth_drive_recorded_the_current_rule_before_it_fired() {
    GRADING.drives_recorded_the_current_rule();
}

/// The leaf of the series' data dir, which is Pulse's workspace key under that launch.
const KEY_2026_10_07_SIXTH: &str = "rm-sixth-series";

#[test]
fn the_2026_10_07_sixth_captures_carry_no_fingerprint_and_no_workspace_key() {
    GRADING.captures_carry_no_fingerprint(Some(KEY_2026_10_07_SIXTH));
}

/// What the rule measured on each 2026-10-07 sixth-series drive: its route, its rank-1 grade, the three
/// further grades and the canary tokens the capture printed.
fn measured_2026_10_07_sixth(label: &str) -> (Route, Grade, [Outcome; 3], CanaryAttempts) {
    let surfaced_twice = CanaryAttempts {
        surfaced: 2,
        dismissed: 0,
        pipeline_fault: 0,
    };
    // The third storm's digest tick fell before the emission instant, so the capture printed a third
    // token; Pulse's own log reads that storm parsed and deduped (the attempt ledger).
    let surfaced_twice_and_a_third_token = CanaryAttempts {
        surfaced: 2,
        dismissed: 0,
        pipeline_fault: 1,
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
            surfaced_twice_and_a_third_token,
        ),
        "d2" => (
            Route::ReadBack,
            Grade::Identified,
            [Outcome::Pass; 3],
            surfaced_twice_and_a_third_token,
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
fn each_2026_10_07_sixth_drive_grades_as_the_ledger_records() {
    GRADING.drives_grade_as_the_ledger_records();
    GRADING.drives_witness_the_real_model_and_a_clear_launch();
}

#[test]
fn the_2026_10_07_sixth_envelopes_carry_the_eleven_keys() {
    for drive in &SERIES_2026_10_07_SIXTH {
        let committed = capture_2026_10_07_sixth(drive);
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
fn v3_09_is_met_by_the_2026_10_07_sixth_series() {
    // The pass condition (posture contract, The drive series (a), carried into The 2026-10-07 sixth
    // series): met only if at least one drive is graded AND every graded drive reads Identified. All
    // three drives attributed an incident on the read-back route and each rank 1 names `conductor` as a
    // whole word with a retry token, so v3-09 is met — on these three drives, and read as no more.
    let graded = GRADING.graded();
    assert_eq!(
        graded,
        [
            ("d1", Grade::Identified),
            ("d2", Grade::Identified),
            ("d3", Grade::Identified)
        ]
    );
    assert_eq!(
        row(Grade::Identified),
        (Some(Verdict::Pass), ReportState::Pass)
    );
    assert!(
        v3_09_met(&graded),
        "the 2026-10-07 sixth series meets v3-09"
    );
}

/// `v3-09`'s ref: read from the committed captures alone, every graded drive reads Identified, on the
/// real model and under a launch whose cwd put no `conductor` in front of it.
#[test]
fn v3_09_ref_identified_with_the_real_model_witnesses_2026_10_07_sixth() {
    let graded = GRADING.graded();
    assert!(v3_09_met(&graded));
    for (label, _) in &graded {
        let drive = SERIES_2026_10_07_SIXTH
            .iter()
            .find(|drive| drive.label == *label)
            .expect("a graded drive is one of the series' drives");
        let committed = capture_2026_10_07_sixth(drive);
        let block = capture_block(&committed);
        assert!(real_model_witnessed(block), "{label}");
        assert!(launch_cwd_clear(block), "{label}");
    }
}
