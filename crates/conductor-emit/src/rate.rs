//! Seeded traffic-rate shaping: per-window span *counts* realizing a target rate curve — a linear
//! ramp or a periodic "breathing" oscillation (P-026 constellation halo-breathing, emission side).
//!
//! The rate analogue of [`crate::latency`] (which seeds per-span durations): here the seed drives
//! the per-window count sequence ([`RateCurve::window_counts`]), with bounded jitter so the seed
//! materially shapes the output (architecture §Cross-cutting Patterns — Determinism discipline).
//! Spans carry wall-clock (`std::time`) stamps; the seed governs counts + identity, never the clock.
//! P-026 is a visual/operator-checklist claim — Conductor emits the shaped stream; the halo itself
//! is verified downstream (ManualCheck), not here.

use std::f64::consts::TAU;

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans, Span};
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

use crate::message::{ok_status, service_resource, span};
use crate::span_tree::gen_id;

/// Per-window jitter band: the realized count is the curve's target rate scaled by a seeded factor
/// in `[1 - JITTER, 1 + JITTER)`, so the seed materially drives the output (different seeds diverge).
const JITTER: f64 = 0.15;

/// A target traffic-rate curve over a sequence of abstract windows. The rate unit is spans per
/// window; the window→wall-clock mapping is the timeline driver's concern (this primitive owns the
/// count shape only). Constructed only through the validating constructors so a window's target rate
/// can never go negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateCurve {
    /// A linear ramp from `from_rate` to `to_rate` across `windows`.
    Ramp {
        from_rate: u32,
        to_rate: u32,
        windows: usize,
    },
    /// A sinusoidal oscillation of `amplitude` around `center_rate` with period `period_windows`,
    /// across `windows`. `amplitude < center_rate`, so the trough stays positive.
    Breathing {
        center_rate: u32,
        amplitude: u32,
        period_windows: usize,
        windows: usize,
    },
}

impl RateCurve {
    /// A linear ramp; `None` unless `windows > 0`.
    pub fn ramp(from_rate: u32, to_rate: u32, windows: usize) -> Option<Self> {
        (windows > 0).then_some(Self::Ramp {
            from_rate,
            to_rate,
            windows,
        })
    }

    /// A breathing oscillation; `None` unless `windows > 0`, `period_windows > 0`, and
    /// `amplitude < center_rate` (the sign guard — a negative target rate is invalid).
    pub fn breathing(
        center_rate: u32,
        amplitude: u32,
        period_windows: usize,
        windows: usize,
    ) -> Option<Self> {
        (windows > 0 && period_windows > 0 && amplitude < center_rate).then_some(Self::Breathing {
            center_rate,
            amplitude,
            period_windows,
            windows,
        })
    }

    /// The number of windows this curve spans.
    pub fn windows(self) -> usize {
        match self {
            Self::Ramp { windows, .. } | Self::Breathing { windows, .. } => windows,
        }
    }

    /// The seeded per-window span counts realizing this curve: each window's target rate scaled by a
    /// bounded seeded jitter, rounded and clamped non-negative. A deterministic function of `seed`
    /// (same seed ⇒ identical sequence; different seeds ⇒ divergent), so it is the asserted shape of
    /// the P-026 breathing curve.
    pub fn window_counts(self, seed: u64) -> Vec<u32> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        (0..self.windows())
            .map(|w| {
                let jitter = 1.0 + JITTER * (2.0 * next_unit(&mut rng) - 1.0);
                (self.target_rate(w) * jitter).round().max(0.0) as u32
            })
            .collect()
    }

    /// The curve's (un-jittered) target rate at window `w`, in spans per window.
    fn target_rate(self, w: usize) -> f64 {
        match self {
            Self::Ramp {
                from_rate,
                to_rate,
                windows,
            } => {
                if windows <= 1 {
                    from_rate as f64
                } else {
                    lerp(from_rate as f64, to_rate as f64, w as f64 / (windows - 1) as f64)
                }
            }
            Self::Breathing {
                center_rate,
                amplitude,
                period_windows,
                ..
            } => center_rate as f64 + amplitude as f64 * (TAU * w as f64 / period_windows as f64).sin(),
        }
    }
}

/// Build an OTLP trace export carrying, per window, that window's seeded count of root `OK` spans
/// named `span_name` under `service_name` — emitted in window order so the stream's throughput
/// follows `curve`. Counts and span identity are a deterministic function of `seed`; `start`/`end`
/// stamps are wall-clock.
pub fn rate_trace_request(
    service_name: &str,
    seed: u64,
    curve: &RateCurve,
    span_name: &str,
) -> ExportTraceServiceRequest {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut spans: Vec<Span> = Vec::new();
    for count in curve.window_counts(seed) {
        for _ in 0..count {
            let trace_id = gen_id::<16>(&mut rng).to_vec();
            let span_id = gen_id::<8>(&mut rng).to_vec();
            spans.push(span(span_name, trace_id, span_id, Vec::new(), ok_status()));
        }
    }

    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(service_resource(service_name)),
            scope_spans: vec![ScopeSpans {
                spans,
                ..Default::default()
            }],
            ..Default::default()
        }],
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// A seeded uniform in `[0, 1)` from 53 random mantissa bits.
fn next_unit(rng: &mut ChaCha8Rng) -> f64 {
    (rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mean(s: &[u32]) -> f64 {
        s.iter().map(|&x| x as f64).sum::<f64>() / s.len() as f64
    }

    #[test]
    fn constructors_validate() {
        assert!(RateCurve::ramp(10, 100, 30).is_some());
        assert!(RateCurve::ramp(10, 100, 0).is_none());
        assert!(RateCurve::breathing(100, 40, 8, 64).is_some());
        assert!(RateCurve::breathing(100, 40, 0, 64).is_none());
        assert!(RateCurve::breathing(100, 40, 8, 0).is_none());
        assert!(RateCurve::breathing(100, 100, 8, 64).is_none());
        assert!(RateCurve::breathing(100, 150, 8, 64).is_none());
    }

    #[test]
    fn same_seed_reproduces_identical_counts() {
        let curve = RateCurve::breathing(100, 40, 8, 64).unwrap();
        assert_eq!(curve.window_counts(7), curve.window_counts(7));
    }

    #[test]
    fn different_seeds_diverge() {
        let curve = RateCurve::breathing(100, 40, 8, 64).unwrap();
        assert_ne!(curve.window_counts(1), curve.window_counts(2));
    }

    #[test]
    fn ramp_rises_from_first_third_to_last_third() {
        let curve = RateCurve::ramp(10, 200, 60).unwrap();
        let c = curve.window_counts(424_242);
        let third = c.len() / 3;
        let first = mean(&c[..third]);
        let last = mean(&c[c.len() - third..]);
        assert!(first < last, "first = {first}, last = {last}");
    }

    #[test]
    fn breathing_oscillates_around_center() {
        let center = 100u32;
        let curve = RateCurve::breathing(center, 40, 8, 128).unwrap();
        let c = curve.window_counts(424_242);
        let lo = *c.iter().min().unwrap();
        let hi = *c.iter().max().unwrap();
        assert!(lo < center, "min = {lo}");
        assert!(hi > center, "max = {hi}");
        assert!((mean(&c) - center as f64).abs() <= center as f64 * 0.10, "mean = {}", mean(&c));
    }

    #[test]
    fn builds_one_span_per_count_under_service_name() {
        let curve = RateCurve::ramp(5, 50, 20).unwrap();
        let seed = 99;
        let expected: u32 = curve.window_counts(seed).iter().sum();
        let req = rate_trace_request("conductor", seed, &curve, "tick");
        let rs = &req.resource_spans[0];
        let attrs = &rs.resource.as_ref().unwrap().attributes;
        assert!(attrs.iter().any(|kv| kv.key == "service.name"));
        let spans = &rs.scope_spans[0].spans;
        assert_eq!(spans.len(), expected as usize);
        assert!(spans.iter().all(|s| s.name == "tick"));
        assert!(spans
            .iter()
            .all(|s| s.trace_id.len() == 16 && s.span_id.len() == 8 && s.parent_span_id.is_empty()));
    }

    #[test]
    fn request_is_seeded_not_wall_clock() {
        let curve = RateCurve::breathing(80, 30, 6, 40).unwrap();
        // Determinism is asserted on the span-id projection, not the wall-clock stamps.
        let ids = |req: &ExportTraceServiceRequest| {
            req.resource_spans[0].scope_spans[0]
                .spans
                .iter()
                .map(|s| s.span_id.clone())
                .collect::<Vec<_>>()
        };
        let a = rate_trace_request("svc", 99, &curve, "op");
        let b = rate_trace_request("svc", 99, &curve, "op");
        assert_eq!(ids(&a), ids(&b));
        let c = rate_trace_request("svc", 100, &curve, "op");
        assert_ne!(ids(&a), ids(&c));
    }
}
