//! SUT-drift detection — has Pulse's ledger moved past what Conductor has classified?
//!
//! [`CapabilityManifest`](crate::CapabilityManifest) is the accepted capability set (the SUT's ledger,
//! as data); [`coverage_matrix`](crate::coverage_matrix) is what Conductor has actually classified.
//! When Pulse ships a release the manifest grows and nothing else in the repo notices that the
//! classification did not — the blind spot that let twenty-two capabilities pass unseen across a pause.
//! [`check_sut_drift`] closes it.
//!
//! A drift is a harness fault ([`CoreError::SutDrift`]) — never a [`Verdict`](crate::Verdict) or
//! [`ReportState`](crate::ReportState), which describe the SUT's behavior rather than Conductor's own
//! staleness (arch §Cross-cutting Patterns "Verdict/error wall"). The comparison reads committed
//! artifacts only, so the same tree always yields the same result.
//!
//! Two integrity gates live here, on different axes. [`check_sut_drift`] asks whether every accepted
//! capability is *classified*; [`check_scenario_backing`] asks whether every capability classified
//! [`Auto`](crate::CoverageMode::Auto) — the mode that claims Conductor both drives the stimulus and
//! asserts the reaction — actually has a scenario behind that claim. A capability can pass the first
//! and fail the second: classification is a statement of intent, a scenario is the thing that makes it
//! true.

use std::collections::BTreeSet;

use crate::{CapabilityManifest, CapabilityRow, CoreError, CoverageMode};

/// Capability ids the manifest accepts that the coverage classification does not yet cover.
///
/// This is Conductor's **not-yet-classified residual ledger**: a record of work owed, NOT a source of
/// the accepted capability set. `contracts/pulse-capabilities.toml` remains the sole reference universe
/// (arch §Established Decisions [Accepted Capability Set]) — nothing here widens or narrows what a
/// scenario may name, and an id listed below is still fully accepted at scenario load.
///
/// **Empty since the `v2-03` Current-SUT coverage classification chunk**, which classified the twenty-two
/// ids (`P-061`..`P-082`) this ledger was minted to record. [`check_sut_drift`] therefore degenerates to
/// the plain zero-drift assertion: every accepted id is classified, and every classified id is accepted.
/// Should Pulse's ledger advance again, prefer classifying the new ids over re-populating this list —
/// a non-empty ledger is a debt, and the gate fails on an entry that has since been classified, so it
/// can only ever shrink under compulsion.
pub const KNOWN_UNCLASSIFIED: &[&str] = &[];

/// Capabilities classified [`Auto`](crate::CoverageMode::Auto) that no scenario in the catalog names.
///
/// `Auto` claims Conductor drives the stimulus AND asserts the reaction, but until now nothing checked
/// that such a row had a scenario at all — [`check_sut_drift`] compares the classification against the
/// manifest, never against the catalog, so a claim could stand with nothing behind it. This is the
/// record of that owed work: a debt, never a source of truth, and never a range expression.
/// [`check_scenario_backing`] holds it to exact-set equality, so it can only shrink under compulsion.
///
/// **Interpretation-correctness (`v2-05`).** Four entries — `P-031`, `P-033`, `P-034`, `P-044` — are the
/// diagnostic-quality cluster, and their absence is a recorded DECISION rather than an oversight.
/// Pulse's deterministic L4 mode replaces the Llama-3.2-3B inference with a canned `L4Output`, which is
/// exactly what makes the live legs reproducible; the cost is that those legs exercise the pipeline and
/// never the interpretation. So **"Conductor green" does not mean Pulse's interpretation is
/// trustworthy** — it means the plumbing carried a canned answer end to end. Proving the real thing
/// means injecting a known root cause with deterministic mode OFF and asserting the top hypothesis
/// identifies it, which needs both real per-check read-back extraction and a non-deterministic live leg
/// that can never be a CI gate. **Owner: a conductor-0.3.0 entry.**
///
/// The remaining four retire as their scenarios land. `P-074` left this ledger when
/// `fingerprint-storm-live-proof` named it in `scenarios/fingerprint-storm.toml`; `P-079` when
/// `constellation-severity-live-wiring` did, and `P-073` when `pulse-run-contract` did.
pub const UNBACKED_AUTO: &[&str] =
    &["P-031", "P-033", "P-034", "P-039", "P-041", "P-042", "P-043", "P-044"];

/// Compare the accepted capability set against the coverage classification.
///
/// `Ok(())` iff the unclassified set matches `known_unclassified` **exactly** and every classified row
/// is still backed by the manifest. Three conditions each make it a drift:
///
/// - an accepted id neither classified nor pinned — Pulse's ledger advanced;
/// - a pinned id that *is* classified — the ledger rotted and must shrink;
/// - a classified id the manifest no longer accepts — Pulse retired a capability.
///
/// The message names the offending ids plus the manifest's Pulse release and capture date, so a red
/// gate says which release moved and what is owed.
///
/// Pass an empty `known_unclassified` for the plain zero-drift assertion this degenerates to once the
/// classification covers the whole ledger.
pub fn check_sut_drift(
    manifest: &CapabilityManifest,
    classification: &[CapabilityRow],
    known_unclassified: &[&str],
) -> crate::Result<()> {
    let accepted: BTreeSet<&str> = manifest.capabilities.iter().map(String::as_str).collect();
    let classified: BTreeSet<&str> = classification.iter().map(|r| r.p_id).collect();
    let pinned: BTreeSet<&str> = known_unclassified.iter().copied().collect();

    let unclassified: BTreeSet<&str> = accepted.difference(&classified).copied().collect();

    let unknown: Vec<&str> = unclassified.difference(&pinned).copied().collect();
    let stale_pin: Vec<&str> = pinned.difference(&unclassified).copied().collect();
    let retired: Vec<&str> = classified.difference(&accepted).copied().collect();

    if unknown.is_empty() && stale_pin.is_empty() && retired.is_empty() {
        return Ok(());
    }
    Err(CoreError::SutDrift(drift_message(manifest, &unknown, &stale_pin, &retired)))
}

/// Render the drift detail. Identity-only: capability ids + the manifest's release/date, never a
/// filesystem path and never an internal type name (security-plan §Error Handling).
fn drift_message(
    manifest: &CapabilityManifest,
    unknown: &[&str],
    stale_pin: &[&str],
    retired: &[&str],
) -> String {
    let mut findings = Vec::new();
    if !unknown.is_empty() {
        findings.push(format!(
            "{} accepted but unclassified and not in the known gap ({}) — classify them or extend the known gap",
            unknown.len(),
            unknown.join(", ")
        ));
    }
    if !stale_pin.is_empty() {
        findings.push(format!(
            "{} in the known gap but already classified ({}) — shrink the known gap",
            stale_pin.len(),
            stale_pin.join(", ")
        ));
    }
    if !retired.is_empty() {
        findings.push(format!(
            "{} classified but no longer accepted ({}) — the capability was retired",
            retired.len(),
            retired.join(", ")
        ));
    }
    format!(
        "Pulse {} (captured {}): {}",
        manifest.sut_version,
        manifest.captured_at,
        findings.join("; ")
    )
}

/// Compare the `Auto`-classified capabilities against the scenarios that actually name them.
///
/// `Ok(())` iff the set of `Auto` rows no scenario covers matches `known_unbacked` **exactly**. Three
/// conditions each make it a fault, and they are disjoint:
///
/// - an `Auto` row no scenario names and the ledger does not pin — a claim with nothing behind it;
/// - a pinned id a scenario now names — the ledger rotted and must shrink;
/// - a pinned id no longer classified `Auto` — the claim was reclassified, so the pin lost its subject.
///
/// Only `Auto` participates: `DriveObserve` is operator-confirmed by definition, and `StaticOnly` /
/// `NotConductors` are deliberately not Conductor's to drive (see [`CoverageMode`]).
///
/// `scenario_p_ids` is what the catalog names — the caller reads it (via
/// [`list_scenarios`](crate::list_scenarios)) so this stays a pure set comparison, mirroring
/// [`check_sut_drift`].
pub fn check_scenario_backing(
    classification: &[CapabilityRow],
    scenario_p_ids: &[&str],
    known_unbacked: &[&str],
) -> crate::Result<()> {
    let auto: BTreeSet<&str> = classification
        .iter()
        .filter(|r| r.mode == CoverageMode::Auto)
        .map(|r| r.p_id)
        .collect();
    let covered: BTreeSet<&str> = scenario_p_ids.iter().copied().collect();
    let pinned: BTreeSet<&str> = known_unbacked.iter().copied().collect();

    let unbacked: BTreeSet<&str> = auto.difference(&covered).copied().collect();

    let unknown: Vec<&str> = unbacked.difference(&pinned).copied().collect();
    let now_covered: Vec<&str> = pinned.intersection(&covered).copied().collect();
    let lost_subject: Vec<&str> =
        pinned.difference(&auto).filter(|id| !covered.contains(*id)).copied().collect();

    if unknown.is_empty() && now_covered.is_empty() && lost_subject.is_empty() {
        return Ok(());
    }
    Err(CoreError::UnbackedCoverage(backing_message(&unknown, &now_covered, &lost_subject)))
}

/// Render the scenario-backing detail. Identity-only: capability ids + counts, never a filesystem path
/// and never an internal type name (security-plan §Error Handling).
fn backing_message(unknown: &[&str], now_covered: &[&str], lost_subject: &[&str]) -> String {
    let mut findings = Vec::new();
    if !unknown.is_empty() {
        findings.push(format!(
            "{} auto-classified with no scenario and not in the unbacked ledger ({}) — author a scenario or extend the ledger",
            unknown.len(),
            unknown.join(", ")
        ));
    }
    if !now_covered.is_empty() {
        findings.push(format!(
            "{} in the unbacked ledger but now named by a scenario ({}) — shrink the ledger",
            now_covered.len(),
            now_covered.join(", ")
        ));
    }
    if !lost_subject.is_empty() {
        findings.push(format!(
            "{} in the unbacked ledger but no longer auto-classified ({}) — the claim was reclassified",
            lost_subject.len(),
            lost_subject.join(", ")
        ));
    }
    findings.join("; ")
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::{CoverageMode, coverage_matrix};

    fn committed_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/pulse-capabilities.toml")
    }

    fn manifest(capabilities: &[&str]) -> CapabilityManifest {
        CapabilityManifest {
            sut_version: "v0.3.0".to_string(),
            captured_at: "2026-08-08".to_string(),
            capabilities: capabilities.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn rows(p_ids: &[&'static str]) -> Vec<CapabilityRow> {
        p_ids
            .iter()
            .map(|p_id| CapabilityRow {
                p_id,
                title: "t",
                category: "c",
                mode: CoverageMode::Auto,
            })
            .collect()
    }

    #[test]
    fn committed_artifacts_match_the_known_gap() {
        let m = CapabilityManifest::load(&committed_path()).expect("committed manifest loads");
        check_sut_drift(&m, coverage_matrix(), KNOWN_UNCLASSIFIED)
            .expect("the committed manifest and classification differ only by the known gap");
    }

    #[test]
    fn an_accepted_id_outside_the_known_gap_is_drift() {
        let m = manifest(&["P-001", "P-002"]);
        let err = check_sut_drift(&m, &rows(&["P-001"]), &["P-003"]).unwrap_err();
        let CoreError::SutDrift(detail) = &err else { panic!("expected a drift fault: {err:?}") };
        assert!(detail.contains("P-002"), "the unknown id must be named: {detail}");
    }

    #[test]
    fn a_known_gap_entry_that_is_now_classified_is_drift() {
        let m = manifest(&["P-001", "P-002"]);
        let err = check_sut_drift(&m, &rows(&["P-001", "P-002"]), &["P-002"]).unwrap_err();
        let CoreError::SutDrift(detail) = &err else { panic!("expected a drift fault: {err:?}") };
        assert!(detail.contains("shrink the known gap"), "pin rot must be named: {detail}");
        assert!(detail.contains("P-002"), "the stale id must be named: {detail}");
    }

    #[test]
    fn a_classified_id_the_manifest_dropped_is_drift() {
        let m = manifest(&["P-001"]);
        let err = check_sut_drift(&m, &rows(&["P-001", "P-002"]), &[]).unwrap_err();
        let CoreError::SutDrift(detail) = &err else { panic!("expected a drift fault: {err:?}") };
        assert!(detail.contains("retired"), "the retired direction must be named: {detail}");
        assert!(detail.contains("P-002"), "the retired id must be named: {detail}");
    }

    #[test]
    fn in_sync_with_an_empty_known_gap_passes() {
        let m = manifest(&["P-001", "P-002"]);
        check_sut_drift(&m, &rows(&["P-001", "P-002"]), &[])
            .expect("a fully classified ledger has no drift");
    }

    #[test]
    fn drift_message_names_identity_without_host_paths_or_type_names() {
        let m = manifest(&["P-001", "P-002"]);
        let err = check_sut_drift(&m, &rows(&["P-001"]), &[]).unwrap_err().to_string();

        assert!(err.contains("v0.3.0"), "the Pulse release must be named: {err}");
        assert!(err.contains("2026-08-08"), "the capture date must be named: {err}");
        assert!(!err.contains(env!("CARGO_MANIFEST_DIR")), "no absolute host path: {err}");
        for leak in ["CapabilityManifest", "CapabilityRow", "BTreeSet", "sut_version", "captured_at"] {
            assert!(!err.contains(leak), "no internal type/field name ({leak}): {err}");
        }
    }

    fn scenarios_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios")
    }

    fn modes(spec: &[(&'static str, CoverageMode)]) -> Vec<CapabilityRow> {
        spec.iter()
            .map(|(p_id, mode)| CapabilityRow { p_id, title: "t", category: "c", mode: *mode })
            .collect()
    }

    /// The live gate: every `Auto` claim in the committed classification either has a scenario in the
    /// committed catalog or is pinned — exactly, in both directions.
    #[test]
    fn the_committed_catalog_matches_the_unbacked_ledger() {
        let m = CapabilityManifest::load(&committed_path()).expect("committed manifest loads");
        let catalog =
            crate::list_scenarios(&scenarios_dir(), &m).expect("committed scenarios load");
        let named: Vec<&str> =
            catalog.iter().flat_map(|s| s.p_ids.iter().map(|p| p.0.as_str())).collect();

        check_scenario_backing(coverage_matrix(), &named, UNBACKED_AUTO)
            .expect("every auto claim is backed by a scenario or pinned as owed");
    }

    #[test]
    fn an_auto_row_with_no_scenario_outside_the_ledger_is_a_fault() {
        let rows = modes(&[("P-001", CoverageMode::Auto), ("P-002", CoverageMode::Auto)]);
        let err = check_scenario_backing(&rows, &["P-001"], &[]).unwrap_err();
        let CoreError::UnbackedCoverage(detail) = &err else {
            panic!("expected an unbacked-coverage fault: {err:?}")
        };
        assert!(detail.contains("P-002"), "the unbacked id must be named: {detail}");
        assert!(detail.contains("author a scenario"), "the remedy must be named: {detail}");
    }

    #[test]
    fn a_ledger_entry_a_scenario_now_names_is_a_fault() {
        let rows = modes(&[("P-001", CoverageMode::Auto), ("P-002", CoverageMode::Auto)]);
        let err = check_scenario_backing(&rows, &["P-001", "P-002"], &["P-002"]).unwrap_err();
        let CoreError::UnbackedCoverage(detail) = &err else {
            panic!("expected an unbacked-coverage fault: {err:?}")
        };
        assert!(detail.contains("shrink the ledger"), "pin rot must be named: {detail}");
        assert!(detail.contains("P-002"), "the rotted id must be named: {detail}");
    }

    #[test]
    fn a_ledger_entry_no_longer_auto_classified_is_a_fault() {
        let rows =
            modes(&[("P-001", CoverageMode::Auto), ("P-002", CoverageMode::DriveObserve)]);
        let err = check_scenario_backing(&rows, &["P-001"], &["P-002"]).unwrap_err();
        let CoreError::UnbackedCoverage(detail) = &err else {
            panic!("expected an unbacked-coverage fault: {err:?}")
        };
        assert!(detail.contains("no longer auto-classified"), "the direction: {detail}");
        assert!(detail.contains("P-002"), "the id that lost its subject: {detail}");
    }

    #[test]
    fn a_fully_backed_classification_with_an_empty_ledger_passes() {
        let rows = modes(&[("P-001", CoverageMode::Auto), ("P-002", CoverageMode::Auto)]);
        check_scenario_backing(&rows, &["P-001", "P-002"], &[])
            .expect("every auto row has a scenario");
    }

    /// Only `Auto` claims programmatic verification, so the other three modes never need a scenario —
    /// otherwise every operator-checklist and out-of-scope row would read as owed work.
    #[test]
    fn only_auto_rows_need_scenario_backing() {
        let rows = modes(&[
            ("P-002", CoverageMode::DriveObserve),
            ("P-003", CoverageMode::StaticOnly),
            ("P-004", CoverageMode::NotConductors),
        ]);
        check_scenario_backing(&rows, &[], &[]).expect("no auto row means nothing is owed");
    }

    #[test]
    fn backing_message_names_identity_without_host_paths_or_type_names() {
        let rows = modes(&[("P-001", CoverageMode::Auto), ("P-002", CoverageMode::Auto)]);
        let err = check_scenario_backing(&rows, &["P-001"], &[]).unwrap_err().to_string();

        assert!(err.contains("P-002"), "the offending id must be named: {err}");
        assert!(!err.contains(env!("CARGO_MANIFEST_DIR")), "no absolute host path: {err}");
        for leak in ["CapabilityRow", "CoverageMode", "BTreeSet", "ScenarioSummary", "p_ids"] {
            assert!(!err.contains(leak), "no internal type/field name ({leak}): {err}");
        }
    }

    #[test]
    fn drift_message_is_byte_identical_for_identical_inputs() {
        let m = manifest(&["P-003", "P-001", "P-002"]);
        let first = check_sut_drift(&m, &rows(&["P-001"]), &[]).unwrap_err().to_string();
        let second = check_sut_drift(&m, &rows(&["P-001"]), &[]).unwrap_err().to_string();
        assert_eq!(first, second);
        assert!(
            first.contains("P-002, P-003"),
            "ids are reported in sorted order regardless of manifest order: {first}"
        );
    }
}
