//! Deterministic seeded phase scheduler (tokio `current_thread`).
//!
//! The timeline engine's determinism substrate: [`run_timeline`] sequences an ordered
//! [`PhaseTimeline`] on `tokio::time`, perturbing each inter-phase gap by a seed-derived bounded
//! jitter so the same seed reproduces the same emission-stream shape (architecture §Design
//! Philosophy). It surfaces [`PhaseTransition`] boundaries to its caller and emits nothing — OTLP
//! emission and the run journal are later seams/chunks; a validated `conductor_core::Scenario`'s
//! phase sequence converts into a `PhaseTimeline` via `From` (the `convert` module).

mod convert;
mod phase;
mod scheduler;

pub use phase::{Phase, PhaseTimeline, PhaseTransition};
pub use scheduler::{run_timeline, TimelineError};
