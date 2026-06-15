//! The machine verdict for a verification check.

use serde::{Deserialize, Serialize};

/// The outcome of a single verification check.
///
/// Deterministic claims are hard [`Verdict::Pass`]/[`Verdict::Fail`]; model-interpretive
/// claims (severity choice, hypothesis quality) route to [`Verdict::CalibrationRegion`] and
/// are reported-for-human, never hard-failed on exact values (arch §Probabilistic-Assertion
/// Policy). A `Verdict` is always a value — returned as `Ok(Verdict)`, never an `Err` (the
/// verdict/error wall; see [`crate::CoreError`]). Serializes to its canonical PascalCase name
/// for the run-report envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    /// Deterministic check met its assertion. Renders green / `[PASS]`.
    Pass,
    /// Deterministic check failed its assertion. Renders red / `[FAIL]`.
    Fail,
    /// Model-interpretive check inside the calibration region — reported for a human, not
    /// hard-failed. Renders amber / `[HOLD]`.
    CalibrationRegion,
}

impl Verdict {
    /// Human-facing status label — distinct from the serde wire name (`CalibrationRegion`
    /// labels as `"HOLD"`).
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Pass => "Pass",
            Verdict::Fail => "Fail",
            Verdict::CalibrationRegion => "HOLD",
        }
    }

    /// ASCII status prefix for cli surfaces — pairs with color so status is never
    /// color-alone (CLAUDE.md universal invariant).
    pub fn status_prefix(&self) -> &'static str {
        match self {
            Verdict::Pass => "[PASS]",
            Verdict::Fail => "[FAIL]",
            Verdict::CalibrationRegion => "[HOLD]",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_canonical_names() {
        assert_eq!(serde_json::to_string(&Verdict::Pass).unwrap(), "\"Pass\"");
        assert_eq!(serde_json::to_string(&Verdict::Fail).unwrap(), "\"Fail\"");
        // Locks the canonical name — an accidental `rename_all` would break this.
        assert_eq!(
            serde_json::to_string(&Verdict::CalibrationRegion).unwrap(),
            "\"CalibrationRegion\""
        );
    }

    #[test]
    fn round_trips_through_json() {
        for v in [Verdict::Pass, Verdict::Fail, Verdict::CalibrationRegion] {
            let json = serde_json::to_string(&v).unwrap();
            let back: Verdict = serde_json::from_str(&json).unwrap();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn label_and_prefix_are_stable() {
        assert_eq!(Verdict::CalibrationRegion.label(), "HOLD");
        assert_eq!(Verdict::CalibrationRegion.status_prefix(), "[HOLD]");
        for v in [Verdict::Pass, Verdict::Fail, Verdict::CalibrationRegion] {
            assert!(!v.label().is_empty());
            assert!(v.status_prefix().starts_with('[') && v.status_prefix().ends_with(']'));
        }
    }
}
