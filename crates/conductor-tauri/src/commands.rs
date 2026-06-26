//! Tauri command surface — scenario/suite picker + run controls (Epoch 9 ch3).
//!
//! `list_scenarios` reads the catalog (read-only); `start_run`/`stop_run` drive a backend
//! run-lifecycle state the titlebar renders. This chunk is a control scaffold — `start_run`
//! validates the selection and transitions state but does NOT execute the run pipeline (that, plus
//! the live `Channel`, lands in ch4). Errors are sanitized at this edge before reaching the webview
//! (security-plan §Error Handling).

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use conductor_core::{resolve_under, sanitize_error, validate_selection, ScenarioSummary};

/// The backend run-lifecycle phase, mirrored to the titlebar `RunState`. `hold` (operator pause) is
/// ch8; this chunk drives idle/live/aborted only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RunPhase {
    #[default]
    Idle,
    Running,
    Aborted,
}

impl RunPhase {
    /// The wire label the frontend `RunState` consumes (`Running` reads as `live`).
    fn label(self) -> &'static str {
        match self {
            RunPhase::Idle => "idle",
            RunPhase::Running => "live",
            RunPhase::Aborted => "aborted",
        }
    }
}

/// The managed run-lifecycle state.
pub type RunState = Mutex<RunPhase>;

/// Resolve the scenarios catalog dir under the current dir, honoring `CONDUCTOR_SCENARIOS_DIR`
/// through the same traversal guard the CLI uses (`paths.rs`).
fn scenarios_dir() -> Result<PathBuf, String> {
    let base = std::env::current_dir().map_err(|e| sanitize_error(&e))?;
    let candidate = std::env::var("CONDUCTOR_SCENARIOS_DIR").unwrap_or_else(|_| "scenarios".to_string());
    resolve_under(&base, Path::new(&candidate)).map_err(|e| sanitize_error(&e))
}

#[tauri::command]
pub fn list_scenarios() -> Result<Vec<ScenarioSummary>, String> {
    let _span = tracing::info_span!("tauri.command.list_scenarios").entered();
    let started = Instant::now();
    let dir = scenarios_dir()?;
    let summaries = list_scenarios_impl(&dir)?;
    tracing::info!(count = summaries.len(), latency_ms = started.elapsed().as_millis() as u64, "listed scenarios");
    Ok(summaries)
}

fn list_scenarios_impl(dir: &Path) -> Result<Vec<ScenarioSummary>, String> {
    conductor_core::list_scenarios(dir).map_err(|e| sanitize_error(&e))
}

#[tauri::command]
pub fn start_run(selection: String, state: tauri::State<'_, RunState>) -> Result<&'static str, String> {
    let _span = tracing::info_span!("tauri.command.start_run").entered();
    let started = Instant::now();
    let catalog = list_scenarios_impl(&scenarios_dir()?)?;
    if !validate_selection(&catalog, &selection) {
        return Err("unknown scenario or suite selection".to_string());
    }
    let mut phase = state.lock().map_err(|_| "run-state lock poisoned".to_string())?;
    *phase = RunPhase::Running;
    tracing::info!(latency_ms = started.elapsed().as_millis() as u64, "run started (control scaffold; execution lands ch4)");
    Ok(phase.label())
}

#[tauri::command]
pub fn stop_run(state: tauri::State<'_, RunState>) -> Result<&'static str, String> {
    let _span = tracing::info_span!("tauri.command.stop_run").entered();
    let mut phase = state.lock().map_err(|_| "run-state lock poisoned".to_string())?;
    *phase = RunPhase::Aborted;
    tracing::info!("run stopped");
    Ok(phase.label())
}
