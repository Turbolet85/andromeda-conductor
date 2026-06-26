//! `conductor-tauri` — desktop control-panel GUI (frameless window shell).
//!
//! A thin Tauri 2 shell over the runtime-agnostic `conductor-core`; the headless `conductor-cli`
//! stays the release gate (architecture §Headless-drivable core, thin shells). Self-observation
//! routes to `logs/conductor-tauri.jsonl` via the shared `ObsSink::File` sink (obs-plan §3); Tauri
//! owns its own event loop — the core `current_thread` runtime is not wired in here yet.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};

use conductor_core::{init_observability, resolve_under, ObsSink};

mod commands;

fn main() {
    init_observability("conductor-tauri", None, obs_sink());
    tauri::Builder::default()
        .manage(commands::RunControl::default())
        .invoke_handler(tauri::generate_handler![
            commands::list_scenarios,
            commands::start_run,
            commands::stop_run,
        ])
        .run(tauri::generate_context!())
        .expect("error while running conductor-tauri");
}

/// The Tauri backend self-obs sink — `logs/conductor-tauri.jsonl` as a sibling of the runs dir,
/// moving with `CONDUCTOR_RUNS_DIR` (obs-plan §3), falling back to stderr when the path can't be
/// resolved. Mirrors `conductor-cli`'s `agent_log_path`.
fn obs_sink() -> ObsSink {
    tauri_log_path().map(ObsSink::File).unwrap_or(ObsSink::Stderr)
}

fn tauri_log_path() -> Option<PathBuf> {
    let base = std::env::current_dir().ok()?;
    let candidate = std::env::var("CONDUCTOR_RUNS_DIR").unwrap_or_else(|_| "runs".to_string());
    let runs_dir = resolve_under(&base, Path::new(&candidate)).ok()?;
    let logs_dir = runs_dir.parent().map(Path::to_path_buf).unwrap_or(base).join("logs");
    Some(logs_dir.join("conductor-tauri.jsonl"))
}
