# tests extract

## Relevance
Partial — the chunk builds a fault helper (emission-gap/resume structure) that is a **testable pure function** (unit level) but the **driven scenario verification** and **MCP read-back** belong to later epochs.

## Constraints
1. **Determinism bar (per test-plan §1 Scope Summary):** The exact gap duration must be deterministically reproducible across runs — same scenario+seed ⇒ identical gap length; tested as a seeded property (test-plan §7 property-test triggers).
2. **Typed error wall (per test-plan §1 Scope Summary + test-plan §4 Unit Test Strategy, conductor-faults bullet):** Invalid config (gap ≤ 20s floor or malformed ordering) surfaces as `FaultError` VALUE, never panic — no unguarded seeded generation allowed; bounds-checking is precondition-enforced.
3. **Pure-function unit testing (per test-plan §2 Test Strategy, Unit level):** The gap/resume helper is a seeded generator (like ramp/silence/fingerprint); unit tests assert output shape in isolation under `#[tokio::test(flavor = "current_thread", start_paused = true)]` (no live timeline wiring in this chunk).
4. **Fixture pattern — rstest seeded (per test-plan §3 Test Data Bootstrap, §4 Unit Test Strategy):** Tests use `#[fixture]` producing seeded generators; table-driven `#[rstest]` + `#[case]` rows for the valid gap matrix + invalid bounds rejection cases; exact-string `assert_eq!` for canonical output shape.
5. **Nextest command scope (per test-plan §3 5-command implementation, `run` leg):** Tests must pass `cargo nextest run -p conductor-faults` with CI profile JUnit XML output; coverage ≥ threshold per §10.
6. **Anti-pattern: no panic on invalid input (per test-plan §11 Test Anti-Patterns, §9 Mocking & Stubbing, §4 What unit tests do NOT cover):** Typed errors are the verdict/error wall; no panics on out-of-range gap/resume construction.

## Patterns to follow
1. **Seeded-generator pattern (established in conductor-faults ramp/silence/fingerprint):** produce identical output structure for the same seed; table-driven `#[rstest]` cases with fixed seed+input combinations verifying the output shape (test-plan §4 conductor-faults bullet). Exact gap duration is the key invariant per scope.
2. **Golden-locked serialization (established in conductor-report unit goldens, amendment 2026-06-16):** canonical JSON shape serialization asserted via exact-string `assert_eq!` at unit level (not insta, which is E2E); matches `verdict.rs`/`report_state.rs` pattern.
3. **Validation-on-construction (garde pattern for config, test-plan §1 & §5):** bounds and ordering rules expressed as typed `FaultError` variants; unit test the rejection cases (invalid gap ≤ floor, malformed resume) with negative-test cases alongside valid cases (test-plan §5 security-vector-coverage, §4 Scenario-config validation).

## Anti-patterns to avoid
1. **No panic on invalid input:** invalid gap ≤ 20s floor or malformed gap/resume ordering must surface as `FaultError` value, testable via `assert!(result.is_err())`, never as an unwrap panic or debug_assert.
2. **Do not conflate emission journal (runs.db envelope) with seeded generator output shape:** the gap/resume helper is a pure input descriptor, not a run report; its serialization (if any) is part of the fault descriptor schema, not the Run-report envelope (test-plan §3 amendment 2026-06-15).
3. **Do not defer determinism testing to E2E:** the seeded reproduction is a unit invariant; property-test over seed+input combinations (test-plan §5 property-test trigger, §7 determinism discipline) before the driven-under-timeline epoch.

## Contract bindings
**Downstream consumer (later epochs, not built here):** the restart-suppression scenario family (P-015/P-016/P-057) and the timeline/emit gap execution (Epochs 5/7) will consume this helper to drive the ≤2s `RestartEvent` MCP read-back verification — the E2E critical path (test-plan §1 Path: Restart-suppression incl. one bypass case). Obs `fault.silence` span name is deferred to the driven-under-timeline epoch; this chunk's helper must not emit OTLP or make MCP calls.

## Acceptance criteria contributions
1. **(tests) `cargo nextest run -p conductor-faults` passes:** the gap/resume helper unit tests green (table-driven valid cases + invalid bounds/ordering rejection cases).
2. **(tests) Determinism: same scenario+seed ⇒ identical gap duration:** property-test or fixed-seed table cases verify reproducibility (test-plan §5 determinism-replay trigger, §7 property-based level).
3. **(tests) Typed error on invalid input:** invalid gap ≤ 20s floor or malformed ordering asserts `Result::Err(FaultError::...)`, never panics (no `unwrap` in the helper public API).
4. **(tests) Coverage ≥ threshold (per test-plan §10):** new helper code (bounds check + gap/resume struct + serialization) covered by nextest LCOV; specific coverage floor deferred to §10 gate check.

## Relevant amendment history
1. **2026-06-16-test-framework-fixtures-coverage-tooling** — reframed external-CLI tool versions (cargo-nextest, cargo-llvm-cov) as reference floors; dev-deps are caret-resolved with `Cargo.lock` authoritative. Applies: this chunk's nextest runs locally and in CI under the floor 0.9.137 (any green-running version ≥ floor satisfies); coverage tooling uses `cargo llvm-cov nextest`.
2. **2026-06-16-emission-journal-writer** — unit serialization goldens use exact-string `assert_eq!` (not insta); insta reserved for E2E mechanisms. Applies: if the gap/resume helper has a canonical JSON descriptor shape, it is golden-locked via exact `assert_eq!` at unit level, matching the `verdict.rs`/`report_state.rs` pattern.
