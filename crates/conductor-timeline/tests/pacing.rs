//! Emission pacing contract for the scheduler's per-phase hook (test-plan §4).
//!
//! The scheduler drives emission through a caller-supplied hook so `conductor-timeline` keeps no
//! dependency on the emit seam. What matters here is WHEN the hook fires: a phase's declared
//! occurrences must land across that phase's own window rather than in a burst after the timeline
//! has elapsed, because a windowed SUT detector only sees the former. Asserted under the virtual
//! clock, never against real elapsed time.

use std::time::Duration;

use conductor_core::Scenario;
use conductor_timeline::{
    EmissionPoint, Phase, PhaseTimeline, TimelineError, run_timeline, run_timeline_observed,
    run_timeline_with,
};
use proptest::prelude::*;

#[derive(Debug, thiserror::Error)]
#[error("hook refused")]
struct HookRefused;

fn timeline(phases: Vec<Phase>) -> PhaseTimeline {
    PhaseTimeline::new(phases, Duration::ZERO)
}

/// The committed `fingerprint-storm` fixture as a runtime timeline, plus its declared seed. Read
/// through the same parse → convert pipeline `replay.rs` freezes the transition stream through, so
/// the two goldens describe the same run from different altitudes.
fn fixture_timeline() -> (PhaseTimeline, u64) {
    let toml = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/fingerprint-storm.toml"
    ))
    .expect("fixture readable");
    let scenario = Scenario::from_toml_str(&toml).expect("fixture valid");
    let seed = scenario.seed;
    (PhaseTimeline::from(&scenario), seed)
}

/// Each emission stamped with the virtual millisecond it fired at.
async fn stamped(tl: &PhaseTimeline, seed: u64) -> Vec<(EmissionPoint, u128)> {
    let start = tokio::time::Instant::now();
    let mut seen = Vec::new();
    run_timeline_with(tl, seed, async |point| {
        seen.push((point, (tokio::time::Instant::now() - start).as_millis()));
        Ok::<(), HookRefused>(())
    })
    .await
    .expect("non-empty timeline");
    seen
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn emissions_land_inside_their_own_phase_window() {
    let tl = timeline(vec![
        Phase::emitting("storm", Duration::from_secs(12), 6),
        Phase::emitting("quiet", Duration::from_secs(30), 0),
        Phase::emitting("resume", Duration::from_secs(4), 2),
    ]);
    let seen = stamped(&tl, 7).await;

    assert_eq!(seen.len(), 8, "6 + 0 + 2 declared emissions");

    let storm: Vec<u128> = seen.iter().filter(|(p, _)| p.phase_index == 0).map(|(_, t)| *t).collect();
    assert_eq!(storm.len(), 6);
    assert!(*storm.last().unwrap() <= 12_000, "the storm completes inside its own 12s phase");
    assert!(storm.windows(2).all(|w| w[0] < w[1]), "paced, not simultaneous: {storm:?}");

    // Nothing at all during the silence phase, and the resume phase's emissions come after it.
    assert!(!seen.iter().any(|(p, _)| p.phase_index == 1));
    let resume: Vec<u128> = seen.iter().filter(|(p, _)| p.phase_index == 2).map(|(_, t)| *t).collect();
    assert_eq!(resume.len(), 2);
    assert!(resume[0] > 42_000, "resume follows the 12s storm + 30s silence, got {resume:?}");
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn a_phase_costs_its_whole_gap_whatever_its_emission_count() {
    // Total elapsed must not depend on how the gap is sliced — the load envelope's duration term
    // and the determinism goldens both read the transition stream.
    let one = timeline(vec![Phase::emitting("p", Duration::from_secs(10), 1)]);
    let many = timeline(vec![Phase::emitting("p", Duration::from_secs(10), 7)]);
    let none = timeline(vec![Phase::emitting("p", Duration::from_secs(10), 0)]);

    let elapsed = |tl: PhaseTimeline| async move {
        run_timeline(&tl, 1).await.expect("non-empty")[0].elapsed_ms
    };
    assert_eq!(elapsed(one).await, 10_000);
    assert_eq!(elapsed(many).await, 10_000);
    assert_eq!(elapsed(none).await, 10_000);
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_transition_stream_is_unchanged_by_the_emission_count() {
    let bare = timeline(vec![
        Phase::new("a", Duration::from_secs(3)),
        Phase::new("b", Duration::from_secs(5)),
    ]);
    let dense = timeline(vec![
        Phase::emitting("a", Duration::from_secs(3), 12),
        Phase::emitting("b", Duration::from_secs(5), 40),
    ]);
    let a = run_timeline(&bare, 424_242).await.expect("non-empty");
    let b = run_timeline(&dense, 424_242).await.expect("non-empty");
    assert_eq!(a, b, "occurrences pace within a phase; they never move its boundary");
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn total_emissions_is_derivable_without_running_the_timeline() {
    let tl = timeline(vec![
        Phase::emitting("a", Duration::from_secs(1), 6),
        Phase::emitting("b", Duration::from_secs(1), 0),
        Phase::emitting("c", Duration::from_secs(1), 12),
    ]);
    assert_eq!(tl.total_emissions(), 18);
    assert_eq!(stamped(&tl, 3).await.len(), 18, "the declared total is what actually fires");
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn a_failing_hook_surfaces_as_a_harness_fault_naming_its_phase() {
    let tl = timeline(vec![
        Phase::emitting("fine", Duration::from_secs(1), 1),
        Phase::emitting("breaks", Duration::from_secs(1), 1),
    ]);
    let mut calls = 0;
    let result = run_timeline_with(&tl, 1, async |point: EmissionPoint| {
        calls += 1;
        if point.phase_index == 1 { Err(HookRefused) } else { Ok(()) }
    })
    .await;

    match result {
        Err(TimelineError::Emission { phase_index, .. }) => assert_eq!(phase_index, 1),
        other => panic!("expected an Emission fault, got {other:?}"),
    }
    assert_eq!(calls, 2, "the run stops at the failing emission");
}

/// Ties the emission golden to `replay.rs`'s transition golden: they describe ONE run, so a phase's
/// last emission sits on that phase's reported boundary — within the timer's resolution.
///
/// `paced_slices` divides a gap into sub-millisecond slices, and `tokio::time` rounds each sleep up
/// to its 1ms tick, so a phase's emissions consume up to one extra millisecond EACH beyond the gap
/// and the excess carries forward. The reported `elapsed_ms` is unaffected (it sums the jittered
/// gaps, never the clock), so the two streams drift apart by a bounded amount rather than disagreeing.
/// Asserting that bound is what catches a real pacing regression; asserting equality would be false.
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn each_phase_last_emission_sits_on_its_boundary_within_timer_resolution() {
    let (tl, seed) = fixture_timeline();
    let transitions = run_timeline(&tl, seed).await.expect("non-empty timeline");
    let stream = stamped(&tl, seed).await;

    let mut emissions_so_far = 0u128;
    for transition in &transitions {
        emissions_so_far += u128::from(tl.phases[transition.index].emissions);
        let last = stream
            .iter()
            .filter(|(p, _)| p.phase_index == transition.index)
            .map(|(_, ms)| *ms)
            .max()
            .expect("every phase in this fixture emits");

        assert!(
            last >= transition.elapsed_ms,
            "phase {} last emission {last} precedes its boundary {}",
            transition.index,
            transition.elapsed_ms
        );
        assert!(
            last <= transition.elapsed_ms + emissions_so_far,
            "phase {} overshoot {} exceeds one tick per emission so far ({emissions_so_far})",
            transition.index,
            last - transition.elapsed_ms
        );
    }
}

/// Freeze the absolute emission stream the committed storm fixture produces — the on-disk tripwire
/// for pacing/ordering drift. `replay.rs` freezes the same run's phase BOUNDARIES; a change that
/// moves emissions within their windows leaves that golden byte-identical and moves this one.
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn fixture_emission_stream_is_frozen() {
    let (tl, seed) = fixture_timeline();
    insta::assert_debug_snapshot!("fixture_emission_stream_seed_4317017", stamped(&tl, seed).await);
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn fixture_emission_stream_is_frozen_at_an_alternate_seed() {
    // A contrasting seed, so seed-SENSITIVITY of the emission stream is pinned too, not just one
    // shape (the pairing `replay.rs` established for the transition stream).
    let (tl, _) = fixture_timeline();
    insta::assert_debug_snapshot!("fixture_emission_stream_seed_7", stamped(&tl, 7).await);
}

/// The phase observer fires once per phase, in order, INCLUDING a phase that declares no emissions —
/// the case `on_emit` can never reach, and the whole reason the hook exists (obs-plan §4).
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_phase_observer_fires_once_per_phase_including_a_silent_one() {
    let tl = timeline(vec![
        Phase::emitting("storm", Duration::from_secs(2), 3),
        Phase::emitting("quiet", Duration::from_secs(1), 0),
        Phase::emitting("resume", Duration::from_secs(1), 2),
    ]);
    let seen = std::cell::RefCell::new(Vec::new());

    run_timeline_observed(
        &tl,
        7,
        |window| seen.borrow_mut().push((window.index, window.name.to_string(), window.gap)),
        async |_: EmissionPoint| Ok::<(), HookRefused>(()),
    )
    .await
    .expect("non-empty timeline");

    let observed = seen.into_inner();
    assert_eq!(
        observed,
        vec![
            (0, "storm".to_string(), Duration::from_secs(2)),
            (1, "quiet".to_string(), Duration::from_secs(1)),
            (2, "resume".to_string(), Duration::from_secs(1)),
        ],
        "one call per phase, in declared order, carrying the effective gap"
    );
}

/// The observer's value is held for exactly its phase: dropped before the NEXT phase opens, so a
/// span it returns brackets that phase's window rather than outliving it (obs-plan §11: no dangling).
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_observed_value_is_dropped_at_its_own_phase_boundary() {
    let events = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

    struct Guard(usize, std::rc::Rc<std::cell::RefCell<Vec<String>>>);
    impl Drop for Guard {
        fn drop(&mut self) {
            self.1.borrow_mut().push(format!("drop {}", self.0));
        }
    }

    let tl = timeline(vec![
        Phase::emitting("a", Duration::from_secs(1), 1),
        Phase::emitting("b", Duration::from_secs(1), 0),
    ]);
    let log = std::rc::Rc::clone(&events);
    run_timeline_observed(
        &tl,
        7,
        |window| {
            log.borrow_mut().push(format!("open {}", window.index));
            Guard(window.index, std::rc::Rc::clone(&log))
        },
        async |_: EmissionPoint| Ok::<(), HookRefused>(()),
    )
    .await
    .expect("non-empty timeline");

    assert_eq!(
        events.borrow().as_slice(),
        ["open 0", "drop 0", "open 1", "drop 1"],
        "each phase's value closes before the next opens"
    );
}

/// The observing form leaves the transition stream identical to the plain one — instrumentation
/// must not move a boundary (test-plan §7 goldens; architecture §Determinism discipline).
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn observing_does_not_move_the_transition_stream() {
    let (tl, seed) = fixture_timeline();
    let plain = run_timeline(&tl, seed).await.expect("non-empty timeline");
    let observed = run_timeline_observed(
        &tl,
        seed,
        |window| window.index,
        async |_: EmissionPoint| Ok::<(), HookRefused>(()),
    )
    .await
    .expect("non-empty timeline");
    assert_eq!(plain, observed);
}

proptest! {
    /// Generalize the goldens across the seed space: whatever the seed, replaying it reproduces the
    /// same emission stream. `#[tokio::test]` cannot wrap a `proptest!` block, so each case owns its
    /// paused runtime and the assertion happens outside the future.
    #[test]
    fn the_emission_stream_replays_identically_under_any_seed(seed in any::<u64>()) {
        let (tl, _) = fixture_timeline();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .expect("paused runtime builds");
        let first = rt.block_on(stamped(&tl, seed));
        let second = rt.block_on(stamped(&tl, seed));
        prop_assert_eq!(first, second, "same timeline + seed must replay one emission stream");
    }

    /// The declared occurrence total is what fires, for every seed — jitter moves WHEN an emission
    /// lands, never HOW MANY there are.
    #[test]
    fn the_declared_occurrence_total_is_seed_independent(seed in any::<u64>()) {
        let (tl, _) = fixture_timeline();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .expect("paused runtime builds");
        let stream = rt.block_on(stamped(&tl, seed));
        prop_assert_eq!(stream.len() as u64, tl.total_emissions());
    }
}
