//! The scheduling-time precondition probe — what a live leg needs before it is worth running.
//!
//! Non-mutating by construction: `:4317` is connected-and-dropped, the sidecar is resolved by a
//! directory walk rather than a spawn, and the `ANDROMEDA_PULSE_*` handles are read for a
//! declaration only. Each handle is graded by the KIND of value it carries, so every subject is
//! satisfiable (arch §Standard Contracts — Liveness equivalent).


use anyhow::Context as _;

use conductor_core::{
    OBSERVED_HANDLES, PreconditionObservation, Preconditions,
    PreconditionsStatus, RunContract, RunContractStatus,
    flag_declared, handle_declared, resolve_under,
};
use conductor_emit::{
    DEFAULT_OTLP_ENDPOINT, probe_egress,
};



use conductor_verify::sidecar_resolves_on_path;

/// Resolve + load the recorded run contract. A fixed in-repo path with no `CONDUCTOR_*` override,
/// guarded the same way as the capability manifest and load envelope; a read / parse / bounds
/// failure is a harness fault, never a silent default.
pub(crate) fn load_run_contract() -> anyhow::Result<RunContract> {
    let base = std::env::current_dir().context("resolve current directory")?;
    let path = resolve_under(&base, &RunContract::default_path())?;
    Ok(RunContract::load(&path)?)
}

/// Evaluate the contract against what Conductor can honestly observe: declarations in its OWN
/// environment — the shell that also launches `pulse-app`. Never a claim about `pulse-app` itself.
pub(crate) fn observe_run_contract(contract: &RunContract) -> RunContractStatus {
    let declared = contract
        .observed_env()
        .into_iter()
        .filter(|name| declares(name))
        .map(str::to_string)
        .collect();
    contract.evaluate(&declared)
}

/// Whether an env var carries an affirmative declaration, for the run contract's `shell-declaration`
/// terms. The env read lives here; the value test is
/// [`flag_declared`](conductor_core::flag_declared), so this path is graded truthy-only whatever a
/// future term names — it never reaches the presence arm of `handle_declared`.
fn declares(name: &str) -> bool {
    flag_declared(std::env::var(name).ok().as_deref())
}

/// Probe the live-Pulse preconditions BEFORE a leg is scheduled, so an absent SUT is named once
/// rather than absorbed by each chunk, gate and wrap in turn.
///
/// Deliberately NOT routed through [`canary_gate`]: a preflight fires its own storm and Pulse
/// dedupes a new incident against any open one on the `(kind, scope, scope_id)` tuple, so probing
/// via the gate would prime exactly the state the following leg collides with (architecture
/// §Established Decisions [Read-Back Dependency Posture]). Nothing here emits, binds or spawns.
///
/// The three observations happen HERE and enter the evaluator as values, keeping the judgment a
/// pure function of its inputs (the [`observe_run_contract`] shape above).
pub async fn observe_preconditions() -> PreconditionsStatus {
    let observation = PreconditionObservation {
        egress_reachable: probe_egress(DEFAULT_OTLP_ENDPOINT).await.is_ok(),
        sidecar_resolved: sidecar_resolves_on_path(),
        declared: OBSERVED_HANDLES
            .iter()
            .filter(|name| {
                let value = std::env::var(**name).ok();
                handle_declared(name, value.as_deref())
            })
            .map(|name| (*name).to_string())
            .collect(),
    };

    let status = Preconditions::evaluate(&observation);

    // A boundary fact inside the caller's own span — the bounded span-name set is a deliberate
    // contract and is not widened for a new boundary (obs-plan §6 Boundary-call wrappers). The
    // unmet subjects ride `message`; no field here carries a path, a PATH value or an env value.
    if status.is_satisfied() {
        tracing::info!("live-Pulse preconditions satisfied");
    } else {
        let unmet =
            status.unmet().iter().map(|u| u.subject.id()).collect::<Vec<_>>().join(", ");
        tracing::warn!("live-Pulse preconditions unmet: {unmet}");
    }

    status
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::*;
    use crate::testkit::UNDECLARED_ENV;
    use conductor_core::{CheckKind, ContractTerm, PreconditionSubject};

    #[test]
    fn observe_run_contract_names_the_term_this_environment_does_not_declare() {
        let contract = contract_of(vec![shell_term("l4-deterministic", UNDECLARED_ENV)]);
        let status = observe_run_contract(&contract);

        assert!(!status.is_satisfied(), "an undeclared shell term is unmet");
        assert_eq!(status.unmet().len(), 1, "one term, one unmet entry");
        assert_eq!(status.unmet()[0].id, "l4-deterministic", "the gate names the term individually");
        assert_ne!(
            status,
            RunContractStatus::default(),
            "a status carrying an unmet term must be distinguishable from the empty default"
        );
    }

    #[test]
    fn declares_rejects_a_name_this_environment_does_not_declare() {
        // The env READ, at the run-contract predicate: the value test now lives in
        // `conductor_core::flag_declared`, so this asserts the read reaches it. A name guaranteed
        // unset keeps it deterministic under both runners with no env mutation.
        assert!(!declares(UNDECLARED_ENV));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn observe_preconditions_grades_each_handle_from_its_own_environment() {
        // The gap the `preconditions.rs` suite cannot cover: every test there builds `declared` by
        // hand, so nothing asserts that the observation site reads the environment and applies
        // `handle_declared` to it. Recomputing the expectation from the ambient env keeps this
        // deterministic on any host — a host with the handles set and one without both pass, and
        // only the site's own read/grade wiring can break it. `handle_declared` itself is pinned by
        // the matrix in `conductor-core`, so this is not circular.
        let status = observe_preconditions().await;

        let undeclared: Vec<&str> = OBSERVED_HANDLES
            .iter()
            .copied()
            .filter(|name| {
                let value = std::env::var(name).ok();
                !handle_declared(name, value.as_deref())
            })
            .collect();

        let finding = status
            .unmet()
            .iter()
            .find(|u| u.subject == PreconditionSubject::HandlesDeclared);

        match finding {
            Some(finding) => {
                assert!(
                    !undeclared.is_empty(),
                    "the subject is unmet, so at least one handle must be undeclared: {}",
                    finding.statement
                );
                for name in &undeclared {
                    assert!(
                        finding.statement.contains(name),
                        "an undeclared handle must be named: {}",
                        finding.statement
                    );
                }
                for name in OBSERVED_HANDLES.iter().filter(|n| !undeclared.contains(n)) {
                    assert!(
                        !finding.statement.contains(name),
                        "a declared handle must not be reported missing: {}",
                        finding.statement
                    );
                }
            }
            None => assert!(
                undeclared.is_empty(),
                "no finding, so every observed handle must be declared; undeclared: {undeclared:?}"
            ),
        }
    }

    #[test]
    fn observe_run_contract_is_satisfied_when_no_term_is_a_shell_declaration() {
        // Only a ShellDeclaration term can be unmet: a term whose truth lives on the SUT's side is
        // recorded and never blocks (arch §Standard Contracts).
        let contract = contract_of(vec![ContractTerm {
            id: "corpus-plaintext".to_string(),
            statement: "true or false entirely on the SUT's side".to_string(),
            check: CheckKind::DeclaredNotObservable,
            env: None,
            causes: "not observable from here".to_string(),
        }]);
        assert!(observe_run_contract(&contract).is_satisfied());
    }
}
