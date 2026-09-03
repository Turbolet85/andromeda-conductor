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
mod pause;

fn main() {
    init_observability("conductor-tauri", None, obs_sink());
    tauri::Builder::default()
        .manage(commands::RunControl::default())
        .manage(pause::HoldGate::default())
        .invoke_handler(tauri::generate_handler![
            commands::list_scenarios,
            commands::coverage_matrix,
            commands::unbacked_auto,
            commands::run_report,
            commands::run_envelope,
            commands::start_run,
            commands::stop_run,
            pause::resolve_operator_hold,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_self_obs_sink_resolves_beside_the_runs_dir() {
        // obs-plan §3 Log file location: the file NAME is fixed and the DIRECTORY rides
        // CONDUCTOR_RUNS_DIR, so asserting the tail is env-robust. `init_observability` is a
        // process-global, first-install-wins singleton (test-plan §11) and is deliberately not
        // called here — only the path derivation is under test.
        let path = tauri_log_path().expect("the sink path resolves under an existing base dir");
        assert!(path.is_absolute(), "the sink path is absolute, got {path:?}");
        // Path::ends_with compares whole COMPONENTS: a string suffix would fail on Windows, where
        // this renders `logs\conductor-tauri.jsonl`.
        assert!(
            path.ends_with(Path::new("logs").join("conductor-tauri.jsonl")),
            "the sink lands at <runs_dir.parent()>/logs/conductor-tauri.jsonl, got {path:?}"
        );
    }

    #[test]
    fn the_obs_sink_is_a_file_sink_whenever_the_path_resolves() {
        // obs-plan §3: the file sink is UNCONDITIONAL, with stderr only as an open-failure fallback.
        // A tauri_log_path that returned None would silently downgrade the whole stream to stderr.
        assert!(
            matches!(obs_sink(), ObsSink::File(_)),
            "a resolvable sink path must yield a File sink, never the stderr fallback"
        );
    }
}
