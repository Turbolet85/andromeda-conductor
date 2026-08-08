//! Tauri command surface — scenario/suite picker + live-streaming run controls (Epoch 9 ch4).
//!
//! `list_scenarios` reads the catalog (read-only). `start_run` drives the real run pipeline
//! (`conductor_run`) on a core-owned `current_thread` runtime spun on a background thread — Tauri's
//! own multi_thread shell stays the GUI's concern (arch §Async Runtime Flavor) — and streams a live
//! [`RunEvent`] per scenario over the IPC `Channel`; `stop_run` sets the cooperative abort flag the
//! run thread polls. Without a live Pulse every scenario resolves `Blocked` (the preflight gate), so
//! the run persists a real Blocked `RunRecord` (runs.db + journal, identical to the CLI — test-plan
//! Path 7) while the per-emission counter stays 0 (Epoch-10 bridge). Errors are sanitized at this edge
//! before reaching the webview (security-plan §Error Handling).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use conductor_core::{
    mint_run_id, resolve_under, sanitize_error, validate_selection, CapabilityManifest,
    CapabilityRow, RunRecord, Scenario, ScenarioSummary, SUITE_SELECTION,
};
use conductor_run::{RunEvent, RunStage};
use tauri::ipc::Channel;

use crate::pause::{HoldGate, HoldPrompt, TauriResolver};

/// The managed run-lifecycle handle. `abort` is the cooperative stop flag the background run thread
/// polls between scenarios (the GUI stop button); the frontend renders the run state from the live
/// `Channel`, so the backend keeps no phase mirror.
#[derive(Default)]
pub struct RunControl {
    abort: Arc<AtomicBool>,
}

/// Resolve a `CONDUCTOR_*` artifact handle under the current dir through the same traversal guard the
/// CLI uses (`paths.rs`), rejecting traversal / absolute escapes before any IO.
fn resolve_handle(var: &str, default: &str) -> Result<PathBuf, String> {
    let base = std::env::current_dir().map_err(|e| sanitize_error(&e))?;
    let candidate = std::env::var(var).unwrap_or_else(|_| default.to_string());
    resolve_under(&base, Path::new(&candidate)).map_err(|e| sanitize_error(&e))
}

fn scenarios_dir() -> Result<PathBuf, String> {
    resolve_handle("CONDUCTOR_SCENARIOS_DIR", "scenarios")
}

fn runs_dir() -> Result<PathBuf, String> {
    resolve_handle("CONDUCTOR_RUNS_DIR", "runs")
}

fn manifest_path() -> Result<PathBuf, String> {
    resolve_handle("CONDUCTOR_CONTRACT_MANIFEST", "contracts/mcp-contract.toml")
}

/// The SUT capability set every loaded scenario's P-IDs are checked against. A fixed in-repo path
/// (no `CONDUCTOR_*` override), resolved through the same traversal guard as the other handles; a
/// missing or malformed manifest is a harness fault.
fn capabilities() -> Result<CapabilityManifest, String> {
    let base = std::env::current_dir().map_err(|e| sanitize_error(&e))?;
    let path = resolve_under(&base, &CapabilityManifest::default_path())
        .map_err(|e| sanitize_error(&e))?;
    CapabilityManifest::load(&path).map_err(|e| sanitize_error(&e))
}

/// Resolve a validated picker selection (a scenario name or the suite sentinel) to its scenarios.
fn resolve_selection(
    dir: &Path,
    selection: &str,
    capabilities: &CapabilityManifest,
) -> Result<Vec<Scenario>, String> {
    if selection == SUITE_SELECTION {
        load_all(dir, capabilities)
    } else {
        Ok(vec![load_one(dir, selection, capabilities)?])
    }
}

fn load_all(dir: &Path, capabilities: &CapabilityManifest) -> Result<Vec<Scenario>, String> {
    let files = conductor_core::scenario_files(dir).map_err(|e| sanitize_error(&e))?;
    let mut scenarios = Vec::new();
    for path in files {
        scenarios.push(load_toml(&path, capabilities)?);
    }
    Ok(scenarios)
}

fn load_one(
    dir: &Path,
    name: &str,
    capabilities: &CapabilityManifest,
) -> Result<Scenario, String> {
    let path = resolve_under(dir, Path::new(&format!("{name}.toml"))).map_err(|e| sanitize_error(&e))?;
    load_toml(&path, capabilities)
}

fn load_toml(path: &Path, capabilities: &CapabilityManifest) -> Result<Scenario, String> {
    let text = std::fs::read_to_string(path).map_err(|e| sanitize_error(&e))?;
    Scenario::from_toml_str_with(&text, capabilities).map_err(|e| sanitize_error(&e))
}

#[tauri::command]
pub fn list_scenarios() -> Result<Vec<ScenarioSummary>, String> {
    let _span = tracing::info_span!("tauri.command.list_scenarios").entered();
    let started = Instant::now();
    let dir = scenarios_dir()?;
    let summaries = list_scenarios_impl(&dir, &capabilities()?)?;
    tracing::info!(count = summaries.len(), latency_ms = started.elapsed().as_millis() as u64, "listed scenarios");
    Ok(summaries)
}

fn list_scenarios_impl(
    dir: &Path,
    capabilities: &CapabilityManifest,
) -> Result<Vec<ScenarioSummary>, String> {
    conductor_core::list_scenarios(dir, capabilities).map_err(|e| sanitize_error(&e))
}

/// The 60-P-ID capability coverage classification (read-only) — the desktop twin of `conductor
/// coverage` / `coverage-matrix.md`. The view single-sources `conductor_core::coverage_matrix()`;
/// it never re-authors the table. Infallible (a `static`, no IO), but kept `Result` for a uniform
/// command surface with `list_scenarios`.
#[tauri::command]
pub fn coverage_matrix() -> Result<Vec<CapabilityRow>, String> {
    let _span = tracing::info_span!("tauri.command.coverage_matrix").entered();
    let started = Instant::now();
    let rows = conductor_core::coverage_matrix().to_vec();
    tracing::info!(
        count = rows.len(),
        latency_ms = started.elapsed().as_millis() as u64,
        "listed coverage matrix"
    );
    Ok(rows)
}

/// The persisted run report (read-only) — the per-scenario `RunRecord`s of a run's JSONL journal, the
/// desktop twin of `conductor report` / the Markdown report. `run_id` defaults to the latest run; a
/// supplied id is `resolve_under`-guarded against traversal before any read (security-plan §Input
/// Validation). An absent/empty runs dir yields an empty list (the webview renders "No run yet"),
/// never an error. Single-sources `conductor_core::read_run_journal`; the envelope is never re-authored.
#[tauri::command]
pub fn run_report(run_id: Option<String>) -> Result<Vec<RunRecord>, String> {
    let _span = tracing::info_span!("tauri.command.run_report").entered();
    let started = Instant::now();
    let dir = runs_dir()?;
    let id = match run_id {
        Some(id) => {
            resolve_under(&dir, Path::new(&format!("{id}.jsonl"))).map_err(|e| sanitize_error(&e))?;
            id
        }
        None => match conductor_core::latest_run_id(&dir).map_err(|e| sanitize_error(&e))? {
            Some(id) => id,
            None => {
                tracing::info!(
                    count = 0,
                    latency_ms = started.elapsed().as_millis() as u64,
                    "no run to report"
                );
                return Ok(Vec::new());
            }
        },
    };
    let records = conductor_core::read_run_journal(&dir, &id).map_err(|e| sanitize_error(&e))?;
    tracing::info!(
        count = records.len(),
        latency_ms = started.elapsed().as_millis() as u64,
        "read run report"
    );
    Ok(records)
}

#[tauri::command]
pub fn start_run(
    selection: String,
    on_event: Channel<RunEvent>,
    on_hold: Channel<HoldPrompt>,
    state: tauri::State<'_, RunControl>,
    hold_gate: tauri::State<'_, HoldGate>,
) -> Result<(), String> {
    let _span = tracing::info_span!("tauri.command.start_run").entered();
    let dir = scenarios_dir()?;
    let capabilities = capabilities()?;
    if !validate_selection(&list_scenarios_impl(&dir, &capabilities)?, &selection) {
        return Err("unknown scenario or suite selection".to_string());
    }
    let scenarios = resolve_selection(&dir, &selection, &capabilities)?;
    let runs_dir = runs_dir()?;
    let manifest = manifest_path()?;
    let run_id = mint_run_id();
    let abort = state.abort.clone();
    abort.store(false, Ordering::SeqCst);
    let resolver = TauriResolver::new(hold_gate.inner().clone(), on_hold);
    tracing::info!(run_id = %run_id, "run starting (background)");
    std::thread::spawn(move || {
        run_thread(scenarios, run_id, runs_dir, manifest, abort, resolver, on_event)
    });
    Ok(())
}

/// The background run driver — a core-owned `current_thread` runtime that drives the pipeline and
/// streams progress. A terminal `Aborted` event is always sent on a harness fault so the webview
/// never hangs on a half-finished run.
fn run_thread(
    scenarios: Vec<Scenario>,
    run_id: String,
    runs_dir: PathBuf,
    manifest: PathBuf,
    abort: Arc<AtomicBool>,
    resolver: TauriResolver,
    on_event: Channel<RunEvent>,
) {
    let runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(rt) => rt,
        Err(err) => {
            tracing::error!("run runtime build failed: {}", sanitize_error(&err));
            let _ = on_event.send(RunEvent { stage: RunStage::Aborted, count: 0 });
            return;
        }
    };
    runtime.block_on(async move {
        let preflight = match conductor_run::preflight(&manifest).await {
            Ok(pf) => pf,
            Err(err) => {
                tracing::error!("preflight failed: {}", sanitize_error(&*err));
                let _ = on_event.send(RunEvent { stage: RunStage::Aborted, count: 0 });
                return;
            }
        };
        let emit = |event| {
            let _ = on_event.send(event);
        };
        let result = conductor_run::drive_run(
            &preflight,
            &scenarios,
            &run_id,
            &runs_dir,
            &resolver,
            emit,
            || abort.load(Ordering::SeqCst),
        )
        .await;
        if let Err(err) = result {
            tracing::error!("run failed: {}", sanitize_error(&*err));
            let _ = on_event.send(RunEvent { stage: RunStage::Aborted, count: 0 });
        }
    });
}

#[tauri::command]
pub fn stop_run(state: tauri::State<'_, RunControl>) -> Result<(), String> {
    let _span = tracing::info_span!("tauri.command.stop_run").entered();
    state.abort.store(true, Ordering::SeqCst);
    tracing::info!("run stop requested");
    Ok(())
}

#[cfg(test)]
mod tests {
    //! GUI-integration tests (Epoch 9 ch9). The `tauri::test` mock-runtime exercises the synchronous
    //! command contracts through the real IPC dispatch; the Path-7 leg proves cross-surface envelope
    //! parity. The live `start_run` background-thread `Channel` frame *sequence* + the real-webview
    //! axe/keyboard sweep are display-gated (Linux+xvfb + live Pulse — a11y-plan §3.5, test-plan §6).
    use super::*;
    use conductor_core::{read_run_journal, HeadlessResolver, ReportState};
    use tauri::ipc::{CallbackFn, InvokeBody};
    use tauri::test::{get_ipc_response, mock_builder, mock_context, noop_assets, INVOKE_KEY};
    use tauri::webview::InvokeRequest;
    use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

    type MockApp = tauri::App<tauri::test::MockRuntime>;
    type MockWindow = tauri::WebviewWindow<tauri::test::MockRuntime>;

    fn test_app() -> MockApp {
        mock_builder()
            .manage(RunControl::default())
            .manage(crate::pause::HoldGate::default())
            .invoke_handler(tauri::generate_handler![
                list_scenarios,
                coverage_matrix,
                run_report,
                start_run,
                stop_run,
                crate::pause::resolve_operator_hold,
            ])
            .build(mock_context(noop_assets()))
            .expect("mock app builds with the full command handler")
    }

    fn main_window(app: &MockApp) -> MockWindow {
        WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
            .build()
            .expect("mock webview builds")
    }

    fn invoke(window: &MockWindow, cmd: &str, body: InvokeBody) -> tauri::ipc::InvokeResponseBody {
        get_ipc_response(
            window,
            InvokeRequest {
                cmd: cmd.into(),
                callback: CallbackFn(0),
                error: CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(),
                body,
                headers: Default::default(),
                invoke_key: INVOKE_KEY.to_string(),
            },
        )
        .unwrap_or_else(|e| panic!("command `{cmd}` dispatched with an error: {e}"))
    }

    #[test]
    fn mock_app_registers_the_full_command_surface() {
        // The mock runtime accepts manage(...) + the generate_handler! list — the GUI-integration
        // harness foundation (test-plan §5). A build failure here means a command/state mismatch.
        let _app = test_app();
    }

    #[test]
    fn coverage_matrix_command_returns_all_sixty_pids() {
        let app = test_app();
        let window = main_window(&app);
        // CapabilityRow is Serialize-only (a command return, never read back), so assert on the JSON
        // shape rather than deserializing the concrete type.
        let rows: serde_json::Value = invoke(&window, "coverage_matrix", InvokeBody::default())
            .deserialize()
            .expect("coverage rows deserialize");
        assert_eq!(
            rows.as_array().map(|a| a.len()),
            Some(60),
            "all P-001..P-060 surface through the IPC dispatch"
        );
    }

    #[test]
    fn run_report_command_returns_a_well_formed_record_list() {
        let app = test_app();
        let window = main_window(&app);
        // No run_id arg ⇒ latest; an absent/empty runs dir yields [] ("No run yet"), never an error.
        let _records: Vec<RunRecord> = invoke(&window, "run_report", InvokeBody::Json(serde_json::json!({})))
            .deserialize()
            .expect("run_report returns a Vec<RunRecord>");
    }

    #[test]
    fn stop_run_command_sets_the_cooperative_abort_flag() {
        let app = test_app();
        let window = main_window(&app);
        let _ = invoke(&window, "stop_run", InvokeBody::Json(serde_json::json!({})));
        assert!(
            app.state::<RunControl>().abort.load(Ordering::SeqCst),
            "stop_run flips the cooperative abort flag the run thread polls"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn path7_tauri_persists_the_same_blocked_envelope_as_the_cli() {
        // Cross-surface parity (test-plan §6 Path 7): the Tauri crate drives the SAME conductor_run
        // composition the start_run thread runs (preflight → drive_run → persist) and must persist the
        // identical Blocked envelope the CLI's cli_smoke proves — same scenario+seed. The live
        // start_run thread/Channel frame stream is the display-gated leg (a11y-plan §3.5).
        let dir = assert_fs::TempDir::new().unwrap();
        let manifest = format!("{}/../../contracts/mcp-contract.toml", env!("CARGO_MANIFEST_DIR"));
        let toml = std::fs::read_to_string(format!(
            "{}/../../scenarios/error-baseline-spike.toml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let scenario = Scenario::from_toml_str(&toml).unwrap();

        // SAFETY: nextest runs each test in its own process; set before any runtime/thread reads env.
        // The injection metachar forces the read-back path unreachable (rejected pre-spawn) — the
        // host-independent Blocked lever cli_smoke uses via the child .env (security-plan §Input Validation).
        unsafe { std::env::set_var("ANDROMEDA_PULSE_DATA_DIR", "pulse;injection") };

        let pf = conductor_run::preflight(Path::new(&manifest)).await.unwrap();
        conductor_run::drive_run(
            &pf,
            std::slice::from_ref(&scenario),
            "run-path7",
            dir.path(),
            &HeadlessResolver::proceed(),
            |_| {},
            || false,
        )
        .await
        .expect("the blocked spine is infallible");

        let records = read_run_journal(dir.path(), "run-path7").expect("the journal was persisted");
        assert_eq!(records.len(), 1);
        assert!(matches!(records[0].state, ReportState::Blocked), "no live Pulse ⇒ Blocked");
        assert!(records[0].verdict.is_none(), "a blocked row carries no verdict");
        assert_eq!(records[0].seed, scenario.seed);
        assert_eq!(records[0].scenario, scenario.name);
    }
}
