//! The core scenario model — the shared shape every seam matches on.
//!
//! Carries the scenario's identity (name, the Pulse P-ID(s) it exercises, the seed, its SLO tier)
//! plus the declarative per-phase emission spec: an ordered [`PhaseSpec`] sequence and the
//! scenario-level jitter bound the seeded timeline scheduler applies. garde validation attaches
//! here — non-empty `p_ids`, the `P-NNN` P-ID shape, no-duplicate P-IDs, a non-empty phase list
//! (each phase validated via `dive`), and a bounded jitter — and the structs derive both serde and
//! [`garde::Validate`].
//!
//! Validation is two-layered: **garde checks the document's own shape**, and
//! [`Scenario::check_capabilities`] checks its P-IDs against the SUT's actual capability set (the
//! [`CapabilityManifest`](crate::CapabilityManifest)). The accepted set used to be the compile-time
//! range `001..=060` here; it is data now, so re-aiming Conductor at a newer Pulse is a manifest
//! edit. [`Scenario::from_toml_str_with`] is the production load path that applies both layers.

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::capability_manifest::CapabilityManifest;
use crate::expected::ExpectedCheck;
use crate::phase_spec::PhaseSpec;

/// A Pulse capability identifier (`P-NNN`). Serializes transparently as the bare string
/// (`"P-009"`); garde enforces the shape, the capability manifest enforces membership.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct PId(#[garde(custom(pid_format))] pub String);

/// Whether `value` has the `P-` + exactly-three-ASCII-digits shape. Shared with the capability
/// manifest's own bounds check so both sides agree on what a well-formed id looks like.
pub(crate) fn is_pid_shaped(value: &str) -> bool {
    value
        .strip_prefix("P-")
        .is_some_and(|rest| rest.len() == 3 && rest.bytes().all(|b| b.is_ascii_digit()))
}

/// garde field rule: accept the `P-NNN` shape. Whether that id is one the SUT actually claims is a
/// separate question, answered against the capability manifest at load — not by a range baked here.
fn pid_format(value: &str, _ctx: &()) -> garde::Result {
    if is_pid_shaped(value) {
        Ok(())
    } else {
        Err(garde::Error::new("expected P-NNN with NNN three ASCII digits"))
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
    /// meet. The tier *is* the tolerance band (architecture §Timing-Tolerance Model), and the floor a
    /// per-check `budget_ms` sits beneath. `const` so the budget ceiling derives from the ladder
    /// itself rather than re-pinning a literal.
    pub const fn deadline_ms(self) -> i64 {
        match self {
            SloTier::Tier5s => 5_000,
            SloTier::Tier20s => 20_000,
            SloTier::Tier90s => 90_000,
        }
    }
}

/// One operator-checklist item: what Conductor drove, and the observation the operator confirms.
///
/// The pair is a contract, not free prose (design-system §Surface: desktop-webview / Component
/// Patterns 7; a11y-plan §1 Critical paths requires each item expose both texts as accessible
/// content), so both halves are required and bounded — they render as one dialog row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct ChecklistItem {
    /// What Conductor drove — the induced state the observation is judged against.
    #[garde(length(min = 1, max = MAX_CHECKLIST_TEXT))]
    pub induced: String,
    /// The expected observation the operator confirms or declines.
    #[garde(length(min = 1, max = MAX_CHECKLIST_TEXT))]
    pub observation: String,
}

/// Upper bound on a [`ChecklistItem`] text half — each renders as one dialog row, so an unbounded
/// string is a render defect rather than a useful declaration.
pub const MAX_CHECKLIST_TEXT: usize = 200;

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
    #[garde(length(min = 1), dive, custom(crate::phase_spec::fault_phases_are_silent))]
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
    /// The operator-checklist items this scenario's hold renders. Optional (defaults to none — a
    /// scenario declaring none keeps the generic hold prompt); each item validates via `dive`.
    /// Only meaningful on a scenario that takes the operator-checklist path, which
    /// [`check_checklist`](Self::check_checklist) enforces against the `expected` sibling.
    #[serde(default)]
    #[garde(dive)]
    pub checklist: Vec<ChecklistItem>,
}

impl Scenario {
    /// Parse and shape-validate a scenario from a TOML document.
    ///
    /// Deserializes into a [`Scenario`], then runs garde validation. A TOML parse failure surfaces
    /// as [`CoreError::Config`](crate::CoreError::Config) and a validation failure as
    /// [`CoreError::Validation`](crate::CoreError::Validation) — both harness faults (`Err`), never
    /// a verdict, never a panic. The parse-error text is scrubbed (no host paths) so it is safe to
    /// surface or log.
    ///
    /// This checks the document against itself only. Every production load path uses
    /// [`from_toml_str_with`](Self::from_toml_str_with), which additionally checks each P-ID against
    /// the SUT capability manifest.
    pub fn from_toml_str(toml: &str) -> crate::Result<Scenario> {
        let scenario: Scenario =
            toml::from_str(toml).map_err(|e| crate::CoreError::Config(crate::sanitize_error(&e)))?;
        scenario.validate()?;
        scenario.check_budgets()?;
        scenario.check_checklist()?;
        Ok(scenario)
    }

    /// The production load path: shape-validate, then reject any P-ID the SUT does not claim.
    pub fn from_toml_str_with(
        toml: &str,
        capabilities: &CapabilityManifest,
    ) -> crate::Result<Scenario> {
        let scenario = Scenario::from_toml_str(toml)?;
        scenario.check_capabilities(capabilities)?;
        Ok(scenario)
    }

    /// Reject any `p_ids` entry absent from the SUT capability manifest. The error names the
    /// manifest and the Pulse release it was captured from, never a hardcoded range.
    pub fn check_capabilities(&self, capabilities: &CapabilityManifest) -> crate::Result<()> {
        for pid in &self.p_ids {
            if !capabilities.accepts(&pid.0) {
                return Err(crate::CoreError::Config(format!(
                    "scenario {:?}: P-ID {:?} is not in the capability manifest for Pulse {}",
                    self.name, pid.0, capabilities.sut_version
                )));
            }
        }
        Ok(())
    }

    /// Reject any per-check `budget_ms` that exceeds this scenario's own `slo_tier` deadline.
    ///
    /// A budget is declared BENEATH its tier, so one above it is incoherent — the check would be
    /// held to a looser bound than the tier it sits in. Raised at load as a harness fault, never a
    /// verdict (the verdict/error wall).
    ///
    /// Lives outside garde for the same reason [`check_capabilities`](Self::check_capabilities)
    /// does: garde 0.22.1's `custom` is field-level and receives only its own field, so a rule
    /// spanning `expected` and `slo_tier` has no field to sit on, and giving `Scenario` a garde
    /// `Context` would change its public `validate()` surface and every nested spec's along with it
    /// (architecture §Established Decisions [Accepted Capability Set]).
    pub fn check_budgets(&self) -> crate::Result<()> {
        let deadline_ms = self.slo_tier.deadline_ms();
        for (index, check) in self.expected.iter().enumerate() {
            let Some(budget_ms) = check.budget_ms else { continue };
            if i64::from(budget_ms) > deadline_ms {
                return Err(crate::CoreError::Config(format!(
                    "scenario {:?}: expected check {} declares budget_ms {} above its {:?} tier deadline of {}ms",
                    self.name, index, budget_ms, self.slo_tier, deadline_ms
                )));
            }
        }
        Ok(())
    }

    /// Reject a declared `checklist` on a scenario that also declares `expected` checks.
    ///
    /// A checklist item is rendered by the operator-checklist hold, which a scenario reaches only by
    /// declaring NO expected checks (`conductor-run`'s firing site keys on `expected.is_empty()`), so
    /// items on a checks-bearing scenario would never render — a silent no-op rather than a
    /// declaration. Raised at load as a harness fault, never a verdict.
    ///
    /// Lives outside garde for the same reason [`check_budgets`](Self::check_budgets) does: the rule
    /// spans `checklist` and its sibling `expected`, and garde 0.22.1's `custom` is field-level and
    /// receives only its own field (architecture §Established Decisions [Validation Library]).
    pub fn check_checklist(&self) -> crate::Result<()> {
        if !self.checklist.is_empty() && !self.expected.is_empty() {
            return Err(crate::CoreError::Config(format!(
                "scenario {:?}: declares {} checklist item(s) beside {} expected check(s) — the checklist renders only on the operator-checklist path, which a scenario declaring expected checks never takes",
                self.name,
                self.checklist.len(),
                self.expected.len()
            )));
        }
        Ok(())
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
    use crate::phase_spec::{
        EmissionShape, EmissionSpec, FaultKindSpec, FaultSpec, Signal, MAX_JITTER_MS,
    };
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
                fault: None,
            }],
            jitter_ms: 50,
            expected: Vec::new(),
            checklist: Vec::new(),
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

    fn test_manifest(capabilities: &[&str]) -> CapabilityManifest {
        CapabilityManifest {
            sut_version: "v0.3.0".to_string(),
            captured_at: "2026-08-08".to_string(),
            capabilities: capabilities.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn malformed_p_ids_fail_the_shape_check() {
        for bad in ["Q-001", "P-99", "P-0600", "p-001", "P-01a", "P-", ""] {
            let s = scenario_with(vec![PId(bad.to_string())]);
            assert!(s.validate().is_err(), "{bad} should fail the P-NNN shape check");
        }
    }

    #[test]
    fn well_shaped_p_ids_pass_the_shape_check_whatever_the_sut_claims() {
        // The shape layer no longer encodes a range: P-061 was hard-rejected until 0.2.0, and
        // P-099 / P-000 are well-shaped but outside the SUT set — membership is the manifest's job.
        for ok in ["P-001", "P-060", "P-061", "P-082", "P-099", "P-000"] {
            let s = scenario_with(vec![PId(ok.to_string())]);
            assert!(s.validate().is_ok(), "{ok} should pass the P-NNN shape check");
        }
    }

    #[test]
    fn capability_membership_is_checked_against_the_manifest() {
        let m = test_manifest(&["P-001", "P-061"]);
        assert!(scenario_with(vec![PId("P-061".to_string())]).check_capabilities(&m).is_ok());

        for absent in ["P-099", "P-000", "P-060"] {
            let s = scenario_with(vec![PId(absent.to_string())]);
            let err = s.check_capabilities(&m).unwrap_err().to_string();
            assert!(err.contains("capability manifest"), "must name the manifest: {err}");
            assert!(!err.contains("001..=060"), "must not name a hardcoded range: {err}");
        }
    }

    #[test]
    fn from_toml_str_with_accepts_an_in_manifest_id_and_rejects_an_absent_one() {
        let doc = |pid: &str| {
            format!(
                "name = \"m\"\np_ids = [\"{pid}\"]\nseed = 1\nslo_tier = \"<5s\"\njitter_ms = 0\n\
                 [[phases]]\nname = \"p1\"\ngap_ms = 100\n"
            )
        };
        let m = test_manifest(&["P-074"]);
        assert!(Scenario::from_toml_str_with(&doc("P-074"), &m).is_ok());
        assert!(Scenario::from_toml_str_with(&doc("P-001"), &m).is_err());
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
            fault: None,
        }];
        assert!(s.validate().is_err());
    }

    #[test]
    fn a_fault_phase_that_emits_is_rejected_at_scenario_level() {
        let mut s = scenario_with(vec![PId("P-003".to_string())]);
        s.phases[0].fault = Some(FaultSpec { kind: FaultKindSpec::PortOccupier });
        assert!(s.validate().is_err(), "the default one-plain-trace emission violates the silence invariant");
        s.phases[0].emission.occurrences = 0;
        assert!(s.validate().is_ok(), "a silent fault phase validates");
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
    fn connection_lifecycle_fixtures_are_declare_only_at_the_harvest_tier(
        #[case] stem: &str,
        #[case] p_id: &str,
    ) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        assert_eq!(s.p_ids, vec![PId(p_id.to_string())]);
        assert!(
            s.expected.is_empty(),
            "{stem} is declare-only — connection state reaches no MCP read-back surface (Pulse's \
             FSM surfaces only via the TauRPC resolver, the broadcast topic and tracing lines), so \
             the family's Contains checks were structurally ungradeable under deterministic L4; \
             the live claims grade at the harvest tier (conductor-run/tests/connection_harvest.rs)"
        );
    }

    /// The port-conflict scenario is the catalog's one fault-declaring member: the occupier rides
    /// the port-held phase, and every phase is a silence window (the choreography emits nothing —
    /// during the hold nothing real listens on the egress target).
    #[test]
    fn receiver_failed_port_conflict_declares_the_occupier_fault() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../scenarios/receiver-failed-port-conflict.toml"
        );
        let toml = std::fs::read_to_string(path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        let fault_phases: Vec<_> = s.phases.iter().filter(|p| p.fault.is_some()).collect();
        assert_eq!(fault_phases.len(), 1, "exactly one fault-declaring phase");
        assert_eq!(fault_phases[0].name, "port-held");
        assert_eq!(fault_phases[0].fault, Some(FaultSpec { kind: FaultKindSpec::PortOccupier }));
        assert!(s.phases.iter().all(|p| p.emission.occurrences == 0), "every phase is silent");
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
        assert!(
            s.expected.is_empty(),
            "{stem} is declare-only — its read-back checks retired because no surface can carry \
             them under deterministic L4; the live assertions grade at the harvest tier \
             (conductor-run/tests/baseline_harvest.rs)"
        );
    }

    #[rstest]
    #[case("error-baseline-spike")]
    #[case("latency-regression")]
    fn statistical_anomaly_fixtures_are_declare_only_at_the_harvest_tier(#[case] stem: &str) {
        // The prior all-Hard floor+candidate checks were structurally ungradeable: CountAtLeast read
        // an evidence count no producer populates, and Contains graded text that cannot carry a cue
        // kind on any read-back surface. Both retired to declare-only; the family's live assertions
        // moved to Pulse's own `triage.cue.emit` line. The tier is pinned at the re-declared <90s —
        // whole-run latency spans the emission window and exceeds every tier by construction, so
        // <90s is the closest honest bucket (per-check latency is v2-19's).
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(s.expected.is_empty(), "{stem} carries no gradeable read-back check");
        assert_eq!(s.slo_tier, SloTier::Tier90s, "{stem} pins the re-declared tier");
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
    fn activity_floor_and_silence_fixtures_load_and_validate(
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
    fn activity_floor_and_silence_checks_are_all_hard(#[case] stem: &str) {
        // Activity-floor + service-went-silent are deterministic lifecycle timing, so every check is
        // Hard (arch §Probabilistic-Assertion Policy); the model-interpretive severity that consumes
        // these cues is P-020, a later severity-lifecycle chunk. restart-suppression left this set
        // 2026-08-18 — declare-only at the harvest tier (its own test below).
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.iter().all(|c| c.class == ClaimClass::Hard),
            "{stem} checks are all Hard (lifecycle timing)"
        );
    }

    /// Re-shaped 2026-08-18 against the SUT's measured semantics at HEAD `efabe8e`:
    /// `persistence_seconds` is the service's cumulative SAMPLE COUNT
    /// (`andromeda-pulse crates/triage/src/cue/evaluate.rs:55`), not spike duration, and no report
    /// branch renders a cue kind under deterministic L4 — so both Contains checks retired to
    /// declare-only and the live assertions grade at the harvest tier
    /// (`conductor-run/tests/restart_harvest.rs`). The tier is the re-declared <90s (whole-run
    /// latency spans the ~180s emission window; the v2-11/v2-12 honesty-bucket precedent).
    #[test]
    fn restart_suppression_is_declare_only_at_the_harvest_tier() {
        let path = format!("{}/../../scenarios/restart-suppression.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).expect("restart-suppression.toml readable");
        let s = Scenario::from_toml_str(&toml).expect("restart-suppression.toml valid");
        assert_eq!(s.name, "restart-suppression");
        let want: Vec<PId> = ["P-015", "P-016", "P-057"].iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
        assert!(s.expected.is_empty(), "restart-suppression carries no gradeable read-back check");
        assert_eq!(s.slo_tier, SloTier::Tier90s, "restart-suppression pins the re-declared tier");
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
    fn presence_scenarios_assert_a_surfaced_incident_via_contains(#[case] stem: &str) {
        // P-014 (death cue) is a presence check. restart-suppression left this set 2026-08-18: its
        // Contains tokens were structurally ungradeable under deterministic L4 — declare-only at
        // the harvest tier (restart_suppression_is_declare_only_at_the_harvest_tier).
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::Contains),
            "{stem} asserts a surfaced incident via Contains"
        );
    }

    #[rstest]
    #[case("fingerprint-storm", &["P-017", "P-018", "P-074"])]
    #[case("fingerprint-distinct", &["P-017"])]
    fn fingerprint_storm_fixtures_load_and_validate(#[case] stem: &str, #[case] p_ids: &[&str]) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
    }

    /// Both fingerprint scenarios are DECLARE-ONLY as of the 2026-08-16 live legs: `retrieve_report`
    /// returns `degraded_mode: true` permanently under deterministic L4, so a read-back token check is
    /// structurally ungradeable — the `Absent` side would even pass vacuously. The assertions moved to
    /// Pulse's own `triage.pattern.storm.detected` lines (`conductor-run/tests/storm_harvest.rs`), and
    /// an empty `expected` correctly routes these to ManualCheck rather than a false green.
    #[rstest]
    #[case("fingerprint-storm")]
    #[case("fingerprint-distinct")]
    fn fingerprint_scenarios_are_declare_only_after_the_degraded_read_back_finding(#[case] stem: &str) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.is_empty(),
            "{stem} declares no read-back check — the harvest surface carries them"
        );
    }

    /// The re-calibration the live legs forced: `latency_ms` spans the scenario's own emission window,
    /// which is ~24s (storm) and ~30s (distinct), so `<20s` was unattainable by construction.
    #[rstest]
    #[case("fingerprint-storm")]
    #[case("fingerprint-distinct")]
    fn fingerprint_scenarios_carry_the_recalibrated_tier(#[case] stem: &str) {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.slo_tier, SloTier::Tier90s, "{stem} re-calibrated to the <90s tier");
    }

    fn severity_fixture(stem: &str) -> Scenario {
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"))
    }

    #[rstest]
    #[case("severity-tier-autonomous", &["P-019", "P-020", "P-060"])]
    #[case("severity-tier-suggested", &["P-019", "P-020", "P-021", "P-060"])]
    #[case("severity-tier-curious", &["P-019", "P-020", "P-021", "P-060"])]
    #[case("incident-auto-resolution", &["P-022", "P-059"])]
    #[case("ack-cooldown", &["P-023"])]
    fn severity_lifecycle_fixtures_load_and_validate(#[case] stem: &str, #[case] p_ids: &[&str]) {
        let s = severity_fixture(stem);
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
    }

    /// The whole family is DECLARE-ONLY as of the 2026-08-21 live legs, on grounds measured per file:
    /// the deterministic-L4 fixture pins ONE severity for every incident and no corpus tool renders a
    /// tier word (the three tier files); `query_incident_list` returns the ACTIVE set only, so a resolved
    /// incident leaves the surface rather than arriving with a status token, and `CountAtLeast` grades
    /// `span_refs` that Pulse's incident producer writes empty (`incident-auto-resolution`); no ack tool
    /// exists in the four-tool contract (`ack-cooldown`). The claims moved to Pulse's own ledger lines
    /// (`conductor-run/tests/severity_harvest.rs`); an empty `expected` routes these to ManualCheck
    /// rather than a false green. Superseded: the mixed-class suite guard — the suite now carries no
    /// checks at all, which is the property worth pinning.
    #[rstest]
    #[case("severity-tier-autonomous")]
    #[case("severity-tier-suggested")]
    #[case("severity-tier-curious")]
    #[case("incident-auto-resolution")]
    #[case("ack-cooldown")]
    fn severity_lifecycle_family_is_declare_only(#[case] stem: &str) {
        assert!(
            severity_fixture(stem).expected.is_empty(),
            "{stem} declares no read-back check — the harvest surface carries them"
        );
    }

    /// Each tier file reaches its band through the SUT's `classify_priority` gates, and the term that
    /// separates the three is CONFIDENCE = samples/100: Autonomous needs >= 0.9, Suggested >= 0.7, and
    /// Curious is what remains once a cue fires at all (the fire condition is already magnitude >= 3x).
    /// So the baseline occurrence count IS the tier selector — pin it, or a later edit "simplifying" the
    /// baselines away silently collapses all three files onto the same band.
    #[rstest]
    #[case("severity-tier-autonomous", 90)]
    #[case("severity-tier-suggested", 70)]
    #[case("severity-tier-curious", 20)]
    fn severity_tier_baselines_select_their_confidence_band(#[case] stem: &str, #[case] samples: u32) {
        let s = severity_fixture(stem);
        let baseline = &s.phases[0];
        assert_eq!(
            baseline.emission.occurrences, samples,
            "{stem}'s baseline buys the sample count its band needs"
        );
        assert_eq!(
            baseline.emission.shape,
            EmissionShape::Plain,
            "{stem}'s baseline is OK spans — an error there would move the very rate the spike measures"
        );
        let spike = s.phases.last().expect("a tier scenario carries a spike phase");
        assert!(
            matches!(spike.emission.shape, EmissionShape::Error { error_percent: 100, .. }),
            "{stem}'s spike is unambiguous errors; its band comes from the sample count, not a partial rate"
        );
    }

    /// The two properties that make the auto-resolve leg REACHABLE, both measured live: the storm phases
    /// must clear Pulse's Autonomous threshold (>= 10 same fingerprints in the 60s window — 8 sat in the
    /// dead band and formed nothing), and a dilution tail of OK spans must follow the storm so the
    /// SAMPLE-driven 30s error EWMA falls under the cue threshold before the silence starts. Without the
    /// tail the EWMA freezes high and every tick re-fires a cue that refreshes `updated_at`, so the 120s
    /// idle window never elapses and auto-resolve cannot happen at all.
    #[test]
    fn incident_auto_resolution_drives_a_reachable_lifecycle() {
        let s = severity_fixture("incident-auto-resolution");
        let storms: Vec<_> = s
            .phases
            .iter()
            .filter(|p| matches!(p.emission.shape, EmissionShape::Exception { .. }))
            .collect();
        assert_eq!(storms.len(), 2, "a trigger storm and a retrigger storm");
        for p in &storms {
            assert!(
                p.emission.occurrences >= 12,
                "{} clears the Autonomous storm threshold (>= 10)",
                p.name
            );
        }
        let dilution = s
            .phases
            .iter()
            .find(|p| p.emission.shape == EmissionShape::Plain && p.emission.occurrences > 0)
            .expect("a dilution tail of OK spans follows the trigger storm");
        assert!(
            dilution.emission.occurrences >= 120,
            "the tail decays the 30s EWMA (alpha 0.0333) under the 3x threshold before silence"
        );
        let wait = s
            .phases
            .iter()
            .filter(|p| p.emission.occurrences == 0)
            .map(|p| p.gap_ms)
            .max()
            .expect("a silence window");
        assert!(
            wait >= 150_000,
            "the idle wait outlasts the 120s no-reemission window plus the 30s observer tick"
        );
    }

    /// `ack-cooldown` keeps its five phases as the record of the intended drive shape even though no
    /// acknowledge tool exists to drive them — the connection-family precedent. Pinning the phase names
    /// keeps that record from being quietly deleted as dead weight while P-023 is still classified Auto.
    #[test]
    fn ack_cooldown_keeps_its_drive_shape_on_record() {
        let s = severity_fixture("ack-cooldown");
        let names: Vec<&str> = s.phases.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "trigger-incident",
                "acknowledge",
                "retrigger-within-cooldown",
                "wait-past-cooldown",
                "retrigger-after-cooldown"
            ]
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
            budget_ms: None,
        }];
        let json = serde_json::to_string(&s).unwrap();
        let back: Scenario = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    /// A scenario at `tier` whose single check declares `budget_ms`.
    fn scenario_budgeted(tier: SloTier, budget_ms: Option<u32>) -> Scenario {
        let mut s = scenario_with(vec![PId("P-001".to_string())]);
        s.slo_tier = tier;
        s.expected = vec![ExpectedCheck {
            kind: ComparisonKind::Contains,
            class: ClaimClass::Hard,
            expected: "Receiving".to_string(),
            budget_ms,
        }];
        s
    }

    #[rstest]
    #[case(SloTier::Tier5s, Some(4_999))]
    #[case(SloTier::Tier5s, Some(5_000))] // the boundary is coherent — beneath means ≤
    #[case(SloTier::Tier20s, Some(2_000))]
    #[case(SloTier::Tier90s, Some(1))]
    #[case(SloTier::Tier5s, None)] // undeclared inherits the tier
    fn a_budget_at_or_under_its_tier_is_accepted(
        #[case] tier: SloTier,
        #[case] budget_ms: Option<u32>,
    ) {
        assert!(scenario_budgeted(tier, budget_ms).check_budgets().is_ok());
    }

    #[rstest]
    #[case(SloTier::Tier5s, 5_001)]
    #[case(SloTier::Tier5s, 20_000)]
    #[case(SloTier::Tier20s, 20_001)]
    #[case(SloTier::Tier90s, 90_001)]
    fn a_budget_above_its_tier_is_a_harness_fault_naming_the_check(
        #[case] tier: SloTier,
        #[case] budget_ms: u32,
    ) {
        let err = scenario_budgeted(tier, Some(budget_ms)).check_budgets().unwrap_err();
        assert!(matches!(err, crate::CoreError::Config(_)), "a harness fault, never a verdict");
        let msg = err.to_string();
        assert!(msg.contains("check 0"), "names the offending check: {msg}");
        assert!(msg.contains(&budget_ms.to_string()), "names the declared budget: {msg}");
    }

    #[test]
    fn the_budget_rule_runs_at_the_toml_load_path() {
        // The rule sits outside garde (it spans `expected` and `slo_tier`), so its ONLY guard is
        // `from_toml_str` calling it — this is what proves the load path actually applies it.
        let toml = r#"
name = "budget-over-tier"
p_ids = ["P-001"]
seed = 1
slo_tier = "<5s"
jitter_ms = 0

[[phases]]
name = "p1"
gap_ms = 1000

[[expected]]
kind = "Contains"
class = "Hard"
expected = "Receiving"
budget_ms = 9000
"#;
        let err = Scenario::from_toml_str(toml).unwrap_err();
        assert!(matches!(err, crate::CoreError::Config(_)));
        assert!(err.to_string().contains("9000"), "{err}");
    }

    #[test]
    fn a_within_tier_budget_survives_the_toml_round_trip() {
        let toml = r#"
name = "budget-ok"
p_ids = ["P-001"]
seed = 1
slo_tier = "<20s"
jitter_ms = 0

[[phases]]
name = "p1"
gap_ms = 1000

[[expected]]
kind = "Contains"
class = "Hard"
expected = "Receiving"
budget_ms = 2000

[[expected]]
kind = "Absent"
class = "Hard"
expected = "WARN"
"#;
        let s = Scenario::from_toml_str(toml).expect("a budget beneath its tier loads");
        assert_eq!(s.expected[0].budget_ms, Some(2_000));
        assert_eq!(s.expected[1].budget_ms, None, "an undeclared budget stays absent");
        assert_eq!(s.expected[0].effective_deadline_ms(s.slo_tier), 2_000);
        assert_eq!(s.expected[1].effective_deadline_ms(s.slo_tier), s.slo_tier.deadline_ms());
    }

    #[test]
    fn the_committed_catalog_declares_budgets_within_its_tiers() {
        // Every shipped scenario must still load — the field is additive, so this is the guard that
        // adding it broke none of them.
        for path in crate::scenario_files(std::path::Path::new("../../scenarios")).unwrap() {
            let toml = std::fs::read_to_string(&path).unwrap();
            let s = Scenario::from_toml_str(&toml)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            for check in &s.expected {
                assert!(
                    check.effective_deadline_ms(s.slo_tier) <= s.slo_tier.deadline_ms(),
                    "{}: a check's effective deadline escaped its tier",
                    s.name
                );
            }
        }
    }

    #[test]
    fn expected_check_with_empty_target_is_rejected_via_dive() {
        let mut s = scenario_with(vec![PId("P-001".to_string())]);
        s.expected = vec![ExpectedCheck {
            kind: ComparisonKind::Exact,
            class: ClaimClass::Hard,
            expected: String::new(),
            budget_ms: None,
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
    fn pii_scrub_is_declare_only_with_the_measurement_recorded() {
        // Retired 2026-08-19 (run 2026-08-19T20-37-25-933): no MCP read-back surface varies with the
        // emitted payload under deterministic L4, so the four Absent sentinels passed VACUOUSLY and the
        // Contains structure marker failed STRUCTURALLY — the checks graded nothing (the
        // fingerprint-storm vacuous-green precedent). The live claim grades at the harvest tier
        // (conductor-run/tests/pii_harvest.rs); scrub semantics are byte-verified at SUT source.
        let path = format!("{}/../../scenarios/pii-scrub.toml", env!("CARGO_MANIFEST_DIR"));
        let s = Scenario::from_toml_str(&std::fs::read_to_string(&path).expect("fixture readable"))
            .expect("fixture valid");
        assert!(
            s.expected.is_empty(),
            "pii-scrub is declare-only (empty expected -> ManualCheck/KnownResidual downstream)"
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
        // (findings-counter / threshold-hot-reload) and declare-only members (pii-scrub — retired
        // 2026-08-19, measurement in its TOML header — / report-render / cadence / degraded-mode).
        // Assert the SUITE exercises both.
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

    #[rstest]
    #[case("live-only-service-truth", &["P-067"])]
    #[case("investigate-actions-functional", &["P-072"])]
    #[case("constellation-severity-live-wiring", &["P-079"])]
    fn in_lane_sut_fixtures_load_and_validate(#[case] stem: &str, #[case] p_ids: &[&str]) {
        // The catalog's first entries above P-060 — expressible only because the accepted set is manifest
        // data rather than the superseded compile-time bound.
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert_eq!(s.name, stem);
        let want: Vec<PId> = p_ids.iter().map(|p| PId(p.to_string())).collect();
        assert_eq!(s.p_ids, want);
    }

    #[rstest]
    #[case("live-only-service-truth")]
    #[case("investigate-actions-functional")]
    fn in_lane_drive_observe_members_are_operator_checklist(#[case] stem: &str) {
        // P-067's registry render and P-072's Investigate action are both DriveObserve: no programmatic
        // read-back, and triggering the action is Pulse UI (a standing non-goal). Empty expected → verdict
        // None → Lamp::Manual.
        let path = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{stem}.toml readable: {e}"));
        let s = Scenario::from_toml_str(&toml).unwrap_or_else(|e| panic!("{stem}.toml valid: {e}"));
        assert!(
            s.expected.is_empty(),
            "{stem} is drive+observe (empty expected -> ManualCheck downstream)"
        );
    }

    #[test]
    fn constellation_severity_live_wiring_asserts_incident_visibility_via_hard_count_floor() {
        // P-079: incidents are filtered by the workspace key, so a diverged app/sidecar key returns zero
        // rows. The floor asserts the storm's incident is visible through read-back AT ALL — deliberately
        // not a candidate token, which belongs to fingerprint-storm (one outcome per token).
        let path = format!(
            "{}/../../scenarios/constellation-severity-live-wiring.toml",
            env!("CARGO_MANIFEST_DIR")
        );
        let toml = std::fs::read_to_string(&path).expect("fixture readable");
        let s = Scenario::from_toml_str(&toml).expect("fixture valid");
        assert!(
            s.expected.iter().any(|c| c.kind == ComparisonKind::CountAtLeast
                && c.class == ClaimClass::Hard
                && c.expected == "1"),
            "P-079 asserts incident visibility via a Hard CountAtLeast \"1\""
        );
    }

    #[test]
    fn in_lane_sut_suite_mixes_operator_checklist_and_auto() {
        // The in-lane family spans two coverage modes — two DriveObserve members and one Auto member — so
        // the suite must exercise both shapes, like the constellation and scrub families before it.
        let stems = [
            "live-only-service-truth",
            "investigate-actions-functional",
            "constellation-severity-live-wiring",
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
        assert!(shapes.iter().any(|empty| *empty), "the suite carries drive+observe members");
        assert!(shapes.iter().any(|empty| !*empty), "the suite carries an auto member");
    }
}
