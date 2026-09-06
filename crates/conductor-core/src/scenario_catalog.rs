//! Read-only scenario-catalog enumeration — shared by the CLI suite loader and the Tauri picker.
//!
//! [`scenario_files`] is the single sorted `*.toml` enumerator both surfaces use (DRY); [`list_scenarios`]
//! reads each into an identity-only [`ScenarioSummary`] for the desktop picker, and [`validate_selection`]
//! is the trust boundary the Tauri `start_run` command checks a picked selection against before any use
//! (security-plan §Input Validation). No execution here — listing + identity only.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{CapabilityManifest, CoreError, PId, Scenario, SloTier};

/// Sentinel selection meaning "the whole catalog" — the picker's suite entry. Distinct from any
/// scenario name (no scenario is named with leading underscores).
pub const SUITE_SELECTION: &str = "__suite__";

/// A scenario's identity for the picker — name, the Pulse P-ID(s) it exercises, its SLO tier. No
/// phase spec or expected checks: the picker renders identity, it does not run the scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScenarioSummary {
    pub name: String,
    pub p_ids: Vec<PId>,
    pub slo_tier: SloTier,
}

/// The sorted `*.toml` files in the scenario catalog directory `dir`.
pub fn scenario_files(dir: &Path) -> crate::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| CoreError::Config(format!("read scenarios directory: {e}")))?
        .flatten()
        .map(|entry| entry.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("toml"))
        .collect();
    files.sort();
    Ok(files)
}

/// Read + validate every scenario in `dir`, returning identity summaries (catalog file order). A
/// malformed scenario is a harness fault (`Err`) — the picker shows the operator a real error, never
/// a silently truncated list.
pub fn list_scenarios(
    dir: &Path,
    capabilities: &CapabilityManifest,
) -> crate::Result<Vec<ScenarioSummary>> {
    let mut summaries = Vec::new();
    for path in scenario_files(dir)? {
        let text = std::fs::read_to_string(&path)
            .map_err(|e| CoreError::Config(format!("read scenario file: {e}")))?;
        let scenario = Scenario::from_toml_str_with(&text, capabilities)?;
        summaries.push(ScenarioSummary {
            name: scenario.name,
            p_ids: scenario.p_ids,
            slo_tier: scenario.slo_tier,
        });
    }
    Ok(summaries)
}

/// Whether `selection` is a runnable choice against `catalog`: the suite sentinel, a known scenario
/// name, or a known Pulse P-ID. The picker only emits these; `start_run` rejects anything else so a
/// selection string never flows unchecked toward execution (security-plan §Input Validation).
pub fn validate_selection(catalog: &[ScenarioSummary], selection: &str) -> bool {
    selection == SUITE_SELECTION
        || catalog
            .iter()
            .any(|s| s.name == selection || s.p_ids.iter().any(|p| p.0 == selection))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed catalog at the workspace root — the real fixture (every entry must parse and
    /// every P-ID it names must be one the committed capability manifest claims).
    fn catalog() -> Vec<ScenarioSummary> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let capabilities =
            CapabilityManifest::load(&root.join("contracts/pulse-capabilities.toml"))
                .expect("the committed capability manifest loads");
        list_scenarios(&root.join("scenarios"), &capabilities)
            .expect("the committed scenario catalog parses")
    }

    #[test]
    fn lists_the_committed_catalog_with_identity() {
        let summaries = catalog();
        assert!(summaries.len() >= 2, "catalog should be non-trivial");
        assert!(summaries.iter().any(|s| s.name == "error-baseline-spike"));
        assert!(
            summaries.iter().all(|s| !s.p_ids.is_empty()),
            "every summary carries a P-ID"
        );
    }

    #[test]
    fn validate_selection_accepts_suite_name_and_pid() {
        let summaries = catalog();
        assert!(validate_selection(&summaries, SUITE_SELECTION));
        assert!(validate_selection(&summaries, "error-baseline-spike"));
        let pid = &summaries
            .iter()
            .flat_map(|s| &s.p_ids)
            .next()
            .expect("catalog has P-IDs")
            .0;
        assert!(validate_selection(&summaries, pid));
    }

    #[test]
    fn validate_selection_rejects_unknown() {
        let summaries = catalog();
        assert!(!validate_selection(&summaries, "no-such-scenario"));
        assert!(!validate_selection(&summaries, "P-999"));
        assert!(!validate_selection(&summaries, ""));
    }
}
