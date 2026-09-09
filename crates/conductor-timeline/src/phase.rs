//! The minimal phase-timing model the scheduler sequences.
//!
//! Just enough structure to drive deterministic sequencing — an ordered list of named phases, each
//! with a base inter-phase gap, plus the bound on the seed-derived jitter that perturbs each gap.
//! A phase also carries the COUNT of emissions it declares, so the scheduler can pace them across
//! the gap and report the total; it never carries what those emissions contain — the shape stays in
//! the config model and the wire types stay in `conductor-emit` (architecture §Design Philosophy).

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
    /// How many emissions this phase declares — the scheduler paces this many hook calls across the
    /// gap. `0` is a deliberate silence window: the gap elapses and nothing is emitted.
    pub emissions: u32,
}

impl Phase {
    /// Construct a phase from its label and base gap, declaring a single emission.
    pub fn new(name: impl Into<String>, gap: Duration) -> Self {
        Self {
            name: name.into(),
            gap,
            emissions: 1,
        }
    }

    /// Construct a phase declaring `emissions` emissions across its gap.
    pub fn emitting(name: impl Into<String>, gap: Duration, emissions: u32) -> Self {
        Self {
            name: name.into(),
            gap,
            emissions,
        }
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

    /// Total emissions this timeline declares — the sum of its phases' counts. Derivable without
    /// running the scenario, which is what lets `timeline.execute` carry `emission_count` as a span
    /// attribute computed at span open (obs-plan §4 Critical Path 1).
    pub fn total_emissions(&self) -> u64 {
        self.phases.iter().map(|p| u64::from(p.emissions)).sum()
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
