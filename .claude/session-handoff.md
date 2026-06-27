# Session Handoff

**Last Updated:** 2026-06-27T00:57:28Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-27-coverage-matrix-view — feat: Coverage-matrix view — dense single-row-per-P-ID list of all 60 P-IDs (classification + verdict-first StatusLamp reuse) via a thin read-only `coverage_matrix` command (Serialize derive on CapabilityRow) + `lampForRecord` projection; live per-P-ID join deferred to Epoch 10; UI + one read-only command, zero engine/seam model change (conductor-tauri)

## Position
- Done: **2026-06-27-coverage-matrix-view** — **Epoch 9 (Desktop control panel) ch6/10.** The dense 60-P-ID coverage surface (the signature mission-control instrument): a thin read-only `coverage_matrix` `#[tauri::command]` single-sources `conductor_core::coverage_matrix()` (added `#[derive(Serialize)]` to `CapabilityRow`); `CoverageMatrix.tsx` renders P-ID · title · category · mode in a dense token-bound `<table>` (no card grid), **reusing `StatusLamp`** + shipping the verdict-first **`lampForRecord`** projection (mirrors `Lamp::for_record` arm-for-arm) in `ui/src/lamp.ts`. Per the **P4 data-source decision (classification + defer live join)** the per-P-ID lamp column is **"Not yet run"** prose (the component is lamp-ready; the DEV gallery exercises `lampForRecord`); the **live runs.db per-P-ID verdict join defers to Epoch 10**. UI + one read-only command, zero engine/seam model change.
- Next: **Epoch 9 ch7 — Run-report + operator-checklist views** (verdict lines + ManualCheck induced-state checklist; **reuse `StatusLamp` + `OperatorChecklist` + the new `lampForRecord` + the read-only-command data-sourcing pattern**). → `/andromeda-phase` to promote + plan.

## Work done
6 MOD (`conductor-core/src/coverage.rs` +Serialize · `conductor-tauri/src/{commands,main}.rs` +`coverage_matrix` command · `ui/src/{lamp.ts,App.tsx,Gallery.tsx}`) + 2 NEW (`ui/src/components/CoverageMatrix.{tsx,css}`). Gates green **1st iteration, no fixes**: `tsc --noEmit` · `npm run build` · `npm audit` 0 · `cargo nextest -p conductor-core` 168/168 · `agent-run.sh run` exit 0 (workspace nextest **402** unchanged + doctest + clippy `-D`; conductor-tauri compiles against fresh ui/dist). Code-graph **1209n/5415e**. No new dep; `Cargo.lock` + `package-lock.json` un-drifted.

## Drift resolved
**drift = 0.** 7 detectors, **2 proposals** (5 returned `[]`), both DISMISSED → **0 spec amendments**. **arch D-arch-resources** (register `coverage_matrix` command name) → auto-dismiss per the existing 2026-06-26 Tauri-command-name over-reach rule (command surface is category-grain; handler names are arch's omitted realization). **obs D-obs-instrumentation** (run_id on the read-only command's `info!` line) → dismissed WITH user + **new playbook rule seeded**: a read-only Tauri query command following the shipped `list_scenarios` span+count/latency_ms pattern (no run_id, runs outside any scenario) is not drift — the proposed synthetic run_id would diverge from `list_scenarios`. Clean: security/design/layout/test/a11y (tokens✓, never-color-alone✓, surface already in layout §Primary-content-block-1, GUI-integration test deferral authorized, a11y baked in). **0 escalations open.** Route: **1 CARRY** to ch7 (reuse `lampForRecord` + the data-sourcing pattern).

## Notes
- **Curation:** Tier 2 ×1 — `frontend.md` (single-source a `conductor-core` static table to the webview via a thin read-only `#[tauri::command]` returning the core type + `#[derive(Serialize)]`, never re-author in TS; `lampForRecord` lives in `lamp.ts` beside `LAMP_META`). Filtered 2 (1 dup [lampForRecord = the 2026-06-26 lamp-mirror learning] · 1 route-captured [classification-vs-results data model = the Epoch-10 carry below]). 0 conflicts · 0 deferred.
- **P4 decision:** data source = **classification + defer live join** (AskUserQuestion) — recorded in `scope.md` + `plan.md`.
- **Last failed command:** none.
- **Operator visual check (carried):** the new `CoverageMatrix` view + the DEV gallery (`npm run dev` → `#gallery`) are build/type-verified but NOT visually smoke-tested (no display; same posture as Epoch-9 ch1–ch5). The release-gate smoke (`agent-run.sh run`) passed — the real conductor-tauri-compiles-against-bundle proof.
- **Follow-up — NEW (Epoch-10):** wire the coverage view's **live per-P-ID verdict lamps** — needs a new `conductor-report` "latest `RunRecord` per P-ID" runs.db query (JSON1 over `p_ids`; `RunsDb` currently exposes only `open`/`insert`/`get`), then populate `CoverageMatrix`'s `lamps` prop. Naturally lands when Epoch-10 live-Pulse runs produce real per-P-ID `RunRecord`s.
- **Follow-up (carried — unchanged):**
  - Expose `conductor_verify::readiness(...)` to retire the `UNREACHABLE_PRECONDITION` dup in `conductor-run`.
  - `scenario.run` root obs span (the run driver) — deferred (Epoch-10).
  - `indicatif` 0.17→0.18 may drop `number_prefix`, retiring the RUSTSEC-2025-0119 deny.toml ignore.
  - test-plan §3 ↔ obs-plan §3 dual-RECORD-SHAPE reconcile (test-plan §3 OWNER).
  - `opentelemetry-proto default-features=false` trim (dormant transitive OTel SDK).
