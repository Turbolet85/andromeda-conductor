# Codebase Research — 2026-06-27-coverage-matrix-view

## Scope
- **Depth:** moderate-deep (mature codebase, well-bounded UI chunk with one backend-shaped open question) · **Reads:** 11 · **Globs/Greps:** 4 / 4 · **Code-graph queries:** 2 (coverage symbols + `Lamp::for_record` callers).

## Files inspected
- `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` (full) — the reuse target. Props: `{ lamp: Lamp, size?: 'sm'|'md' }`. Renders `LAMP_META[lamp]` → glyph (color via inline `style={{ color: 'var(${meta.token})' }}`, `aria-hidden`) + visible `.lamp__label` text. **Consume as-is; do not modify.**
- `crates/conductor-tauri/ui/src/lamp.ts` (full) — `type Lamp = 'Pass'|'Fail'|'Hold'|'Manual'|'Residual'|'Blocked'` + `LAMP_META` (label/prefix/token/glyph) + `LAMP_ORDER`. **Mirrors `lamp.rs`.** Has the lamp vocabulary but **NOT** a `RunRecord → Lamp` projection (no `for_record` equivalent yet).
- `crates/conductor-core/src/lamp.rs` (full) — `Lamp::for_record(&RunRecord)` is the verdict-first projection to mirror: `(Blocked,_)→Blocked`; `(KnownResidual,_)→Residual`; `(_,Some(Pass))→Pass`; `(_,Some(Fail))→Fail`; `(_,Some(CalibrationRegion))→Hold`; `(ManualCheck,None)→Manual`; `(Pass,None)→Pass`; `(Fail,None)→Fail`. **This exact match arm order is the spec for a TS `lampForRecord`.**
- `crates/conductor-core/src/coverage.rs` (full) — `coverage_matrix() -> &'static [CapabilityRow; 60]`; `CapabilityRow { p_id, title, category, mode: CoverageMode }`; `CoverageMode { Auto, DriveObserve, StaticOnly }` (serde `"auto"`/`"drive+observe"`/`"static-only"`, `.label()` identical to wire). **The single source of the 60-P-ID classification; pure, no IO.** Completeness tests already guard P-001..P-060 zero-gap.
- `crates/conductor-report/src/coverage.rs` (full) — `CoverageMatrix::render()` Markdown = `P-ID | Title | Category | Mode` ONLY (no verdict, no lamp); deterministic pure render; `CoverageMatrix::write()` atomic overwrite of `coverage-matrix.md`. **Confirms: the canonical "coverage matrix" artifact is the classification, lamp-free.**
- `crates/conductor-cli/src/commands/coverage.rs` (full) + `render.rs` (grep) — `conductor coverage` prints `render::coverage_table()` (the 60-row classification table); `--write` regenerates the artifact. The CLI coverage surface shows **mode, not verdict lamps**. (`render.rs` has separate `status_line`/`results_table` that DO call `Lamp::for_record` — those are the per-run results surfaces, distinct from coverage.)
- `crates/conductor-report/src/lib.rs` (full) + `db.rs` (grep) — public API: `CoverageMatrix`, `RunsDb`, `JournalWriter`, `RunReport`. **`RunsDb` exposes only `open` / `insert` / `get(run_id, scenario)`** — there is **no** "latest run per P-ID" and **no** "list all runs" query. A per-P-ID verdict join is NEW backend code (a JSON1 query over the `p_ids` array column).
- `crates/conductor-tauri/ui/src/App.tsx` (full) — current app body: a `<main>` with the scenario picker + run controls + the live `Channel` (`start_run`/`stop_run`). The view mounts here; `invoke<T>('cmd')` + `Channel` are the established frontend↔backend pattern. No router (single window; per `frontend.md`).
- `crates/conductor-tauri/ui/src/main.tsx` (full) — root render; DEV `#gallery` branch (`import.meta.env.DEV`) renders `Gallery` instead of `App`. The new view's all-states surface can ride the SAME DEV-gallery affordance.
- `crates/conductor-tauri/ui/src/components/ScenarioPicker.tsx` (full) — the closest UI precedent for a dense, list-of-rows component: `cmdk` `Command`/`Item` with `type-label` + `type-data` spans, `aria-current` on the selected item, co-located `ScenarioPicker.css`. The coverage list mirrors this structure (sans the combobox filter unless wanted).
- `crates/conductor-tauri/src/commands.rs` (full) — command surface: `list_scenarios` (read-only, manual `tracing::info_span!("tauri.command.list_scenarios")` + `count`/`latency_ms` log), `start_run` (background-thread `current_thread` runtime + `Channel`), `stop_run`. Path handles resolved via `resolve_under` + `sanitize_error` at the edge. **A new read-only `coverage_matrix` command slots in beside `list_scenarios` (same shape).**

## Graph impact (from the code-graph query → `tree-query-2026-06-27-coverage-matrix-view.json`)
- **`coverage_matrix()` / `CapabilityRow` / `CoverageMode`** — consumers today are `conductor-report::CoverageMatrix` (render) + `conductor-cli` (coverage cmd/table/smoke). Adding a Tauri command consumer is **additive** (a new inbound edge `conductor-tauri → conductor-core`/`conductor-report`, already-declared crate deps); zero blast radius on existing callers.
- **`Lamp::for_record`** — 3 non-test callers, all in `conductor-cli` (`commands/exit_code`, `render/status_line_styled`, `render/results_table_styled`); the rest are `conductor-core` unit tests. **No TS caller** (the desktop projection does not exist yet). Mirroring it in TS adds no Rust caller and cannot regress the Rust sites.

## Patterns detected
- **Lamp single-source mirror** (`ui/src/lamp.ts:1`): TS mirrors `lamp.rs` via `LAMP_META`; `StatusLamp` drives color from `LAMP_META.token` inline (never a per-variant CSS class) so the token→state map can't drift. The new `lampForRecord` belongs in `lamp.ts` next to `LAMP_META` (same single-source file).
- **Read-only command shape** (`commands.rs:80`): `#[tauri::command] pub fn list_scenarios() -> Result<Vec<ScenarioSummary>, String>` with a manual `tracing::info_span!("tauri.command.<name>")` + `info!(count, latency_ms)` boundary line + `sanitize_error` edge. A `coverage_matrix` command copies this exactly.
- **Dense-row component** (`ScenarioPicker.tsx:42`): `.map` of rows, each a label span (`type-label`) + meta span (`type-data`), `aria-current` selection, co-located `.css` binding `var(--…)` tokens. The coverage list reuses this row idiom + `StatusLamp`.
- **DEV-gallery exercise surface** (`main.tsx:14`): all-states inspection rides `import.meta.env.DEV && #gallery` (prod-stripped) — no production route needed to demo every lamp/row state.

## Conventions to follow
- **Single-source the classification** — mirror `conductor-core::coverage_matrix()` (don't re-author 60 rows in TS); expose it via a command, OR if static-mirrored, key the drift risk explicitly (the `coverage.rs:73` table is the truth). (`frontend.md` lamp-mirror rule; arch single-source binding.)
- **Tokens by name, never hardcode** (`frontend.md`): P-ID `--color-id-cyan` + `type-data` (JetBrains Mono), titles/mode `type-label` (IBM Plex Sans), row dividers `1px --border-subtle`, `space-md` padding, `--radius-md` container, no shadows. Dense list, **not** a KPI-card grid (design `§Component Patterns §3`; hard ban).
- **Never color-alone** (`a11y.md`): every lamp = glyph + text label (StatusLamp already does this); empty state real prose ("Not yet run" / "No scenarios loaded"), never a gray skeleton, never downgraded to Fail.
- **Semantic list + keyboard** (`a11y.md`): `list`/`listitem` (or table) semantics; rows reachable; `aria-current`/`--border-emphasis` for selection distinct from focus; virtual-scrolled rows stay reachable/announceable.
- **Manual span on any new command** (`observability.md` 2026-06-26): `let _span = tracing::info_span!("tauri.command.coverage_matrix").entered();` — NOT `#[tracing::instrument]`; `sanitize_error` on the edge; `count`/`latency_ms` are allowlisted log fields.
- **Frontend gates** (`testing.md`): `tsc --noEmit` + `vite build` + `npm audit` 0; if a Rust command is added, `agent-run.sh run` (workspace nextest + doctest + clippy `-D`) stays green + `Cargo.lock` un-drifted; GUI integration (tauri-driver) tests **defer to the Epoch-9 a11y-harness chunk** (the routine deferral — don't force a flaky thread-timing test).
- **Build order** (`frontend.md` 2026-06-24): `npm run build` must precede any `cargo` compile of `conductor-tauri` (the `ensure_frontend` step in `agent-run.sh run` already handles the smoke).

## New files to create
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (+ `.css`) — the dense 60-P-ID view; a `CoverageRow` sub-component (P-ID · title · category/mode · `StatusLamp`).
- (lamp projection) extend `ui/src/lamp.ts` with `lampForRecord(...)` — NOT a new file (keep the single lamp-truth file).
- **(Option B only)** a `conductor-report` runs.db query (e.g. `RunsDb::latest_per_p_id`) — a JSON1 query over `p_ids`.

## Files to modify
- `crates/conductor-tauri/ui/src/lamp.ts` — add the verdict-first `lampForRecord` (mirror `Lamp::for_record`) + a `RunRecord`-shaped TS type (verdict/state union).
- `crates/conductor-tauri/ui/src/App.tsx` and/or `Gallery.tsx`/`main.tsx` — mount the view (a panel/section in `App`, and/or the DEV gallery for all-states).
- **(Option A/B, if a command is added)** `crates/conductor-tauri/src/commands.rs` + `main.rs` (`generate_handler!`) — register the read-only `coverage_matrix` command. (No capability ACL entry needed — app commands aren't ACL-gated, per the scenario-suite-picker security learning.)

## Open questions
1. **Data source (the P4 decision — AskUserQuestion):** (A) classification view via a thin read-only `coverage_matrix` command exposing `coverage_matrix()`, reuse StatusLamp + ship the TS `lampForRecord`, per-P-ID lamps render "not yet run" — **defer** the live runs.db per-P-ID verdict join to Epoch 10; (B) full live verdict matrix — NEW `conductor-report` "latest RunRecord per P-ID" JSON1 query + command + full layout row (lamp·slo_tier·latency_ms) + un-run state; (C) pure static TS mirror, no backend (simplest but duplicates the conductor-core classification → drift risk vs the single-source rule).
2. **Mount placement:** an always-visible panel/section in `App` (below the picker) vs a switchable view — `frontend.md` says no router (single window, run-state-driven). Lean: a `App` section + the DEV-gallery all-states surface, consistent with Epoch-9 so far.
3. **Un-run lamp state:** how a P-ID with no RunRecord renders — a real-prose "Not yet run" affordance, kept **distinct** from measured-`Blocked` (`a11y.md` empty-state-is-prose). Only live if Option A/B reads run data.
