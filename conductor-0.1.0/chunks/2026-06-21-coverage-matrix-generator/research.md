# Codebase Research — 2026-06-21-coverage-matrix-generator

## Scope
- **Depth:** moderate · **Reads:** 7 (report.rs, lamp.rs, core lib.rs, report lib.rs, run_record.rs, 2× Cargo.toml) · **Globs/Greps:** 1 grep · **Code-graph:** 1 query (trace at `tree-query-2026-06-21-coverage-matrix-generator.json`)

## Files inspected
- `crates/conductor-report/src/report.rs` (full) — **the seam to mirror.** `RunReport::render(run_id, &[RunRecord]) -> String` is a pure clock-free `writeln!`-into-`String` builder; `RunReport::write(runs_dir, run_id, records) -> Result<PathBuf, ReportError>` opens `create_new` (never-overwrite). `ReportError` is `#[non_exhaustive]`, `Io(#[from] std::io::Error)` only — harness fault, never a verdict. Helper `wire(serde_json::to_value(enum))` renders enums by their serde wire string (single source of truth). Tests include `no_host_paths_or_struct_names_leak` (asserts no `C:\`, `/Users/`, `/home/`, `RunRecord`, `RunReport`, `Lamp`, …) and `render_is_deterministic`.
- `crates/conductor-core/src/lamp.rs` (full) — `Lamp::for_record(&RunRecord) -> Lamp` (verdict-first 6-way). **Doc comment line 9-10 already names "coverage matrix" as an intended Lamp consumer.** Key fact: `for_record` takes a **`RunRecord`**, i.e. Lamp encodes verdict *status*, NOT the auto/drive+observe/static-only *classification mode* — they are different axes. `status_prefix()`/`label()` give the `[PASS]`-style ASCII.
- `crates/conductor-core/src/lib.rs` (full) — core is the workspace dep-root; owns `Verdict`, `ReportState`, `RunRecord`, `Scenario`/`PId`/`SloTier`, `Lamp`, `ClaimClass`/`ComparisonKind`/`ExpectedCheck`, `redact_value`/`sanitize_error`, obs. Module-per-type (`mod lamp;`, `mod run_record;`, …) with a flat `pub use`. A new `mod coverage;` + `pub use` fits this exactly.
- `crates/conductor-core/src/run_record.rs` (1-60) — `RunRecord` 11-field envelope; `PId(String)`, ctors `::measured`/`::blocked`. Confirms there is **no per-P-ID run index** — a `RunRecord` carries `p_ids: Vec<PId>` (a scenario → many P-IDs), so "latest status per P-ID" is non-trivial and has no producer until Epoch 7 scenarios exist.
- `crates/conductor-core/Cargo.toml` — deps: serde, serde_json, toml, thiserror, garde, tracing(-subscriber). **All needed deps present; no new dep required** for a code-native classification table + serde wire forms.
- `crates/conductor-report/Cargo.toml` — deps: conductor-core, rusqlite, serde_json, thiserror; dev: assert_fs. Render/write home; no new dep needed.

## Graph impact (from the code-graph query)
- **`RunRecord#`** — 42 refs (conductor-core) — the dominant shared type; a new coverage model does NOT touch it (classification is P-ID-keyed, not RunRecord-keyed).
- **`RunReport#` / `render()`** — 14 / 10 refs (conductor-report) — concentrated in report.rs + its tests; the pattern is self-contained, so a parallel `CoverageMatrix`/`CoverageReport` adds a sibling without disturbing callers.
- **`Lamp::for_record()` / `status_prefix()`** — 10 / 15 refs (conductor-core, consumed in report.rs) — reuse target IF a status column is wired; untouched under classify-only.

## Patterns detected
- **Pure clock-free render → exact-string test** (`report.rs:57`, `render_is_deterministic` `report.rs:286`): render builds a `String` with `writeln!`; deterministic for fixed inputs. The coverage render follows this verbatim (exact-string `assert_eq!` golden, not insta — per tests extract + 2026-06-16 amendment).
- **Enum wire-string render** (`report.rs:160` `wire()`): enums rendered via their `serde_json` wire form so Markdown/JSONL/runs.db spellings never drift. `CoverageMode` should carry `#[serde(rename = "...")]` and render through the same idea.
- **Artifact-hygiene test** (`report.rs:293` `no_host_paths_or_struct_names_leak`): a coverage render needs the same leak test (no host paths / struct names) — security + obs + a11y all require it.
- **Module-per-type in core** (`lib.rs:16-40`): `mod {type}; pub use {type}::{Type};` — the new `mod coverage;` lands here.

## Conventions to follow
- **Model in core, render in report** (arch extract AC #2): `CoverageMode` enum + the 60-row classification table in `conductor-core::coverage`; the Markdown render in `conductor-report`.
- **Code-native classification, not runtime file read** (security extract — "no new file deserialization / input boundary"): the 60-row table is a committed Rust `const`/fn (the source of truth), *seeded from* `.andromeda/refs/capability-verification-matrix.json` + `input.md` §Coverage classification at authoring time — NOT read from disk at runtime. Keeps render pure, deterministic, golden-testable, and input-boundary-free.
- **Verdict/error wall** (`report.rs:25`): a write fault is a typed `Err` (harness fault); classification correctness is enforced by test, not runtime error.

## New files to create
- `crates/conductor-core/src/coverage.rs` — `CoverageMode` enum (`Auto` / `DriveObserve` / `StaticOnly`, serde-renamed `"auto"`/`"drive+observe"`/`"static-only"`, + `label()`); a `CapabilityRow { p_id, title, category, mode }` (or similar) and the committed 60-row `coverage_matrix() -> [...; 60]` table covering P-001..P-060; a completeness self-test (all 60 present, contiguous, each exactly one mode).
- `crates/conductor-report/src/coverage.rs` (or extend `report.rs`) — `CoverageMatrix::render() -> String` (pure, clock-free) + `::write(path) -> Result<PathBuf, …>`; exact-string golden + completeness + no-leak tests.

## Files to modify
- `crates/conductor-core/src/lib.rs` — `mod coverage;` + `pub use coverage::{CoverageMode, …};`.
- `crates/conductor-report/src/lib.rs` — `pub use coverage::{CoverageMatrix, …};` (if a new module).

## Open questions
1. **Status column vs classify-only (the headline fork).** Lamp::for_record needs a `RunRecord`; there is **no per-P-ID run index and no scenarios until Epoch 7**, and arch+layouts extracts explicitly say "no live-status/runs.db reads in the Epoch-6 classification model." → Strong recommendation: **classify-only** now (mode per P-ID, no Lamp, no runs.db), with Lamp reuse as the documented Epoch-8/9 status-overlay seam. Confirm with the user at P4 (it decides whether runs.db/Lamp are touched; it also means follow-up (c)'s "coverage-matrix reuses Lamp" defers to the Epoch-9 *view*).
2. **Write semantics divergence.** `coverage-matrix.md` is a project-root **regenerated singleton** (definition-of-done), NOT a run_id-stemmed immutable artifact — so `create_new`/never-overwrite (the security+tests extracts' assumption, borrowed from the run report) is **wrong here**: regeneration must succeed. Recommend **atomic deterministic overwrite** (`.tmp` → rename), loud on IO fault. Flag at P5.
3. **Artifact path resolution.** Project-root `coverage-matrix.md` vs a `CONDUCTOR_*`-overridable dir — path resolution is a cli-edge concern (Epoch 8); the Epoch-6 `write` takes a caller-resolved path. Low-stakes; resolve in plan.
