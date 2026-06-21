//! The machine verdict for a verification check.

use serde::{Deserialize, Serialize};

use crate::ReportState;

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

    /// The default run-report state for the auto-verified path. `Pass`/`Fail` pass straight through;
    /// a `CalibrationRegion` ([HOLD]) check is auto-measured but model-interpretive, so it lands in
    /// [`ReportState::ManualCheck`] — terminal, awaiting a human (arch §Probabilistic-Assertion
    /// Policy). The envelope carries `verdict` and `state` independently, so a producer with a
    /// context-specific state (e.g. a `degraded_mode` residual) sets it directly; this is the default.
    pub fn default_report_state(self) -> ReportState {
        match self {
            Verdict::Pass => ReportState::Pass,
            Verdict::Fail => ReportState::Fail,
            Verdict::CalibrationRegion => ReportState::ManualCheck,
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

    #[test]
    fn default_report_state_maps_each_verdict() {
        use crate::ReportState;
        assert_eq!(Verdict::Pass.default_report_state(), ReportState::Pass);
        assert_eq!(Verdict::Fail.default_report_state(), ReportState::Fail);
        assert_eq!(
            Verdict::CalibrationRegion.default_report_state(),
            ReportState::ManualCheck
        );
    }
}
