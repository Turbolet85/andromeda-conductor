//! The guarded path handles every capture-reading test binary resolves through: the live-suite
//! capture (`live_suite.rs`), the real-model capture (`real_model_live.rs`) and the journal
//! conformance gate (`journal_conformance.rs`). A `tests/` subdirectory module, so it is never a
//! test target of its own; its hermetic cases live in `capture_paths_guard.rs`.
//!
//! Each guard takes the handle's VALUE as a parameter — the caller reads the environment — so both
//! branches are testable without mutating process env (test-plan §11 → Integration). Every error
//! names the handle and a fixed reason, never the value or a resolved path: a test binary's failure
//! text is subject to the host-path rule too (security-plan §Security Anti-Patterns → Logging).
#![allow(dead_code)]

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use conductor_core::CoreError;

/// The workspace root — cargo runs a test binary with its PACKAGE root as the cwd, so a
/// repo-relative handle is anchored here rather than at `crates/conductor-run/`.
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// `CONDUCTOR_RUNS_DIR` resolved under `root` through the shipped traversal guard: unset means
/// `runs`, and an absolute or `..`-escaping value is rejected, never honoured or clamped.
pub fn runs_dir_from(root: &Path, handle: Option<&str>) -> Result<PathBuf, String> {
    conductor_core::resolve_under(root, Path::new(handle.unwrap_or("runs"))).map_err(|e| match e {
        // The inner text is a fixed literal; the error's `Display` would prefix
        // `scenario config error:`, which misnames a path-handle fault.
        CoreError::Config(reason) => format!("CONDUCTOR_RUNS_DIR rejected: {reason}"),
        _ => "CONDUCTOR_RUNS_DIR rejected".to_string(),
    })
}

/// The live Pulse's `logs/` directory under `ANDROMEDA_PULSE_DATA_DIR`. The data dir is
/// legitimately absolute (it is Pulse's, outside the workspace), so it is canonicalized and must
/// be a directory rather than rooted under the workspace.
pub fn pulse_logs_dir_from(value: Option<&OsStr>) -> Result<PathBuf, String> {
    let value = value.ok_or_else(|| "ANDROMEDA_PULSE_DATA_DIR unset".to_string())?;
    let canonical = std::fs::canonicalize(value)
        .map_err(|e| format!("ANDROMEDA_PULSE_DATA_DIR rejected: {}", e.kind()))?;
    if !canonical.is_dir() {
        return Err("ANDROMEDA_PULSE_DATA_DIR rejected: not a directory".to_string());
    }
    Ok(canonical.join("logs"))
}
