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
            }],
            jitter_ms: 0,
            expected: Vec::new(),
        };
        let report = invalid.validate().expect_err("empty scenario must fail validation");
        let err: CoreError = report.into();
        assert!(matches!(err, CoreError::Validation(_)));
    }
}
