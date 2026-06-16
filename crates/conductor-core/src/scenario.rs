//! The core scenario model — the shared shape every seam matches on.
//!
//! Carries the scenario's identity (name, the Pulse P-ID(s) it exercises, the seed, its SLO tier)
//! plus the declarative per-phase emission spec: an ordered [`PhaseSpec`] sequence and the
//! scenario-level jitter bound the seeded timeline scheduler applies. garde validation attaches
//! here — non-empty `p_ids`, the `P-NNN` (001..=060) P-ID format, no-duplicate P-IDs, a non-empty
//! phase list (each phase validated via `dive`), and a bounded jitter — and the structs derive
//! both serde and [`garde::Validate`].

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::phase_spec::PhaseSpec;

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

/// A scenario: its identity plus its declarative per-phase emission spec.
///
/// Every scenario carries at least one Pulse P-ID — the "no scenario without a P-ID" law is the
/// non-optional `p_ids` field, enforced non-empty (and duplicate-free) by garde — and at least one
/// [`PhaseSpec`] in `phases`, the ordered sequence the timeline scheduler runs.
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
    /// The ordered per-phase emission spec — the declarative timeline the scheduler sequences.
    /// Required, non-empty; each phase is validated via `dive`.
    #[garde(length(min = 1), dive)]
    pub phases: Vec<PhaseSpec>,
    /// Symmetric per-gap jitter bound (milliseconds) the seeded scheduler may perturb each phase
    /// gap by; `0` means gaps land exactly as declared. Bounded by garde.
    #[garde(range(max = crate::phase_spec::MAX_JITTER_MS))]
    pub jitter_ms: u64,
}

impl Scenario {
    /// Parse and validate a scenario from a TOML document.
    ///
    /// Deserializes into a [`Scenario`], then runs garde validation. A TOML parse failure surfaces
    /// as [`CoreError::Config`](crate::CoreError::Config) and a validation failure as
    /// [`CoreError::Validation`](crate::CoreError::Validation) — both harness faults (`Err`), never
    /// a verdict, never a panic. The parse-error text is scrubbed (no host paths) so it is safe to
    /// surface or log.
    pub fn from_toml_str(toml: &str) -> crate::Result<Scenario> {
        let scenario: Scenario =
            toml::from_str(toml).map_err(|e| crate::CoreError::Config(crate::sanitize_error(&e)))?;
        scenario.validate()?;
        Ok(scenario)
    }
}

// garde 0.22.1 has no container-level `custom`, so this lives on the `p_ids` field it concerns.
// The latency-target ordering invariants (p50≤p95≤p99) join it when the Epoch-3 latency spec lands.
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
    use crate::phase_spec::{EmissionSpec, Signal, MAX_JITTER_MS};
    use garde::Validate;

    fn scenario_with(p_ids: Vec<PId>) -> Scenario {
        Scenario {
            name: "error-baseline-spike".to_string(),
            p_ids,
            seed: 424242,
            slo_tier: SloTier::Tier5s,
            phases: vec![PhaseSpec {
                name: "baseline".to_string(),
                gap_ms: 2000,
                emission: EmissionSpec::default(),
            }],
            jitter_ms: 50,
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

    #[test]
    fn empty_phases_is_rejected() {
        let mut s = scenario_with(vec![PId("P-009".to_string())]);
        s.phases = Vec::new();
        assert!(s.validate().is_err());
    }

    #[test]
    fn invalid_phase_is_rejected_via_dive() {
        let mut s = scenario_with(vec![PId("P-009".to_string())]);
        s.phases = vec![PhaseSpec {
            name: String::new(), // empty phase name fails PhaseSpec validation
            gap_ms: 100,
            emission: EmissionSpec::default(),
        }];
        assert!(s.validate().is_err());
    }

    #[test]
    fn over_bound_jitter_is_rejected() {
        let mut s = scenario_with(vec![PId("P-009".to_string())]);
        s.jitter_ms = MAX_JITTER_MS + 1;
        assert!(s.validate().is_err());
    }

    #[test]
    fn from_toml_str_parses_and_validates() {
        let toml = r#"
name = "error-baseline-spike"
p_ids = ["P-009", "P-010"]
seed = 424242
slo_tier = "<5s"
jitter_ms = 50

[[phases]]
name = "baseline"
gap_ms = 2000

[[phases]]
name = "spike"
gap_ms = 1000
"#;
        let s = Scenario::from_toml_str(toml).expect("valid scenario");
        assert_eq!(s.name, "error-baseline-spike");
        assert_eq!(s.phases.len(), 2);
        // `emission` is omitted in the document, so it defaults.
        assert_eq!(s.phases[0].emission.signal, Signal::Traces);
    }

    #[test]
    fn from_toml_str_rejects_malformed_toml_as_config_error() {
        let err = Scenario::from_toml_str("name = \"unterminated").unwrap_err();
        assert!(matches!(err, crate::CoreError::Config(_)));
    }

    #[test]
    fn from_toml_str_rejects_invalid_scenario_as_validation_error() {
        // Well-formed TOML, but empty p_ids fails garde validation.
        let toml = r#"
name = "x"
p_ids = []
seed = 1
slo_tier = "<5s"
jitter_ms = 0

[[phases]]
name = "p"
gap_ms = 1
"#;
        let err = Scenario::from_toml_str(toml).unwrap_err();
        assert!(matches!(err, crate::CoreError::Validation(_)));
    }

    #[test]
    fn committed_fixture_loads_and_validates() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../scenarios/error-baseline-spike.toml"
        );
        let toml = std::fs::read_to_string(path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert_eq!(s.name, "error-baseline-spike");
        assert!(!s.phases.is_empty());
    }
}
