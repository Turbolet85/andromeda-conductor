//! The pinned Pulse run contract — the launch conditions under which a live Pulse can be verified
//! (arch §Standard Contracts; security-plan §Input Validation).
//!
//! Read from `contracts/pulse-run-contract.toml`; the path is resolved at the binary edge and passed
//! in, exactly as the capability manifest and load envelope are. A read / parse / bounds failure is a
//! harness fault ([`CoreError::Config`](crate::CoreError::Config)) — never a verdict, never a panic,
//! never a silent default.
//!
//! Evaluation is deliberately PURE: [`RunContract::evaluate`] takes the set of environment variable
//! names the caller observed declared and returns the unmet terms. The seam never reads the process
//! environment itself, so the readiness gate stays deterministic and its legs are testable without
//! mutating env (which edition 2024 makes `unsafe`).
//!
//! Only a [`CheckKind::ShellDeclaration`] term can be unmet. That is the honest limit: Conductor
//! launches no Pulse process, so it can observe a declaration in its OWN environment — the shell that
//! also launches `pulse-app` — and never a fact about `pulse-app` itself. A term whose truth lives
//! entirely on the other side is recorded as [`CheckKind::DeclaredNotObservable`] and never blocks,
//! the load envelope's declared-not-derivable precedent.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::CoreError;

/// The versioned run contract: which Pulse release it describes, where its terms came from, the
/// harness's own incident-formation obligations, and the terms themselves.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RunContract {
    /// The Pulse release these conditions were captured for (e.g. `"v0.3.0"`).
    pub sut_version: String,
    /// The capture date, `YYYY-MM-DD`.
    pub captured_at: String,
    /// Where the terms came from. They are transcribed from the SUT's record, never measured by
    /// Conductor, and the artifact says so — a reader must be able to tell how far to trust them.
    pub provenance: String,
    /// What the harness must do for an incident to be able to form at all.
    pub incident_formation: IncidentFormation,
    /// The contract's terms.
    #[serde(default, rename = "term")]
    pub terms: Vec<ContractTerm>,
}

/// The harness's own obligations — parameters, not checks, because Conductor satisfies them itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct IncidentFormation {
    /// Benign pre-roll duration carrying the canary service out of Pulse's baseline bootstrap before
    /// the counted storm. Without it the storm is the service's first traffic and no cue fires.
    pub warmup_ms: u64,
    /// How many pre-roll emissions to spread across `warmup_ms`.
    pub warmup_emissions: u32,
    /// The canary poll floor in seconds — must outlast Pulse's L3 digest cadence with L4 behind it.
    pub min_canary_poll_seconds: u64,
}

/// How a term is checked, which is also a statement about what Conductor can honestly know.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckKind {
    /// Observable as a declaration in Conductor's own environment — a same-shell proxy for a
    /// condition on `pulse-app`, never a measurement of it. The only kind that can block.
    ShellDeclaration,
    /// Satisfied by construction: the harness itself guarantees it, or reaching this gate proves it.
    Asserted,
    /// True or false entirely on the SUT's side, with no read-back surface exposing it. Recorded so
    /// an operator can satisfy it; never blocks, because a block would claim a measurement.
    DeclaredNotObservable,
}

/// One contract term.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ContractTerm {
    /// Stable id, kebab-case (e.g. `"l4-deterministic"`).
    pub id: String,
    /// The condition in operator-facing prose.
    pub statement: String,
    /// How it is checked.
    pub check: CheckKind,
    /// The environment variable carrying the declaration — required for, and only meaningful to, a
    /// [`CheckKind::ShellDeclaration`] term.
    #[serde(default)]
    pub env: Option<String>,
    /// Why the term might not hold. Mandatory: a term that names a condition without naming what
    /// could cause it sends an operator looking in the wrong place.
    pub causes: String,
}

/// A term the contract requires and the observation did not satisfy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnmetTerm {
    pub id: String,
    pub statement: String,
    pub causes: String,
}

/// The contract's standing against one observation — a value on the `Ok` path, never an error: an
/// unmet launch condition is a readiness outcome, not a harness fault (arch §Cross-cutting Patterns
/// "Verdict/error wall").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RunContractStatus {
    unmet: Vec<UnmetTerm>,
}

impl RunContractStatus {
    /// A status with nothing unmet — the shape a caller with no contract to check would present.
    pub fn satisfied() -> Self {
        Self { unmet: Vec::new() }
    }

    /// Build a status directly from unmet terms (the test seam).
    pub fn from_unmet(unmet: Vec<UnmetTerm>) -> Self {
        Self { unmet }
    }

    pub fn is_satisfied(&self) -> bool {
        self.unmet.is_empty()
    }

    /// The unmet terms, in contract order — the gate names each one individually.
    pub fn unmet(&self) -> &[UnmetTerm] {
        &self.unmet
    }
}

impl RunContract {
    /// The default contract path, relative to the workspace root. The binary edge resolves it before
    /// calling [`load`](Self::load) — never this module.
    pub fn default_path() -> PathBuf {
        PathBuf::from("contracts/pulse-run-contract.toml")
    }

    /// Read + bounds-check the contract from an already-resolved path.
    pub fn load(path: &Path) -> crate::Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|e| {
            // never the path itself — io::Error's Display leaks it (artifact hygiene).
            CoreError::Config(format!("could not read run contract ({:?})", e.kind()))
        })?;
        let contract: Self = toml::from_str(&text).map_err(|e| {
            CoreError::Config(format!(
                "invalid run contract: {}",
                crate::sanitize_error(&e)
            ))
        })?;
        contract.validate()?;
        tracing::info!(
            count = contract.terms.len(),
            "loaded run contract for Pulse {}",
            contract.sut_version
        );
        Ok(contract)
    }

    /// Evaluate the contract against the environment variable names the caller observed declared.
    /// Only a [`CheckKind::ShellDeclaration`] term participates.
    pub fn evaluate(&self, declared: &BTreeSet<String>) -> RunContractStatus {
        let unmet = self
            .terms
            .iter()
            .filter(|t| t.check == CheckKind::ShellDeclaration)
            .filter(|t| !t.env.as_ref().is_some_and(|e| declared.contains(e)))
            .map(|t| UnmetTerm {
                id: t.id.clone(),
                statement: t.statement.clone(),
                causes: t.causes.clone(),
            })
            .collect();
        RunContractStatus { unmet }
    }

    /// The environment variable names whose declaration the contract observes.
    pub fn observed_env(&self) -> Vec<&str> {
        self.terms
            .iter()
            .filter(|t| t.check == CheckKind::ShellDeclaration)
            .filter_map(|t| t.env.as_deref())
            .collect()
    }

    /// The preflight's own duration: the warm-up plus the full poll budget.
    pub fn preflight_budget_ms(&self) -> u64 {
        self.incident_formation.warmup_ms.saturating_add(
            self.incident_formation
                .min_canary_poll_seconds
                .saturating_mul(1_000),
        )
    }

    /// Assert the harness's own preflight budget stays inside the proven-good SUT bounds. An
    /// over-envelope preflight would induce the DuckDB append stall the envelope exists to avoid.
    pub fn check_within_envelope(&self, max_scenario_duration_ms: u64) -> crate::Result<()> {
        let budget = self.preflight_budget_ms();
        if budget > max_scenario_duration_ms {
            return Err(CoreError::Config(format!(
                "run contract: preflight budget {budget}ms exceeds the SUT load envelope's \
                 {max_scenario_duration_ms}ms bound"
            )));
        }
        Ok(())
    }

    fn validate(&self) -> crate::Result<()> {
        if self.sut_version.trim().is_empty() {
            return Err(CoreError::Config(
                "run contract: sut_version is empty".to_string(),
            ));
        }
        if self.captured_at.trim().is_empty() {
            return Err(CoreError::Config(
                "run contract: captured_at is empty".to_string(),
            ));
        }
        if self.provenance.trim().is_empty() {
            return Err(CoreError::Config(
                "run contract: provenance is empty".to_string(),
            ));
        }
        if self.terms.is_empty() {
            return Err(CoreError::Config(
                "run contract: terms is empty".to_string(),
            ));
        }
        if self.incident_formation.min_canary_poll_seconds == 0 {
            return Err(CoreError::Config(
                "run contract: min_canary_poll_seconds is zero".to_string(),
            ));
        }
        if self.incident_formation.warmup_ms > 0 && self.incident_formation.warmup_emissions == 0 {
            return Err(CoreError::Config(
                "run contract: a warm-up window with zero emissions warms nothing".to_string(),
            ));
        }
        let mut seen = std::collections::HashSet::with_capacity(self.terms.len());
        for term in &self.terms {
            if term.id.trim().is_empty() {
                return Err(CoreError::Config(
                    "run contract: a term id is empty".to_string(),
                ));
            }
            if !seen.insert(term.id.as_str()) {
                return Err(CoreError::Config(format!(
                    "run contract: duplicate term id {:?}",
                    term.id
                )));
            }
            if term.statement.trim().is_empty() || term.causes.trim().is_empty() {
                return Err(CoreError::Config(format!(
                    "run contract: term {:?} needs both a statement and its causes",
                    term.id
                )));
            }
            if term.check == CheckKind::ShellDeclaration
                && term.env.as_ref().is_none_or(|e| e.trim().is_empty())
            {
                return Err(CoreError::Config(format!(
                    "run contract: shell-declaration term {:?} names no env var to observe",
                    term.id
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LoadEnvelope;

    fn contracts_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts")
    }

    fn committed_path() -> PathBuf {
        contracts_dir().join("pulse-run-contract.toml")
    }

    fn declared(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    fn term(id: &str, check: CheckKind, env: Option<&str>) -> ContractTerm {
        ContractTerm {
            id: id.to_string(),
            statement: "a condition".to_string(),
            check,
            env: env.map(str::to_string),
            causes: "a cause".to_string(),
        }
    }

    fn contract(terms: Vec<ContractTerm>) -> RunContract {
        RunContract {
            sut_version: "v0.3.0".to_string(),
            captured_at: "2026-08-10".to_string(),
            provenance: "transcribed".to_string(),
            incident_formation: IncidentFormation {
                warmup_ms: 45_000,
                warmup_emissions: 3,
                min_canary_poll_seconds: 90,
            },
            terms,
        }
    }

    #[test]
    fn loads_and_bounds_checks_the_committed_contract() {
        let c = RunContract::load(&committed_path()).expect("committed contract loads");
        assert_eq!(c.sut_version, "v0.3.0");
        assert!(c.terms.iter().any(|t| t.id == "l4-deterministic"));
        assert!(c.terms.iter().any(|t| t.id == "mcp-enabled"));
        assert!(c.terms.iter().any(|t| t.id == "shared-data-dir"));
        assert!(
            c.incident_formation.warmup_ms > 0,
            "a warm-up is the chunk's operative term"
        );
        assert!(
            c.incident_formation.min_canary_poll_seconds >= 60,
            "the poll floor must outlast L3's 20-60s digest cadence"
        );
        // Both handles Conductor can honestly observe in its OWN environment. `shared-data-dir` is
        // deliberately absent: it is declared-not-observable and names no env var, because the
        // agreement it asserts lives on the SUT's side.
        assert_eq!(
            c.observed_env(),
            vec![
                "ANDROMEDA_PULSE_L4_DETERMINISTIC",
                "ANDROMEDA_PULSE_MCP_ENABLED"
            ]
        );
    }

    #[test]
    fn missing_file_is_a_harness_fault() {
        let err = RunContract::load(Path::new("contracts/does-not-exist.toml")).unwrap_err();
        assert!(matches!(err, CoreError::Config(_)));
    }

    #[test]
    fn load_failure_message_never_contains_the_path() {
        let path = contracts_dir().join("no-such-run-contract.toml");
        let err = RunContract::load(&path).unwrap_err().to_string();
        assert!(
            !err.contains("no-such-run-contract"),
            "the path must not leak: {err}"
        );
        assert!(
            !err.contains(env!("CARGO_MANIFEST_DIR")),
            "no absolute host path: {err}"
        );
    }

    #[test]
    fn rejects_empty_identity_or_terms() {
        assert!(matches!(
            RunContract {
                sut_version: "  ".to_string(),
                ..contract(vec![])
            }
            .validate(),
            Err(CoreError::Config(_))
        ));
        assert!(matches!(
            RunContract {
                captured_at: String::new(),
                ..contract(vec![])
            }
            .validate(),
            Err(CoreError::Config(_))
        ));
        assert!(matches!(
            RunContract {
                provenance: "  ".to_string(),
                ..contract(vec![])
            }
            .validate(),
            Err(CoreError::Config(_))
        ));
        assert!(matches!(
            contract(vec![]).validate(),
            Err(CoreError::Config(_))
        ));
    }

    #[test]
    fn rejects_a_shell_declaration_term_naming_no_env_var() {
        let c = contract(vec![term("l4", CheckKind::ShellDeclaration, None)]);
        assert!(matches!(c.validate(), Err(CoreError::Config(_))));
        let c = contract(vec![term("l4", CheckKind::ShellDeclaration, Some("  "))]);
        assert!(matches!(c.validate(), Err(CoreError::Config(_))));
    }

    #[test]
    fn rejects_a_duplicate_term_id_or_a_zero_poll_floor() {
        let c = contract(vec![
            term("dup", CheckKind::Asserted, None),
            term("dup", CheckKind::Asserted, None),
        ]);
        assert!(matches!(c.validate(), Err(CoreError::Config(_))));

        let mut c = contract(vec![term("t", CheckKind::Asserted, None)]);
        c.incident_formation.min_canary_poll_seconds = 0;
        assert!(matches!(c.validate(), Err(CoreError::Config(_))));

        let mut c = contract(vec![term("t", CheckKind::Asserted, None)]);
        c.incident_formation.warmup_emissions = 0;
        assert!(matches!(c.validate(), Err(CoreError::Config(_))));
    }

    #[test]
    fn only_a_shell_declaration_term_can_be_unmet() {
        let c = contract(vec![
            term("l4", CheckKind::ShellDeclaration, Some("SOME_ENV")),
            term("other-side", CheckKind::DeclaredNotObservable, None),
            term("by-construction", CheckKind::Asserted, None),
        ]);

        let status = c.evaluate(&declared(&[]));
        assert!(!status.is_satisfied());
        assert_eq!(
            status.unmet().len(),
            1,
            "only the shell-declaration term participates"
        );
        assert_eq!(status.unmet()[0].id, "l4");

        let status = c.evaluate(&declared(&["SOME_ENV"]));
        assert!(
            status.is_satisfied(),
            "a declared env var satisfies its term"
        );
    }

    #[test]
    fn an_unmet_term_carries_its_statement_and_causes() {
        let c = RunContract::load(&committed_path()).expect("committed contract loads");
        let status = c.evaluate(&declared(&[]));
        let unmet = status
            .unmet()
            .first()
            .expect("the l4 term is unmet with nothing declared");
        assert!(!unmet.statement.trim().is_empty());
        assert!(
            !unmet.causes.trim().is_empty(),
            "a condition without its causes misleads"
        );
    }

    #[test]
    fn a_satisfied_status_names_nothing() {
        let status = RunContractStatus::satisfied();
        assert!(status.is_satisfied());
        assert!(status.unmet().is_empty());
    }

    #[test]
    fn the_committed_preflight_budget_stays_inside_the_committed_load_envelope() {
        let contract = RunContract::load(&committed_path()).expect("committed contract loads");
        let envelope = LoadEnvelope::load(&contracts_dir().join("pulse-load-envelope.toml"))
            .expect("committed envelope loads");
        contract
            .check_within_envelope(envelope.envelope.max_scenario_duration_ms)
            .expect("the preflight budget must stay inside the proven-good bounds");
    }

    #[test]
    fn an_over_envelope_budget_is_a_harness_fault() {
        let c = contract(vec![term("t", CheckKind::Asserted, None)]);
        assert!(matches!(
            c.check_within_envelope(1_000),
            Err(CoreError::Config(_))
        ));
    }
}
