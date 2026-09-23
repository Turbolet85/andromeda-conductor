//! The live-Pulse precondition probe — whether a leg against a real Pulse can mean anything at all
//! (arch §Standard Contracts; security-plan §Security Anti-Patterns).
//!
//! Evaluation is PURE, exactly as [`RunContract::evaluate`](crate::RunContract::evaluate) is: the
//! caller observes the three subjects and passes them in as values, so the seam never reads the
//! process environment, never opens a socket and never touches the filesystem. Both arms of every
//! subject are therefore reachable in a test with no env mutation (which edition 2024 makes
//! `unsafe`) and no dependence on the host that happens to be running it.
//!
//! This is NOT the readiness gate. It runs BEFORE a leg is scheduled and emits no telemetry, so it
//! primes none of the SUT state a preflight canary would (arch §Established Decisions
//! [Read-Back Dependency Posture] — Pulse dedupes a new incident against any open one on the
//! `(kind, scope, scope_id)` tuple). It reports; it never launches, binds or spawns.
//!
//! A probe result is a VALUE, never a verdict: no `Verdict`, no `ReportState`, and no `Blocked` row
//! for a scenario that was never run (arch §Cross-cutting Patterns "Verdict/error wall").

use std::collections::BTreeSet;
use std::fmt;

use serde::Serialize;

use crate::L4Posture;

/// The Pulse-side environment handles whose declaration a live leg depends on.
///
/// Held here rather than at the observation site so the evaluator, the observer and the pin test
/// read one list. Distinct from [`RunContract::observed_env`](crate::RunContract::observed_env),
/// which yields only the handles a shell TERM (`shell-declaration` or `shell-absence`) names — a
/// narrower set by design.
pub const OBSERVED_HANDLES: [&str; 3] = [
    "ANDROMEDA_PULSE_DATA_DIR",
    "ANDROMEDA_PULSE_L4_DETERMINISTIC",
    "ANDROMEDA_PULSE_MCP_ENABLED",
];

/// The one member of [`OBSERVED_HANDLES`] whose value is a filesystem PATH rather than a flag.
///
/// It is graded by PRESENCE because a path can never read `"true"`: grading it for truthiness made
/// `handles-declared` unsatisfiable under every environment, and `agent-run boot` short-circuited
/// before every preflight from `480bc66` until this was fixed.
const PATH_VALUED_HANDLE: &str = "ANDROMEDA_PULSE_DATA_DIR";

/// The member of [`OBSERVED_HANDLES`] that selects Pulse's canned L4 mode — declared under the
/// deterministic posture, required ABSENT or falsy under the real-model one.
const L4_HANDLE: &str = "ANDROMEDA_PULSE_L4_DETERMINISTIC";

/// Whether a FLAG-valued handle carries an affirmative declaration. Presence alone is not enough —
/// an explicit `false` declares the opposite of the term it would otherwise satisfy.
///
/// The single definition of truthiness in the workspace: `conductor-run`'s run-contract reader
/// delegates its value test here, so a `shell-declaration` term and a probe flag handle cannot
/// drift apart. It takes a value and NO name, which is what keeps the run-contract path off
/// [`handle_declared`]'s presence arm by construction rather than by discipline.
pub fn flag_declared(value: Option<&str>) -> bool {
    value.is_some_and(|v| {
        let v = v.trim().to_ascii_lowercase();
        v == "true" || v == "1"
    })
}

/// Whether an observed handle is declared, graded by the KIND of value it carries.
///
/// `ANDROMEDA_PULSE_DATA_DIR` is declared when present and non-empty after trim; EVERY other name —
/// including one absent from [`OBSERVED_HANDLES`] — is graded by [`flag_declared`], so a handle
/// added without a grading decision fails closed rather than being satisfied by mere presence
/// (security-plan §Input Validation: never defaulted, never silently widened).
pub fn handle_declared(name: &str, value: Option<&str>) -> bool {
    if name == PATH_VALUED_HANDLE {
        return value.is_some_and(|v| !v.trim().is_empty());
    }
    flag_declared(value)
}

/// Whether a flag handle reads truthy on EITHER side of the boundary: Conductor's own rule
/// ([`flag_declared`]) or the one Pulse applies to its deterministic-L4 handle — trimmed,
/// lowercased, one of `1` / `true` / `yes` (andromeda-pulse `deterministic_mode_enabled_for`,
/// `pulse-app/src/deterministic_inference.rs:86-89`, HEAD `83d4060`).
///
/// The real-model posture reads a handle for ABSENCE under this rule, so a value Pulse would honour
/// as ON (`yes`) can never pass as absent while Conductor's own rule reads it off. It widens nothing
/// a `shell-declaration` term or a probe flag reads — those stay on [`flag_declared`].
pub fn flag_declared_on_either_side(value: Option<&str>) -> bool {
    flag_declared(value)
        || value
            .is_some_and(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
}

/// Whether an observed handle is declared, graded for the probe's posture. The one
/// posture-dependent arm is the L4 handle under [`L4Posture::RealModel`], read for absence and so
/// graded by [`flag_declared_on_either_side`]; every other handle and posture is
/// [`handle_declared`] unchanged.
pub fn handle_declared_for(name: &str, value: Option<&str>, posture: L4Posture) -> bool {
    if posture == L4Posture::RealModel && name == L4_HANDLE {
        return flag_declared_on_either_side(value);
    }
    handle_declared(name, value)
}

/// The fixed OTLP ingest target a live Pulse owns. Named for the operator-facing statement only —
/// the connect itself belongs to `conductor-emit`.
const EGRESS_TARGET: &str = "127.0.0.1:4317";

/// The MCP sidecar's program name, as it must resolve on the inherited `PATH`.
const SIDECAR_PROGRAM: &str = "andromeda-pulse-mcp";

/// Which precondition a finding is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PreconditionSubject {
    /// Something is listening on Pulse's loopback OTLP ingest.
    EgressReachable,
    /// The MCP sidecar resolves on the inherited `PATH`.
    SidecarResolvable,
    /// The Pulse-side environment handles are declared in Conductor's own environment.
    HandlesDeclared,
}

impl PreconditionSubject {
    /// The stable, operator-facing id — kebab-case, matching the run contract's term-id style.
    pub fn id(&self) -> &'static str {
        match self {
            Self::EgressReachable => "egress-reachable",
            Self::SidecarResolvable => "sidecar-resolvable",
            Self::HandlesDeclared => "handles-declared",
        }
    }
}

impl fmt::Display for PreconditionSubject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

/// What the caller observed. Every field is a plain value — the observation happens at the caller,
/// never here.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreconditionObservation {
    /// Whether a client could connect to the OTLP ingest target.
    pub egress_reachable: bool,
    /// Whether the sidecar's fixed program name resolved on the inherited `PATH`.
    pub sidecar_resolved: bool,
    /// The subset of [`OBSERVED_HANDLES`] the caller found affirmatively declared, each graded by
    /// [`handle_declared_for`] under the posture the probe runs for.
    pub declared: BTreeSet<String>,
}

impl PreconditionObservation {
    /// The handles `posture` requires declared that this observation did NOT find declared, in
    /// [`OBSERVED_HANDLES`] order.
    fn undeclared(&self, posture: L4Posture) -> Vec<&'static str> {
        OBSERVED_HANDLES
            .iter()
            .copied()
            .filter(|h| !must_be_absent(h, posture))
            .filter(|h| !self.declared.contains(*h))
            .collect()
    }

    /// The handles `posture` requires ABSENT that this observation found declared.
    fn declared_against(&self, posture: L4Posture) -> Vec<&'static str> {
        OBSERVED_HANDLES
            .iter()
            .copied()
            .filter(|h| must_be_absent(h, posture))
            .filter(|h| self.declared.contains(*h))
            .collect()
    }
}

/// Whether `posture` requires `handle` absent or falsy rather than declared.
fn must_be_absent(handle: &str, posture: L4Posture) -> bool {
    posture == L4Posture::RealModel && handle == L4_HANDLE
}

/// A precondition the probe found unmet.
///
/// `causes` names what could produce the miss rather than asserting a fact Conductor cannot
/// measure — the shape the readiness gate's workspace-key and run-contract arms already use. Both
/// strings are host-path-free by construction: no field here is ever built from a resolved path,
/// a `PATH` value, or a data dir (security-plan §Security Anti-Patterns → Logging).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnmetPrecondition {
    pub subject: PreconditionSubject,
    pub statement: String,
    pub causes: String,
}

/// The probe's standing against one observation — a value on the `Ok` path, never an error.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreconditionsStatus {
    unmet: Vec<UnmetPrecondition>,
}

impl PreconditionsStatus {
    /// Build a status directly from unmet preconditions (the test seam).
    pub fn from_unmet(unmet: Vec<UnmetPrecondition>) -> Self {
        Self { unmet }
    }

    pub fn is_satisfied(&self) -> bool {
        self.unmet.is_empty()
    }

    /// The unmet preconditions, in subject order — the probe names each one individually.
    pub fn unmet(&self) -> &[UnmetPrecondition] {
        &self.unmet
    }

    /// Whether a given subject is among the unmet.
    pub fn is_unmet(&self, subject: PreconditionSubject) -> bool {
        self.unmet.iter().any(|u| u.subject == subject)
    }
}

/// The probe's evaluation seam.
pub struct Preconditions;

impl Preconditions {
    /// Judge an observation under the deterministic posture — every live leg's but the real-model
    /// one, so this keeps the exact result it had before postures existed.
    pub fn evaluate(observation: &PreconditionObservation) -> PreconditionsStatus {
        Self::evaluate_for(observation, L4Posture::Deterministic)
    }

    /// Judge an observation for a leg run under `posture`. Pure: no environment read, no IO, no
    /// socket. The postures differ only on the L4 handle: the deterministic one requires it
    /// declared, the real-model one requires it absent or falsy.
    pub fn evaluate_for(
        observation: &PreconditionObservation,
        posture: L4Posture,
    ) -> PreconditionsStatus {
        let mut unmet = Vec::new();

        if !observation.egress_reachable {
            unmet.push(UnmetPrecondition {
                subject: PreconditionSubject::EgressReachable,
                statement: format!("nothing accepted a connection on {EGRESS_TARGET}"),
                causes: "either pulse-app is not running, or it is running without its OTLP \
                         receiver bound — Conductor launches no Pulse process and cannot tell \
                         the two apart from here"
                    .to_string(),
            });
        }

        if !observation.sidecar_resolved {
            unmet.push(UnmetPrecondition {
                subject: PreconditionSubject::SidecarResolvable,
                statement: format!("{SIDECAR_PROGRAM} did not resolve on the inherited PATH"),
                causes: "either the sidecar was never built (cargo build -p mcp-server --bin \
                         andromeda-pulse-mcp --features mcp-server, in the Pulse repo), or it is \
                         built but its directory is absent from PATH — an unresolvable sidecar \
                         makes every read-back arm report the unreachable path in ~0s, which at \
                         row level looks exactly like a genuine SUT-side gate failure"
                    .to_string(),
            });
        }

        let undeclared = observation.undeclared(posture);
        let declared_against = observation.declared_against(posture);
        let mut statements = Vec::new();
        let mut causes = Vec::new();
        if !undeclared.is_empty() {
            statements.push(format!(
                "undeclared in this environment: {}",
                undeclared.join(", ")
            ));
            causes.push(
                "either the launching shell never declared them, or pulse-app was started from a \
                 different environment than Conductor's — Conductor reads only its own environment \
                 and cannot inspect a process it does not launch",
            );
        }
        if !declared_against.is_empty() {
            statements.push(format!(
                "declared in this environment, but the {posture} posture requires it absent or \
                 falsy: {}",
                declared_against.join(", ")
            ));
            causes.push(
                "the launching shell declares it truthy under Conductor's rule or Pulse's own (which \
                 also reads yes), so a pulse-app launched from that shell would serve canned \
                 interpretations — Conductor reads only its own environment and never sets or unsets \
                 the handle",
            );
        }
        if !statements.is_empty() {
            unmet.push(UnmetPrecondition {
                subject: PreconditionSubject::HandlesDeclared,
                statement: statements.join("; "),
                causes: causes.join("; "),
            });
        }

        PreconditionsStatus { unmet }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn declared(handles: &[&str]) -> BTreeSet<String> {
        handles.iter().map(|h| h.to_string()).collect()
    }

    fn all_declared() -> BTreeSet<String> {
        declared(&OBSERVED_HANDLES)
    }

    fn satisfied() -> PreconditionObservation {
        PreconditionObservation {
            egress_reachable: true,
            sidecar_resolved: true,
            declared: all_declared(),
        }
    }

    #[rstest]
    #[case(Some("true"), true)]
    #[case(Some("1"), true)]
    #[case(Some("TRUE"), true)]
    #[case(Some("True"), true)]
    #[case(Some(" 1 "), true)]
    #[case(None, false)]
    #[case(Some(""), false)]
    #[case(Some("   "), false)]
    #[case(Some("false"), false)]
    #[case(Some("0"), false)]
    #[case(Some("yes"), false)]
    #[case(Some("D:\\pulse\\data"), false)]
    fn flag_declared_accepts_only_an_affirmative_value(
        #[case] value: Option<&str>,
        #[case] expected: bool,
    ) {
        assert_eq!(flag_declared(value), expected, "value {value:?}");
    }

    #[rstest]
    #[case(Some("D:\\pulse\\data"), true)]
    #[case(Some("/home/dev/.andromeda-pulse"), true)]
    #[case(Some("  /tmp/pulse  "), true)]
    #[case(Some("false"), true)]
    #[case(Some("0"), true)]
    #[case(None, false)]
    #[case(Some(""), false)]
    #[case(Some("   "), false)]
    fn the_path_handle_is_graded_by_presence(#[case] value: Option<&str>, #[case] expected: bool) {
        assert_eq!(
            handle_declared(PATH_VALUED_HANDLE, value),
            expected,
            "value {value:?}"
        );
    }

    #[rstest]
    #[case("ANDROMEDA_PULSE_L4_DETERMINISTIC")]
    #[case("ANDROMEDA_PULSE_MCP_ENABLED")]
    #[case("ANDROMEDA_PULSE_SOMETHING_UNKNOWN")]
    fn every_non_path_name_delegates_to_the_flag_rule(#[case] name: &str) {
        for value in [
            None,
            Some(""),
            Some("   "),
            Some("false"),
            Some("0"),
            Some("yes"),
            Some("D:\\pulse\\data"),
            Some("true"),
            Some("1"),
            Some("TRUE"),
            Some(" 1 "),
        ] {
            assert_eq!(
                handle_declared(name, value),
                flag_declared(value),
                "{name} must grade {value:?} exactly as the flag rule does"
            );
        }
    }

    #[test]
    fn the_two_grading_rules_are_not_interchangeable() {
        // The one pair that fails if presence and truthiness are ever swapped: a data dir whose
        // text happens to read `false` is still a declared PATH, while the flag it mimics is not.
        assert!(handle_declared(PATH_VALUED_HANDLE, Some("false")));
        assert!(!handle_declared(
            "ANDROMEDA_PULSE_MCP_ENABLED",
            Some("false")
        ));
    }

    #[test]
    fn the_presence_graded_name_is_one_the_probe_observes() {
        // Two literals name the same handle; without this they can drift and the path handle would
        // silently fall back to the flag rule — the state that made `handles-declared` unsatisfiable.
        assert!(OBSERVED_HANDLES.contains(&PATH_VALUED_HANDLE));
    }

    #[test]
    fn every_subject_satisfied_is_a_satisfied_status() {
        let status = Preconditions::evaluate(&satisfied());
        assert!(status.is_satisfied());
        assert!(status.unmet().is_empty());
        // The NEGATIVE side of `is_unmet`: every shipped assertion asks it for a subject that IS
        // unmet, so nothing observed a wrong `true` until the tier named it.
        for subject in [
            PreconditionSubject::EgressReachable,
            PreconditionSubject::SidecarResolvable,
            PreconditionSubject::HandlesDeclared,
        ] {
            assert!(!status.is_unmet(subject), "{subject} is satisfied here");
        }
    }

    #[test]
    fn the_default_observation_is_unmet_on_every_subject() {
        // The all-absent host — no Pulse, no sidecar, no handles — is the case the probe exists for.
        let status = Preconditions::evaluate(&PreconditionObservation::default());
        assert!(!status.is_satisfied());
        assert_eq!(
            status.unmet().len(),
            3,
            "all three subjects report individually"
        );
        for subject in [
            PreconditionSubject::EgressReachable,
            PreconditionSubject::SidecarResolvable,
            PreconditionSubject::HandlesDeclared,
        ] {
            assert!(status.is_unmet(subject), "{subject} must be named");
        }
    }

    #[rstest]
    #[case(PreconditionSubject::EgressReachable)]
    #[case(PreconditionSubject::SidecarResolvable)]
    #[case(PreconditionSubject::HandlesDeclared)]
    fn each_subject_fails_alone_while_the_others_pass(#[case] subject: PreconditionSubject) {
        let mut observation = satisfied();
        match subject {
            PreconditionSubject::EgressReachable => observation.egress_reachable = false,
            PreconditionSubject::SidecarResolvable => observation.sidecar_resolved = false,
            PreconditionSubject::HandlesDeclared => {
                observation.declared.remove(OBSERVED_HANDLES[0]);
            }
        }
        let status = Preconditions::evaluate(&observation);
        assert_eq!(status.unmet().len(), 1, "only the mutated subject is unmet");
        assert_eq!(status.unmet()[0].subject, subject);
        assert!(!status.is_satisfied());
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    fn a_single_undeclared_handle_is_named_individually(#[case] missing: usize) {
        let mut observation = satisfied();
        observation.declared.remove(OBSERVED_HANDLES[missing]);

        let status = Preconditions::evaluate(&observation);
        let finding = &status.unmet()[0];
        assert_eq!(finding.subject, PreconditionSubject::HandlesDeclared);
        assert!(
            finding.statement.contains(OBSERVED_HANDLES[missing]),
            "the miss names the handle: {}",
            finding.statement
        );
        for (i, other) in OBSERVED_HANDLES.iter().enumerate() {
            if i != missing {
                assert!(
                    !finding.statement.contains(other),
                    "a declared handle is not reported missing: {}",
                    finding.statement
                );
            }
        }
    }

    #[test]
    fn undeclared_handles_are_reported_in_observed_order() {
        let observation = PreconditionObservation {
            egress_reachable: true,
            sidecar_resolved: true,
            declared: BTreeSet::new(),
        };
        let status = Preconditions::evaluate(&observation);
        let statement = &status.unmet()[0].statement;
        let positions: Vec<_> = OBSERVED_HANDLES
            .iter()
            .map(|h| statement.find(h).expect("named"))
            .collect();
        assert!(
            positions.windows(2).all(|w| w[0] < w[1]),
            "stable order: {statement}"
        );
    }

    #[test]
    fn an_unrelated_declared_name_satisfies_nothing() {
        let observation = PreconditionObservation {
            egress_reachable: true,
            sidecar_resolved: true,
            declared: declared(&["ANDROMEDA_PULSE_SOMETHING_ELSE"]),
        };
        let status = Preconditions::evaluate(&observation);
        assert!(status.is_unmet(PreconditionSubject::HandlesDeclared));
        assert_eq!(
            status.unmet()[0]
                .statement
                .matches("ANDROMEDA_PULSE_")
                .count(),
            3
        );
    }

    #[test]
    fn the_observed_handle_set_is_pinned() {
        // Exact-set equality in BOTH directions, the check_scenario_backing precedent: the pin
        // fails on an addition, a removal and pin rot alike.
        let pinned: BTreeSet<&str> = BTreeSet::from([
            "ANDROMEDA_PULSE_DATA_DIR",
            "ANDROMEDA_PULSE_L4_DETERMINISTIC",
            "ANDROMEDA_PULSE_MCP_ENABLED",
        ]);
        let actual: BTreeSet<&str> = OBSERVED_HANDLES.iter().copied().collect();
        assert_eq!(actual, pinned);
        assert_eq!(OBSERVED_HANDLES.len(), pinned.len(), "no duplicate handle");
    }

    #[test]
    fn every_subject_carries_a_distinct_stable_id() {
        let ids: BTreeSet<&str> = [
            PreconditionSubject::EgressReachable,
            PreconditionSubject::SidecarResolvable,
            PreconditionSubject::HandlesDeclared,
        ]
        .iter()
        .map(|s| s.id())
        .collect();
        assert_eq!(ids.len(), 3);
        assert_eq!(
            PreconditionSubject::EgressReachable.to_string(),
            "egress-reachable"
        );
    }

    #[rstest]
    #[case(Some("true"), true)]
    #[case(Some("1"), true)]
    #[case(Some("yes"), true)]
    #[case(Some("YES"), true)]
    #[case(Some(" Yes "), true)]
    #[case(Some("TRUE"), true)]
    #[case(None, false)]
    #[case(Some(""), false)]
    #[case(Some("false"), false)]
    #[case(Some("0"), false)]
    #[case(Some("no"), false)]
    #[case(Some("on"), false)]
    fn either_side_reads_pulse_truthy_set_as_declared(
        #[case] value: Option<&str>,
        #[case] expected: bool,
    ) {
        assert_eq!(
            flag_declared_on_either_side(value),
            expected,
            "value {value:?}"
        );
        if flag_declared(value) {
            assert!(
                flag_declared_on_either_side(value),
                "Conductor's own truthy set is inside the union: {value:?}"
            );
        }
    }

    #[test]
    fn yes_separates_the_two_truthy_sets() {
        // The value that makes the union load-bearing: Pulse would run canned L4 on it while
        // Conductor's own rule reads it off.
        assert!(!flag_declared(Some("yes")));
        assert!(flag_declared_on_either_side(Some("yes")));
    }

    #[test]
    fn handle_declared_for_changes_only_the_l4_handle_under_real_model() {
        let values = [
            None,
            Some(""),
            Some("false"),
            Some("0"),
            Some("yes"),
            Some("true"),
            Some("1"),
            Some("D:\\pulse\\data"),
        ];
        for name in OBSERVED_HANDLES {
            for value in values {
                assert_eq!(
                    handle_declared_for(name, value, L4Posture::Deterministic),
                    handle_declared(name, value),
                    "deterministic grading is unchanged: {name} {value:?}"
                );
                let real_model = handle_declared_for(name, value, L4Posture::RealModel);
                if name == L4_HANDLE {
                    assert_eq!(real_model, flag_declared_on_either_side(value), "{value:?}");
                } else {
                    assert_eq!(real_model, handle_declared(name, value), "{name} {value:?}");
                }
            }
        }
    }

    #[test]
    fn the_absence_graded_name_is_one_the_probe_observes() {
        assert!(OBSERVED_HANDLES.contains(&L4_HANDLE));
    }

    #[test]
    fn real_model_with_l4_declared_is_unmet_naming_it_alone() {
        let status = Preconditions::evaluate_for(&satisfied(), L4Posture::RealModel);
        assert_eq!(status.unmet().len(), 1);
        let finding = &status.unmet()[0];
        assert_eq!(finding.subject, PreconditionSubject::HandlesDeclared);
        assert!(
            finding.statement.contains(L4_HANDLE),
            "{}",
            finding.statement
        );
        assert!(
            finding
                .statement
                .contains("real-model posture requires it absent or falsy"),
            "names the posture: {}",
            finding.statement
        );
        for other in OBSERVED_HANDLES.iter().filter(|h| **h != L4_HANDLE) {
            assert!(
                !finding.statement.contains(other),
                "a satisfied handle is not named: {}",
                finding.statement
            );
        }
    }

    #[test]
    fn real_model_with_l4_absent_and_the_others_declared_is_satisfied() {
        let mut observation = satisfied();
        observation.declared.remove(L4_HANDLE);
        assert!(Preconditions::evaluate_for(&observation, L4Posture::RealModel).is_satisfied());
        assert!(
            Preconditions::evaluate(&observation).is_unmet(PreconditionSubject::HandlesDeclared),
            "the same environment is refused under the deterministic posture"
        );
    }

    #[test]
    fn real_model_still_requires_the_other_handles_and_names_both_kinds_of_miss() {
        let observation = PreconditionObservation {
            egress_reachable: true,
            sidecar_resolved: true,
            declared: declared(&[L4_HANDLE]),
        };
        let status = Preconditions::evaluate_for(&observation, L4Posture::RealModel);
        let finding = &status.unmet()[0];
        assert_eq!(status.unmet().len(), 1, "one subject, one finding");
        assert!(finding.statement.contains(
            "undeclared in this environment: ANDROMEDA_PULSE_DATA_DIR, ANDROMEDA_PULSE_MCP_ENABLED"
        ));
        assert!(
            finding
                .statement
                .contains(&format!("absent or falsy: {L4_HANDLE}"))
        );
        assert!(!finding.causes.trim().is_empty());
    }

    #[test]
    fn evaluate_is_the_deterministic_posture_on_every_observation_shape() {
        let mut shapes = vec![PreconditionObservation::default(), satisfied()];
        for handle in OBSERVED_HANDLES {
            let mut o = satisfied();
            o.declared.remove(handle);
            shapes.push(o);
        }
        for observation in shapes {
            assert_eq!(
                Preconditions::evaluate(&observation),
                Preconditions::evaluate_for(&observation, L4Posture::Deterministic)
            );
        }
    }

    #[test]
    fn no_finding_carries_a_host_path_or_an_env_value() {
        // The drive-letter token is WORD-ANCHORED: the egress statement carries 127.0.0.1:4317 and
        // an unanchored `[A-Za-z]:[\\/]` would match inside a `scheme://` form. The real-model
        // status with the L4 handle declared carries the posture's own statement beside the
        // undeclared one, so both wordings are covered.
        let deterministic = Preconditions::evaluate(&PreconditionObservation::default());
        let real_model = Preconditions::evaluate_for(
            &PreconditionObservation {
                declared: declared(&[L4_HANDLE]),
                ..PreconditionObservation::default()
            },
            L4Posture::RealModel,
        );
        for finding in deterministic.unmet().iter().chain(real_model.unmet()) {
            for text in [&finding.statement, &finding.causes] {
                for token in ["%APPDATA%", "/Users/", "/home/", ".cargo", ".rustup"] {
                    assert!(!text.contains(token), "host-path token {token} in {text}");
                }
                let bytes = text.as_bytes();
                for (i, w) in bytes.windows(2).enumerate() {
                    let drive_letter = w[0].is_ascii_alphabetic() && w[1] == b':';
                    let after = bytes.get(i + 2).copied();
                    let path_sep = matches!(after, Some(b'\\') | Some(b'/'));
                    let at_boundary = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
                    assert!(
                        !(drive_letter && path_sep && at_boundary),
                        "drive-letter path in {text}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_status_built_from_unmet_round_trips() {
        let finding = UnmetPrecondition {
            subject: PreconditionSubject::EgressReachable,
            statement: "s".to_string(),
            causes: "c".to_string(),
        };
        let status = PreconditionsStatus::from_unmet(vec![finding.clone()]);
        assert!(!status.is_satisfied());
        assert_eq!(status.unmet(), &[finding]);
        assert_ne!(
            status,
            PreconditionsStatus::default(),
            "distinguishable from the empty default"
        );
    }
}
