//! Runtime-agnostic engine library — the shared types every Conductor seam depends on.
//!
//! `conductor-core` is the dependency root of the workspace. It owns the verification
//! outcome vocabulary ([`Verdict`] / [`ReportState`]), the scenario identity model
//! ([`Scenario`] / [`PId`] / [`SloTier`]) with its garde validation, and the harness-fault
//! [`CoreError`] — the two halves of the *verdict/error wall* (outcomes are `Ok` values; `Err`
//! is harness faults only). Plain data types — no async, and no I/O beyond the single sync
//! `std::fs` path-handle guard ([`resolve_under`]).

mod config_path;
mod error;
mod report_state;
mod scenario;
mod verdict;

pub use config_path::resolve_under;
pub use error::{CoreError, Result};
pub use report_state::ReportState;
pub use scenario::{PId, Scenario, SloTier};
pub use verdict::Verdict;
