# Scope — runs.db index

**Marker:** 2026-06-21-runs-db-index
**Epoch:** 6 — Run report & persistence (chunk 2 of 4)
**Working entry:** "runs.db index — rusqlite schema, bound-parameter writes, JSON1 fingerprint arrays"

## What it builds
The embedded **SQLite cross-run index** (`runs/runs.db`) storage path inside **conductor-report** — the thin,
synchronous convenience layer over the on-disk JSONL journal (the ground truth). Concretely:
- pull `rusqlite` (`bundled`, already pinned `0.38.0` in the workspace manifest but unused) into the
  `conductor-report` seam;
- a storage type (e.g. `RunsDb`) that **opens/creates** `runs.db` and **bootstraps the schema** with raw
  `CREATE TABLE IF NOT EXISTS` SQL (no migration framework — arch [ORM] None);
- a **bound-parameter** insert that persists one [`conductor_core::RunRecord`] (the per-scenario-check
  envelope) as a row, honoring the **Blocked-row NULL rule**;
- **fingerprints stored as a JSON1 TEXT array** of fingerprint strings (arch Data-model conventions), the
  representation the Epoch-6 P-036 recurrence query will index via SQLite JSON1.

This is the runs.db realization of the canonical envelope produced in Epoch-6 chunk 1
(`2026-06-21-run-report-envelope-serializer`). The envelope (`RunRecord`) already exists in conductor-core
with golden-locked serde + `::blocked()` / `::measured()` constructors + the NULL rule; this chunk only adds
the **rusqlite write seam** that maps that struct onto the fixed column contract.

## The fixed column contract (arch §Data model conventions — column types fixed on first write)
- keyed by **`run_id`** (with `seed` / `scenario`) — **no synthetic UUID/serial PK** is introduced; one row
  per **per-scenario-check** envelope, so the natural key spans `(run_id, scenario)` (a run drives many
  scenarios — the row grain is the check, not the run);
- `latency_ms` — **INTEGER** milliseconds, **NULL** for a blocked row;
- `slo_tier` — closed **TEXT** enum over exactly `<5s` / `<20s` / `<90s` (the `SloTier` wire form);
- `journal_emitted_at` / `read_back_observed_at` — stored as **integer-millisecond journal offsets** the SLO
  math consumes (arch Data-model conventions + obs-rule), **not ISO strings** — NULL for a blocked row;
- `fingerprints` — **JSON1 TEXT array** of fingerprint strings, indexed via SQLite JSON1; NULL for a blocked
  row;
- `verdict` — NULL for a blocked row; `state` — always present (`Blocked` for a blocked row).

## Boundaries (what this chunk does NOT do)
- NOT the cross-run **query surface** — percentile / latency aggregation and the **P-036 fingerprint-recurrence
  ("Previously seen") `JSON1` query** arrive with their consuming scenarios (Epoch 7) / the `status` read
  (Epoch 8); this chunk DESIGNS the schema to support them (JSON1 fingerprints, NULL-excludable measurement
  columns) and may add only the minimal read needed to prove round-trip.
- NOT the **Markdown** run report render (Epoch 6 chunk 3).
- NOT the **coverage-matrix** generator (Epoch 6 chunk 4).
- NOT live-run wiring — producing real envelopes from a running timeline + the agent-run `status` command that
  reads runs.db is Epoch 8.
- NOT a new envelope/struct — `RunRecord` is reused verbatim (no schema duplication, no parallel shape).
- NOT an async/connection-pool layer — rusqlite is deliberately **synchronous, off the tokio runtime** (arch
  [Database]); no `sqlx`, no pool.

## Contracts / surfaces it touches
- **Blocked-row NULL rule** (arch §Standard Contracts): a `Blocked` row populates only `run_id`, `seed`,
  `scenario`, `p_ids`, `slo_tier`; `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`,
  `fingerprints` are stored **NULL** — never phantom values. Cross-run latency/percentile + P-036 queries MUST
  be able to exclude NULL-measurement rows (the schema must make that cheap).
- **Security — bound parameters only** (security-rule §Input Validation): rusqlite **bound parameters**
  exclusively; **never** `format!` / string-concatenated SQL, even for synthetic data.
- **Artifact hygiene** (CLAUDE.md universal invariant): no absolute host paths or internal struct/field names
  leak into `runs.db`; the db path itself is resolved/canonicalized at the `conductor-cli` edge
  (`CONDUCTOR_RUNS_DIR`), not inside the seam.
- **Verdict/error wall:** a write failure is a **harness fault** (`Result::Err` → a `conductor-report` typed
  error, `thiserror`), never a verification outcome; persisting an envelope never reinterprets the SUT verdict.
- **Determinism / `std::time`:** runs.db stamps derive from the journal (whose stamps are `std::time`), never
  tokio's virtual clock.

## Open questions (for the plan / val-1 to resolve)
- **Timestamp column representation:** arch Data-model conventions fix `journal_emitted_at` /
  `read_back_observed_at` as **integer-millisecond journal offsets**, but the `RunRecord` envelope carries them
  as **second-precision RFC-3339 strings** and already carries the precomputed millisecond `latency_ms`. So:
  store the ISO strings as TEXT (contradicts arch), convert ISO→epoch-ms on write, or store `latency_ms` as the
  authoritative measurement and treat the two instant columns as integer offsets derived how? Resolve in P4
  (likely AskUserQuestion) — this is the central schema-fidelity decision.
- **Read surface in scope?** Whether ch2 ships only the write + schema (read deferred), or also a minimal
  `get`/`list`-by-`run_id` read to prove round-trip in tests (the integration-test pattern uses
  `open_in_memory()` / `assert_fs` file DB).
- **`p_ids` storage:** arch indexes "P-IDs" but only explicitly fixes *fingerprints* as JSON1 — store `p_ids`
  (a `Vec<PId>`) as a parallel JSON1 TEXT array, or a delimited TEXT? Resolve in P4.
- **Insert grain / conflict policy:** one row per `(run_id, scenario)` check — PRIMARY KEY / UNIQUE on that
  composite, and the on-conflict behavior (append-mostly: reject vs replace a re-run of the same check).

## Resolution (P3 research + P4 decision) — refined scope
P3 confirmed the envelope (`conductor-core::RunRecord`, 11-field golden-locked serde + `::blocked()`/
`::measured()`) and the sibling writer pattern (`conductor-report::JournalWriter::create(runs_dir, run_id)`,
already-resolved dir, `#[non_exhaustive]` thiserror, artifact-hygiene test) pre-exist; `rusqlite 0.38.0
bundled` is pinned in the workspace manifest but unused. The deliverable is the **`RunsDb` write seam in
conductor-report** (new `db.rs` + `lib.rs` export + the `rusqlite` dep). The open questions resolve as:
- **Timestamp columns (P4 user decision = Option A):** store `journal_emitted_at`/`read_back_observed_at` as
  **TEXT RFC-3339** (verbatim from the JSONL envelope) and `latency_ms` as **INTEGER** (the value the SLO math
  consumes). Dep-free + lossless. This **deviates from arch §Data-model conventions' literal "integer-ms
  offsets, not ISO strings"** for those two columns — the envelope only carries second-precision RFC-3339 +
  a precomputed `latency_ms`, so literal integer-ms storage would need a date-parser dep for no gain. Recorded
  as a **wrap doc-reconcile** (arch under-specified the columns given the envelope's real shape; `latency_ms`
  IS the integer the SLO math reads and is stored as INTEGER, satisfying the convention's intent).
- **`p_ids` storage:** JSON1 TEXT array (parallel to `fingerprints`), via `serde_json`.
- **Read surface:** a minimal `get(run_id, scenario)` is included to prove round-trip in tests (forward-useful
  for the Epoch-8 `status` read); cross-run aggregation / P-036 recurrence stay deferred.
- **Insert grain / conflict:** PRIMARY KEY `(run_id, scenario)` (no synthetic PK) + plain `INSERT` (a
  duplicate raises a loud constraint error — never a silent clobber).
- **`seed: u64`:** stored in the SQLite signed-`INTEGER` column via `as i64` bit-cast (round-trips `as u64`).
- **`db.insert_run` obs span:** DEFERRED to Epoch 8 (cli/timeline caller, run_id-scoped traced context) —
  conductor-report has no `tracing` dep and the sibling `JournalWriter` carries no library-level span.
The boundaries above (no cross-run query, no Markdown, no coverage-matrix, no live wiring, no new envelope, no
async/pool) still hold.
