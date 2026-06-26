# Codebase Research — 2026-06-26-live-counter-channel-stream

## Scope
- **Depth:** deep (mature codebase; a cross-crate composition-root extraction + a new live IPC surface) · **Reads:** 16 files · **Code-graph queries:** 5 (`tree-query-2026-06-26-live-counter-channel-stream.json`)

## Files inspected
- `crates/conductor-cli/src/pipeline.rs` (full) — **the extraction target.** Its own doc-comment calls it "the composition root that drives one scenario to a `RunRecord`". Public: `Preflight` struct, `preflight()`, `readiness()`, `execute_scenario(pf, scenario, run_id, resolver: &CliResolver)`; private `unreachable_state`/`coarse_emit`/`manual_record`/`severity_rank`/`now_ms`. Imports `conductor_core` + `conductor_timeline` + `conductor_emit` + `conductor_verify` (and the Blocked record is `conductor_core::RunRecord::blocked`). **Blocked-path short-circuit at line 109:** `if !pf.ready { return Ok(RunRecord::blocked(...)) }` — returns BEFORE `coarse_emit`, so with no live Pulse there is **no emission at all** (the live counters have nothing to tick until Epoch 10).
- `crates/conductor-tauri/src/commands.rs` (full) — `RunPhase{Idle,Running,Aborted}` + `RunState = Mutex<RunPhase>` managed state; `start_run` is the **option-A scaffold** (validates `validate_selection`, flips state to Running, returns the label — NO pipeline). Sync `#[tauri::command] fn`s. `scenarios_dir()` resolves via `resolve_under` + `CONDUCTOR_SCENARIOS_DIR`.
- `crates/conductor-tauri/src/main.rs` (full) — `tauri::Builder` with `.manage(Mutex::new(RunPhase::default()))` + `generate_handler![list_scenarios, start_run, stop_run]`. Comment: "Tauri owns its own event loop — the core `current_thread` runtime is **not wired in here yet**." Self-obs via `ObsSink::File` → `logs/conductor-tauri.jsonl`.
- `crates/conductor-core/src/pause.rs` (full) — `trait PauseResolver { fn resolve(&self,&HoldPoint) -> impl Future<Output=Decision>; }` — **returns `impl Future`, so NOT dyn-compatible** (the doc-comment + cli/pause.rs both say so explicitly). `resolve_hold<R: PauseResolver>(&R, &HoldPoint)` is **generic, no `dyn`/boxing**. `HeadlessResolver{proceed/abort/new}` (the never-block default, core-shipped).
- `crates/conductor-cli/src/pause.rs` (full) — `enum CliResolver{Interactive(PromptResolver), Headless(HeadlessResolver)}` impls `PauseResolver`; `CliResolver::select(spinner, agent_mode)` is the isatty/agent gate. **CLI-local** (depends on `inquire`/`indicatif`).
- `crates/conductor-cli/src/commands/{run,suite,mod}.rs` (full) — the two callers of `pipeline::{preflight,execute_scenario}`; `mod.rs::persist(runs_dir, run_id, records)` = the 3-artifact writer (`JournalWriter` + `RunsDb` + `RunReport` from `conductor_report`); `exit_code` uses `Lamp::for_record` (CLI-only).
- `crates/conductor-tauri/ui/src/App.tsx` (full) — `DEV_CYCLE` backtick cycler gated `import.meta.env.DEV` (**to retire**); `start`/`stop` set `runState` from the `invoke` return; `<Titlebar runState count="00:00:00" />` (count hardcoded).
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` (full) — `RunState='idle'|'live'|'hold'|'aborted'`, props `{runState, count?}`, `aria-live="assertive"` label, `titlebar__count--${runState}` tint class. **No `blocked` runState; `count` is a plain string.**
- `Cargo.toml` (workspace) — 8 members, `resolver="3"`; `[workspace.dependencies]` is the single source of versions (incl. `tokio="1.48"`).
- `crates/conductor-tauri/Cargo.toml` — deps = `conductor-core`, `tauri`, `tracing` ONLY (no tokio, no seam crates).
- `crates/conductor-cli/Cargo.toml` — deps on core/timeline/emit/verify/report + clap/tokio(macros,rt)/anyhow/tracing/serde_json/owo-colors/indicatif/comfy-table/inquire.
- `crates/conductor-tauri/capabilities/default.json` — deny-by-default; description already states app commands + "later … live-counter Channel surfaces add only the **core/plugin** permissions they require."

## Graph impact (from the code-graph query)
- **`pipeline::{execute_scenario, preflight}`** — callers: `commands/run::run() @ run.rs:20`, `commands/suite::suite() @ suite.rs:28` (+ `execute_scenario`→`coarse_emit` internal @ pipeline.rs:125). Blast radius of the extraction = exactly these two CLI verbs; they re-point `use crate::pipeline` → `use conductor_run`.
- **`CliResolver`** — refs only in `cli/pause.rs` + `cli/pipeline.rs:27,106`. The pipeline.rs use is the ONE site the extraction must generalize (the cli/pause.rs uses stay).
- **`PauseResolver`** — `conductor-core/src/lib.rs:36` (re-export), `core/pause.rs:125/136/137` (trait + `resolve_hold`), impls in `cli/pause.rs`. The trait already lives in core; both bins can impl/pass it.
- **`crate_edges`** — `conductor-tauri → conductor-core` is the ONLY outbound edge today; `conductor-cli → {core,timeline,emit,verify,report}`. A new `conductor-run` crate adds edges `{cli,tauri} → conductor-run → {core,timeline,emit,verify,report}` with **zero cycles** (run sits ABOVE the seams, below the bins).

## Patterns detected
- **Composition-root altitude** (pipeline.rs:1–8): the pipeline is the only code that fans across timeline+emit+verify+report. `conductor-core` is the BASE every seam depends on (`crate_edges`: core is never a `from_crate`) → **the pipeline cannot live in core** (it would invert every seam edge → won't compile, arch §Compiler-enforced module seams). It belongs in a crate ABOVE the seams.
- **Generic resolver dispatch** (core/pause.rs:137, `resolve_hold<R: PauseResolver>`): the established way to be resolver-agnostic is a **generic bound, not a trait object** (the trait's `-> impl Future` is deliberately not object-safe). `execute_scenario` generalizes the same way: `execute_scenario<R: PauseResolver>(…, resolver: &R)`.
- **Sanitized command edge** (commands.rs:42–44, the obs session-rule): every `#[tauri::command]` opens `tracing::info_span!("tauri.command.<name>").entered()` and maps errors via `conductor_core::sanitize_error(&e)` → `Result<_, String>`.
- **Prop-contract for live data** (frontend.md session-rule, App.tsx:7–36): the DEV cycler was shipped precisely so "the later `Channel` plugs into the SAME `runState` prop" — retiring it is swapping the data source behind an existing prop, not a component rewrite.
- **3-artifact persistence** (mod.rs:23–32): `JournalWriter::create` + `RunsDb::open`/`insert` + `RunReport::write` — the parity unit both surfaces must produce identically.

## Conventions to follow
- **Workspace edges are the architecture** (`Cargo.toml`): a new crate is `members += "crates/conductor-run"` + `[workspace.dependencies] conductor-run = { path = … }`; deps reference `*.workspace = true`. Determinism runtime = `tokio` `current_thread` (`Cargo.toml:30`).
- **Manual span + `sanitize_error`** on the new `start_run` body (obs rule, 2026-06-26) — NOT `#[tracing::instrument]`.
- **Channel = no new capability** (capabilities/default.json + security.md 2026-06-26): app-command `Channel` args are not ACL-gated; the deny-by-default window allowlist stays unchanged. (The security extract's "declare the Channel in capabilities/*.json" is generic-rule over-reach for *app-command* Channels — flagged, not adopted.)
- **Frontend tokens by name** + `vite-env.d.ts` already present; retiring `import.meta.env.DEV` removes the only `import.meta.env` use but the reference file stays harmless.
- **Run/blocked exit + state semantics:** `blocked` is a reported envelope state, not a hard Fail (testing.md); the titlebar has no `blocked` runState.

## New files to create
- `crates/conductor-run/Cargo.toml` — new workspace member (deps: core, timeline, emit, verify, report, anyhow, tokio, tracing).
- `crates/conductor-run/src/lib.rs` — the extracted pipeline (`Preflight`, `preflight`, `readiness`, `execute_scenario<R: PauseResolver>`, `coarse_emit`, `manual_record`, …) + likely a shared `persist` (parity), made `pub`.
- `crates/conductor-tauri/src/run.rs` (or in commands.rs) — the core-owned `current_thread` runtime driver + `Channel<RunEvent>` streaming, behind `start_run`/`stop_run`.
- `crates/conductor-tauri/ui/src/` — a small Channel hook/wiring in `App.tsx` (no new component required; `Titlebar` already takes the props).

## Files to modify
- `Cargo.toml` (+member, +workspace.dep) · `crates/conductor-cli/Cargo.toml` (+conductor-run; prune now-unused direct seam deps if the extraction removes their last user) · `crates/conductor-tauri/Cargo.toml` (+conductor-run, +tokio).
- `crates/conductor-cli/src/pipeline.rs` → **deleted/relocated**; `crates/conductor-cli/src/main.rs` (drop `mod pipeline`) + `commands/{run,suite,preflight}.rs` (re-point imports to `conductor_run`); `commands/mod.rs::persist` (keep, or call the shared one).
- `crates/conductor-tauri/src/{main,commands}.rs` — wire the runtime + Channel; `start_run`/`stop_run` drive a real `RunRecord` + persist + stream.
- `crates/conductor-tauri/ui/src/App.tsx` — retire `DEV_CYCLE`; subscribe the Channel; drive `runState` + `count`.

## Open questions (→ P4 AskUserQuestion)
1. **Pipeline library home.** New `conductor-run` crate (clean crate-per-seam; **recommended** — `conductor-core` is architecturally impossible) vs. give `conductor-cli` a `[lib]` target the Tauri bin depends on (no new member, but Tauri then transitively inherits clap/inquire/indicatif). → user ratifies.
2. **`start_run` execution/streaming depth** (no live Pulse ⇒ every run resolves Blocked, emitting nothing). (A) background OS thread + core-owned `current_thread` runtime + `Channel` streams lifecycle/target-status live + `stop_run` aborts (forward-compatible for Epoch-10 live counters; more plumbing) vs. (B) synchronous core-owned `current_thread` `block_on` drive + `Channel` emits the lifecycle events it passes + persist (thinner; true-concurrent streaming + abort deferred to Epoch-10). Both ship: real execution, real Blocked `RunRecord` persistence, the Channel, and the DEV-cycler retire.
3. (minor, P4-decidable) Terminal-Blocked → titlebar: return to `idle` after a Blocked run vs. add a `blocked` runState. Lean: return to `idle` (no new runState — `blocked` lamp is the later report-view chunk's surface), with the count showing the run's emitted total (0 on the Blocked path).
