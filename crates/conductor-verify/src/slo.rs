//! Expected-outcome comparison + the journal-relative SLO timing model.
//!
//! The evaluator that bridges a read-back observation to [`classify`]: [`compare`] applies an
//! [`ExpectedCheck`]'s [`ComparisonKind`], [`evaluate_slo`] turns two journal instants into a
//! tier-scaled deadline outcome, and [`evaluate_check`] folds both into the `(matched, ClaimClass)`
//! the classifier consumes. All infallible — values, never `Result::Err` (the verdict/error wall).

use conductor_core::{ClaimClass, ComparisonKind, ExpectedCheck, SloTier};
use serde::Serialize;

use crate::verdict::{Assessment, classify};

/// The journal-relative SLO timing outcome for one check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SloOutcome {
    /// `read_back_observed_at − journal_emitted_at`, in milliseconds (journal-relative, `std::time`).
    pub latency_ms: i64,
    /// Whether the latency met the deadline (`0 ≤ latency_ms ≤ deadline_ms`).
    pub within_tolerance: bool,
    /// The deadline this outcome was judged against — the check's own `budget_ms` when it declared
    /// one, else its scenario tier's. Carried so a reader can tell WHICH bound the verdict rests on:
    /// checks of one scenario share a read-back instant, so the deadline is the only thing that can
    /// separate their timing verdicts.
    pub deadline_ms: i64,
}

/// Compute the journal-relative SLO outcome against an effective deadline.
///
/// Latency is `read_back_observed_at_ms − journal_emitted_at_ms`; a negative latency (read-back
/// observed before emission — a clock anomaly) is never within tolerance. The deadline is the
/// check's own budget when it declared one, else its scenario tier's — see
/// [`conductor_core::ExpectedCheck::effective_deadline_ms`].
pub fn evaluate_slo(
    deadline_ms: i64,
    journal_emitted_at_ms: i64,
    read_back_observed_at_ms: i64,
) -> SloOutcome {
    let latency_ms = read_back_observed_at_ms - journal_emitted_at_ms;
    let within_tolerance = (0..=deadline_ms).contains(&latency_ms);
    SloOutcome { latency_ms, within_tolerance, deadline_ms }
}

/// Apply a check's [`ComparisonKind`] to an observed read-back value, yielding the deterministic match.
pub fn compare(check: &ExpectedCheck, observed: &str) -> bool {
    match check.kind {
        ComparisonKind::Exact => observed == check.expected,
        ComparisonKind::Contains => observed.contains(check.expected.as_str()),
        ComparisonKind::Absent => !observed.contains(check.expected.as_str()),
        ComparisonKind::CountAtLeast => {
            match (observed.trim().parse::<i64>(), check.expected.trim().parse::<i64>()) {
                (Ok(count), Ok(floor)) => count >= floor,
                _ => false,
            }
        }
    }
}

/// One check fully evaluated: the classified [`Assessment`] plus the SLO timing it was measured against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CheckOutcome {
    /// The classified verdict + redacted observed/expected + calibration delta.
    pub assessment: Assessment,
    /// The journal-relative SLO timing.
    pub slo: SloOutcome,
    /// The tier the deadline was measured against.
    pub slo_tier: SloTier,
}

/// Evaluate one expected check end-to-end against an observed value and its journal timing.
///
/// A `Hard` claim matches only when BOTH the comparison holds AND the latency is within the check's
/// effective deadline — its own `budget_ms` when declared, else `tier.deadline_ms()`. A sample-count
/// floor ([`ComparisonKind::CountAtLeast`]) that is unmet routes to the calibration region regardless
/// of the declared class — sample floors never hard-fail (architecture §Timing-Tolerance Model).
/// Infallible; `observed`/`expected` are redacted inside [`classify`].
pub fn evaluate_check(
    check: &ExpectedCheck,
    observed: &str,
    tier: SloTier,
    journal_emitted_at_ms: i64,
    read_back_observed_at_ms: i64,
) -> CheckOutcome {
    let slo = evaluate_slo(
        check.effective_deadline_ms(tier),
        journal_emitted_at_ms,
        read_back_observed_at_ms,
    );
    let value_matched = compare(check, observed);
    let matched = value_matched && slo.within_tolerance;

    let class = if matches!(check.kind, ComparisonKind::CountAtLeast) && !value_matched {
        ClaimClass::CalibrationRegion
    } else {
        check.class
    };

    let assessment = classify(class, matched, observed, check.expected.as_str());
    CheckOutcome { assessment, slo, slo_tier: tier }
}

#[cfg(test)]
mod tests {
    use super::*;
    use conductor_core::Verdict;

    fn check(kind: ComparisonKind, class: ClaimClass, expected: &str) -> ExpectedCheck {
        ExpectedCheck { kind, class, expected: expected.to_string(), budget_ms: None }
    }

    #[test]
    fn exact_contains_absent_compare_as_expected() {
        assert!(compare(&check(ComparisonKind::Exact, ClaimClass::Hard, "ok"), "ok"));
        assert!(!compare(&check(ComparisonKind::Exact, ClaimClass::Hard, "ok"), "nope"));
        assert!(compare(&check(ComparisonKind::Contains, ClaimClass::Hard, "P-009"), "[P-009,P-010]"));
        assert!(compare(&check(ComparisonKind::Absent, ClaimClass::Hard, "secret"), "scrubbed"));
        assert!(!compare(&check(ComparisonKind::Absent, ClaimClass::Hard, "secret"), "secret=xyz"));
    }

    #[test]
    fn count_at_least_floor() {
        assert!(compare(&check(ComparisonKind::CountAtLeast, ClaimClass::Hard, "50"), "50"));
        assert!(compare(&check(ComparisonKind::CountAtLeast, ClaimClass::Hard, "50"), "51"));
        assert!(!compare(&check(ComparisonKind::CountAtLeast, ClaimClass::Hard, "50"), "49"));
        assert!(!compare(&check(ComparisonKind::CountAtLeast, ClaimClass::Hard, "50"), "notanumber"));
    }

    #[test]
    fn slo_within_and_exceeded_per_tier() {
        assert!(evaluate_slo(SloTier::Tier5s.deadline_ms(), 1000, 5000).within_tolerance); // 4000 ≤ 5000
        assert!(!evaluate_slo(SloTier::Tier5s.deadline_ms(), 1000, 6001).within_tolerance); // 5001 > 5000
        assert!(evaluate_slo(SloTier::Tier5s.deadline_ms(), 0, 5000).within_tolerance); // boundary
        assert!(!evaluate_slo(SloTier::Tier5s.deadline_ms(), 0, 5001).within_tolerance);
    }

    #[test]
    fn negative_latency_is_never_within_tolerance() {
        let o = evaluate_slo(SloTier::Tier90s.deadline_ms(), 5000, 1000);
        assert_eq!(o.latency_ms, -4000);
        assert!(!o.within_tolerance);
    }

    #[test]
    fn hard_check_passes_only_when_matched_and_within_slo() {
        let c = check(ComparisonKind::Exact, ClaimClass::Hard, "ok");
        assert_eq!(evaluate_check(&c, "ok", SloTier::Tier5s, 0, 1000).assessment.verdict, Verdict::Pass);
        assert_eq!(evaluate_check(&c, "ok", SloTier::Tier5s, 0, 9000).assessment.verdict, Verdict::Fail);
        assert_eq!(evaluate_check(&c, "no", SloTier::Tier5s, 0, 1000).assessment.verdict, Verdict::Fail);
    }

    #[test]
    fn unmet_count_floor_routes_to_calibration_region_even_when_declared_hard() {
        let c = check(ComparisonKind::CountAtLeast, ClaimClass::Hard, "50");
        let out = evaluate_check(&c, "10", SloTier::Tier20s, 0, 1000);
        assert_eq!(out.assessment.verdict, Verdict::CalibrationRegion);
    }

    #[test]
    fn evaluation_is_deterministic() {
        let c = check(ComparisonKind::Contains, ClaimClass::Hard, "P-009");
        let a = evaluate_check(&c, "[P-009]", SloTier::Tier5s, 100, 900);
        let b = evaluate_check(&c, "[P-009]", SloTier::Tier5s, 100, 900);
        assert_eq!(a, b);
    }

    fn budgeted(budget_ms: Option<u32>) -> ExpectedCheck {
        ExpectedCheck {
            kind: ComparisonKind::Contains,
            class: ClaimClass::Hard,
            expected: "ok".to_string(),
            budget_ms,
        }
    }

    #[test]
    fn two_checks_sharing_one_instant_reach_different_verdicts_on_their_budgets() {
        // THE non-vacuity assertion. The corpus is observed once, so both checks are handed the
        // SAME instant pair and measure the SAME latency; only the declared budget differs, and
        // that alone must be enough to separate their verdicts. Were the budget scenario-level,
        // these two would be indistinguishable.
        let (emitted, observed) = (0, 3_000);
        let tight = evaluate_check(&budgeted(Some(2_000)), "ok", SloTier::Tier90s, emitted, observed);
        let loose = evaluate_check(&budgeted(Some(4_000)), "ok", SloTier::Tier90s, emitted, observed);

        assert_eq!(tight.slo.latency_ms, loose.slo.latency_ms, "one observation, one latency");
        assert_ne!(tight.slo.deadline_ms, loose.slo.deadline_ms, "the deadline is what differs");
        assert_eq!(tight.assessment.verdict, Verdict::Fail, "3000ms missed its 2000ms budget");
        assert_eq!(loose.assessment.verdict, Verdict::Pass, "3000ms met its 4000ms budget");
    }

    #[test]
    fn the_outcome_records_which_bound_it_was_judged_against() {
        let declared = evaluate_check(&budgeted(Some(1_500)), "ok", SloTier::Tier90s, 0, 100);
        assert_eq!(declared.slo.deadline_ms, 1_500);
        let inherited = evaluate_check(&budgeted(None), "ok", SloTier::Tier20s, 0, 100);
        assert_eq!(inherited.slo.deadline_ms, SloTier::Tier20s.deadline_ms());
    }

    #[test]
    fn a_budget_can_fail_a_check_its_tier_would_have_passed() {
        // The budget must actually TIGHTEN the bound, not merely be carried alongside it.
        let latency = 6_000;
        let by_tier = evaluate_check(&budgeted(None), "ok", SloTier::Tier90s, 0, latency);
        let by_budget = evaluate_check(&budgeted(Some(5_000)), "ok", SloTier::Tier90s, 0, latency);
        assert_eq!(by_tier.assessment.verdict, Verdict::Pass);
        assert_eq!(by_budget.assessment.verdict, Verdict::Fail);
    }
}
