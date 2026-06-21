//! Verdict classification — the two-state probabilistic-assertion policy (architecture
//! §Probabilistic-Assertion Policy).
//!
//! [`classify`] turns one check's outcome into a [`Verdict`]: a [`ClaimClass::Hard`] (deterministic)
//! claim passes or fails on its comparison; a [`ClaimClass::CalibrationRegion`] (model-interpretive)
//! claim is reported-for-human and routes to [`Verdict::CalibrationRegion`] — never hard-failed on an
//! exact-value mismatch. The result is an [`Assessment`] value (the verdict + the redacted
//! observed/expected and a calibration-region delta), never a `Result::Err` — the verdict/error wall.

use conductor_core::{ClaimClass, Verdict, redact_value};
use serde::Serialize;

/// The outcome of classifying one check — a value, never an error (the verdict/error wall).
///
/// Mirrors the outcome-as-value shape of the preflight [`crate::ReadyState`]: a serializable struct the
/// Epoch-6 run-report writers consume. `observed`/`expected`/`delta` are pre-redacted
/// ([`redact_value`]) so no absolute host path can reach a log line or the run-report artifact
/// (security-plan §Error Handling).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Assessment {
    /// The machine verdict this check earned.
    pub verdict: Verdict,
    /// What read-back observed, redacted for the artifact edge.
    pub observed: String,
    /// What the check expected, redacted for the artifact edge.
    pub expected: String,
    /// The human-facing observed-vs-expected delta — `Some` only for a [`Verdict::CalibrationRegion`]
    /// outcome (the signal a human reviews); `None` for a hard `Pass`/`Fail` (the verdict is itself the
    /// signal).
    pub delta: Option<String>,
}

impl Assessment {
    /// The machine verdict this check earned.
    pub fn verdict(&self) -> Verdict {
        self.verdict
    }
}

/// Classify one check into a [`Verdict`] under the two-state assertion policy.
///
/// `matched` is the caller's deterministic comparison result. A [`ClaimClass::Hard`] claim passes when
/// `matched` and fails otherwise; a [`ClaimClass::CalibrationRegion`] claim routes to
/// [`Verdict::CalibrationRegion`] regardless of `matched` (never hard-failed) and captures the delta as
/// the human-facing signal. `observed`/`expected` are redacted ([`redact_value`]) before capture.
pub fn classify(
    class: ClaimClass,
    matched: bool,
    observed: impl Into<String>,
    expected: impl Into<String>,
) -> Assessment {
    let observed: String = observed.into();
    let expected: String = expected.into();
    let observed = redact_value(&observed).into_owned();
    let expected = redact_value(&expected).into_owned();

    match class {
        ClaimClass::Hard => Assessment {
            verdict: if matched { Verdict::Pass } else { Verdict::Fail },
            observed,
            expected,
            delta: None,
        },
        ClaimClass::CalibrationRegion => {
            let delta = format!("observed {observed} vs expected {expected}");
            Assessment {
                verdict: Verdict::CalibrationRegion,
                observed,
                expected,
                delta: Some(delta),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification_is_deterministic() {
        let a = classify(ClaimClass::Hard, false, "obs", "exp");
        let b = classify(ClaimClass::Hard, false, "obs", "exp");
        assert_eq!(a, b);
        let c = classify(ClaimClass::CalibrationRegion, true, "obs", "exp");
        let d = classify(ClaimClass::CalibrationRegion, true, "obs", "exp");
        assert_eq!(c, d);
    }

    #[test]
    fn observed_and_expected_are_redacted() {
        let a = classify(ClaimClass::Hard, true, "C:\\Users\\turbo\\corpus.db", "ok");
        assert_eq!(a.observed, "<redacted>");
        assert!(!a.observed.contains("C:\\Users"));
        let c = classify(ClaimClass::CalibrationRegion, false, "/home/turbo/x", "y");
        assert_eq!(c.observed, "<redacted>");
        // the composed delta is built from already-redacted parts — no host path survives.
        assert!(!c.delta.unwrap().contains("/home/"));
    }
}
