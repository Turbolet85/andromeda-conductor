//! Runtime-agnostic engine library — the shared types every Conductor seam depends on.
//!
//! `conductor-core` is the dependency root of the workspace. It owns the verification
//! outcome vocabulary ([`Verdict`] / [`ReportState`]) plus the run-report envelope
//! ([`RunRecord`]) the journal records, the scenario identity model
//! ([`Scenario`] / [`PId`] / [`SloTier`]) with its garde validation, and the harness-fault
//! [`CoreError`] — the two halves of the *verdict/error wall* (outcomes are `Ok` values; `Err`
//! is harness faults only). Beyond those plain data types it exposes the operator-pause resolver
//! abstraction ([`PauseResolver`] / [`resolve_hold`]) — the crate's one async surface, a
//! runtime-agnostic `impl Future` carrying no runtime dependency — and one side-effecting
//! entrypoint — the self-observation init ([`init_observability`]), which installs the global
//! `tracing` JSON subscriber + `std::panic` hook so the CLI and Tauri shells initialize logging
//! identically; otherwise no I/O beyond the single sync `std::fs` path-handle guard
//! ([`resolve_under`]).

mod config_path;
mod error;
mod expected;
mod lamp;
mod obs;
mod pause;
mod phase_spec;
mod redact;
mod report_state;
mod run_record;
mod scenario;
mod verdict;

pub use config_path::resolve_under;
pub use error::{CoreError, Result};
pub use expected::{ClaimClass, ComparisonKind, ExpectedCheck};
pub use lamp::Lamp;
pub use obs::{init_observability, mint_run_id, now_rfc3339, ServiceIdentity};
pub use pause::{Decision, HeadlessResolver, HoldPoint, HoldResolution, PauseResolver, resolve_hold};
pub use phase_spec::{EmissionSpec, PhaseSpec, Signal};
pub use redact::{redact_value, sanitize_error};
pub use report_state::ReportState;
pub use run_record::RunRecord;
pub use scenario::{PId, Scenario, SloTier};
pub use verdict::Verdict;
