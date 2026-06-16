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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured() -> RunRecord {
        RunRecord {
            journal_emitted_at: Some("2026-06-16T21:10:06Z".to_string()),
            read_back_observed_at: Some("2026-06-16T21:10:07Z".to_string()),
            run_id: "2026-06-16T21-10-06-abc".to_string(),
            seed: 424242,
            scenario: "error-baseline-spike".to_string(),
            p_ids: vec![PId("P-009".to_string()), PId("P-010".to_string())],
            verdict: Some(Verdict::Pass),
            state: ReportState::Pass,
            latency_ms: Some(1840),
            slo_tier: SloTier::Tier5s,
            fingerprints: Some(vec!["fp-1".to_string()]),
        }
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
