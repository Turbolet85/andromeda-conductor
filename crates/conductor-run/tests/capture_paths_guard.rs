//! Hermetic cases for the capture-path guards in `capture_paths/mod.rs`. A default-suite target:
//! the captures that use the guards are `live-pulse`-gated, so their own files never run here.
//!
//! Absolute inputs are built at runtime from the OS temp dir, never written as literals, so this
//! source carries no host path. Every rejection is also checked for NOT echoing its input.

mod capture_paths;

use std::path::Path;

use assert_fs::TempDir;
use assert_fs::prelude::*;
use capture_paths::{pulse_logs_dir_from, runs_dir_from, workspace_root};

fn canonical_root() -> std::path::PathBuf {
    workspace_root()
        .canonicalize()
        .expect("workspace root exists")
}

#[test]
fn an_unset_runs_dir_resolves_to_runs_under_the_root() {
    let resolved = runs_dir_from(&workspace_root(), None).expect("the default is in scope");
    assert_eq!(resolved, canonical_root().join("runs"));
}

#[test]
fn a_relative_runs_dir_resolves_under_the_root() {
    let resolved =
        runs_dir_from(&workspace_root(), Some("runs/e2e-fixture")).expect("relative is in scope");
    assert!(resolved.starts_with(canonical_root()), "{resolved:?}");
    assert!(resolved.ends_with(Path::new("runs").join("e2e-fixture")));
}

#[test]
fn an_absolute_runs_dir_is_rejected_without_echoing_it() {
    let outside = std::env::temp_dir();
    let input = outside.to_str().expect("temp dir is UTF-8");
    let err = runs_dir_from(&workspace_root(), Some(input)).unwrap_err();
    assert!(err.contains("CONDUCTOR_RUNS_DIR"), "{err}");
    assert!(
        !err.contains(input),
        "the rejection echoed its input: {err}"
    );
}

#[test]
fn a_parent_escaping_runs_dir_is_rejected() {
    let err = runs_dir_from(&workspace_root(), Some("a/../../x")).unwrap_err();
    assert!(err.contains("CONDUCTOR_RUNS_DIR"), "{err}");
    assert!(err.contains(".."), "{err}");
}

#[test]
fn an_unset_data_dir_is_rejected_by_name() {
    let err = pulse_logs_dir_from(None).unwrap_err();
    assert!(err.contains("ANDROMEDA_PULSE_DATA_DIR"), "{err}");
}

#[test]
fn a_missing_data_dir_is_rejected_without_echoing_it() {
    let temp = TempDir::new().unwrap();
    let missing = temp.path().join("no-such-pulse-dir");
    let err = pulse_logs_dir_from(Some(missing.as_os_str())).unwrap_err();
    assert!(err.contains("ANDROMEDA_PULSE_DATA_DIR"), "{err}");
    assert!(!err.contains(missing.to_str().unwrap()), "{err}");
    assert!(!err.contains(temp.path().to_str().unwrap()), "{err}");
}

#[test]
fn an_existing_absolute_data_dir_resolves_to_its_logs() {
    let temp = TempDir::new().unwrap();
    let logs = pulse_logs_dir_from(Some(temp.path().as_os_str())).expect("an existing dir");
    assert!(logs.is_absolute(), "{logs:?}");
    assert!(logs.ends_with("logs"), "{logs:?}");
}

#[test]
fn a_data_dir_naming_a_file_is_rejected_without_echoing_it() {
    let temp = TempDir::new().unwrap();
    let file = temp.child("not-a-dir.txt");
    file.write_str("x").unwrap();
    let err = pulse_logs_dir_from(Some(file.path().as_os_str())).unwrap_err();
    assert!(err.contains("not a directory"), "{err}");
    assert!(!err.contains(file.path().to_str().unwrap()), "{err}");
}
