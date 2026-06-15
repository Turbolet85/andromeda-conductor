//! The core scenario *identity* model — the shared shape every seam matches on.
//!
//! Identity skeleton only: name, the Pulse P-ID(s) the scenario exercises, the seed, and its
//! SLO tier. The declarative per-phase emission spec is a later chunk. garde validation attaches
//! here: non-empty `p_ids`, the `P-NNN` (001..=060) P-ID format, and a no-duplicate-P-IDs
//! cross-cutting rule — the structs derive both serde and [`garde::Validate`].

use garde::Validate;
use serde::{Deserialize, Serialize};

/// A Pulse capability identifier (`P-001`..`P-060`). Serializes transparently as the bare
/// string (`"P-009"`); garde enforces the `P-NNN` shape with `NNN` in `001..=060`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct PId(#[garde(custom(pid_format))] pub String);

/// garde field rule: accept `P-` followed by exactly three ASCII digits whose value is `1..=60`.
fn pid_format(value: &str, _ctx: &()) -> garde::Result {
    let in_range = value
        .strip_prefix("P-")
        .filter(|rest| rest.len() == 3 && rest.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|rest| rest.parse::<u16>().ok())
        .is_some_and(|n| (1..=60).contains(&n));
    if in_range {
        Ok(())
    } else {
        Err(garde::Error::new("expected P-NNN with NNN in 001..=060"))
    }
}

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
/// "no scenario without a P-ID" law is the non-optional `p_ids` field, enforced non-empty
/// (and free of duplicates) by garde.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct Scenario {
    /// Scenario name (e.g. `"error-baseline-spike"`).
    #[garde(length(min = 1))]
    pub name: String,
    /// The Pulse capability P-ID(s) this scenario exercises. Required, non-empty, each validated,
    /// and free of duplicates.
    #[garde(length(min = 1), dive, custom(no_duplicate_pids))]
    pub p_ids: Vec<PId>,
    /// Deterministic seed — same scenario + seed yields the same emission-stream shape.
    #[garde(skip)]
    pub seed: u64,
    /// The SLO timing tier this scenario's deadline is measured against.
    #[garde(skip)]
    pub slo_tier: SloTier,
}

// garde 0.22.1 has no container-level `custom`, so this lives on the `p_ids` field it concerns —
// the worked `custom` validator the Epoch-2 emission-spec invariants (p50≤p95≤p99, …) will sit beside.
fn no_duplicate_pids(p_ids: &[PId], _ctx: &()) -> garde::Result {
    let mut seen = std::collections::HashSet::with_capacity(p_ids.len());
    for pid in p_ids {
        if !seen.insert(pid.0.as_str()) {
            return Err(garde::Error::new(format!("duplicate P-ID: {}", pid.0)));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use garde::Validate;

    fn scenario_with(p_ids: Vec<PId>) -> Scenario {
        Scenario {
            name: "error-baseline-spike".to_string(),
            p_ids,
            seed: 424242,
            slo_tier: SloTier::Tier5s,
        }
    }

    #[test]
    fn slo_tier_serializes_to_wire_forms() {
        assert_eq!(serde_json::to_string(&SloTier::Tier5s).unwrap(), "\"<5s\"");
        assert_eq!(serde_json::to_string(&SloTier::Tier20s).unwrap(), "\"<20s\"");
        assert_eq!(serde_json::to_string(&SloTier::Tier90s).unwrap(), "\"<90s\"");
    }

    #[test]
    fn scenario_round_trips_with_p_ids_as_bare_strings() {
        let s = scenario_with(vec![PId("P-009".to_string()), PId("P-010".to_string())]);
        let json = serde_json::to_string(&s).unwrap();
        // PId is serde-transparent — the array is bare strings, not tagged objects.
        assert!(json.contains("[\"P-009\",\"P-010\"]"));
        let back: Scenario = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    #[test]
    fn well_formed_scenario_validates() {
        let s = scenario_with(vec![PId("P-009".to_string()), PId("P-010".to_string())]);
        assert!(s.validate().is_ok());
    }

    #[test]
    fn empty_p_ids_is_rejected() {
        assert!(scenario_with(vec![]).validate().is_err());
    }

    #[test]
    fn empty_name_is_rejected() {
        let mut s = scenario_with(vec![PId("P-001".to_string())]);
        s.name = String::new();
        assert!(s.validate().is_err());
    }

    #[test]
    fn malformed_or_out_of_range_p_ids_are_rejected() {
        for bad in ["Q-001", "P-99", "P-099", "P-000", "P-0600", "P-061", "p-001", "P-01a"] {
            let s = scenario_with(vec![PId(bad.to_string())]);
            assert!(s.validate().is_err(), "{bad} should fail P-ID validation");
        }
    }

    #[test]
    fn boundary_p_ids_are_accepted() {
        for ok in ["P-001", "P-009", "P-060"] {
            let s = scenario_with(vec![PId(ok.to_string())]);
            assert!(s.validate().is_ok(), "{ok} should pass P-ID validation");
        }
    }

    #[test]
    fn duplicate_p_ids_are_rejected() {
        let s = scenario_with(vec![PId("P-009".to_string()), PId("P-009".to_string())]);
        assert!(s.validate().is_err());
    }
}
