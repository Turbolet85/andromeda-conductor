# tests extract

## Relevance
Relevant — emission journal is a core test artifact, its writer is explicitly an obs/tests cross-domain responsibility per test-plan §3, and this chunk owns the JSONL schema + wall-clock stamping.

## Constraints
- The journal-entry type schema is owned jointly by obs/tests specialists (test-plan §3 Log format) per test-plan §3 and obs-plan §Log format binding (per test-plan §39 cross-domain bindings).
- Wall-clock stamping via `std::time::SystemTime`/`Instant` only, never tokio's virtual clock (test-plan §3 §logs), a security anti-pattern enforcement per test-plan §6 (artifact-sanitization discipline).
- Journal I/O failure is a harness fault (`Result::Err`), never a verdict (scope validation-1).
- Artifact paths are run-id-stemmed, never overwritten (scope Boundaries + test-plan §3 Test data bootstrap).
- No absolute host paths or internal struct names leak into journal lines (test-plan §3 Log format required-fields + test-plan §96 artifact-sanitization discipline). Enforced by negative test per test-plan §6.
- JSONL format is structured—one JSON object per line, parseable with `jq -c` or `serde_json::from_str` (test-plan §3 Log format).

## Patterns to follow
- rstest seeded fixtures per test-plan §4 / §5 — deterministic seed ⇒ same stream shape (property-test target for determinism replay per test-plan §7 coverage-trigger).
- Per-run journal written during scenario runs; per-test isolation via `assert_fs::TempDir` with `.env("CONDUCTOR_RUNS_DIR", temp.path())` (test-plan §3 Test data bootstrap).
- Golden test assertions on envelope/journal shape via insta (test-plan §4 conductor-report bullet + test-plan §6 Artifacts).
- Negative test asserting no host paths / struct names leak (test-plan §96 artifact-sanitization discipline + test-plan §6 Vector 2 required test type).

## Anti-patterns to avoid
- String concatenation / `format!` to build paths (no SQL, but file-path hygiene applies) — use `std::path::Path` / `PathBuf` canonicalization per test-plan §6 Vector 1.
- Writing virtual-clock timestamps from tokio into the journal (test-plan §3 §logs security anti-pattern).
- Overwriting or truncating existing journal files — append-mostly only per test-plan §3 Test data bootstrap + scope Boundaries.

## Contract bindings
- obs ↔ tests (journal-entry type schema): obs-plan §Log format + test-plan §3 Log format §189 binding note — the `tracing-subscriber` JSON schema is defined in test-plan §3 and obs derives from it, not the reverse.
- tests harness ↔ emission-journal-writer: the journal is "the left side of every SLO check" (test-plan §3 §logs ground truth) and the artifact that the agent parses via `jq -e` / `serde_json::from_str` per test-plan §3 §status command body.
- timeline engine ↔ emission-journal-writer: wiring so the timeline (chunks 1–2) records a journal entry per phase transition / emission event (scope Boundaries).

## Acceptance criteria contributions
- (tests) Journal-entry type schema matches obs-plan §Log format (no host paths, no struct names, required identity fields `run_id`/`seed`/`scenario`/`p_ids`/`verdict`/`state`/`latency_ms`/`slo_tier`/`fingerprints` per test-plan §3).
- (tests) `std::time` wall-clock stamps only; `cargo nextest run -p conductor-report` passes golden insta assertions on journal line shape (per test-plan §3 / §4).
- (tests) Negative test: journal lines stripped of absolute paths / internal struct names per test-plan §96 + Vector 2 required test type.
- (tests) Determinism property test: same scenario + seed ⇒ identical emission-journal stream shape across runs (test-plan §7 property-test coverage trigger).

## Relevant amendment history
- 2026-06-15-structured-logging-stack: clarified that `tracing` self-observation stream (per obs-plan §3) is SEPARATE from the per-run emission journal (`runs/<run_id>.jsonl`); the two schemas must not be conflated. No envelope change; binding stands.
- 2026-06-16-test-framework-fixtures-coverage-tooling: external-CLI tool versions (cargo-nextest 0.9.137 / cargo-llvm-cov 0.8.7) reframed as floors, not exact pins; any green install satisfies gates. Affects test invocation baseline but not journal/artifact contract. Coverage tooling readiness: `cargo llvm-cov nextest --lcov` green per §10 Quality Gates.