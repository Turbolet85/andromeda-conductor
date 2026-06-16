//! The deterministic seeded phase scheduler.
//!
//! [`run_timeline`] sequences a [`PhaseTimeline`] on a `current_thread` tokio runtime using
//! `tokio::time`: for each phase it perturbs the base gap by a seed-derived bounded jitter, sleeps
//! the jittered gap on the *virtual* clock, then surfaces a [`PhaseTransition`]. Same seed + timeline
//! ⇒ identical transition sequence and timing; a different seed ⇒ a different (still-reproducible)
//! shape — the determinism substrate the emission/fault seams build on (architecture §Design
//! Philosophy). It is runtime-agnostic (the caller owns the runtime) and emits nothing — no OTLP, no
//! journal; boundaries are handed back as values.

use std::time::Duration;

use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

use crate::phase::{PhaseTimeline, PhaseTransition};

/// A harness fault from the timeline scheduler — never a verification outcome (the verdict/error
/// wall: outcomes ride in `Ok`, `Err` is a Conductor fault only). `#[non_exhaustive]` because the
/// emission/fault seams extend the timeline-fault surface in later chunks.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TimelineError {
    /// The timeline carried no phases — there is nothing to sequence.
    #[error("timeline has no phases to sequence")]
    EmptyTimeline,
}

/// Sequence `timeline` deterministically under `seed`, returning its ordered phase boundaries.
///
/// Awaits each phase's seed-jittered gap on `tokio::time` (the virtual clock — drive it with
/// `start_paused` in tests), then records a [`PhaseTransition`]. Pure timing: nothing is emitted.
/// Returns [`TimelineError::EmptyTimeline`] when there are no phases.
#[tracing::instrument(name = "timeline.execute", skip(timeline), fields(phase_count = timeline.phases.len()))]
pub async fn run_timeline(
    timeline: &PhaseTimeline,
    seed: u64,
) -> Result<Vec<PhaseTransition>, TimelineError> {
    if timeline.phases.is_empty() {
        return Err(TimelineError::EmptyTimeline);
    }

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let bound_ms = timeline.jitter.as_millis() as u64;
    let mut elapsed = Duration::ZERO;
    let mut transitions = Vec::with_capacity(timeline.phases.len());

    for (index, phase) in timeline.phases.iter().enumerate() {
        let effective = jittered_gap(phase.gap, bound_ms, &mut rng);
        tokio::time::sleep(effective).await;
        elapsed = elapsed.saturating_add(effective);
        transitions.push(PhaseTransition {
            index,
            name: phase.name.clone(),
            elapsed_ms: elapsed.as_millis(),
        });
    }

    Ok(transitions)
}

/// Perturb `base` by a seed-derived delta in `[-bound_ms, +bound_ms]`, clamped at zero. The draw is
/// the scheduler's sole non-determinism, so a fixed seed reproduces the exact gap sequence.
fn jittered_gap(base: Duration, bound_ms: u64, rng: &mut ChaCha8Rng) -> Duration {
    if bound_ms == 0 {
        return base;
    }
    let span = bound_ms * 2 + 1;
    let delta = (rng.next_u64() % span) as i64 - bound_ms as i64;
    let base_ms = base.as_millis() as i64;
    Duration::from_millis((base_ms + delta).max(0) as u64)
}
