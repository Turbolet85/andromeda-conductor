//! Harness-fault error type — one half of the verdict/error wall.
//!
//! Verification *outcomes* ([`crate::Verdict`] / [`crate::ReportState`]) are returned as
//! `Ok(..)` values; `Result::Err` is reserved for Conductor's own failures (config parse,
//! transport down, MCP unreachable). The split lets the run report classify a model-backed
//! SUT by matching on outcome *values*, never by catching errors.

use thiserror::Error;

/// Conductor's own harness faults — never a verification outcome.
///
/// `#[non_exhaustive]` because seam crates and later chunks extend the harness-fault surface
/// (the config-validation chunk adds a `garde::Report` `#[from]` variant).
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CoreError {
    /// A scenario configuration was malformed or could not be parsed.
    #[error("scenario config error: {0}")]
    Config(String),
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
}
