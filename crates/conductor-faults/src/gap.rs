//! Emission gap/resume fault (P-015): an exact-length silence window that *resumes* — Pulse reads a
//! gap past its restart threshold (>20s) followed by resume as a service restart (it emits a
//! `RestartEvent` ≤2s; restart-suppression scenarios use a 25s gap).
//!
//! [`EmissionGap`] holds an exact gap [`Duration`] strictly above the 20s threshold ([`MIN_GAP`]) and
//! within a sanity ceiling ([`MAX_GAP`]). "Exact" is the point: the timeline perturbs phase gaps by
//! seeded jitter, which could drop a gap below the threshold; this helper owns an un-jittered value so
//! the restart is deterministically triggered. The resume is intrinsic — the gap ends and emission
//! continues, the contrast with the permanent-silence fault (P-014). An out-of-range gap is a typed
//! [`FaultError`], never a panic — the verdict/error wall.

use std::time::Duration;

use crate::error::FaultError;

/// Restart-detection threshold: the gap must strictly exceed 20s for Pulse to read it as a restart
/// (restart-suppression scenarios use 25s). A gap at or below this floor is rejected.
pub const MIN_GAP: Duration = Duration::from_secs(20);

/// Sanity ceiling on a single gap (one hour) — parity with the core phase-gap bound; rejects absurd
/// values. The valid range is `(MIN_GAP, MAX_GAP]`.
pub const MAX_GAP: Duration = Duration::from_secs(3_600);

/// An exact-length emission gap that resumes — the P-015 restart-detection lever. Built only through
/// [`EmissionGap::new`], so the gap is always in `(MIN_GAP, MAX_GAP]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmissionGap {
    gap: Duration,
}

impl EmissionGap {
    /// Build a gap of exactly `gap`. It must strictly exceed [`MIN_GAP`] (the 20s restart-detection
    /// threshold) and not exceed [`MAX_GAP`]; otherwise a typed [`FaultError`].
    ///
    /// ```
    /// use std::time::Duration;
    /// use conductor_faults::EmissionGap;
    ///
    /// let gap = EmissionGap::new(Duration::from_secs(25)).unwrap();
    /// assert_eq!(gap.gap(), Duration::from_secs(25));
    ///
    /// assert!(EmissionGap::new(Duration::from_secs(20)).is_err()); // at the floor — rejected
    /// ```
    pub fn new(gap: Duration) -> Result<Self, FaultError> {
        if gap <= MIN_GAP {
            return Err(FaultError::GapTooShort { gap, min: MIN_GAP });
        }
        if gap > MAX_GAP {
            return Err(FaultError::GapTooLong { gap, max: MAX_GAP });
        }
        Ok(Self { gap })
    }

    /// The exact gap duration, returned verbatim — no rounding, no jitter.
    pub fn gap(&self) -> Duration {
        self.gap
    }

    /// The exact gap as whole milliseconds — the representation the timeline / `PhaseSpec` boundary
    /// consumes when this fault is driven under the scheduler (Epoch 7).
    pub fn gap_ms(&self) -> u64 {
        // `gap` is validated `<= MAX_GAP` (3_600_000 ms), so the u128->u64 narrowing never truncates.
        self.gap.as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_gap_above_the_floor() {
        let g = EmissionGap::new(Duration::from_secs(25)).unwrap();
        assert_eq!(g.gap(), Duration::from_secs(25));
        assert_eq!(g.gap_ms(), 25_000);
    }

    #[test]
    fn floor_boundary_is_exclusive() {
        assert!(EmissionGap::new(MIN_GAP).is_err());
        assert!(EmissionGap::new(MIN_GAP + Duration::from_millis(1)).is_ok());
    }

    #[test]
    fn ceiling_boundary_is_inclusive() {
        assert!(EmissionGap::new(MAX_GAP).is_ok());
        assert!(EmissionGap::new(MAX_GAP + Duration::from_millis(1)).is_err());
    }

    #[test]
    fn gap_is_preserved_exactly() {
        for ms in [20_001u64, 25_000, 60_000, 600_000] {
            let d = Duration::from_millis(ms);
            assert_eq!(EmissionGap::new(d).unwrap().gap(), d);
        }
    }

    #[test]
    fn distinct_inputs_yield_distinct_gaps() {
        let a = EmissionGap::new(Duration::from_secs(25)).unwrap();
        let b = EmissionGap::new(Duration::from_secs(30)).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn out_of_range_gaps_return_typed_variants() {
        assert!(matches!(
            EmissionGap::new(Duration::from_secs(5)),
            Err(FaultError::GapTooShort { .. })
        ));
        assert!(matches!(
            EmissionGap::new(Duration::from_secs(7_200)),
            Err(FaultError::GapTooLong { .. })
        ));
    }
}
