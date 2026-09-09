//! Public-surface tests for verdict classification — the two-state assertion policy.

use conductor_core::Verdict;
use conductor_verify::{Assessment, ClaimClass, classify};
use rstest::rstest;

#[rstest]
#[case(ClaimClass::Hard, true, Verdict::Pass)]
#[case(ClaimClass::Hard, false, Verdict::Fail)]
#[case(ClaimClass::CalibrationRegion, true, Verdict::CalibrationRegion)]
#[case(ClaimClass::CalibrationRegion, false, Verdict::CalibrationRegion)]
fn classify_maps_each_class_to_its_verdict(
    #[case] class: ClaimClass,
    #[case] matched: bool,
    #[case] expected_verdict: Verdict,
) {
    assert_eq!(
        classify(class, matched, "obs", "exp").verdict(),
        expected_verdict
    );
}

#[rstest]
#[case(true)]
#[case(false)]
fn calibration_region_is_never_a_hard_fail(#[case] matched: bool) {
    // The policy invariant: a model-interpretive claim is reported-for-human, never hard-failed —
    // even when its comparison did not match.
    let a = classify(ClaimClass::CalibrationRegion, matched, "obs", "exp");
    assert_ne!(a.verdict(), Verdict::Fail);
    assert_eq!(a.verdict(), Verdict::CalibrationRegion);
}

#[test]
fn delta_is_present_only_for_calibration_region() {
    assert!(classify(ClaimClass::Hard, true, "o", "e").delta.is_none());
    assert!(classify(ClaimClass::Hard, false, "o", "e").delta.is_none());
    assert!(
        classify(ClaimClass::CalibrationRegion, true, "o", "e")
            .delta
            .is_some()
    );
}

#[test]
fn assessment_carries_observed_expected_and_delta() {
    let a: Assessment = classify(
        ClaimClass::CalibrationRegion,
        false,
        "halo burgundy",
        "halo amber",
    );
    assert_eq!(a.observed, "halo burgundy");
    assert_eq!(a.expected, "halo amber");
    assert_eq!(
        a.delta.unwrap(),
        "observed halo burgundy vs expected halo amber"
    );
}
