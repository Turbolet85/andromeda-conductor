//! The run-report state for a verification check.

use serde::{Deserialize, Serialize};

/// The terminal state of a check in the run report.
///
/// Distinct from [`crate::Verdict`] (the machine verdict): `ReportState` is the five-way
/// classification the Markdown report + `runs.db` row carry (arch §Standard Contracts) — the
/// two are never flattened into one enum. Serializes to its canonical PascalCase name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportState {
    /// Auto-verified check passed. `[PASS]`.
    Pass,
    /// Auto-verified check failed. `[FAIL]`.
    Fail,
    /// Terminal, awaits a human go/no-go: either an auto-measured but model-interpretive
    /// (calibration-region) check, or a visual/operator-checklist claim with no programmatic
    /// read-back. `[MANUAL]`.
    ManualCheck,
    /// Deviation matches a pre-accepted residual (e.g. a `degraded_mode` report),
    /// distinguished from a real `Fail`. `[RESIDUAL]`.
    KnownResidual,
    /// A preflight precondition failed (protocol mismatch / missing tool / empty canary) —
    /// the scenario was never measured. `[BLOCKED]`.
    Blocked,
}

impl ReportState {
    /// Human-facing status label.
    pub fn label(&self) -> &'static str {
        match self {
            ReportState::Pass => "Pass",
            ReportState::Fail => "Fail",
            ReportState::ManualCheck => "Manual",
            ReportState::KnownResidual => "Residual",
            ReportState::Blocked => "Blocked",
        }
    }

    /// ASCII status prefix for cli surfaces — pairs with color so status is never
    /// color-alone (CLAUDE.md universal invariant).
    pub fn status_prefix(&self) -> &'static str {
        match self {
            ReportState::Pass => "[PASS]",
            ReportState::Fail => "[FAIL]",
            ReportState::ManualCheck => "[MANUAL]",
            ReportState::KnownResidual => "[RESIDUAL]",
            ReportState::Blocked => "[BLOCKED]",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_canonical_names() {
        assert_eq!(serde_json::to_string(&ReportState::Pass).unwrap(), "\"Pass\"");
        assert_eq!(
            serde_json::to_string(&ReportState::ManualCheck).unwrap(),
            "\"ManualCheck\""
        );
        assert_eq!(
            serde_json::to_string(&ReportState::KnownResidual).unwrap(),
            "\"KnownResidual\""
        );
        assert_eq!(
            serde_json::to_string(&ReportState::Blocked).unwrap(),
            "\"Blocked\""
        );
    }

    #[test]
    fn round_trips_through_json() {
        for s in [
            ReportState::Pass,
            ReportState::Fail,
            ReportState::ManualCheck,
            ReportState::KnownResidual,
            ReportState::Blocked,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let back: ReportState = serde_json::from_str(&json).unwrap();
            assert_eq!(s, back);
        }
    }

    #[test]
    fn prefixes_cover_all_five_states() {
        assert_eq!(ReportState::Pass.status_prefix(), "[PASS]");
        assert_eq!(ReportState::Fail.status_prefix(), "[FAIL]");
        assert_eq!(ReportState::ManualCheck.status_prefix(), "[MANUAL]");
        assert_eq!(ReportState::KnownResidual.status_prefix(), "[RESIDUAL]");
        assert_eq!(ReportState::Blocked.status_prefix(), "[BLOCKED]");
    }
}
