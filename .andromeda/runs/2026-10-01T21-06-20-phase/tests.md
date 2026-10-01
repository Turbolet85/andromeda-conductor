# tests extract

## Relevance
relevant — the chunk changes seeded emission identity (or drive pacing) in `conductor-emit`/the real-model harness and must ship a test holding it, which touches the plan's determinism-replay, golden, mocking and live-leg rules.

## Constraints
- test-plan §1 (property-test trigger, determinism discipline) and §2 (Property-based row) require a property/golden test that a fixed scenario+seed reproduces an identical emission-journal stream SHAPE across runs; whether `trace_id`/`span_id` are part of that asserted "shape" (so shape (a) moves a pinned invariant) is research's question, not settled by the plan's wording.
- test-plan §7 (Seed strategies → Golden artifacts) requires the seeded-stream insta goldens (`replay__*` · `pacing__*` · `dispatch_wire__*`) to stay committed and CI fail-don't-write; the dispatch-tier projection is defined there by what it EXCLUDES (`*_time_unix_nano`), so whether it carries span identity — and would therefore churn under shape (a) — is research's question. Any re-baseline is a reviewed committed snapshot change, never `cargo insta review` (per test-plan §11 Universal).
- test-plan §10 (Zero-flakiness budget) and §11 Test Data require every generator to be seed-controlled; a per-run identity source (shape a) must be injectable/deterministic under test, not an unseeded randomness leak into the deterministic tier.
- test-plan §4 (What unit tests cover → conductor-emit) places OTLP struct construction + byte-level encoding at the unit tier; the identity-derivation change is tested there, with `cargo test -p conductor-emit` additionally green as the §4 runner-portability gate.
- test-plan §6 (Real-model interpretation leg) and §2 (agent-runnable invariants) hold the real-model leg operator-gated, never CI, driven only as the pre-stated series of `contracts/pulse-real-model-leg-posture.md`; the chunk's scope also forbids a new series — so "both land" against live Pulse cannot be this chunk's CI proof.
- test-plan §11 E2E (`sleep(N)` ban) admits a harness-level wait BETWEEN legs only when it reproduces a documented SUT-side precondition; shape (b) (a minimum same-seed gap) must be stated as that class, with its duration MEASURED against Pulse's buffer/eviction behaviour, never a tuned guess.
- test-plan §10 (Coverage thresholds) requires `--fail-under-lines 60` to hold over the workspace with the change.

## Patterns to follow
- Loopback OTLP egress test server — an in-process tonic `TraceServiceServer` on ephemeral `127.0.0.1:0` (per test-plan §2 Integration row; §8 What to mock → OTLP/gRPC egress) to capture two same-seed drives' dispatched spans byte-level and compare their identities.
- Seeded generators + proptest with `proptest-regressions/` persistence (per test-plan §7 Seed strategies → Randomized data) for an identity-distinctness or determinism property.
- Insta stream goldens per test-file family × seed under `<crate>/tests/snapshots/` (per test-plan §7 Golden artifacts) — extend or re-baseline the family the change touches rather than adding an ad-hoc literal.
- Crate-local `tests/` with `tests/common` shared fixtures for slower in-crate integration (per test-plan §4 Conventions; §10 coverage regex already excludes `tests/` trees).

## Anti-patterns to avoid
- Faking Pulse's reaction (a stub that "refuses duplicate (trace_id, span_id)") and grading it as the "both land" verdict in CI (per test-plan §11 Test Strategy, §8 What NOT to mock → Live Pulse's reaction): the CI test can prove the Conductor-side property only.
- Unseeded or wall-clock-derived identity in the deterministic tier, or real wall-clock waits used to synchronise inside a test (per test-plan §11 Test Data, §11 Unit, §11 E2E).
- Retrying or quarantining-by-knob a test that differs run-to-run (per test-plan §10 Zero-flakiness budget, §11 Quality).

## Contract bindings
- tests ↔ arch: the "same scenario+seed ⇒ same stream shape" invariant (test-plan §1, §7) is the architecture's determinism bar; which fields constitute "shape" is owned by `.andromeda/architecture.md` (§Cross-cutting Patterns per CLAUDE.md) — a shape (a) fix must be reconciled there, not only in tests.
- tests ↔ harness: shape (b) lands in `scripts/agent-run.{sh,ps1}` `run --live real-model` (test-plan §3 `run`), carried in both shells; the series posture is `contracts/pulse-real-model-leg-posture.md` (test-plan §6).
- tests ↔ mutation audit: `conductor-emit`'s `exception.rs` is in the dispositioned mutation population, and `mutation-gate.py conductor-emit` currently fails closed on unrostered timeouts (per test-plan §12 `conductor-emit`'s accepted-deliberate SINGLE) — operator-local, non-CI; an edit to `exception.rs` shifts that population.

## Acceptance criteria contributions
- (tests) A test drives two same-scenario, same-seed emissions through the real dispatch path into a loopback capture and asserts the property the chosen shape claims — (a) distinct `(trace_id, span_id)` per drive, or (b) the enforced minimum gap — and fails when the fix is reverted; `cargo nextest run -p <touched crate>` and `cargo test -p <touched crate>` both green (per test-plan §4, §8 What to mock → OTLP/gRPC egress).
- (tests) The determinism-replay property/goldens still pass for the same scenario+seed: every committed `replay__*` / `pacing__*` / `dispatch_wire__*` snapshot is either byte-unchanged or re-baselined with the change named and justified, never via interactive review (per test-plan §1 property-test trigger, §7 Golden artifacts).
- (tests) Workspace `cargo nextest run --workspace --profile ci` green with zero retries, and `--fail-under-lines 60` holds (per test-plan §10).
- (tests) The live "both land against Pulse" claim is recorded as operator-local/owed, not asserted as met by a CI or stub test (per test-plan §6 Real-model interpretation leg, §11 Test Strategy).
