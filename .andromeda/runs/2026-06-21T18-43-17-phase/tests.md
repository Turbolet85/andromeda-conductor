# tests extract

## Relevance
relevant — the chunk's runs.db schema write seam and bound-parameter discipline touch core test contracts (unit/integration test data bootstrap, persistence round-trip, security constraint).

## Constraints
- **Bound-parameter discipline exclusively** (per test-plan.md §3 + security bindings): all `runs.db` writes via rusqlite bound parameters; never `format!`-built SQL (§3 Log format, §1 Coverage triggers anti-pattern list, §8 Mocking & Stubbing; security plan Vector 5).
- **NULL-row semantics for blocked checks** (per test-plan.md §1 Blocked-row NULL rule + §3 Status endpoint shape): a `Blocked` state row stores only `run_id`/`seed`/`scenario`/`p_ids`/`slo_tier`; `verdict`, `latency_ms`, `journal_emitted_at`, `read_back_observed_at`, `fingerprints` are NULL — never phantom values (§1 arch Standard Contracts).
- **Timestamp representation fidelity** (per test-plan.md §3 + scope.md open questions): `journal_emitted_at` / `read_back_observed_at` stored per arch Data-model conventions (integer-millisecond journal offsets, not ISO strings); stamps derive from `std::time`, never tokio virtual clock (security anti-pattern, §3 logs).
- **JSON1 fingerprints for P-036 query readiness** (per test-plan.md §1 + scope.md): fingerprints stored as JSON1 TEXT array (not delimited); schema design supports future cross-run `JSON1` percentile/recurrence queries (scope.md P-036 language).
- **Determinism / synthetic data only** (per test-plan.md §3 bootstrap + §2 agent-runnable invariants): test `runs.db` written via in-memory `rusqlite::Connection` (unit/golden) or `assert_fs::TempDir`-backed file DB (integration); seeded synthetic envelope generation via `conductor-timeline` fixture (rstest `#[fixture]`); no developer-seeded rows.
- **Artifact hygiene** (per test-plan.md §1 artifacts + scope.md open questions): no absolute host paths or internal struct names leak into `runs.db` content; the db path itself canonicalized at `conductor-cli` edge.
- **Schema as immutable write contract** (per test-plan.md §3 Status endpoint shape + scope.md fixed column contract): one row per `(run_id, scenario)` check (no synthetic UUID); column types fixed on first write; insert grain / conflict policy (UNIQUE / REPLACE semantics) supports append-mostly run history without re-runs silently clobbering prior results.

## Patterns to follow
- **Unit/golden-test round-trip pattern** (per test-plan.md §4 conductor-report bullet + 2026-06-16 amendment): envelope serialization locked via exact-string `assert_eq!` at unit level; integration tests use `open_in_memory()` / `assert_fs::TempDir` to prove `runs.db` round-trip of a `RunRecord` envelope without mutating test state between runs.
- **rstest `#[fixture]` + `#[rstest]` table-driven over P-ID catalog** (per test-plan.md §4 Fixture pattern): seeded `conductor-timeline` generator passed to per-check envelope construction, then written to test DB; `#[case]` rows parameterize over scenario names / seeds.
- **Assertion via bound-parameter `SELECT`** (per test-plan.md §3 `status` command): integration/E2E tests read back `runs.db` rows via `rusqlite::Connection::execute / query` with `?1` placeholders (language-native, no jq); in CI, `status` command body uses `jq -e` for shell-level smoke gates on the JSONL journal (runs.db read is test-internal).
- **Security negative-test on bound parameters** (per test-plan.md §1 coverage-triggers Vector 5 / §8): explicit negative-test case asserting string-concatenated SQL is rejected; path-traversal `CONDUCTOR_RUNS_DIR` is canonicalized before DB path is constructed.

## Anti-patterns to avoid
- **String-concatenated or `format!`-built SQL** (per test-plan.md §1 coverage-triggers + §8 antis): no `format!("INSERT INTO runs (run_id) VALUES ('{}')", id)` — all writes bound; caught by security vector negative-test.
- **Phantom/default values for blocked rows** (per test-plan.md §1 arch Standard Contracts): never populate `latency_ms` with 0, `verdict` with a default, or `journal_emitted_at` with epoch — NULL is the contract; NULL-excludable columns (latency/timestamps/fingerprints) enable future aggregation queries to skip measurement-less rows cheaply.
- **Tokio virtual-clock stamps in persisted state** (per test-plan.md §3 security anti-pattern): all timestamps derive from `std::time::SystemTime` or `Instant` (wall-clock), never tokio's `tokio::time::now()` or test-paused virtual clock — the emission journal is the ground truth.

## Contract bindings
- **obs §Log format + §3 Self-obs stream** ↔ **tests §3 Log format & Status endpoint shape**: the per-run emission journal (`runs/<run_id>.jsonl`) is the SLO ground truth; `runs.db` is a convenience index of the envelope; both carry identical envelope fields (`run_id`, `verdict`, `state`, `latency_ms`, etc.) — the two persist the same semantic shape, not separate schemas. Self-obs stream (service-identity fields, per obs-plan §3) is SEPARATE and flows to `logs/agent-latest.jsonl`; test-plan §3 amended 2026-06-15 to clarify this boundary.
- **security plan §Input Validation + Vector 5** ↔ **tests §1 coverage-triggers + §8**: bound-parameter discipline enforced by negative-test asserting injection rejection; no string-built SQL surfaces in the prod or test codebase.

## Acceptance criteria contributions
- "(tests) Unit: `conductor-report` round-trip golden-locked via exact-string `assert_eq!` on serialized `RunRecord` envelope (verdict.rs/report_state.rs/scenario.rs pattern) — all 7 verdict/state/slo_tier enum variants tested."
- "(tests) Integration: `runs.db` write via bound-parameter `INSERT` + NULL-row contract — in-memory `Connection` write + `SELECT` read-back of a `(run_id, scenario)` key proves envelope preservation; NULL-measurement columns confirmed NULL for a blocked row."
- "(tests) Security: negative-test asserting string-concatenated SQL `format!(\"-built INSERT /{} VALUES ...\", unsafe_id)` is rejected; `CONDUCTOR_RUNS_DIR` canonicalization tested before DB path construction."
- "(tests) Integration: fingerprints stored as JSON1 TEXT array (not delimited); a schema-forward test confirms JSON1 `json_array_length()` predicate works on the fingerprints column (prep for P-036 cross-run queries)."

## Relevant amendment history
- **2026-06-16-emission-journal-writer** (§4 conductor-report bullet): unit serialization goldens use exact-string `assert_eq!` (not insta); insta reserved for E2E journal goldens with redaction (§6/§7) — this chunk follows the exact-string pattern at unit level for envelope shape validation.
- **2026-06-17-raw-otlp-message-scaffold** (§2 Integration row): loopback gRPC stub (tokio-stream) registered as an integration test mechanism; while this chunk focuses on `runs.db` write, the integration boundary includes OTLP egress mocking + DB persistence round-trip together (same cross-boundary test altitude as MCP stub + Tauri mock).
