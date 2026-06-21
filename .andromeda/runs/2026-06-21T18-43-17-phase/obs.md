# obs extract

## Relevance
Relevant — this chunk adds the persistent-storage seam (rusqlite) that materializes the canonical run-report envelope defined in obs-plan, critical for the `db.insert_run` must-trace path and the runs.db storage contract.

## Constraints
- Log format JSON schema is binding contract from tests (per obs-plan §6): the `RunRecord` envelope must carry exactly the 11 fields including `journal_emitted_at`, `read_back_observed_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`, plus scenario-specific fields (per obs-plan §4 must-trace paths).
- Blocked-row NULL rule (arch Standard Contracts, cited in scope.md): a `Blocked` row populates only `run_id`, `seed`, `scenario`, `p_ids`, `slo_tier`; `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `fingerprints` are stored NULL — never phantom values; schema must make NULL-measurement-row exclusion cheap for latency/percentile queries.
- Bound parameters only (security §Input Validation, cited in scope.md): use rusqlite bound parameters exclusively, never `format!` or string-concatenated SQL, even for synthetic data.
- No absolute host paths or internal struct names in runs.db (artifact hygiene, CLAUDE.md universal invariant, §11 Anti-Patterns / Logs): the redaction layer masks absolute host-file paths → `<redacted>`; internal struct names are kept out by field-name allowlist + `Display`-not-`Debug` at the `anyhow` edge; the allowlisted `target` module path is preserved.
- Determinism: runs.db stamps derive from the journal (whose stamps are `std::time`), never tokio's virtual clock (§11 Anti-Patterns / Project-specific bans: "NEVER use tokio's virtual clock for journal timestamps").
- Write failure is a harness fault (Result::Err → typed `conductor-report` error via `thiserror`), never a verification outcome; persisting an envelope never reinterprets the SUT verdict.

## Patterns to follow
- Must-trace span on `db.insert_run` (§4 Critical Path 1 + all 7 scenarios): span attributes must include `row_count` (1), log fields must include the 11-field envelope schema.
- Envelope shape from `conductor_core::RunRecord` with golden-locked serde is the single source of truth; no schema duplication or parallel shape in runs.db.
- Fingerprints stored as JSON1 TEXT array (arch Data-model conventions, scope.md): the JSON1 representation enables the Epoch-7 P-036 recurrence query to index via SQLite JSON1.
- Synchronous off-tokio rusqlite (no pool, no sqlx) per arch §Database; applies to this seam only.

## Anti-patterns to avoid
- NEVER leak absolute host paths or internal struct names in logs / run-report / runs.db — the redaction layer applies before runs.db insert (§11 / Logs).
- NEVER use OTel SDK or metrics backend for self-observation (§11 / Telemetry Strategy, Metrics) — performance budget is JSON field assertion (`latency_ms` + `slo_tier`), not a histogram instrument; runs.db is observation infrastructure, not metrics export.
- NEVER skip the field-allowlist / redaction layer — redaction applies at processor (tracing-subscriber) + report generation + runs.db schema validation on insert (§11 / PII Scrubbing).

## Contract bindings
- Obs ↔ tests harness (per focus-guide): the Run-report envelope schema (11 fields) is binding contract from tests §3 (tests excerpt §5 / §6 journal goldens consume it); runs.db must preserve the envelope fields round-trippable for test assertion.
- Obs ↔ report seam: `db.insert_run` must-trace span (§4 Critical Path 1-7) emits the final verdict + state as log fields at insert time.
- Obs ↔ security: bound parameters + redaction layer (§11 / PII Scrubbing).

## Acceptance criteria contributions
- (obs) DB schema accepts the 11-field envelope (`RunRecord`): `journal_emitted_at` (TEXT ISO-8601 or integer-ms; *open question* in scope.md), `read_back_observed_at` (TEXT or integer-ms), `run_id`, `seed`, `scenario`, `p_ids` (JSON1 array), `verdict`, `state`, `latency_ms` (INTEGER or NULL for blocked), `slo_tier` (TEXT enum), `fingerprints` (JSON1 array or NULL).
- (obs) Blocked-row NULL rule enforced: `Blocked` state populates only `run_id`, `seed`, `scenario`, `p_ids`, `slo_tier`; measurement columns NULL (verdict, journal_emitted_at, read_back_observed_at, latency_ms, fingerprints) — cross-run queries can exclude NULL rows via WHERE clause.
- (obs) Bound-parameter insert: no string concatenation in SQL; all envelope values passed as bound params.
- (obs) Redaction applied on insert: no absolute host paths or internal struct names in any stored field (field allowlist + Display-scrub at source applies before rows reach runs.db).
- (obs) Round-trip proof: minimal read surface (e.g., `get` / `list` by run_id) validates schema round-trip in integration tests if in scope (per scope.md "Read surface in scope?" open question).

## Relevant amendment history
- 2026-06-15-structured-logging-stack (§3 Observability Harness Contract / Log format JSON schema): clarified two record shapes — foundational self-obs base line (`timestamp_ms`, `level`, `target`, service-identity, `run_id`) vs Run-report envelope (scenario-result record with 11 fields including `verdict`, `latency_ms`, `slo_tier`). The envelope is the shape this chunk persists.
- 2026-06-16-emission-journal-writer (§3 Logging stack + §3 Harness Contract + §6 Log Coverage): added `read_back_observed_at` (ISO-8601 from `std::time`; null until read-back, null for blocked rows) as the 2nd field of the JSONL Run-report envelope — schema now 11 fields, not 10. Resolved via realignment to tests-plan owner (Option A); latency_ms = read_back_observed_at − journal_emitted_at. This amendment directly affects the runs.db schema contract this chunk materializes.
