//! The capability coverage matrix — every Pulse capability classified by Conductor verification mode.
//!
//! `coverage-matrix.md` is Conductor's definition-of-done artifact: all sixty Pulse capabilities
//! (`P-001`..`P-060`) enumerated with zero gaps, each classified into exactly one [`CoverageMode`].
//! The classification is **code-native here** (the source of truth), authored from `input.md`
//! §Coverage classification + the audit verdict tables in `.andromeda/refs/`; the Markdown render
//! lives in `conductor-report`. A missing or duplicate P-ID is a defect — the completeness test is
//! the gate (a P-XXX absent from the matrix fails "0.1.0 done").

use serde::{Deserialize, Serialize};

/// How Conductor verifies a Pulse capability (the coverage-classification axis — distinct from a
/// [`crate::Verdict`], which is a per-check outcome).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoverageMode {
    /// Conductor drives the stimulus AND asserts the reaction (emission journal + MCP read-back +
    /// journal-relative timing).
    #[serde(rename = "auto")]
    Auto,
    /// Conductor induces the state; the operator confirms the visual/UX claim via the generated
    /// checklist — there is no programmatic read-back for the claim itself.
    #[serde(rename = "drive+observe")]
    DriveObserve,
    /// No dynamic telemetry dimension — stays with Pulse's own test matrix; Conductor does not
    /// duplicate it (it provides the workload where one is needed).
    #[serde(rename = "static-only")]
    StaticOnly,
}

impl CoverageMode {
    /// Every mode in canonical render order — the fixed order surfaces tally and list by.
    pub const ALL: [CoverageMode; 3] =
        [CoverageMode::Auto, CoverageMode::DriveObserve, CoverageMode::StaticOnly];

    /// The canonical label — identical to the serde wire spelling (input.md §Coverage classification).
    pub fn label(&self) -> &'static str {
        match self {
            CoverageMode::Auto => "auto",
            CoverageMode::DriveObserve => "drive+observe",
            CoverageMode::StaticOnly => "static-only",
        }
    }
}

/// One capability row: the P-ID, its Pulse title + category, and the Conductor verification mode.
/// `&'static str`-backed so the table is a `static` with no allocation and no runtime IO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CapabilityRow {
    /// The Pulse capability id, `P-001`..`P-060`.
    pub p_id: &'static str,
    /// The capability title (from the Pulse capability audit).
    pub title: &'static str,
    /// The capability category (from the Pulse capability audit).
    pub category: &'static str,
    /// How Conductor verifies it.
    pub mode: CoverageMode,
}

/// The complete `P-001`..`P-060` coverage classification — Conductor's definition of done.
pub fn coverage_matrix() -> &'static [CapabilityRow] {
    &COVERAGE
}

const fn row(
    p_id: &'static str,
    title: &'static str,
    category: &'static str,
    mode: CoverageMode,
) -> CapabilityRow {
    CapabilityRow { p_id, title, category, mode }
}

static COVERAGE: [CapabilityRow; 60] = [
    row("P-001", "Receiver Lifecycle State", "Connection & Health Awareness", CoverageMode::DriveObserve),
    row("P-002", "Last-Span-Ago Tracking", "Connection & Health Awareness", CoverageMode::DriveObserve),
    row("P-003", "Receiver Failure Surface", "Connection & Health Awareness", CoverageMode::DriveObserve),
    row("P-004", "Orthogonal Health Domains", "Connection & Health Awareness", CoverageMode::DriveObserve),
    row("P-005", "Span Status Error Detection", "Hard Signal Detection", CoverageMode::Auto),
    row("P-006", "Exception Event Capture", "Hard Signal Detection", CoverageMode::Auto),
    row("P-007", "High-Severity Log Capture", "Hard Signal Detection", CoverageMode::Auto),
    row("P-008", "Root-Span Error Scope Distinction", "Hard Signal Detection", CoverageMode::Auto),
    row("P-009", "Per-Service Error Rate Baseline", "Statistical Anomaly Detection", CoverageMode::Auto),
    row("P-010", "Error Rate Spike Detection", "Statistical Anomaly Detection", CoverageMode::Auto),
    row("P-011", "Per-Operation Latency Baseline", "Statistical Anomaly Detection", CoverageMode::Auto),
    row("P-012", "Latency Regression Detection", "Statistical Anomaly Detection", CoverageMode::Auto),
    row("P-013", "Service Activity Floor Learning", "Statistical Anomaly Detection", CoverageMode::Auto),
    row("P-014", "Service Went Silent Detection", "Statistical Anomaly Detection", CoverageMode::Auto),
    row("P-015", "Restart Event Detection", "Pattern Recognition", CoverageMode::Auto),
    row("P-016", "Restart-Window Suppression (Surgical)", "Pattern Recognition", CoverageMode::Auto),
    row("P-017", "Exception Fingerprinting", "Pattern Recognition", CoverageMode::Auto),
    row("P-018", "Retry Storm Detection", "Pattern Recognition", CoverageMode::Auto),
    row("P-019", "Three-Tier Severity Model", "Severity Calibration", CoverageMode::Auto),
    row("P-020", "Model-Driven Severity Decision", "Severity Calibration", CoverageMode::Auto),
    row("P-021", "Algorithmic Attention Cues", "Severity Calibration", CoverageMode::Auto),
    row("P-022", "Auto-Resolution and Lifecycle", "Severity Calibration", CoverageMode::Auto),
    row("P-023", "Acknowledge Cool-Down", "Severity Calibration", CoverageMode::Auto),
    row("P-024", "Widget Ambient Surface", "Three-Surface Communication", CoverageMode::DriveObserve),
    row("P-025", "Halo Hue Encoding", "Three-Surface Communication", CoverageMode::DriveObserve),
    row("P-026", "Halo Breathing Encoding", "Three-Surface Communication", CoverageMode::DriveObserve),
    row("P-027", "Service Constellation Auto-Discovery", "Three-Surface Communication", CoverageMode::DriveObserve),
    row("P-028", "Findings Counter", "Three-Surface Communication", CoverageMode::DriveObserve),
    row("P-029", "Findings Dropdown", "Three-Surface Communication", CoverageMode::DriveObserve),
    row("P-030", "No Interrupting Notifications by Default", "Three-Surface Communication", CoverageMode::DriveObserve),
    row("P-031", "Report Structure", "Diagnostic Quality", CoverageMode::Auto),
    row("P-032", "Project Context Grounding", "Diagnostic Quality", CoverageMode::Auto),
    row("P-033", "Ranked Hypothesis Generation", "Diagnostic Quality", CoverageMode::Auto),
    row("P-034", "Suggested Investigation Steps", "Diagnostic Quality", CoverageMode::Auto),
    row("P-035", "Anonymized Telemetry Excerpts", "Diagnostic Quality", CoverageMode::Auto),
    row("P-036", "Cross-Incident Pattern Reference", "Diagnostic Quality", CoverageMode::Auto),
    row("P-037", "In-App Report Surface", "Output Channels", CoverageMode::DriveObserve),
    row("P-038", "Copy to Clipboard", "Output Channels", CoverageMode::StaticOnly),
    row("P-039", "MCP Delivery When Configured", "Output Channels", CoverageMode::Auto),
    row("P-040", "MCP Independence", "Output Channels", CoverageMode::StaticOnly),
    row("P-041", "Persistent Incident Corpus", "Memory & Learning", CoverageMode::Auto),
    row("P-042", "Cross-Session Continuity", "Memory & Learning", CoverageMode::Auto),
    row("P-043", "Project-Scoped Memory", "Memory & Learning", CoverageMode::Auto),
    row("P-044", "Retrieval-Augmented Interpretation", "Memory & Learning", CoverageMode::Auto),
    row("P-045", "Counter Derivation from Corpus", "Memory & Learning", CoverageMode::Auto),
    row("P-046", "Export for Community Training", "Memory & Learning", CoverageMode::StaticOnly),
    row("P-047", "PII Scrubbing at Ingestion", "Privacy & Trust", CoverageMode::Auto),
    row("P-048", "No Raw OTLP Attribute Values Stored", "Privacy & Trust", CoverageMode::Auto),
    row("P-049", "Encryption at Rest", "Privacy & Trust", CoverageMode::StaticOnly),
    row("P-050", "Cross-Project Sharing Opt-In", "Privacy & Trust", CoverageMode::StaticOnly),
    row("P-051", "Transparent Storage", "Privacy & Trust", CoverageMode::StaticOnly),
    row("P-052", "Cadence Configuration", "Pipeline Operations", CoverageMode::Auto),
    row("P-053", "Fallback Model Tier", "Pipeline Operations", CoverageMode::Auto),
    row("P-054", "Hardware Profile Awareness", "Pipeline Operations", CoverageMode::StaticOnly),
    row("P-055", "Configuration Hot Reload", "Pipeline Operations", CoverageMode::Auto),
    row("P-056", "Prospective Threshold Application", "Pipeline Operations", CoverageMode::Auto),
    row("P-057", "Dual-Condition Suppression Bypass", "Pipeline Operations", CoverageMode::Auto),
    row("P-058", "Pipeline Self-Observability", "Pipeline Operations", CoverageMode::DriveObserve),
    row("P-059", "Active-Incident Interpretation Continuity", "Pipeline Operations", CoverageMode::Auto),
    row("P-060", "Tiered Triggering Priority", "Severity Calibration", CoverageMode::Auto),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_has_exactly_sixty_capabilities() {
        assert_eq!(coverage_matrix().len(), 60);
    }

    #[test]
    fn p_ids_are_contiguous_p001_to_p060_zero_gaps_no_dups() {
        let nums: Vec<u32> = coverage_matrix()
            .iter()
            .map(|r| {
                let n = r.p_id.strip_prefix("P-").expect("p_id starts with `P-`");
                assert_eq!(n.len(), 3, "p_id is zero-padded to 3 digits: {}", r.p_id);
                n.parse::<u32>().expect("numeric p_id suffix")
            })
            .collect();
        let expected: Vec<u32> = (1..=60).collect();
        assert_eq!(nums, expected, "P-001..P-060 in order, zero gaps, no duplicates");
    }

    #[test]
    fn named_static_only_anchors_are_static_only() {
        // input.md §Coverage classification names these explicitly as static-only.
        for id in ["P-038", "P-040", "P-046", "P-049", "P-051", "P-054"] {
            let row = coverage_matrix().iter().find(|r| r.p_id == id).expect("anchor present");
            assert_eq!(row.mode, CoverageMode::StaticOnly, "{id} must be static-only");
        }
    }

    #[test]
    fn every_mode_is_represented() {
        for mode in CoverageMode::ALL {
            assert!(
                coverage_matrix().iter().any(|r| r.mode == mode),
                "no capability classified {}",
                mode.label()
            );
        }
    }

    #[test]
    fn mode_label_matches_serde_wire() {
        for mode in CoverageMode::ALL {
            assert_eq!(serde_json::to_string(&mode).unwrap(), format!("\"{}\"", mode.label()));
        }
    }

    #[test]
    fn mode_round_trips_through_json() {
        for mode in CoverageMode::ALL {
            let json = serde_json::to_string(&mode).unwrap();
            let back: CoverageMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, back);
        }
    }
}
