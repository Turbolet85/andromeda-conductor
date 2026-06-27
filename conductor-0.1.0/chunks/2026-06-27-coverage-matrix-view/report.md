# Report — 2026-06-27-coverage-matrix-view

**Chunk:** Coverage-matrix view — dense single-row-per-P-ID list of all 60 P-IDs (P-001..P-060) with coverage-mode classification + verdict-first StatusLamp reuse (conductor-tauri).
**Date:** 2026-06-27
**Commits:** none yet — this wrap commits the chunk (prior `last_wrap` HEAD = `27c7d41` component-primitives-library).

## Changes (structured — detectors read this)
- **Files:** M `crates/conductor-core/src/coverage.rs` · M `crates/conductor-tauri/src/commands.rs` · M `crates/conductor-tauri/src/main.rs` · M `crates/conductor-tauri/ui/src/lamp.ts` · M `crates/conductor-tauri/ui/src/App.tsx` · M `crates/conductor-tauri/ui/src/Gallery.tsx` · N `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` · N `crates/conductor-tauri/ui/src/components/CoverageMatrix.css`.
- **Symbols / APIs:**
  - NEW read-only Tauri command `coverage_matrix() -> Result<Vec<CapabilityRow>, String>` (nullary; registered in `main.rs` `generate_handler!` beside `list_scenarios`/`start_run`/`stop_run`).
  - `conductor_core::CapabilityRow` gains `#[derive(Serialize)]` (public type now serde-serializable; `CoverageMode` was already `Serialize`).
  - NEW TS exports in `lamp.ts`: `lampForRecord(state, verdict): Lamp` + `Verdict` (`'Pass'|'Fail'|'CalibrationRegion'`) + `ReportState` (`'Pass'|'Fail'|'ManualCheck'|'KnownResidual'|'Blocked'`) unions.
  - NEW `CoverageMatrix` React component (`rows` + optional `lamps` props) + exported TS `CapabilityRow`/`CoverageMode` (mirror the command's wire shape).
  - No new ports / sockets / env vars.
- **Crates / modules:** none added/removed. Changed: `conductor-core` (coverage.rs) · `conductor-tauri` (commands.rs/main.rs + ui/).
- **Dependencies:** none added/bumped. `Cargo.lock` + `package-lock.json` un-drifted (Serialize already in `conductor-core`; no new npm dep — `StatusLamp`/`cmdk`/Radix already vendored).
- **Schema / config:** none. No `runs.db` schema change, no config keys, no capability-ACL entry (app commands aren't ACL-gated — security.md 2026-06-26), no violation schema.
- **Coverage of new surfaces:**
  - `coverage_matrix` (Tauri IPC command, read-only) → validation **n/a** (nullary, no input) · instrumentation **span✓** (`tauri.command.coverage_matrix` manual span + `info!(count, latency_ms)` boundary line; `sanitize_error` edge) · PII **n/a** (static `&'static str` classification) · tests **compile-gated** (workspace clippy/nextest via agent-run; GUI-integration `tauri::test` deferred to ch9 per route) · a11y **n/a** (backend) · tokens **n/a**.
  - `CoverageMatrix` (UI view) → validation **n/a** · instrumentation **n/a** (frontend; console-only obs, no new logging) · PII **n/a** · tests **build-gated** (`tsc`/`vite build`/`npm audit`) + DEV `#gallery` exercise; GUI-integration deferred ch9 · a11y **✓** (semantic `<table>` + `scope="col"` headers · `tabindex=0` scroll-region with `:focus-visible` ring · never-color-alone via reused `StatusLamp` glyph+label · empty/loading/error + "Not yet run" real prose) · tokens **✓** (all `var(--…)`; zero hardcoded hex/px).

## Deviations from intent
1. **`CoverageMatrix` presentational** (`rows` + optional `lamps` props); `App` owns the `invoke('coverage_matrix')` fetch + loading/empty/error prose. Justified: matches the existing `ScenarioPicker`/`list_scenarios` precedent (App fetches, component renders) and lets the DEV gallery render the view backend-free. All acceptance criteria still met.
2. **`<table>` semantics** (a11y extract allowed "`list`/`listitem` *or* table") with `scope="col"` headers + a focusable scroll region — free row/cell/header semantics for SC 1.3.1.
3. **No virtual-scroll** (layout extract suggested it for 60 rows) — all 60 rendered. Justified: zero perf concern at 60 static rows, avoids a dependency, all-rows-in-DOM trivially satisfies the a11y "rows reachable/announceable" concern.
4. **Command kept `Result<Vec<CapabilityRow>, String>`** (per plan) though infallible — a one-line doc comment records the WHY (uniform command surface + single-source). Durable-WHY note within comment policy.
5. **Live-view lamp column uniformly "Not yet run"** — the planned P4 scope (live per-P-ID runs.db join deferred to Epoch 10). The component is lamp-ready (`StatusLamp` renders when a `lamps` entry exists); the gallery exercises `lampForRecord` (Pass + Blocked) + the un-run state.

## Decisions & corrections
- **P4 data-source decision (AskUserQuestion):** chose **Option A — classification + defer live join** (the recommended option): a thin read-only `coverage_matrix` command exposing `conductor_core::coverage_matrix()`, reuse `StatusLamp` + ship `lampForRecord`, defer the live runs.db per-P-ID verdict join to Epoch 10. Recorded in `scope.md` (P4 resolution) + `plan.md`.
- **Single-source the classification** — the 60 rows come from `conductor-core` via the command, NOT re-authored in TS (the mirror-don't-re-spell rule the `lamp.ts` pattern is built on). The TS `lampForRecord` byte-mirrors `Lamp::for_record` arm-for-arm (the load-bearing arm order: `Blocked`/`KnownResidual` state-driven first).
- **Epoch-10 carry:** the live per-P-ID runs.db verdict join (a new `conductor-report` JSON1 query over `p_ids`, since `RunsDb` exposes only `open`/`insert`/`get`) lands in Epoch 10.

## Outcome
- **Acceptance criteria:** met (single-sourced classification via command · `lampForRecord` mirrors `Lamp::for_record` · dense token-bound list, no card grid · per-mode tally · `list`/table semantics + keyboard + prose empty states · `npm audit` 0 · manual command span · gates green · GUI-integration deferred ch9).
- **Gates (all green, 1st iteration, no fixes):** `npx tsc --noEmit` ✓ · `npm run build` ✓ (ui/dist regenerated) · `npm audit` 0 ✓ · `cargo nextest run -p conductor-core` 168/168 ✓ · `bash scripts/agent-run.sh run` **EXIT=0** (workspace nextest **402/402** + doctests + clippy `-D warnings`).
- **Smoke (boot-path changed — conductor-tauri command surface + ui):** ✓ `agent-run.sh run` — `conductor-tauri` compiles against the fresh `ui/dist`; the transient rust-analyzer `CapabilityRow`/`coverage_matrix` warnings were confirmed not-real (clippy `-D` clean).
