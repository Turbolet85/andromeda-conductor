# security extract

## Relevance
relevant

## Constraints
- All `rusqlite` writes to `runs.db` MUST use bound parameters exclusively; never string-formatted or `format!`-concatenated SQL (security-plan.md §Input Validation, table row 7; §Anti-Patterns § Input).
- Fingerprints MUST be stored as JSON1 TEXT arrays (not delimited TEXT or parallel columns) to enable cost-effective NULL-exclusion queries in cross-run aggregations (security-plan.md §Input Validation, table row 7; scope.md fixed column contract).
- Blocked rows MUST store identity fields (`run_id`, `seed`, `scenario`, `p_ids`, `slo_tier`) only; `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `fingerprints` MUST be NULL—never phantom values (security-plan.md §Threat Model Summary, Data classification; scope.md Blocked-row NULL rule).
- Run-report artifacts (`runs.db` rows) MUST NOT leak absolute host paths (canonicalized `CONDUCTOR_RUNS_DIR`, `ANDROMEDA_PULSE_DATA_DIR`) or internal seam-crate struct/field names (security-plan.md §Error Handling, run-report artifact sanitization).
- Timestamp columns (`journal_emitted_at`, `read_back_observed_at`) MUST derive from `std::time` (not tokio virtual clock) to keep journal-relative SLO math correct (security-plan.md §Anti-Patterns § Logging, universal rules).
- Write failures to `runs.db` MUST surface as typed `thiserror` errors (not panics), routed through the verdict/error wall as harness faults (`Result::Err`), never reinterpreting the SUT verdict (scope.md Verdict/error wall contract).

## Patterns to follow
- Use rusqlite 0.38.0 parameterized statements with `?` placeholders for all `run_id`/`seed`/fingerprint-JSON values; verify via grep that no `format!` or string concatenation appears in SQL construction (security-plan.md §Input Validation, table row 7; Anti-Patterns § Input).
- Schema MUST use `CREATE TABLE IF NOT EXISTS` (no migration framework per scope.md) with fixed column types set on first write (security-plan.md Data model conventions; scope.md fixed column contract).
- NULL-excludable measurement columns enable cheap cardinality filtering for cross-run latency/percentile queries downstream (scope.md Boundaries implicit contract; security-plan.md §Error Handling).

## Anti-patterns to avoid
- NEVER use string concatenation, `format!`, or any non-parameterized SQL for writes, even for synthetic self-generated data (security-plan.md §Anti-Patterns § Input).
- NEVER store `p_ids` or fingerprints as delimited TEXT or separate TEXT columns when JSON1 arrays are specified—the schema shape must support O(1) NULL-exclusion filtering (security-plan.md §Input Validation; scope.md fixed column contract).
- NEVER emit absolute paths or seam-internal struct/field names into `runs.db` rows or journal; the db is agent-parseable ground truth shared across hosts (security-plan.md §Anti-Patterns § Logging).

## Contract bindings
- Blocked-row NULL rule ↔ verdict/error wall: harness faults (e.g. write-failed) MUST not corrupt the SUT verdict state; a failed write surfaces as `Result::Err`, not a `Blocked` row (scope.md Verdict/error wall; security-plan.md §Error Handling).
- Run-record envelope contract ↔ obs/tests: `RunRecord` shape + fingerprint JSON1 format is ground truth for P-036 recurrence cross-run query and test round-trip assertions (scope.md reuses RunRecord verbatim; security-plan.md §Logging & Monitoring implicitly; obs-plan § PII Scrubbing read-back).

## Acceptance criteria contributions
- (security) All `INSERT`/`UPDATE` statements in the `RunsDb` storage type contain parameterized placeholders (`?`) and no `format!` / string concatenation; `grep -E 'format!|[+\s]+".*\$|concat'` over the `conductor-report` seam returns zero matches on SQL construction paths.
- (security) Blocked rows store only identity fields (`run_id`, `seed`, `scenario`, `p_ids`, `slo_tier`) with measurement columns (`verdict`, timestamps, `latency_ms`, `fingerprints`) as SQL `NULL`; unit/integration tests assert NULL cardinality via `SELECT COUNT(*) WHERE verdict IS NULL`.
- (security) No absolute paths or struct names appear in inserted row values; artifact-hygiene tests read back a row and confirm identity/measurement fields contain only synthetic data envelopes, never `CONDUCTOR_*` or `ANDROMEDA_*` resolved paths.
- (security) Write failures surface as typed `conductor_report::StorageError` (thiserror), routed via `Result::Err` in the caller, never panicking or reinterpreting verdicts.

## Relevant amendment history
- **2026-06-15-config-validation-surface** — garde pinned 0.23.0 → 0.22.1. Not directly relevant (input-validation library, not storage), but notes the version-floor discipline: pinned floors + actuals, not over-precise locks for ecosystem tools.
- **2026-06-15-dependency-audit-gate** — cargo-audit/cargo-deny floor versions confirmed; toolchain bump done. Relevant: rusqlite 0.38.0 is already pinned + installed (unused until this chunk); no further audit/deny updates required for this storage seam itself, but the general floor discipline (audit fresh on each build; `Cargo.lock` committed) applies.
