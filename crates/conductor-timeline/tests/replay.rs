//! Determinism-replay harness (test-plan §1/§4; arch §Design Philosophy).
//!
//! Two layers over the seeded scheduler, distinct from `determinism.rs`'s hand-picked invariant
//! unit tests: an insta golden that *freezes the absolute* `Vec<PhaseTransition>` the
//! `error-baseline-spike` fixture produces through the full
//! `Scenario::from_toml_str → PhaseTimeline::from → run_timeline` pipeline (the on-disk tripwire
//! for any RNG/jitter/gap/conversion drift — the ChaCha8 cross-version stability promise), and
//! proptest properties generalizing replay determinism + the structural invariants across the seed
//! space. All on the virtual clock (`start_paused`); only the seeded dimension is captured, never a
//! `std::time` stamp.

use std::time::Duration;

use conductor_core::Scenario;
use conductor_timeline::{run_timeline, Phase, PhaseTimeline};
use proptest::prelude::*;

/// The committed `error-baseline-spike` fixture as a runtime timeline, plus its declared seed —
/// the full parse → validate → convert pipeline the golden freezes.
fn fixture_timeline() -> (PhaseTimeline, u64) {
    let toml = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/error-baseline-spike.toml"
    ))
    .expect("fixture readable");
    let scenario = Scenario::from_toml_str(&toml).expect("fixture valid");
    let seed = scenario.seed;
    (PhaseTimeline::from(&scenario), seed)
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn fixture_stream_shape_is_frozen() {
    let (tl, seed) = fixture_timeline();
    let transitions = run_timeline(&tl, seed).await.expect("non-empty timeline");
    insta::assert_debug_snapshot!("fixture_seed_424242", transitions);
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn fixture_stream_shape_is_frozen_at_an_alternate_seed() {
    // A contrasting seed: freezes that a *different* seed yields a different — but equally stable —
    // shape, so seed-sensitivity itself is pinned, not just one seed.
    let (tl, _) = fixture_timeline();
    let transitions = run_timeline(&tl, 7).await.expect("non-empty timeline");
    insta::assert_debug_snapshot!("fixture_seed_7", transitions);
}

/// Arbitrary *valid* runtime timelines: 1..=6 phases with modest base gaps and a modest symmetric
/// jitter bound (well under `phase_spec` limits; the runtime types are not garde-validated, so the
/// strategy owns its ranges). Index-derived names avoid a string-regex strategy.
fn arb_timeline() -> impl Strategy<Value = PhaseTimeline> {
    (proptest::collection::vec(0u64..=10_000, 1..=6), 0u64..=1_000).prop_map(|(gaps, jitter)| {
        let phases = gaps
            .into_iter()
            .enumerate()
            .map(|(i, gap)| Phase::new(format!("p{i}"), Duration::from_millis(gap)))
            .collect();
        PhaseTimeline::new(phases, Duration::from_millis(jitter))
    })
}

proptest! {
    /// Replay determinism across the whole seed space: the same (timeline, seed) reproduces the
    /// exact transition sequence on every run — the architecture's headline invariant, generalized
    /// beyond `determinism.rs`'s hand-picked seeds.
    #[test]
    fn replay_is_deterministic(seed in any::<u64>(), tl in arb_timeline()) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .expect("runtime");
        let (first, second) = rt.block_on(async {
            let first = run_timeline(&tl, seed).await.expect("non-empty timeline");
            let second = run_timeline(&tl, seed).await.expect("non-empty timeline");
            (first, second)
        });
        prop_assert_eq!(first, second);
    }

    /// Structural invariants for any seed: one transition per phase in order, every gap within its
    /// declared `[base − jitter, base + jitter]` (clamped ≥ 0), and `elapsed_ms` monotonic.
    #[test]
    fn gaps_stay_bounded_and_elapsed_is_monotonic(seed in any::<u64>(), tl in arb_timeline()) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .expect("runtime");
        let transitions =
            rt.block_on(async { run_timeline(&tl, seed).await.expect("non-empty timeline") });

        prop_assert_eq!(transitions.len(), tl.phases.len());
        let bound = tl.jitter.as_millis() as i128;
        let mut prev: u128 = 0;
        for (t, phase) in transitions.iter().zip(&tl.phases) {
            prop_assert_eq!(&t.name, &phase.name);
            let gap = t.elapsed_ms as i128 - prev as i128;
            let base = phase.gap.as_millis() as i128;
            let lo = (base - bound).max(0);
            let hi = base + bound;
            prop_assert!(gap >= lo && gap <= hi, "phase {} gap {gap}ms outside [{lo}, {hi}]", t.name);
            prop_assert!(t.elapsed_ms >= prev, "elapsed went backwards at phase {}", t.name);
            prev = t.elapsed_ms;
        }
    }
}
