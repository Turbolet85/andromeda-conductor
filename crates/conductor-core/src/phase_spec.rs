//! The declarative per-phase emission spec — what each scenario phase emits, as data.
//!
//! A [`Scenario`](crate::Scenario) carries an ordered list of [`PhaseSpec`]s. Each pairs a legible
//! label and a base inter-phase gap (the timing the `conductor-timeline` scheduler sequences) with
//! an [`EmissionSpec`] describing what the phase emits: how MANY emissions
//! ([`EmissionSpec::occurrences`]) and of what SHAPE ([`EmissionShape`]). The shape variants mirror
//! the `conductor-emit` primitive families one-for-one; the mapping from a shape to its primitive is
//! the dispatcher's job in `conductor-run`, which is why the parameter enums here are core-local
//! mirrors rather than re-exported wire types (`conductor-core` depends on no seam crate).
//!
//! Before this seam the shape a phase intended lived in its NAME (`storm-same-fp-6x`) by a catalog
//! convention; it is declared data now, so garde validates it at load and the emission count is
//! derivable without running the scenario.

use garde::Validate;
use serde::{Deserialize, Serialize};

/// Upper bound on a single phase's base gap (milliseconds) — generous enough for the longest
/// planned phase (the Epoch-4 bursty-train ~10-minute quiet window) yet rejecting absurd values.
pub(crate) const MAX_GAP_MS: u64 = 3_600_000;

/// Upper bound on a scenario's symmetric per-gap jitter (milliseconds).
pub(crate) const MAX_JITTER_MS: u64 = 60_000;

/// Upper bound on a single phase's declared emissions, and on any per-shape sample/window count.
/// Keeps a declared intensity inside the order of magnitude the SUT load envelope records as
/// proven-good (`contracts/pulse-load-envelope.toml` §envelope) — the model cannot express a storm
/// far outside it even though the envelope's rate term is not itself asserted.
pub(crate) const MAX_OCCURRENCES: u32 = 10_000;

/// One phase of a scenario's declarative timeline: a legible label, the base gap before its
/// boundary, and a declarative description of what it emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct PhaseSpec {
    /// Stable, machine-readable phase label (e.g. `"baseline"`). Bounded to 1..=40 chars so it
    /// stays legible as a phase-line / titlebar heading on the downstream surfaces.
    #[garde(length(min = 1, max = 40))]
    pub name: String,
    /// Base inter-phase gap in milliseconds before this phase's boundary (the timeline scheduler
    /// perturbs it by the scenario's seeded jitter). Bounded to reject pathological timings.
    #[garde(range(max = MAX_GAP_MS))]
    pub gap_ms: u64,
    /// What this phase emits, declaratively. Optional in config (defaults to one plain trace).
    #[serde(default)]
    #[garde(dive)]
    pub emission: EmissionSpec,
    /// A fault this phase applies for its window, declaratively (the port-occupier hold). Absent on
    /// every ordinary phase. A fault-declaring phase must also be a silence window — asserted at the
    /// scenario level (`fault_phases_are_silent`), since garde customs are field-level.
    #[serde(default)]
    #[garde(dive)]
    pub fault: Option<FaultSpec>,
}

/// A fault applied for one phase's window — declared data, like the emission spec. The run path
/// drives the named `conductor-faults` helper for exactly the phase's scheduler-held window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct FaultSpec {
    /// Which fault helper this phase applies. Closed set — the enum is the validation.
    #[garde(skip)]
    pub kind: FaultKindSpec,
}

/// The fault helpers a phase may declare (core-local mirror of `conductor-faults`, like the
/// emission-shape mirrors above).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultKindSpec {
    /// The `:4317` port-occupier — Conductor's sole deliberate inbound bind, held for the window
    /// and RAII-released at the phase boundary.
    PortOccupier,
}

/// A fault-declaring phase must be a silence window: while the occupier holds the egress port
/// nothing real listens there, so a declared emission would land in the occupier, never Pulse.
/// Field-level on `Scenario::phases` (garde 0.22.1 has no container-level `custom`) — the same
/// altitude trick as [`shape_is_realizable`], one level up.
pub(crate) fn fault_phases_are_silent(phases: &[PhaseSpec], _ctx: &()) -> garde::Result {
    for phase in phases {
        if phase.fault.is_some() && phase.emission.occurrences != 0 {
            return Err(garde::Error::new(
                "a fault-declaring phase must be a silence window (occurrences = 0)",
            ));
        }
    }
    Ok(())
}

/// A declarative description of a phase's emission: how many, and of what shape.
///
/// `#[non_exhaustive]` so a later seam can add detail without breaking the config shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[non_exhaustive]
pub struct EmissionSpec {
    /// The OTLP signal class this phase emits. Authoritative for [`EmissionShape::Plain`]; every
    /// other shape implies its own signal (a severity mix is logs, a latency profile is traces).
    #[garde(skip)]
    #[serde(default)]
    pub signal: Signal,
    /// How many times this phase emits, paced evenly across its gap. `0` declares a deliberate
    /// silence window (the gap still elapses), which is how the activity-floor family expresses a
    /// quiet phase without reaching for a fault helper.
    #[garde(range(max = MAX_OCCURRENCES))]
    #[serde(default = "default_occurrences")]
    pub occurrences: u32,
    /// The shape of each emission — selects the `conductor-emit` primitive family and carries its
    /// parameters. Flattened, so a phase declares `kind = "exception"` inline.
    #[garde(custom(shape_is_realizable))]
    #[serde(flatten, default)]
    pub shape: EmissionShape,
}

fn default_occurrences() -> u32 {
    1
}

impl Default for EmissionSpec {
    fn default() -> Self {
        Self { signal: Signal::default(), occurrences: 1, shape: EmissionShape::default() }
    }
}

impl EmissionSpec {
    /// Construct a plain single-emission spec for a given signal class.
    pub fn new(signal: Signal) -> Self {
        Self { signal, ..Self::default() }
    }

    /// Construct a spec emitting `occurrences` times with `shape`.
    pub fn shaped(signal: Signal, occurrences: u32, shape: EmissionShape) -> Self {
        Self { signal, occurrences, shape }
    }
}

/// The shape of a phase's emissions — one variant per `conductor-emit` primitive family.
///
/// Core-local by design: these are config mirrors of the wire builders' parameters, not the wire
/// types themselves, because `conductor-core` sits below every seam crate. `conductor-run`'s
/// dispatcher maps each variant onto its primitive.
// Deliberately NOT `#[non_exhaustive]`: the dispatcher in `conductor-run` must carry an arm per
// shape, and a wildcard arm would let a newly-added shape emit nothing at all. Exhaustive matching
// is the compiler-enforced link between a declarable shape and a primitive that realizes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EmissionShape {
    /// An unremarkable OK span (or log record, when the signal is logs) — the baseline traffic a
    /// phase emits when it is not exercising a specific capability.
    #[default]
    Plain,
    /// Spans carrying `Status.Code=ERROR`, `error_percent` of them per emission batch, placed at
    /// `depth` in a root→child chain (`0` = the root span itself).
    Error {
        #[serde(default)]
        depth: u32,
        error_percent: u32,
    },
    /// Exception span events whose fingerprint relationship to the base is controlled by the
    /// variant mix, cycled across the phase's occurrences.
    Exception { variants: Vec<FingerprintVariantSpec> },
    /// Log records at the given OTel `SeverityNumber`s (`1..=24`), cycled across the occurrences —
    /// the WARN(13-16)→ERROR(17-20) boundary is expressed by including 16 and 17.
    Severity { severities: Vec<u32> },
    /// Sample spans for one operation realizing a target p50/p95/p99 latency profile.
    Latency { operation: String, p50_ms: u64, p95_ms: u64, p99_ms: u64, samples: u32 },
    /// Synthetic PII values across the named categories, embedded in spans or log records.
    Pii { categories: Vec<PiiCategorySpec> },
    /// A linear traffic ramp from `from_rate` to `to_rate` spans per window across `windows`.
    Ramp { from_rate: u32, to_rate: u32, windows: u32 },
    /// A sinusoidal traffic oscillation of `amplitude` around `center_rate` — the halo-breathing
    /// curve. `amplitude < center_rate`, so the trough stays positive.
    Breathing { center_rate: u32, amplitude: u32, period_windows: u32, windows: u32 },
    /// A multi-service trace: one `ResourceSpans` per service joined by a shared trace id, with an
    /// optional error placed at `error_depth`.
    Topology {
        services: Vec<String>,
        #[serde(default)]
        error_depth: Option<u32>,
    },
}

/// A controlled transformation of the base exception, driving its fingerprint into a known
/// relationship with the base (mirrors `conductor_emit::FingerprintVariant`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FingerprintVariantSpec {
    /// Identical to the base ⇒ same fingerprint.
    Identical,
    /// Different source path below an unchanged leading segment ⇒ different fingerprint —
    /// normalization preserves a relative path in full, so it is significant at every depth.
    Path,
    /// Different LEADING path segment ⇒ different fingerprint. Differs from [`Self::Path`] only in
    /// where the path changes.
    RelativePath,
    /// Different source line ⇒ same fingerprint (line-insensitive).
    Line,
    /// Different exception type ⇒ different fingerprint.
    Type,
    /// Different stack-frame function ⇒ different fingerprint.
    Frame,
}

/// One of the seven P-047 PII categories (mirrors `conductor_emit::PiiCategory`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PiiCategorySpec {
    /// An email address.
    Email,
    /// A JSON Web Token.
    Jwt,
    /// An HTTP `Bearer` authorization token.
    Bearer,
    /// A provider-style secret API key.
    ApiKey,
    /// A Luhn-valid credit-card PAN.
    CreditCard,
    /// A US Social Security Number.
    Ssn,
    /// A secret-like `key=value` pair.
    SecretKeyValue,
}

/// Reject a shape whose parameters no primitive could realize. Field-level (garde 0.22.1 has no
/// container-level `custom`), so every cross-field invariant a shape carries — percentile ordering,
/// the breathing sign guard, a distinct service pair — is asserted here at load rather than
/// surfacing as a panic or a silently-degraded stream at emission time.
fn shape_is_realizable(shape: &EmissionShape, _ctx: &()) -> garde::Result {
    let bad = |m: &str| Err(garde::Error::new(m));
    match shape {
        EmissionShape::Plain => Ok(()),
        EmissionShape::Error { error_percent, .. } => {
            if *error_percent > 100 {
                return bad("error_percent must be in 0..=100");
            }
            Ok(())
        }
        EmissionShape::Exception { variants } => {
            if variants.is_empty() {
                return bad("exception shape needs at least one fingerprint variant");
            }
            Ok(())
        }
        EmissionShape::Severity { severities } => {
            if severities.is_empty() {
                return bad("severity shape needs at least one severity number");
            }
            if !severities.iter().all(|s| (1..=24).contains(s)) {
                return bad("severity numbers must be in 1..=24");
            }
            Ok(())
        }
        EmissionShape::Latency { p50_ms, p95_ms, p99_ms, samples, .. } => {
            if !(p50_ms <= p95_ms && p95_ms <= p99_ms) {
                return bad("latency percentiles must be ordered p50 <= p95 <= p99");
            }
            if *samples == 0 || *samples > MAX_OCCURRENCES {
                return bad("latency samples must be in 1..=10000");
            }
            Ok(())
        }
        EmissionShape::Pii { categories } => {
            if categories.is_empty() {
                return bad("pii shape needs at least one category");
            }
            Ok(())
        }
        EmissionShape::Ramp { windows, .. } => {
            if *windows == 0 || *windows > MAX_OCCURRENCES {
                return bad("ramp windows must be in 1..=10000");
            }
            Ok(())
        }
        EmissionShape::Breathing { center_rate, amplitude, period_windows, windows } => {
            if *windows == 0 || *windows > MAX_OCCURRENCES {
                return bad("breathing windows must be in 1..=10000");
            }
            if *period_windows == 0 {
                return bad("breathing period_windows must be non-zero");
            }
            if amplitude >= center_rate {
                return bad("breathing amplitude must stay below center_rate");
            }
            Ok(())
        }
        EmissionShape::Topology { services, .. } => {
            if services.len() < 2 {
                return bad("topology needs at least two services");
            }
            if services.iter().enumerate().any(|(i, s)| services[i + 1..].contains(s)) {
                return bad("topology services must be distinct");
            }
            Ok(())
        }
    }
}

/// The OTLP signal class a phase emits — the three opentelemetry-proto signal types. Closed and
/// low-cardinality, so it is safe as an observability span/field value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Signal {
    /// Trace spans (the default — the early fault scenarios emit error spans).
    #[default]
    Traces,
    /// Metric data points.
    Metrics,
    /// Log records.
    Logs,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn phase(name: &str, gap_ms: u64) -> PhaseSpec {
        PhaseSpec {
            name: name.to_string(),
            gap_ms,
            emission: EmissionSpec::default(),
            fault: None,
        }
    }

    #[test]
    fn well_formed_phase_validates() {
        assert!(phase("baseline", 2000).validate().is_ok());
    }

    #[test]
    fn empty_phase_name_is_rejected() {
        assert!(phase("", 2000).validate().is_err());
    }

    #[test]
    fn phase_name_length_boundary() {
        assert!(phase(&"x".repeat(40), 2000).validate().is_ok());
        assert!(phase(&"x".repeat(41), 2000).validate().is_err());
    }

    #[test]
    fn gap_bound_is_enforced() {
        assert!(phase("baseline", MAX_GAP_MS).validate().is_ok());
        assert!(phase("baseline", MAX_GAP_MS + 1).validate().is_err());
    }

    #[test]
    fn signal_serializes_snake_case() {
        assert_eq!(serde_json::to_string(&Signal::Traces).unwrap(), "\"traces\"");
        assert_eq!(serde_json::to_string(&Signal::Metrics).unwrap(), "\"metrics\"");
        assert_eq!(serde_json::to_string(&Signal::Logs).unwrap(), "\"logs\"");
    }

    #[test]
    fn emission_defaults_to_traces() {
        assert_eq!(EmissionSpec::default().signal, Signal::Traces);
        assert_eq!(EmissionSpec::new(Signal::Logs).signal, Signal::Logs);
    }

    #[test]
    fn emission_defaults_to_one_plain_occurrence() {
        let spec = EmissionSpec::default();
        assert_eq!(spec.occurrences, 1);
        assert_eq!(spec.shape, EmissionShape::Plain);
    }

    fn shaped(occurrences: u32, shape: EmissionShape) -> PhaseSpec {
        PhaseSpec {
            name: "under-test".to_string(),
            gap_ms: 2000,
            emission: EmissionSpec::shaped(Signal::Traces, occurrences, shape),
            fault: None,
        }
    }

    #[test]
    fn occurrence_bound_is_enforced() {
        assert!(shaped(0, EmissionShape::Plain).validate().is_ok(), "zero is a silence window");
        assert!(shaped(MAX_OCCURRENCES, EmissionShape::Plain).validate().is_ok());
        assert!(shaped(MAX_OCCURRENCES + 1, EmissionShape::Plain).validate().is_err());
    }

    /// The nested rules only run because `PhaseSpec.emission` dives — a `skip` there would let every
    /// shape invariant below pass unchecked.
    #[test]
    fn well_formed_shapes_validate() {
        let ok = [
            EmissionShape::Error { depth: 2, error_percent: 100 },
            EmissionShape::Exception { variants: vec![FingerprintVariantSpec::Identical] },
            EmissionShape::Severity { severities: vec![1, 17, 24] },
            EmissionShape::Latency {
                operation: "checkout".to_string(),
                p50_ms: 100,
                p95_ms: 100,
                p99_ms: 100,
                samples: 50,
            },
            EmissionShape::Pii { categories: vec![PiiCategorySpec::Email] },
            EmissionShape::Ramp { from_rate: 0, to_rate: 50, windows: 1 },
            EmissionShape::Breathing {
                center_rate: 100,
                amplitude: 99,
                period_windows: 1,
                windows: 1,
            },
            EmissionShape::Topology {
                services: vec!["a".to_string(), "b".to_string()],
                error_depth: None,
            },
        ];
        for shape in ok {
            assert!(shaped(1, shape.clone()).validate().is_ok(), "{shape:?} should validate");
        }
    }

    #[test]
    fn unrealizable_shapes_are_rejected_at_load() {
        let bad = [
            EmissionShape::Error { depth: 0, error_percent: 101 },
            EmissionShape::Exception { variants: Vec::new() },
            EmissionShape::Severity { severities: Vec::new() },
            EmissionShape::Severity { severities: vec![0] },
            EmissionShape::Severity { severities: vec![25] },
            // inverted percentiles
            EmissionShape::Latency {
                operation: "op".to_string(),
                p50_ms: 500,
                p95_ms: 100,
                p99_ms: 2000,
                samples: 50,
            },
            EmissionShape::Latency {
                operation: "op".to_string(),
                p50_ms: 1,
                p95_ms: 2,
                p99_ms: 3,
                samples: 0,
            },
            EmissionShape::Pii { categories: Vec::new() },
            EmissionShape::Ramp { from_rate: 1, to_rate: 2, windows: 0 },
            // the sign guard: amplitude at or above center would drive the trough negative
            EmissionShape::Breathing {
                center_rate: 100,
                amplitude: 100,
                period_windows: 8,
                windows: 8,
            },
            EmissionShape::Breathing {
                center_rate: 100,
                amplitude: 10,
                period_windows: 0,
                windows: 8,
            },
            EmissionShape::Topology { services: vec!["a".to_string()], error_depth: None },
            EmissionShape::Topology {
                services: vec!["a".to_string(), "a".to_string()],
                error_depth: None,
            },
        ];
        for shape in bad {
            assert!(shaped(1, shape.clone()).validate().is_err(), "{shape:?} should be rejected");
        }
    }

    #[test]
    fn a_shape_round_trips_through_its_flattened_toml_form() {
        let toml = r#"
            name = "storm"
            gap_ms = 12000
            [emission]
            kind = "exception"
            occurrences = 6
            variants = ["identical", "path", "line"]
        "#;
        let spec: PhaseSpec = toml::from_str(toml).expect("parses");
        assert_eq!(spec.emission.occurrences, 6);
        assert_eq!(
            spec.emission.shape,
            EmissionShape::Exception {
                variants: vec![
                    FingerprintVariantSpec::Identical,
                    FingerprintVariantSpec::Path,
                    FingerprintVariantSpec::Line,
                ],
            }
        );
        assert!(spec.validate().is_ok());
    }

    #[test]
    fn a_phase_with_no_emission_table_keeps_the_default() {
        let spec: PhaseSpec =
            toml::from_str("name = \"baseline\"\ngap_ms = 2000\n").expect("parses");
        assert_eq!(spec.emission, EmissionSpec::default());
        assert_eq!(spec.fault, None, "no fault table declares no fault");
    }

    #[test]
    fn a_fault_table_parses_to_the_declared_kind() {
        let toml = r#"
            name = "port-held"
            gap_ms = 40000
            [emission]
            kind = "plain"
            occurrences = 0
            [fault]
            kind = "port_occupier"
        "#;
        let spec: PhaseSpec = toml::from_str(toml).expect("parses");
        assert_eq!(spec.fault, Some(FaultSpec { kind: FaultKindSpec::PortOccupier }));
        assert!(spec.validate().is_ok());
    }

    #[test]
    fn an_unknown_fault_kind_is_rejected_at_parse() {
        let toml = r#"
            name = "port-held"
            gap_ms = 40000
            [fault]
            kind = "chaos_monkey"
        "#;
        assert!(toml::from_str::<PhaseSpec>(toml).is_err());
    }

    #[test]
    fn a_fault_phase_that_emits_is_rejected_by_the_silence_invariant() {
        let mut emitting = phase("port-held", 40000);
        emitting.fault = Some(FaultSpec { kind: FaultKindSpec::PortOccupier });
        assert_eq!(emitting.emission.occurrences, 1, "the default emission is one plain trace");
        assert!(fault_phases_are_silent(std::slice::from_ref(&emitting), &()).is_err());

        emitting.emission.occurrences = 0;
        assert!(fault_phases_are_silent(std::slice::from_ref(&emitting), &()).is_ok());
        assert!(fault_phases_are_silent(&[], &()).is_ok(), "no phases, nothing to violate");
    }
}
