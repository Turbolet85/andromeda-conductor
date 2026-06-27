# Scope — Run-report + operator-checklist views

**Marker:** 2026-06-27-run-report-operator-checklist-views
**Version:** conductor-0.1.0 · Epoch 9 (Desktop control panel) · ch7/10
**Mode:** UI + (thin, read-only) command — desktop surface over existing `conductor-core` / `conductor-report` types

## What it builds
Two desktop (React 19 webview) views that bring the run-report + operator-checklist surfaces — already
rendered headless (the Markdown run report + the CLI status lines) — into the Tauri control panel:

1. **Run-report view** — a per-scenario list of a run's `RunRecord`s rendered as **verdict lines**: each row
   shows the canonical envelope fields (P-ID(s) · scenario · verdict · state · latency_ms · slo_tier ·
   fingerprints) with a **verdict-first `StatusLamp`** (Pass/Fail/ManualCheck/KnownResidual/Blocked — the
   never-color-alone label+glyph). The desktop equivalent of `conductor-report`'s Markdown per-scenario render
   and the CLI's `[PASS]/[FAIL]/[HOLD]/[BLOCKED]` status lines.

2. **Operator-checklist view** — for **ManualCheck** scenarios (the DriveObserve / visual-claim family), the
   **induced-state checklist** the operator works through to record a go/no-go on a claim with no programmatic
   MCP read-back. Reuses the **`OperatorChecklist`** primitive (controlled `items` + `onToggle`;
   native-checkbox induced-state rows).

## Folded carries (from the working-route entry)
- **CARRY — reuse, don't rebuild:** `StatusLamp` (verdict lines) + `OperatorChecklist` (controlled `items` +
  `onToggle`; native-checkbox induced-state rows) ship from `component-primitives-library` (ch6,
  `ui/src/components/`). This chunk consumes them; it does NOT author new lamp/checklist primitives.
- **CARRY — data-sourcing pattern:** `lampForRecord` (verdict-first `RunRecord`→`Lamp` projection in
  `ui/src/lamp.ts`, mirrors `conductor-core::Lamp::for_record`) + the **read-only-`#[tauri::command]`-returns-
  the-`conductor-core`-type** pattern (`#[derive(Serialize)]` on the core/report struct + `invoke<T[]>` in TS,
  never re-author the shape in TS) ship from `coverage-matrix-view` (ch6). This chunk reuses BOTH to surface
  `RunRecord`s as verdict lines.

## Boundaries / non-goals
- **UI-focused, presentational-first** — mirror the ch5/ch6 posture (zero engine/seam model change expected).
  Any backend addition is at most a thin read-only `#[tauri::command]` returning an existing
  `conductor-core`/`conductor-report` type (+ a `#[derive(Serialize)]`), per the carried data-sourcing pattern.
- **No new lamp/checklist primitive** — reuse the shipped `StatusLamp` + `OperatorChecklist`.
- **No operator-pause WIRING** — the go/no-go AlertDialog gating a live timeline step is the NEXT chunk
  (ch8, "Operator-pause go/no-go dialog"). This chunk's operator-checklist is the ManualCheck *induced-state
  record* surface, not the live-hold dialog.
- **Status is never color-alone** — every verdict/state carries its text label + glyph (the `StatusLamp`
  contract); color encodes state only.

## Surfaces & contracts touched
- **Frontend:** `crates/conductor-tauri/ui/src/components/` (new view component[s] + CSS) · `App.tsx` (mount) ·
  `Gallery.tsx` (DEV exercise), reusing `StatusLamp` / `OperatorChecklist` / `lampForRecord` / `LAMP_META`.
- **Backend (thin, if needed):** `crates/conductor-tauri/src/commands.rs` + `main.rs` (register a read-only
  query command), single-sourcing a `conductor-report` / `conductor-core` type (run-report envelope =
  `RunRecord`; operator-checklist items = the scenario model's operator-checklist).
- **Reused contracts:** the run-report envelope (`run_id/seed/scenario/p_ids/verdict/state/journal_emitted_at/
  read_back_observed_at/latency_ms/slo_tier/fingerprints`, Blocked-row null rule) · the verdict-first lamp
  precedence · the ManualCheck operator-checklist (DriveObserve scenarios).

## Open scope question (resolve at P4)
**Run-report data source.** `coverage-matrix-view` deferred the live `runs.db` per-P-ID join to Epoch 10
(`RunsDb` currently exposes only `open`/`insert`/`get` — no "records for a run" / "latest report" query). The
same fork applies here: does the run-report view (a) wire a real read-only query over `runs.db` / the latest
JSONL journal now, or (b) follow the ch6 precedent — render from the shipped types over sample/DEV data and
defer the live `runs.db` read to Epoch 10 (the live-Pulse E2E epoch where real `RunRecord`s first exist)?
Flagged for the P4 AskUserQuestion; the extracts + research will inform the recommendation.

**RESOLVED (P4 AskUserQuestion → "live journal read now"):** (a). Research found the CLI `report` verb sources
`RunRecord`s from the **JSONL journal, not runs.db** (`report.rs`: `latest_run_id` → `read_journal`), so a live
read is cheap — no new crate edge (`RunRecord ∈ conductor-core`, already a dep), no new `RunsDb` query. Plan:
lift `latest_run_id`/`read_journal` to `conductor-core` (CLI↔Tauri DRY) + a thin read-only `run_report(run_id?)`
command. The runs.db "latest per P-ID" query the handoff deferred remains an Epoch-10 concern for the *coverage*
per-P-ID join (a different need). The **operator-checklist view stays presentational/sample-only** (no structured
induced/observation model in `conductor-core` — declare-only TOML comments; live items need a future scenario-model field).
