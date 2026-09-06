//! The coverage-completeness gate — every capability the SUT capability manifest accepts is
//! classified into exactly one `CoverageMode`, and the committed `coverage-matrix.md` is the
//! rendering of that classification (test-plan §1 Critical Path 6, arch §Cross-cutting Scope law).
//!
//! Three arms, and the second is the one the crate-internal tests do not have. The POSITIVE arm
//! asserts `check_sut_drift` over the real committed pair. The NEGATIVE arm removes a row from the
//! REAL classification — `drift.rs`'s own negative tests build synthetic `rows(&[…])` fixtures
//! against a fixture manifest, so nothing before this proved the committed classification's removal
//! goes red. The ARTIFACT arm byte-compares the committed file to `CoverageMatrix::render()`, which
//! is what makes a classification change that skips regeneration fail.
//!
//! This lives in `conductor-report` rather than `conductor-core` because only this side of the
//! dependency edge can reach BOTH `conductor_core::check_sut_drift` and `CoverageMatrix::render` —
//! `conductor-report → conductor-core`, never the reverse.
//!
//! Paths resolve from `CARGO_MANIFEST_DIR`, never a CWD-relative default: a cargo test binary runs
//! with its CWD at the package root, where neither `contracts/` nor the artifact exists.

use std::path::{Path, PathBuf};

use conductor_core::{CapabilityManifest, CapabilityRow, coverage_matrix};
use conductor_report::CoverageMatrix;

/// The workspace root, relative to this package.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn manifest() -> CapabilityManifest {
    let path = repo_root().join("contracts/pulse-capabilities.toml");
    CapabilityManifest::load(&path).expect("the committed capability manifest loads")
}

#[test]
fn every_accepted_capability_is_classified() {
    let manifest = manifest();
    assert!(
        !manifest.capabilities.is_empty(),
        "the manifest declares no capabilities — the gate would pass vacuously"
    );

    conductor_core::check_sut_drift(&manifest, coverage_matrix(), &[])
        .expect("the committed classification covers the committed manifest with zero gaps");
}

#[test]
fn removing_a_real_classification_row_fails_the_gate() {
    let manifest = manifest();
    let full = coverage_matrix();

    // Every row in turn: no single capability may be droppable without the gate noticing.
    for dropped in full {
        let reduced: Vec<CapabilityRow> = full
            .iter()
            .filter(|r| r.p_id != dropped.p_id)
            .copied()
            .collect();

        let err = conductor_core::check_sut_drift(&manifest, &reduced, &[])
            .expect_err("a classification missing an accepted capability must be drift");

        let rendered = err.to_string();
        assert!(
            rendered.contains(dropped.p_id),
            "the drift message must name the missing capability {}, got: {rendered}",
            dropped.p_id
        );
    }
}

#[test]
fn the_committed_artifact_is_the_rendered_classification() {
    let path = repo_root().join("coverage-matrix.md");
    let committed = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "coverage-matrix.md is the committed definition-of-done artifact and must be readable \
             from the workspace root ({}); regenerate it with `conductor coverage --write`",
            e.kind()
        )
    });

    assert_eq!(
        committed,
        CoverageMatrix::render(),
        "the committed coverage-matrix.md is stale — regenerate it with `conductor coverage --write`"
    );
}
