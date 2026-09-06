//! The unified status lamp — the verdict-first projection of a check's outcome.
//!
//! A run-report check carries two independent fields ([`Verdict`] and [`ReportState`]); the lamp is
//! the single six-way status the report surfaces render. It is chosen **verdict-first**: a
//! [`Verdict::CalibrationRegion`] check renders [`Lamp::Hold`], never [`Lamp::Manual`], even though
//! its default state is [`ReportState::ManualCheck`] (arch §Probabilistic-Assertion Policy). The two
//! states the verdict cannot express — [`ReportState::KnownResidual`] and [`ReportState::Blocked`] —
//! are state-driven and take precedence over the verdict, so an accepted residual is never mis-shown
//! as the raw [`Lamp::Fail`]. One source of lamp truth, reused by the Markdown report, coverage
//! matrix, cli, and desktop.

use crate::{ReportState, RunRecord, Verdict};

/// The six-way status lamp a report surface renders for a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lamp {
    /// Deterministic pass. `[PASS]`.
    Pass,
    /// Deterministic fail. `[FAIL]`.
    Fail,
    /// Model-interpretive calibration-region check, reported for a human. `[HOLD]`.
    Hold,
    /// Operator-checklist claim with no programmatic verdict. `[MANUAL]`.
    Manual,
    /// Deviation matched a pre-accepted residual. `[RESIDUAL]`.
    Residual,
    /// A preflight precondition failed; the check was never measured. `[BLOCKED]`.
    Blocked,
}

impl Lamp {
    /// The verdict-first lamp for a check (arch §Probabilistic-Assertion Policy).
    ///
    /// `Blocked`/`KnownResidual` are state-driven and checked first: a measured `KnownResidual` row
    /// still carries a `verdict`, which must not override the accepted-residual signal. Otherwise the
    /// verdict drives (`CalibrationRegion → Hold`); a verdict-less check falls back to its state.
    pub fn for_record(record: &RunRecord) -> Lamp {
        match (record.state, record.verdict) {
            (ReportState::Blocked, _) => Lamp::Blocked,
            (ReportState::KnownResidual, _) => Lamp::Residual,
            (_, Some(Verdict::Pass)) => Lamp::Pass,
            (_, Some(Verdict::Fail)) => Lamp::Fail,
            (_, Some(Verdict::CalibrationRegion)) => Lamp::Hold,
            (ReportState::ManualCheck, None) => Lamp::Manual,
            (ReportState::Pass, None) => Lamp::Pass,
            (ReportState::Fail, None) => Lamp::Fail,
        }
    }

    /// ASCII status prefix — the status encoding where there is no color (cli, Markdown); pairs with
    /// color elsewhere so status is never color-alone (CLAUDE.md universal invariant).
    pub fn status_prefix(&self) -> &'static str {
        match self {
            Lamp::Pass => "[PASS]",
            Lamp::Fail => "[FAIL]",
            Lamp::Hold => "[HOLD]",
            Lamp::Manual => "[MANUAL]",
            Lamp::Residual => "[RESIDUAL]",
            Lamp::Blocked => "[BLOCKED]",
        }
    }

    /// Human-facing status label (matches the [`Verdict`] / [`ReportState`] label spellings).
    pub fn label(&self) -> &'static str {
        match self {
            Lamp::Pass => "Pass",
            Lamp::Fail => "Fail",
            Lamp::Hold => "HOLD",
            Lamp::Manual => "Manual",
            Lamp::Residual => "Residual",
            Lamp::Blocked => "Blocked",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PId, SloTier};

    fn rec(verdict: Option<Verdict>, state: ReportState) -> RunRecord {
        RunRecord {
            journal_emitted_at: None,
            read_back_observed_at: None,
            run_id: "R".to_string(),
            seed: 1,
            scenario: "s".to_string(),
            p_ids: vec![PId("P-001".to_string())],
            verdict,
            state,
            latency_ms: None,
            slo_tier: SloTier::Tier5s,
            fingerprints: None,
        }
    }

    #[test]
    fn verdict_first_maps_pass_fail_hold() {
        assert_eq!(
            Lamp::for_record(&rec(Some(Verdict::Pass), ReportState::Pass)),
            Lamp::Pass
        );
        assert_eq!(
            Lamp::for_record(&rec(Some(Verdict::Fail), ReportState::Fail)),
            Lamp::Fail
        );
        // The rule: a CalibrationRegion check (default state ManualCheck) renders HOLD, not Manual.
        assert_eq!(
            Lamp::for_record(&rec(
                Some(Verdict::CalibrationRegion),
                ReportState::ManualCheck
            )),
            Lamp::Hold
        );
    }

    #[test]
    fn known_residual_and_blocked_are_state_driven_over_verdict() {
        // A measured residual carries a verdict; the accepted-residual signal must still win.
        assert_eq!(
            Lamp::for_record(&rec(Some(Verdict::Fail), ReportState::KnownResidual)),
            Lamp::Residual
        );
        assert_eq!(
            Lamp::for_record(&rec(Some(Verdict::Pass), ReportState::KnownResidual)),
            Lamp::Residual
        );
        assert_eq!(
            Lamp::for_record(&rec(None, ReportState::Blocked)),
            Lamp::Blocked
        );
    }

    #[test]
    fn verdict_less_manual_check_is_manual_not_hold() {
        assert_eq!(
            Lamp::for_record(&rec(None, ReportState::ManualCheck)),
            Lamp::Manual
        );
    }

    #[test]
    fn prefix_and_label_cover_all_six() {
        for (lamp, prefix, label) in [
            (Lamp::Pass, "[PASS]", "Pass"),
            (Lamp::Fail, "[FAIL]", "Fail"),
            (Lamp::Hold, "[HOLD]", "HOLD"),
            (Lamp::Manual, "[MANUAL]", "Manual"),
            (Lamp::Residual, "[RESIDUAL]", "Residual"),
            (Lamp::Blocked, "[BLOCKED]", "Blocked"),
        ] {
            assert_eq!(lamp.status_prefix(), prefix);
            assert_eq!(lamp.label(), label);
        }
    }

    #[test]
    fn blocked_constructor_lamps_blocked() {
        let r = RunRecord::blocked(
            "R",
            1,
            "s",
            vec![PId("P-003".to_string())],
            SloTier::Tier20s,
        );
        assert_eq!(Lamp::for_record(&r), Lamp::Blocked);
    }
}
