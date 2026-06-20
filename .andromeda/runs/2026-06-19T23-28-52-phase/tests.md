# tests extract

## Relevance
partial — chunk builds a fault helper (abrupt-silence) with unit-testable pure logic; integration/E2E coverage of scenario invocation deferred to later epochs

## Constraints
- §4 Unit Test Strategy: unit framework (cargo-nextest 0.9.137 + rstest 0.26.1 fixtures) for pure functions (faults generators are "seeded functions — same seed ⇒ same shape")
- §5 Integration Test Strategy: conductor-faults seeded generators as pure-function tests (not integration boundaries); the `:4317` port-occupier bind/release is an integration concern (not applicable to abrupt-silence, which is permanent-stop only)
- §5 Coverage Trigger: bounded fault-injection scenario tests (P-060 "typical/high" profiles only); abrupt-silence assertion via MCP read-back verification deferred to scenario epoch
- §8 Mocking & Stubbing: time NOT mocked (abrupt-silence is deterministic/seed-independent, no time dimension); randomness NOT applicable (no seeded randomness — deterministic by construction)
- §10 Quality Gates: line coverage ≥60% (Minimal tier); no perf budget (Creator Brief anti-pattern: "NOT a load-tester ... explicitly out")
- §11 Test Anti-Patterns / Unit: NEVER assert on real wall-clock duration; test public seam API + observable behavior; NEVER test implementation details

## Patterns to follow
- rstest `#[fixture]` + table-driven `#[case]` over valid/invalid fault configurations (if construction is fallible)
- Exact-string `assert_eq!` for canonical serialization shape (per §4 conductor-report / emission-journal golden pattern)
- Pure-function unit assertion via deterministic-replay (same scenario+seed ⇒ same fault shape); no `different-seeds-diverge` property test (there is no seed to drive divergence)
- `#[tokio::test(flavor = "current_thread", start_paused = true)]` if any async fixture; direct function calls for pure logic

## Anti-patterns to avoid
- NEVER mock the fault primitive itself (test the public constructor + observable fault state)
- NEVER assert against real elapsed time (abrupt-silence is atemporal; scope note: "The 30s cue is Pulse's reaction latency, not a Conductor input bound")
- NEVER introduce a perf budget or load-saturation test (out-of-scope per Creator Brief)

## Contract bindings
timeline ↔ tests: the "active lead-in phase" (5 min active before the stop) and stop execution wiring deferred to Epochs 7/8; this chunk ships the helper only

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-faults` passes for the abrupt-silence unit tests
- (tests) Coverage: new-code line coverage ≥60% (Minimal tier, §10)
- (tests) Permanence property exercised: unit test asserts abrupt-silence has no resume boundary (distinguishing it from `EmissionGap` per scope "The *absence* of a resume boundary is the contract")

## Relevant amendment history
(none) — no prior amendments to test-plan.md touching fault-helper unit testing or conductor-faults. The fault-injection trigger (§5 coverage-trigger: chaos-test) and the seeded-generators pattern (§8 Mocking & Stubbing) are stable references; no recent amendments to that zone.