//! Per-operation latency shaping: seeded span durations realizing a target p50/p95/p99 profile
//! (P-011 per-operation latency baseline; P-012 latency-regression detection — emission side).
//!
//! Each span's duration is a deterministic function of the seed (a stratified inverse-CDF draw);
//! the absolute `start`/`end` stamps are wall-clock (`std::time`), so the determinism contract
//! governs the duration sequence only (architecture §Cross-cutting Patterns — Determinism
//! discipline). The `LatencyProfile` self-validates p50 ≤ p95 ≤ p99 in its constructor (mirroring
//! [`crate::logs::Severity`]); the scenario-config garde layer in `conductor-core` is the
//! authoritative validator once latency targets join the scenario model.

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans, Span};
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

use crate::message::{ok_status, service_resource, timed_span};
use crate::span_tree::gen_id;

const NANOS_PER_MS: f64 = 1_000_000.0;

/// A per-operation target latency profile, in milliseconds. Constructed only in p50 ≤ p95 ≤ p99
/// order, so the shaped distribution can never invert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LatencyProfile {
    p50_ms: u64,
    p95_ms: u64,
    p99_ms: u64,
}

impl LatencyProfile {
    /// Build a profile; `None` unless `p50 ≤ p95 ≤ p99`.
    pub fn new(p50_ms: u64, p95_ms: u64, p99_ms: u64) -> Option<Self> {
        (p50_ms <= p95_ms && p95_ms <= p99_ms).then_some(Self {
            p50_ms,
            p95_ms,
            p99_ms,
        })
    }

    /// The target 50th-percentile latency (milliseconds).
    pub fn p50_ms(self) -> u64 {
        self.p50_ms
    }

    /// The target 95th-percentile latency (milliseconds).
    pub fn p95_ms(self) -> u64 {
        self.p95_ms
    }

    /// The target 99th-percentile latency (milliseconds).
    pub fn p99_ms(self) -> u64 {
        self.p99_ms
    }
}

/// One operation in a latency batch: the span `name`, its target `profile`, and how many sample
/// spans to emit (pass ≥ Pulse's 50-sample latency exclusion floor to clear the baseline).
#[derive(Debug, Clone, Copy)]
pub struct LatencyOp<'a> {
    /// The operation name stamped on every sample span (the OTLP span `name`).
    pub operation: &'a str,
    /// The target latency profile this operation's spans realize.
    pub profile: LatencyProfile,
    /// Number of sample spans to emit for this operation.
    pub samples: usize,
}

/// Build an OTLP trace export carrying, per operation, `samples` independent root spans named by
/// the operation, each with a seeded duration drawn so the operation's spans realize its target
/// p50/p95/p99. Span identity and the duration sequence are a deterministic function of `seed`;
/// `start`/`end` stamps are wall-clock. Distinct operations carry independent profiles in one batch.
pub fn latency_trace_request(
    service_name: &str,
    seed: u64,
    operations: &[LatencyOp<'_>],
) -> ExportTraceServiceRequest {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut spans: Vec<Span> = Vec::new();
    for op in operations {
        for duration_nanos in sample_durations_nanos(op.profile, op.samples, &mut rng) {
            let trace_id = gen_id::<16>(&mut rng).to_vec();
            let span_id = gen_id::<8>(&mut rng).to_vec();
            spans.push(timed_span(
                op.operation,
                trace_id,
                span_id,
                Vec::new(),
                ok_status(),
                duration_nanos,
            ));
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

/// Draw `n` span durations (nanoseconds) whose realized percentiles converge to `profile`'s
/// targets. Stratified inverse-CDF: stratum `i` draws one seeded uniform in `[i/n, (i+1)/n)` and
/// maps it through the piecewise-linear quantile anchored at the profile percentiles — low
/// discrepancy (tight convergence at ≥50 samples) yet seed-sensitive.
fn sample_durations_nanos(profile: LatencyProfile, n: usize, rng: &mut ChaCha8Rng) -> Vec<u64> {
    (0..n)
        .map(|i| {
            let u = (i as f64 + next_unit(rng)) / n as f64;
            (quantile(profile, u) * NANOS_PER_MS) as u64
        })
        .collect()
}

/// The piecewise-linear quantile (inverse-CDF) in milliseconds for `u` in `[0, 1)`: `0 → 0`,
/// `0.5 → p50`, `0.95 → p95`, `0.99 → p99`, then a bounded extrapolated tail past p99.
fn quantile(profile: LatencyProfile, u: f64) -> f64 {
    let p50 = profile.p50_ms as f64;
    let p95 = profile.p95_ms as f64;
    let p99 = profile.p99_ms as f64;
    let tail = p99 + (p99 - p95) / 4.0;
    if u < 0.5 {
        lerp(0.0, p50, u / 0.5)
    } else if u < 0.95 {
        lerp(p50, p95, (u - 0.5) / 0.45)
    } else if u < 0.99 {
        lerp(p95, p99, (u - 0.95) / 0.04)
    } else {
        lerp(p99, tail, (u - 0.99) / 0.01)
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

    fn profile() -> LatencyProfile {
        LatencyProfile::new(100, 500, 2000).unwrap()
    }

    fn sorted_nanos(profile: LatencyProfile, n: usize, seed: u64) -> Vec<u64> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut d = sample_durations_nanos(profile, n, &mut rng);
        d.sort_unstable();
        d
    }

    /// Interpolated (type-7) percentile in milliseconds over ascending nanos.
    fn percentile_ms(sorted: &[u64], p: f64) -> f64 {
        let rank = p * (sorted.len() as f64 - 1.0);
        let lo = rank.floor() as usize;
        let hi = rank.ceil() as usize;
        let frac = rank - lo as f64;
        (sorted[lo] as f64 + (sorted[hi] as f64 - sorted[lo] as f64) * frac) / NANOS_PER_MS
    }

    fn durations_of(req: &ExportTraceServiceRequest) -> Vec<u64> {
        req.resource_spans[0].scope_spans[0]
            .spans
            .iter()
            .map(|s| s.end_time_unix_nano - s.start_time_unix_nano)
            .collect()
    }

    #[test]
    fn profile_requires_ordered_percentiles() {
        assert!(LatencyProfile::new(100, 500, 2000).is_some());
        assert!(LatencyProfile::new(100, 100, 100).is_some());
        assert!(LatencyProfile::new(500, 100, 2000).is_none());
        assert!(LatencyProfile::new(100, 2000, 500).is_none());
    }

    #[test]
    fn realized_percentiles_converge_to_targets() {
        let d = sorted_nanos(profile(), 2000, 424_242);
        let within = |got: f64, target: f64| (got - target).abs() <= target * 0.12;
        let (p50, p95, p99) = (
            percentile_ms(&d, 0.50),
            percentile_ms(&d, 0.95),
            percentile_ms(&d, 0.99),
        );
        assert!(within(p50, 100.0), "p50 = {p50}");
        assert!(within(p95, 500.0), "p95 = {p95}");
        assert!(within(p99, 2000.0), "p99 = {p99}");
    }

    #[test]
    fn same_seed_reproduces_identical_durations() {
        let mut a = ChaCha8Rng::seed_from_u64(7);
        let mut b = ChaCha8Rng::seed_from_u64(7);
        assert_eq!(
            sample_durations_nanos(profile(), 64, &mut a),
            sample_durations_nanos(profile(), 64, &mut b)
        );
    }

    #[test]
    fn different_seeds_diverge() {
        let mut a = ChaCha8Rng::seed_from_u64(1);
        let mut b = ChaCha8Rng::seed_from_u64(2);
        assert_ne!(
            sample_durations_nanos(profile(), 64, &mut a),
            sample_durations_nanos(profile(), 64, &mut b)
        );
    }

    #[test]
    fn builds_a_span_per_sample_with_nonzero_durations_under_service_name() {
        let ops = [LatencyOp {
            operation: "op",
            profile: profile(),
            samples: 50,
        }];
        let req = latency_trace_request("conductor", 1, &ops);
        let rs = &req.resource_spans[0];
        let attrs = &rs.resource.as_ref().unwrap().attributes;
        assert!(attrs.iter().any(|kv| kv.key == "service.name"));
        let spans = &rs.scope_spans[0].spans;
        assert_eq!(spans.len(), 50);
        assert!(spans.iter().all(|s| s.name == "op"));
        assert!(spans.iter().all(|s| s.end_time_unix_nano >= s.start_time_unix_nano));
        assert!(spans.iter().any(|s| s.end_time_unix_nano > s.start_time_unix_nano));
        assert!(spans
            .iter()
            .all(|s| s.trace_id.len() == 16 && s.span_id.len() == 8 && s.parent_span_id.is_empty()));
    }

    #[test]
    fn distinct_operations_keep_distinct_profiles() {
        let fast = LatencyProfile::new(10, 30, 60).unwrap();
        let slow = LatencyProfile::new(1000, 3000, 6000).unwrap();
        let ops = [
            LatencyOp {
                operation: "cache_get",
                profile: fast,
                samples: 200,
            },
            LatencyOp {
                operation: "db_query",
                profile: slow,
                samples: 200,
            },
        ];
        let req = latency_trace_request("svc", 5, &ops);
        let spans = &req.resource_spans[0].scope_spans[0].spans;
        let median = |name: &str| {
            let mut ds: Vec<u64> = spans
                .iter()
                .filter(|s| s.name == name)
                .map(|s| s.end_time_unix_nano - s.start_time_unix_nano)
                .collect();
            ds.sort_unstable();
            ds[ds.len() / 2]
        };
        assert_eq!(spans.iter().filter(|s| s.name == "cache_get").count(), 200);
        assert_eq!(spans.iter().filter(|s| s.name == "db_query").count(), 200);
        assert!(median("cache_get") < median("db_query"));
    }

    #[test]
    fn request_durations_are_seeded_not_wall_clock() {
        let ops = [LatencyOp {
            operation: "checkout",
            profile: LatencyProfile::new(50, 200, 800).unwrap(),
            samples: 60,
        }];
        let a = latency_trace_request("svc", 99, &ops);
        let b = latency_trace_request("svc", 99, &ops);
        assert_eq!(durations_of(&a), durations_of(&b));
        let c = latency_trace_request("svc", 100, &ops);
        assert_ne!(durations_of(&a), durations_of(&c));
    }
}
