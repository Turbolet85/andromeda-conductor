//! The pinned MCP contract manifest — the single source of truth for the protocol version + required
//! read-back tool names the preflight gate asserts against (arch §Standard Contracts; security-plan
//! §Input Validation). Loaded from `contracts/mcp-contract.toml` (or the `CONDUCTOR_CONTRACT_MANIFEST`
//! override, which the cli edge resolves + canonicalizes — not here) and bounds-checked at load.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::client::{
    MARK_INCIDENT_RESOLVED, QUERY_INCIDENT_LIST, RETRIEVE_REPORT, RETRIEVE_TELEMETRY_SLICE,
};
use crate::error::VerifyError;

/// The four read-back tools Conductor's contract pins (arch §Occupied Resources) — the set a valid
/// manifest must declare and the preflight gate asserts present.
pub const READBACK_TOOLS: [&str; 4] = [
    QUERY_INCIDENT_LIST,
    RETRIEVE_REPORT,
    RETRIEVE_TELEMETRY_SLICE,
    MARK_INCIDENT_RESOLVED,
];

/// The pinned contract the preflight gate checks: the expected protocol version + required tools.
#[derive(Debug, Clone, Deserialize)]
pub struct ContractManifest {
    pub expected_protocol_version: String,
    pub required_tools: Vec<String>,
}

impl ContractManifest {
    /// The default manifest path, relative to the workspace root. A `CONDUCTOR_CONTRACT_MANIFEST`
    /// override is read + `resolve_under`-canonicalized at the cli edge (Epoch 8), never here.
    pub fn default_path() -> PathBuf {
        PathBuf::from("contracts/mcp-contract.toml")
    }

    /// Read + bounds-check a manifest from an already-resolved path. A read / parse / bounds failure
    /// is a harness fault ([`VerifyError::Manifest`]), never a verification verdict.
    pub fn load(path: &Path) -> Result<Self, VerifyError> {
        let text = std::fs::read_to_string(path).map_err(|e| VerifyError::Manifest {
            // never the path itself — io::Error's Display leaks it (artifact hygiene).
            reason: format!("could not read contract manifest ({:?})", e.kind()),
        })?;
        let manifest: ContractManifest = toml::from_str(&text).map_err(|e| VerifyError::Manifest {
            reason: format!("invalid TOML: {e}"),
        })?;
        manifest.validate()?;
        Ok(manifest)
    }

    fn validate(&self) -> Result<(), VerifyError> {
        if self.expected_protocol_version.trim().is_empty() {
            return Err(VerifyError::Manifest {
                reason: "expected_protocol_version is empty".to_string(),
            });
        }
        if self.required_tools.iter().any(|t| t.trim().is_empty()) {
            return Err(VerifyError::Manifest {
                reason: "a required_tools entry is empty".to_string(),
            });
        }
        for pinned in READBACK_TOOLS {
            if !self.required_tools.iter().any(|t| t == pinned) {
                return Err(VerifyError::Manifest {
                    reason: format!("required_tools is missing the pinned tool {pinned:?}"),
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pinned_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/mcp-contract.toml")
    }

    #[test]
    fn loads_and_bounds_checks_the_committed_manifest() {
        let m = ContractManifest::load(&pinned_path()).expect("pinned manifest loads");
        assert_eq!(m.expected_protocol_version, "2024-11-05");
        for pinned in READBACK_TOOLS {
            assert!(m.required_tools.iter().any(|t| t == pinned), "missing {pinned}");
        }
    }

    #[test]
    fn missing_file_is_a_manifest_fault() {
        let err = ContractManifest::load(Path::new("contracts/does-not-exist.toml")).unwrap_err();
        assert!(matches!(err, VerifyError::Manifest { .. }));
    }

    #[test]
    fn rejects_empty_version() {
        let m = ContractManifest {
            expected_protocol_version: "  ".to_string(),
            required_tools: READBACK_TOOLS.iter().map(|s| s.to_string()).collect(),
        };
        assert!(matches!(m.validate(), Err(VerifyError::Manifest { .. })));
    }

    #[test]
    fn rejects_a_dropped_pinned_tool() {
        let m = ContractManifest {
            expected_protocol_version: "2024-11-05".to_string(),
            required_tools: vec![QUERY_INCIDENT_LIST.to_string(), RETRIEVE_REPORT.to_string()],
        };
        assert!(matches!(m.validate(), Err(VerifyError::Manifest { .. })));
    }
}
