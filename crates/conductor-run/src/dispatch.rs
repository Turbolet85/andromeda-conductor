//! The per-phase emission dispatcher: a scenario's declared shape → the `conductor-emit` primitive
//! that realizes it.
//!
//! This is the seam that replaced `coarse_emit`, which sent one signal per phase boundary after the
//! whole timeline had already elapsed. The dispatcher is driven by `conductor-timeline`'s per-phase
//! hook instead, so a phase's declared occurrences land INSIDE that phase's window — the shape a
//! windowed SUT detector (an N-in-30s storm cue, a silence floor) actually observes.
//!
//! Every emission's identity derives from a pure mix of the scenario seed with the phase ordinal and
//! occurrence, so the stream is a deterministic function of the seed with no RNG state of its own
//! (architecture §Established Decisions — Determinism RNG).

use conductor_core::{EmissionShape, FingerprintVariantSpec, PiiCategorySpec, Scenario, Signal};
use conductor_emit::{
    DEFAULT_SERVICE_NAME, EmitError, ErrorPlacement, ExceptionSpec, FingerprintVariant, Frame,
    LatencyOp, LatencyProfile, LogsEmitter, PiiCategory, PiiCorpus, RateCurve, ServiceTopology,
    Severity, TraceEmitter, error_trace_request, exception_trace_request, latency_trace_request,
    pii_logs_request, pii_trace_request, rate_trace_request, service_topology_request,
    severity_logs_request, trace_request,
};
use conductor_timeline::EmissionPoint;

/// `SeverityNumber` a plain logs-signal phase emits — INFO, the benign baseline record.
const PLAIN_LOG_SEVERITY: i32 = 9;

/// Benign fixed error message — never a host path or struct name (security-plan §Error Handling).
const ERROR_MESSAGE: &str = "conductor synthetic fault";

/// A dispatch fault — a harness error, never a verification outcome (the verdict/error wall).
#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    /// The OTLP egress leg failed.
    #[error(transparent)]
    Emit(#[from] EmitError),
    /// The timeline surfaced a phase the scenario does not carry — an internal invariant break.
    #[error("phase {0} is not in the scenario")]
    UnknownPhase(usize),
    /// A declared shape no primitive could build. garde rejects these at load, so reaching this is
    /// a validation gap rather than an authoring error — it stays a typed fault, never a panic.
    #[error("phase {phase} declares an unrealizable shape: {reason}")]
    Unrealizable {
        /// Zero-based ordinal of the offending phase.
        phase: usize,
        /// Which parameter could not be realized.
        reason: &'static str,
    },
}

/// Holds the egress clients for one scenario run and turns each [`EmissionPoint`] into OTLP.
pub struct Dispatcher<'a> {
    scenario: &'a Scenario,
    endpoint: &'a str,
    traces: TraceEmitter,
    logs: Option<LogsEmitter>,
}

impl<'a> Dispatcher<'a> {
    /// Connect the trace egress for `scenario`. The logs client stays unconnected until a
    /// logs-emitting phase is first reached (the laziness `coarse_emit` established).
    pub async fn connect(scenario: &'a Scenario, endpoint: &'a str) -> Result<Self, EmitError> {
        Ok(Self { scenario, endpoint, traces: TraceEmitter::connect(endpoint).await?, logs: None })
    }

    /// Emit the one signal `point` declares.
    pub async fn dispatch(&mut self, point: EmissionPoint) -> Result<(), DispatchError> {
        let index = point.phase_index;
        let phase = self.scenario.phases.get(index).ok_or(DispatchError::UnknownPhase(index))?;
        let emission = &phase.emission;
        let seed = emission_seed(self.scenario.seed, index, point.occurrence);
        let unrealizable =
            |reason: &'static str| DispatchError::Unrealizable { phase: index, reason };

        match &emission.shape {
            EmissionShape::Plain => match emission.signal {
                Signal::Logs => {
                    let severity = Severity::new(PLAIN_LOG_SEVERITY)
                        .ok_or_else(|| unrealizable("plain log severity"))?;
                    let request = severity_logs_request(DEFAULT_SERVICE_NAME, &[severity]);
                    self.logs().await?.export(request).await?;
                }
                Signal::Traces | Signal::Metrics => {
                    self.traces
                        .export(trace_request(DEFAULT_SERVICE_NAME, seed, &phase.name))
                        .await?;
                }
            },
            EmissionShape::Error { depth, error_percent } => {
                let request = if is_error_occurrence(point.occurrence, emission.occurrences, *error_percent)
                {
                    let placement = if *depth == 0 {
                        ErrorPlacement::Root
                    } else {
                        ErrorPlacement::DeepChild { depth: *depth as usize }
                    };
                    error_trace_request(DEFAULT_SERVICE_NAME, seed, placement, ERROR_MESSAGE)
                } else {
                    trace_request(DEFAULT_SERVICE_NAME, seed, &phase.name)
                };
                self.traces.export(request).await?;
            }
            EmissionShape::Exception { variants } => {
                let variant = variants[point.occurrence as usize % variants.len()];
                let spec = wire_variant(variant).derive(&base_exception());
                self.traces
                    .export(exception_trace_request(DEFAULT_SERVICE_NAME, seed, &spec))
                    .await?;
            }
            EmissionShape::Severity { severities } => {
                let number = severities[point.occurrence as usize % severities.len()];
                let severity = Severity::new(number as i32)
                    .ok_or_else(|| unrealizable("severity number out of 1..=24"))?;
                let request = severity_logs_request(DEFAULT_SERVICE_NAME, &[severity]);
                self.logs().await?.export(request).await?;
            }
            EmissionShape::Latency { operation, p50_ms, p95_ms, p99_ms, samples } => {
                let profile = LatencyProfile::new(*p50_ms, *p95_ms, *p99_ms)
                    .ok_or_else(|| unrealizable("latency percentiles out of order"))?;
                let ops =
                    [LatencyOp { operation: operation.as_str(), profile, samples: *samples as usize }];
                self.traces
                    .export(latency_trace_request(DEFAULT_SERVICE_NAME, seed, &ops))
                    .await?;
            }
            EmissionShape::Pii { categories } => {
                let corpus = PiiCorpus::seeded(seed);
                let wire: Vec<PiiCategory> = categories.iter().map(|c| wire_category(*c)).collect();
                match emission.signal {
                    Signal::Logs => {
                        let request = pii_logs_request(DEFAULT_SERVICE_NAME, &corpus, &wire);
                        self.logs().await?.export(request).await?;
                    }
                    Signal::Traces | Signal::Metrics => {
                        self.traces
                            .export(pii_trace_request(DEFAULT_SERVICE_NAME, &corpus, &wire))
                            .await?;
                    }
                }
            }
            EmissionShape::Ramp { from_rate, to_rate, windows } => {
                let curve = RateCurve::ramp(*from_rate, *to_rate, *windows as usize)
                    .ok_or_else(|| unrealizable("ramp windows must be non-zero"))?;
                self.traces
                    .export(rate_trace_request(DEFAULT_SERVICE_NAME, seed, &curve, &phase.name))
                    .await?;
            }
            EmissionShape::Breathing { center_rate, amplitude, period_windows, windows } => {
                let curve = RateCurve::breathing(
                    *center_rate,
                    *amplitude,
                    *period_windows as usize,
                    *windows as usize,
                )
                .ok_or_else(|| unrealizable("breathing curve bounds"))?;
                self.traces
                    .export(rate_trace_request(DEFAULT_SERVICE_NAME, seed, &curve, &phase.name))
                    .await?;
            }
            EmissionShape::Topology { services, error_depth } => {
                let names: Vec<&str> = services.iter().map(String::as_str).collect();
                let topology = ServiceTopology::new(&names)
                    .ok_or_else(|| unrealizable("topology needs >=2 distinct services"))?;
                let placement = error_depth.map(|d| {
                    if d == 0 {
                        ErrorPlacement::Root
                    } else {
                        ErrorPlacement::DeepChild { depth: d as usize }
                    }
                });
                self.traces
                    .export(service_topology_request(&topology, seed, placement, ERROR_MESSAGE))
                    .await?;
            }
        }
        Ok(())
    }

    /// The logs egress, connected on first use — a scenario that never emits a log record never
    /// opens a second channel. Take-then-insert so the reference is always valid without an
    /// unreachable branch.
    async fn logs(&mut self) -> Result<&mut LogsEmitter, EmitError> {
        let emitter = match self.logs.take() {
            Some(emitter) => emitter,
            None => LogsEmitter::connect(self.endpoint).await?,
        };
        Ok(self.logs.insert(emitter))
    }
}

/// Whether occurrence `i` of `total` carries the error, realizing `percent` of the phase's emissions
/// as errors — a pure positional rule, so the same declaration always yields the same error slots.
fn is_error_occurrence(i: u32, total: u32, percent: u32) -> bool {
    u64::from(i) * 100 < u64::from(percent) * u64::from(total.max(1))
}

/// A per-emission seed: a pure mix of the scenario seed with the phase ordinal and occurrence, so
/// every emission has distinct identity while the whole stream stays a function of `seed` alone.
fn emission_seed(seed: u64, phase_index: usize, occurrence: u32) -> u64 {
    seed.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        .wrapping_add((phase_index as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9))
        .wrapping_add(u64::from(occurrence).wrapping_mul(0x94d0_49bb_1331_11eb))
}

/// The canonical synthetic exception every fingerprint variant is derived from. Fixed content, so
/// the fingerprint is stable across runs and hosts; frame paths are relative by construction.
fn base_exception() -> ExceptionSpec {
    ExceptionSpec::new(
        "ValueError",
        "conductor synthetic exception",
        vec![
            Frame::new("conductor::worker::handle", "src/worker.rs", 42),
            Frame::new("conductor::worker::parse", "src/worker.rs", 17),
        ],
    )
}

fn wire_variant(spec: FingerprintVariantSpec) -> FingerprintVariant {
    match spec {
        FingerprintVariantSpec::Identical => FingerprintVariant::Identical,
        FingerprintVariantSpec::Path => FingerprintVariant::PathVariant,
        FingerprintVariantSpec::Line => FingerprintVariant::LineVariant,
        FingerprintVariantSpec::Type => FingerprintVariant::TypeVariant,
        FingerprintVariantSpec::Frame => FingerprintVariant::FrameVariant,
    }
}

fn wire_category(spec: PiiCategorySpec) -> PiiCategory {
    match spec {
        PiiCategorySpec::Email => PiiCategory::Email,
        PiiCategorySpec::Jwt => PiiCategory::Jwt,
        PiiCategorySpec::Bearer => PiiCategory::Bearer,
        PiiCategorySpec::ApiKey => PiiCategory::ApiKey,
        PiiCategorySpec::CreditCard => PiiCategory::CreditCard,
        PiiCategorySpec::Ssn => PiiCategory::Ssn,
        PiiCategorySpec::SecretKeyValue => PiiCategory::SecretKeyValue,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_occurrences_realize_the_declared_percentage() {
        let errors = |percent: u32, total: u32| {
            (0..total).filter(|i| is_error_occurrence(*i, total, percent)).count()
        };
        assert_eq!(errors(0, 10), 0);
        assert_eq!(errors(100, 10), 10);
        assert_eq!(errors(30, 10), 3);
        assert_eq!(errors(50, 4), 2);
    }

    #[test]
    fn emission_seed_is_distinct_per_slot_and_stable_per_input() {
        assert_eq!(emission_seed(7, 0, 0), emission_seed(7, 0, 0));
        assert_ne!(emission_seed(7, 0, 0), emission_seed(7, 0, 1));
        assert_ne!(emission_seed(7, 0, 0), emission_seed(7, 1, 0));
        assert_ne!(emission_seed(7, 0, 0), emission_seed(8, 0, 0));
    }

    #[test]
    fn the_base_exception_carries_no_absolute_host_path() {
        for frame in &base_exception().frames {
            assert!(!frame.file.contains(":\\"), "{}", frame.file);
            assert!(!frame.file.starts_with('/'), "{}", frame.file);
        }
    }
}
