//! The committed run contract's observable term set, pinned by exact set-equality.
//!
//! Only a `shell-declaration` term can ever be unmet, so THAT set is what decides which launch
//! conditions the preflight gate can name. Pinning it in both directions (the
//! `check_scenario_backing` precedent) fails the gate on an addition, a removal and pin rot alike —
//! a term silently losing its `env`, or gaining a kind that stops it participating, is caught here
//! rather than by a live leg discovering the gate went quiet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use conductor_core::{CheckKind, RunContract};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn pinned_contract() -> RunContract {
    RunContract::load(&workspace_root().join(RunContract::default_path()))
        .expect("the pinned Pulse run contract loads from its fixed path")
}

#[test]
fn the_observable_env_set_is_pinned() {
    let expected: BTreeSet<&str> = BTreeSet::from([
        "ANDROMEDA_PULSE_L4_DETERMINISTIC",
        "ANDROMEDA_PULSE_MCP_ENABLED",
    ]);
    let contract = pinned_contract();
    let actual: BTreeSet<&str> = contract.observed_env().into_iter().collect();

    assert_eq!(
        actual, expected,
        "the shell-declaration term set moved — a new blockable launch condition, or one that \
         stopped participating"
    );
}

#[test]
fn every_shell_declaration_term_names_an_env_var() {
    // `evaluate` treats a shell-declaration term with no `env` as permanently unmet, so a term that
    // lost its handle would block every run with no way to satisfy it. `validate` rejects that at
    // load; this asserts the committed artifact actually clears it.
    for term in pinned_contract()
        .terms
        .iter()
        .filter(|t| t.check == CheckKind::ShellDeclaration)
    {
        let env = term.env.as_deref().unwrap_or_default();
        assert!(
            !env.trim().is_empty(),
            "shell-declaration term {:?} names no env var",
            term.id
        );
    }
}

#[test]
fn the_mcp_enabled_term_is_a_blockable_shell_declaration() {
    let contract = pinned_contract();
    let term = contract
        .terms
        .iter()
        .find(|t| t.id == "mcp-enabled")
        .expect("the contract carries the mcp-enabled term");

    assert_eq!(
        term.check,
        CheckKind::ShellDeclaration,
        "only a shell-declaration term can be unmet — any other kind makes this term inert"
    );
    assert_eq!(term.env.as_deref(), Some("ANDROMEDA_PULSE_MCP_ENABLED"));
    assert!(
        !term.causes.trim().is_empty(),
        "a term names what could cause the miss"
    );
}

#[test]
fn an_undeclared_environment_leaves_every_observable_term_unmet() {
    // The evaluator is pure, so the unsatisfied arm needs no env mutation: an empty observation is
    // exactly what a shell declaring nothing produces.
    let contract = pinned_contract();
    let status = contract.evaluate(&BTreeSet::new());

    assert!(!status.is_satisfied());
    assert_eq!(
        status.unmet().len(),
        contract.observed_env().len(),
        "every observable term reports individually"
    );
}

#[test]
fn declaring_every_observable_handle_satisfies_the_contract() {
    let contract = pinned_contract();
    let declared: BTreeSet<String> = contract
        .observed_env()
        .into_iter()
        .map(str::to_string)
        .collect();

    assert!(contract.evaluate(&declared).is_satisfied());
}
