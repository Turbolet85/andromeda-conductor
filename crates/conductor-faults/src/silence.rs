//! Abrupt-silence fault (P-014): an abrupt, *permanent* emission stop — emission ceases and never
//! resumes. This is the activity-floor "the service died" lever: a fresh service that was actively
//! emitting (establishing an activity floor) goes abruptly silent, and Pulse raises an activity-floor
//! cue ~30s after the last emission.
//!
//! [`AbruptSilence`] is the terminal counterpart to [`EmissionGap`](crate::EmissionGap) (P-015). Where
//! `EmissionGap` owns a finite, exact gap duration, validates the 20s restart floor, and *resumes*
//! (Pulse reads a restart), abrupt-silence has no resume boundary and no finite length to own —
//! permanence is the structural *absence* of a duration. It acquires no resource and has no bound to
//! validate, so construction is infallible: the one fault helper with no
//! [`FaultError`](crate::FaultError) variant — the verdict/error wall holds vacuously.

/// An abrupt, permanent emission stop with no resume — the P-014 activity-floor "service died" lever
/// and the structural inverse of [`EmissionGap`](crate::EmissionGap) (no duration, no resume).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AbruptSilence;

impl AbruptSilence {
    /// Construct the permanent-stop marker. Infallible — a permanent stop holds no resource and has no
    /// bound to validate, so there is nothing to fail on (contrast
    /// [`EmissionGap::new`](crate::EmissionGap::new), which validates a 20s floor).
    ///
    /// ```
    /// use conductor_faults::AbruptSilence;
    ///
    /// let silence = AbruptSilence::new();
    /// assert!(!silence.resumes()); // permanent — never resumes
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Always `false` — an abrupt silence is permanent and never resumes. This is the property that
    /// distinguishes P-014 from the resuming [`EmissionGap`](crate::EmissionGap) (P-015).
    pub fn resumes(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_resumes() {
        assert!(!AbruptSilence::new().resumes());
    }

    #[test]
    fn is_a_copy_value() {
        let a = AbruptSilence::new();
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn construction_is_reproducible() {
        // Seed-independent by construction (no seed, no parameter); a different-seeds-diverge test is
        // N/A — there is no seed to drive divergence.
        let first = AbruptSilence::new();
        let second = AbruptSilence::new();
        assert_eq!(first, second);
    }
}
