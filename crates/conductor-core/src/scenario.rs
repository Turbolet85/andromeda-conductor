//! The core scenario *identity* model — the shared shape every seam matches on.
//!
//! Identity skeleton only: name, the Pulse P-ID(s) the scenario exercises, the seed, and its
//! SLO tier. The declarative per-phase emission spec is a later chunk; garde validation
//! (non-empty P-IDs, range / cross-field rules) attaches in the config-validation chunk —
//! here the structs derive serde only.

use serde::{Deserialize, Serialize};

/// A Pulse capability identifier (`P-001`..`P-060`). Serializes transparently as the bare
/// string (`"P-009"`). Format validation is deferred to the config-validation chunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PId(pub String);

/// The SLO timing tier a scenario's deadline is measured against. Closed set; the wire forms
/// are not Rust identifiers, so each variant is serde-renamed (arch §Data model conventions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SloTier {
    /// Hard deterministic tier — deadline under 5 seconds.
    #[serde(rename = "<5s")]
    Tier5s,
    /// Mid tier — deadline under 20 seconds.
    #[serde(rename = "<20s")]
    Tier20s,
    /// Slow tier — deadline under 90 seconds.
    #[serde(rename = "<90s")]
    Tier90s,
}

/// A scenario's identity. Every scenario carries at least one Pulse P-ID — the
/// "no scenario without a P-ID" law is expressed as the non-optional `p_ids` field
/// (non-emptiness is enforced by garde in the config-validation chunk).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scenario {
    /// Scenario name (e.g. `"error-baseline-spike"`).
    pub name: String,
    /// The Pulse capability P-ID(s) this scenario exercises. Required.
    pub p_ids: Vec<PId>,
    /// Deterministic seed — same scenario + seed yields the same emission-stream shape.
    pub seed: u64,
    /// The SLO timing tier this scenario's deadline is measured against.
    pub slo_tier: SloTier,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slo_tier_serializes_to_wire_forms() {
        assert_eq!(serde_json::to_string(&SloTier::Tier5s).unwrap(), "\"<5s\"");
        assert_eq!(serde_json::to_string(&SloTier::Tier20s).unwrap(), "\"<20s\"");
        assert_eq!(serde_json::to_string(&SloTier::Tier90s).unwrap(), "\"<90s\"");
    }

    #[test]
    fn scenario_round_trips_with_p_ids_as_bare_strings() {
        let s = Scenario {
            name: "error-baseline-spike".to_string(),
            p_ids: vec![PId("P-009".to_string()), PId("P-010".to_string())],
            seed: 424242,
            slo_tier: SloTier::Tier5s,
        };
        let json = serde_json::to_string(&s).unwrap();
        // PId is serde-transparent — the array is bare strings, not tagged objects.
        assert!(json.contains("[\"P-009\",\"P-010\"]"));
        let back: Scenario = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }
}
