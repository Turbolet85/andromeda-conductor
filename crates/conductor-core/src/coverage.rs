//! The capability coverage matrix — every Pulse capability classified by Conductor verification mode.
//!
//! `coverage-matrix.md` is Conductor's definition-of-done artifact: every capability the SUT capability
//! manifest accepts, enumerated with zero gaps, each classified into exactly one [`CoverageMode`].
//! The classification is **code-native here** (the source of truth), authored from `input.md`
//! §Coverage classification, the audit verdict tables in `.andromeda/refs/`, and the SUT's own
//! capability ledger; the Markdown render lives in `conductor-report`. A missing or duplicate P-ID is
//! a defect — [`check_sut_drift`](crate::check_sut_drift) is the gate, comparing this classification
//! against the accepted set in `contracts/pulse-capabilities.toml`.

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
    /// Outside Conductor's remit by standing non-goal — the SUT's own suites verify it (its webview
    /// tests, its integration e2e, its own tooling). Distinct from [`StaticOnly`](Self::StaticOnly):
    /// static-only means the capability has no dynamic telemetry dimension to drive, whereas this
    /// means Conductor deliberately does not verify it. Recording it keeps the boundary a decision
    /// rather than an absence (arch §Cross-cutting Patterns "Scope law").
    #[serde(rename = "not-conductors")]
    NotConductors,
}

impl CoverageMode {
    /// Every mode in canonical render order — the fixed order surfaces tally and list by.
    pub const ALL: [CoverageMode; 4] = [
        CoverageMode::Auto,
        CoverageMode::DriveObserve,
        CoverageMode::StaticOnly,
        CoverageMode::NotConductors,
    ];

    /// The canonical label — identical to the serde wire spelling (input.md §Coverage classification).
    pub fn label(&self) -> &'static str {
        match self {
            CoverageMode::Auto => "auto",
            CoverageMode::DriveObserve => "drive+observe",
            CoverageMode::StaticOnly => "static-only",
            CoverageMode::NotConductors => "not-conductors",
        }
    }
}

/// One capability row: the P-ID, its Pulse title + category, and the Conductor verification mode.
/// `&'static str`-backed so the table is a `static` with no allocation and no runtime IO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CapabilityRow {
    /// The Pulse capability id, in `P-NNN` form.
    pub p_id: &'static str,
    /// The capability title (from the Pulse capability audit).
    pub title: &'static str,
    /// The capability category (from the Pulse capability audit).
    pub category: &'static str,
    /// How Conductor verifies it.
    pub mode: CoverageMode,
}

/// The complete coverage classification over the accepted capability set — Conductor's definition of
/// done. Kept in step with `contracts/pulse-capabilities.toml` by
/// [`check_sut_drift`](crate::check_sut_drift).
pub fn coverage_matrix() -> &'static [CapabilityRow] {
    &COVERAGE
}

const fn row(
    p_id: &'static str,
    title: &'static str,
    category: &'static str,
    mode: CoverageMode,
) -> CapabilityRow {
    CapabilityRow {
        p_id,
        title,
        category,
        mode,
    }
}

static COVERAGE: [CapabilityRow; 82] = [
    row(
        "P-001",
        "Receiver Lifecycle State",
        "Connection & Health Awareness",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-002",
        "Last-Span-Ago Tracking",
        "Connection & Health Awareness",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-003",
        "Receiver Failure Surface",
        "Connection & Health Awareness",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-004",
        "Orthogonal Health Domains",
        "Connection & Health Awareness",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-005",
        "Span Status Error Detection",
        "Hard Signal Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-006",
        "Exception Event Capture",
        "Hard Signal Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-007",
        "High-Severity Log Capture",
        "Hard Signal Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-008",
        "Root-Span Error Scope Distinction",
        "Hard Signal Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-009",
        "Per-Service Error Rate Baseline",
        "Statistical Anomaly Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-010",
        "Error Rate Spike Detection",
        "Statistical Anomaly Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-011",
        "Per-Operation Latency Baseline",
        "Statistical Anomaly Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-012",
        "Latency Regression Detection",
        "Statistical Anomaly Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-013",
        "Service Activity Floor Learning",
        "Statistical Anomaly Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-014",
        "Service Went Silent Detection",
        "Statistical Anomaly Detection",
        CoverageMode::Auto,
    ),
    row(
        "P-015",
        "Restart Event Detection",
        "Pattern Recognition",
        CoverageMode::Auto,
    ),
    row(
        "P-016",
        "Restart-Window Suppression (Surgical)",
        "Pattern Recognition",
        CoverageMode::Auto,
    ),
    row(
        "P-017",
        "Exception Fingerprinting",
        "Pattern Recognition",
        CoverageMode::Auto,
    ),
    row(
        "P-018",
        "Retry Storm Detection",
        "Pattern Recognition",
        CoverageMode::Auto,
    ),
    row(
        "P-019",
        "Three-Tier Severity Model",
        "Severity Calibration",
        CoverageMode::Auto,
    ),
    row(
        "P-020",
        "Model-Driven Severity Decision",
        "Severity Calibration",
        CoverageMode::Auto,
    ),
    row(
        "P-021",
        "Algorithmic Attention Cues",
        "Severity Calibration",
        CoverageMode::Auto,
    ),
    row(
        "P-022",
        "Auto-Resolution and Lifecycle",
        "Severity Calibration",
        CoverageMode::Auto,
    ),
    row(
        "P-023",
        "Acknowledge Cool-Down",
        "Severity Calibration",
        CoverageMode::Auto,
    ),
    row(
        "P-024",
        "Widget Ambient Surface",
        "Three-Surface Communication",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-025",
        "Halo Hue Encoding",
        "Three-Surface Communication",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-026",
        "Halo Breathing Encoding",
        "Three-Surface Communication",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-027",
        "Service Constellation Auto-Discovery",
        "Three-Surface Communication",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-028",
        "Findings Counter",
        "Three-Surface Communication",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-029",
        "Findings Dropdown",
        "Three-Surface Communication",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-030",
        "No Interrupting Notifications by Default",
        "Three-Surface Communication",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-031",
        "Report Structure",
        "Diagnostic Quality",
        CoverageMode::Auto,
    ),
    row(
        "P-032",
        "Project Context Grounding",
        "Diagnostic Quality",
        CoverageMode::Auto,
    ),
    row(
        "P-033",
        "Ranked Hypothesis Generation",
        "Diagnostic Quality",
        CoverageMode::Auto,
    ),
    row(
        "P-034",
        "Suggested Investigation Steps",
        "Diagnostic Quality",
        CoverageMode::Auto,
    ),
    row(
        "P-035",
        "Anonymized Telemetry Excerpts",
        "Diagnostic Quality",
        CoverageMode::Auto,
    ),
    row(
        "P-036",
        "Cross-Incident Pattern Reference",
        "Diagnostic Quality",
        CoverageMode::Auto,
    ),
    row(
        "P-037",
        "In-App Report Surface",
        "Output Channels",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-038",
        "Copy to Clipboard",
        "Output Channels",
        CoverageMode::StaticOnly,
    ),
    row(
        "P-039",
        "MCP Delivery When Configured",
        "Output Channels",
        CoverageMode::Auto,
    ),
    row(
        "P-040",
        "MCP Independence",
        "Output Channels",
        CoverageMode::StaticOnly,
    ),
    row(
        "P-041",
        "Persistent Incident Corpus",
        "Memory & Learning",
        CoverageMode::Auto,
    ),
    row(
        "P-042",
        "Cross-Session Continuity",
        "Memory & Learning",
        CoverageMode::Auto,
    ),
    row(
        "P-043",
        "Project-Scoped Memory",
        "Memory & Learning",
        CoverageMode::Auto,
    ),
    row(
        "P-044",
        "Retrieval-Augmented Interpretation",
        "Memory & Learning",
        CoverageMode::Auto,
    ),
    row(
        "P-045",
        "Counter Derivation from Corpus",
        "Memory & Learning",
        CoverageMode::Auto,
    ),
    row(
        "P-046",
        "Export for Community Training",
        "Memory & Learning",
        CoverageMode::StaticOnly,
    ),
    row(
        "P-047",
        "PII Scrubbing at Ingestion",
        "Privacy & Trust",
        CoverageMode::Auto,
    ),
    row(
        "P-048",
        "No Raw OTLP Attribute Values Stored",
        "Privacy & Trust",
        CoverageMode::Auto,
    ),
    row(
        "P-049",
        "Encryption at Rest",
        "Privacy & Trust",
        CoverageMode::StaticOnly,
    ),
    row(
        "P-050",
        "Cross-Project Sharing Opt-In",
        "Privacy & Trust",
        CoverageMode::StaticOnly,
    ),
    row(
        "P-051",
        "Transparent Storage",
        "Privacy & Trust",
        CoverageMode::StaticOnly,
    ),
    row(
        "P-052",
        "Cadence Configuration",
        "Pipeline Operations",
        CoverageMode::Auto,
    ),
    row(
        "P-053",
        "Fallback Model Tier",
        "Pipeline Operations",
        CoverageMode::Auto,
    ),
    row(
        "P-054",
        "Hardware Profile Awareness",
        "Pipeline Operations",
        CoverageMode::StaticOnly,
    ),
    row(
        "P-055",
        "Configuration Hot Reload",
        "Pipeline Operations",
        CoverageMode::Auto,
    ),
    row(
        "P-056",
        "Prospective Threshold Application",
        "Pipeline Operations",
        CoverageMode::Auto,
    ),
    row(
        "P-057",
        "Dual-Condition Suppression Bypass",
        "Pipeline Operations",
        CoverageMode::Auto,
    ),
    row(
        "P-058",
        "Pipeline Self-Observability",
        "Pipeline Operations",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-059",
        "Active-Incident Interpretation Continuity",
        "Pipeline Operations",
        CoverageMode::Auto,
    ),
    row(
        "P-060",
        "Tiered Triggering Priority",
        "Severity Calibration",
        CoverageMode::Auto,
    ),
    // Pulse v0.3.0 (P-061..P-078 from the SUT's requirements themes; P-079..P-082 were minted
    // mid-build as operator-surfaced capabilities and carry no theme of their own).
    row(
        "P-061",
        "Window geometry + movable shell",
        "Window & shell hygiene",
        CoverageMode::NotConductors,
    ),
    row(
        "P-062",
        "Window size constraints",
        "Window & shell hygiene",
        CoverageMode::NotConductors,
    ),
    row(
        "P-063",
        "Predictable close + honest tray",
        "Window & shell hygiene",
        CoverageMode::NotConductors,
    ),
    row(
        "P-064",
        "Suppress browser context menu",
        "Window & shell hygiene",
        CoverageMode::NotConductors,
    ),
    row(
        "P-065",
        "Canvas not a browser image",
        "Window & shell hygiene",
        CoverageMode::NotConductors,
    ),
    row(
        "P-066",
        "Widget-to-dashboard navigation",
        "Window & shell hygiene",
        CoverageMode::NotConductors,
    ),
    row(
        "P-067",
        "Live-only service truth",
        "State honesty & legibility",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-068",
        "Anomaly surfacing",
        "State honesty & legibility",
        CoverageMode::NotConductors,
    ),
    row(
        "P-069",
        "Legible labeled constellation",
        "State honesty & legibility",
        CoverageMode::NotConductors,
    ),
    row(
        "P-070",
        "Plain-language connection status",
        "State honesty & legibility",
        CoverageMode::NotConductors,
    ),
    row(
        "P-071",
        "Self-explaining empty states",
        "State honesty & legibility",
        CoverageMode::NotConductors,
    ),
    row(
        "P-072",
        "Investigate actions functional",
        "AI-debug climax",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-073",
        "Deterministic env-gated L4 mode",
        "AI-debug climax",
        CoverageMode::Auto,
    ),
    row(
        "P-074",
        "Tier1 incident-path reliability under load",
        "AI-debug climax",
        CoverageMode::Auto,
    ),
    row(
        "P-075",
        "Conductor e2e + delegated-timing verification",
        "External verification",
        CoverageMode::DriveObserve,
    ),
    row(
        "P-076",
        "Integration UX e2e test",
        "Test gap & housekeeping",
        CoverageMode::NotConductors,
    ),
    row(
        "P-077",
        "Demo telemetry injector formalized",
        "Test gap & housekeeping",
        CoverageMode::NotConductors,
    ),
    row(
        "P-078",
        "Agent-headful self-verify harness",
        "Test gap & housekeeping",
        CoverageMode::NotConductors,
    ),
    row(
        "P-079",
        "Constellation severity live-wiring",
        "Operator-surfaced",
        CoverageMode::Auto,
    ),
    row(
        "P-080",
        "Incidents dropdown bounded popover",
        "Operator-surfaced",
        CoverageMode::NotConductors,
    ),
    row(
        "P-081",
        "Traces table live refresh",
        "Operator-surfaced",
        CoverageMode::NotConductors,
    ),
    row(
        "P-082",
        "Traces table internal scroll",
        "Operator-surfaced",
        CoverageMode::NotConductors,
    ),
];

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::CapabilityManifest;

    fn committed_manifest() -> CapabilityManifest {
        let path: PathBuf =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/pulse-capabilities.toml");
        CapabilityManifest::load(&path).expect("committed manifest loads")
    }

    #[test]
    fn matrix_covers_every_accepted_capability() {
        let manifest = committed_manifest();
        let accepted: BTreeSet<&str> = manifest.capabilities.iter().map(String::as_str).collect();
        let classified: BTreeSet<&str> = coverage_matrix().iter().map(|r| r.p_id).collect();
        assert_eq!(
            classified, accepted,
            "the classification must equal the accepted capability set — zero unclassified, zero orphans"
        );
    }

    #[test]
    fn p_ids_are_well_formed_sorted_and_unique() {
        let nums: Vec<u32> = coverage_matrix()
            .iter()
            .map(|r| {
                let n = r.p_id.strip_prefix("P-").expect("p_id starts with `P-`");
                assert_eq!(n.len(), 3, "p_id is zero-padded to 3 digits: {}", r.p_id);
                n.parse::<u32>().expect("numeric p_id suffix")
            })
            .collect();
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        assert_eq!(nums, sorted, "rows are in ascending p_id order");
        assert_eq!(
            nums.iter().collect::<BTreeSet<_>>().len(),
            nums.len(),
            "no duplicate p_id"
        );
    }

    #[test]
    fn every_row_carries_a_title_and_category() {
        for r in coverage_matrix() {
            assert!(!r.title.trim().is_empty(), "{} has no title", r.p_id);
            assert!(!r.category.trim().is_empty(), "{} has no category", r.p_id);
        }
    }

    #[test]
    fn the_out_of_remit_set_is_explicitly_enumerated() {
        let not_ours: BTreeSet<&str> = coverage_matrix()
            .iter()
            .filter(|r| r.mode == CoverageMode::NotConductors)
            .map(|r| r.p_id)
            .collect();
        // Pulse's UI/visual set plus its own tooling — a recorded decision, never an absent row
        // (arch §Cross-cutting Patterns "Scope law"; intent §5 Explicit boundary).
        let expected: BTreeSet<&str> = [
            "P-061", "P-062", "P-063", "P-064", "P-065", "P-066", "P-068", "P-069", "P-070",
            "P-071", "P-076", "P-077", "P-078", "P-080", "P-081", "P-082",
        ]
        .into_iter()
        .collect();
        assert_eq!(not_ours, expected);
    }

    #[test]
    fn the_in_lane_capabilities_are_classified_verifiable() {
        for (id, mode) in [
            ("P-073", CoverageMode::Auto),
            ("P-074", CoverageMode::Auto),
            ("P-079", CoverageMode::Auto),
            ("P-067", CoverageMode::DriveObserve),
            ("P-072", CoverageMode::DriveObserve),
            ("P-075", CoverageMode::DriveObserve),
        ] {
            let row = coverage_matrix()
                .iter()
                .find(|r| r.p_id == id)
                .expect("in-lane row present");
            assert_eq!(row.mode, mode, "{id} must be {}", mode.label());
        }
    }

    #[test]
    fn not_conductors_wire_spelling_is_stable() {
        assert_eq!(
            serde_json::to_string(&CoverageMode::NotConductors).unwrap(),
            "\"not-conductors\""
        );
        assert_eq!(CoverageMode::NotConductors.label(), "not-conductors");
    }

    #[test]
    fn named_static_only_anchors_are_static_only() {
        // input.md §Coverage classification names these explicitly as static-only.
        for id in ["P-038", "P-040", "P-046", "P-049", "P-051", "P-054"] {
            let row = coverage_matrix()
                .iter()
                .find(|r| r.p_id == id)
                .expect("anchor present");
            assert_eq!(
                row.mode,
                CoverageMode::StaticOnly,
                "{id} must be static-only"
            );
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
            assert_eq!(
                serde_json::to_string(&mode).unwrap(),
                format!("\"{}\"", mode.label())
            );
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
