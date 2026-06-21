# tests extract

## Relevance
Partial — the chunk adds a pure fault-helper descriptor to `conductor-faults`; most test strategy applies (unit + doctests); E2E fixture/scenario-wiring is deferred to Epoch 7.

## Constraints
- **Unit-level isolation:** test the `BurstyTrain` public descriptor API + observable phase-query behavior in isolation via `cargo nextest run -p conductor-faults` (per test-plan §4, per-seam crate-local tests).
- **Verdict/error wall:** validate that out-of-range duty-cycle bounds are typed `FaultError` values, never panics (per test-plan §1, error-handling surface testability).
- **Determinism invariant:** same inputs ⇒ same `BurstyTrain` descriptor output (no randomness in the descriptor itself; seeded jitter is Epoch 7's timeline concern) — golden-testable via exact-string `assert_eq!` on the duty-cycle accessors (per test-plan §4, conductor-report serialization-golden pattern).
- **No new production dependencies:** `Cargo.lock` remains un-drifted; only dev-deps (rstest, etc.) may be used if new (per test-plan §10, supply-chain audit binding).
- **Coverage gates green:** `cargo nextest run -p conductor-faults` must pass; `cargo llvm-cov` line coverage ≥ 60% (Minimal tier); `cargo clippy -D warnings`; `cargo test --doc` for inline doctest examples (per test-plan §4, §10).

## Patterns to follow
- **Module naming:** follow the `EmissionGap`/`AbruptSilence` module patterns in `conductor-faults` — concept-named `train.rs` exporting the type, with crate-doc "Shipped so far" line extended to 4/4 (per test-plan §2, §4, test directory conventions).
- **Fault-helper unit structure:** rstest table-driven `#[case]` rows over valid duty-cycle bounds + a valid/invalid bounds matrix mirroring the garde/scenario-config pattern (per test-plan §7, fixture-library precedent).
- **Both-directions determinism:** assert same-inputs ⇒ identical descriptor AND distinct-inputs ⇒ distinct descriptors (the `EmissionGap` `distinct_inputs_yield_distinct_gaps` precedent), per test-plan §4 + Session-Additions determinism rule.

## Anti-patterns to avoid
- **Never panic on invalid bounds:** out-of-range active/quiet durations must surface as typed `FaultError` enum variants, not unwraps/panics (per test-plan §11, Unit section + §1, error-handling surface).
- **Never skip doctest examples:** any public `BurstyTrain` method with a docstring must carry an inline example (per test-plan §4, doctest runnable via `cargo test --doc`).

## Contract bindings
**obs ↔ tests:** a future `fault.*` bursty-train tracing span (Epoch 7/8 wiring) will consume the `BurstyTrain` descriptor's `*_ms` accessors; this chunk's exact-assert on those values pre-stabilizes them for that span's attributes (binding tightened at Epoch 7 when the timeline injects jitter + the obs span is wired). No binding at this epoch.

## Acceptance criteria contributions
- **(tests)** `cargo nextest run -p conductor-faults` passes — unit tests cover valid/invalid duty-cycle cases + phase-query assertions (per test-plan §4, Unit Test Strategy).
- **(tests)** Coverage: new `train.rs` keeps `conductor-faults` line coverage ≥ 60% (per test-plan §10, Quality Gates, Minimal-tier `--fail-under-lines 60`).
- **(tests)** Doctest: `cargo test --doc` passes — public `BurstyTrain` docstring examples execute and assert (per test-plan §4, doctest fallback for nextest skip).
- **(tests)** `cargo clippy -D warnings` green — no warnings on the new module (per test-plan §9, CI Integration, Lint stage failure condition).

## Relevant amendment history
**2026-06-16-test-framework-fixtures-coverage-tooling** — external-CLI tool versions (cargo-nextest, cargo-llvm-cov) reframed as reference floors (not exact pins). This chunk's gates execute `cargo nextest run -p conductor-faults` (floor satisfied by any green-running install).