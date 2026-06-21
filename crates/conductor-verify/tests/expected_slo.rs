//! Integration coverage for the expected-outcome + SLO timing evaluator over the public seam API.

use conductor_core::{ClaimClass, ComparisonKind, ExpectedCheck, SloTier, Verdict};
use conductor_verify::{compare, evaluate_check, evaluate_slo};
use rstest::rstest;

fn check(kind: ComparisonKind, class: ClaimClass, expected: &str) -> ExpectedCheck {
    ExpectedCheck { kind, class, expected: expected.to_string() }
}

#[rstest]
#[case(ComparisonKind::Exact, "ok", "ok", true)]
#[case(ComparisonKind::Exact, "ok", "OK", false)]
#[case(ComparisonKind::Contains, "P-009", "[P-009,P-010]", true)]
#[case(ComparisonKind::Contains, "P-099", "[P-009,P-010]", false)]
#[case(ComparisonKind::Absent, "secret", "scrubbed-report", true)]
#[case(ComparisonKind::Absent, "secret", "secret=abc", false)]
#[case(ComparisonKind::CountAtLeast, "50", "50", true)]
#[case(ComparisonKind::CountAtLeast, "50", "49", false)]
fn comparison_kind_matrix(
    #[case] kind: ComparisonKind,
    #[case] expected: &str,
    #[case] observed: &str,
    #[case] want: bool,
) {
    assert_eq!(compare(&check(kind, ClaimClass::Hard, expected), observed), want);
}

#[rstest]
#[case(SloTier::Tier5s, 0, 5_000, true)]
#[case(SloTier::Tier5s, 0, 5_001, false)]
#[case(SloTier::Tier20s, 0, 20_000, true)]
#[case(SloTier::Tier20s, 0, 20_001, false)]
#[case(SloTier::Tier90s, 0, 90_000, true)]
#[case(SloTier::Tier90s, 0, 90_001, false)]
fn slo_tier_deadline_matrix(
    #[case] tier: SloTier,
    #[case] emitted: i64,
    #[case] observed: i64,
    #[case] within: bool,
) {
    let o = evaluate_slo(tier, emitted, observed);
    assert_eq!(o.within_tolerance, within);
    assert_eq!(o.latency_ms, observed - emitted);
}

#[test]
fn hard_pass_requires_match_and_within_slo() {
    let c = check(ComparisonKind::Exact, ClaimClass::Hard, "resolved");
    let out = evaluate_check(&c, "resolved", SloTier::Tier5s, 1_000, 2_840);
    assert_eq!(out.assessment.verdict, Verdict::Pass);
    assert_eq!(out.slo.latency_ms, 1_840);
    assert!(out.slo.within_tolerance);
    assert_eq!(out.slo_tier, SloTier::Tier5s);
}

#[test]
fn calibration_region_check_never_hard_fails() {
    let c = check(ComparisonKind::Exact, ClaimClass::CalibrationRegion, "high");
    // Mismatched value AND outside the SLO, yet routes to CalibrationRegion — never a hard Fail.
    let out = evaluate_check(&c, "low", SloTier::Tier5s, 0, 9_000);
    assert_eq!(out.assessment.verdict, Verdict::CalibrationRegion);
    assert!(out.assessment.delta.is_some());
}

#[test]
fn determinism_same_inputs_same_outcome() {
    let c = check(ComparisonKind::Contains, ClaimClass::Hard, "fp-1");
    let a = evaluate_check(&c, "[fp-1,fp-2]", SloTier::Tier20s, 100, 1_500);
    let b = evaluate_check(&c, "[fp-1,fp-2]", SloTier::Tier20s, 100, 1_500);
    assert_eq!(a, b);
}

#[test]
fn observed_with_host_path_is_redacted_in_assessment() {
    let c = check(ComparisonKind::Exact, ClaimClass::Hard, "ok");
    let out = evaluate_check(&c, "C:\\Users\\turbo\\corpus.db", SloTier::Tier5s, 0, 1_000);
    assert_eq!(out.assessment.observed, "<redacted>");
    assert!(!out.assessment.observed.contains("C:\\Users"));
}
