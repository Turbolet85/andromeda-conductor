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

use std::collections::BTreeSet;

use crate::{CapabilityManifest, CapabilityRow, CoreError};

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
