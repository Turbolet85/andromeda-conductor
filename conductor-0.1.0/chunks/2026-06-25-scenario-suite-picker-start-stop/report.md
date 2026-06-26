# Report — 2026-06-25-scenario-suite-picker-start-stop

**Chunk:** Scenario/suite picker + start/stop — shadcn Command/Select picker + start_run/stop_run `#[tauri::command]`s + deny-by-default capability; backend run-lifecycle state, no live Channel (Epoch 9 ch3/10, conductor-tauri).
**Date:** 2026-06-26
**Commits:** (pending — flipped to complete in this wrap) feat(2026-06-25-scenario-suite-picker-start-stop)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-core/src/scenario_catalog.rs` (catalog helper + 3 unit tests)
  - NEW `crates/conductor-tauri/src/commands.rs` (3 IPC commands + `RunPhase` state)
  - NEW `crates/conductor-tauri/ui/src/components/ScenarioPicker.{tsx,css}` · `RunControls.{tsx,css}`
  - MOD `crates/conductor-core/src/lib.rs` (module decl + re-exports)
  - MOD `crates/conductor-cli/src/paths.rs` (`load_all_scenarios` delegates enumeration to core — DRY)
  - MOD `crates/conductor-tauri/src/main.rs` (`.manage()` + `.invoke_handler(generate_handler![…])`)
  - MOD `crates/conductor-tauri/Cargo.toml` (+`tracing` edge) · `capabilities/default.json` (description only)
  - MOD `crates/conductor-tauri/ui/src/App.tsx` (picker + controls + `invoke` wiring; DEV cycler retained)
  - MOD `ui/package.json` + `package-lock.json` (+`cmdk`) · `Cargo.lock` (+1 edge line, no new package)
  - REBUILT `ui/dist/*` (git-ignored)
- **Symbols / APIs:**
  - New public `conductor-core`: `ScenarioSummary { name, p_ids, slo_tier }` (derive `Serialize`), `list_scenarios(dir) -> Result<Vec<ScenarioSummary>>`, `scenario_files(dir) -> Result<Vec<PathBuf>>`, `validate_selection(&[ScenarioSummary], &str) -> bool`, `const SUITE_SELECTION: &str`.
  - New Tauri IPC commands (`conductor-tauri`): `list_scenarios()` · `start_run(selection: String)` · `stop_run()` (all `#[tauri::command]`, return sanitized `Result<_, String>`); managed state `Mutex<RunPhase>` (`RunPhase{Idle,Running,Aborted}` → wire labels `idle`/`live`/`aborted`).
  - Changed `conductor-cli`: `Paths::load_all_scenarios` now calls `conductor_core::scenario_files` (callers: `suite.rs:18` only).
  - No ports/sockets opened. No new env var (reuses `CONDUCTOR_SCENARIOS_DIR` via `resolve_under`).
- **Crates / modules:** new module `conductor_core::scenario_catalog`; new module `conductor_tauri::commands`. No crates added/removed; `conductor-tauri → conductor-core` stays the only Rust crate edge.
- **Dependencies:** Rust — `tracing` added as a direct dep of `conductor-tauri` (already a workspace dep ⇒ no new `Cargo.lock` package, +1 edge line). npm — `cmdk` added under `ui/` (+ its Radix transitive deps; `npm audit` 0).
- **Schema / config:** none — no `runs.db` schema, no run-report envelope change, no new contract/manifest, no scenario-model change. `capabilities/default.json` permissions unchanged (description-only edit).
- **Coverage of new surfaces:**
  - `list_scenarios` (IPC, read-only catalog) → validation `resolve_under` traversal-guard ✓ (no user input) · instrumentation span `tauri.command.list_scenarios` + info(count, latency_ms) ✓ · PII errors `sanitize_error` ✓ · tests unit (core, real catalog) ✓ · a11y n/a · tokens n/a
  - `start_run(selection)` (IPC) → validation `validate_selection` against catalog ✓ (unknown rejected; never to argv/shell) · instrumentation span `tauri.command.start_run` + info ✓ · PII `sanitize_error` ✓ · tests unit (`validate_selection` accept/reject) ✓ · a11y n/a · tokens n/a
  - `stop_run()` (IPC) → validation n/a · instrumentation span `tauri.command.stop_run` + info ✓ · PII `sanitize_error` ✓ · tests (state transition; covered by the pure `validate_selection` + compile) · a11y n/a · tokens n/a
  - `ScenarioPicker` (UI, shadcn Command/cmdk) → validation n/a · instrumentation n/a (browser console only — no OTLP) · PII n/a · tests build-gate (`tsc` strict + `vite build`); no E2E (Epoch-9 closing chunks) · a11y keyboard combobox via cmdk (roving focus + type-ahead + Enter), `aria-current` committed-choice + " · selected" text (not-color-alone), focus ring on input ✓ · tokens all `var(--…)`, no hex/px ✓
  - `RunControls` (UI, Start/Stop) → validation n/a · instrumentation n/a · PII n/a · tests build-gate · a11y semantic `<button>`, native `disabled` (not-color-alone), `:focus-visible` ring ✓ · tokens `var(--…)` ✓

## Deviations from intent
1. **Picker uses `cmdk` directly + token `.css`**, not vendored shadcn `ui/{command,select,button}.tsx` + `lib/utils.ts` + cva/clsx/tailwind-merge/lucide. The project styles via inline `var(--token)` + component `.css` (Titlebar/App pattern), NOT Tailwind utility classes — shadcn's utility-class components reference a `--popover`-style palette absent here, so they'd need full rewriting. cmdk IS shadcn Command's engine; used directly it matches the detected style, adds ONE runtime dep, and still satisfies a11y "use shadcn Command (cmdk), never hand-roll combobox" (cmdk supplies combobox/listbox roles + roving focus + type-ahead).
2. **No `list_suites` command** — research found no named suites ("suite" = whole catalog). The "Suite — all scenarios" picker entry uses the `SUITE_SELECTION` sentinel through `list_scenarios`/`start_run`.
3. **`conductor-tauri/Cargo.toml` gained `tracing`** (plan said "likely unchanged") — required for the `tauri.command.*` obs spans; already a workspace dep (no new Cargo.lock package). Used `info_span!().entered()` over `#[tracing::instrument]` to avoid attribute-macro stacking with `#[tauri::command]`.
4. **ACL permissions unchanged (description only)** — Tauri 2 app-defined commands are not ACL-gated (only core/plugin permissions are listed), so deny-by-default holds with no new entries.
5. **`aria-current` (not `data-*`) marks the committed choice** — type-safe on cmdk's component props under strict `tsc`, and SR-announced; paired with " · selected" text.
6. **`start_run`/`stop_run` return `&'static str`** (idle/live/aborted) — avoids adding `serde` to conductor-tauri; the frontend types it via `invoke<RunState>` (no `as` cast).

## Decisions & corrections
- **P4 depth decision = Control scaffold (A)** (user AskUserQuestion at phase): start_run validates + transitions, no pipeline execution; real execution + the pipeline-to-library extraction + the Channel deferred to ch4.
- **shadcn-over-Radix in this project = cmdk/Radix primitives + token `.css`/inline `var()`**, NOT shadcn-cli Tailwind-utility component files — the project has no `@/` path alias, no Tailwind-utility styling, tokens on `:root`. (reusable frontend pattern)
- **Tauri 2 app commands are not ACL-gated** — only `core:`/plugin permissions appear in `capabilities/*.json`; app `#[tauri::command]`s registered via `invoke_handler` are reachable without an ACL entry. (reusable Tauri/security fact)
- **`#[tracing::instrument]` does not stack cleanly under `#[tauri::command]`** — use a manual `info_span!(name).entered()` inside the command body instead. (reusable Tauri/obs gotcha)
- **Name-collision bug:** a command fn named identically to an imported fn (`list_scenarios`) made `generate_handler!` bind the wrong (CoreError-returning) signature → cryptic `blocking_kind`/`IpcResponse` errors. Fix: don't shadow — call the core fn fully-qualified. (Tauri gotcha)

## Outcome
- **Acceptance criteria met** for the Control-scaffold (A) scope: 3 IPC commands registered; managed run-lifecycle drives the titlebar RunState; shared `conductor_core::list_scenarios` helper (CLI reuses `scenario_files`, DRY); selection validated against catalog; deny-by-default ACL un-widened; tokens-by-name + not-color-alone + keyboard-operable picker; commands self-observed; `npm audit` 0 + `package-lock.json` committed; no engine/seam model change; headless `agent-run` untouched.
- **Gates green:** `cargo nextest -p conductor-core` 168 ✓ (+3 new) · `cargo nextest --workspace --profile ci` **398** ✓ · `cargo test --workspace --doc` ✓ · `cargo clippy --workspace --all-targets -- -D warnings` ✓ · ui `tsc --noEmit` + `vite build` ✓ · `npm audit` 0 ✓ · `cargo audit` ✓ + `cargo deny check` ✓ · `Cargo.lock` +1 line (no new package).
- **Smoke:** `bash scripts/agent-run.sh run` **exit 0** (release-gate path; Tauri entry-point wiring compile-verified within it).
- **Carried (operator visual check, ch1/ch2 posture):** GUI window boot + `invoke()` round-trip + command logging to `logs/conductor-tauri.jsonl` are compile-verified but not headless-smoke-tested (no tauri-driver until the closing Epoch-9 a11y chunks).
