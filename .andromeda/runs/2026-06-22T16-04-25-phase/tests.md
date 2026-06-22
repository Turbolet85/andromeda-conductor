# tests extract

## Relevance
Partial — the chunk formalizes scenario configs + fixtures; it does NOT execute live verification runs, so only the config-validation + unit-golden-fixture legs apply; E2E/integration run verification is deferred to Epoch 8 CLI driver.

## Constraints
- Per test-plan.md §1 **Test Scope Summary** — entity `Scenario-config validation surface` is testable via unit-level garde validation (error fraction ∈ [0,1], non-negative durations, p50≤p95≤p99 cross-field invariants); all four P-IDs carry `class = "Hard"` (per scope.md § Definition of done).
- Per test-plan.md §4 **Unit Test Strategy** — scenario TOML fixtures deserialized via `Scenario::from_toml_str` + unit-golden envelope shape via exact-string `assert_eq!` (conductor-report canonical line), NOT insta (insta reserved for E2E with redaction).
- Per test-plan.md §4 **Fixture pattern** — rstest 0.26.1 table-driven `#[case]` rows over the P-009..P-012 catalog matrix (deserialize + garde-validate + `PhaseTimeline` round-trip), mirroring ch1/ch2 + existing `error-baseline-spike` fixture test.
- Per test-plan.md §1 **Coverage triggers** — property-test (proptest 1.9.0) over `#[garde(custom)]` cross-field invariants (baseline-match tolerance, persistence-window timing, p50≤p95≤p99 ordering), marked "included via trigger".
- Per test-plan.md §2 **Agent-runnable invariants** — determinism: `#[tokio::test(flavor = "current_thread", start_paused = true)]` + `tokio::time::advance` for phase-timeline tests; proptest counterexamples persisted to `proptest-regressions/`.
- Per test-plan.md §4 **What unit tests cover** — Scenario-config validation surface (garde validation), conductor-core `Scenario`/`ExpectedCheck` deserialization + model round-trip (no live emit, no execution).

## Patterns to follow
- Per test-plan.md §4 — rstest `#[fixture]` for seeded `conductor-timeline` generator; `#[rstest]` + `#[case]` rows enumerate valid P-009/P-010 + P-011/P-012 configs + invalid garde-rejection fixtures (error-fraction >1, negative durations, p50>p95 ordering violation, severity-mix sum violation).
- Per test-plan.md §4 — crate-local `#[cfg(test)] mod tests` in `conductor-core` (Scenario deserialization + phase-timeline round-trip); canonical-serialization goldens use exact-string `assert_eq!`.
- Per scope.md § Definition of done — fixture round-trip tests prove each TOML deserializes + garde-validates + builds valid `PhaseTimeline` through existing scheduler (mirrors ch1/ch2 + existing `error-baseline-spike` fixture test precedent).

## Anti-patterns to avoid
- Per test-plan.md §1 **Coverage triggers** (security Vector 2) — do NOT deserialize scenario config without garde validation at load (rejected: out-of-range error-fraction, negative durations, p50>p95>p99 ordering violations, severity-mix sum violations).
- Per test-plan.md §4 — do NOT use insta for unit-level serialization goldens (insta reserved for E2E with redaction; unit carries exact-string `assert_eq!`).
- Per scope.md § Open questions — do NOT add model changes to `Scenario`/`ExpectedCheck`/`Phase` (zero-model-change goal); declare tolerance-band intent in TOML only; defer ±10%/±15% math + persistence-window timing to Epoch-8 evaluator.

## Contract bindings
- **tests ↔ obs:** Per test-plan.md §3 Log format (amended 2026-06-15) — the per-run emission journal (`runs/<run_id>.jsonl`, SLO ground truth + Run-report envelope) is SEPARATE from the self-observation stream (`logs/agent-latest.jsonl`); two schemas must not conflate; this chunk owns the envelope schema for the emission journal (unit golden asserts shape); obs derives from it.
- **tests ↔ arch:** Per scope.md § Requirement source of truth — baseline math ("baseline matches injection within ±10%/±15%") and suppression/bypass logic are hard pass/fail per arch Probabilistic-Assertion Policy; model-interpretive severity that consumes cues is P-020 (later chunk, out of scope here).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core` passes for P-009/P-010 + P-011/P-012 scenario deserialization + garde-validation `#[case]` rows (valid + invalid matrix).
- (tests) Unit serialization golden: Run-report envelope shape locked via exact-string `assert_eq!` for all four P-IDs' expected checks (class=Hard, deterministic baseline-match + detection-candidate assertions).
- (tests) Fixture round-trip: both `scenarios/error-baseline-spike.toml` (formalized) + new `scenarios/latency-regression.toml` deserialize + garde-validate + produce valid `PhaseTimeline` via existing scheduler (property-tested under `start_paused` seed determinism).
- (tests) Zero-model-change: all four checks (P-009 baseline match ±10%, P-010 spike candidate, P-011 latency baseline ±15%, P-012 regression candidate) expressible via existing `ComparisonKind`/`ClaimClass`/`Phase` — no `Scenario`/`ExpectedCheck` extension.

## Relevant amendment history
- **2026-06-15-emission-journal-writer** — unit serialization goldens use exact-string `assert_eq!`; insta reserved for E2E mechanism (this chunk's conductor-report unit goldens for P-009..P-012 envelope shape follow exact-assert pattern, not insta).
- **2026-06-16-test-framework-fixtures-coverage-tooling** — external-CLI tool versions reframed as floors (cargo-nextest 0.9.137 / cargo-llvm-cov 0.8.7 reference floors; Cargo.lock authoritative for dev-deps incl. proptest 1.9.0); this chunk's proptest property tests conform to floor baseline.
