//! The pinned SUT capability manifest — the accepted Pulse capability set a scenario's P-IDs are
//! checked against at load (arch §Occupied Resources; security-plan §Input Validation).
//!
//! Read from `contracts/pulse-capabilities.toml`; the path is resolved at the binary edge and passed
//! in, exactly as the MCP contract manifest is. A read / parse / bounds failure is a harness fault
//! ([`CoreError::Config`](crate::CoreError::Config)) — never a verdict, never a panic, and never a
//! silent widening back to a hardcoded range. Re-aiming Conductor at a newer Pulse release is an edit
//! to that file, not a Rust change.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::CoreError;

/// The versioned SUT capability set: which Pulse release it was captured from, and every capability
/// id Conductor accepts in a scenario's `p_ids`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CapabilityManifest {
    /// The Pulse release this set was captured from (e.g. `"v0.3.0"`).
    pub sut_version: String,
    /// The capture date, `YYYY-MM-DD`.
    pub captured_at: String,
    /// Every accepted Pulse capability id, in `P-NNN` form.
    pub capabilities: Vec<String>,
}

impl CapabilityManifest {
    /// The default manifest path, relative to the workspace root. The `conductor-cli` / Tauri edge
    /// resolves it under the current dir before calling [`load`](Self::load) — never this module.
    pub fn default_path() -> PathBuf {
        PathBuf::from("contracts/pulse-capabilities.toml")
    }

    /// Read + bounds-check the manifest from an already-resolved path.
    pub fn load(path: &Path) -> crate::Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|e| {
            // never the path itself — io::Error's Display leaks it (artifact hygiene).
            CoreError::Config(format!("could not read capability manifest ({:?})", e.kind()))
        })?;
        let manifest: Self = toml::from_str(&text).map_err(|e| {
            CoreError::Config(format!("invalid capability manifest: {}", crate::sanitize_error(&e)))
        })?;
        manifest.validate()?;
        tracing::info!(
            count = manifest.capabilities.len(),
            "loaded capability manifest for Pulse {}",
            manifest.sut_version
        );
        Ok(manifest)
    }

    /// Whether `p_id` is in the accepted set.
    pub fn accepts(&self, p_id: &str) -> bool {
        self.capabilities.iter().any(|c| c == p_id)
    }

    fn validate(&self) -> crate::Result<()> {
        if self.sut_version.trim().is_empty() {
            return Err(CoreError::Config("capability manifest: sut_version is empty".to_string()));
        }
        if self.captured_at.trim().is_empty() {
            return Err(CoreError::Config("capability manifest: captured_at is empty".to_string()));
        }
        if self.capabilities.is_empty() {
            return Err(CoreError::Config("capability manifest: capabilities is empty".to_string()));
        }
        let mut seen = std::collections::HashSet::with_capacity(self.capabilities.len());
        for id in &self.capabilities {
            if !crate::scenario::is_pid_shaped(id) {
                return Err(CoreError::Config(format!(
                    "capability manifest: malformed capability id {id:?} (expected P-NNN)"
                )));
            }
            if !seen.insert(id.as_str()) {
                return Err(CoreError::Config(format!(
                    "capability manifest: duplicate capability id {id:?}"
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn loads_and_bounds_checks_the_committed_manifest() {
        let m = CapabilityManifest::load(&committed_path()).expect("committed manifest loads");
        assert_eq!(m.sut_version, "v0.3.0");
        assert!(m.accepts("P-001"), "the original range must still be accepted");
        assert!(m.accepts("P-060"));
        assert!(m.accepts("P-061"), "the id 0.1.0 hard-rejected must now be accepted");
        assert!(m.accepts("P-074"), "Pulse storm-coalescing must be expressible");
        assert!(m.accepts("P-082"), "the ledger head must be accepted");
        assert!(!m.accepts("P-083"), "beyond the captured ledger must be rejected");
    }

    #[test]
    fn missing_file_is_a_harness_fault() {
        let err = CapabilityManifest::load(Path::new("contracts/does-not-exist.toml")).unwrap_err();
        assert!(matches!(err, CoreError::Config(_)));
    }

    #[test]
    fn load_failure_message_never_contains_the_path() {
        let path = committed_path().parent().unwrap().join("no-such-capability-manifest.toml");
        let err = CapabilityManifest::load(&path).unwrap_err().to_string();
        assert!(
            !err.contains("no-such-capability-manifest"),
            "the manifest path must not leak into the error (artifact hygiene): {err}"
        );
        assert!(!err.contains(env!("CARGO_MANIFEST_DIR")), "no absolute host path: {err}");
    }

    #[test]
    fn rejects_empty_version_date_or_set() {
        assert!(matches!(
            CapabilityManifest { sut_version: "  ".to_string(), ..manifest(&["P-001"]) }.validate(),
            Err(CoreError::Config(_))
        ));
        assert!(matches!(
            CapabilityManifest { captured_at: String::new(), ..manifest(&["P-001"]) }.validate(),
            Err(CoreError::Config(_))
        ));
        assert!(matches!(manifest(&[]).validate(), Err(CoreError::Config(_))));
    }

    #[test]
    fn rejects_a_malformed_or_duplicated_capability_id() {
        assert!(matches!(manifest(&["P-001", "P-1"]).validate(), Err(CoreError::Config(_))));
        assert!(matches!(manifest(&["P-001", "Q-002"]).validate(), Err(CoreError::Config(_))));
        assert!(matches!(manifest(&["P-001", "P-001"]).validate(), Err(CoreError::Config(_))));
    }

    #[test]
    fn accepts_only_ids_in_the_set() {
        let m = manifest(&["P-001", "P-074"]);
        assert!(m.accepts("P-001"));
        assert!(m.accepts("P-074"));
        assert!(!m.accepts("P-002"));
    }
}
