//! Bridge: a validated `conductor_core::Scenario` → the runtime [`PhaseTimeline`] the scheduler
//! sequences.
//!
//! Total, deterministic, order-preserving — each phase spec becomes a [`Phase`] in declaration
//! order, and the scenario's `jitter_ms` becomes the timeline's symmetric jitter bound. Conversion
//! is infallible: garde guarantees a non-empty, in-bounds phase list before this runs, and an empty
//! timeline is still caught downstream by [`run_timeline`](crate::run_timeline). The per-phase
//! emission COUNT crosses over (the scheduler paces that many hook calls across the gap); the
//! emission SHAPE does not — it stays in the config model, and the dispatcher reads it there.

use std::time::Duration;

use conductor_core::Scenario;

use crate::{Phase, PhaseTimeline};

impl From<&Scenario> for PhaseTimeline {
    fn from(scenario: &Scenario) -> Self {
        let phases = scenario
            .phases
            .iter()
            .map(|spec| {
                Phase::emitting(
                    spec.name.clone(),
                    Duration::from_millis(spec.gap_ms),
                    spec.emission.occurrences,
                )
            })
            .collect();
        PhaseTimeline::new(phases, Duration::from_millis(scenario.jitter_ms))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use conductor_core::{EmissionSpec, PId, PhaseSpec, Scenario, SloTier};

    use crate::{run_timeline, PhaseTimeline};

    fn scenario(seed: u64) -> Scenario {
        Scenario {
            name: "error-baseline-spike".to_string(),
            p_ids: vec![PId("P-009".to_string())],
            seed,
            slo_tier: SloTier::Tier5s,
            phases: vec![
                PhaseSpec {
                    name: "baseline".to_string(),
                    gap_ms: 2000,
                    emission: EmissionSpec::default(),
                },
                PhaseSpec {
                    name: "spike".to_string(),
                    gap_ms: 1000,
                    emission: EmissionSpec::default(),
                },
            ],
            jitter_ms: 200,
            expected: Vec::new(),
        }
    }

    #[test]
    fn conversion_preserves_order_and_maps_timing() {
        let tl = PhaseTimeline::from(&scenario(1));
        assert_eq!(tl.phases.len(), 2);
        assert_eq!(tl.phases[0].name, "baseline");
        assert_eq!(tl.phases[0].gap, Duration::from_millis(2000));
        assert_eq!(tl.phases[1].name, "spike");
        assert_eq!(tl.phases[1].gap, Duration::from_millis(1000));
        assert_eq!(tl.jitter, Duration::from_millis(200));
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn same_seed_reproduces_transitions_in_phase_order() {
        let s = scenario(42);
        let tl = PhaseTimeline::from(&s);
        let a = run_timeline(&tl, s.seed).await.expect("transitions");
        let b = run_timeline(&tl, s.seed).await.expect("transitions");
        assert_eq!(a, b);
        assert_eq!(
            a.iter().map(|t| t.name.as_str()).collect::<Vec<_>>(),
            vec!["baseline", "spike"]
        );
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn different_seeds_diverge() {
        let tl = PhaseTimeline::from(&scenario(1));
        let a = run_timeline(&tl, 1).await.expect("transitions");
        let b = run_timeline(&tl, 7).await.expect("transitions");
        assert_ne!(
            a.iter().map(|t| t.elapsed_ms).collect::<Vec<_>>(),
            b.iter().map(|t| t.elapsed_ms).collect::<Vec<_>>(),
        );
    }
}
