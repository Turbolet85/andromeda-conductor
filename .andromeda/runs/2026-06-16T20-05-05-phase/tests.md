# tests extract

## Relevance
Partial — scenario-config model builds the bridge between declarative config (serde/garde) and the timeline scheduler; tests must validate config parsing, garde validation, and the conversion to PhaseTimeline.

## Constraints
- Per test-plan §1: scenario-config validation surface is testable via garde `range` + `#[garde(custom)]` cross-field rules with valid/invalid fixtures; config-parse + garde-validation failures surface as `ConfigError`/`CoreError` (harness `Result::Err`), never a verdict (per test-plan §3 Verdict/error wall).
- Per test-plan §4: garde validation of per-phase fields (non-negative durations, sane ordering/bounds, cross-field invariants) lives as unit tests with valid/invalid config-fixture matrices; same-seed conversion ⇒ identical stream shape (per test-plan §7 determinism requirement).
- Per test-plan §2: fixtures are rstest-driven table-driven `#[rstest]` + `#[case]` over the garde-config matrix and P-001..P-060 catalog (per test-plan §7 Fixture library).
- Per test-plan §11 Unit anti-pattern: NEVER assert on implementation details — test public seam API + observable behavior (envelope/verdict/state output); brittle fixture assertions banned.
- Per test-plan §11 Test Data anti-pattern: NEVER use non-deterministic generators without seed control — `conductor-timeline` seed + proptest strategies with `proptest-regressions/` persistence.
- Per test-plan §3 Test data bootstrap: self-bootstrapping via deterministic seeded synthetic generation; config fixtures are declarative scenario files (serde + garde) one-per-P-ID under `scenarios/`.
- Per test-plan §1: determinism as a hard quality bar — same scenario + seed ⇒ same `PhaseTimeline` shape; the config→timeline conversion is order-preserving and total, never panics.

## Patterns to follow
- Per test-plan §4 Unit Test Strategy: per-seam crate-local `#[cfg(test)]` modules for unit tests; snake_case function naming under `#[test]` / `#[tokio::test]` / rstest `#[rstest]`.
- Per test-plan §7 Test Data & Fixtures: rstest 0.26.1 `#[fixture]` + `#[rstest]` + `#[case]` for table-driven P-ID and garde-config matrices; `#[once]` fixture for in-memory `runs.db` schema setup (per-test-isolation pattern).
- Per test-plan §1 Coverage triggers § property-test: property/golden test asserting fixed scenario+seed reproduces identical emission-journal stream shape across runs (the deterministic-replay invariant on `conductor-timeline`); Epoch 2 chunk 4 owns the full golden/proptest sweep, but basic determinism asserts here are fine.
- Per test-plan §5 Integration Test Strategy: the `Scenario → PhaseTimeline` conversion is a module ↔ module boundary — direct function calls with shared rstest fixtures across `conductor-<seam>` crates (cargo-nextest 0.9.137).

## Anti-patterns to avoid
- Per test-plan §11 Unit: NEVER assert on real wall-clock duration in a `conductor-timeline` test — drive `tokio::time::advance` under `start_paused = true` and assert scheduled ordering/shape, not real elapsed time.
- Per test-plan §11 Integration: NEVER build `runs.db` SQL via `format!`/string concatenation — use rusqlite bound parameters even for synthetic data (security anti-pattern); a negative/static test asserts this.
- Per test-plan §11 Test Data: NEVER stamp the emission journal/report from tokio's virtual clock — use `std::time::SystemTime`/`Instant`; a paused-clock leak into the ground-truth artifact corrupts SLO checks (security anti-pattern) and must be caught by a golden test.

## Contract bindings
tests ↔ obs: the Run-report envelope JSON shape (test-plan §3 Status endpoint shape + Log format) is shared by JSONL journal + `runs.db` row + Markdown report; the log-format binding (§3 Log format) is the per-run emission-journal schema — this plan defines it as the source of truth and obs derives its product-side log envelope FROM this subsection.

## Acceptance criteria contributions
- "(tests) `cargo nextest run -p conductor-core` passes for new tests" (and per-seam unit layer via `cargo nextest run -p conductor-<seam>` per test-plan §4).
- "(tests) Coverage: new-code line coverage ≥ 60%" per test-plan §10 Quality Gates (Minimal tier threshold).
- "(tests) Scenario-config fixture loads, garde-validates (bounds + cross-field rules), and converts deterministically into a `PhaseTimeline` with invalid configs rejected as typed `Err` (harness fault, not a verdict)" (scope.md Definition of done).
- "(tests) Fixtures use rstest factory functions (§7 Fixture library), no raw object literals; per-test isolation via in-memory `runs.db` per test (parallel-safe under nextest's per-process model)."

## Relevant amendment history
- **2026-06-16-test-framework-fixtures-coverage-tooling** (§4 Unit Test Strategy): external-CLI tool versions (cargo-nextest, cargo-llvm-cov) reframed as reference floors (outside `Cargo.lock`; any green-running install satisfies the gate) rather than exact pins; crate dev-deps are caret-resolved with `Cargo.lock` authoritative. Reason: chunk resolved dev-deps (nextest 0.9.133, cargo-llvm-cov 0.8.5, proptest 1.11.0, insta 1.48.0, etc.) differing from §4's named pins while all gates ran green — reframing to floors stops the detector recurring. Pattern applies to the scenario-config-model chunk's dev-dep resolutions (rstest, assert_cmd, assert_fs, insta, proptest).
