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

use crate::expected::ExpectedCheck;
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

impl SloTier {
    /// The journal-relative deadline bound in milliseconds — the latency a scenario in this tier must
    /// meet. The tier *is* the tolerance band (architecture §Timing-Tolerance Model).
    pub fn deadline_ms(self) -> i64 {
        match self {
            SloTier::Tier5s => 5_000,
            SloTier::Tier20s => 20_000,
            SloTier::Tier90s => 90_000,
        }
    }
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
    /// The declarative per-scenario read-back checks the `conductor-verify` evaluator compares
    /// observed MCP read-back against. Optional in config (defaults to none — a drive+observe
    /// scenario may assert only via the operator checklist); each present check validates via `dive`.
    #[serde(default)]
    #[garde(dive)]
    pub expected: Vec<ExpectedCheck>,
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
    use crate::expected::{ClaimClass, ComparisonKind};
    use crate::phase_spec::{EmissionSpec, Signal, MAX_JITTER_MS};
    use garde::Validate;
    use rstest::rstest;

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
            expected: Vec::new(),
        }
    }

    #[test]
    fn slo_tier_serializes_to_wire_forms() {
        assert_eq!(serde_json::to_string(&SloTier::Tier5s).unwrap(), "\"<5s\"");
        assert_eq!(serde_json::to_string(&SloTier::Tier20s).unwrap(), "\"<20s\"");
        assert_eq!(serde_json::to_string(&SloTier::Tier90s).unwrap(), "\"<90s\"");
    }

    #[test]
    fn slo_tier_deadline_ms_maps_the_three_tiers() {
        assert_eq!(SloTier::Tier5s.deadline_ms(), 5_000);
        assert_eq!(SloTier::Tier20s.deadline_ms(), 20_000);
        assert_eq!(SloTier::Tier90s.deadline_ms(), 90_000);
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

    #[rstest]
    #[case("receiver-lifecycle-state", "P-001")]
    #[case("last-span-ago-tracking", "P-002")]
    #[case("receiver-failed-port-conflict", "P-003")]
    #[case("orthogonal-health-domains", "P-004")]
    fn connection_lifecycle_fixtures_load_and_validate(#[case] stem: &str, #[case] p_id: &str) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        assert_eq!(s.p_ids, vec![PId(p_id.to_string())]);
        assert!(!s.expected.is_empty(), "{stem} declares at least one expected check");
    }

    #[rstest]
    #[case("span-status-error-detection", "P-005")]
    #[case("exception-event-capture", "P-006")]
    #[case("high-severity-log-capture", "P-007")]
    #[case("root-span-error-scope", "P-008")]
    fn hard_signal_fixtures_load_and_validate(#[case] stem: &str, #[case] p_id: &str) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        assert_eq!(s.p_ids, vec![PId(p_id.to_string())]);
        assert!(!s.expected.is_empty(), "{stem} declares at least one expected check");
    }

    #[rstest]
    #[case("error-baseline-spike", &["P-009", "P-010"])]
    #[case("latency-regression", &["P-011", "P-012"])]
    fn statistical_anomaly_fixtures_load_and_validate(#[case] stem: &str, #[case] p_ids: &[&str]) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
        assert!(!s.expected.is_empty(), "{stem} declares at least one expected check");
    }

    #[rstest]
    #[case("error-baseline-spike")]
    #[case("latency-regression")]
    fn statistical_anomaly_checks_are_hard_with_floor_and_candidate(#[case] stem: &str) {
        // P-009..P-012 are deterministic baseline-math + threshold-detection, so every check is Hard
        // (no CalibrationRegion, unlike P-008). Each scenario pairs a CountAtLeast sample-count floor
        // (the baseline P-ID) with a Contains detection candidate (the detection P-ID).
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.iter().all(|c| c.class == ClaimClass::Hard),
            "{stem} checks are all Hard (deterministic baseline-math + threshold-detection)"
        );
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::CountAtLeast),
            "{stem} asserts the sample-count floor via CountAtLeast"
        );
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Contains),
            "{stem} asserts the detection candidate via Contains"
        );
    }

    #[test]
    fn p008_root_span_error_scope_checks_are_calibration_region() {
        // Root-vs-deep severity weighting is model-side (P-020) per the v2.1 amendment, so P-008's
        // check is calibration-region (a tendency), never a hard assert.
        let path =
            format!("{}/../../scenarios/root-span-error-scope.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected.iter().all(|c| c.class == ClaimClass::CalibrationRegion),
            "P-008 checks are calibration-region per the v2.1 amendment"
        );
    }

    #[test]
    fn p007_high_severity_log_capture_asserts_both_sides_of_the_boundary() {
        // The SeverityNumber 17 boundary is two-sided: ERROR/FATAL (>=17) contributes (Contains),
        // WARN-and-below (<17) does not (Absent).
        let path =
            format!("{}/../../scenarios/high-severity-log-capture.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Contains),
            "P-007 asserts the >=17 contribution via Contains"
        );
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Absent),
            "P-007 asserts the <17 non-contribution via Absent"
        );
    }

    #[rstest]
    #[case("activity-floor", &["P-013"])]
    #[case("service-went-silent", &["P-014"])]
    #[case("restart-suppression", &["P-015", "P-016", "P-057"])]
    fn activity_floor_and_restart_suppression_fixtures_load_and_validate(
        #[case] stem: &str,
        #[case] p_ids: &[&str],
    ) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
        assert!(!s.expected.is_empty(), "{stem} declares at least one expected check");
    }

    #[rstest]
    #[case("activity-floor")]
    #[case("service-went-silent")]
    #[case("restart-suppression")]
    fn activity_floor_and_restart_suppression_checks_are_all_hard(#[case] stem: &str) {
        // Activity-floor + restart-suppression are deterministic suppression/bypass + lifecycle
        // timing, so every check is Hard (arch §Probabilistic-Assertion Policy); the model-interpretive
        // severity that consumes these cues is P-020, a later severity-lifecycle chunk.
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.iter().all(|c| c.class == ClaimClass::Hard),
            "{stem} checks are all Hard (suppression/bypass logic + lifecycle timing)"
        );
    }

    #[test]
    fn activity_floor_asserts_the_learned_quiet_via_absent() {
        // P-013's false-positive guard is an ABSENCE — no ServiceWentSilent during the learned quiet
        // (the same kind P-007 uses for its <17 non-contribution); P-014's death cue is its presence
        // counterpart in a separate scenario, since Absent + Contains of one token cannot coexist.
        let path = format!("{}/../../scenarios/activity-floor.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Absent),
            "P-013 asserts the no-false-silent guard via Absent"
        );
    }

    #[rstest]
    #[case("service-went-silent")]
    #[case("restart-suppression")]
    fn presence_scenarios_assert_a_surfaced_incident_via_contains(#[case] stem: &str) {
        // P-014 (death cue) and P-015/P-016/P-057 (RestartEvent + surfaced ErrorRateSpike) are
        // presence checks; the suppressed legs are declare-only (Epoch-8 evaluator-owned).
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Contains),
            "{stem} asserts a surfaced incident via Contains"
        );
    }

    #[rstest]
    #[case("fingerprint-storm", &["P-017", "P-018"])]
    #[case("fingerprint-distinct", &["P-017"])]
    fn fingerprint_storm_fixtures_load_and_validate(#[case] stem: &str, #[case] p_ids: &[&str]) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
        assert!(!s.expected.is_empty(), "{stem} declares at least one expected check");
    }

    #[rstest]
    #[case("fingerprint-storm")]
    #[case("fingerprint-distinct")]
    fn fingerprint_storm_checks_are_all_hard(#[case] stem: &str) {
        // Fingerprint identity (deterministic hash) + storm-count detection are Hard; the
        // 6->Suggested/12->Autonomous severity escalation is model-side (P-020), declared-only.
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.iter().all(|c| c.class == ClaimClass::Hard),
            "{stem} checks are all Hard (fingerprint identity + storm-count detection)"
        );
    }

    #[test]
    fn fingerprint_storm_asserts_the_storm_via_contains() {
        // P-018: the same-fp triple stormed past the floor surfaces a RetryStorm (Contains); the
        // distinct-fp guard is its Absent counterpart in a separate scenario (one token, opposite outcome).
        let path = format!("{}/../../scenarios/fingerprint-storm.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected
                .iter()
                .any(|c| c.kind == ComparisonKind::Contains && c.expected == "RetryStorm"),
            "fingerprint-storm asserts the storm via Contains RetryStorm"
        );
    }

    #[test]
    fn fingerprint_distinct_asserts_no_aggregation_via_absent() {
        // P-017: type/frame variants are DISTINCT fingerprints, so sub-floor counts never aggregate
        // into a storm — the no-false-aggregation guard (the Absent side of the same RetryStorm token).
        let path = format!("{}/../../scenarios/fingerprint-distinct.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected
                .iter()
                .any(|c| c.kind == ComparisonKind::Absent && c.expected == "RetryStorm"),
            "fingerprint-distinct asserts no-aggregation via Absent RetryStorm"
        );
    }

    #[rstest]
    #[case("severity-tier-autonomous", &["P-019", "P-020", "P-060"])]
    #[case("severity-tier-suggested", &["P-019", "P-020", "P-021", "P-060"])]
    #[case("severity-tier-curious", &["P-019", "P-020", "P-021", "P-060"])]
    #[case("incident-auto-resolution", &["P-022", "P-059"])]
    #[case("ack-cooldown", &["P-023"])]
    fn severity_lifecycle_fixtures_load_and_validate(#[case] stem: &str, #[case] p_ids: &[&str]) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
        assert!(!s.expected.is_empty(), "{stem} declares at least one expected check");
    }

    #[rstest]
    #[case("severity-tier-autonomous", "Autonomous")]
    #[case("severity-tier-suggested", "Suggested")]
    #[case("severity-tier-curious", "Curious")]
    fn severity_tier_scenarios_assert_their_tier_as_calibration_region(
        #[case] stem: &str,
        #[case] tier: &str,
    ) {
        // P-019/P-020: the severity TIER choice is model-driven, so each tier scenario asserts its tier
        // token as a calibration-region tendency (Contains) routed to ManualCheck — never a hard match.
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.iter().all(|c| c.class == ClaimClass::CalibrationRegion),
            "{stem} severity-choice checks are calibration-region (model-driven per P-020)"
        );
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Contains && c.expected == tier),
            "{stem} asserts the {tier} tier via Contains"
        );
    }

    #[test]
    fn incident_auto_resolution_asserts_resolved_and_new_not_reopen_hard() {
        // P-022: the 120s auto-resolution transition (Contains "Resolved") and the new-not-reopen
        // (CountAtLeast 2 distinct incidents) are deterministic lifecycle timing -> Hard. P-059's
        // resolution-summary continuity is model-interpretive -> CalibrationRegion (so this file is mixed).
        let path =
            format!("{}/../../scenarios/incident-auto-resolution.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Contains
                && c.class == ClaimClass::Hard
                && c.expected == "Resolved"),
            "P-022 asserts the Resolved transition via Hard Contains"
        );
        assert!(
            s.expected
                .iter()
                .any(|c| c.kind == ComparisonKind::CountAtLeast && c.class == ClaimClass::Hard),
            "P-022 asserts new-not-reopen via Hard CountAtLeast"
        );
        assert!(
            s.expected.iter().any(|c| c.class == ClaimClass::CalibrationRegion),
            "P-059 interpretation continuity is calibration-region"
        );
    }

    #[test]
    fn ack_cooldown_asserts_new_incident_after_via_hard_count_at_least() {
        // P-023: only the drivable after-cool-down leg is asserted -> a new incident forms after the 5-min
        // window, so the run carries >= 2 incidents (CountAtLeast 2, Hard). The within-window suppression +
        // the ack mechanism are declare-only (Epoch-8), per the phase P4 Q2 decision.
        let path = format!("{}/../../scenarios/ack-cooldown.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected.iter().all(|c| c.class == ClaimClass::Hard),
            "ack-cooldown's asserted leg (after-window new incident) is Hard"
        );
        assert!(
            s.expected
                .iter()
                .any(|c| c.kind == ComparisonKind::CountAtLeast && c.expected == "2"),
            "P-023 asserts the after-cool-down new incident via CountAtLeast 2"
        );
    }

    #[test]
    fn severity_lifecycle_suite_is_mixed_class() {
        // The first mixed-class family: severity choice (P-019/P-020) is CalibrationRegion; lifecycle
        // timing (P-022/P-023) + tier routing (P-060) is Hard. Assert the SUITE exercises BOTH classes
        // (every prior family guard was all-Hard).
        let stems = [
            "severity-tier-autonomous",
            "severity-tier-suggested",
            "severity-tier-curious",
            "incident-auto-resolution",
            "ack-cooldown",
        ];
        let mut checks = Vec::new();
        for stem in stems {
            let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
            let toml =
                std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
            let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
            checks.extend(s.expected);
        }
        assert!(
            checks.iter().any(|c| c.class == ClaimClass::Hard),
            "the severity-lifecycle suite carries at least one Hard check"
        );
        assert!(
            checks.iter().any(|c| c.class == ClaimClass::CalibrationRegion),
            "the severity-lifecycle suite carries at least one CalibrationRegion check"
        );
    }

    #[test]
    fn from_toml_str_parses_an_expected_block() {
        let toml = r#"
name = "x"
p_ids = ["P-001"]
seed = 1
slo_tier = "<5s"
jitter_ms = 0

[[phases]]
name = "p"
gap_ms = 1

[[expected]]
kind = "Contains"
class = "Hard"
expected = "Receiving"
"#;
        let s = Scenario::from_toml_str(toml).expect("valid scenario");
        assert_eq!(s.expected.len(), 1);
        assert_eq!(s.expected[0].kind, ComparisonKind::Contains);
        assert_eq!(s.expected[0].class, ClaimClass::Hard);
        assert_eq!(s.expected[0].expected, "Receiving");
    }

    #[test]
    fn expected_defaults_to_empty_when_omitted() {
        // `#[serde(default)]` — a scenario without an `[[expected]]` block stays valid (and the
        // pre-existing error-baseline-spike.toml fixture keeps loading).
        let s = scenario_with(vec![PId("P-009".to_string())]);
        let json = serde_json::to_string(&s).unwrap();
        let back: Scenario = serde_json::from_str(&json).unwrap();
        assert!(back.expected.is_empty());
    }

    #[test]
    fn expected_checks_round_trip_through_json() {
        let mut s = scenario_with(vec![PId("P-001".to_string())]);
        s.expected = vec![ExpectedCheck {
            kind: ComparisonKind::Contains,
            class: ClaimClass::Hard,
            expected: "Receiving".to_string(),
        }];
        let json = serde_json::to_string(&s).unwrap();
        let back: Scenario = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    #[test]
    fn expected_check_with_empty_target_is_rejected_via_dive() {
        let mut s = scenario_with(vec![PId("P-001".to_string())]);
        s.expected = vec![ExpectedCheck {
            kind: ComparisonKind::Exact,
            class: ClaimClass::Hard,
            expected: String::new(),
        }];
        assert!(s.validate().is_err());
    }

    #[rstest]
    #[case("halo-hue-encoding", &["P-025"])]
    #[case("halo-breathing-encoding", &["P-026"])]
    #[case("service-constellation-discovery", &["P-027"])]
    #[case("project-context-grounding", &["P-032"])]
    #[case("cross-incident-recurrence", &["P-036"])]
    fn constellation_context_grounding_fixtures_load_and_validate(
        #[case] stem: &str,
        #[case] p_ids: &[&str],
    ) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
        // NB: unlike every prior family loader, this one does NOT assert `!expected.is_empty()` — the
        // operator-checklist / declare-only members carry an empty expected (asserted below).
    }

    #[rstest]
    #[case("halo-hue-encoding")]
    #[case("halo-breathing-encoding")]
    #[case("service-constellation-discovery")]
    #[case("project-context-grounding")]
    fn constellation_and_p032_are_operator_checklist_declare_only(#[case] stem: &str) {
        // The catalog's first operator-checklist / declare-only members. A DriveObserve constellation claim
        // (P-025/026/027) has NO programmatic read-back (the hue/breathing/dots are operator-verified), and
        // P-032's KnownResidual routing is Epoch-8-owned — both declare an EMPTY `expected`, which routes to
        // verdict None -> Lamp::Manual (the inversion of every prior family's "declares at least one check").
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.is_empty(),
            "{stem} is operator-checklist / declare-only (empty expected -> ManualCheck/KnownResidual downstream)"
        );
    }

    #[test]
    fn cross_incident_recurrence_asserts_previously_seen_via_hard_contains() {
        // P-036: a fingerprint recurring across two runs surfaces "Previously seen" from the runs.db
        // cross-run index — a deterministic index lookup -> Hard Contains (the one auto-assertable check in
        // the family). The "Previously seen" token is inferred (substring-tolerant), an Epoch-8 calibration point.
        let path =
            format!("{}/../../scenarios/cross-incident-recurrence.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Contains
                && c.class == ClaimClass::Hard
                && c.expected == "Previously seen"),
            "P-036 asserts the recurrence reference via Hard Contains \"Previously seen\""
        );
    }

    #[test]
    fn constellation_context_grounding_suite_has_operator_checklist_members() {
        // The first family that is NOT all-non-empty: the constellation trio + P-032 declare nothing
        // (-> ManualCheck/KnownResidual downstream) while P-036 carries a Hard check. Assert the SUITE
        // exercises BOTH shapes (every prior family had every scenario carry at least one expected check).
        let empty_stems = [
            "halo-hue-encoding",
            "halo-breathing-encoding",
            "service-constellation-discovery",
            "project-context-grounding",
        ];
        let has_empty = empty_stems.iter().any(|stem| {
            let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
            let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
            Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}")).expected.is_empty()
        });
        let recurrence_path =
            format!("{}/../../scenarios/cross-incident-recurrence.toml", env!("CARGO_MANIFEST_DIR"));
        let recurrence = Scenario::from_toml_str(
            &std::fs::read_to_string(&recurrence_path).expect("fixture readable"),
        )
        .expect("fixture valid");
        assert!(has_empty, "the suite carries operator-checklist (empty-expected) members");
        assert!(!recurrence.expected.is_empty(), "P-036 (cross-incident-recurrence) carries a Hard check");
    }

    #[rstest]
    #[case("pii-scrub", &["P-035", "P-047", "P-048"])]
    #[case("report-render-surface", &["P-037"])]
    #[case("findings-counter-refresh", &["P-045"])]
    #[case("cadence-config", &["P-052"])]
    #[case("threshold-hot-reload", &["P-055", "P-056"])]
    #[case("degraded-mode-report", &["P-053"])]
    fn scrub_pipeline_degraded_fixtures_load_and_validate(#[case] stem: &str, #[case] p_ids: &[&str]) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
        // Mixed-shape family (Hard + declare-only), so — like the constellation loader — no blanket
        // `!expected.is_empty()` assertion here; the per-shape guards below carry the precise checks.
    }

    #[rstest]
    #[case("report-render-surface")]
    #[case("cadence-config")]
    #[case("degraded-mode-report")]
    fn report_surface_cadence_degraded_are_declare_only(#[case] stem: &str) {
        // P-037 (operator-checklist render), P-052 (operator-set cadence, deferred measurement), and P-053
        // (degraded_mode KnownResidual — producer-assigned downstream) declare an EMPTY `expected`.
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.is_empty(),
            "{stem} is declare-only (empty expected -> ManualCheck/KnownResidual downstream)"
        );
    }

    #[test]
    fn pii_scrub_asserts_scrub_via_hard_absent_and_structure_via_contains() {
        // P-035/047/048: the seeded PiiCorpus carries stable category sentinels (`@example.com`, `sk_live_`,
        // `Bearer `, `password=`) that appear in read-back ONLY if scrubbing failed -> Hard Absent. A distinct
        // Contains token proves structure is preserved (P-035). All checks are Hard (deterministic scrub).
        let path = format!("{}/../../scenarios/pii-scrub.toml", env!("CARGO_MANIFEST_DIR"));
        let s = Scenario::from_toml_str(&std::fs::read_to_string(&path).expect("fixture readable"))
            .expect("fixture valid");
        assert!(
            s.expected.iter().all(|c| c.class == ClaimClass::Hard),
            "pii-scrub checks are all Hard (deterministic PII scrub)"
        );
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Absent),
            "pii-scrub asserts the raw-PII scrub via Absent sentinels"
        );
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Contains),
            "pii-scrub asserts structure preserved via a distinct Contains marker"
        );
        // Absent and Contains must never share a token (a same-token pair contradicts on one read-back).
        let absent: Vec<&str> = s
            .expected
            .iter()
            .filter(|c| c.kind == ComparisonKind::Absent)
            .map(|c| c.expected.as_str())
            .collect();
        assert!(
            s.expected
                .iter()
                .filter(|c| c.kind == ComparisonKind::Contains)
                .all(|c| !absent.contains(&c.expected.as_str())),
            "no Contains token equals an Absent token (they would contradict on one read-back)"
        );
    }

    #[test]
    fn findings_counter_and_threshold_reload_carry_their_hard_checks() {
        // P-045 findings counter = len(query_incident_list | unread/active) -> deterministic Hard CountAtLeast.
        let counter_path =
            format!("{}/../../scenarios/findings-counter-refresh.toml", env!("CARGO_MANIFEST_DIR"));
        let counter = Scenario::from_toml_str(
            &std::fs::read_to_string(&counter_path).expect("fixture readable"),
        )
        .expect("fixture valid");
        assert!(
            counter
                .expected
                .iter()
                .any(|c| c.kind == ComparisonKind::CountAtLeast && c.class == ClaimClass::Hard),
            "P-045 asserts the findings count via Hard CountAtLeast"
        );
        // P-056 prospective-only -> Hard Absent (the pre-change baseline raises no retroactive cue); the P-055
        // <2s hot-reload timing is the declare-only leg (Epoch-8 measurement, no content token).
        let reload_path =
            format!("{}/../../scenarios/threshold-hot-reload.toml", env!("CARGO_MANIFEST_DIR"));
        let reload = Scenario::from_toml_str(
            &std::fs::read_to_string(&reload_path).expect("fixture readable"),
        )
        .expect("fixture valid");
        assert!(
            reload
                .expected
                .iter()
                .any(|c| c.kind == ComparisonKind::Absent && c.class == ClaimClass::Hard),
            "P-056 asserts prospective-only application via Hard Absent (no retroactive cue)"
        );
    }

    #[test]
    fn scrub_pipeline_degraded_suite_mixes_hard_and_declare_only() {
        // Like the constellation family, this catalog family carries BOTH shapes: Hard auto members
        // (pii-scrub / findings-counter / threshold-hot-reload) and declare-only members (report-render /
        // cadence / degraded-mode). Assert the SUITE exercises both.
        let stems = [
            "pii-scrub",
            "report-render-surface",
            "findings-counter-refresh",
            "cadence-config",
            "threshold-hot-reload",
            "degraded-mode-report",
        ];
        let shapes: Vec<bool> = stems
            .iter()
            .map(|stem| {
                let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
                let toml = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
                Scenario::from_toml_str(&toml)
                    .unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"))
                    .expected
                    .is_empty()
            })
            .collect();
        assert!(shapes.iter().any(|empty| *empty), "the suite carries declare-only (empty-expected) members");
        assert!(shapes.iter().any(|empty| !*empty), "the suite carries Hard (non-empty-expected) members");
    }
}
