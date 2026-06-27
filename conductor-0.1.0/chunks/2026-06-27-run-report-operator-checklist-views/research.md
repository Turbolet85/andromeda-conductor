# Codebase Research — 2026-06-27-run-report-operator-checklist-views

## Scope
- **Depth:** moderate-deep · **Reads:** 11 · **Globs/Greps:** 4 · **code-graph queries:** 3

## Files inspected
- `crates/conductor-core/src/run_record.rs` (full) — the 11-field envelope. **Already `#[derive(Serialize, Deserialize)]`** (line 16) — surfacing it via a command needs NO new derive (unlike `CapabilityRow`). `verdict: Option<Verdict>`, `state: ReportState` (always present), `latency_ms: Option<i64>`, `slo_tier: SloTier`, `fingerprints: Option<Vec<String>>`, `p_ids: Vec<PId>`. `blocked()` / `measured()` constructors exist (the Blocked-row null rule is structural).
- `crates/conductor-tauri/ui/src/lamp.ts` (full) — `lampForRecord(state, verdict)` (line 31) is the verdict-first projection the run-report view needs — **already shipped**. `Lamp` union + `LAMP_META` (label/prefix/token/glyph) + `LAMP_ORDER`. Blocked/KnownResidual are state-driven and precede the verdict.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (full) — the **precedent view**: presentational `{ rows, lamps? }`, dense `<table>`, `type-data`/`type-label`/`type-body` classes, `aria-label` + `role="group"` scroll container, reuses `StatusLamp`, `lamp ? <StatusLamp/> : "Not yet run"`. The run-report view mirrors this shape arm-for-arm.
- `crates/conductor-tauri/ui/src/components/OperatorChecklist.tsx` (full) — `ChecklistItem { id, induced, observation, checked }` + `{ items, onToggle(id, checked) }`, controlled native `<input type=checkbox>` in a `<label>`. Ready to reuse; has NO header/footer/unticked-count (the view adds that).
- `crates/conductor-tauri/src/commands.rs` (full) — the read-only command pattern: `coverage_matrix()` / `list_scenarios()` are nullary `Result<Vec<T>, String>` with a manual `tracing::info_span!("tauri.command.<name>").entered()` + `count`/`latency_ms` info line + `sanitize_error` edge. `runs_dir()` (line 44) already resolves `CONDUCTOR_RUNS_DIR` via `resolve_under`. `start_run` persists a real Blocked `RunRecord` to runs.db + `runs/<run_id>.jsonl`.
- `crates/conductor-tauri/ui/src/App.tsx` (full) — mount + data-sourcing: `invoke<T[]>('cmd').then(setState).catch(setError).finally(setLoading)` in a `useEffect`; renders loading / error (`role="alert"`) / empty (real prose) / data states; each surface is a `<section>` with a `type-heading` `<h2>`. The run-report section plugs in identically.
- `crates/conductor-tauri/ui/src/Gallery.tsx` (full) — DEV `#gallery` (behind `import.meta.env.DEV`): `SAMPLE_ROWS`/`SAMPLE_LAMPS` exercise `lampForRecord('Pass','Pass')` + `lampForRecord('Blocked', null)`; `OperatorChecklist` driven by `useState<ChecklistItem[]>` + a `toggle`. The new view's DEV exercise lands here.
- `crates/conductor-cli/src/commands/report.rs` (full) — **the canonical RunRecord read path**: `report(run_id?)` → `latest_run_id` (lexicographically-greatest `<run_id>.jsonl` stem) → `read_journal` (read file, `serde_json::from_str::<RunRecord>` per non-empty line) → `render::results_table`. **Reads the JSONL journal, NOT runs.db.** Both fns are bin-private (lift-to-core candidate for DRY).
- `crates/conductor-report/src/db.rs` (full) — `RunsDb` exposes only `open` / `insert` / `get(run_id, scenario)` (single-row). **No "all records for a run" query.** Confirms: a runs.db-backed run-report would need a NEW query; the journal-read path avoids `RunsDb` (and `conductor-report`) entirely.
- `crates/conductor-core/src/scenario.rs` (grep) — the operator-checklist (DriveObserve P-025/026/027/P-032) scenarios are **declare-only with empty `expected`** (tests at :718/:750); the induced/observation pairs live only in TOML comments. **No structured operator-checklist data in the Rust model** — surfacing live checklist items would require a scenario-model field (an engine change, out of scope).
- `crates/conductor-core/src/lamp.rs` (grep — `Lamp` @ :16, `for_record` @ :37) — the single lamp-truth `lamp.ts` mirrors; confirmed present.

## Graph impact (from the code-graph query)
- **`RunRecord`** — referenced only in `conductor-cli` (`commands/mod.rs:16,20,25`, `commands/report.rs:6,23,28`, `render.rs:17,60`). `conductor-tauri` does NOT yet reference it → surfacing it there is **purely additive, zero cross-crate blast**.
- **`conductor-report` crate edges** — inbound from `conductor-run` + `conductor-cli`; outbound to `conductor-core`. **`conductor-tauri` does NOT depend on `conductor-report`** → a runs.db-backed read would add a new Cargo edge; the **journal-read path needs none** (`RunRecord ∈ conductor-core`, already a dep).
- **`RunsDb`** (`conductor-report/src/db.rs`) — `open`/`insert`/`get` only; no list-for-run accessor (consulted, confirms the gap).

## Patterns detected
- **Read-only command** (`commands.rs:98-109`): nullary `Result<Vec<T>, String>`, manual `tauri.command.*` span + `count`/`latency_ms`, `sanitize_error` edge, single-sources a core type. The run-report command copies this (with an optional `run_id` arg, validated/sanitized).
- **Presentational view + `invoke` in App** (`CoverageMatrix.tsx` + `App.tsx:42-47,141-160`): the component is pure props; App owns the `invoke`/loading/error/empty wiring. Same split for the run-report view.
- **DRY-to-core data sourcing** (frontend.md 2026-06-27): single-source a `conductor-core` type through a thin command; never re-author in TS. `report.rs`'s `latest_run_id`/`read_journal` are the lift-to-core candidates so CLI + Tauri share one journal reader.
- **lampForRecord reuse** (`Gallery.tsx:20-23`): verdict-first projection already exercised over Pass + Blocked — the run-report view consumes it identically.

## Conventions to follow
- **Manual span, not `#[tracing::instrument]`** on the command (obs.md 2026-06-26; `commands.rs:82,100`).
- **Never color-alone / Blocked ≠ Fail** (a11y.md, layouts extract): six lamps carry label+glyph; Blocked rows render measurement cols as `—`/null, hollow-ring glyph, never red.
- **Empty/in-progress as real prose** (frontend.md, a11y.md): "No run yet" — never a skeleton, never downgraded to Fail.
- **Tokens by name** (`tokens.css`, frontend.md): `var(--space-*)`/`--color-raised-1`/`--border-subtle`/`--radius-md`; borders-only depth; IBM Plex Sans body + JetBrains Mono data tier (`type-data`/`type-label`/`type-body`).
- **No new capability-ACL entry** (security.md 2026-06-26): app commands aren't ACL-gated.
- **Frontend build-gated only** (testing.md): `tsc --noEmit` + `vite build` + `npm audit`; GUI E2E/tauri-driver assertions defer to the Epoch-9 a11y-harness chunk.

## New files to create
- `crates/conductor-tauri/ui/src/components/RunReport.tsx` (+ `.css`) — presentational verdict-line list of `RunRecord[]` (reuse `StatusLamp` + `lampForRecord`), mirrors `CoverageMatrix.tsx`; Blocked-row `—`/null rendering.
- `crates/conductor-tauri/ui/src/components/OperatorChecklistView.tsx` (+ `.css`) — *(if a wrapper is chosen)* composes the `OperatorChecklist` primitive with the header + unticked-count footer roll-up (`role="status"`/`aria-live`); design §7. (Alternatively folded into `RunReport` as a ManualCheck-row expansion — HOW decided at implement.)

## Files to modify
- `crates/conductor-tauri/src/commands.rs` + `src/main.rs` — *(data-source-dependent)* add + register the read-only `run_report` command (mirrors `coverage_matrix`).
- `crates/conductor-tauri/ui/src/App.tsx` — mount the run-report section (invoke/loading/error/empty wiring) — *data-source-dependent*.
- `crates/conductor-tauri/ui/src/Gallery.tsx` — DEV exercise of `RunReport` (sample `RunRecord[]`) + the operator-checklist view.
- *(if live read lifted to core)* `crates/conductor-core/src/lib.rs` + a new `run_journal.rs` (move `latest_run_id`/`read_journal` from `conductor-cli/src/commands/report.rs`; CLI then calls core).

## Open questions
1. **Run-report data source (→ P4 AskUserQuestion).** (a) Wire a live journal read now — a `run_report(run_id?)` command mirroring `report.rs` (latest `<run_id>.jsonl` → parse `RunRecord`s), cheap (no new crate edge / no runs.db query; `start_run` already writes a journal so the GUI gets a real run→report loop, Blocked pre-Epoch-10); ideally lift `latest_run_id`/`read_journal` to `conductor-core` for CLI↔Tauri DRY. vs (b) Presentational-only + DEV/sample data, defer the live read to Epoch 10 (the ch6 coverage precedent). Extracts leaned "defer," but that assumed a runs.db query was needed — the journal path makes (a) materially cheaper than ch6's deferred join.
2. **Operator-checklist view data:** presentational/sample-only this chunk regardless — no structured induced/observation model exists in `conductor-core` (declare-only TOML comments); live items wiring needs a scenario-model field (a future engine chunk). Surface in the plan as a non-goal, not a question.
