//! Harness-fault error type — one half of the verdict/error wall.
//!
//! Verification *outcomes* ([`crate::Verdict`] / [`crate::ReportState`]) are returned as
//! `Ok(..)` values; `Result::Err` is reserved for Conductor's own failures (config parse,
//! transport down, MCP unreachable). The split lets the run report classify a model-backed
//! SUT by matching on outcome *values*, never by catching errors.

use thiserror::Error;

/// Conductor's own harness faults — never a verification outcome.
///
/// `#[non_exhaustive]` because seam crates and later chunks extend the harness-fault surface.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CoreError {
    /// A scenario configuration was malformed or could not be parsed.
    #[error("scenario config error: {0}")]
    Config(String),
    /// A scenario failed garde validation at load — a harness fault raised before any scenario
    /// is trusted, never a [`crate::Verdict`]/[`crate::ReportState`].
    #[error("config validation failed: {0}")]
    Validation(#[from] garde::Report),
    /// The SUT capability manifest and the coverage classification disagree — Pulse's ledger moved
    /// past what Conductor has classified, or a classified capability lost its manifest backing.
    #[error("SUT capability drift: {0}")]
    SutDrift(String),
    /// An `Auto`-classified capability claims programmatic verification that no scenario provides, or
    /// the pinned ledger of such claims no longer matches the catalog. Conductor's own coverage
    /// integrity — distinct from [`CoreError::SutDrift`], which tracks the SUT's ledger moving past
    /// the classification.
    #[error("unbacked coverage claim: {0}")]
    UnbackedCoverage(String),
    /// The committed scenario catalog and the pinned SUT load envelope disagree — a scenario drove
    /// past the proven-good bounds without an exemption, or the exemption ledger no longer matches
    /// the catalog. Conductor's own scenario-authoring integrity; a *run* that exceeds the envelope
    /// is an [`EnvelopeStatus`](crate::EnvelopeStatus) value, never this fault.
    #[error("load envelope violation: {0}")]
    LoadEnvelope(String),
}

/// `conductor-core`'s harness-fault result alias: `Ok` carries a value (often a verification
/// outcome), `Err` is a harness fault only.
pub type Result<T> = core::result::Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_error_displays_its_message() {
        let e = CoreError::Config("missing seed".to_string());
        assert_eq!(e.to_string(), "scenario config error: missing seed");
    }

    #[test]
    fn result_alias_carries_outcomes_in_ok() {
        // The verdict/error wall: a verification outcome rides in `Ok`, never as an `Err`.
        let ok: Result<crate::Verdict> = Ok(crate::Verdict::Pass);
        assert!(matches!(ok, Ok(crate::Verdict::Pass)));
    }

    #[test]
    fn garde_report_becomes_a_validation_fault() {
        use garde::Validate;
        // An empty scenario fails validation; its garde Report converts via `#[from]` into a
        // harness-fault `Err` — the config-validation half of the wall.
        let invalid = crate::Scenario {
            name: String::new(),
            p_ids: Vec::new(),
            seed: 0,
            slo_tier: crate::SloTier::Tier5s,
            phases: vec![crate::PhaseSpec {
                name: "p".to_string(),
                gap_ms: 1,
                emission: crate::EmissionSpec::default(),
                fault: None,
            }],
            jitter_ms: 0,
            expected: Vec::new(),
            checklist: Vec::new(),
        };
        let report = invalid
            .validate()
            .expect_err("empty scenario must fail validation");
        let err: CoreError = report.into();
        assert!(matches!(err, CoreError::Validation(_)));
    }
}
