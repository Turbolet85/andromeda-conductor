# Codebase Research — 2026-06-16-emission-journal-writer

## Scope
- **Depth:** moderate · **Reads:** 12 (report/timeline/core src + Cargo.tomls + obs-plan §3) · **Globs/Greps:** 3

## Files inspected
- `crates/conductor-report/src/lib.rs` (full) — **bare stub**: a single doc-comment line, no code. The whole seam is greenfield.
- `crates/conductor-report/Cargo.toml` (full) — only dep is `conductor-core.workspace = true`. Needs `serde_json` + `thiserror` (+ light `tracing`) added; dev-deps `assert_fs` + `insta`.
- `crates/conductor-core/src/lib.rs` (full) — exports the shared vocab the envelope composes: `Verdict`, `ReportState`, `PId`, `Scenario`, `SloTier`, `CoreError`/`Result`, `mint_run_id`, `redact_value`/`sanitize_error`, `resolve_under`. **core is the dependency root** — the place a cross-seam shared type belongs.
- `crates/conductor-core/src/verdict.rs` (full) — `enum Verdict { Pass, Fail, CalibrationRegion }`, derives `Serialize`/`Deserialize`, **serializes to canonical PascalCase** (test-locked). `label()`/`status_prefix()` present.
- `crates/conductor-core/src/report_state.rs` (full) — `enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }`, same serde + canonical-name lock.
- `crates/conductor-core/src/scenario.rs` (full) — `PId(String)` is `#[serde(transparent)]` (serializes as bare `"P-009"`); `SloTier` serde-renames to `"<5s"`/`"<20s"`/`"<90s"`. Tests already lock these wire forms.
- `crates/conductor-core/src/obs.rs` (full) — **key reuse:** `pub fn mint_run_id()` already mints the filesystem-safe `YYYY-MM-DDTHH-MM-SS-mmm` stem from `std::time::SystemTime` (never tokio); private `civil_from_unix(secs)` is the UTC civil-conversion building block (dependency-free, Howard Hinnant). The self-obs layer stamps `timestamp_ms` via private `unix_millis()`. **The emission journal is a DISTINCT artifact from this `tracing` self-obs stream** (rules/observability.md is explicit — two record shapes).
- `crates/conductor-core/src/error.rs` (full) — `CoreError` is `#[non_exhaustive] { Config(String), Validation(garde::Report) }`. The verdict/error wall: outcomes ride in `Ok`, `Err` is harness-fault only.
- `crates/conductor-core/src/config_path.rs` (full) — `pub fn resolve_under(base, candidate) -> Result<PathBuf>` is the `CONDUCTOR_*` path guard; "reading the env vars and choosing the base happen at the cli edge (a later chunk)". → **the journal writer takes an already-resolved `runs_dir`; it does NOT read `CONDUCTOR_RUNS_DIR` itself.**
- `crates/conductor-timeline/src/{lib,scheduler,phase,convert}.rs` (full) — `run_timeline(&PhaseTimeline, seed) -> Result<Vec<PhaseTransition>, TimelineError>` "**emits nothing — no OTLP, no journal**". A `PhaseTransition` carries `{index, name, elapsed_ms}` (elapsed = **virtual** tokio ms). There is **no verdict-producing path at Epoch 2** (verify is Epoch 5).
- `Cargo.toml` workspace deps — `serde_json`, `thiserror`, `serde`, `tracing`, plus dev `assert_fs`/`insta`/`rstest`/`predicates` are all present in `[workspace.dependencies]`; no new *external* crate is introduced (audit/deny untouched).
- `.andromeda/obs-plan.md` §3 (lines 63–99) — the **authoritative JSONL schema** (binding from tests §5).

## Patterns detected
- **Canonical-name serde lock** (`verdict.rs:50`, `report_state.rs:56`, `scenario.rs:129`): every shared enum has a `serializes_to_canonical_names` test pinning the wire string. The envelope inherits this for free and a golden re-locks the composite line.
- **std::time stamping, never tokio** (`obs.rs:109` `mint_run_id`, `obs.rs:118` `unix_millis`): all wall-clock stamps already route through `std::time::SystemTime`; `civil_from_unix` (`obs.rs:127`) converts to UTC civil fields with zero deps.
- **Per-seam thiserror, verdict/error wall** (`scheduler.rs:21` `TimelineError`, `error.rs:13` `CoreError`): each seam owns a `#[non_exhaustive]` thiserror enum; I/O/transport faults are `Err`, outcomes are `Ok`. The report seam needs its own `JournalError`.
- **`#[tokio::test(flavor="current_thread", start_paused=true)]` + `assert_eq!(a,b)` determinism** (`convert.rs:69`): the established way to test seeded ordering — but the journal writer is **not** seeded (determinism-replay over the journal is route chunk #4, deferred).
- **Test style:** plain `#[test]` + small factory fns + `serde_json` assertions (the whole core uses this, not rstest) — match it.

## Conventions to follow
- **Shared types live in `conductor-core`** (`lib.rs:1` "the shared types every Conductor seam depends on"): `Verdict`/`ReportState` set the precedent; the envelope that composes them belongs beside them — this also avoids a `conductor-verify → conductor-report` edge (verify *produces* the envelope in Epoch 5).
- **Path resolution at the cli edge** (`config_path.rs:5`): the writer is parameterized by a resolved `runs_dir`; `CONDUCTOR_RUNS_DIR` + `resolve_under` land at the cli edge (Epoch 8).
- **Artifact hygiene** (rules/observability.md, security extract): journal lines carry only allowlisted schema fields — no host paths, no internal struct names; `Display`-not-`Debug` at edges.
- **insta in assert mode** (rules/testing.md): goldens fail (don't auto-write) in CI; the `.snap` files are generated once via `cargo insta` and committed.

## New files to create
- `crates/conductor-core/src/run_record.rs` — the `RunRecord` run-report envelope type (serde `Serialize`+`Deserialize`, 10 fields in canonical schema order, `Option` for the 4 measurement-dependent fields, a `RunRecord::blocked(..)` constructor) + its serde-lock tests.
- `crates/conductor-report/src/journal.rs` — `JournalWriter` (create+append `<runs_dir>/<run_id>.jsonl`, one JSON line per record, flush-per-append, never truncate) + `JournalError` (thiserror: `Io`, `Serialize`) + tests.
- `crates/conductor-report/src/snapshots/*.snap` — insta goldens (committed after first accept).

## Files to modify
- `crates/conductor-core/src/lib.rs` — `mod run_record;` + `pub use run_record::RunRecord;`; export `now_rfc3339`.
- `crates/conductor-core/src/obs.rs` — add `pub fn now_rfc3339() -> String` (RFC-3339 `…Z`, colon-delimited, seconds precision) reusing `civil_from_unix`; the `journal_emitted_at` stamp source. (Co-located here to reuse the existing civil math without refactoring/relocating `civil_from_unix` + its test.)
- `crates/conductor-report/src/lib.rs` — `mod journal;` + `pub use journal::{JournalWriter, JournalError};`.
- `crates/conductor-report/Cargo.toml` — add `serde_json`, `thiserror`, `tracing` (workspace); dev-deps `assert_fs`, `insta`, `predicates`.

## Open questions
- **journal_emitted_at format** — RESOLVED: RFC-3339 `…Z` string from `std::time` (obs-plan §3 "ISO-8601 from std::time::SystemTime"; arch §Timestamp formats colon-delimited `Z`). The integer-ms journal offset is the `runs.db` column form (Epoch 6), not the JSONL.
- **`read_back_observed_at` in the JSONL** — RESOLVED: NOT present. The tests/obs JSONL schema (the schema owner) carries `journal_emitted_at` + the *computed* `latency_ms` only; `read_back_observed_at` is the SLO-math intermediate / future `runs.db` column. (The arch §Standard Contracts example shows it, but obs/tests own the line schema and omit it — a deliberate divergence to note in the plan.)
- **"Wired into the timeline"** — this chunk has **no live call-site**: `run_timeline` emits nothing and no verdict producer exists until Epoch 5, so a real `RunRecord` cannot be produced yet. Deliver the type + stamper + writer + tests as a cohesive unit; defer the call-site to the verify/cli epochs. → `scope.md` slightly overstated the wiring; amend at P5 (intent-incomplete, not a defect).
