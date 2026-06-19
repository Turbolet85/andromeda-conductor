//! Fault helpers — the Epoch-4 fault-injection seam (ramps · silence · port-occupier · fingerprint-storm).
//!
//! Each helper produces one fault condition Conductor injects into Pulse's environment. Shipped so far:
//! [`PortOccupier`] (P-003) — a sacrificial loopback bind on Pulse's OTLP ingest port, Conductor's
//! *sole* deliberate inbound listener (every other surface is a pure client); loopback-only and released
//! on cleanup (architecture §Cross-cutting Patterns — Trust boundary). A refused bind is a typed
//! [`FaultError`] value (`Result::Err`), never a panic — the verdict/error wall.

mod error;
mod port_occupier;

pub use error::FaultError;
pub use port_occupier::{PortOccupier, OTLP_INGEST_PORT};
