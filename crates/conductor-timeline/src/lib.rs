//! Deterministic seeded phase scheduler (tokio `current_thread`).
//!
//! The timeline engine's determinism substrate: [`run_timeline_with`] sequences an ordered
//! [`PhaseTimeline`] on `tokio::time`, perturbing each inter-phase gap by a seed-derived bounded
//! jitter so the same seed reproduces the same emission-stream shape (architecture §Design
//! Philosophy). It surfaces [`PhaseTransition`] boundaries to its caller and drives emission only
//! through a caller-supplied hook, paced across each phase's gap — so this crate stays free of any
//! dependency on the emit seam. [`run_timeline`] is the timing-only form. A validated
//! `conductor_core::Scenario`'s phase sequence converts into a `PhaseTimeline` via `From` (the
//! `convert` module).

mod convert;
mod phase;
mod scheduler;

pub use phase::{Phase, PhaseTimeline, PhaseTransition};
pub use scheduler::{
    run_timeline, run_timeline_observed, run_timeline_with, EmissionPoint, PhaseWindow,
    TimelineError,
};
