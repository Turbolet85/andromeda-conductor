//! The pinned SUT load envelope — the proven-good storm bounds a scenario must stay inside
//! (arch §Occupied Resources; intent §Theme 1 F4).
//!
//! Pulse's DuckDB append path stalls under sustained high-rate storm: ingest keeps receiving while
//! the append stops, so read-back returns zero rows until restart. Conductor's method IS sustained
//! shaped load, so the harness can induce that stall itself — which means a zero-row read-back from
//! an over-envelope run says nothing about Pulse. This module is the attribution boundary.
//!
//! Two surfaces, on different axes. [`check_load_envelope`] is the static gate: every committed
//! scenario sits inside the envelope's sustained terms or is an exact-set pinned exemption — a harness
//! fault ([`CoreError::LoadEnvelope`]) when not, mirroring [`check_sut_drift`](crate::check_sut_drift).
//! [`LoadEnvelope::classify`] is the per-run judgment, returning an [`EnvelopeStatus`] **value** on
//! the `Ok` path — never an error, because "we drove the SUT too hard" is an outcome about the run,
//! not a Conductor failure (arch §Cross-cutting Patterns "Verdict/error wall"). Both read the same
//! `phase_breach`, so the gate and the run-level caption can never mean different things.
//!
//! **What is asserted, and why it changed.** The two sustained terms are now the asserted ones,
//! judged PER EMITTING PHASE: no phase declaring occurrences may run longer than
//! `max_sustained_storm_ms`, nor emit faster than `max_sustained_rate_spans_per_s`.
//! `EmissionSpec::occurrences` is what made both computable. Whole-scenario duration is recorded but
//! no longer asserted.
//!
//! The rate term counts **wire records, not dispatches**. A dispatch is not one span — a latency
//! profile emits `samples`, a rate curve its per-window counts, a topology one span per service —
//! so the phase rate is `occurrences × EmissionSpec::max_spans_per_dispatch()` over `gap_ms`.
//! Counting dispatches read as bounding wire load while bounding something ~100× smaller: the
//! catalog's true peak is `halo-breathing-encoding` at ~232 records/s, which a dispatch count put at
//! 2/s. It is an upper BOUND because the rate curves' per-window counts carry seeded jitter and the
//! static catalog gate has no seed.
//!
//! The artifact predicted a different landing — that asserting SUMMED emitting-phase duration would
//! let both exemptions retire on their own merits. Measured across the committed catalog it does not:
//! `activity-floor` sums to 900s of emitting time and `incident-auto-resolution` to 610s, so both
//! stay over a 600s ceiling and the summed re-scope changes no verdict. The cause is that summing
//! disjoint bursts is not *sustained* — `activity-floor`'s three 300s bursts are separated by 10-minute
//! quiets, which is exactly what its exemption reason said made it idle. Bounding the longest single
//! emitting window instead matches the stall the envelope actually describes, and under it every
//! catalog scenario passes unaided (longest emitting phase 600s; peak rate 4/s against 10000/s), so
//! the exemption ledger is empty rather than merely smaller.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{CoreError, Scenario};

/// The versioned proven-good load envelope: which Pulse release it describes, where its terms came
/// from, the bounds themselves, and the scenarios exempt from the asserted bound.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LoadEnvelope {
    /// The Pulse release these bounds were captured for (e.g. `"v0.3.0"`).
    pub sut_version: String,
    /// The capture date, `YYYY-MM-DD`.
    pub captured_at: String,
    /// Where the terms came from. They are transcribed from the SUT's record, never measured by
    /// Conductor, and the artifact says so — a reader must be able to tell how far to trust them.
    pub provenance: String,
    /// The bounds themselves.
    pub envelope: EnvelopeTerms,
    /// Scenarios over the asserted bound whose duration is deliberately idle.
    #[serde(default)]
    pub exempt: Vec<Exemption>,
}

/// The envelope's bounds. The two sustained terms are asserted per emitting phase; the whole-scenario
/// duration is recorded but no longer read by a gate.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EnvelopeTerms {
    /// Sustained emission rate the SUT tolerates, in WIRE RECORDS per second. **Asserted** per
    /// emitting phase: a phase's rate is its `occurrences × EmissionSpec::max_spans_per_dispatch()`
    /// over its `gap_ms` — the per-dispatch record count is what makes the term mean what it is
    /// named, since a dispatch may put anything from one span to a whole rate curve on the wire.
    pub max_sustained_rate_spans_per_s: u64,
    /// How long the SUT tolerates that rate. **Asserted** against the longest single emitting phase —
    /// one continuous window, because a storm interrupted by quiet is not a sustained one.
    pub max_sustained_storm_ms: u64,
    /// A scenario's total declared duration. **Recorded, not asserted**: elapsed time is not storm
    /// time, and summing bursts separated by quiet describes no load the SUT ever sees.
    pub max_scenario_duration_ms: u64,
}

/// One pinned exemption: a scenario whose storm bounds are breached for a recorded reason. The
/// committed ledger is empty — every catalog scenario passes the asserted terms on its own merits.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Exemption {
    /// The exempt scenario's name, matching its `name` field in `scenarios/*.toml`.
    pub scenario: String,
    /// Why its duration is not sustained storm. A reason is mandatory: an exemption without one is
    /// indistinguishable from a silent widening.
    pub reason: String,
}

/// A run's standing against the load envelope — a run-level qualifier, deliberately NOT a sixth
/// [`ReportState`](crate::ReportState).
///
/// The state set stays closed at five: this qualifier is orthogonal to every per-check verdict and
/// never alters one. It answers a different question — not "did the SUT behave?" but "was this run
/// capable of telling us?" — so it rides the run, not the check (arch §Standard Contracts).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "classification", content = "cause")]
pub enum EnvelopeStatus {
    /// The run stayed inside the proven-good envelope; its read-back is evidence about the SUT.
    InEnvelope,
    /// The run drove the SUT past its proven-good envelope, so a zero-row or degraded read-back is
    /// attributable to the harness, not to Pulse. Distinct from `Fail` — nothing was disproven.
    EnvironmentSuspect(String),
}

impl EnvelopeStatus {
    /// The stable wire/label form — the always-rendered text that carries the signal when color is
    /// stripped (design-system §Iconography: status is never color-alone).
    pub fn label(&self) -> &'static str {
        match self {
            Self::InEnvelope => "IN-ENVELOPE",
            Self::EnvironmentSuspect(_) => "ENVIRONMENT-SUSPECT",
        }
    }

    /// The breach cause, when there is one.
    pub fn cause(&self) -> Option<&str> {
        match self {
            Self::InEnvelope => None,
            Self::EnvironmentSuspect(cause) => Some(cause),
        }
    }

    /// Whether this run's read-back can be read as evidence about the SUT.
    pub fn is_suspect(&self) -> bool {
        matches!(self, Self::EnvironmentSuspect(_))
    }
}

/// A scenario's total declared duration: the sum of its per-phase base gaps. Saturating, so a
/// pathological config cannot wrap (garde already bounds each gap). Recorded for reporting; the
/// envelope no longer asserts against it (see [`EnvelopeTerms::max_scenario_duration_ms`]).
pub fn scenario_duration_ms(scenario: &Scenario) -> u64 {
    scenario
        .phases
        .iter()
        .fold(0u64, |acc, p| acc.saturating_add(p.gap_ms))
}

/// A scenario's sustained-storm window: the longest single EMITTING phase. A phase declaring zero
/// occurrences is quiet, not storm, so it never contributes — which is why disjoint bursts separated
/// by quiet do not accumulate into one.
pub fn sustained_storm_ms(scenario: &Scenario) -> u64 {
    scenario
        .phases
        .iter()
        .filter(|p| p.emission.occurrences > 0)
        .map(|p| p.gap_ms)
        .max()
        .unwrap_or(0)
}

/// Which envelope term an emitting phase breaks. Named so the gate and the per-run judgment can
/// never describe the same breach differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BreachTerm {
    Storm,
    Rate,
}

/// The first emitting phase outside the envelope, in declaration order.
struct PhaseBreach<'a> {
    phase: &'a str,
    term: BreachTerm,
}

/// Whether an emitting phase sustains more than `max_rate` WIRE RECORDS per second.
///
/// `spans_per_dispatch` is the bound one dispatch puts on the wire
/// ([`EmissionSpec::max_spans_per_dispatch`]) — a dispatch is not one span, so counting dispatches
/// would read as bounding wire load while bounding something ~100× smaller.
///
/// Compared as `occurrences * spans_per_dispatch * 1000 > max_rate * gap_ms` — exact integer math,
/// no division and no float, so the bound behaves identically on every host. A zero-length window
/// declaring emissions is an unbounded rate and always breaches.
fn phase_rate_exceeds(
    occurrences: u32,
    spans_per_dispatch: u64,
    gap_ms: u64,
    max_rate: u64,
) -> bool {
    if gap_ms == 0 {
        return true;
    }
    u64::from(occurrences)
        .saturating_mul(spans_per_dispatch)
        .saturating_mul(1_000)
        > max_rate.saturating_mul(gap_ms)
}

/// The single source of "is this scenario inside the envelope?", shared by [`check_load_envelope`]
/// and [`LoadEnvelope::classify`] so the static gate and the per-run judgment cannot diverge.
///
/// Only emitting phases are judged: a silence window costs elapsed time but places no load.
fn phase_breach<'a>(terms: &EnvelopeTerms, scenario: &'a Scenario) -> Option<PhaseBreach<'a>> {
    scenario
        .phases
        .iter()
        .filter(|p| p.emission.occurrences > 0)
        .find_map(|p| {
            if p.gap_ms > terms.max_sustained_storm_ms {
                Some(PhaseBreach {
                    phase: &p.name,
                    term: BreachTerm::Storm,
                })
            } else if phase_rate_exceeds(
                p.emission.occurrences,
                p.emission.max_spans_per_dispatch(),
                p.gap_ms,
                terms.max_sustained_rate_spans_per_s,
            ) {
                Some(PhaseBreach {
                    phase: &p.name,
                    term: BreachTerm::Rate,
                })
            } else {
                None
            }
        })
}

impl LoadEnvelope {
    /// The default envelope path, relative to the workspace root. The binary edge resolves it before
    /// calling [`load`](Self::load) — never this module.
    pub fn default_path() -> PathBuf {
        PathBuf::from("contracts/pulse-load-envelope.toml")
    }

    /// Read + bounds-check the envelope from an already-resolved path.
    pub fn load(path: &Path) -> crate::Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|e| {
            // never the path itself — io::Error's Display leaks it (artifact hygiene).
            CoreError::Config(format!("could not read load envelope ({:?})", e.kind()))
        })?;
        let envelope: Self = toml::from_str(&text).map_err(|e| {
            CoreError::Config(format!(
                "invalid load envelope: {}",
                crate::sanitize_error(&e)
            ))
        })?;
        envelope.validate()?;
        tracing::info!(
            count = envelope.exempt.len(),
            "loaded load envelope for Pulse {}",
            envelope.sut_version
        );
        Ok(envelope)
    }

    /// Whether `scenario_name` is pinned exempt from the asserted duration bound.
    pub fn is_exempt(&self, scenario_name: &str) -> bool {
        self.exempt.iter().any(|e| e.scenario == scenario_name)
    }

    /// Judge one scenario for a run. An exempt scenario is in-envelope by pin; otherwise each of its
    /// emitting phases is judged against the sustained-storm and sustained-rate terms.
    ///
    /// Returns a VALUE on the `Ok` path in every case — an envelope breach is an outcome, never a
    /// harness fault.
    pub fn classify(&self, scenario: &Scenario) -> EnvelopeStatus {
        if self.is_exempt(&scenario.name) {
            return EnvelopeStatus::InEnvelope;
        }
        let Some(breach) = phase_breach(&self.envelope, scenario) else {
            return EnvelopeStatus::InEnvelope;
        };
        EnvelopeStatus::EnvironmentSuspect(format!(
            "scenario {:?} phase {:?} {} (Pulse {}, captured {}) — this run's read-back is not \
             evidence about the SUT",
            scenario.name,
            breach.phase,
            self.breach_detail(breach.term),
            self.sut_version,
            self.captured_at,
        ))
    }

    /// The human-readable half of a breach cause: which proven-good bound the phase left, in prose.
    /// Field names stay out of it — an operator-facing string names bounds, not struct members
    /// (security-plan §Error Handling).
    fn breach_detail(&self, term: BreachTerm) -> String {
        match term {
            BreachTerm::Storm => format!(
                "sustains emission longer than the proven-good storm window of {}s",
                self.envelope.max_sustained_storm_ms / 1000
            ),
            BreachTerm::Rate => format!(
                "emits faster than the proven-good sustained rate of {}/s",
                self.envelope.max_sustained_rate_spans_per_s
            ),
        }
    }

    fn validate(&self) -> crate::Result<()> {
        for (field, value) in [
            ("sut_version", &self.sut_version),
            ("captured_at", &self.captured_at),
            ("provenance", &self.provenance),
        ] {
            if value.trim().is_empty() {
                return Err(CoreError::Config(format!(
                    "load envelope: {field} is empty"
                )));
            }
        }
        for (term, value) in [
            (
                "max_sustained_rate_spans_per_s",
                self.envelope.max_sustained_rate_spans_per_s,
            ),
            (
                "max_sustained_storm_ms",
                self.envelope.max_sustained_storm_ms,
            ),
            (
                "max_scenario_duration_ms",
                self.envelope.max_scenario_duration_ms,
            ),
        ] {
            if value == 0 {
                return Err(CoreError::Config(format!(
                    "load envelope: {term} must be positive"
                )));
            }
        }
        let mut seen = BTreeSet::new();
        for exemption in &self.exempt {
            if exemption.scenario.trim().is_empty() {
                return Err(CoreError::Config(
                    "load envelope: an exemption names no scenario".to_string(),
                ));
            }
            if exemption.reason.trim().is_empty() {
                return Err(CoreError::Config(format!(
                    "load envelope: exemption {:?} carries no reason",
                    exemption.scenario
                )));
            }
            if !seen.insert(exemption.scenario.as_str()) {
                return Err(CoreError::Config(format!(
                    "load envelope: duplicate exemption {:?}",
                    exemption.scenario
                )));
            }
        }
        Ok(())
    }
}

/// Compare the committed scenario catalog against the load envelope.
///
/// `Ok(())` iff every scenario's emitting phases sit inside the envelope's sustained terms, or the
/// scenario is pinned exempt **exactly**. Three conditions each make it a fault, and they are
/// disjoint:
///
/// - a scenario over a sustained term that no exemption pins — the catalog drifted out of bounds;
/// - an exemption whose scenario is now inside the envelope — the ledger rotted and must shrink;
/// - an exemption naming no scenario in the catalog — the pin lost its subject.
///
/// Judged per emitting phase, via the same [`phase_breach`] the per-run judgment reads, so the gate
/// and the caption can never disagree. The message names scenarios plus the envelope's release and
/// capture date, so a red gate says which bound moved and what is owed.
pub fn check_load_envelope(envelope: &LoadEnvelope, catalog: &[Scenario]) -> crate::Result<()> {
    let names: BTreeSet<&str> = catalog.iter().map(|s| s.name.as_str()).collect();
    let over: BTreeSet<&str> = catalog
        .iter()
        .filter(|s| phase_breach(&envelope.envelope, s).is_some())
        .map(|s| s.name.as_str())
        .collect();
    let pinned: BTreeSet<&str> = envelope
        .exempt
        .iter()
        .map(|e| e.scenario.as_str())
        .collect();

    let unpinned: Vec<&str> = over.difference(&pinned).copied().collect();
    let rotted: Vec<&str> = pinned
        .intersection(&names)
        .filter(|n| !over.contains(*n))
        .copied()
        .collect();
    let lost_subject: Vec<&str> = pinned.difference(&names).copied().collect();

    if unpinned.is_empty() && rotted.is_empty() && lost_subject.is_empty() {
        return Ok(());
    }
    Err(CoreError::LoadEnvelope(envelope_message(
        envelope,
        &unpinned,
        &rotted,
        &lost_subject,
    )))
}

/// Render the envelope detail. Identity-only: scenario names + the envelope's release/date, never a
/// filesystem path and never an internal type name (security-plan §Error Handling).
fn envelope_message(
    envelope: &LoadEnvelope,
    unpinned: &[&str],
    rotted: &[&str],
    lost_subject: &[&str],
) -> String {
    let mut findings = Vec::new();
    if !unpinned.is_empty() {
        findings.push(format!(
            "{} over the proven-good storm bounds ({}s sustained at {}/s) and not exempt ({}) — \
             shorten the emitting phase, slow it, or pin an exemption with its reason",
            unpinned.len(),
            envelope.envelope.max_sustained_storm_ms / 1000,
            envelope.envelope.max_sustained_rate_spans_per_s,
            unpinned.join(", ")
        ));
    }
    if !rotted.is_empty() {
        findings.push(format!(
            "{} exempt but now inside the envelope ({}) — shrink the exemption ledger",
            rotted.len(),
            rotted.join(", ")
        ));
    }
    if !lost_subject.is_empty() {
        findings.push(format!(
            "{} exempt but named by no scenario ({}) — the exemption lost its subject",
            lost_subject.len(),
            lost_subject.join(", ")
        ));
    }
    format!(
        "Pulse {} (captured {}): {}",
        envelope.sut_version,
        envelope.captured_at,
        findings.join("; ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EmissionSpec, PId, PhaseSpec, SloTier};

    fn committed_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/pulse-load-envelope.toml")
    }

    fn scenarios_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios")
    }

    fn committed_catalog() -> Vec<Scenario> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let capabilities =
            crate::CapabilityManifest::load(&root.join("contracts/pulse-capabilities.toml"))
                .expect("the committed capability manifest loads");
        crate::scenario_files(&scenarios_dir())
            .expect("the committed catalog enumerates")
            .into_iter()
            .map(|path| {
                let text = std::fs::read_to_string(&path).expect("scenario file reads");
                Scenario::from_toml_str_with(&text, &capabilities).expect("scenario parses")
            })
            .collect()
    }

    /// `storm_ms` drives the term the gate actually asserts — the per-phase sustained-storm window.
    fn envelope(storm_ms: u64, exempt: &[(&str, &str)]) -> LoadEnvelope {
        LoadEnvelope {
            sut_version: "v0.3.0".to_string(),
            captured_at: "2026-08-09".to_string(),
            provenance: "test".to_string(),
            envelope: EnvelopeTerms {
                max_sustained_rate_spans_per_s: 10_000,
                max_sustained_storm_ms: storm_ms,
                max_scenario_duration_ms: 600_000,
            },
            exempt: exempt
                .iter()
                .map(|(scenario, reason)| Exemption {
                    scenario: scenario.to_string(),
                    reason: reason.to_string(),
                })
                .collect(),
        }
    }

    /// Every phase emits once, so a gap alone decides the storm window.
    fn scenario(name: &str, gaps: &[u64]) -> Scenario {
        paced(name, &gaps.iter().map(|g| (*g, 1)).collect::<Vec<_>>())
    }

    /// `(gap_ms, occurrences)` per phase — `occurrences: 0` declares a silence window.
    fn paced(name: &str, phases: &[(u64, u32)]) -> Scenario {
        Scenario {
            name: name.to_string(),
            p_ids: vec![PId("P-001".to_string())],
            seed: 1,
            slo_tier: SloTier::Tier5s,
            phases: phases
                .iter()
                .enumerate()
                .map(|(i, (gap_ms, occurrences))| PhaseSpec {
                    name: format!("phase-{i}"),
                    gap_ms: *gap_ms,
                    emission: EmissionSpec {
                        occurrences: *occurrences,
                        ..EmissionSpec::default()
                    },
                    fault: None,
                })
                .collect(),
            jitter_ms: 0,
            expected: Vec::new(),
            checklist: Vec::new(),
        }
    }

    #[test]
    fn loads_and_bounds_checks_the_committed_envelope() {
        let e = LoadEnvelope::load(&committed_path()).expect("committed envelope loads");
        assert_eq!(e.sut_version, "v0.3.0");
        assert!(e.envelope.max_sustained_storm_ms > 0);
        assert!(e.envelope.max_sustained_rate_spans_per_s > 0);
        assert!(
            !e.provenance.trim().is_empty(),
            "the envelope must say where its terms came from"
        );
        assert!(
            e.exempt.is_empty(),
            "the ledger retired when the gate moved onto the per-phase sustained terms; a new \
             entry needs its reason and a note on why the scenario cannot be shortened"
        );
    }

    /// The regression guard for the correction this term landed with: three emitting bursts
    /// separated by quiet are three storms, not one 900s storm. Summing them would put
    /// `activity-floor` over a ceiling it is comfortably inside.
    #[test]
    fn the_storm_window_is_the_longest_emitting_phase_not_the_sum_of_them() {
        let e = envelope(600_000, &[]);
        let bursty = paced(
            "bursty",
            &[
                (300_000, 60),
                (600_000, 0),
                (300_000, 60),
                (600_000, 0),
                (300_000, 60),
            ],
        );
        assert_eq!(sustained_storm_ms(&bursty), 300_000);
        assert_eq!(
            scenario_duration_ms(&bursty),
            2_100_000,
            "elapsed time is far larger"
        );
        assert_eq!(e.classify(&bursty), EnvelopeStatus::InEnvelope);
        check_load_envelope(&e, &[bursty]).expect("disjoint bursts are not one sustained storm");
    }

    #[test]
    fn a_silence_phase_is_never_storm_however_long_it_runs() {
        let e = envelope(1_000, &[]);
        let quiet = paced("quiet", &[(9_000, 0), (500, 1)]);
        assert_eq!(
            sustained_storm_ms(&quiet),
            500,
            "only emitting phases count"
        );
        assert_eq!(e.classify(&quiet), EnvelopeStatus::InEnvelope);
    }

    #[test]
    fn an_emitting_phase_faster_than_the_rate_term_is_a_breach() {
        let e = envelope(600_000, &[]);
        // 500 emissions across 1ms is 500_000/s, far over the 10_000/s term.
        let hot = paced("hot", &[(1, 500)]);
        let suspect = e.classify(&hot);
        assert!(
            suspect.is_suspect(),
            "an over-rate phase leaves the envelope"
        );
        let cause = suspect.cause().expect("a suspect run names its cause");
        assert!(
            cause.contains("phase-0"),
            "the offending phase is named: {cause}"
        );
        assert!(
            cause.contains("faster than"),
            "the rate term is named: {cause}"
        );
    }

    #[test]
    fn a_zero_length_emitting_phase_is_an_unbounded_rate() {
        let e = envelope(600_000, &[]);
        assert!(e.classify(&paced("instant", &[(0, 1)])).is_suspect());
        assert_eq!(
            e.classify(&paced("instant-but-quiet", &[(0, 0)])),
            EnvelopeStatus::InEnvelope,
            "a zero-length silence declares no load at all"
        );
    }

    #[test]
    fn the_rate_bound_is_exclusive_at_its_boundary() {
        let e = envelope(600_000, &[]);
        // 10_000 emissions across exactly 1s is the term itself, not over it.
        assert_eq!(
            e.classify(&paced("at", &[(1_000, 10_000)])),
            EnvelopeStatus::InEnvelope
        );
        assert!(e.classify(&paced("over", &[(1_000, 10_001)])).is_suspect());
    }

    /// The bound is documented where scenario authors actually work, not only in a plan: every
    /// committed scenario carries the pointer header at its top.
    #[test]
    fn every_committed_scenario_points_authors_at_the_envelope() {
        let files =
            crate::scenario_files(&scenarios_dir()).expect("the committed catalog enumerates");
        assert!(!files.is_empty(), "the catalog is not empty");
        for path in &files {
            let text = std::fs::read_to_string(path).expect("scenario file reads");
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("<scenario>");
            assert!(
                text.contains("LOAD ENVELOPE"),
                "scenario {stem:?} carries no envelope pointer for its author"
            );
        }
    }

    /// One basis, one meaning: the static gate and the per-run judgment must agree scenario by
    /// scenario, or a run could be gate-green and captioned environment-suspect at once.
    #[test]
    fn the_gate_and_the_per_run_judgment_agree_across_the_committed_catalog() {
        let e = LoadEnvelope::load(&committed_path()).expect("committed envelope loads");
        for scenario in committed_catalog() {
            assert_eq!(
                e.classify(&scenario),
                EnvelopeStatus::InEnvelope,
                "the gate passes the committed catalog, so no scenario may classify as suspect"
            );
        }
    }

    /// The live gate: every committed scenario is inside the committed envelope or exactly pinned.
    #[test]
    fn the_committed_catalog_matches_the_committed_envelope() {
        let e = LoadEnvelope::load(&committed_path()).expect("committed envelope loads");
        check_load_envelope(&e, &committed_catalog())
            .expect("every scenario is in-envelope or pinned exempt with its reason");
    }

    #[test]
    fn missing_file_is_a_harness_fault() {
        let err = LoadEnvelope::load(Path::new("contracts/does-not-exist.toml")).unwrap_err();
        assert!(matches!(err, CoreError::Config(_)));
    }

    #[test]
    fn load_failure_message_never_contains_the_path() {
        let path = committed_path()
            .parent()
            .unwrap()
            .join("no-such-load-envelope.toml");
        let err = LoadEnvelope::load(&path).unwrap_err().to_string();
        assert!(
            !err.contains("no-such-load-envelope"),
            "the path must not leak: {err}"
        );
        assert!(
            !err.contains(env!("CARGO_MANIFEST_DIR")),
            "no absolute host path: {err}"
        );
    }

    #[test]
    fn rejects_empty_identity_or_provenance() {
        for mutate in [
            |e: &mut LoadEnvelope| e.sut_version = "  ".to_string(),
            |e: &mut LoadEnvelope| e.captured_at = String::new(),
            |e: &mut LoadEnvelope| e.provenance = "   ".to_string(),
        ] {
            let mut e = envelope(600_000, &[]);
            mutate(&mut e);
            assert!(matches!(e.validate(), Err(CoreError::Config(_))));
        }
    }

    #[test]
    fn rejects_a_zero_term() {
        for mutate in [
            |e: &mut LoadEnvelope| e.envelope.max_scenario_duration_ms = 0,
            |e: &mut LoadEnvelope| e.envelope.max_sustained_rate_spans_per_s = 0,
            |e: &mut LoadEnvelope| e.envelope.max_sustained_storm_ms = 0,
        ] {
            let mut e = envelope(600_000, &[]);
            mutate(&mut e);
            assert!(matches!(e.validate(), Err(CoreError::Config(_))));
        }
    }

    #[test]
    fn rejects_a_reasonless_or_duplicated_exemption() {
        assert!(matches!(
            envelope(600_000, &[("a", "  ")]).validate(),
            Err(CoreError::Config(_))
        ));
        assert!(matches!(
            envelope(600_000, &[("", "why")]).validate(),
            Err(CoreError::Config(_))
        ));
        assert!(matches!(
            envelope(600_000, &[("a", "why"), ("a", "why")]).validate(),
            Err(CoreError::Config(_))
        ));
    }

    #[test]
    fn an_over_envelope_scenario_outside_the_ledger_is_a_fault() {
        let e = envelope(1_000, &[]);
        let err = check_load_envelope(&e, &[scenario("long-one", &[2_000])]).unwrap_err();
        let CoreError::LoadEnvelope(detail) = &err else {
            panic!("expected an envelope fault: {err:?}")
        };
        assert!(
            detail.contains("long-one"),
            "the over-envelope scenario must be named: {detail}"
        );
        assert!(
            detail.contains("not exempt"),
            "the remedy direction must be named: {detail}"
        );
    }

    #[test]
    fn a_ledger_entry_now_inside_the_envelope_is_a_fault() {
        let e = envelope(5_000, &[("short-one", "was long once")]);
        let err = check_load_envelope(&e, &[scenario("short-one", &[1_000])]).unwrap_err();
        let CoreError::LoadEnvelope(detail) = &err else {
            panic!("expected an envelope fault: {err:?}")
        };
        assert!(
            detail.contains("shrink the exemption ledger"),
            "pin rot must be named: {detail}"
        );
        assert!(
            detail.contains("short-one"),
            "the rotted entry must be named: {detail}"
        );
    }

    #[test]
    fn a_ledger_entry_no_scenario_names_is_a_fault() {
        let e = envelope(5_000, &[("ghost", "retired scenario")]);
        let err = check_load_envelope(&e, &[scenario("real-one", &[1_000])]).unwrap_err();
        let CoreError::LoadEnvelope(detail) = &err else {
            panic!("expected an envelope fault: {err:?}")
        };
        assert!(
            detail.contains("lost its subject"),
            "the lost-subject direction must be named: {detail}"
        );
        assert!(
            detail.contains("ghost"),
            "the orphaned pin must be named: {detail}"
        );
    }

    #[test]
    fn an_in_envelope_catalog_with_an_empty_ledger_passes() {
        let e = envelope(5_000, &[]);
        check_load_envelope(
            &e,
            &[scenario("a", &[1_000]), scenario("b", &[2_000, 2_000])],
        )
        .expect("a catalog inside the ceiling has no envelope fault");
    }

    #[test]
    fn envelope_message_names_identity_without_host_paths_or_type_names() {
        let e = envelope(1_000, &[]);
        let err = check_load_envelope(&e, &[scenario("long-one", &[2_000])])
            .unwrap_err()
            .to_string();

        assert!(
            err.contains("v0.3.0"),
            "the Pulse release must be named: {err}"
        );
        assert!(
            err.contains("2026-08-09"),
            "the capture date must be named: {err}"
        );
        assert!(
            !err.contains(env!("CARGO_MANIFEST_DIR")),
            "no absolute host path: {err}"
        );
        for leak in [
            "LoadEnvelope",
            "EnvelopeTerms",
            "Exemption",
            "BTreeSet",
            "max_scenario_duration_ms",
        ] {
            assert!(
                !err.contains(leak),
                "no internal type/field name ({leak}): {err}"
            );
        }
    }

    #[test]
    fn classify_returns_a_value_never_an_error() {
        let e = envelope(1_000, &[("pinned-long", "deliberate idle")]);

        assert_eq!(
            e.classify(&scenario("short", &[500])),
            EnvelopeStatus::InEnvelope
        );
        assert_eq!(
            e.classify(&scenario("pinned-long", &[9_000])),
            EnvelopeStatus::InEnvelope,
            "a pinned exemption is in-envelope by pin"
        );

        let suspect = e.classify(&scenario("long", &[9_000]));
        assert!(suspect.is_suspect());
        assert_eq!(suspect.label(), "ENVIRONMENT-SUSPECT");
        let cause = suspect.cause().expect("a suspect run names its cause");
        assert!(
            cause.contains("long"),
            "the cause names the scenario: {cause}"
        );
        assert!(
            cause.contains("not evidence about the SUT"),
            "the cause names the attribution: {cause}"
        );
    }

    #[test]
    fn classify_cause_carries_no_host_path_or_type_name() {
        let e = envelope(1_000, &[]);
        let cause = e
            .classify(&scenario("long", &[9_000]))
            .cause()
            .expect("suspect")
            .to_string();
        assert!(
            !cause.contains(env!("CARGO_MANIFEST_DIR")),
            "no absolute host path: {cause}"
        );
        for leak in ["LoadEnvelope", "EnvelopeStatus", "PhaseSpec", "gap_ms"] {
            assert!(
                !cause.contains(leak),
                "no internal type/field name ({leak}): {cause}"
            );
        }
    }

    /// A phase sitting exactly ON the storm ceiling is inside it. `incident-auto-resolution`'s
    /// 600s `sustain-10min` phase is that case in the committed catalog, so an inclusive comparison
    /// here would red the gate on a scenario the bound admits.
    #[test]
    fn a_phase_exactly_at_the_storm_ceiling_is_inside_the_envelope() {
        let e = envelope(600_000, &[]);
        assert_eq!(
            e.classify(&paced("exact", &[(600_000, 60)])),
            EnvelopeStatus::InEnvelope
        );
        assert!(e.classify(&paced("over", &[(600_001, 60)])).is_suspect());
    }

    #[test]
    fn envelope_status_round_trips_its_wire_form() {
        let suspect = EnvelopeStatus::EnvironmentSuspect("over by 3300s".to_string());
        let wire = serde_json::to_string(&suspect).expect("serializes");
        assert_eq!(
            serde_json::from_str::<EnvelopeStatus>(&wire).expect("deserializes"),
            suspect
        );
        assert!(
            wire.contains("environment-suspect"),
            "kebab-case wire form: {wire}"
        );

        let ok = EnvelopeStatus::InEnvelope;
        let wire = serde_json::to_string(&ok).expect("serializes");
        assert_eq!(
            serde_json::from_str::<EnvelopeStatus>(&wire).expect("deserializes"),
            ok
        );
    }

    #[test]
    fn duration_is_the_sum_of_phase_gaps() {
        assert_eq!(scenario_duration_ms(&scenario("s", &[100, 200, 300])), 600);
        assert_eq!(scenario_duration_ms(&scenario("empty-ish", &[0])), 0);
        assert_eq!(
            scenario_duration_ms(&scenario("saturating", &[u64::MAX, u64::MAX])),
            u64::MAX,
            "a pathological config saturates rather than wrapping"
        );
    }
}
