//! The scenario-assertion audit — the mechanical form of two corpus properties that were previously
//! held in prose (arch §Occupied Resources; test-plan §6).
//!
//! Two properties, established by one gate. Every committed scenario's declared checks are
//! satisfiable or retired, and every scenario's declared tier is accounted for against its own summed
//! phase duration. Both were true when they were established by hand; what was missing is a way for a
//! later reader to RE-establish them, and for a regression in either to be noticed.
//!
//! [`check_scenario_audit`] is the gate, modelled arm-for-arm on
//! [`check_load_envelope`](crate::check_load_envelope): two sets measured from the catalog as loaded,
//! each graded against its committed ledger array in BOTH directions, reported as a
//! [`CoreError::ScenarioAudit`] harness fault. Neither set is judged by a count — a count re-stales at
//! the next scenario, so both are exact-set comparisons over names.
//!
//! **Why a ledger rather than a computed assertion.** Satisfiability is a property of the SUT's
//! rendering, not of the committed TOML, so no static check can derive it; and a scenario whose phases
//! outlast every tier in the closed set is the ratified honest-bucket posture rather than a defect.
//! Both are therefore JUDGMENTS, recorded once with their reasons and held to exact-set equality — the
//! shape that records work at full precision and can only shrink under compulsion, never a threshold
//! lowered to pass.
//!
//! **What the gate deliberately does not grade.** A live assertion's `discriminates` flag records
//! whether its token depends on the scenario's own stimulus. It is read by no condition here: the
//! answer lives in the SUT's behaviour, which a static check cannot reach (test-plan §11 routes such a
//! claim to the live/operator gate). Recording it keeps the finding enforced as data — a new weak
//! assertion cannot arrive unnoticed, because the row itself must be added — without pretending the
//! gate measured it.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{CoreError, Scenario};

/// The two retired glosses the sweep looks for in scenario comment prose. Both describe the latency
/// measurement in ways HEAD disproves: `latency_ms` spans the whole run, so a scenario's own phase
/// window is not "not the SLO budget", and the span is not Conductor's MCP round-trip.
const RETIRED_GLOSSES: [&str; 2] = ["not the SLO budget", "MCP round-trip"];

/// The committed audit ledger: which Pulse release and head its grounds were read at, and the two
/// exact-set pins.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ScenarioAuditLedger {
    /// The Pulse release the grounds below describe (e.g. `"v0.3.0"`).
    pub sut_version: String,
    /// The SUT head the grounds were measured at. A rendering change there rots them, and the ledger
    /// says where to look rather than leaving the staleness invisible.
    pub sut_head: String,
    /// The capture date, `YYYY-MM-DD`.
    pub captured_at: String,
    /// How far to trust the grounds — they are point-in-time readings of the SUT, not derivations.
    pub provenance: String,
    /// Scenarios carrying a live `[[expected]]` block.
    #[serde(default)]
    pub live_assertion: Vec<LiveAssertion>,
    /// Scenarios whose summed phase duration exceeds their declared tier's deadline.
    #[serde(default)]
    pub over_tier: Vec<OverTier>,
}

/// One pinned live assertion, with the ground on which its declared token is satisfiable.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LiveAssertion {
    pub scenario: String,
    pub ground: String,
    /// Whether the token depends on this scenario's own stimulus. Recorded, never gated — see the
    /// module doc.
    pub discriminates: bool,
    /// Why it does not, when it does not.
    #[serde(default)]
    pub weakness: String,
}

/// One pinned over-tier scenario, with the stated reason for declaring the tier anyway.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct OverTier {
    pub scenario: String,
    pub reason: String,
}

impl ScenarioAuditLedger {
    /// The default ledger path, relative to the workspace root. The caller's edge resolves it before
    /// calling [`load`](Self::load) — never this module (the [`LoadEnvelope`](crate::LoadEnvelope)
    /// split; security-plan §Input Validation gives it no `CONDUCTOR_*` override).
    pub fn default_path() -> PathBuf {
        PathBuf::from("contracts/scenario-audit-ledger.toml")
    }

    /// Read the ledger from an already-resolved path.
    pub fn load(path: &Path) -> crate::Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|e| {
            // never the path itself — io::Error's Display leaks it (artifact hygiene).
            CoreError::Config(format!(
                "could not read scenario audit ledger ({:?})",
                e.kind()
            ))
        })?;
        toml::from_str(&text).map_err(|e| {
            CoreError::Config(format!(
                "invalid scenario audit ledger: {}",
                crate::sanitize_error(&e)
            ))
        })
    }
}

/// Whether `scenario` declares at least one live `[[expected]]` check.
fn carries_live_assertion(scenario: &Scenario) -> bool {
    !scenario.expected.is_empty()
}

/// Whether `scenario`'s summed phase duration exceeds its own declared tier's deadline.
///
/// `latency_ms` is measured journal-relative over the whole run, so the summed `gap_ms` is what a
/// declared tier has to hold. The bound comes from [`SloTier::deadline_ms`](crate::SloTier::deadline_ms)
/// — the obs-owned single definition, never a re-declared threshold (obs-plan §5, §10).
fn exceeds_own_tier(scenario: &Scenario) -> bool {
    let summed_ms: u64 = scenario.phases.iter().map(|p| p.gap_ms).sum();
    i64::try_from(summed_ms).is_ok_and(|ms| ms > scenario.slo_tier.deadline_ms())
}

/// Whether `toml_text` carries either retired latency gloss in its comment prose.
///
/// The form is the one `verification-matrix.json#v3-05`'s acceptance pins as REQUIRED, and it is
/// wrap-tolerant by construction: a phrase split across two comment lines is invisible to a
/// line-granular search, so each line's leading `#` marker is stripped, the lines are joined, and
/// whitespace is collapsed before the test. The plain single-line grep is a KNOWN FALSE NEGATIVE for
/// this property and must never be substituted.
pub fn carries_retired_gloss(toml_text: &str) -> bool {
    let joined: String = toml_text
        .lines()
        .map(|line| line.trim_start())
        .filter(|line| line.starts_with('#'))
        .map(|line| line.trim_start_matches('#').trim())
        .collect::<Vec<_>>()
        .join(" ");
    let collapsed = joined.split_whitespace().collect::<Vec<_>>().join(" ");
    RETIRED_GLOSSES
        .iter()
        .any(|gloss| collapsed.contains(gloss))
}

/// The gate: the committed catalog agrees with the committed audit ledger, in both directions, on
/// both axes.
///
/// Six conditions, and EVERY one is graded — the message names each non-empty finding separately
/// rather than reporting only the first. A gate that keys its verdict off one tally while its claim
/// rests on several is the failure shape this gate exists not to repeat.
pub fn check_scenario_audit(
    ledger: &ScenarioAuditLedger,
    catalog: &[Scenario],
) -> crate::Result<()> {
    let names: BTreeSet<&str> = catalog.iter().map(|s| s.name.as_str()).collect();

    let measured_live: BTreeSet<&str> = catalog
        .iter()
        .filter(|s| carries_live_assertion(s))
        .map(|s| s.name.as_str())
        .collect();
    let pinned_live: BTreeSet<&str> = ledger
        .live_assertion
        .iter()
        .map(|a| a.scenario.as_str())
        .collect();

    let measured_over: BTreeSet<&str> = catalog
        .iter()
        .filter(|s| exceeds_own_tier(s))
        .map(|s| s.name.as_str())
        .collect();
    let pinned_over: BTreeSet<&str> = ledger
        .over_tier
        .iter()
        .map(|o| o.scenario.as_str())
        .collect();

    let live = Findings::grade(&measured_live, &pinned_live, &names);
    let over = Findings::grade(&measured_over, &pinned_over, &names);

    if live.is_clean() && over.is_clean() {
        return Ok(());
    }
    Err(CoreError::ScenarioAudit(audit_message(
        ledger, &live, &over,
    )))
}

/// One axis's three findings.
struct Findings<'a> {
    unpinned: Vec<&'a str>,
    rotted: Vec<&'a str>,
    lost_subject: Vec<&'a str>,
}

impl<'a> Findings<'a> {
    fn grade(
        measured: &BTreeSet<&'a str>,
        pinned: &BTreeSet<&'a str>,
        names: &BTreeSet<&'a str>,
    ) -> Self {
        Self {
            unpinned: measured.difference(pinned).copied().collect(),
            rotted: pinned
                .intersection(names)
                .filter(|n| !measured.contains(*n))
                .copied()
                .collect(),
            lost_subject: pinned.difference(names).copied().collect(),
        }
    }

    fn is_clean(&self) -> bool {
        self.unpinned.is_empty() && self.rotted.is_empty() && self.lost_subject.is_empty()
    }
}

/// Render the audit detail. Identity-only: scenario names + the ledger's release/head, never a
/// filesystem path and never an internal type name (security-plan §Error Handling).
fn audit_message(ledger: &ScenarioAuditLedger, live: &Findings, over: &Findings) -> String {
    let mut findings = Vec::new();
    push_axis(
        &mut findings,
        live,
        "declares a live check but is not pinned as a live assertion",
        "pinned as a live assertion but declares no check",
        "pinned as a live assertion but named by no scenario",
    );
    push_axis(
        &mut findings,
        over,
        "exceeds its own declared tier and is not pinned over-tier",
        "pinned over-tier but now fits its declared tier",
        "pinned over-tier but named by no scenario",
    );
    format!(
        "Pulse {} at {} (captured {}): {}",
        ledger.sut_version,
        ledger.sut_head,
        ledger.captured_at,
        findings.join("; ")
    )
}

fn push_axis(
    findings: &mut Vec<String>,
    axis: &Findings,
    unpinned: &str,
    rotted: &str,
    lost: &str,
) {
    for (subjects, complaint) in [
        (&axis.unpinned, unpinned),
        (&axis.rotted, rotted),
        (&axis.lost_subject, lost),
    ] {
        if !subjects.is_empty() {
            findings.push(format!(
                "{} {} ({})",
                subjects.len(),
                complaint,
                subjects.join(", ")
            ));
        }
    }
}
