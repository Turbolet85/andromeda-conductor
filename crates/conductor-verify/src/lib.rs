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
//! `CalibrationRegion`, never hard-failed on exact values). And the expected-outcome + SLO timing
//! layer ([`compare`] / [`evaluate_slo`] / [`evaluate_check`]): it applies a
//! [`conductor_core::ExpectedCheck`]'s comparison kind to a read-back value and folds in the
//! journal-relative tier deadline, producing the `matched`/class the classifier consumes. And
//! finally the producer bridge ([`CheckOutcome::to_run_record`]) — folding a fully-evaluated outcome
//! into the canonical [`conductor_core::RunRecord`] the Epoch-6 report seam persists.

mod client;
mod error;
mod jsonrpc;
mod manifest;
mod preflight;
mod record;
mod slo;
mod spawn;
mod verdict;

pub use client::{
    MARK_INCIDENT_RESOLVED, QUERY_INCIDENT_LIST, RETRIEVE_REPORT, RETRIEVE_TELEMETRY_SLICE,
    ReadbackClient,
};
pub use error::VerifyError;
pub use manifest::{ContractManifest, READBACK_TOOLS};
pub use conductor_core::ClaimClass;
pub use preflight::{
    CanaryMarker, CanaryOutcome, CanaryPoll, ReadyState, ToolPresence, preflight_boot, run_preflight,
};
pub use slo::{CheckOutcome, SloOutcome, compare, evaluate_check, evaluate_slo};
pub use verdict::{Assessment, classify};
