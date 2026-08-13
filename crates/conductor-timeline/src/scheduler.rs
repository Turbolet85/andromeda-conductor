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
    /// The caller's emission hook failed. The scheduler owns timing only, so the cause is opaque
    /// here and stays a harness fault all the way to the binary edge.
    #[error("emission failed at phase {phase_index}")]
    Emission {
        /// Zero-based ordinal of the phase whose emission failed.
        phase_index: usize,
        /// The hook's own error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// Where in the timeline an emission falls — handed to the hook so a caller can look up the phase's
/// declared shape without the scheduler ever knowing what a shape is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmissionPoint {
    /// Zero-based ordinal of the phase within the timeline.
    pub phase_index: usize,
    /// Zero-based ordinal of this emission within its phase (`0..phase.emissions`).
    pub occurrence: u32,
}

/// Sequence `timeline` deterministically under `seed`, returning its ordered phase boundaries.
///
/// Awaits each phase's seed-jittered gap on `tokio::time` (the virtual clock — drive it with
/// `start_paused` in tests), then records a [`PhaseTransition`]. Emits nothing — see
/// [`run_timeline_with`] for the emitting form. Returns [`TimelineError::EmptyTimeline`] when there
/// are no phases.
pub async fn run_timeline(
    timeline: &PhaseTimeline,
    seed: u64,
) -> Result<Vec<PhaseTransition>, TimelineError> {
    run_timeline_with(timeline, seed, async |_| Ok::<(), std::convert::Infallible>(())).await
}

/// Sequence `timeline` deterministically under `seed`, calling `on_emit` for each declared emission.
///
/// A phase's emissions are paced evenly ACROSS its jittered gap — the phase's slice of the timeline
/// is where its traffic lands, rather than arriving in a burst after the whole timeline has elapsed.
/// This is what lets a windowed SUT detector (an N-in-30s storm cue, a 20s silence floor) observe
/// the shape the scenario declares. A phase declaring zero emissions simply sleeps its gap.
///
/// Total elapsed time per phase is exactly the jittered gap either way, and the seeded draw sequence
/// is one per phase as before, so the transition stream is unchanged by the emission count.
/// `on_emit`'s error becomes [`TimelineError::Emission`]; the scheduler gains no emit dependency.
#[tracing::instrument(
    name = "timeline.execute",
    skip(timeline, on_emit),
    fields(phase_count = timeline.phases.len(), emission_count = timeline.total_emissions())
)]
pub async fn run_timeline_with<F, E>(
    timeline: &PhaseTimeline,
    seed: u64,
    mut on_emit: F,
) -> Result<Vec<PhaseTransition>, TimelineError>
where
    F: AsyncFnMut(EmissionPoint) -> Result<(), E>,
    E: std::error::Error + Send + Sync + 'static,
{
    if timeline.phases.is_empty() {
        return Err(TimelineError::EmptyTimeline);
    }

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let bound_ms = timeline.jitter.as_millis() as u64;
    let mut elapsed = Duration::ZERO;
    let mut transitions = Vec::with_capacity(timeline.phases.len());

    for (index, phase) in timeline.phases.iter().enumerate() {
        let effective = jittered_gap(phase.gap, bound_ms, &mut rng);
        for (slice, occurrence) in paced_slices(effective, phase.emissions) {
            tokio::time::sleep(slice).await;
            if let Some(occurrence) = occurrence {
                on_emit(EmissionPoint { phase_index: index, occurrence })
                    .await
                    .map_err(|e| TimelineError::Emission {
                        phase_index: index,
                        source: Box::new(e),
                    })?;
            }
        }
        elapsed = elapsed.saturating_add(effective);
        transitions.push(PhaseTransition {
            index,
            name: phase.name.clone(),
            elapsed_ms: elapsed.as_millis(),
        });
    }

    Ok(transitions)
}

/// Split `gap` into one sleep per declared emission, the last slice absorbing the integer-division
/// remainder so the slices always sum to exactly `gap`. Zero emissions yields the whole gap with no
/// emission point, so a silence phase still costs its full duration.
fn paced_slices(gap: Duration, emissions: u32) -> Vec<(Duration, Option<u32>)> {
    if emissions == 0 {
        return vec![(gap, None)];
    }
    let total_ns = gap.as_nanos();
    let slice_ns = total_ns / u128::from(emissions);
    (0..emissions)
        .map(|i| {
            let ns = if i == emissions - 1 {
                total_ns - slice_ns * u128::from(emissions - 1)
            } else {
                slice_ns
            };
            (Duration::from_nanos(ns as u64), Some(i))
        })
        .collect()
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
