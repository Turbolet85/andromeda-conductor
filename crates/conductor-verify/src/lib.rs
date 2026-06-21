//! MCP read-back verification seam (`conductor-verify`).
//!
//! Shipped so far: [`ReadbackClient`] — the rmcp client/transport foundation. It spawns the Pulse MCP
//! sidecar over stdio from a fixed program path (the data-dir is passed via `.env` after
//! injection-metacharacter rejection — the rmcp STDIO injection class, security-plan §Anti-Patterns),
//! negotiates the protocol version *down* to Pulse's hand-rolled `2024-11-05` (architecture
//! §Established Decisions [MCP Read-Back Client]), and exposes a typed surface over the four consumed
//! read-back tools.
//!
//! A failed spawn / handshake / call is a typed [`VerifyError`] value (`Result::Err`) — a harness
//! fault, never a verification verdict; a protocol-version difference or an absent tool is reported as
//! data the later preflight gate maps to a `Blocked` state (the verdict/error wall).
//!
//! The preflight readiness gate, the data-dir canary round-trip, the OTLP egress liveness check, and
//! verdict classification build on this handle in later Epoch-5 chunks.

mod client;
mod error;
mod spawn;

pub use client::{
    MARK_INCIDENT_RESOLVED, QUERY_INCIDENT_LIST, RETRIEVE_REPORT, RETRIEVE_TELEMETRY_SLICE,
    ReadbackClient,
};
pub use error::VerifyError;
