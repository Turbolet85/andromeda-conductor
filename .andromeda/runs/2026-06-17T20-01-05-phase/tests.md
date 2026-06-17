# tests extract

## Relevance
relevant — this chunk is a core Minimal-tier test infrastructure piece (determinism-replay harness), directly enabling the Foundation's determinism hard bar.

## Constraints
- Per test-plan §1, determinism is a non-negotiable quality gate: "Determinism as a hard quality bar: same scenario + seed ⇒ same stream shape" (security Vector 0 / Creator Brief Risk Tolerance).
- Per §2 Test Strategy, all tests must run under `#[tokio::test(flavor = "current_thread", start_paused = true)]` + `tokio::time::advance` — never real-time waits, never flaky.
- Per §4 Unit Test Strategy, `conductor-timeline` determinism is "unit-assertable (same seed ⇒ same stream shape per Creator Brief)"; this chunk's golden + proptest generalize unit coverage to the full pipeline.
- Per §3 Test Harness Contract, the emission journal (`runs/<run_id>.jsonl`) carries wall-clock stamps from `std::time::SystemTime`/`Instant` (NOT virtual clock) — the golden captures *only* the seeded/virtual dimension (`Vec<PhaseTransition>`), keeping the wall-clock-vs-virtual-clock boundary intact.
- Per scope §Intent anchor (validation-1), the golden must be an insta snapshot in assert/CI mode (fail, never auto-write), committed under `crates/conductor-timeline/tests/snapshots/`.
- Per scope §Boundaries, proptest regressions must be committed to `proptest-regressions/` for deterministic replay of discovered counterexamples.

## Patterns to follow
- Per §3 bootstrap phases, dev-deps already include `insta 1.x` and `proptest 1.9.0` (test-plan-amendments §2026-06-16 establishes caret-resolved dev-deps with `Cargo.lock` authoritative).
- Per §4 fixture pattern, use `#[tokio::test(flavor = "current_thread", start_paused = true)]` + `tokio::time::advance` for determinism tests (no real-time waits).
- Per §2, fixtures use rstest `#[fixture]` for seeded generator + `#[once]` for in-memory setup; golden tests via insta's `assert_debug_snapshot!` (per scope §Boundaries, `PhaseTransition: Debug` suffices — no new `Serialize` derive required unless YAML/JSON snapshot clarity demands it).

## Anti-patterns to avoid
- Per §11 Test Anti-Patterns (inferred from §4), do NOT snapshot the JSONL emission journal or its `journal_emitted_at` stamps — those are wall-clock, non-reproducible, and would flake. The golden captures *only* the seeded/virtual `Vec<PhaseTransition>` dimension.
- Per §3 Log format + §1 Untestable zones, do NOT assert real duration within a test — the virtual-clock boundary must remain clean (a paused-clock leak would be caught by the snapshot drift).
- Per scope §Boundaries, do NOT add new production logic to the scheduler/conversion — `run_timeline`, `Phase`, `PhaseTimeline`, `PhaseTransition` already exist; the test is purely assertional.

## Contract bindings
- tests §3 ↔ obs §3: Log format binds the emission journal schema (per-run `<run_id>.jsonl` carries the Run-report envelope fields — `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `journal_emitted_at`, `read_back_observed_at`, `fingerprints`); this chunk's golden *excludes* the wall-clock stamps, keeping the self-obs stream (separate per amendment §2026-06-15-structured-logging-stack) distinct.

## Acceptance criteria contributions
- "(tests) `cargo nextest run -p conductor-timeline` passes; determinism golden snapshot (insta, assert mode) committed under `crates/conductor-timeline/tests/snapshots/` for the `error-baseline-spike` scenario at a pinned seed, capturing the exact `Vec<PhaseTransition>` shape." — per scope §Intent anchor.
- "(tests) Proptest property suite generalizing replay determinism (`∀ seed: run_timeline(tl, seed)` twice ⇒ identical shape) + structural invariants (gap bounds, monotonic `elapsed_ms`) with committed `proptest-regressions/` for counterexample replay." — per scope §Intent anchor + test-plan §1 property-test coverage trigger.
- "(tests) Zero nextest retries; all determinism tests run under `start_paused`, capturing only seeded/virtual dimension (never real-clock waits)." — per §2 agent-runnable invariants.
- "(tests) `coverage-tooling` gate via `cargo llvm-cov nextest --lcov ... --fail-under-lines` ≥ {threshold from §10} over new `conductor-timeline` determinism test paths." — per §10 Quality Gates (Minimal-tier threshold ~70%).

## Relevant amendment history
- **2026-06-16-test-framework-fixtures-coverage-tooling:** External CLI tool versions (cargo-nextest, cargo-llvm-cov) reframed as reference floors, not exact pins — any green-running install satisfies the gate (per cargo-audit/deny precedent, with `Cargo.lock` authoritative for crate dev-deps). Cascaded to `.claude/docs/{stack,tests-summary}.md`. — *Relevance:* This chunk's golden + proptest use cargo-nextest + cargo-llvm-cov; floor semantics ensure the chunk's CI gate runs green regardless of resolved tool micro-versions.
- **2026-06-16-emission-journal-writer:** Unit serialization goldens use exact-string `assert_eq!`; insta reserved for E2E journal goldens with redaction. — *Relevance:* This chunk's snapshot must use insta (E2E pipeline context); the amendment clarifies that unit-level serialization (e.g., `PhaseTransition` Debug string) can co-exist with insta's E2E role without duplication.
