//! The minimal phase-timing model the scheduler sequences.
//!
//! Just enough structure to drive deterministic sequencing — an ordered list of named phases, each
//! with a base inter-phase gap, plus the bound on the seed-derived jitter that perturbs each gap.
//! The declarative per-phase *emission* spec is a later seam/chunk; these types carry timing only,
//! never what a phase emits (architecture §Design Philosophy).

use std::time::Duration;

/// One phase of a scenario timeline: a named segment the scheduler dwells on before advancing.
///
/// `gap` is the *base* delay before this phase's boundary is surfaced; the scheduler perturbs it by
/// a seed-derived bounded jitter (see [`PhaseTimeline::jitter`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase {
    /// Stable phase label (e.g. `"error-baseline-spike"`), surfaced on the boundary event so a
    /// downstream surface renders the phase line without inferring state.
    pub name: String,
    /// Base inter-phase gap before this phase's boundary (perturbed by the timeline's jitter bound).
    pub gap: Duration,
}

impl Phase {
    /// Construct a phase from its label and base gap.
    pub fn new(name: impl Into<String>, gap: Duration) -> Self {
        Self { name: name.into(), gap }
    }
}

/// An ordered timeline of [`Phase`]s plus the symmetric bound on per-gap jitter.
///
/// `jitter` bounds how far (±) the seeded PRNG may move each phase's `gap`; [`Duration::ZERO`] means
/// gaps land exactly as declared. The scheduler draws each gap within `[base − jitter, base + jitter]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhaseTimeline {
    /// The phases, sequenced in this order.
    pub phases: Vec<Phase>,
    /// Symmetric bound on the seed-derived per-gap jitter.
    pub jitter: Duration,
}

impl PhaseTimeline {
    /// Construct a timeline from its ordered phases and a symmetric jitter bound.
    pub fn new(phases: Vec<Phase>, jitter: Duration) -> Self {
        Self { phases, jitter }
    }
}

/// A phase boundary the scheduler surfaces to its caller as it advances.
///
/// Carries the phase's ordinal + label and the *virtual* (tokio) milliseconds elapsed since the run
/// start when the boundary was reached — enough for a later seam to drive emission or render a phase
/// line. The scheduler yields one per phase, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhaseTransition {
    /// Zero-based ordinal of the phase within the timeline.
    pub index: usize,
    /// The phase's stable label (mirrors [`Phase::name`]).
    pub name: String,
    /// Milliseconds of virtual (tokio) time elapsed since the run start at this boundary.
    pub elapsed_ms: u128,
}
