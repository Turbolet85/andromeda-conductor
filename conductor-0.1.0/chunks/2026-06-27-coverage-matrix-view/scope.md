# Scope — Coverage-matrix view

**Marker:** `2026-06-27-coverage-matrix-view`
**Version / Epoch:** conductor-0.1.0 · Epoch 9 (Desktop control panel) · ch6/10
**Working entry:** _Coverage-matrix view — dense single-row-per-P-ID list with verdict/report-state lamps_
**Folded annotations (from the taken-up working line):**
- **CARRY:** reuse the `StatusLamp` primitive (`crates/conductor-tauri/ui/src/components/StatusLamp.tsx`, from `2026-06-26-component-primitives-library`) for the per-P-ID lamps — mirror `conductor-core` `lamp.rs` spellings via `ui/src/lamp.ts` `LAMP_META`, **don't re-spell**; project each `RunRecord` → `Lamp` the **verdict-first** way `conductor-core::Lamp::for_record` does.

## Intent (one line)
Build the desktop **Coverage-matrix view** — a dense, single-row-per-P-ID surface listing all 60 Pulse capability P-IDs (P-001..P-060) with their verdict / report-state lamps, **reusing the `StatusLamp` primitive** — the first ch5-primitive-consuming *view* of Epoch 9 and the desktop mirror of the CLI's comfy-table 60-P-ID coverage table.

## What it builds

### 1. The dense 60-P-ID coverage list
A React 19 view component (under `crates/conductor-tauri/ui/src/`) that renders **all 60 P-IDs (P-001..P-060)** as a dense, **single row per P-ID** list/table — the definition-of-done coverage surface the desktop equivalent of `coverage-matrix.md` / the CLI coverage table. Each compact row carries, at minimum:
- the **P-ID** (P-001..P-060) and its scenario/capability title;
- its **coverage mode** classification (auto / drive+observe (operator-checklist) / static-only) — the same three-way split the Epoch-6 coverage-matrix generator + `coverage-matrix.md` already pin;
- a **`StatusLamp`** rendering the row's verdict / report-state, **never color-alone** (token color + glyph + label), chosen **verdict-first** (a CalibrationRegion row renders HOLD, not Manual — the `Lamp::for_record` precedence).

### 2. StatusLamp reuse (the folded CARRY)
The per-P-ID lamps **reuse the existing `StatusLamp` primitive** + `ui/src/lamp.ts` `LAMP_META` (which byte-mirrors `conductor-core/src/lamp.rs`); this chunk **must not re-spell** the lamp palette/glyph/label set, and **must not rebuild** the lamp component. The RunRecord→Lamp projection follows the verdict-first semantics of `conductor-core::Lamp::for_record` (verdict precedence over state for lamp selection).

### 3. A mount into the desktop surface
The view becomes reachable in the running app (a panel / route / tab off the existing window shell + titlebar), composing with the Epoch-9 surfaces already shipped (frameless titlebar, picker, run controls). Exact placement (always-on panel vs a switchable view) is a layout decision for the plan, grounded in `layout-templates.md`.

## Boundaries

**In scope:**
- The Coverage-matrix **view component** + its row sub-component, under `crates/conductor-tauri/ui/src/` (co-located CSS per the existing `Titlebar`/`RunControls`/`ScenarioPicker`/`components/*` convention).
- Rendering **all 60 P-IDs** dense, one row each, with the three-way coverage-mode classification + a per-row `StatusLamp`.
- **Reuse** of `StatusLamp` + `ui/src/lamp.ts` `LAMP_META`; the verdict-first RunRecord→Lamp projection in TypeScript, kept byte-consistent with `conductor-core::Lamp::for_record` (shared vocabulary, never a re-definition).
- A way to mount/reach the view in the app; accessibility baked in at the component level (semantic table/list + row semantics, keyboard reachability, the never-color-alone lamp encoding).
- TypeScript strict (no `any`, functional components + hooks); design-token-sourced styling; frontend gates `tsc` (strict) + `vite build` + `npm audit` (0).

**Out of scope (deferred — do not build here):**
- The other Epoch-9 views — Run-report + operator-checklist views (ch7), the Operator-pause go/no-go dialog wiring (ch8).
- The desktop **a11y harness + verification** (axe/Lighthouse/contrast/keyboard, tauri-driver) — ch9; this chunk bakes accessibility *into* the view but does not stand up the harness, and the deferred `tauri::test` GUI-integration tests remain pinned to ch9.
- New scenarios / P-IDs / seam crates; any change to the `Verdict`/`ReportState`/`Lamp` engine model (the lamp vocabulary is consumed, never redefined).

**Ambiguous — resolved in the plan (P4), see Open questions:** whether the 60-P-ID coverage data (and per-P-ID latest verdict/state) is sourced **live** (a new read-only `#[tauri::command]` projecting the coverage classification + latest `RunRecord` per P-ID from `runs.db`/the coverage generator) or **presentationally** (a static/`conductor-core`-mirrored projection in the UI, deferring live runs.db wiring) — this determines whether the chunk stays UI-only or adds a thin read-only backend command.

## Surfaces & contracts touched
- `crates/conductor-tauri/ui/src/` — new view + row component (+ co-located CSS), and a mount point (App/titlebar/panel).
- `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` + `ui/src/lamp.ts` `LAMP_META` — **consumed** (read/reuse), not modified (unless a genuinely missing lamp affordance surfaces — a plan call).
- The **coverage-mode classification** + per-P-ID verdict/state projection — must stay consistent with `conductor-core`'s coverage-matrix generator + `Lamp::for_record` and the `coverage-matrix.md` / CLI coverage-table semantics (shared vocabulary).
- **If** the live-data option is chosen: a read-only `#[tauri::command]` in `crates/conductor-tauri/` (deny-by-default capability posture unchanged — app commands aren't ACL-gated, per the scenario-suite-picker security learning) + a `conductor-core`/`conductor-report` read path; **else** UI-only, zero Rust/seam change.
- Gates: frontend `tsc` + `vite build` + `npm audit`; if a Rust command is added, the workspace `agent-run.sh run` (nextest + doctest + clippy `-D`) stays green and `Cargo.lock` un-drifted.

## P4 resolution (data source) — RESOLVED
**Classification + defer live join** (AskUserQuestion, 2026-06-27). The view renders the single-sourced
`conductor-core::coverage_matrix()` classification via a thin read-only `coverage_matrix` command, **reuses
`StatusLamp`**, and **ships the verdict-first TS `lampForRecord`** (exercised in the DEV gallery); the per-P-ID
lamp column is the **"Not yet run"** prose this chunk. The **live runs.db per-P-ID verdict join defers to
Epoch 10** (a new `conductor-report` query) — so "with verdict/report-state lamps" is delivered as the lamp
*reuse + projection*, not live per-P-ID data. See `plan.md`.

## Open questions for the plan (P4)
- **Data source (primary decision):** ~~live read-only `#[tauri::command]` vs presentational static/core-mirrored projection~~ — **RESOLVED above** (classification via a thin command + reuse + projection; defer the live join).
- **Coverage classification source:** does the UI re-derive the 60-P-ID auto/drive-observe/static split, read the committed `coverage-matrix.md`, or get it from `conductor-core` via a command? (Single-source-of-truth: avoid re-spelling the classification the generator owns — mirror, don't re-author.)
- **Layout:** dense table vs virtualized list; column set (P-ID · title · mode · lamp); placement (always-on panel vs switchable view) — grounded in `layout-templates.md`.
- **Un-run state:** how a P-ID with **no** RunRecord renders its lamp (a neutral/"not yet run" affordance vs Blocked) — keep distinct from the measured-but-Blocked state.
