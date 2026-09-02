# Codebase Research — 2026-09-01-live-per-p-id-verdict-lamps

## Scope
- **Depth:** moderate · **Reads:** 12 · **Globs/Greps:** 9 · **Graph queries:** 6 (rust ×3, ts ×3)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL (5 sections +
  20 `## Session Additions` entries). **Read deliberately, because it does NOT auto-load here:** its
  `paths:` are `scripts/agent-run.*` · `crates/conductor-cli/**` · `crates/conductor-verify/**` ·
  `crates/**/tests/**`, and this chunk's modify-set is `conductor-tauri` + `conductor-run`. This is the
  exact miss the last wrap recorded (`recurrence-despite-learning` ×3).

## Files inspected
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (full) — already declares
  `lamps?: Record<string, Lamp>`, renders `<StatusLamp lamp={lamp} size="sm" />` when present, falls back
  to a `cov__unrun` "Not yet run" cell. Column order is **P-ID · Capability · Mode · Status**.
- `crates/conductor-tauri/ui/src/App.tsx` (full) — the call site. `<CoverageMatrix rows={coverage}
  unbacked={unbacked} />` at `:246`: **`lamps` is never passed**, so every row renders "Not yet run"
  forever. `report` state (`RunRecord[]`) is already in the same component, re-read on every terminal
  Channel stage via `loadReport()`.
- `crates/conductor-tauri/ui/src/lamp.ts` (full) — `LAMP_META` (six), `LAMP_ORDER`, `lampForRecord`,
  `RunRecord`. The TS mirror is faithful (see Patterns).
- `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` (full) — emits `lamp__glyph`
  (`aria-hidden="true"`, colored by `var(meta.token)`) **plus** `lamp__label` text. Not-color-alone is
  already satisfied by the primitive; no per-lamp `aria-live`.
- `crates/conductor-tauri/ui/src/components/RunReport.tsx` (full) — `<section className="report">` with a
  `report__summary` header then the table. The banner's insertion point is that header region.
- `crates/conductor-tauri/src/commands.rs` (`:160-290`, `:300-400`) — six commands; `run_report` reads the
  **JSONL journal** through `conductor_core::read_run_journal`, not `runs.db`. `EnvelopeStatus` is imported
  but used only on the WRITE path (`classify_run` at `:214`, threaded into `run_thread` at `:236/:266`).
- `crates/conductor-report/src/db.rs` (`:205-245`) — `insert_envelope` / `get_envelope`, both
  `rusqlite::params![…]` bound.
- `crates/conductor-core/src/lamp.rs` (`:37-48`) — `Lamp::for_record`, the lamp truth.
- `crates/conductor-core/src/run_journal.rs` (`:30-45`) — `read_run_journal` parses each line as a bare
  `RunRecord`; **the journal carries no envelope record**.
- `crates/conductor-run/src/lib.rs` (`:34`, `:820-860`) — `use conductor_report::{JournalWriter, RunReport,
  RunsDb}`; `persist` writes the envelope; `classify_run` computes it. **No read-side counterpart exists.**
- `crates/conductor-tauri/ui/wdio.conf.ts` (`:70-100`) — `specs: ['./test/a11y/accessibility.e2e.ts']`;
  `suites: { driven: ['./test/a11y/operator-hold.e2e.ts'] }`.
- `crates/conductor-tauri/capabilities/default.json` (full) — exactly three `core:window:*` permissions.

## Graph impact (from the code-graph query; `{run_dir}/tree-query-{marker}.json`)
- **`RunsDb::get_envelope`** — indexed (`conductor-report`, `db.rs:221`) and **0 callers** in the rust
  plane. Both cookbook preconditions hold (the plane built; the symbol is confirmed present via the
  `symbol` probe), so this is a real *consulted-but-no-match*: the method ships and nothing in production
  calls it. This is the CARRY's claim, confirmed independently.
- **`crate_edges`** — `conductor-tauri → {conductor-core, conductor-run}` only. `conductor-cli →
  {core, report, run, verify}`. `conductor-run → {core, emit, faults, report, timeline, verify}`.
  **`conductor-tauri` has no `conductor-report` edge** — the load-bearing fact for Half 2's route.
- **`CoverageMatrix` (ts)** — referenced from `App.tsx:5,51,71,246` and `Gallery.tsx:5,9-17,205`. The
  `lamps` prop is referenced at `Gallery.tsx:205` but **not** at `App.tsx:246`; the DEV-only Gallery is the
  sole existing consumer of the prop.
- **`lampForRecord` (ts)** — `RunReport.tsx:1,49` and `Gallery.tsx:7,22,23`. Adding a coverage-matrix
  caller is additive; no signature change, so no caller threading.
- **`RunReport` (ts)** — `App.tsx:6,267` and `Gallery.tsx:6,212`. **Both are caller-threading members**: a
  new required prop on `RunReport` must be supplied at `App.tsx:267` AND `Gallery.tsx:212`, or the DEV
  Gallery breaks `tsc --noEmit`. An optional prop threads only `App.tsx`.

## Patterns detected
- **The lamp mirror is faithful** (`lamp.ts:29-39` vs `lamp.rs:37-48`): arm-for-arm — `Blocked`→Blocked,
  `KnownResidual`→Residual, then verdict-first `Pass`/`Fail`/`CalibrationRegion`→Hold, then the
  verdict-less `ManualCheck`/`Pass`/`Fail` fallbacks. Answers arch's research question: the join must go
  through `lampForRecord`, never a `state`-only switch.
- **Lamps inside tables carry no per-row `aria-live`** (`RunReport.tsx:49` ships exactly this and was a11y-
  verified at `v2-22`). The live regions in the tree are `Titlebar.tsx:25` (`assertive`, run state) and
  `OperatorChecklistView.tsx:18` (`polite`, roll-up). This is the shipped answer to a11y's "per-row
  announcement noise" concern.
- **Read-only command shape** (`commands.rs:167-196`): `info_span!("tauri.command.<name>").entered()` →
  resolve dir → optional `resolve_under` traversal guard on a caller-supplied id → seam call →
  `tracing::info!(count, latency_ms, …)` → `Ok(value)`, with `.map_err(|e| sanitize_error(&e))` at every
  fallible step. The eighth handler copies this exactly.
- **Composition-root projection** (`commands.rs:214`): `conductor_run::classify_run(&load_envelope()?,
  &scenarios)` — `conductor-tauri` already reaches report-seam concerns *through `conductor-run`*.
- **Mock-runtime IPC tests ship for every read-only command** (`commands.rs:350/366/378`) —
  `coverage_matrix_command_returns_every_classified_pid`, `unbacked_auto_command_returns_the_core_ledger`,
  `run_report_command_returns_a_well_formed_record_list`. Assertions are on the JSON shape (the row types
  are `Serialize`-only). The new command inherits this standing tier.

## Conventions to follow
- **App-defined Tauri commands need NO capabilities entry.** `capabilities/default.json` holds exactly
  three `core:window:*` permissions and names no command; `.claude/rules/security.md` §Session Additions
  (2026-06-26) states app commands are not ACL-gated and warns *"Do NOT reflexively add per-command
  allowlist entries"*; arch's 2026-06-24 amendment dismissed registering them. **This answers security's
  research question in the negative** — the extract's "must be extended in lockstep" constraint does not
  apply to an app-defined command, and following it would add a meaningless entry.
- `.cov__unrun { color: var(--text-tertiary) }` (`CoverageMatrix.css:87`) already satisfies the
  layouts/design "absence is prose in text-tertiary" rule — no new not-yet-run treatment is needed.
- `--status-residual` is declared in both themes (`tokens.css:22` light `#9A93A8`, `:71` dark `#6E6478`);
  the sibling `.cov__mode--out-of-scope` (`:83`) is the in-repo precedent for the banner's treatment.
- `get_envelope` binds `run_id` (`rusqlite::params!`) — security's SQL question answered; the new path
  adds no SQL of its own.

## New files to create
- (none) — every surface this chunk touches already exists.

## Files to modify
- `crates/conductor-run/src/lib.rs` — a read-side counterpart to `persist` returning a run's
  `EnvelopeStatus`; it already holds the `conductor-report` edge and `use`s `RunsDb`.
- `crates/conductor-tauri/src/commands.rs` — the eighth `#[tauri::command]` + its `generate_handler!`
  registration (`:307-315`) + its mock-runtime IPC test.
- `crates/conductor-tauri/ui/src/App.tsx` — build `p_id → Lamp` from `report`; pass `lamps`; fetch the
  envelope and pass it to `RunReport`.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` — correct the stale `:38` comment
  ("Epoch 10", and "runs.db join" when the join reads the journal).
- `crates/conductor-tauri/ui/src/components/RunReport.tsx` + `RunReport.css` — the run-level banner.
- `crates/conductor-tauri/ui/src/Gallery.tsx` — **caller-threading member** from the graph: it calls both
  `CoverageMatrix` (`:205`) and `RunReport` (`:212`), so a new REQUIRED prop on either breaks
  `tsc --noEmit` here. Optional props thread `App.tsx` alone.
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — the ONLY file the routine `--e2e` suite
  loads; new assertions must live here, not in a new spec file.

## Open questions
- **Which record's lamp wins when several name one P-ID?** `RunRecord.p_ids` is a list and a suite writes
  many records. Worst-verdict-wins (with design's constraint that it must never downgrade a non-verdict
  `ReportState` into the red `Fail` treatment) vs most-recent-run-wins. → blocks: **plan-decision**.
- **Does the banner ride `RunReport` as a required or optional prop?** Required threads `Gallery.tsx`;
  optional does not. → blocks: **implementation-scope** (the file list above covers both).

## Noted, not taken on
- `agent-run.{sh,ps1}` `cleanup` runs `DELETE FROM runs` **only** (`agent-run.sh:138`,
  `agent-run.ps1:151`), while test-plan §3 now contractually requires `run_check` and `run_envelope` too —
  so a cleaned run leaves an orphan `run_envelope` row. **Owned by a different route entry** (Epoch 6's
  *Run-report envelope conformance gate*, which carries this as an explicit CARRY), so it stays out of
  this chunk. It is not a correctness hazard here: `run_id` is a never-reused timestamp and
  `insert_envelope` raises the PK constraint rather than clobbering, so orphans accumulate but never
  mis-attribute to a new run.
- The layouts extract states the coverage row anatomy as *lamp → scenario label → P-ID → Mode → slo_tier →
  latency_ms*; the shipped component is *P-ID → Capability → Mode → Status* with no slo_tier or
  latency_ms cell. A spec↔reality divergence that predates this chunk — surfaced for wrap's reconcile,
  not re-authored here (this chunk fills the existing Status cell, adding no column).
- a11y-plan §6's contrast table enumerates no pair for `--status-residual` over `--color-base`, and the
  banner colors **text** (SC 1.4.3, 4.5:1) rather than a non-text indicator (SC 1.4.11, 3:1). A plan gap
  this chunk exposes; the assertion is computable from the shipped tokens either way.
