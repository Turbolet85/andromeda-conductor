//! Determinism + timing contract for the seeded phase scheduler (test-plan §4).
//!
//! Drives the public scheduler under tokio's virtual clock (`start_paused`) so timing is asserted
//! against scheduled shape, never real wall-clock: same seed reproduces the sequence, a different
//! seed yields a different (still-reproducible) one, and every gap stays within its declared bound.

use std::time::Duration;

use conductor_timeline::{Phase, PhaseTimeline, TimelineError, run_timeline};

fn sample_timeline() -> PhaseTimeline {
    PhaseTimeline::new(
        vec![
            Phase::new("connect", Duration::from_secs(2)),
            Phase::new("error-baseline-spike", Duration::from_secs(5)),
            Phase::new("fingerprint-storm", Duration::from_secs(3)),
            Phase::new("restart-suppression", Duration::from_secs(8)),
        ],
        Duration::from_millis(500),
    )
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn same_seed_reproduces_the_sequence() {
    let tl = sample_timeline();
    let first = run_timeline(&tl, 424_242)
        .await
        .expect("non-empty timeline");
    let second = run_timeline(&tl, 424_242)
        .await
        .expect("non-empty timeline");
    assert_eq!(first, second);
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn different_seeds_yield_different_shapes() {
    let tl = sample_timeline();
    let a = run_timeline(&tl, 1).await.expect("non-empty timeline");
    let b = run_timeline(&tl, 2).await.expect("non-empty timeline");
    assert_ne!(a, b, "distinct seeds must perturb the jitter differently");
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn gaps_stay_within_the_declared_jitter_bound() {
    let tl = sample_timeline();
    let bound = tl.jitter.as_millis() as i128;
    let transitions = run_timeline(&tl, 7).await.expect("non-empty timeline");
    assert_eq!(transitions.len(), tl.phases.len());

    let mut prev: u128 = 0;
    for (t, phase) in transitions.iter().zip(&tl.phases) {
        let gap = (t.elapsed_ms - prev) as i128;
        let base = phase.gap.as_millis() as i128;
        let lo = (base - bound).max(0);
        let hi = base + bound;
        assert!(
            gap >= lo && gap <= hi,
            "phase {} gap {gap}ms outside [{lo}, {hi}]",
            t.name
        );
        prev = t.elapsed_ms;
    }
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn zero_jitter_lands_each_gap_exactly_as_declared() {
    let tl = PhaseTimeline::new(
        vec![
            Phase::new("a", Duration::from_secs(1)),
            Phase::new("b", Duration::from_secs(4)),
        ],
        Duration::ZERO,
    );
    // No jitter ⇒ the seed cannot move the schedule; both seeds land on the declared cumulative gaps.
    for seed in [0_u64, 999] {
        let t = run_timeline(&tl, seed).await.expect("non-empty timeline");
        assert_eq!(t[0].elapsed_ms, 1_000);
        assert_eq!(t[1].elapsed_ms, 5_000);
    }
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn jitter_never_produces_a_negative_gap() {
    // A zero base with a wide bound would underflow without the clamp; elapsed must stay monotonic.
    let tl = PhaseTimeline::new(
        vec![
            Phase::new("p0", Duration::ZERO),
            Phase::new("p1", Duration::ZERO),
            Phase::new("p2", Duration::ZERO),
        ],
        Duration::from_millis(250),
    );
    let transitions = run_timeline(&tl, 13).await.expect("non-empty timeline");
    let mut prev: u128 = 0;
    for t in &transitions {
        assert!(
            t.elapsed_ms >= prev,
            "elapsed went backwards — a negative gap leaked"
        );
        prev = t.elapsed_ms;
    }
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn an_empty_timeline_is_a_harness_fault() {
    let empty = PhaseTimeline::new(Vec::new(), Duration::ZERO);
    let err = run_timeline(&empty, 1).await.unwrap_err();
    assert!(matches!(err, TimelineError::EmptyTimeline));
}
