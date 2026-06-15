//! `CONDUCTOR_*` path-handle guard — the one sync filesystem helper in `conductor-core`.
//!
//! Resolves an operator-supplied path handle (`CONDUCTOR_RUNS_DIR` / `CONDUCTOR_SCENARIOS_DIR` /
//! `CONDUCTOR_CONTRACT_MANIFEST`) against a known base directory and rejects anything that escapes
//! it — a `..` traversal, an absolute path, or a symlink that climbs out. The guard is
//! runtime-agnostic; reading the env vars and choosing the base happen at the cli edge (a later
//! chunk), which calls this before any path use (security-plan §Input Validation).

use std::path::{Component, Path, PathBuf};

use crate::CoreError;

/// Resolve `candidate` against the existing directory `base`, returning the path only if it stays
/// within `base`. A `..` component, an absolute `candidate`, or a resolved path outside `base` is
/// rejected as a [`CoreError`] — never silently clamped. `base` must exist (it is canonicalized);
/// `candidate` need not (a not-yet-created run dir is fine, as long as it cannot climb out).
pub fn resolve_under(base: &Path, candidate: &Path) -> crate::Result<PathBuf> {
    let canonical_base = base
        .canonicalize()
        .map_err(|e| CoreError::Config(format!("base directory is not resolvable: {e}")))?;

    if candidate.is_absolute() {
        return Err(CoreError::Config(
            "path handle must be relative to its base directory".to_string(),
        ));
    }
    if candidate.components().any(|c| c == Component::ParentDir) {
        return Err(CoreError::Config(
            "path handle must not contain a `..` component".to_string(),
        ));
    }

    let joined = canonical_base.join(candidate);
    // Canonicalize when the target exists (catches symlink escapes); otherwise the lexical guards
    // above already guarantee containment for a not-yet-created path.
    let resolved = joined.canonicalize().unwrap_or(joined);

    if resolved.starts_with(&canonical_base) {
        Ok(resolved)
    } else {
        Err(CoreError::Config(
            "path handle escapes its permitted base directory".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crate_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn accepts_an_in_scope_relative_path() {
        // `src/` exists under the crate root, so it canonicalizes and stays within base.
        let resolved = resolve_under(&crate_root(), Path::new("src")).expect("src is in scope");
        assert!(resolved.ends_with("src"));
    }

    #[test]
    fn rejects_parent_dir_traversal() {
        let err = resolve_under(&crate_root(), Path::new("../../..")).unwrap_err();
        assert!(matches!(err, CoreError::Config(_)));
    }

    #[test]
    fn rejects_an_out_of_scope_absolute_path() {
        let outside = std::env::temp_dir();
        let err = resolve_under(&crate_root(), &outside).unwrap_err();
        assert!(matches!(err, CoreError::Config(_)));
    }
}
