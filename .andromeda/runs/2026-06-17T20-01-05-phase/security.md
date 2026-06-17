# security extract

## Relevance
partial — timeline harness is a test infrastructure chunk within `conductor-timeline` that does not cross external boundaries; primary security relevance is input validation (scenario config fixture) and test determinism (dependency on RNG seeding).

## Constraints
- Scenario fixtures deserialized in tests MUST be pre-validated via garde (security plan §Input Validation, scenario-config boundary row) — the committed `error-baseline-spike.toml` fixture, loaded via `Scenario::from_toml_str`, must derive `Validate` with `range` + `#[garde(custom)]` rules at load, never silently pass malformed config.
- All test RNG seeding MUST remain deterministic and cross-platform stable — `ChaCha8Rng` pinned per security plan §Dependency Security, with committed `proptest-regressions/` to catch seed-space counterexamples and prevent silent drift (security plan §Threat Model Summary, Input Validation).
- Virtual-clock isolation under `start_paused` is mandatory — no real wall-clock waits, no tokio threads that could cause flakiness-masking; the harness MUST run under `#[tokio::test(flavor = "current_thread", start_paused = true)]` to assert scheduled shape, never real duration (determinism invariant per scope).
- Insta snapshots committed to `crates/conductor-timeline/tests/snapshots/` MUST run in **assert mode (fail, never auto-write)** per test-plan §4 — no CI auto-upgrade of golden files; a snapshot change is a deliberate decision requiring human code review.

## Patterns to follow
- **Config validation at fixture load:** the `Scenario::from_toml_str` call in the golden/proptest setup inherits the same garde validation boundary as production config — test fixtures are not exempt (per security plan §Input Validation, Error Handling: type-erased `anyhow` edge must stay sanitized).
- **Seeded randomness throughout:** any RNG used in the test (timeline seed, proptest strategies) MUST be seeded consistently and captured in the committed regressions/snapshots — zero ambient randomness leaks into assertion results (security plan §Threat Model Summary: "input validation on operator-supplied scenario config").
- **Determinism over coverage:** the harness prioritizes reproducibility (same seed ⇒ identical shape) over line coverage; proptest generalizes replay determinism, not branch-hit metrics.

## Anti-patterns to avoid
- NEVER let insta snapshots auto-write in CI or commit pre-verified diffs as golden truth — assert mode only, human review required for any update (per test-plan §4 and security plan §Error Handling).
- NEVER use real wall-clock timers or tokio::spawn threads in the determinism tests — `start_paused` isolation MUST remain tight; any flakiness masks the determinism property under test.
- NEVER deserialize scenario fixtures without garde validation, even in tests — a silent parse success on invalid config bypasses the security plan's trust boundary at the CLI edge (Input Validation).

## Contract bindings
**design §Auth UX + timeline** — the scenario config (repro fixture) is authored by the operator on the local host; no remote/untrusted source. **tests §CI Integration** — insta snapshots and proptest regressions are committed artifacts verified in the existing `cargo test` CI job (no new job); insta assert mode enforces human review of drift. **obs §PII Scrubbing** — the timeline harness emits no observations (only the scheduler's `PhaseTransition` seq); PII scrubbing applies to OTLP emission (Epoch 3, out of scope here).

## Acceptance criteria contributions
- (security) Scenario fixture `error-baseline-spike.toml` deserializes with garde validation green at test setup (no silent config parse).
- (security) Committed insta golden snapshot (`crates/conductor-timeline/tests/snapshots/*` `.snap` files) under assert mode passes on CI; any deviation is human-reviewed before merge (no auto-write).
- (security) All RNG seeding in proptest strategies and test setup captured in committed `proptest-regressions/` (zero ambient randomness); proptest replay-determinism property (`∀ seed: run_timeline(seed)` twice ⇒ identical) passes for the seed space.
- (security) Timeline harness runs under `start_paused` virtual-clock discipline; zero real-time waits and zero nextest retries (determinism invariant = scheduled shape, not wall-clock duration).

## Relevant amendment history
- **2026-06-15-config-validation-surface** — garde pinned 0.23.0 → 0.22.1 (both §Input Validation scenario-config row + §Bootstrap phases input-validation-library-install). Affects this chunk: the `Scenario::from_toml_str` call in the test fixture inherits garde 0.22.1 validation, not 0.23.0.
- **2026-06-15-dependency-audit-gate** — audit-tool versions reframed as minimum floors (cargo-audit ≥0.22, cargo-deny ≥0.19); toolchain "done" (1.95.0 channel, `rust-version = 1.94.1`). Affects this chunk: no new toolchain/audit changes needed (already satisfied); the proptest/insta dev-deps audit is gated by the existing `cargo test` CI gate.
