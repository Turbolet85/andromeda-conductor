//! The per-scenario expected-outcome model — the declarative comparison target a check verifies
//! read-back against, plus the claim's assertion-policy class.
//!
//! Lives in the core (beside the scenario model) because it is declarative scenario config:
//! serde-deserialized and garde-validated at load, so the [`ClaimClass`] is *declared up front, never
//! guessed at runtime* (architecture §Probabilistic-Assertion Policy). The `conductor-verify`
//! evaluator consumes an [`ExpectedCheck`] to produce the `matched`/class the verdict classifier folds
//! into an `Assessment`.

use garde::Validate;
use serde::{Deserialize, Serialize};

/// Which side of the probabilistic-assertion policy a claim falls on (architecture
/// §Probabilistic-Assertion Policy).
///
/// Declared up front as a property of the claim, never guessed at runtime: deterministic claims (hard
/// signals, baseline math, suppression/bypass logic, lifecycle timing) are [`ClaimClass::Hard`];
/// model-interpretive claims (severity choice, hypothesis quality, the P-008 root-vs-deep weighting)
/// are [`ClaimClass::CalibrationRegion`] and are never hard-failed on exact values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimClass {
    /// A deterministic claim — hard `Pass`/`Fail` on its comparison.
    Hard,
    /// A model-interpretive claim — reported-for-human, routed to the calibration region.
    CalibrationRegion,
}

/// The deterministic comparison a check applies to its observed read-back value against the
/// [`ExpectedCheck::expected`] target. Closed set; serializes to its canonical PascalCase name.
///
/// `CountAtLeast` is the sample-count floor (≥50-sample latency / ≥10-span error-rate): when unmet it
/// routes to the calibration region instead of hard-failing (architecture §Timing-Tolerance Model),
/// a rule the verify evaluator applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonKind {
    /// `observed == expected`.
    Exact,
    /// `observed` contains the `expected` token (membership / substring — incident present,
    /// fingerprint ∈ set).
    Contains,
    /// `observed` does NOT contain the `expected` token (e.g. a scrubbed PII secret is absent).
    Absent,
    /// `observed` parsed as an integer is ≥ the `expected` floor (the sample-count floor).
    CountAtLeast,
}

/// The absolute ceiling on a declared [`ExpectedCheck::budget_ms`] — the coarsest deadline in the
/// closed [`crate::SloTier`] ladder, derived from the tier itself rather than re-pinned as a
/// literal. The scenario-level rule tightens it further to the scenario's OWN tier
/// ([`crate::Scenario::check_budgets`]).
pub const MAX_BUDGET_MS: u32 = crate::SloTier::Tier90s.deadline_ms() as u32;

/// One per-scenario expected-outcome check: which comparison to apply, the declared claim class, the
/// expected target value, and the optional timing budget it is held to.
///
/// `#[derive(Validate)]` enforces a non-empty `expected` target at load (`CoreError::Validation`);
/// the `kind`/`class` enums are closed so they need no range rule. For `CountAtLeast`, `expected` is
/// the floor as its decimal string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct ExpectedCheck {
    /// The deterministic comparison this check applies.
    #[garde(skip)]
    pub kind: ComparisonKind,
    /// The assertion-policy class — declared, never inferred at runtime.
    #[garde(skip)]
    pub class: ClaimClass,
    /// The expected target the comparison reads (a token, or the decimal floor for `CountAtLeast`).
    #[garde(length(min = 1))]
    pub expected: String,
    /// This check's own timing budget in milliseconds, declared BENEATH the scenario's
    /// [`crate::SloTier`]. Absent means the tier's own `deadline_ms()` governs, which is the shape
    /// every pre-budget scenario keeps. Two checks of one scenario share a read-back instant (the
    /// corpus is observed once), so the budget is what lets them reach different timing verdicts.
    #[serde(default)]
    #[garde(range(min = 1, max = MAX_BUDGET_MS))]
    pub budget_ms: Option<u32>,
}

impl ExpectedCheck {
    /// The deadline this check is actually graded against: its own budget when declared, else the
    /// scenario tier's (architecture §Timing-Tolerance Model).
    pub fn effective_deadline_ms(&self, tier: crate::SloTier) -> i64 {
        match self.budget_ms {
            Some(budget) => i64::from(budget),
            None => tier.deadline_ms(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(kind: ComparisonKind, class: ClaimClass, expected: &str) -> ExpectedCheck {
        ExpectedCheck { kind, class, expected: expected.to_string(), budget_ms: None }
    }

    #[test]
    fn claim_class_serializes_to_canonical_names() {
        // Locks the wire spelling — an accidental `rename_all` would break it.
        assert_eq!(serde_json::to_string(&ClaimClass::Hard).unwrap(), "\"Hard\"");
        assert_eq!(
            serde_json::to_string(&ClaimClass::CalibrationRegion).unwrap(),
            "\"CalibrationRegion\""
        );
    }

    #[test]
    fn comparison_kind_serializes_to_canonical_names() {
        assert_eq!(serde_json::to_string(&ComparisonKind::Exact).unwrap(), "\"Exact\"");
        assert_eq!(serde_json::to_string(&ComparisonKind::Contains).unwrap(), "\"Contains\"");
        assert_eq!(serde_json::to_string(&ComparisonKind::Absent).unwrap(), "\"Absent\"");
        assert_eq!(
            serde_json::to_string(&ComparisonKind::CountAtLeast).unwrap(),
            "\"CountAtLeast\""
        );
    }

    #[test]
    fn well_formed_check_validates() {
        assert!(check(ComparisonKind::Exact, ClaimClass::Hard, "P-009").validate().is_ok());
        assert!(
            check(ComparisonKind::CountAtLeast, ClaimClass::CalibrationRegion, "50")
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn empty_expected_target_is_rejected() {
        assert!(check(ComparisonKind::Contains, ClaimClass::Hard, "").validate().is_err());
    }

    #[test]
    fn round_trips_through_json() {
        let c = check(ComparisonKind::Contains, ClaimClass::Hard, "incident-42");
        let json = serde_json::to_string(&c).unwrap();
        let back: ExpectedCheck = serde_json::from_str(&json).unwrap();
        assert_eq!(c, back);
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
    fn an_absent_budget_falls_back_to_the_scenario_tier() {
        // The shape every pre-budget scenario keeps: no declaration ⇒ the tier's own deadline, for
        // each tier in the closed ladder (asserted against the rule, never a re-pinned literal).
        for tier in [crate::SloTier::Tier5s, crate::SloTier::Tier20s, crate::SloTier::Tier90s] {
            assert_eq!(budgeted(None).effective_deadline_ms(tier), tier.deadline_ms());
        }
    }

    #[test]
    fn a_declared_budget_overrides_the_tier() {
        let tier = crate::SloTier::Tier90s;
        assert_eq!(budgeted(Some(2_000)).effective_deadline_ms(tier), 2_000);
        assert!(
            budgeted(Some(2_000)).effective_deadline_ms(tier) < tier.deadline_ms(),
            "a budget sits BENEATH its tier"
        );
    }

    #[test]
    fn the_budget_ceiling_derives_from_the_ladder_not_a_literal() {
        assert_eq!(i64::from(MAX_BUDGET_MS), crate::SloTier::Tier90s.deadline_ms());
    }

    #[test]
    fn a_zero_or_over_ceiling_budget_is_rejected_but_the_bounds_are_accepted() {
        assert!(budgeted(Some(0)).validate().is_err(), "a 0ms budget is unmeetable");
        assert!(budgeted(Some(MAX_BUDGET_MS + 1)).validate().is_err());
        assert!(budgeted(Some(1)).validate().is_ok());
        assert!(budgeted(Some(MAX_BUDGET_MS)).validate().is_ok());
    }

    #[test]
    fn budget_round_trips_through_json_and_defaults_absent() {
        let c = budgeted(Some(2_500));
        let back: ExpectedCheck = serde_json::from_str(&serde_json::to_string(&c).unwrap()).unwrap();
        assert_eq!(c, back);
        // A document predating the field still deserializes — `budget_ms` is serde-defaulted.
        let legacy: ExpectedCheck = serde_json::from_str(
            r#"{"kind":"Contains","class":"Hard","expected":"ok"}"#,
        )
        .unwrap();
        assert_eq!(legacy.budget_ms, None);
    }
}
