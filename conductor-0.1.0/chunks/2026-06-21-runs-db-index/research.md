# Codebase Research — 2026-06-21-runs-db-index

## Scope
- **Depth:** moderate · **Reads:** 5 (`conductor-report/{lib,journal}.rs`, `conductor-core/{run_record,lib,scenario}.rs`) · **Globs/Greps/Queries:** 5 (Cargo.tomls + 3 code-graph queries)

## Files inspected
- `crates/conductor-report/src/lib.rs` (full) — the seam root is **2 lines**: `mod journal; pub use journal::{JournalError, JournalWriter};`. The module doc already names the target: *"JSONL emission journal + Markdown run report + `runs.db` (rusqlite) storage seam."* This chunk adds the `runs.db` half (a sibling module + a `pub use`).
- `crates/conductor-report/src/journal.rs` (full) — **the sibling-writer template to mirror.** `JournalWriter::create(runs_dir: &Path, run_id: &str) -> Result<Self, JournalError>` takes an **already-resolved** `runs_dir` (CONDUCTOR_RUNS_DIR resolution is the cli edge's job, NOT the seam's). `JournalError` is a `#[non_exhaustive] #[derive(thiserror::Error)]` enum (`Io(#[from] std::io::Error)`, `Serialize(#[from] serde_json::Error)`). Tests use `assert_fs::TempDir`, build `RunRecord` directly + via `::blocked()`, and include an **artifact-hygiene test** asserting no `C:\` / `/Users/` / `RunRecord` (struct name) leaks and that the JSON keys are exactly the 11 schema names.
- `crates/conductor-core/src/run_record.rs` (full) — the 11-field `RunRecord` (already built). 5 measurement fields are `Option` (`journal_emitted_at`/`read_back_observed_at`/`verdict`/`latency_ms`/`fingerprints` → `None` ⇒ JSON `null` for blocked). `::blocked()` + `::measured()` constructors exist. Timestamps are `Option<String>` **second-precision RFC-3339** (e.g. `"2026-06-16T21:10:06Z"`); `latency_ms: Option<i64>` is the **precomputed** journal-relative delta ("not re-derived from the second-precision instant strings").
- `crates/conductor-core/src/scenario.rs` (full) — `SloTier` is serde-renamed to the wire forms `<5s`/`<20s`/`<90s` (variants `Tier5s`/`Tier20s`/`Tier90s`) + has `deadline_ms()`. `PId` is `#[serde(transparent)]` → bare string `"P-009"`. `seed` is **`u64`**.
- `crates/conductor-core/src/lib.rs` (full) — exports for this chunk: `RunRecord`, `Verdict`, `ReportState`, `SloTier`, `PId`, `resolve_under` (the `std::fs` path-handle guard, used at the cli edge).

## Graph impact (code-graph `occ` query)
- **`RunRecord`** — defined `conductor-core/src/run_record.rs` (49 refs incl. own tests); **2 real consumers**: `conductor-report/src/journal.rs` (18) + `conductor-verify/src/record.rs` (15, the ch1 `CheckOutcome→RunRecord` bridge). The new `RunsDb` is a **3rd, additive consumer** — it changes **no** existing symbol signature, so impact is low/isolated. (`defs`/`occ` are SCIP-encoded — query on `symbol LIKE …`, not a `name` column.)

## Patterns detected
- **Sibling writer pattern** (`journal.rs:46`): `create(runs_dir, run_id)` over an already-resolved dir; harness faults are a typed `thiserror` enum (`Err`), never a verdict (verdict/error wall). `RunsDb::open(...)` should mirror this exactly.
- **`#[non_exhaustive]` seam error** (`journal.rs:18-27`): "so later report-seam chunks extend the fault surface" — add a `RunsDbError` (or extend) with a `Sqlite(#[from] rusqlite::Error)` variant + `#[non_exhaustive]`.
- **Serde wire-form is the single source of truth** (`scenario.rs:142`, ch1 golden): `SloTier`/`Verdict`/`ReportState` already serialize to canonical strings (`<5s`, `Pass`, `Blocked`). The TEXT columns should derive the stored string **from serde** (e.g. `serde_json::to_value(&v)?.as_str()`), not a parallel hand-written match that could drift from the `#[serde(rename)]`.
- **Artifact-hygiene test** (`journal.rs:128`): the runs.db tests should carry the analogous assertion (no host paths / struct names in any stored cell).

## Conventions to follow
- **Already-resolved path in, no env in the seam** (`journal.rs:7-8`) — `RunsDb` takes `runs_dir` (or the `runs.db` path); `CONDUCTOR_RUNS_DIR` + `resolve_under` stay at the cli edge (Epoch 8).
- **rusqlite bound parameters only** (security/obs/tests extracts; CLAUDE.md) — `?`/`named_params!`, never `format!`-built SQL.
- **`bundled` rusqlite** already pinned at workspace `Cargo.toml:46` (`rusqlite = { version = "0.38.0", features = ["bundled"] }`) — add `rusqlite.workspace = true` to `conductor-report/Cargo.toml` `[dependencies]`.
- **Test DBs**: in-memory `Connection::open_in_memory()` (unit) or `assert_fs::TempDir` file DB (integration) — `assert_fs` is already a `conductor-report` dev-dep.

## New files to create
- `crates/conductor-report/src/db.rs` — the `RunsDb` storage type: `open(runs_dir|path)` (creates `runs.db` + `CREATE TABLE IF NOT EXISTS`), `insert(&RunRecord)` (bound-param), the `RunsDbError` thiserror enum, a minimal read used by tests (and forward-useful for the Epoch-8 `status` command), + `#[cfg(test)]` module.

## Files to modify
- `crates/conductor-report/src/lib.rs` — `mod db;` + `pub use db::{RunsDb, RunsDbError};` (the doc comment already promises the seam).
- `crates/conductor-report/Cargo.toml` — add `rusqlite.workspace = true` to `[dependencies]` (and `serde_json` already present for the JSON1 array (de)serialization).

## Open questions
1. **Timestamp column representation** — arch §Data model conventions fix `journal_emitted_at`/`read_back_observed_at` as **integer-millisecond offsets, "not ISO strings"**, but the envelope only carries **second-precision RFC-3339 strings** + a precomputed `latency_ms`. Storing epoch-ms needs an RFC-3339 parser dep (chrono/`time` — currently ABSENT; would need audit/deny review) for no SLO-math gain (`latency_ms` already is the delta). → **AskUserQuestion in P4** (the central schema-fidelity decision; flagged by all 4 in-domain distillers).
2. **`seed` is `u64`, SQLite INTEGER is signed i64** — rusqlite rejects `u64 > i64::MAX`. Default: bit-cast `as i64` on write / `as u64` on read (lossless round-trip). Resolve as an implementation note, not a user ask.
3. **Insert grain / conflict policy** — row grain is `(run_id, scenario)` per check; default `PRIMARY KEY (run_id, scenario)` (arch "no synthetic PK") + plain `INSERT` (a duplicate raises a constraint error = a loud harness fault, honoring "never silently clobber"). Default; surface at P5.
