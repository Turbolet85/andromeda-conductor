# Codebase Research — 2026-06-27-operator-pause-go-no-go-dialog

## Scope
- **Depth:** deep · **Reads:** 9 · **Globs/Greps:** 3 · **Code-graph queries:** 1

## Files inspected
- `crates/conductor-core/src/pause.rs` (full) — the shipped hold model: `HoldPoint { scenario, p_id, step, prompt, allow_no_go }` (Validate + Serialize/Deserialize), `Decision { Go, NoGo }` (Serialize/Deserialize PascalCase + `.label()`), `HoldResolution` (Serialize), `PauseResolver::resolve(&self, &HoldPoint) -> impl Future<Output=Decision>` (NOT object-safe → generic/enum dispatch, never `dyn`), `HeadlessResolver::{proceed,abort,new}` (never blocks), `resolve_hold<R>(resolver, hold)` (infallible; redacts `prompt` into the resolution).
- `crates/conductor-run/src/lib.rs` (full) — **`execute_scenario<R: PauseResolver>` ALREADY calls `resolve_hold`** (lines 145-162): when `scenario.expected.is_empty()` (the operator-checklist DriveObserve scenarios) on the `pf.ready` path it builds a `HoldPoint` and resolves it, returning a `ManualCheck` record. **BUT this is only reached on a live-Pulse (ready) gate** — the Blocked path short-circuits at line 118 first. **`drive_run` HARDCODES `let resolver = HeadlessResolver::proceed();`** (line 303) — so the GUI cannot inject a resolver today. `RunEvent { stage: RunStage, count: u64 }` is `#[derive(Copy, Serialize)]`; `RunStage` is `{Progress, Blocked, Done, Aborted}` (serde lowercase). `drive_run<E,A>` is generic over the emit fn + abort fn only.
- `crates/conductor-tauri/src/commands.rs` (full) — `start_run` validates the selection, mints a run_id, and `std::thread::spawn`s `run_thread` (a core-owned `current_thread` runtime → `drive_run`), streaming `RunEvent` over `on_event: Channel<RunEvent>`; `stop_run` sets `RunControl.abort: Arc<AtomicBool>`. Every command opens a manual `tauri.command.*` span + returns `Result<_, String>` via `sanitize_error`. Dir handles resolved through `resolve_under` (traversal guard).
- `crates/conductor-tauri/src/main.rs` — `tauri::Builder` `.manage(RunControl::default())` + `.invoke_handler(generate_handler![list_scenarios, coverage_matrix, run_report, start_run, stop_run])`.
- `crates/conductor-tauri/ui/src/App.tsx` (full) — the run loop: ONE `new Channel<RunEvent>()` in `start()`, `channel.onmessage` sets `count` + `runState` (via `STATE_FOR_STAGE`) and re-reads the report on a terminal stage. `RunState` imported from `Titlebar`.
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` — **`RunState = 'idle' | 'live' | 'hold' | 'aborted'` ALREADY includes `'hold'`**; `STATE_LABEL.hold = 'Conductor · HOLD — operator pause'`; the count carries a `titlebar__count--hold` class (the freeze/tint signature shipped by paused-count-hold-point-signature). The frontend already RENDERS hold — App just needs to drive `runState='hold'`.
- `crates/conductor-tauri/ui/src/components/OperatorPauseDialog.{tsx,css}` (full) — the shipped scaffold: Radix AlertDialog, controlled `open`/`onOpenChange` + `title`/`body`/`onProceed`/`onAbort`/`allowNoGo`/`proceedLabel`/`abortLabel`. CSS is fully tokened (`--color-raised-3` surface, `--border-subtle`, `--radius-lg`, `--motion-micro`/`--ease-quiet` fades, `.dialog__btn--proceed` = `--count-nominal`, `:focus-visible` = `--color-focus`). Reduced-motion drop is global in tokens.css.
- `crates/conductor-cli/src/pause.rs` (full) — the PARALLEL resolver to mirror: `CliResolver` is an **enum** `{Interactive(PromptResolver), Headless(HeadlessResolver)}` dispatched by `match` in its `PauseResolver` impl (because the trait isn't object-safe); `CliResolver::select(spinner, agent_mode)` is the isatty+agent gate; a cancelled/`Err` prompt collapses to `Decision::NoGo` (the abort-safe default). Tested at the gate + dispatch (injected `HeadlessResolver`), NEVER a real prompt.
- `crates/conductor-tauri/capabilities/default.json` — 3 window perms only (`start-dragging`/`minimize`/`close`); the description states app commands need NO permission and "later operator-pause … surfaces add only the core/plugin permissions they require."

## Graph impact (from `tree-query-2026-06-27-operator-pause-go-no-go-dialog.json`)
- **`resolve_hold`** — referenced by `execute_scenario` (conductor-run) + the `conductor-core`/`conductor-cli` pause tests. No production caller outside `execute_scenario`.
- **`drive_run`** — the query surfaced no resolved cross-crate caller edge; from direct reads its ONLY non-test caller is the Tauri `run_thread` (`commands.rs:201`), and its 2 unit tests (`lib.rs:366,394`). **The CLI run/suite verbs call `execute_scenario` directly (already generic over the resolver), NOT `drive_run`.** ⇒ Generalizing `drive_run`'s signature to accept a resolver has blast radius = the Tauri caller + 2 unit tests; the CLI release-gate path is untouched.

## Patterns detected
- **Resolver-as-enum/generic, never `dyn`** (`conductor-cli/src/pause.rs:62`, `conductor-run/src/lib.rs:112`): `PauseResolver::resolve -> impl Future` is not object-safe; the CLI uses an enum + match. The Tauri resolver is a single kind → a struct implementing `PauseResolver` suffices.
- **Channel-as-command-arg** (`commands.rs:150` `on_event: Channel<RunEvent>`): backend→frontend streaming WITHOUT any ACL permission (Channels are command args, not gated). Reusing this for the hold signal avoids the `core:event:allow-listen` permission a Tauri `emit`/`listen` would require.
- **Background-thread run + manual `tauri.command.*` span + `sanitize_error` edge** (`commands.rs`): the new resolve command follows this exact shape (obs 2026-06-26 rule).
- **`'hold'` run-state already wired in the titlebar** (`Titlebar.tsx:4`): App flips `runState` to `'hold'` on hold-open and back to `'live'` on resolve — the freeze/tint is already rendered.
- **DEV/gallery demonstration for components without a live data source** (frontend rule 2026-06-24 + `Gallery.tsx`): the wired dialog is demonstrable in the gallery driven by local state (every Epoch-9 view is build-verified, not live-smoke-tested — no display, no live Pulse).

## Conventions to follow
- `PauseResolver::resolve` is `impl Future` → generic `<R: PauseResolver>` or a concrete struct, NEVER `Box<dyn>` (pause.rs module doc).
- A cancelled/dropped operator decision collapses to `Decision::NoGo` (the abort-safe default — `conductor-cli/src/pause.rs:56`).
- `HeadlessResolver::proceed()` stays the agent/headless default — the source-of-truth path never blocks (CLAUDE.md headless invariant; verification-harness.md).
- App `#[tauri::command]`s need NO capability entry; `default.json` stays the 3 window perms (security 2026-06-26 rule + the file's own description).
- Errors to the webview go through `conductor_core::sanitize_error`; the boundary log line carries `run_id` + allowlist-safe fields only (obs 2026-06-26 rule).

## New files to create
- `crates/conductor-tauri/src/pause.rs` — the `TauriResolver` (implements `PauseResolver`; bridges the background run thread ↔ webview via a `tokio::sync::oneshot` held in managed state + an `AppHandle`/`Channel` to surface the `HoldPoint`) + the `resolve_operator_hold(decision)` `#[tauri::command]` + its managed `HoldGate` state. (Or fold the command into `commands.rs`; a sibling module mirrors `conductor-cli/src/pause.rs`.)

## Files to modify
- `crates/conductor-run/src/lib.rs` — **[Option A1 only]** generalize `drive_run` to `drive_run<R: PauseResolver, E, A>(…, resolver: &R, …)` (stop hardcoding `HeadlessResolver::proceed()`); update its 2 unit tests to pass `&HeadlessResolver::proceed()`. CLI unaffected.
- `crates/conductor-tauri/src/commands.rs` — `run_thread` constructs + passes the `TauriResolver`; `start_run` gains the hold signal (a 2nd `Channel<HoldPrompt>`) + an `AppHandle` (for managed-state access); register `HoldGate` (oneshot slot) in state.
- `crates/conductor-tauri/src/main.rs` — `.manage(HoldGate::default())` + add `resolve_operator_hold` to `generate_handler!`.
- `crates/conductor-tauri/ui/src/App.tsx` — create a 2nd `Channel` for hold prompts; render `OperatorPauseDialog` open on a hold; `invoke('resolve_operator_hold', { decision })` on Proceed/Abort; flip `runState` `'hold'`↔`'live'`.
- `crates/conductor-tauri/ui/src/Gallery.tsx` — demonstrate the fully-wired dialog driven by local state (DEV).
- `crates/conductor-tauri/capabilities/default.json` — **UNCHANGED** (confirm; app command + Channel need no permission).

## Open questions
1. **Scope boundary (→ P4 AskUserQuestion, recommend A1):** does ch8 generalize `drive_run` to thread the `TauriResolver` through now (full end-to-end bridge — fires on the live-Pulse operator-checklist holds in Epoch-10; touches conductor-run + 2 tests, CLI untouched), OR stay purely conductor-tauri (TauriResolver + resolve command + dialog + gallery demonstration) and defer the one-line `drive_run` resolver-injection to Epoch-10?
2. **Hold-signal mechanism (recommend; implement finalizes):** a dedicated `Channel<HoldPrompt>` arg to `start_run` (ACL-free, keeps the `RunEvent` live-counter contract stable) vs. widening `RunEvent` into an enum vs. Tauri `emit`/`listen` (needs `core:event:allow-listen` — avoided). Recommend the 2nd Channel.
