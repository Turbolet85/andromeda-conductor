# Codebase Research — 2026-06-17-determinism-replay-harness

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 4
- Mature-codebase chunk: the system-under-test (`conductor-timeline` scheduler + `Scenario→PhaseTimeline` bridge) already exists and ships unit/integration tests; this chunk adds a *golden + property* layer over it. No production code expected to change.

## Files inspected
- `crates/conductor-timeline/src/lib.rs` (full) — public surface = re-exports `Phase`, `PhaseTimeline`, `PhaseTransition`, `run_timeline`, `TimelineError`; modules `convert`/`phase`/`scheduler`. This is the seam under test.
- `crates/conductor-timeline/src/scheduler.rs` (full) — `run_timeline(&PhaseTimeline, seed) -> Result<Vec<PhaseTransition>, TimelineError>`: seeds `ChaCha8Rng::seed_from_u64(seed)`, draws `jittered_gap` per phase, `sleep`s the gap on the **virtual** clock, and reports `elapsed_ms`. **Key:** `elapsed_ms` is an internal accumulator (`let mut elapsed = Duration::ZERO; … elapsed = elapsed.saturating_add(effective)`, lines 45/51/55) — it is **never a clock read**, so the output `Vec<PhaseTransition>` is a *pure function of (timeline, seed)*. `jittered_gap` (lines 64-72): `delta = rng.next_u64() % (2*bound+1) - bound`, clamped `≥ 0`.
- `crates/conductor-timeline/src/phase.rs` (full) — `Phase{name:String, gap:Duration}`, `PhaseTimeline{phases:Vec<Phase>, jitter:Duration}`, `PhaseTransition{index:usize, name:String, elapsed_ms:u128}`. All derive `Debug, Clone, PartialEq, Eq` → `insta::assert_debug_snapshot!` works with **zero production change**.
- `crates/conductor-timeline/src/convert.rs` (full) — `impl From<&Scenario> for PhaseTimeline` (order-preserving, infallible). Its in-crate `#[cfg(test)]` already asserts scenario-level same-seed reproduction + different-seed divergence at the **unit** level — so this chunk's value is *absolute freeze* (golden) + *seed-space generalization* (proptest), not re-proving relative consistency.
- `crates/conductor-timeline/tests/determinism.rs` (full) — the existing integration suite (6 tests, all `#[tokio::test(flavor="current_thread", start_paused=true)]`): `same_seed_reproduces_the_sequence`, `different_seeds_yield_different_shapes`, `gaps_stay_within_the_declared_jitter_bound`, `zero_jitter_lands_each_gap_exactly_as_declared`, `jitter_never_produces_a_negative_gap`, `an_empty_timeline_is_a_harness_fault`. **None pin absolute `elapsed_ms`; none use insta or proptest.** This is the precedent file + the exact gap this chunk fills.
- `crates/conductor-core/src/scenario.rs` (full) — `Scenario::from_toml_str` (serde→garde); test `committed_fixture_loads_and_validates` (line 260) loads `scenarios/error-baseline-spike.toml` via `../../scenarios/…`. Confirms the fixture path + load idiom for the full-pipeline golden.
- `crates/conductor-core/src/phase_spec.rs` (full) — bounds for proptest strategies: `MAX_GAP_MS = 3_600_000`, `MAX_JITTER_MS = 60_000`, phase `name` 1..=40 chars. (Note: these garde bounds live on `PhaseSpec`/`Scenario`; the runtime `Phase`/`PhaseTimeline` are *not* garde-validated, so a proptest strategy can build `PhaseTimeline` directly with chosen bounds.)
- `scenarios/error-baseline-spike.toml` — the canonical golden input: `seed = 424242`, `jitter_ms = 50`, phases `baseline`(gap 2000ms) + `spike`(gap 1000ms).
- `.config/nextest.toml` — `retries = 0` in **both** `ci` and `default` profiles (zero-flakiness invariant). `ci` writes `junit.xml`.
- `Cargo.toml` (workspace) — dev-deps `insta = "1"`, `proptest = "1"` already declared; `conductor-timeline` dev-tokio carries `test-util` (enables `start_paused`). **No new dependency needed.**
- `.github/workflows/ci.yml` (lines 36-58) — CI runs `scripts/agent-run.ps1 run` (= `nextest --profile ci` + `test --doc` + `clippy --all-targets -D warnings`) then `cargo llvm-cov nextest --workspace --profile ci --summary-only`. Comment line 56: *"the coverage threshold + … artifact upload are Epoch 10"* → **no `--fail-under-lines` gate exists here.**
- `.gitignore` — ignores only `/target/`; `tests/snapshots/` and `proptest-regressions/` are **committed by default** (no `git add` exclusion to fight).
- `.andromeda/context/api-surface.md` — corroborates the timeline public surface (Phase/PhaseTimeline/PhaseTransition/run_timeline/TimelineError) and that `PhaseTransition` derives `Debug, Clone, PartialEq, Eq`.

## Patterns detected
- **`start_paused` virtual-clock test** (`determinism.rs:23`, `convert.rs:69`): `#[tokio::test(flavor = "current_thread", start_paused = true)]` — the established way to drive `run_timeline` instantly with no real wait.
- **Both-directions determinism** (`determinism.rs:24/32`, `convert.rs:70/83` + testing.md session note): same-seed-identical AND different-seeds-divergent, each with *fixed* seeds known to diverge over multiple draws.
- **Clock-independent output** (`scheduler.rs:45-55`): `elapsed_ms` is an accumulated sum of jittered gaps, not an `Instant::now()` read — so a golden's exact values are stable, and a future switch to a real-clock read would break the golden (the "paused-clock leak" tripwire from testing.md).
- **Fixture-driven full pipeline** (`scenario.rs:260`): `Scenario::from_toml_str(read_to_string("../../scenarios/<name>.toml"))` → `PhaseTimeline::from(&s)` → `run_timeline` is the end-to-end path the golden should exercise.

## Conventions to follow
- New integration tests live in `crates/conductor-timeline/tests/` (precedent: `determinism.rs`); seam-local, public-API-only.
- insta in **assert mode** under CI (`CI` env auto-detected by insta → no `.snap.new` writes, fail on mismatch); the committed `.snap` is generated once locally via `cargo insta review`/`accept` and committed (test-plan §4; security extract).
- proptest persists counterexamples to `proptest-regressions/` (default `SourceParallel` → `tests/proptest-regressions/replay.txt`); commit it (testing.md "proptest-regressions/ persisted").
- Zero nextest retries; test code must be `clippy --all-targets -D warnings`-clean (CI runs it).

## New files to create
- `crates/conductor-timeline/tests/replay.rs` — the replay harness: insta golden(s) over the full `Scenario`-fixture pipeline + proptest property suite (replay determinism + structural invariants), all under `start_paused`.
- `crates/conductor-timeline/tests/snapshots/replay__*.snap` — the committed insta golden(s) (generated locally + accepted + committed; CI asserts).
- `crates/conductor-timeline/tests/proptest-regressions/replay.txt` — created only if/when proptest finds a counterexample; the directory is tracked (a real failure here = a determinism bug to fix, never a flake to retry).

## Files to modify
- **None (production).** The chunk is purely additive test code; `assert_debug_snapshot!` over `PhaseTransition: Debug` needs no `Serialize` derive. (If a YAML/JSON snapshot were judged clearer at P4, the *only* production delta would be `#[derive(Serialize)]` on `PhaseTransition` — default is Debug, so no change.)

## Open questions
- **proptest search-RNG**: use proptest's default entropy-seeded search + committed regressions (standard) vs pinning `PROPTEST_RNG_SEED`? → **Resolve at P4: default + committed regressions.** Rationale: the properties are universally true (∀ seed), so the search's own randomness cannot produce a flake — a discovered counterexample is a *real* determinism break, persisted and replayed deterministically (consistent with zero-retry). This is the only place "randomness" is not pinned, and it is the test *search* strategy, not the SUT.
- **Second golden**: add a contrasting-seed golden so seed-sensitivity is also frozen (not just the fixture seed)? → minor; fold into P4.
