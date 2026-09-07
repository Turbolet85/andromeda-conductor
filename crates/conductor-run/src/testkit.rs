//! Fixtures the crate's unit tests share.
//!
//! One home rather than a copy per module: several of these construct crate-private shapes (a
//! blocked [`Preflight`], a fault-declaring phase), so they cannot live in `tests/` — and a copy in
//! each sibling would be the duplication this split exists to remove.
#![cfg(test)]

use conductor_core::{
    CheckKind, ContractTerm, EmissionShape, EmissionSpec, IncidentFormation, LoadEnvelope,
    RunContract, Scenario,
};
use conductor_verify::Observation;

use crate::canary::Preflight;
use crate::lifecycle::LifecycleObservation;

/// A blocked-gate `Preflight` (no connected client) — the no-live-Pulse spine, constructed
/// directly so the test needs neither a sidecar nor an env handle.
pub(crate) fn blocked_preflight() -> Preflight {
    Preflight {
        client: None,
        ready: false,
    }
}

pub(crate) fn observation(degraded: bool) -> Observation {
    Observation {
        text: "active\nRetryStorm".to_string(),
        evidence_count: 6,
        degraded,
        fingerprints: vec!["fp-1".to_string()],
    }
}

pub(crate) fn fixture(seed: u64) -> Scenario {
    let toml = format!(
        "name = \"blocked-fixture\"\np_ids = [\"P-001\"]\nseed = {seed}\nslo_tier = \"<5s\"\njitter_ms = 0\n[[phases]]\nname = \"p1\"\ngap_ms = 100\n"
    );
    Scenario::from_toml_str(&toml).expect("fixture scenario validates")
}

pub(crate) fn named_fixture(name: &str, gap_ms: u64) -> Scenario {
    let toml = format!(
        "name = \"{name}\"\np_ids = [\"P-001\"]\nseed = 1\nslo_tier = \"<5s\"\njitter_ms = 0\n[[phases]]\nname = \"p1\"\ngap_ms = {gap_ms}\n"
    );
    Scenario::from_toml_str(&toml).expect("fixture scenario validates")
}

/// A single fault-declaring silence phase — the port-conflict shape.
pub(crate) fn occupier_fixture() -> Scenario {
    let toml = "name = \"occupier-fixture\"\np_ids = [\"P-003\"]\nseed = 3\nslo_tier = \"<90s\"\njitter_ms = 0\n[[phases]]\nname = \"port-held\"\ngap_ms = 100\n[phases.emission]\nkind = \"plain\"\noccurrences = 0\n[phases.fault]\nkind = \"port_occupier\"\n";
    Scenario::from_toml_str(toml).expect("fixture scenario validates")
}

/// `storm_ms` drives the asserted per-phase sustained-storm window; `named_fixture` builds a
/// single emitting phase, so its gap is the storm.
pub(crate) fn test_envelope(storm_ms: u64, exempt: &[(&str, &str)]) -> LoadEnvelope {
    LoadEnvelope {
        sut_version: "v0.3.0".to_string(),
        captured_at: "2026-08-09".to_string(),
        provenance: "test".to_string(),
        envelope: conductor_core::EnvelopeTerms {
            max_sustained_rate_spans_per_s: 10_000,
            max_sustained_storm_ms: storm_ms,
            max_scenario_duration_ms: 600_000,
        },
        exempt: exempt
            .iter()
            .map(|(scenario, reason)| conductor_core::Exemption {
                scenario: scenario.to_string(),
                reason: reason.to_string(),
            })
            .collect(),
    }
}

pub(crate) fn emission(occurrences: u32, shape: EmissionShape) -> EmissionSpec {
    EmissionSpec::shaped(conductor_core::Signal::Traces, occurrences, shape)
}

/// An env var no shell declares. The run-contract assertions below turn on its ABSENCE, so a
/// real handle would make them pass or fail on the operator's environment instead of on the
/// gate's logic — and reading env at the caller is what keeps this testable without `unsafe`
/// (`.claude/rules/testing.md` 2026-08-10).
pub(crate) const UNDECLARED_ENV: &str = "CONDUCTOR_UNDECLARED_RUN_CONTRACT_PROBE";

pub(crate) fn shell_term(id: &str, env: &str) -> ContractTerm {
    ContractTerm {
        id: id.to_string(),
        statement: "a launch condition the runner's shell does not declare".to_string(),
        check: CheckKind::ShellDeclaration,
        env: Some(env.to_string()),
        causes: "the declaration is absent from this environment".to_string(),
    }
}

pub(crate) fn contract_of(terms: Vec<ContractTerm>) -> RunContract {
    RunContract {
        sut_version: "v0.3.0".to_string(),
        captured_at: "2026-09-03".to_string(),
        provenance: "test fixture".to_string(),
        incident_formation: IncidentFormation {
            warmup_ms: 1_000,
            warmup_emissions: 4,
            min_canary_poll_seconds: 90,
        },
        terms,
    }
}

pub(crate) fn resolved_away(before: Vec<i64>, resolved: i64) -> LifecycleObservation {
    LifecycleObservation {
        before,
        resolved,
        after: Vec::new(),
    }
}
