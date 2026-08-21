//! The run-report envelope — one JSONL emission-journal record per scenario check.

use serde::{Deserialize, Serialize};

use crate::{PId, ReportState, SloTier, Verdict};

/// One scenario-result record in the per-run JSONL emission journal (`runs/<run_id>.jsonl`).
///
/// The agent-parseable ground truth the journal-relative SLO math reads — `latency_ms` is
/// `read_back_observed_at − journal_emitted_at`, both raw instants carried so the journal is
/// self-verifying. The schema is owned by tests/obs (test-plan §3 / obs-plan §3): exactly these
/// eleven fields, serialized in this declaration order. The five measurement-dependent fields are
/// `Option` so a [`RunRecord::blocked`] row emits them as JSON `null` (arch §Standard Contracts),
/// distinguishing a never-measured scenario from a measured one. The timestamps are RFC-3339 from
/// `std::time` ([`crate::now_rfc3339`]), never tokio's virtual clock.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunRecord {
    /// Wall-clock instant the scenario's signal was emitted (RFC-3339 `…Z`); `None` for a blocked
    /// row that was never measured.
    pub journal_emitted_at: Option<String>,
    /// Wall-clock instant read-back observed the scenario's reaction (RFC-3339 `…Z`); `None` until
    /// read-back, and for a blocked row. `latency_ms` is `read_back_observed_at − journal_emitted_at`.
    pub read_back_observed_at: Option<String>,
    /// The run this record belongs to — the filesystem-safe `runs/<run_id>.jsonl` stem.
    pub run_id: String,
    /// The scenario's deterministic seed.
    pub seed: u64,
    /// The scenario name.
    pub scenario: String,
    /// The Pulse capability P-ID(s) the scenario exercises.
    pub p_ids: Vec<PId>,
    /// The machine verdict; `None` for a blocked row.
    pub verdict: Option<Verdict>,
    /// The terminal run-report state — always present (`Blocked` for a blocked row).
    pub state: ReportState,
    /// Journal-relative latency in milliseconds; `None` for a blocked row.
    pub latency_ms: Option<i64>,
    /// The SLO tier the scenario's deadline is measured against.
    pub slo_tier: SloTier,
    /// Fingerprints observed; `None` for a blocked row (excluded from the Epoch-6 recurrence query),
    /// `Some([])` when measured with none.
    pub fingerprints: Option<Vec<String>>,
}

impl RunRecord {
    /// Construct a **blocked** record — a preflight precondition failed, so the scenario was never
    /// measured. Only the identity + `slo_tier` are populated; `journal_emitted_at` /
    /// `read_back_observed_at` / `verdict` / `latency_ms` / `fingerprints` are `None` (serialize as
    /// JSON `null`) and `state` is [`ReportState::Blocked`] (arch §Standard Contracts blocked-row
    /// shape).
    pub fn blocked(
        run_id: impl Into<String>,
        seed: u64,
        scenario: impl Into<String>,
        p_ids: Vec<PId>,
        slo_tier: SloTier,
    ) -> Self {
        Self {
            journal_emitted_at: None,
            read_back_observed_at: None,
            run_id: run_id.into(),
            seed,
            scenario: scenario.into(),
            p_ids,
            verdict: None,
            state: ReportState::Blocked,
            latency_ms: None,
            slo_tier,
            fingerprints: None,
        }
    }

    /// Construct a **measured** record — the scenario was driven and read-back observed, so every
    /// field is populated (the five measurement fields are `Some`, the inverse of
    /// [`RunRecord::blocked`]). `verdict` and `state` are supplied independently (the envelope never
    /// flattens them — a producer on the auto-verified path derives `state` from
    /// [`Verdict::default_report_state`]). `latency_ms` is the journal-relative
    /// `read_back_observed_at − journal_emitted_at` already in milliseconds — not re-derived from the
    /// second-precision instant strings.
    #[allow(clippy::too_many_arguments)] // eleven fields by contract (arch §Standard Contracts)
    pub fn measured(
        run_id: impl Into<String>,
        seed: u64,
        scenario: impl Into<String>,
        p_ids: Vec<PId>,
        verdict: Verdict,
        state: ReportState,
        journal_emitted_at: impl Into<String>,
        read_back_observed_at: impl Into<String>,
        latency_ms: i64,
        slo_tier: SloTier,
        fingerprints: Vec<String>,
    ) -> Self {
        Self {
            journal_emitted_at: Some(journal_emitted_at.into()),
            read_back_observed_at: Some(read_back_observed_at.into()),
            run_id: run_id.into(),
            seed,
            scenario: scenario.into(),
            p_ids,
            verdict: Some(verdict),
            state,
            latency_ms: Some(latency_ms),
            slo_tier,
            fingerprints: Some(fingerprints),
        }
    }
}

/// One expected-check outcome inside a scenario's run — the per-check grain beside the
/// scenario-level [`RunRecord`].
///
/// A scenario's row keeps the worst check's verdict (the lamp is chosen verdict-first), which used
/// to be the ONLY outcome that survived: the run seam graded every check and then discarded all but
/// the most severe. This record is where the rest land, so a reader can see that check 1 failed
/// while check 2 passed, and against which bound each was judged.
///
/// `latency_ms` is the same journal-relative measurement the scenario row carries, because the
/// corpus is observed ONCE per scenario and every check grades a projection of that one observation
/// — so `deadline_ms` is what separates two checks' timing verdicts, never the latency.
///
/// Only measured scenarios produce these: a blocked row and a declare-only scenario (zero
/// `[[expected]]`) emit none at all, never a synthesized value for something never measured
/// (arch §Standard Contracts).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckRecord {
    /// The run this check belongs to — the `runs/<run_id>.jsonl` stem.
    pub run_id: String,
    /// The scenario that declared the check.
    pub scenario: String,
    /// The check's ordinal position in the scenario's `expected` list — its stable key, since an
    /// `ExpectedCheck` carries no name.
    pub check_index: usize,
    /// The comparison the check applied — serialized to its canonical wire name.
    pub kind: crate::ComparisonKind,
    /// The machine verdict for this check alone.
    pub verdict: Verdict,
    /// This check's terminal report state.
    pub state: ReportState,
    /// Journal-relative latency in milliseconds — shared with the scenario row by construction.
    pub latency_ms: i64,
    /// The deadline this check was actually judged against: its own `budget_ms` when declared, else
    /// its scenario tier's.
    pub deadline_ms: i64,
    /// The budget the check declared, or `None` when it inherits its scenario's tier.
    pub budget_ms: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured() -> RunRecord {
        RunRecord::measured(
            "2026-06-16T21-10-06-abc",
            424242,
            "error-baseline-spike",
            vec![PId("P-009".to_string()), PId("P-010".to_string())],
            Verdict::Pass,
            ReportState::Pass,
            "2026-06-16T21:10:06Z",
            "2026-06-16T21:10:07Z",
            1840,
            SloTier::Tier5s,
            vec!["fp-1".to_string()],
        )
    }

    #[test]
    fn measured_record_serializes_in_canonical_schema_order() {
        // Golden: locks the eleven-field order + canonical enum spellings + bare-string p_ids +
        // the `<5s` slo_tier wire form (test-plan §3 / obs-plan §3).
        let json = serde_json::to_string(&measured()).unwrap();
        let expected = r#"{"journal_emitted_at":"2026-06-16T21:10:06Z","read_back_observed_at":"2026-06-16T21:10:07Z","run_id":"2026-06-16T21-10-06-abc","seed":424242,"scenario":"error-baseline-spike","p_ids":["P-009","P-010"],"verdict":"Pass","state":"Pass","latency_ms":1840,"slo_tier":"<5s","fingerprints":["fp-1"]}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn measured_record_populates_the_five_measurement_fields() {
        // Inverse of the blocked-row null rule: a measured record nulls nothing.
        let v: serde_json::Value = serde_json::to_value(measured()).unwrap();
        assert!(!v["journal_emitted_at"].is_null());
        assert!(!v["read_back_observed_at"].is_null());
        assert!(!v["verdict"].is_null());
        assert!(!v["latency_ms"].is_null());
        assert!(!v["fingerprints"].is_null());
        assert_eq!(v["state"], serde_json::json!("Pass"));
    }

    #[test]
    fn blocked_record_nulls_the_five_measurement_fields() {
        let r = RunRecord::blocked(
            "2026-06-16T21-10-06-xyz",
            7,
            "port-occupier",
            vec![PId("P-003".to_string())],
            SloTier::Tier20s,
        );
        let v: serde_json::Value = serde_json::to_value(&r).unwrap();
        assert!(v["journal_emitted_at"].is_null());
        assert!(v["read_back_observed_at"].is_null());
        assert!(v["verdict"].is_null());
        assert!(v["latency_ms"].is_null());
        assert!(v["fingerprints"].is_null());
        assert_eq!(v["state"], serde_json::json!("Blocked"));
        assert_eq!(v["run_id"], serde_json::json!("2026-06-16T21-10-06-xyz"));
        assert_eq!(v["seed"], serde_json::json!(7));
        assert_eq!(v["scenario"], serde_json::json!("port-occupier"));
        assert_eq!(v["p_ids"], serde_json::json!(["P-003"]));
        assert_eq!(v["slo_tier"], serde_json::json!("<20s"));
    }

    #[test]
    fn round_trips_through_json() {
        for r in [
            measured(),
            RunRecord::blocked("R", 1, "s", vec![PId("P-001".to_string())], SloTier::Tier90s),
        ] {
            let json = serde_json::to_string(&r).unwrap();
            let back: RunRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(r, back);
        }
    }

    #[test]
    fn serialization_is_deterministic() {
        let r = measured();
        assert_eq!(serde_json::to_string(&r).unwrap(), serde_json::to_string(&r).unwrap());
    }
}
