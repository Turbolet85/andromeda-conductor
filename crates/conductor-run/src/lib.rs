//! `conductor-run` — the run composition root shared by both shells.
//!
//! Drives one scenario (or a suite) to its [`conductor_core::RunRecord`]s. The MCP preflight gate is the pivot: when
//! the read-back path is unreachable or not ready, the scenario short-circuits to a `Blocked` record
//! (never a silent downgrade — arch §Read-Back Dependency Posture). On a ready gate it drives the
//! seeded timeline with the per-phase [`Dispatcher`] attached, so each phase's declared emission
//! shape reaches the wire inside that phase's own window, then reads back and classifies. Both
//! `conductor-cli` (the release gate) and `conductor-tauri` (the GUI) call this identically — the
//! "headless-drivable core, thin shells" split (arch §Design Philosophy). Read-back is per-check:
//! `conductor_verify::observe` composes one pass over the corpus tools and each check grades against
//! the value its `ComparisonKind` reads.
//!
//! The resolver is generic ([`execute_scenario`]`<R: PauseResolver>`) — the CLI passes its interactive
//! `CliResolver`, the GUI the core [`conductor_core::HeadlessResolver`] — never a trait object
//! (`PauseResolver::resolve` returns `impl Future`, so it is not object-safe; this mirrors the generic
//! `conductor_core::resolve_hold`).

mod canary;
mod dispatch;
mod drive;
mod envelope;
mod execute;
mod lifecycle;
mod preconditions;
#[cfg(test)]
mod testkit;

pub use canary::{
    CANARY_SERVICE_NAME, CANARY_STORM_COUNT, Preflight, canary_spec, canary_storm_seed,
    canary_warmup_seed, emit_canary_storm, preflight, readiness,
};
pub use dispatch::{DispatchError, Dispatcher};
pub use drive::{RunEvent, RunStage, drive_run};
pub use envelope::{classify_run, persist, read_envelope};
pub use execute::{ScenarioOutcome, execute_scenario};
pub use lifecycle::{
    AUTO_RESOLVE_IDLE_SECONDS, LifecycleObservation, LifecycleVerdict, attribute_by_liveness,
    evaluate_lifecycle, probe_resolve_lifecycle, select_resolve_target,
};
pub use preconditions::observe_preconditions;
