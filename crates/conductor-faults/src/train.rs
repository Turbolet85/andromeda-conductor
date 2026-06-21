//! Bursty-train pattern fault (P-013): a repeating duty cycle of an *active* emission window followed
//! by a *quiet* window, looping indefinitely (canonically 5 min active / 10 min quiet). This is the
//! activity-floor *false-positive guard*: a legitimately periodic service whose long quiet windows
//! (10 min, up to a 60 min "lunch" variant) deliberately exceed the ~30s naïve activity-floor death
//! cue, so only a detector that recognises the *recurring train* avoids flagging it dead.
//!
//! [`BurstyTrain`] is the healthy-but-intermittent third member of the activity-floor family: the
//! recurring contrast to [`AbruptSilence`](crate::AbruptSilence) (P-014, permanent death, never
//! resumes) and [`EmissionGap`](crate::EmissionGap) (P-015, one finite gap then resume). It owns
//! exact, un-jittered active/quiet [`Duration`]s — the timeline applies seeded jitter when it later
//! drives the pattern. An out-of-range duty cycle is a typed [`FaultError`], never a panic — the
//! verdict/error wall.

use std::time::Duration;

use crate::error::FaultError;

/// Per-window sanity ceiling (one hour) — parity with [`MAX_GAP`](crate::MAX_GAP); rejects absurd
/// values. Inclusive, so the Epoch-7 "lunch" 60-min quiet window is valid.
pub const MAX_WINDOW: Duration = Duration::from_secs(3_600);

/// The canonical active window — 5 minutes (the P-013 surface description's "active 5min").
pub const CANONICAL_ACTIVE: Duration = Duration::from_secs(300);

/// The canonical quiet window — 10 minutes (the P-013 surface description's "quiet 10min").
pub const CANONICAL_QUIET: Duration = Duration::from_secs(600);

/// A repeating active/quiet emission duty cycle — the P-013 activity-floor false-positive guard. Built
/// through [`BurstyTrain::new`] (validated) or [`BurstyTrain::canonical`] (the 5min/10min default), so
/// both windows are always non-zero and within [`MAX_WINDOW`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BurstyTrain {
    active: Duration,
    quiet: Duration,
}

impl BurstyTrain {
    /// Build a duty cycle of `active` emission then `quiet` silence, repeating. Both windows must be
    /// non-zero (a zero active window is permanent silence — P-014's domain; a zero quiet window is
    /// continuous emission, not bursty) and at most [`MAX_WINDOW`]; otherwise a typed [`FaultError`].
    ///
    /// ```
    /// use std::time::Duration;
    /// use conductor_faults::BurstyTrain;
    ///
    /// let train = BurstyTrain::new(Duration::from_secs(300), Duration::from_secs(600)).unwrap();
    /// assert_eq!(train.period(), Duration::from_secs(900));
    /// assert!(train.is_active_at(Duration::from_secs(0)));    // a cycle starts active
    /// assert!(!train.is_active_at(Duration::from_secs(300))); // first instant of the quiet window
    ///
    /// assert!(BurstyTrain::new(Duration::ZERO, Duration::from_secs(600)).is_err()); // zero active
    /// ```
    pub fn new(active: Duration, quiet: Duration) -> Result<Self, FaultError> {
        if active.is_zero() {
            return Err(FaultError::ActiveZero);
        }
        if quiet.is_zero() {
            return Err(FaultError::QuietZero);
        }
        if active > MAX_WINDOW {
            return Err(FaultError::WindowTooLong { window: active, max: MAX_WINDOW });
        }
        if quiet > MAX_WINDOW {
            return Err(FaultError::WindowTooLong { window: quiet, max: MAX_WINDOW });
        }
        Ok(Self { active, quiet })
    }

    /// The canonical 5-min-active / 10-min-quiet train ([`CANONICAL_ACTIVE`] / [`CANONICAL_QUIET`]).
    /// Infallible — the canonical windows are statically valid, so no [`FaultError`] is possible.
    pub fn canonical() -> Self {
        Self { active: CANONICAL_ACTIVE, quiet: CANONICAL_QUIET }
    }

    /// The active (emitting) window length, returned verbatim.
    pub fn active(&self) -> Duration {
        self.active
    }

    /// The quiet (silent) window length, returned verbatim.
    pub fn quiet(&self) -> Duration {
        self.quiet
    }

    /// One full cycle: `active + quiet`. Never zero (the active window is always non-zero).
    pub fn period(&self) -> Duration {
        self.active + self.quiet
    }

    /// The active window as whole milliseconds — the representation the timeline / `PhaseSpec` boundary
    /// consumes when this fault is driven under the scheduler (Epoch 7).
    pub fn active_ms(&self) -> u64 {
        // `active` is validated `<= MAX_WINDOW` (3_600_000 ms), so the u128->u64 narrowing never truncates.
        self.active.as_millis() as u64
    }

    /// The quiet window as whole milliseconds (see [`BurstyTrain::active_ms`]).
    pub fn quiet_ms(&self) -> u64 {
        self.quiet.as_millis() as u64
    }

    /// One full cycle as whole milliseconds — `active_ms + quiet_ms` (each `<= MAX_WINDOW`, so the sum
    /// `<= 7_200_000` ms fits a u64 with no truncation).
    pub fn period_ms(&self) -> u64 {
        self.period().as_millis() as u64
    }

    /// Whether emission is active at `offset` from the start of the first cycle. The active window is
    /// the half-open `[0, active)` of each cycle, quiet is `[active, period)`; so `is_active_at(active)`
    /// is `false` (first instant of the quiet window) and `is_active_at(period)` is `true` (the cycle
    /// wraps to the next active window).
    pub fn is_active_at(&self, offset: Duration) -> bool {
        // Duration does not implement Rem; compute the cycle position in nanos. `period` is never zero
        // (the active window is non-zero), so the modulo never divides by zero.
        let position = offset.as_nanos() % self.period().as_nanos();
        position < self.active.as_nanos()
    }
}

impl Default for BurstyTrain {
    fn default() -> Self {
        Self::canonical()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_valid_duty_cycle() {
        let t = BurstyTrain::new(Duration::from_secs(300), Duration::from_secs(600)).unwrap();
        assert_eq!(t.active(), Duration::from_secs(300));
        assert_eq!(t.quiet(), Duration::from_secs(600));
        assert_eq!(t.period(), Duration::from_secs(900));
        assert_eq!(t.active_ms(), 300_000);
        assert_eq!(t.quiet_ms(), 600_000);
        assert_eq!(t.period_ms(), 900_000);
    }

    #[test]
    fn canonical_is_5min_10min_and_is_the_default() {
        let c = BurstyTrain::canonical();
        assert_eq!(c.active(), CANONICAL_ACTIVE);
        assert_eq!(c.quiet(), CANONICAL_QUIET);
        assert_eq!(c, BurstyTrain::new(CANONICAL_ACTIVE, CANONICAL_QUIET).unwrap());
        assert_eq!(BurstyTrain::default(), c);
    }

    #[test]
    fn rejects_zero_active() {
        assert!(matches!(
            BurstyTrain::new(Duration::ZERO, Duration::from_secs(600)),
            Err(FaultError::ActiveZero)
        ));
    }

    #[test]
    fn rejects_zero_quiet() {
        assert!(matches!(
            BurstyTrain::new(Duration::from_secs(300), Duration::ZERO),
            Err(FaultError::QuietZero)
        ));
    }

    #[test]
    fn ceiling_is_inclusive_and_covers_the_lunch_variant() {
        // both windows at the ceiling is OK (the lunch variant uses a 3600s quiet)
        assert!(BurstyTrain::new(MAX_WINDOW, MAX_WINDOW).is_ok());
        assert!(BurstyTrain::new(CANONICAL_ACTIVE, MAX_WINDOW).is_ok());
        // one tick over either window is rejected
        let over = MAX_WINDOW + Duration::from_millis(1);
        assert!(matches!(
            BurstyTrain::new(over, Duration::from_secs(600)),
            Err(FaultError::WindowTooLong { .. })
        ));
        assert!(matches!(
            BurstyTrain::new(Duration::from_secs(300), over),
            Err(FaultError::WindowTooLong { .. })
        ));
    }

    #[test]
    fn phase_query_repeats_across_cycles() {
        let t = BurstyTrain::new(Duration::from_secs(300), Duration::from_secs(600)).unwrap();
        // cycle 0: active [0,300), quiet [300,900)
        assert!(t.is_active_at(Duration::from_secs(0)));
        assert!(t.is_active_at(Duration::from_secs(299)));
        assert!(!t.is_active_at(Duration::from_secs(300))); // first instant of quiet (half-open)
        assert!(!t.is_active_at(Duration::from_secs(899)));
        // cycle 1 wraps: the period boundary is active again
        assert!(t.is_active_at(Duration::from_secs(900)));
        assert!(t.is_active_at(Duration::from_secs(1199)));
        assert!(!t.is_active_at(Duration::from_secs(1200)));
    }

    #[test]
    fn distinct_inputs_yield_distinct_trains() {
        let a = BurstyTrain::new(Duration::from_secs(300), Duration::from_secs(600)).unwrap();
        let b = BurstyTrain::new(Duration::from_secs(300), Duration::from_secs(601)).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn is_a_copy_value() {
        let a = BurstyTrain::canonical();
        let b = a;
        assert_eq!(a, b);
    }
}
