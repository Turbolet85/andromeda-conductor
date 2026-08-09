# tests extract

## Relevance
Relevant — a unit-tier static assertion inside `conductor-core` that the plan already mandates (coverage-matrix vs. SUT capability manifest, zero unclassified entries); no new test surface.

## Constraints
- Tier is `Minimal (0)` and the unit layer carries the bulk — this check belongs in the per-seam unit tier (`cargo nextest run -p conductor-core`), not a new E2E path; per test-plan.md §1 (Test tier) + §2 (pyramid, Unit row).
- The plan already owns this assertion: "every catalog scenario carries a P-ID and the generated `coverage-matrix.md` enumerates every capability in the SUT capability manifest with zero unclassified entries" — this chunk implements the detector for an assertion already specified; per test-plan.md §4 (What unit tests cover — Scenario catalog + coverage-matrix) and §1 (Scenario catalog entity).
- Harness-side failures are `Result::Err`, never envelope verdicts: only a hard `Fail` is a non-zero exit; `Blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` are reported states — a drift must not be laundered into any of them; per test-plan.md §3 (`run` exit-code semantics) + §11 (E2E, stack-specific ban), with the refused-transport-⇒-`Result::Err` precedent in §5 (OTLP egress liveness).
- Failure text and any artifact it reaches must carry ids + manifest metadata only — no absolute host paths, no internal seam-crate struct names; per test-plan.md §3 (Status endpoint shape; Log format — Required fields).
- Zero-flakiness budget: no nextest `retries`, no quarantine-by-retry; the drifted-id list must be deterministically ordered so the message is stable across runs; per test-plan.md §10 (Zero-flakiness budget) + §2 (agent-runnable invariants — Deterministic).
- Coverage gate is binding: workspace line ≥ 60% (`--fail-under-lines 60`), function ≥ 70%; per test-plan.md §10 (Coverage thresholds).
- Test location/naming: crate-local `#[cfg(test)] mod tests` in the seam source file, snake_case `fn`, table-driven variants via rstest `#[case]`; per test-plan.md §2 (Test directory + naming conventions) + §4 (Conventions).

## Patterns to follow
- Crate-local unit-test module alongside the code under test, matching the two files this chunk consumes: `crates/conductor-core/src/capability_manifest.rs` (`#[cfg(test)]` at line 85) and `crates/conductor-core/src/coverage.rs` (line 136) — per test-plan.md §4 (Conventions).
- Factory/builder fixture over raw literals: the existing `manifest(&["P-001"])` helper in `capability_manifest.rs` tests is the in-repo shape for constructing in-sync vs. drifted manifests without re-sourcing the accepted set; per test-plan.md §7 (Fixture library / Seed strategies — Fixture files) + §4 (Fixture pattern at unit level: `#[fixture]`, `#[rstest]` + `#[case]`).
- Exact-string `assert_eq!` canonical golden at unit level for message/serialization shape (insta reserved for E2E journal goldens) — the established `conductor-core` pattern; per test-plan.md §4 (conductor-report bullet).
- Sanitization negative test modeled on the existing `load_failure_message_never_contains_the_path` in `capability_manifest.rs` — assert the drift message names ids/`sut_version`/`captured_at` and nothing path- or struct-shaped; per test-plan.md §3 (Status endpoint shape, artifact-sanitization) + §11 (Universal).
- If P4 makes the check harness-reachable, the CLI leg already has a defined shape: assert_cmd exit code + static row-count/golden, NO_COLOR-stable labels — per test-plan.md §6 (Scenario: Coverage-matrix completeness gate) + §1 Critical path 6.

## Anti-patterns to avoid
- No second hardcoded count or id range in the tests themselves — the accepted set is read through `CapabilityManifest::load`/`accepts`; the existing `matrix_has_exactly_sixty_capabilities` / contiguity assertions in `coverage.rs` are exactly the shape not to replicate (they are `v2-03`'s to resolve); per test-plan.md §11 (Unit — brittle assertions against fixture-internal values) + §1 (Scenario catalog entity, de-hardcoded).
- Never map a drift condition to a scenario `Blocked`/`Fail`/`KnownResidual` or to a `Verdict`; per test-plan.md §11 (E2E, stack-specific).
- Never test private internals of `coverage.rs`/`capability_manifest.rs` to reach the answer — assert through the public seam API (`coverage_matrix()`, `load`/`accepts`); per test-plan.md §11 (Unit).

## Contract bindings
- tests ↔ arch/core error surface: the drift result must land on the `CoreError`/`Result::Err` harness-fault side of the verdict/error wall (scope invariant), mirroring §3's transport-refusal-is-a-harness-fault and §5's OTLP `Result::Err` rule — test-plan.md §3, §5.
- tests ↔ obs: the field-allowlist/redaction layer is obs-owned, but this plan asserts the boundary by negative test — the drift message is an artifact subject to that assertion; per test-plan.md §3 (log-format-bind-with-obs; "this plan asserts that boundary via a negative test").
- tests ↔ CI/quality gates: if P4 lands the check as a hard gate, it becomes a §9 stage and a §10 build-failure condition — a gate that is red-until-`v2-03` conflicts with §10 (build failure conditions) and §11 Quality ("never skip quality gates just this once"); the non-gating-signal option must still be agent-parseable. Flag as the tests-side input to the P4 decision.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core` passes with the new drift tests; workspace nextest count increases and no test is retried (nextest `retries` stays 0) — §10.
- (tests) Positive + negative pair: an in-sync manifest fixture passes; a manifest carrying ids the classification lacks fails and the failure text names every drifted id plus `sut_version` and `captured_at`, asserted by exact-string/substring `assert_eq!`-style golden — §4, §7.
- (tests) Sanitization negative test: the drift failure text contains no absolute host path and no internal struct name — §3, §11 Universal.
- (tests) Coverage holds the Minimal gate: `cargo llvm-cov nextest --fail-under-lines 60` green (function ≥ 70%) — §10.

## Relevant amendment history
- **2026-08-08-sut-capability-manifest** (§1 Scenario catalog entity · §4 · §6 · §7) — catalog/coverage assertions were re-sourced from the SUT capability manifest instead of a hard-coded 60; loader/validator + membership-rejection coverage recorded (8 unit tests, 420 → 428). Why: the plan mandated a static assertion over an accepted set the code no longer defines. Direct predecessor: it made the accepted set data; this chunk builds the check the amendment's rationale implies, and must not reintroduce the hardcoded count it removed.
- **2026-06-16-emission-journal-writer** (§4, conductor-report bullet) — unit serialization goldens use exact-string `assert_eq!` matching the `conductor-core` canonical-golden pattern (`verdict.rs`/`report_state.rs`/`scenario.rs`); insta stays the E2E mechanism. Why: alignment of spec illustration with the established in-crate pattern. Applies here: the drift message golden is a unit-level exact-assert, not an insta snapshot.
- **2026-06-16-test-framework-fixtures-coverage-tooling** (§4) — external test-CLI versions (cargo-nextest, cargo-llvm-cov) are floors, not pins; crate dev-deps are caret-resolved with `Cargo.lock` authoritative. Applies only if this chunk adds a dev-dep (e.g. rstest cases) — the resolved version need not match §4's named number.
