# arch extract

## Relevance
Partial — the chunk's focus is a test harness validating the timeline engine's determinism (already existing), not new architecture; it does inherit arch patterns for runtime and testing discipline.

## Constraints
- Per §Design Philosophy: determinism under a seed via `current_thread` tokio runtime with zero work-stealing enforces reproducibility in the runtime flavor, not as a convention.
- Per §Established Decisions [Async Runtime Flavor]: `tokio::current_thread`, core-owned; must use `#[tokio::test(flavor = "current_thread", start_paused = true)]` to assert scheduled ordering, never real wall-clock.
- Per §Established Decisions [Determinism RNG]: `rand_chacha::ChaCha8Rng` seeded via `seed_from_u64` is the sole source of non-determinism; test must seed all randomness and assert cross-platform/version stability.
- Per §Stack and Technologies (Determinism RNG row): `rand_chacha 0.9 (ChaCha8Rng) + rand_core 0.9 (SeedableRng)` are already workspace-pinned and must be reused, not augmented.
- Per §Infrastructure Patterns (Build system): dev-test stack includes `insta` + `proptest` (already dev-deps); snapshots in assert/CI mode (fail, never auto-write); zero-retry `ci` profile in `.config/nextest.toml`.
- Per §Conventions (Data model): no new serialization required for golden (PhaseTransition::Debug suffices); if JSON/YAML snapshot chosen, minimal `#[derive(Serialize)]` is the only production delta (resolve in planning).

## Patterns to follow
- **Determinism test structure:** `run_timeline(tl, seed)` twice ⇒ identical (the replay property); structural invariants (per-gap ∈ `[base−jitter, base+jitter]` clamped ≥ 0; `elapsed_ms` monotonic non-decreasing).
- **Virtual-clock discipline:** all tests under `start_paused = true`; assert scheduled shape, never real duration; zero real-time waits eliminates flakiness and nextest retries.
- **Fixture loading:** the committed `scenarios/error-baseline-spike.toml` is the canonical input; load via `Scenario::from_toml_str` (already proven by core test).
- **Golden + proptest split:** insta pins absolute stream shape; proptest generalizes replay + invariants across the seed space; `proptest-regressions/` committed so counterexamples replay.

## Anti-patterns to avoid
- Do NOT introduce new production logic or types (the timeline, `Phase`, `PhaseTimeline`, `PhaseTransition`, and `From<&Scenario>` seam already exist and are system-under-test).
- Do NOT snapshot the JSONL `journal_emitted_at` stamps (those are `std::time` real-clock, deliberately non-reproducible); capture only the seeded/virtual dimension (`Vec<PhaseTransition>`) to keep the wall-clock-vs-virtual wall intact.
- Do NOT add new workspace dependencies; `insta` and `proptest` are already `conductor-timeline` dev-deps (per scope §Boundaries).

## Contract bindings
Timeline engine → tests harness: the public seam (`run_timeline`, `Phase`, `PhaseTimeline`, `PhaseTransition`, `TimelineError`) is the system under test; must be driven under `start_paused` to assert determinism is a property of the code, not an artifact of slow real-time runs. Cross-test-plan §4 for exact toolchain versions (insta/proptest/rstest/etc.); test-plan prescribes verdict/error wall remains unchanged (empty timeline ⇒ `TimelineError::EmptyTimeline`, a harness fault, not a verdict).

## Acceptance criteria contributions
- (arch) Timeline determinism test harness lives entirely within `conductor-timeline` (tests/ integration + #[cfg(test)] proptest strategies) per workspace boundary rules §Inherited Defaults.
- (arch) Golden snapshot committed under `crates/conductor-timeline/tests/snapshots/` via insta in assert mode; proptest regressions committed; driven under `start_paused = true`.
- (arch) All test randomness seeded; replay property (same seed ⇒ identical shape) + structural invariants (bounded per-gap jitter, monotonic elapsed_ms) asserted with zero nextest retries.
- (arch) No new dependencies; fixture `scenarios/error-baseline-spike.toml` loaded via existing `Scenario::from_toml_str` → `PhaseTimeline::from(&Scenario)` → `run_timeline` pipeline.

## Relevant amendment history
- 2026-06-16-seeded-phase-scheduler: rand_chacha 0.9 (ChaCha8Rng) + rand_core 0.9 registered in §Stack and §Established Decisions; platform/version-stable `seed_from_u64` pinned as the sole non-determinism source — this chunk seeds and validates that choice under `start_paused`.
- 2026-06-16-test-framework-fixtures-coverage-tooling: test/coverage toolchain (`insta`/`proptest`/`rstest`/`assert_cmd`/`assert_fs`/`predicates` dev-test stack + cargo-nextest zero-retry `ci` profile + `cargo-llvm-cov`) registered in §Infrastructure — this chunk inherits the pinned runners and assert-mode snapshot discipline.
