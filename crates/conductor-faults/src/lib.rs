//! Fault helpers — the Epoch-4 fault-injection seam (ramps · silence · bursty-train · port-occupier · fingerprint-storm).
//!
//! Each helper produces one fault condition Conductor injects into Pulse's environment. Shipped so far:
//! [`PortOccupier`] (P-003) — a sacrificial loopback bind on Pulse's OTLP ingest port, Conductor's
//! *sole* deliberate inbound listener (every other surface is a pure client); loopback-only and released
//! on cleanup (architecture §Cross-cutting Patterns — Trust boundary). A refused bind is a typed
//! [`FaultError`] value (`Result::Err`), never a panic — the verdict/error wall.
//! [`EmissionGap`] (P-015) — an exact-length emission gap that *resumes*; a gap past the 20s threshold
//! reads as a service restart. An out-of-range gap is a typed [`FaultError`] value, never a panic.
//! [`AbruptSilence`] (P-014) — an abrupt *permanent* emission stop with no resume (the inverse of
//! [`EmissionGap`]); the activity-floor "service died" lever. Construction is infallible — a permanent
//! stop has nothing to validate, so it carries no [`FaultError`] variant.
//! [`BurstyTrain`] (P-013) — a *repeating* active/quiet emission duty cycle (canonically 5 min / 10
//! min); the activity-floor false-positive guard, the healthy-but-intermittent counterpart to
//! [`AbruptSilence`] and [`EmissionGap`] whose long quiet windows must not read as dead. An
//! out-of-range duty cycle is a typed [`FaultError`] value, never a panic.

mod error;
mod gap;
mod port_occupier;
mod silence;
mod train;

pub use error::FaultError;
pub use gap::{EmissionGap, MIN_GAP, MAX_GAP};
pub use port_occupier::{PortOccupier, OTLP_INGEST_PORT};
pub use silence::AbruptSilence;
pub use train::{BurstyTrain, MAX_WINDOW, CANONICAL_ACTIVE, CANONICAL_QUIET};
