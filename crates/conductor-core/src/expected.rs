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

/// One per-scenario expected-outcome check: which comparison to apply, the declared claim class, and
/// the expected target value.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(kind: ComparisonKind, class: ClaimClass, expected: &str) -> ExpectedCheck {
        ExpectedCheck { kind, class, expected: expected.to_string() }
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
}
