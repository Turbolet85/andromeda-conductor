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
//! Now shipped on top: the preflight readiness gate ([`run_preflight`] / [`preflight_boot`]) —
//! protocol-version pin + required-tool presence (against the pinned [`ContractManifest`]) + the
//! data-dir canary read-back, each a distinct [`ReadyState`] `Blocked` precondition. And on top of
//! that: verdict classification ([`classify`] / [`Assessment`]) — the two-state assertion policy that
//! turns a check's outcome into a `Verdict` (hard `Pass`/`Fail` vs a model-interpretive
//! `CalibrationRegion`, never hard-failed on exact values).

mod client;
mod error;
mod manifest;
mod preflight;
mod spawn;
mod verdict;

pub use client::{
    MARK_INCIDENT_RESOLVED, QUERY_INCIDENT_LIST, RETRIEVE_REPORT, RETRIEVE_TELEMETRY_SLICE,
    ReadbackClient,
};
pub use error::VerifyError;
pub use manifest::{ContractManifest, READBACK_TOOLS};
pub use preflight::{
    CanaryMarker, CanaryOutcome, ReadyState, ToolPresence, preflight_boot, run_preflight,
};
pub use verdict::{Assessment, ClaimClass, classify};
