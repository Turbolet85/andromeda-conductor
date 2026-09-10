//! The verification-ledger gate — Conductor's own delivery is coverage-tracked, and the tracking
//! is complete (v2-21; test-plan §1 Critical Path 6).
//!
//! Three arms over every version directory that carries BOTH ledger files: the id sets in
//! `requirements.md` and `verification-matrix.json` are EQUAL, the matrix's ids are unique, and no
//! Pulse `P-NNN` occupies an id position (P-IDs belong in acceptance text, never in the id space —
//! the two id spaces are kept visibly separate).
//!
//! Set equality, never a count: a literal `32` would pass while one id was swapped for another,
//! and it would have to be edited by hand every time a capability is added (test-plan §1 forbids
//! the count assertion for exactly that reason).
//!
//! The version directories are RESOLVED BY SCAN rather than named: a baked `conductor-0.2.0`
//! would silently stop covering the ledger at 0.3.0, which is the failure mode this gate exists
//! to prevent. A version predating the matrix feature carries no `verification-matrix.json` and is
//! skipped by the both-files filter rather than by a version literal.
//!
//! Paths resolve from `CARGO_MANIFEST_DIR`, never a CWD-relative default: a cargo test binary runs
//! with its CWD at the package root, where no version directory exists.

use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every version directory carrying both ledger files, sorted for a stable failure message.
fn ledger_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(repo_root())
        .expect("the workspace root is readable")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.join("requirements.md").is_file() && p.join("verification-matrix.json").is_file()
        })
        .collect();
    dirs.sort();
    dirs
}

/// The declaring form in `requirements.md`: a list item whose first token is the bolded id.
fn requirement_ids(markdown: &str) -> Vec<String> {
    markdown
        .lines()
        .filter_map(|l| l.strip_prefix("- **"))
        .filter_map(|rest| rest.split_once("**").map(|(id, _)| id.to_string()))
        .filter(|id| id.starts_with("v2-"))
        .collect()
}

/// Every value sitting at an `id` key, at any depth — "an id position anywhere in the matrix".
fn id_positions(node: &Value, path: &str, found: &mut Vec<(String, String)>) {
    match node {
        Value::Object(map) => {
            for (key, value) in map {
                if key == "id"
                    && let Some(s) = value.as_str()
                {
                    found.push((format!("{path}/id"), s.to_string()));
                }
                id_positions(value, &format!("{path}/{key}"), found);
            }
        }
        Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                id_positions(item, &format!("{path}[{i}]"), found);
            }
        }
        _ => {}
    }
}

/// `(declared-but-absent, present-but-undeclared)` — both directions, so a failure names which.
fn set_difference(required: &[String], matrix: &[String]) -> (Vec<String>, Vec<String>) {
    let missing = required
        .iter()
        .filter(|id| !matrix.contains(id))
        .cloned()
        .collect();
    let extra = matrix
        .iter()
        .filter(|id| !required.contains(id))
        .cloned()
        .collect();
    (missing, extra)
}

fn is_pulse_p_id(s: &str) -> bool {
    match s.strip_prefix("P-") {
        Some(digits) => !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()),
        None => false,
    }
}

struct Ledger {
    label: String,
    requirement_ids: Vec<String>,
    id_positions: Vec<(String, String)>,
}

fn ledgers() -> Vec<Ledger> {
    let dirs = ledger_dirs();
    assert!(
        !dirs.is_empty(),
        "no version directory carries both requirements.md and verification-matrix.json — \
         the gate would pass vacuously"
    );

    dirs.into_iter()
        .map(|dir| {
            let label = dir
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| dir.display().to_string());

            let markdown = std::fs::read_to_string(dir.join("requirements.md"))
                .unwrap_or_else(|e| panic!("{label}/requirements.md is readable ({})", e.kind()));
            let raw =
                std::fs::read_to_string(dir.join("verification-matrix.json")).unwrap_or_else(|e| {
                    panic!(
                        "{label}/verification-matrix.json is readable ({})",
                        e.kind()
                    )
                });
            let json: Value = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("{label}/verification-matrix.json parses: {e}"));

            let mut positions = Vec::new();
            id_positions(&json, "", &mut positions);

            Ledger {
                label,
                requirement_ids: requirement_ids(&markdown),
                id_positions: positions,
            }
        })
        .collect()
}

#[test]
fn every_requirement_capability_has_exactly_one_matrix_entry() {
    for ledger in ledgers() {
        let Ledger {
            label,
            requirement_ids: required,
            id_positions: positions,
        } = ledger;

        assert!(
            !required.is_empty(),
            "{label}/requirements.md declares no v2-NN capability — the gate would pass vacuously"
        );
        assert!(
            !positions.is_empty(),
            "{label}/verification-matrix.json holds no id — the gate would pass vacuously"
        );

        let matrix: Vec<String> = positions.iter().map(|(_, id)| id.clone()).collect();

        let (missing, extra) = set_difference(&required, &matrix);

        assert!(
            missing.is_empty() && extra.is_empty(),
            "{label}: the requirements and matrix id sets differ — \
             declared in requirements.md but absent from the matrix: {missing:?}; \
             present in the matrix but not declared in requirements.md: {extra:?} \
             ({} requirement ids, {} matrix ids)",
            required.len(),
            matrix.len()
        );
    }
}

#[test]
fn matrix_ids_are_unique() {
    for ledger in ledgers() {
        let mut seen: Vec<&str> = Vec::new();
        let mut duplicates: Vec<&str> = Vec::new();
        for (_, id) in &ledger.id_positions {
            if seen.contains(&id.as_str()) {
                duplicates.push(id);
            } else {
                seen.push(id);
            }
        }
        assert!(
            duplicates.is_empty(),
            "{}: verification-matrix.json repeats {} id(s): {duplicates:?}",
            ledger.label,
            duplicates.len()
        );
    }
}

#[test]
fn no_pulse_p_id_occupies_an_id_position() {
    for ledger in ledgers() {
        let offenders: Vec<&(String, String)> = ledger
            .id_positions
            .iter()
            .filter(|(_, id)| is_pulse_p_id(id))
            .collect();

        assert!(
            offenders.is_empty(),
            "{}: a Pulse P-ID occupies an id position — the v2-NN and P-NNN id spaces must stay \
             visibly separate, with P-IDs confined to acceptance text: {offenders:?}",
            ledger.label
        );
    }
}

#[test]
fn the_gate_discriminates() {
    // A gate that cannot fail is not evidence. Each arm is driven against an input it must reject.
    let swapped = requirement_ids("- **v2-01** · a\n- **v2-99** · b\n");
    assert_eq!(swapped, vec!["v2-01".to_string(), "v2-99".to_string()]);

    let prose_only = requirement_ids("v2-01 mentioned in prose, not declared\n- not an id\n");
    assert!(
        prose_only.is_empty(),
        "only the bolded list-item declaring form counts, got {prose_only:?}"
    );

    let json: Value = serde_json::from_str(
        r#"{"capabilities":[{"id":"v2-01"},{"id":"P-017"},{"nested":{"id":"P-9"}}]}"#,
    )
    .unwrap();
    let mut positions = Vec::new();
    id_positions(&json, "", &mut positions);
    assert_eq!(
        positions.len(),
        3,
        "id positions at any depth: {positions:?}"
    );

    let flagged: Vec<&String> = positions
        .iter()
        .filter(|(_, id)| is_pulse_p_id(id))
        .map(|(_, id)| id)
        .collect();
    assert_eq!(
        flagged,
        vec!["P-017", "P-9"],
        "the P-ID predicate must catch a nested id position too"
    );

    assert!(!is_pulse_p_id("v2-01"));
    assert!(!is_pulse_p_id("P-"), "a bare prefix is not a P-ID");
    assert!(!is_pulse_p_id("P-01a"), "digits only");

    // The set-equality arm must FAIL on a divergence, in each direction independently —
    // an equal-CARDINALITY swap is the case a count assertion would wave through.
    let required = vec!["v2-01".to_string(), "v2-02".to_string()];
    let swapped = vec!["v2-01".to_string(), "v2-99".to_string()];
    let (missing, extra) = set_difference(&required, &swapped);
    assert_eq!(missing, vec!["v2-02".to_string()], "declared but unmatched");
    assert_eq!(extra, vec!["v2-99".to_string()], "matched but undeclared");

    let (none_missing, none_extra) = set_difference(&required, &required);
    assert!(
        none_missing.is_empty() && none_extra.is_empty(),
        "equal sets must produce no difference"
    );
}
