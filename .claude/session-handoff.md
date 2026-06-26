# Session Handoff

**Last Updated:** 2026-06-26T09:23:03Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-25-scenario-suite-picker-start-stop — feat: scenario/suite picker (cmdk shadcn Command) + list_scenarios/start_run/stop_run #[tauri::command]s + run-lifecycle state driving the titlebar; option-A control scaffold (no pipeline exec — deferred ch4); shared conductor_core::list_scenarios (CLI reuses scenario_files, DRY); zero engine/seam model change (conductor-tauri)

## Position
- Done: **2026-06-25-scenario-suite-picker-start-stop** — **Epoch 9 (Desktop control panel) ch3/10.** The first GUI **control surface**: a searchable scenario/suite picker (shadcn Command via `cmdk`, styled by token `.css`) over a real read-only `conductor_core::list_scenarios` helper + `start_run`/`stop_run`/`list_scenarios` `#[tauri::command]`s driving a backend `RunPhase` (Idle/Running/Aborted) into the titlebar `RunState`. **P4 decision = Control scaffold (A):** `start_run` validates the selection (`validate_selection` against the catalog) + transitions state but does NOT execute the pipeline — real execution + the pipeline-to-library extraction defer to ch4. CLI's `load_all_scenarios` now reuses the shared `scenario_files` enumerator (DRY). Deny-by-default ACL un-widened (Tauri 2 app commands aren't ACL-gated). **Zero engine/seam MODEL change; headless `agent-run` untouched.**
- Next: **Epoch 9 ch4 — Live-counter Channel stream** (now CARRYs the run-execution + pipeline-extraction deferral + the DEV-cycler retire). → `/andromeda-phase` to promote + plan.

## Work done
4 NEW (`conductor-core/src/scenario_catalog.rs` [+3 tests]; `conductor-tauri/src/commands.rs`; `ui/src/components/{ScenarioPicker,RunControls}.{tsx,css}`) + 7 MOD (`core/src/lib.rs` exports; `cli/src/paths.rs` DRY→`scenario_files`; `tauri/src/main.rs` invoke_handler+manage; `tauri/Cargo.toml` +`tracing` edge; `capabilities/default.json` desc; `ui/src/App.tsx`; `ui/package{,-lock}.json` +`cmdk`). Gates: core nextest **168** (+3) · workspace nextest **398** · doctest · clippy `-D` · ui `tsc`+`vite build` · `npm audit` 0 · `cargo audit`+`cargo deny` · `agent-run.sh run` exit 0. Code-graph **1211n/5273e**. `Cargo.lock` +1 line (tracing edge, no new package). New deps: `cmdk` (npm). User decision: **P4** depth = Control scaffold (A) (AskUserQuestion).

## Drift resolved
**drift = 0.** 7 detectors, **5 firings — ALL warning-severity over-reach, 0 spec-body amendments**; **0 escalate-severity fired** (security/obs/tests/design/a11y all clean — the new IPC input is validated via `validate_selection`+`resolve_under`, deps audit-green, commands instrumented `tauri.command.*` + `sanitize_error`'d, tokens-by-name, picker a11y ✓). **4 arch + 1 layouts** proposals dismissed (1 escalation, user-confirmed): arch already registers the Tauri command surface at category grain (§Occupied Resources) + the component-lib decision lives in design-system §Component Patterns §5; the picker surface is already in layout §Wireframe. **+1 playbook rule** appended (Tauri-command-name/internal-module/frontend-component-package over-reach — pre-empts re-fire on ch4/ch5/ch8). No cascade (no spec body changed).

## Notes
- **Curation:** Tier 2 ×3 — `frontend.md` (shadcn = cmdk/Radix primitives used DIRECTLY + token `.css`, not shadcn-cli utility files; no `@/` alias, no Lucide) · `security.md` (Tauri 2 app `#[tauri::command]`s are NOT ACL-gated — only core/plugin perms in `capabilities/*.json`; the real boundary is input validation) · `observability.md` (instrument a `#[tauri::command]` with manual `info_span!().entered()`, NOT `#[tracing::instrument]` — doesn't stack under the command macro). Filtered: 0 dup / 0 task-specific / 0 conflict.
- **Deferred learning (max-3 cap):** name-collision gotcha — a Tauri command fn named identically to an imported fn makes `generate_handler!` bind the wrong signature → cryptic `IpcResponse`/`blocking_kind` errors; call the core fn fully-qualified. (Apply via `/andromeda-wrap-session --review` if wanted.)
- **Last failed command:** none.
- **Operator visual check (NEW — carried):** the GUI window boot + `invoke()` round-trip + command logging to `logs/conductor-tauri.jsonl` are build/compile-verified but NOT headless-smoke-tested (launching the Tauri window needs a display + would block; no tauri-driver harness until the closing Epoch-9 a11y chunks; same posture as ch1/ch2). Launch the dev app → picker lists 31 scenarios, type-ahead filters by name/P-ID, ↑/↓/Enter selects, Start/Stop drive the titlebar run-state, commands log to `logs/conductor-tauri.jsonl`.
- **Follow-up (NEW this chunk):**
  - **ch4 CARRY** — the option-A deferral: extract `conductor-cli`'s bin-local `pipeline.rs` → a library (conductor-core / new conductor-run) + generalize `&CliResolver` → trait object + core-owned `current_thread` runtime under Tauri, so `start_run` drives a real `RunRecord` (Blocked w/o Pulse) persisting runs.db/journal (CLI↔Tauri parity = test-plan Path 7); reconcile the DEV-cycler retire with ch3's start/stop→`setRunState` wiring.
- **Follow-up (carried — unchanged):**
  - `indicatif` 0.17→0.18 may drop `number_prefix`, retiring the RUSTSEC-2025-0119 deny.toml ignore.
  - DRY: expose `conductor_verify::readiness(...)` to retire the `UNREACHABLE_PRECONDITION` dup in `pipeline.rs`.
  - `scenario.run` root obs span (CLI driver) — deferred (Epoch-10).
  - Live interactive operator-pause leg + two faithful content bridges → Epoch-10 (live runs stay Blocked until then).
  - test-plan §3 ↔ obs-plan §3 dual-RECORD-SHAPE reconcile (test-plan §3 OWNER).
  - `opentelemetry-proto default-features=false` trim (dormant transitive OTel SDK).
