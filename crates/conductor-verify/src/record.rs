//! The producer bridge: a fully-evaluated [`CheckOutcome`] → the canonical run-report [`RunRecord`].
//!
//! [`CheckOutcome`] carries the classified verdict + journal-relative SLO timing; the run/scenario
//! identity, the wall-clock instants, and the observed fingerprints come from the run context the
//! outcome does not hold. The verdict drives the report `state` via
//! [`conductor_core::Verdict::default_report_state`]. Infallible — a value, never a `Result::Err`
//! (the verdict/error wall). `Assessment.observed`/`expected`/`delta` are Assessment-internal and
//! never reach the eleven-field envelope.

use conductor_core::{PId, RunRecord};

use crate::slo::CheckOutcome;

impl CheckOutcome {
    /// Assemble the canonical measured [`RunRecord`] from this outcome plus the run/scenario context.
    ///
    /// `state` is the verdict's [default mapping](conductor_core::Verdict::default_report_state); a
    /// producer needing a context-specific state (a `degraded_mode` `KnownResidual`) builds the
    /// record via [`RunRecord::measured`] directly. `latency_ms`/`slo_tier` come from this outcome's
    /// SLO timing; the caller supplies the identity, the two RFC-3339 instants, and the fingerprints.
    #[allow(clippy::too_many_arguments)] // assembles the eleven-field envelope (arch §Standard Contracts)
    pub fn to_run_record(
        &self,
        run_id: impl Into<String>,
        seed: u64,
        scenario: impl Into<String>,
        p_ids: Vec<PId>,
        journal_emitted_at: impl Into<String>,
        read_back_observed_at: impl Into<String>,
        fingerprints: Vec<String>,
    ) -> RunRecord {
        let verdict = self.assessment.verdict;
        RunRecord::measured(
            run_id,
            seed,
            scenario,
            p_ids,
            verdict,
            verdict.default_report_state(),
            journal_emitted_at,
            read_back_observed_at,
            self.slo.latency_ms,
            self.slo_tier,
            fingerprints,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slo::evaluate_check;
    use conductor_core::{ClaimClass, ComparisonKind, ExpectedCheck, ReportState, SloTier, Verdict};

    fn outcome(class: ClaimClass, observed: &str) -> CheckOutcome {
        let check = ExpectedCheck { kind: ComparisonKind::Exact, class, expected: "ok".to_string() };
        evaluate_check(&check, observed, SloTier::Tier5s, 0, 1_000)
    }

    fn record(class: ClaimClass, observed: &str) -> RunRecord {
        outcome(class, observed).to_run_record(
            "2026-06-16T21-10-06-abc",
            424242,
            "error-baseline-spike",
            vec![PId("P-009".to_string())],
            "2026-06-16T21:10:06Z",
            "2026-06-16T21:10:07Z",
            vec!["fp-1".to_string()],
        )
    }

    #[test]
    fn pass_outcome_maps_to_pass_state_and_carries_slo_timing() {
        let r = record(ClaimClass::Hard, "ok");
        assert_eq!(r.verdict, Some(Verdict::Pass));
        assert_eq!(r.state, ReportState::Pass);
        assert_eq!(r.latency_ms, Some(1_000));
        assert_eq!(r.slo_tier, SloTier::Tier5s);
        assert_eq!(r.journal_emitted_at.as_deref(), Some("2026-06-16T21:10:06Z"));
        assert_eq!(r.fingerprints, Some(vec!["fp-1".to_string()]));
    }

    #[test]
    fn fail_outcome_maps_to_fail_state() {
        let r = record(ClaimClass::Hard, "nope");
        assert_eq!(r.verdict, Some(Verdict::Fail));
        assert_eq!(r.state, ReportState::Fail);
    }

    #[test]
    fn calibration_region_outcome_maps_to_manual_check() {
        // a model-interpretive class routes to CalibrationRegion regardless of the comparison
        let r = record(ClaimClass::CalibrationRegion, "anything");
        assert_eq!(r.verdict, Some(Verdict::CalibrationRegion));
        assert_eq!(r.state, ReportState::ManualCheck);
    }

    #[test]
    fn bridged_record_serializes_to_exactly_the_eleven_owned_keys() {
        let r = record(ClaimClass::Hard, "ok");
        let obj: serde_json::Value = serde_json::to_value(&r).unwrap();
        let mut keys: Vec<&str> = obj.as_object().unwrap().keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "fingerprints", "journal_emitted_at", "latency_ms", "p_ids", "read_back_observed_at",
                "run_id", "scenario", "seed", "slo_tier", "state", "verdict",
            ]
        );
        // The exact eleven-key set proves the Assessment-internal fields (`observed`/`expected`/
        // `delta`) never reach the envelope — they are not among the keys.
    }

    #[test]
    fn bridging_is_deterministic() {
        assert_eq!(record(ClaimClass::Hard, "ok"), record(ClaimClass::Hard, "ok"));
    }
}
