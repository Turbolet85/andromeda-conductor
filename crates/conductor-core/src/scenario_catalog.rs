//! Read-only scenario-catalog enumeration — shared by the CLI's scenario loaders and its catalogue
//! listing.
//!
//! [`scenario_files`] is the single sorted `*.toml` enumerator every loader uses (DRY);
//! [`list_scenarios`] reads each into an identity-only [`ScenarioSummary`] for `conductor scenarios`.
//! No execution here — listing + identity only.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{CapabilityManifest, CoreError, PId, Scenario, SloTier};

/// A scenario's identity for the catalogue listing — name, the Pulse P-ID(s) it exercises, its SLO
/// tier. No phase spec or expected checks: the listing renders identity, it does not run the scenario.
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
/// malformed scenario is a harness fault (`Err`) — `conductor scenarios` shows the operator a real
/// error, never a silently truncated list.
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

/// Read + validate every scenario in `dir`, returning the WHOLE [`Scenario`] (catalog file order).
///
/// The sibling of [`list_scenarios`], which returns identity only: a gate judging a scenario against
/// its own phase data — the scenario-audit gate's tier term — needs the phases a
/// [`ScenarioSummary`] deliberately drops. Same validation path, same harness-fault-on-malformed
/// contract.
pub fn load_catalog(dir: &Path, capabilities: &CapabilityManifest) -> crate::Result<Vec<Scenario>> {
    let mut catalog = Vec::new();
    for path in scenario_files(dir)? {
        let text = std::fs::read_to_string(&path)
            .map_err(|e| CoreError::Config(format!("read scenario file: {e}")))?;
        catalog.push(Scenario::from_toml_str_with(&text, capabilities)?);
    }
    Ok(catalog)
}

#[cfg(test)]
mod tests {
    use super::*;

    // andromeda:walks-tree — lists and reads every scenario file in the scenarios dir.
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
}
