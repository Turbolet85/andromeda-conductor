//! Runtime-agnostic engine library — the shared types every Conductor seam depends on.
//!
//! `conductor-core` is the dependency root of the workspace. It owns the verification
//! outcome vocabulary ([`Verdict`] / [`ReportState`]), the scenario identity model
//! ([`Scenario`] / [`PId`] / [`SloTier`]), and the harness-fault [`CoreError`] — the two
//! halves of the *verdict/error wall* (outcomes are `Ok` values; `Err` is harness faults
//! only). Plain data types: no async, no I/O.

mod error;
mod report_state;
mod scenario;
mod verdict;

pub use error::{CoreError, Result};
pub use report_state::ReportState;
pub use scenario::{PId, Scenario, SloTier};
pub use verdict::Verdict;
