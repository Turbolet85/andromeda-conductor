//! The declarative per-phase emission spec — what each scenario phase emits, as data.
//!
//! A [`Scenario`](crate::Scenario) carries an ordered list of [`PhaseSpec`]s. Each pairs a legible
//! label and a base inter-phase gap (the timing the `conductor-timeline` scheduler sequences) with
//! an [`EmissionSpec`] describing what the phase emits. The emission descriptor is declarative data
//! only — the concrete OTLP taxonomy (severity boundaries, fingerprint identity, latency targets,
//! ramps) lands in the Epoch-3 emission seam, which extends [`EmissionSpec`] (it is
//! `#[non_exhaustive]`) without reshaping [`PhaseSpec`] or the scenario model.

use garde::Validate;
use serde::{Deserialize, Serialize};

/// Upper bound on a single phase's base gap (milliseconds) — generous enough for the longest
/// planned phase (the Epoch-4 bursty-train ~10-minute quiet window) yet rejecting absurd values.
pub(crate) const MAX_GAP_MS: u64 = 3_600_000;

/// Upper bound on a scenario's symmetric per-gap jitter (milliseconds).
pub(crate) const MAX_JITTER_MS: u64 = 60_000;

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
    /// What this phase emits, declaratively. Optional in config (defaults to traces); the Epoch-3
    /// emission seam fills in the concrete taxonomy.
    #[serde(default)]
    #[garde(skip)]
    pub emission: EmissionSpec,
}

/// A declarative, forward-compatible description of a phase's emission.
///
/// Minimal by design: it records only the OTLP signal class the phase exercises. `#[non_exhaustive]`
/// so the Epoch-3 emission seam can add per-signal detail without breaking the config shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, Default)]
#[non_exhaustive]
pub struct EmissionSpec {
    /// The OTLP signal class this phase emits.
    #[garde(skip)]
    pub signal: Signal,
}

impl EmissionSpec {
    /// Construct an emission spec for a given signal class.
    pub fn new(signal: Signal) -> Self {
        Self { signal }
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
}
