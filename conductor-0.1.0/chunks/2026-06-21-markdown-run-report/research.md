# Codebase Research — 2026-06-21-markdown-run-report

## Scope
- **Depth:** moderate · **Reads:** 7 (journal.rs, db.rs, run_record.rs, verdict.rs, report_state.rs, scenario.rs, core+report lib/Cargo) · **Globs/Greps:** 1 grep (`lamp|fn label|status_prefix`) + 3 code-graph queries

## Files inspected
- `crates/conductor-report/src/lib.rs` (full) — seam exports today: `mod db; mod journal;` + `pub use` of `RunsDb`/`RunsDbError`/`JournalWriter`/`JournalError`. The Markdown module is added here the same way.
- `crates/conductor-report/src/journal.rs` (full) — **the seam-writer pattern to mirror.** `JournalWriter::create(runs_dir: &Path, run_id: &str)` does `fs::create_dir_all(runs_dir)` then opens `<run_id>.jsonl`; `JournalError` is `#[non_exhaustive]` thiserror with `Io(#[from] std::io::Error)` + `Serialize(#[from] serde_json::Error)`. Hygiene test asserts the line contains no `C:\`, `/Users/`, `RunRecord`. Never-overwrite there = create+append (never truncate).
- `crates/conductor-report/src/db.rs` (full) — sibling seam: `RunsDb::open(runs_dir)` / `insert` / `get`; `RunsDbError` `#[non_exhaustive]` (Io/Sqlite/Json); `#[cfg(test)] open_in_memory()`; **loud duplicate-key error** (PK conflict → `Err`, never clobber). `value_as_wire(serde_json::to_value(enum))` extracts the bare serde wire string for `Verdict`/`ReportState`/`SloTier`. Hygiene test bans `C:\`/`/Users/`/`/home/`/`RunRecord`/`RunsDb` in stored cells.
- `crates/conductor-core/src/run_record.rs` (full) — the envelope `RunRecord`: 11 fields, 5 measurement fields are `Option` (`journal_emitted_at`, `read_back_observed_at`, `verdict`, `latency_ms`, `fingerprints`). `verdict: Option<Verdict>`, `state: ReportState` (always present). `blocked()` nulls the five; `measured()` requires a concrete `Verdict` (so a measured KnownResidual row **has** `Some(verdict)`).
- `crates/conductor-core/src/verdict.rs` (full) — `Verdict {Pass, Fail, CalibrationRegion}`; `label()` (CalibrationRegion→`"HOLD"`), `status_prefix()` (→`[HOLD]`), `default_report_state()` (CalibrationRegion→`ManualCheck`).
- `crates/conductor-core/src/report_state.rs` (full) — `ReportState {Pass, Fail, ManualCheck, KnownResidual, Blocked}`; `label()` (`Manual`/`Residual`/`Blocked`), `status_prefix()` (`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`).
- `crates/conductor-core/src/scenario.rs` (full) — `PId` is `#[serde(transparent)]` → renders as bare `"P-009"`. `SloTier {Tier5s,Tier20s,Tier90s}` serde-renames to `<5s`/`<20s`/`<90s` (directly human-renderable) + `deadline_ms()`.
- `crates/conductor-core/src/lib.rs` (full) — exports `Verdict`, `ReportState`, `RunRecord`, `PId`, `SloTier`, `now_rfc3339`, `mint_run_id`, etc. A new `Lamp` type + `mod lamp` slots in here.
- `crates/conductor-report/Cargo.toml` — deps: `conductor-core`, `rusqlite`, `serde_json`, `thiserror`; **dev-dep `assert_fs` only (no `insta`).**

## Graph impact (from the code-graph query)
- **`RunRecord`** — consumers outside its own module: `conductor-core/src/lib.rs` (re-export), `conductor-report/src/db.rs`, `conductor-report/src/journal.rs`, `conductor-verify/src/record.rs` (the `CheckOutcome→RunRecord` bridge). Adding a `RunRecord::lamp()` / `Lamp::for_record` is **purely additive** — no consumer breaks.
- **`conductor-report`** — `crate_edges` shows **zero** crates depend on it yet (cli/tauri are Epoch 8/9). The Markdown module has no downstream caller to break; it is a third sibling consumer of the envelope.

## Patterns detected
- **Seam-writer trio** (`journal.rs`/`db.rs`): each takes an already-resolved `runs_dir: &Path`, `create_dir_all`s it, writes a `run_id`-stemmed artifact, exposes a `#[non_exhaustive]` thiserror harness-fault enum. The Markdown writer is the third sibling.
- **Exact-string serialization goldens** (`run_record.rs:131` `measured_record_serializes_in_canonical_schema_order`, `scenario.rs:142`): the crate locks wire shapes with exact `assert_eq!` on the produced string, **not `insta`**. Markdown render should follow this (no new dep — security extract bans new crates).
- **Artifact-hygiene negative tests** (`journal.rs:128`, `db.rs:303`): assert the output contains none of `C:\` / `/Users/` / `/home/` / `RunRecord` / struct names. Reuse verbatim for the `.md`.
- **Enum wire-form reuse** (`db.rs:180` `value_as_wire`): `SloTier`/`Verdict`/`ReportState` already serialize to their canonical display strings — `SloTier` is `<5s` etc., usable directly in Markdown.
- **Loud-not-silent on conflict** (`db.rs:72` PK error): a repeat write is an `Err`, never a clobber — the model for the report's never-overwrite.

## Conventions to follow
- **`#[non_exhaustive]` thiserror seam error** with `Io(#[from] std::io::Error)` (`journal.rs:18`) — the new `ReportError` mirrors it; harness-fault only (verdict/error wall).
- **`runs_dir` is handed in already-resolved** (`journal.rs:7`, `db.rs:51`) — the render seam never touches `CONDUCTOR_RUNS_DIR` (cli edge owns it, Epoch 8).
- **Doc-comment cites the arch §anchor** (every seam file's module doc) — keep the convention.
- **`#[cfg(test)]` in-crate test module** with `assert_fs::TempDir` for file behavior (`journal.rs:62`, `db.rs:196`).

## New files to create
- `crates/conductor-core/src/lamp.rs` — the shared **`Lamp` enum** `{Pass, Fail, Hold, Manual, Residual, Blocked}` + the **verdict-first precedence resolver** (`Lamp::for_record(&RunRecord)` or `RunRecord::lamp()`) + `status_prefix()`/`label()`. The single source of lamp truth reused by Markdown (now), coverage-matrix (ch4), cli (Epoch 8), desktop (Epoch 9). Unit-tested for the full precedence table.
- `crates/conductor-report/src/report.rs` — the Markdown renderer: a pure `render(run_id, &[RunRecord]) -> String` (golden-testable, no IO, no clock) + a thin `RunReport::write(runs_dir, run_id, &[RunRecord]) -> Result<PathBuf, ReportError>` (render + `create_new` write, never overwrite) + `ReportError`.

## Files to modify
- `crates/conductor-core/src/lib.rs` — `mod lamp;` + `pub use lamp::Lamp;`.
- `crates/conductor-report/src/lib.rs` — `mod report;` + `pub use report::{RunReport, ReportError};` (names TBD in plan).
- (no `Cargo.toml` change expected — render needs only stdlib + `conductor-core` + `thiserror`; `serde_json` already present if a wire-form helper is wanted.)

## Open questions
- **Lamp precedence for KnownResidual when a verdict is present.** `measured()` always supplies a `Verdict`, so a KnownResidual row carries `Some(verdict)`. Strict "verdict-first else state" would render it by its verdict (e.g. `[FAIL]`), defeating KnownResidual's defined purpose ("distinguishing an accepted residual from a real Fail"). Resolution (to confirm at P5): `KnownResidual` and `Blocked` are **state-driven** (the verdict can't express them); the verdict-first rule governs only the Pass/Fail/Hold trichotomy and the Calibration→HOLD-vs-Manual case. Forced by both states' definitions; not a free choice.
- **Generated-at stamp.** Proposed: the renderer reads **no** wall-clock — it is a pure function of `(run_id, records)` (run_id already embeds the timestamp), keeping it exact-golden-able and sidestepping the std::time-vs-tokio concern. If a human-facing "generated at" is later wanted, the cli edge injects it. (Confirm at P5.)
