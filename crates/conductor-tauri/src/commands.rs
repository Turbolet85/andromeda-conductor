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
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use conductor_core::{
    CapabilityManifest, CapabilityRow, EnvelopeStatus, LoadEnvelope, RunRecord, SUITE_SELECTION,
    Scenario, ScenarioSummary, mint_run_id, resolve_under, sanitize_error, validate_selection,
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

/// Resolve the runs dir and the run a read-only command should read, or `None` when no run exists.
///
/// The two run-data commands share this prologue exactly: a SUPPLIED `run_id` is
/// `resolve_under`-guarded against traversal before any read (an absolute candidate or a `..` escape
/// is REJECTED, never clamped — security-plan §Input Validation), an absent one resolves to the
/// latest run, and every fault crosses the edge through [`sanitize_error`]. `None` is the no-run
/// case each caller renders in its own empty shape, never an error.
fn resolve_run_target(run_id: Option<String>) -> Result<Option<(PathBuf, String)>, String> {
    let dir = runs_dir()?;
    let id = match run_id {
        Some(id) => {
            resolve_under(&dir, Path::new(&format!("{id}.jsonl")))
                .map_err(|e| sanitize_error(&e))?;
            id
        }
        None => match conductor_core::latest_run_id(&dir).map_err(|e| sanitize_error(&e))? {
            Some(id) => id,
            None => return Ok(None),
        },
    };
    Ok(Some((dir, id)))
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

/// The pinned SUT load envelope a run is judged against. A fixed in-repo path (no `CONDUCTOR_*`
/// override), resolved through the same traversal guard; a missing or malformed envelope is a
/// harness fault, never a silently unjudged run.
fn load_envelope() -> Result<LoadEnvelope, String> {
    let base = std::env::current_dir().map_err(|e| sanitize_error(&e))?;
    let path =
        resolve_under(&base, &LoadEnvelope::default_path()).map_err(|e| sanitize_error(&e))?;
    LoadEnvelope::load(&path).map_err(|e| sanitize_error(&e))
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

fn load_one(dir: &Path, name: &str, capabilities: &CapabilityManifest) -> Result<Scenario, String> {
    let path =
        resolve_under(dir, Path::new(&format!("{name}.toml"))).map_err(|e| sanitize_error(&e))?;
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
    tracing::info!(
        count = summaries.len(),
        latency_ms = started.elapsed().as_millis() as u64,
        "listed scenarios"
    );
    Ok(summaries)
}

fn list_scenarios_impl(
    dir: &Path,
    capabilities: &CapabilityManifest,
) -> Result<Vec<ScenarioSummary>, String> {
    conductor_core::list_scenarios(dir, capabilities).map_err(|e| sanitize_error(&e))
}

/// The capability coverage classification (read-only) — the desktop twin of `conductor
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

/// The capabilities classified `auto` that no scenario backs (read-only) — the webview half of the
/// coverage roll-up's unbacked qualifier. Single-sources `conductor_core::UNBACKED_AUTO`, which
/// `check_scenario_backing` holds to the committed catalog; the webview never mirrors the ledger in TS.
#[tauri::command]
pub fn unbacked_auto() -> Result<Vec<String>, String> {
    let _span = tracing::info_span!("tauri.command.unbacked_auto").entered();
    let started = Instant::now();
    let ids: Vec<String> = conductor_core::UNBACKED_AUTO
        .iter()
        .map(|s| s.to_string())
        .collect();
    tracing::info!(
        count = ids.len(),
        latency_ms = started.elapsed().as_millis() as u64,
        "listed unbacked auto claims"
    );
    Ok(ids)
}

/// The persisted run report (read-only) — the per-scenario `RunRecord`s of a run's JSONL journal, the
/// desktop twin of `conductor report` / the Markdown report. `run_id` defaults to the latest run; a
/// supplied id is `resolve_under`-guarded against traversal before any read (security-plan §Input
/// Validation). With `run_id` absent an empty or missing runs dir yields an empty list (the webview
/// renders "No run yet") rather than an error, because `latest_run_id` maps a failed `read_dir` to
/// `Ok(None)`; a SUPPLIED id resolves against a base that need not exist, so that arm can return
/// `Err`. Single-sources `conductor_core::read_run_journal`; the envelope is never re-authored.
#[tauri::command]
pub fn run_report(run_id: Option<String>) -> Result<Vec<RunRecord>, String> {
    let _span = tracing::info_span!("tauri.command.run_report").entered();
    let started = Instant::now();
    let Some((dir, id)) = resolve_run_target(run_id)? else {
        tracing::info!(
            count = 0,
            latency_ms = started.elapsed().as_millis() as u64,
            "no run to report"
        );
        return Ok(Vec::new());
    };
    let records = conductor_core::read_run_journal(&dir, &id).map_err(|e| sanitize_error(&e))?;
    tracing::info!(
        count = records.len(),
        latency_ms = started.elapsed().as_millis() as u64,
        "read run report"
    );
    Ok(records)
}

/// A run's load-envelope standing as the webview renders it. The `label` is carried from
/// [`EnvelopeStatus::label`] rather than re-spelled in TypeScript, so the always-rendered text that
/// makes the banner not-color-alone has one source (design-system §Iconography).
#[derive(serde::Serialize)]
pub struct EnvelopeStanding {
    label: &'static str,
    cause: Option<String>,
    suspect: bool,
}

impl From<EnvelopeStatus> for EnvelopeStanding {
    fn from(status: EnvelopeStatus) -> Self {
        Self {
            label: status.label(),
            cause: status.cause().map(str::to_string),
            suspect: status.is_suspect(),
        }
    }
}

/// The run's load-envelope standing (read-only) — the run-level qualifier the desktop run-report
/// banners, the third surface of the signal the cli caption and the Markdown report already carry.
/// `run_id` defaults to the latest run; a supplied id is `resolve_under`-guarded against traversal
/// before any read (security-plan §Input Validation). A run that recorded no envelope row yields
/// `None` — the banner is simply absent. So does an absent runs dir when `run_id` is absent, since
/// `latest_run_id` maps a failed `read_dir` to `Ok(None)`; a SUPPLIED id resolves against a base
/// that need not exist, so that arm can return `Err`.
#[tauri::command]
pub fn run_envelope(run_id: Option<String>) -> Result<Option<EnvelopeStanding>, String> {
    let _span = tracing::info_span!("tauri.command.run_envelope").entered();
    let started = Instant::now();
    let Some((dir, id)) = resolve_run_target(run_id)? else {
        tracing::info!(
            latency_ms = started.elapsed().as_millis() as u64,
            "no run to report an envelope for"
        );
        return Ok(None);
    };
    let standing = conductor_run::read_envelope(&dir, &id).map_err(|e| sanitize_error(&*e))?;
    tracing::info!(
        latency_ms = started.elapsed().as_millis() as u64,
        message = standing.as_ref().map_or("no envelope row", |s| s.label()),
        "read run envelope standing"
    );
    Ok(standing.map(EnvelopeStanding::from))
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
    let envelope = conductor_run::classify_run(&load_envelope()?, &scenarios);
    let run_id = mint_run_id();
    let abort = state.abort.clone();
    abort.store(false, Ordering::SeqCst);
    let resolver = TauriResolver::new(hold_gate.inner().clone(), on_hold);
    tracing::info!(run_id = %run_id, envelope = envelope.label(), "run starting (background)");
    std::thread::spawn(move || {
        run_thread(
            scenarios, run_id, runs_dir, manifest, envelope, abort, resolver, on_event,
        )
    });
    Ok(())
}

/// The background run driver — a core-owned `current_thread` runtime that drives the pipeline and
/// streams progress. A terminal `Aborted` event is always sent on a harness fault so the webview
/// never hangs on a half-finished run.
#[allow(clippy::too_many_arguments)] // the owned inputs the background thread takes across the
// std::thread boundary; each is moved, so a bundling struct would only rename the same list
fn run_thread(
    scenarios: Vec<Scenario>,
    run_id: String,
    runs_dir: PathBuf,
    manifest: PathBuf,
    envelope: EnvelopeStatus,
    abort: Arc<AtomicBool>,
    resolver: TauriResolver,
    on_event: Channel<RunEvent>,
) {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(err) => {
            tracing::error!("run runtime build failed: {}", sanitize_error(&err));
            let _ = on_event.send(RunEvent {
                stage: RunStage::Aborted,
                count: 0,
            });
            return;
        }
    };
    runtime.block_on(async move {
        let preflight = match conductor_run::preflight(&manifest).await {
            Ok(pf) => pf,
            Err(err) => {
                tracing::error!("preflight failed: {}", sanitize_error(&*err));
                let _ = on_event.send(RunEvent {
                    stage: RunStage::Aborted,
                    count: 0,
                });
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
            &envelope,
            &resolver,
            emit,
            || abort.load(Ordering::SeqCst),
        )
        .await;
        if let Err(err) = result {
            tracing::error!("run failed: {}", sanitize_error(&*err));
            let _ = on_event.send(RunEvent {
                stage: RunStage::Aborted,
                count: 0,
            });
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
    //! command contracts through the real IPC dispatch — the `resolve_under` / `sanitize_error`
    //! boundary that governs app-defined commands is provable only at this tier (security-plan
    //! §Input Validation). The Path-7 cross-surface parity leg lives in
    //! `conductor-cli/tests/cross_surface_parity.rs`, the package declaring the `conductor` bin, so
    //! its CLI arm binds through `CARGO_BIN_EXE_conductor` rather than a build artifact no edge
    //! guarantees. The live `start_run` background-thread `Channel` frame *sequence* + the
    //! real-webview axe/keyboard sweep are display-gated (a11y-plan §3.5, test-plan §6).
    use super::*;
    use tauri::ipc::{CallbackFn, InvokeBody};
    use tauri::test::{INVOKE_KEY, get_ipc_response, mock_builder, mock_context, noop_assets};
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
                unbacked_auto,
                run_report,
                run_envelope,
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

    /// One dispatched IPC request. The `url` MUST be the mock webview's real origin
    /// (`http://tauri.localhost`) — any other value fails dispatch with "Plugin not found"
    /// (`.claude/rules/testing.md` 2026-06-27), so both call sites share this one construction.
    fn request(cmd: &str, body: InvokeBody) -> InvokeRequest {
        InvokeRequest {
            cmd: cmd.into(),
            callback: CallbackFn(0),
            error: CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body,
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        }
    }

    fn invoke(window: &MockWindow, cmd: &str, body: InvokeBody) -> tauri::ipc::InvokeResponseBody {
        get_ipc_response(window, request(cmd, body))
            .unwrap_or_else(|e| panic!("command `{cmd}` dispatched with an error: {e}"))
    }

    #[test]
    fn mock_app_registers_the_full_command_surface() {
        // The mock runtime accepts manage(...) + the generate_handler! list — the GUI-integration
        // harness foundation (test-plan §5). A build failure here means a command/state mismatch.
        let _app = test_app();
    }

    #[test]
    fn coverage_matrix_command_returns_every_classified_pid() {
        let app = test_app();
        let window = main_window(&app);
        // CapabilityRow is Serialize-only (a command return, never read back), so assert on the JSON
        // shape rather than deserializing the concrete type.
        let rows: serde_json::Value = invoke(&window, "coverage_matrix", InvokeBody::default())
            .deserialize()
            .expect("coverage rows deserialize");
        assert_eq!(
            rows.as_array().map(|a| a.len()),
            Some(conductor_core::coverage_matrix().len()),
            "every classified capability surfaces through the IPC dispatch"
        );
    }

    #[test]
    fn unbacked_auto_command_returns_the_core_ledger() {
        let app = test_app();
        let window = main_window(&app);
        let ids: Vec<String> = invoke(&window, "unbacked_auto", InvokeBody::default())
            .deserialize()
            .expect("unbacked ids deserialize");
        // The webview reads the ledger through IPC rather than mirroring it in TS, so the count the
        // roll-up qualifier renders is conductor-core's, not a copy that can drift.
        assert_eq!(ids, conductor_core::UNBACKED_AUTO.to_vec());
    }

    #[test]
    fn run_report_command_returns_a_well_formed_record_list() {
        let app = test_app();
        let window = main_window(&app);
        // No run_id arg ⇒ latest; an absent/empty runs dir yields [] ("No run yet"), never an error.
        let _records: Vec<RunRecord> = invoke(
            &window,
            "run_report",
            InvokeBody::Json(serde_json::json!({})),
        )
        .deserialize()
        .expect("run_report returns a Vec<RunRecord>");
    }

    #[test]
    fn run_envelope_command_returns_null_when_no_run_has_an_envelope() {
        let app = test_app();
        let window = main_window(&app);
        // EnvelopeStanding is Serialize-only, so assert on the JSON shape. No run_id ⇒ latest; an
        // absent/empty runs dir yields null (the banner is simply absent), never an error — the same
        // honest degrade run_report gives the report table. The populated arm round-trips through
        // conductor_run::read_envelope, where the RunsDb source it single-sources lives.
        let standing: serde_json::Value = invoke(
            &window,
            "run_envelope",
            InvokeBody::Json(serde_json::json!({})),
        )
        .deserialize()
        .expect("run_envelope returns an Option<EnvelopeStanding>");
        assert!(
            standing.is_null(),
            "no run ⇒ no envelope standing: {standing}"
        );
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

    /// Dispatch a command that MUST fail, returning the rendered error. The sibling `invoke` panics
    /// on an error response, so it cannot express "the failure is the assertion".
    fn invoke_expecting_error(window: &MockWindow, cmd: &str, body: InvokeBody) -> String {
        let response = get_ipc_response(window, request(cmd, body));
        match response {
            Ok(_) => panic!("command `{cmd}` returned Ok where an error was required"),
            Err(err) => format!("{err:?}"),
        }
    }

    /// The committed read-only scenario catalog under `tests/fixtures/`. The parameterised helpers
    /// take `dir` directly and never read the process CWD, so a committed directory is subject
    /// enough — no temp dir, no `unsafe` env mutation.
    fn fixture_scenarios_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("scenarios")
    }

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
    }

    fn fixture_capabilities() -> CapabilityManifest {
        CapabilityManifest::load(&workspace_root().join(CapabilityManifest::default_path()))
            .expect("the committed SUT capability manifest loads from the workspace root")
    }

    const FIXTURE_SCENARIO_COUNT: usize = 2;

    /// A var name no process sets, so `resolve_handle` takes its `default` argument — which is what
    /// makes both arms reachable with no `unsafe { set_var }` (`.claude/rules/testing.md` 2026-08-10).
    const UNSET_HANDLE: &str = "CONDUCTOR_FIXTURE_UNSET_HANDLE_PROBE";

    #[test]
    fn the_committed_scenarios_fixture_stays_loadable() {
        // test-plan §7: pin the fixture's MEANING, not just that the files parse — a load-path change
        // that leaves the TOML readable but no longer loadable fails HERE rather than silently
        // inerting every killing test below. Doubles as the `list_scenarios_impl` kill.
        let summaries = list_scenarios_impl(&fixture_scenarios_dir(), &fixture_capabilities())
            .expect("the committed fixture scenarios load through the production catalog reader");
        assert_eq!(
            summaries.len(),
            FIXTURE_SCENARIO_COUNT,
            "the fixture must present every committed scenario, not an empty catalog"
        );
        let names: Vec<&str> = summaries.iter().map(|s| s.name.as_str()).collect();
        assert!(
            names.contains(&"fixture-alpha") && names.contains(&"fixture-beta"),
            "both fixture identities must survive the round-trip: {names:?}"
        );
    }

    #[test]
    fn resolve_handle_resolves_an_in_scope_default_under_the_current_dir() {
        let resolved = resolve_handle(UNSET_HANDLE, "src")
            .expect("`src` exists under the crate dir and stays inside its base");
        assert!(
            resolved.is_absolute(),
            "a resolved handle is absolute, got {resolved:?}"
        );
        assert!(
            resolved.ends_with("src"),
            "the default segment survives resolution: {resolved:?}"
        );
    }

    #[test]
    fn resolve_handle_rejects_a_traversal_default_naming_the_traversal() {
        // The ONLY assertion in this crate that reaches `resolve_under`'s `..` guard: here the base
        // is the process CWD, which exists, so canonicalization succeeds and the guard runs. The
        // run-data commands below fail earlier, on an unresolvable base — a different mechanism that
        // must not be described as proving rejection (security-plan §Input Validation).
        let err = resolve_handle(UNSET_HANDLE, "../escape")
            .expect_err("a `..` default must be REJECTED, never clamped to a safe path");
        assert!(
            err.contains("component"),
            "the error must name the traversal guard: {err}"
        );
        assert!(
            !err.contains("base directory is not resolvable"),
            "this must be the traversal rejection, not the absent-base error: {err}"
        );
    }

    #[test]
    fn the_artifact_handles_resolve_absolute_paths_under_the_current_dir() {
        let base = std::env::current_dir()
            .expect("a current dir")
            .canonicalize()
            .expect("the current dir canonicalizes");
        for (label, resolved) in [
            ("scenarios_dir", scenarios_dir()),
            ("runs_dir", runs_dir()),
            ("manifest_path", manifest_path()),
        ] {
            let path = resolved.unwrap_or_else(|e| panic!("{label} resolves its handle: {e}"));
            assert!(
                path.is_absolute(),
                "{label} must return an absolute path, got {path:?}"
            );
            assert!(
                path.starts_with(&base),
                "{label} must stay under its base: {path:?}"
            );
        }
    }

    #[test]
    fn resolve_selection_loads_the_whole_suite_for_the_sentinel() {
        let selected = resolve_selection(
            &fixture_scenarios_dir(),
            SUITE_SELECTION,
            &fixture_capabilities(),
        )
        .expect("the suite sentinel resolves against the fixture catalog");
        assert_eq!(
            selected.len(),
            FIXTURE_SCENARIO_COUNT,
            "the sentinel selects every scenario in the directory"
        );
    }

    #[test]
    fn resolve_selection_loads_exactly_one_for_a_named_scenario() {
        // Paired with the sentinel test above: the two branches must differ in CARDINALITY, or the
        // `==` -> `!=` mutant would route both to an indistinguishable result.
        let selected = resolve_selection(
            &fixture_scenarios_dir(),
            "fixture-alpha",
            &fixture_capabilities(),
        )
        .expect("a named scenario resolves against the fixture catalog");
        assert_eq!(
            selected.len(),
            1,
            "a named selection is exactly one scenario"
        );
        assert_eq!(
            selected[0].name, "fixture-alpha",
            "and it is the one that was named"
        );
    }

    #[test]
    fn load_all_reads_every_committed_fixture_scenario() {
        let scenarios = load_all(&fixture_scenarios_dir(), &fixture_capabilities())
            .expect("the fixture catalog loads in full");
        assert_eq!(
            scenarios.len(),
            FIXTURE_SCENARIO_COUNT,
            "load_all returns the whole directory, never an empty vec"
        );
    }

    #[test]
    fn run_report_errors_on_a_supplied_run_id_with_an_unresolvable_runs_dir() {
        // What this pins, precisely: `runs_dir()` resolves fine (its candidate need not exist), so
        // the inner guard runs against `<crate>/runs`, which does NOT exist — canonicalization of the
        // BASE fails before the `..` check. The mutant's `Ok(vec![])` cannot produce an error at all.
        let app = test_app();
        let window = main_window(&app);
        let err = invoke_expecting_error(
            &window,
            "run_report",
            InvokeBody::Json(serde_json::json!({ "runId": "../escape" })),
        );
        assert!(!err.is_empty(), "the failure surfaces a sanitized message");
    }

    #[test]
    fn run_envelope_errors_on_a_supplied_run_id_with_an_unresolvable_runs_dir() {
        // Same mechanism as run_report above; the mutant returns `Ok(None)`, which is a success.
        let app = test_app();
        let window = main_window(&app);
        let err = invoke_expecting_error(
            &window,
            "run_envelope",
            InvokeBody::Json(serde_json::json!({ "runId": "../escape" })),
        );
        assert!(!err.is_empty(), "the failure surfaces a sanitized message");
    }

    #[test]
    fn list_scenarios_errors_when_the_capability_manifest_is_unreachable() {
        // Under the crate-dir CWD `contracts/pulse-capabilities.toml` does not exist, so
        // `capabilities()` fails; `scenarios_dir()` itself resolves fine. The `Ok(vec![])` mutant
        // returns success and is killed here.
        let app = test_app();
        let window = main_window(&app);
        let err = invoke_expecting_error(&window, "list_scenarios", InvokeBody::default());
        assert!(!err.is_empty(), "the failure surfaces a sanitized message");
    }

    #[test]
    fn start_run_errors_before_spawning_a_run_thread() {
        // Called directly rather than through IPC: the two `Channel` arguments are constructed here,
        // sidestepping channel deserialization entirely. The error arises at `capabilities()`, an
        // EARLY RETURN, so no background thread is spawned and the deferred Channel frame-sequence
        // leg stays deferred (.claude/rules/testing.md 2026-06-26).
        let app = test_app();
        let err = start_run(
            "definitely-not-a-scenario".to_string(),
            Channel::new(|_| Ok(())),
            Channel::new(|_| Ok(())),
            app.state::<RunControl>(),
            app.state::<crate::pause::HoldGate>(),
        )
        .expect_err("start_run cannot reach a spawn without a resolvable capability manifest");
        assert!(!err.is_empty(), "the failure surfaces a sanitized message");
        assert!(
            !app.state::<RunControl>().abort.load(Ordering::SeqCst),
            "an early-return error leaves the abort flag untouched — nothing was started"
        );
    }
}
