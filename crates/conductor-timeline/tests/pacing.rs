//! Emission pacing contract for the scheduler's per-phase hook (test-plan §4).
//!
//! The scheduler drives emission through a caller-supplied hook so `conductor-timeline` keeps no
//! dependency on the emit seam. What matters here is WHEN the hook fires: a phase's declared
//! occurrences must land across that phase's own window rather than in a burst after the timeline
//! has elapsed, because a windowed SUT detector only sees the former. Asserted under the virtual
//! clock, never against real elapsed time.

use std::time::Duration;

use conductor_timeline::{
    EmissionPoint, Phase, PhaseTimeline, TimelineError, run_timeline, run_timeline_with,
};

#[derive(Debug, thiserror::Error)]
#[error("hook refused")]
struct HookRefused;

fn timeline(phases: Vec<Phase>) -> PhaseTimeline {
    PhaseTimeline::new(phases, Duration::ZERO)
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
