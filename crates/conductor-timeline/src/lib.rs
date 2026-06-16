//! Deterministic seeded phase scheduler (tokio `current_thread`).
//!
//! The timeline engine's determinism substrate: [`run_timeline`] sequences an ordered
//! [`PhaseTimeline`] on `tokio::time`, perturbing each inter-phase gap by a seed-derived bounded
//! jitter so the same seed reproduces the same emission-stream shape (architecture §Design
//! Philosophy). It surfaces [`PhaseTransition`] boundaries to its caller and emits nothing — OTLP
//! emission, the run journal, and the declarative per-phase spec are later seams/chunks.

mod phase;
mod scheduler;

pub use phase::{Phase, PhaseTimeline, PhaseTransition};
pub use scheduler::{run_timeline, TimelineError};
