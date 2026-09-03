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

/// The Pulse-side environment handles whose declaration a live leg depends on.
///
/// Held here rather than at the observation site so the evaluator, the observer and the pin test
/// read one list. Distinct from [`RunContract::observed_env`](crate::RunContract::observed_env),
/// which yields only the handles a `shell-declaration` TERM names — a narrower set by design.
pub const OBSERVED_HANDLES: [&str; 3] = [
    "ANDROMEDA_PULSE_DATA_DIR",
    "ANDROMEDA_PULSE_L4_DETERMINISTIC",
    "ANDROMEDA_PULSE_MCP_ENABLED",
];

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
    /// The subset of [`OBSERVED_HANDLES`] the caller found affirmatively declared.
    pub declared: BTreeSet<String>,
}

impl PreconditionObservation {
    /// The handles this observation did NOT find declared, in [`OBSERVED_HANDLES`] order.
    fn undeclared(&self) -> Vec<&'static str> {
        OBSERVED_HANDLES
            .iter()
            .copied()
            .filter(|h| !self.declared.contains(*h))
            .collect()
    }
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
    /// Judge an observation. Pure: no environment read, no IO, no socket.
    pub fn evaluate(observation: &PreconditionObservation) -> PreconditionsStatus {
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

        let undeclared = observation.undeclared();
        if !undeclared.is_empty() {
            unmet.push(UnmetPrecondition {
                subject: PreconditionSubject::HandlesDeclared,
                statement: format!("undeclared in this environment: {}", undeclared.join(", ")),
                causes: "either the launching shell never declared them, or pulse-app was started \
                         from a different environment than Conductor's — Conductor reads only its \
                         own environment and cannot inspect a process it does not launch"
                    .to_string(),
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

    #[test]
    fn every_subject_satisfied_is_a_satisfied_status() {
        let status = Preconditions::evaluate(&satisfied());
        assert!(status.is_satisfied());
        assert!(status.unmet().is_empty());
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

    #[test]
    fn no_finding_carries_a_host_path_or_an_env_value() {
        // The drive-letter token is WORD-ANCHORED: the egress statement carries 127.0.0.1:4317 and
        // an unanchored `[A-Za-z]:[\\/]` would match inside a `scheme://` form.
        let status = Preconditions::evaluate(&PreconditionObservation::default());
        for finding in status.unmet() {
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
