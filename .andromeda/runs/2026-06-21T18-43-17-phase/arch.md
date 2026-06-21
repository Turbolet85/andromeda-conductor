# arch extract

## Relevance
partial — the chunk implements the storage schema and write path for run metadata, addressing the established SQLite decision and occupied-resource registration, but cross-run queries defer to later epochs.

## Constraints
- (arch §Stack and Technologies) rusqlite 0.38.0 + bundled SQLite 3.51.1 (JSON1) — synchronous, append-mostly embedded index off the tokio runtime; pinned versions in workspace.dependencies.
- (arch §Established Decisions [Database]) Raw SQL, no ORM/migrations; fixed column types on first write; no async-runtime entanglement; `bundled` yields cross-platform self-contained build.
- (arch §Data model conventions) Primary key keyed by `run_id` (with `seed`/`scenario`), one row per per-scenario-check; latency_ms INTEGER (NULL for blocked), slo_tier closed TEXT enum `<5s`/`<20s`/`<90s`, timestamps stored as integer-millisecond journal offsets (ground truth in JSONL, not ISO strings), fingerprints JSON1 TEXT array, verdict NULL for blocked, state always present.
- (arch §Standard Contracts) Blocked-row NULL rule — Blocked rows populate only run_id/seed/scenario/p_ids/slo_tier; verdict/journal_emitted_at/read_back_observed_at/latency_ms/fingerprints stored NULL (never phantom). Cross-run queries MUST exclude NULL-measurement rows.
- (arch §Inherited Defaults [Database]) Workspace.dependencies freeze rusqlite 0.38.0 + bundled SQLite; synchronous raw SQL (no sqlx/pool); no migration framework.
- (arch §Occupied Resources) runs.db file is an on-disk artifact under the `CONDUCTOR_RUNS_DIR` environment variable (default `runs/`), with runs.db as the SQLite index file living there.
- (arch §Cross-cutting Patterns [Determinism discipline]) Timestamp columns derive from the JSONL journal (`std::time::SystemTime`/`Instant`), never tokio's virtual clock, preserving journal-relative SLO math.

## Patterns to follow
- (arch §Standard Contracts §Data model conventions) JSON1 fingerprints indexed via `json_array` construction on write; column types fixed at schema creation (INTEGER/TEXT/NULL), no ALTER TABLE after first write.
- (arch §Established Decisions [Error Handling]) Write failures are harness faults (thiserror-typed error in conductor-report), never verification-outcome reinterpretation; Result::Err path only.
- (arch §Infrastructure Patterns [Build system]) Cargo workspace pinned versions in Cargo.lock; rusqlite `bundled` feature compiles SQLite C from source (budget longer cold builds).

## Anti-patterns to avoid
- (arch §Standard Contracts [Artifact hygiene]) No absolute host paths or internal struct/field names leak into runs.db; path resolution/canonicalization happens at conductor-cli edge (CONDUCTOR_RUNS_DIR), not inside the seam.
- (arch §Standard Contracts [Security — bound parameters only]) Never string-concatenate or format! SQL; bound parameters exclusively.
- (arch §Established Decisions [Module Boundaries]) No async-runtime dependency; rusqlite stays synchronous by design, off tokio's reactor.

## Contract bindings
- Emit seam ↔ Report seam: conductor-emit produces `RunRecord` envelope (serialized JSON+JSONL); conductor-report persists the same struct as `runs.db` row (round-trip binding).
- Verify seam ↔ Report seam: verdict/state/blocked-status classification originates in verify (verdict), reported seam finalizes (state via default mapping). Blocked rows originate in verify's preflight gate (standard contracts §Readiness gate).
- CLI bin ↔ Report seam: conductor-cli resolves `CONDUCTOR_RUNS_DIR`, opens the database on the resolved path, delegates write+read to report seam.
- Tests harness ↔ Report seam: test suite uses in-memory SQLite (`open_in_memory()`) for schema round-trip validation; production path uses file DB.

## Acceptance criteria contributions
- (arch) Code lives in `conductor-report` crate per workspace boundary rules (arch §Inherited Defaults §Module Boundaries).
- (arch) `runs.db` schema enforces Blocked-row NULL rule — measurement columns (verdict, latency_ms, timestamps, fingerprints) stored NULL ⇒ cross-run queries exclude via IS NOT NULL.
- (arch) Bound-parameter write path only (no string concatenation); SQL injection gates applied.
- (arch) Fingerprints persisted as JSON1 TEXT array (indexable via SQLite JSON1 for P-036 recurrence query, deferred).

## Relevant amendment history
- **2026-06-21-run-report-envelope-serializer** (§Read-Back Dependency Posture · §Probabilistic-Assertion Policy) — the upstream Epoch-6 chunk-1 that produced the RunRecord envelope this chunk persists; ManualCheck + Verdict→ReportState mapping finalized (impacts state column read semantics, though write-only for this chunk).
- **2026-06-15-conductor-core-shared-types** (§Stack and Technologies) — serde_json 1.0 registered as the JSON serializer for run-report envelope (JSONL journal, roundtrip tests); this chunk reuses RunRecord struct verbatim.
- **2026-06-14-cargo-workspace-scaffold** (§Stack and Technologies, §Infrastructure Patterns) — rusqlite 0.38.0 + bundled SQLite 3.51.1 pinned in workspace.dependencies; MSRV 1.94.1 floor established (impacts build environment).
