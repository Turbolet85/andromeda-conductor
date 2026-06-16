//! Runtime-agnostic engine library — the shared types every Conductor seam depends on.
//!
//! `conductor-core` is the dependency root of the workspace. It owns the verification
//! outcome vocabulary ([`Verdict`] / [`ReportState`]), the scenario identity model
//! ([`Scenario`] / [`PId`] / [`SloTier`]) with its garde validation, and the harness-fault
//! [`CoreError`] — the two halves of the *verdict/error wall* (outcomes are `Ok` values; `Err`
//! is harness faults only). Beyond those plain data types it exposes one side-effecting
//! entrypoint — the self-observation init ([`init_observability`]), which installs the global
//! `tracing` JSON subscriber + `std::panic` hook so the CLI and Tauri shells initialize logging
//! identically; otherwise no async and no I/O beyond the single sync `std::fs` path-handle guard
//! ([`resolve_under`]).

mod config_path;
mod error;
mod obs;
mod phase_spec;
mod redact;
mod report_state;
mod scenario;
mod verdict;

pub use config_path::resolve_under;
pub use error::{CoreError, Result};
pub use obs::{init_observability, mint_run_id, ServiceIdentity};
pub use phase_spec::{EmissionSpec, PhaseSpec, Signal};
pub use redact::{redact_value, sanitize_error};
pub use report_state::ReportState;
pub use scenario::{PId, Scenario, SloTier};
pub use verdict::Verdict;
