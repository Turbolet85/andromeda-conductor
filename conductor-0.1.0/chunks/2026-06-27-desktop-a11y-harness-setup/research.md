# Codebase Research — 2026-06-27-desktop-a11y-harness-setup

## Scope
- **Depth:** deep (mature codebase, additive test/harness chunk) · **Reads:** 9 · **Globs/Greps:** 6 · **code-graph queries:** 2

## Files inspected
- `crates/conductor-tauri/Cargo.toml` (full) — deps: conductor-core, conductor-run, serde, tauri, tokio(rt,sync), tracing. **No `[dev-dependencies]`, no `tauri` `test` feature** → both are NEW for the mock-runtime tests.
- `crates/conductor-tauri/src/commands.rs` (full) — the command surface: `list_scenarios`/`coverage_matrix`/`run_report` (sync, read-only, `tracing.info_span!("tauri.command.*")`); `start_run` (spawns a **background `std::thread`** → core-owned `current_thread` runtime → `conductor_run::preflight` + `drive_run`, streams `RunEvent` over `Channel`, raises holds over `Channel<HoldPrompt>`); `stop_run` (sets `RunControl.abort: AtomicBool`). Input is `validate_selection`-gated; dirs via `resolve_under`; errors via `sanitize_error`.
- `crates/conductor-tauri/src/pause.rs` (full) — `HoldPrompt` (Serialize projection of `HoldPoint`), `HoldGate{arm,deliver}` (private fns; oneshot slot), `TauriResolver: PauseResolver`, `resolve_operator_hold` command. **Already has inline `#[cfg(test)] mod tests`** covering `HoldGate` arm/deliver round-trip, no-op, abort-safe-NoGo (the bridge CORE — per the 2026-06-27 deferral note). The deferred leg = the `resolve_operator_hold` **command** (IPC dispatch + `State` extraction) + the `Channel<HoldPrompt>` frame.
- `crates/conductor-tauri/src/main.rs` (full) — `tauri::Builder` manages `RunControl::default()` + `HoldGate::default()`, registers all 6 commands via `generate_handler!`. The mock-runtime test app mirrors this (`mock_builder().manage(...).invoke_handler(generate_handler![...])`).
- `crates/conductor-run/src/lib.rs` (full) — `drive_run`/`execute_scenario`/`persist`/`preflight`/`readiness` are `pub`; `Preflight{client,ready}` fields are **private** (a blocked `Preflight` is constructible only inside conductor-run — its own tests do so; Tauri/CLI reach Blocked via `preflight()` with the read-back path unreachable). `drive_run` is generic + DI-free, **already unit-tested** for Progress→Blocked + abort + persist. `persist` writes journal+runs.db+Markdown "so the CLI and the GUI write an identical envelope for the same scenario+seed (test-plan Path 7)".
- `crates/conductor-cli/tests/cli_smoke.rs` (full) — the parity reference: forces Blocked hermetically via `ANDROMEDA_PULSE_DATA_DIR=pulse;injection` (injection metachar rejected pre-spawn), asserts the persisted envelope `state=="Blocked"`, `verdict` null, no host-path leak. `blocked_state()` reads the journal's first JSONL line. **This is the CLI half of the Path-7 parity the Tauri half must match.**
- `Cargo.toml` (workspace) — `tauri="2.10.3"`, `tauri-build="2"`; dev-frameworks already declared as workspace deps: `rstest`/`proptest`/`insta`/`assert_cmd`/`assert_fs`/`predicates`; `serde_json="1.0"`. The mock-runtime tests reuse `assert_fs` (TempDir) + `serde_json`.
- `crates/conductor-tauri/ui/package.json` (full) — deps: radix alert-dialog, cmdk, tauri-apps/api, react 19, fontsource; devDeps: tailwind/vite, vite 8, typescript 5.6. **No webdriverio/@crabnebula/tauri-driver/axe-core/@axe-core/webdriverio/lighthouse/colorjs.io** → all NEW devDeps. scripts: dev/build/typecheck/preview — **no test/a11y script**.
- `crates/conductor-tauri/ui/` listing + `tauri.conf.json` — `frontendDist:"ui/dist"`, `beforeBuildCommand:"npm run build"`. **No wdio/playwright/a11y/vitest config exists** (all new).

## Graph impact (code-graph query → `tree-query-2026-06-27-desktop-a11y-harness-setup.json`)
- **crate edges:** `conductor-tauri → {conductor-core, conductor-run}` only; **zero inbound** edges → `conductor-tauri` is a **leaf**. Adding `[dev-dependencies]` + inline tests + npm devDeps has **zero cross-crate blast radius** (additive, leaf).
- **`HoldGate::{arm,deliver}` @ `pause.rs`**, **`TauriResolver` + `resolve_operator_hold` @ `pause.rs`**, **`start_run`/`stop_run`/`run_report` @ `commands.rs`** — all confirmed present; this chunk EXERCISES them (no signature change). `drive_run`/`execute_scenario`/`persist` @ `conductor-run/src/lib.rs` unchanged (consumed by the Path-7 parity assertion).

## Patterns detected
- **Inline `#[cfg(test)]` for bin-crate command/bridge tests** (`pause.rs:106`): conductor-tauri is a BIN (no lib.rs), so commands are crate-private — tested inline, NOT via a `tests/` integration dir. The new mock-runtime tests follow this (inline in `commands.rs`/`pause.rs`).
- **Hermetic Blocked via injection-metachar `ANDROMEDA_PULSE_DATA_DIR`** (`cli_smoke.rs:29`): the host-independent way to force the no-Pulse Blocked spine without a sidecar. The Tauri Path-7 leg uses the same lever.
- **Manual `tauri.command.*` span + `sanitize_error` edge** (`commands.rs:84`, obs §4 + 2026-06-26 obs note): every command logs an allowlisted `count`/`latency_ms` info line; errors go through `sanitize_error` (Display-not-Debug + host-path scrub). Tests must not assert on raw errors.
- **Persisted-envelope parity is the cross-surface contract** (`lib.rs:247` `persist` doc): Path 7 is asserted on the runs.db/journal envelope (same seed ⇒ same verdict/state), NOT on trace correlation (obs §3: no traceparent; `run_id` is the only key).
- **`ensure_frontend` = `npm ci && npm run build`** (`agent-run.sh:28`) is the ONLY frontend step in the `run` gate; `tauri::generate_context!` needs `ui/dist` to exist before any `conductor-tauri` cargo compile (frontend.md 2026-06-24).

## Conventions to follow
- **tauri::test mock-runtime** (tests extract §5; tauri 2.10): `mock_builder()` + `mock_context(noop_assets())` + `.manage(...)` + `generate_handler![...]` + `get_ipc_response(&webview, req)`; requires `tauri = { workspace=true, features=["test"] }` in `[dev-dependencies]`. Fresh `assert_fs::TempDir` per test (nextest per-process isolation — testing.md).
- **a11y harness tooling** (a11y-plan §3.5 + a11y.md): `axe-core@4.12.0` via `@axe-core/webdriverio`, `lighthouse@13.0.3`, `colorjs.io@0.6.1`, ONE webview stack = `@crabnebula/tauri-driver@2.0.9` + `webdriverio` on **Linux+xvfb** (never a 2nd puppeteer/CDP stack); axe `withTags(['wcag2a','wcag2aa','wcag21aa'])` (no wcag22aa).
- **Selector discipline** (testing.md + tests §6): role / text / aria-live / brand anchors (`[PASS]`/`[HOLD]`/`role="alertdialog"`/`role=status`) — never xpath or hashed-CSS; never `sleep(N)` — wait for an explicit signal (terminal `RunEvent`, JSONL line, file existence).
- **Token-bound contrast** (design §Color Palette + a11y §6): colorjs.io reads compiled `:root` from `tokens.css`; pairs `--text-primary`/`--color-base` (4.5:1), `--count-nominal`/`--color-base` + `--color-focus`/`--color-base` (3:1); Blocked never `--status-fail`.
- **No OTel SDK in the harness** (obs §3): tauri-driver/axe/Lighthouse/mock-runtime never init an OTel SDK; `tracing`-only.

## New files to create
- `crates/conductor-tauri/ui/wdio.conf.ts` — WebdriverIO + `@crabnebula/tauri-driver` config (Linux+xvfb capability; axe service); display-gated.
- `crates/conductor-tauri/ui/test/a11y/*.e2e.ts` (or similar) — axe-scan + colorjs.io token-pair-contrast + WebdriverIO keyboard/focus-trap specs for the four accessible paths (RunReport / OperatorChecklistView / OperatorPauseDialog / picker), authored against role/text selectors; display-gated.
- `crates/conductor-tauri/tests/`-equivalent **inline** `#[cfg(test)]` blocks in `commands.rs` (+ extend `pause.rs`) — the deterministic mock-runtime command tests + the Path-7 parity leg. (Inline, per the bin-crate pattern.)

## Files to modify
- `crates/conductor-tauri/Cargo.toml` — add `[dev-dependencies]` (`tauri = { workspace=true, features=["test"] }`, `assert_fs`, `serde_json`, `tokio` test feature as needed) — `cargo audit`/`deny` + `Cargo.lock` un-drifted.
- `crates/conductor-tauri/ui/package.json` (+ `package-lock.json`) — add the a11y devDeps + `a11y`/`test:e2e` npm script(s); `npm audit` 0.
- `crates/conductor-tauri/src/commands.rs` + `src/pause.rs` — add the inline mock-runtime tests.
- (Maybe) `scripts/agent-run.{sh,ps1}` / `.github/workflows/ci.yml` — IF the harness joins a gate; default is NOT the always-on Windows/CI gate (display-gated → Epoch-10 "A11y CI gate"); `npm audit` already covers the new devDeps via the existing Frontend gate job.

## Open questions
1. **Boundary of "landing the deferred tests" under the zero-retry bar** — how much of `start_run`'s background-thread Channel-frame stream to assert now vs. defer (the 2026-06-26/06-27 deferral notes warn the frame-SEQUENCE flakes). → P4 AskUserQuestion.
2. **In-process runs-dir isolation for `run_report` mock-runtime test** — `run_report` reads `CONDUCTOR_RUNS_DIR`/CWD; `std::env::set_var` is `unsafe` in edition 2024 (security.md review flag). Prefer a no-IO command (`coverage_matrix` → 60 rows) for the pure dispatch smoke + assert the dir-dependent envelope at the `persist`/parity layer. → resolve in /implement.
3. **CI xvfb/tauri-driver job placement** — author the harness now; the always-on a11y CI gate is the Epoch-10 route chunk "A11y CI gate + violation JSON". → stated in plan, not asked.
