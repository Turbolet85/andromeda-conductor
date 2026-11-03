//! The verification-ledger gate — Conductor's own delivery is coverage-tracked, and the tracking
//! is complete (v2-21; test-plan §1 Critical Path 6).
//!
//! The subject is each version's `coverage.md`, the store's render of that version's ledger: a
//! capability line names every capability once, as a column-0 list item opening with its id in
//! either form the ledger contract allows — `- {id} · ` or `- **{id}** · `. Three arms hold over
//! every version that keeps a ledger: its file declares at least one capability id (a file
//! declaring none fails rather than passing vacuously), the ids are unique, and no Pulse `P-NNN`
//! stands at an id position (P-IDs belong in acceptance text, never in the id space — the two id
//! spaces are kept visibly separate).
//!
//! An earlier form held the id sets of `requirements.md` and `verification-matrix.json` EQUAL.
//! That arm left with its subject: the two files retired once a requirement's line came to live on
//! its capability's own record, so no second file remains for a set to be compared against.
//!
//! The version directories are RESOLVED BY SCAN rather than named: a baked `conductor-0.2.0`
//! would silently stop covering the ledger at 0.3.0, which is the failure mode this gate exists
//! to prevent. A version predating the ledger renders the `no ledger` line and is skipped by that
//! line rather than by a version literal.
//!
//! The ID SPACE is matched the same way and for the same reason: by SHAPE (`v{major}-{index}`),
//! never a prefix literal. A baked `v2-` stops covering the ledger the moment the version
//! advances — the identical failure one axis over, and the one this gate went red on at 0.3.0.
//!
//! Paths resolve from `CARGO_MANIFEST_DIR`, never a CWD-relative default: a cargo test binary runs
//! with its CWD at the package root, where no version directory exists.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every version directory carrying a `coverage.md`, sorted for a stable failure message.
fn coverage_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(repo_root())
        .expect("the workspace root is readable")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.join("coverage.md").is_file())
        .collect();
    dirs.sort();
    dirs
}

/// What one `coverage.md` is to this gate.
#[derive(Debug, PartialEq)]
enum Coverage {
    /// The version keeps no ledger: its file reads the `no ledger` line.
    NoLedger,
    /// The tokens standing at a capability line's id position, in file order.
    Ledger(Vec<String>),
}

fn read_coverage(markdown: &str) -> Coverage {
    if markdown.lines().any(|l| l == "no ledger") {
        return Coverage::NoLedger;
    }
    Coverage::Ledger(markdown.lines().filter_map(id_position).collect())
}

const SEPARATOR: &str = " · ";

/// The token at a capability line's id position: a column-0 list item opening `- {id} · ` or
/// `- **{id}** · `. Any other line has no id position.
fn id_position(line: &str) -> Option<String> {
    let rest = line.strip_prefix("- ")?;
    let token = match rest.strip_prefix("**") {
        Some(bolded) => {
            let (token, after) = bolded.split_once("**")?;
            after.strip_prefix(SEPARATOR)?;
            token
        }
        None => rest.split_once(SEPARATOR)?.0,
    };
    (!token.is_empty() && !token.contains(char::is_whitespace)).then(|| token.to_string())
}

/// The declared capability ids: the id-position tokens in the version id space.
fn capability_ids(tokens: &[String]) -> Vec<String> {
    tokens
        .iter()
        .filter(|t| is_version_capability_id(t))
        .cloned()
        .collect()
}

/// Every id a file repeats, once per repeat.
fn duplicates(ids: &[String]) -> Vec<String> {
    let mut seen: Vec<&str> = Vec::new();
    let mut repeated = Vec::new();
    for id in ids {
        if seen.contains(&id.as_str()) {
            repeated.push(id.clone());
        } else {
            seen.push(id);
        }
    }
    repeated
}

fn p_id_positions(tokens: &[String]) -> Vec<String> {
    tokens
        .iter()
        .filter(|t| is_pulse_p_id(t))
        .cloned()
        .collect()
}

fn is_pulse_p_id(s: &str) -> bool {
    match s.strip_prefix("P-") {
        Some(digits) => !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()),
        None => false,
    }
}

/// The version-capability id space, matched by SHAPE: `v{major}-{index}`, digits on both sides.
fn is_version_capability_id(s: &str) -> bool {
    match s.strip_prefix('v').and_then(|rest| rest.split_once('-')) {
        Some((major, index)) => {
            !major.is_empty()
                && !index.is_empty()
                && major.bytes().all(|b| b.is_ascii_digit())
                && index.bytes().all(|b| b.is_ascii_digit())
        }
        None => false,
    }
}

struct Ledger {
    label: String,
    tokens: Vec<String>,
}

fn ledgers() -> Vec<Ledger> {
    let dirs = coverage_dirs();
    assert!(
        !dirs.is_empty(),
        "no version directory carries a coverage.md — the gate would pass vacuously"
    );

    let ledgers: Vec<Ledger> = dirs
        .into_iter()
        .filter_map(|dir| {
            let label = dir
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| dir.display().to_string());
            let markdown = std::fs::read_to_string(dir.join("coverage.md"))
                .unwrap_or_else(|e| panic!("{label}/coverage.md is readable ({})", e.kind()));
            match read_coverage(&markdown) {
                Coverage::NoLedger => None,
                Coverage::Ledger(tokens) => Some(Ledger { label, tokens }),
            }
        })
        .collect();
    assert!(
        !ledgers.is_empty(),
        "every coverage.md reads `no ledger` — the gate would pass vacuously"
    );
    ledgers
}

#[test]
fn every_ledger_declares_a_capability() {
    for ledger in ledgers() {
        assert!(
            !capability_ids(&ledger.tokens).is_empty(),
            "{}/coverage.md keeps a ledger and declares no version-capability id — the gate \
             would pass vacuously",
            ledger.label
        );
    }
}

#[test]
fn capability_ids_are_unique() {
    for ledger in ledgers() {
        let repeated = duplicates(&capability_ids(&ledger.tokens));
        assert!(
            repeated.is_empty(),
            "{}/coverage.md repeats {} capability id(s): {repeated:?}",
            ledger.label,
            repeated.len()
        );
    }
}

#[test]
fn no_pulse_p_id_occupies_an_id_position() {
    for ledger in ledgers() {
        let offenders = p_id_positions(&ledger.tokens);
        assert!(
            offenders.is_empty(),
            "{}/coverage.md: a Pulse P-ID occupies an id position — the version-capability and \
             P-NNN id spaces must stay visibly separate, with P-IDs confined to acceptance text: \
             {offenders:?}",
            ledger.label
        );
    }
}

fn ledger_tokens(markdown: &str) -> Vec<String> {
    match read_coverage(markdown) {
        Coverage::Ledger(tokens) => tokens,
        Coverage::NoLedger => panic!("a ledger fixture read as `no ledger`"),
    }
}

#[test]
fn the_gate_discriminates() {
    // A gate that cannot fail is not evidence. Each arm is driven against an input it must reject.
    // Both line forms, across versions, are extracted alike — a prefix literal or a one-form
    // parser fails exactly here.
    let both_forms = capability_ids(&ledger_tokens(
        "# Coverage — x\n\n## verified (3)\n- v2-01 · a\n- **v3-11** · b\n- v10-7 · c · marker\n",
    ));
    assert_eq!(
        both_forms,
        vec![
            "v2-01".to_string(),
            "v3-11".to_string(),
            "v10-7".to_string()
        ],
        "the bare and the bolded form, of different versions, are extracted alike"
    );

    let prose_title = ledger_tokens(
        "- **`secret-scanning-ci-gate` — unrealized security-plan bootstrap item.**\n",
    );
    assert!(
        prose_title.is_empty(),
        "a bolded prose title is not a declared id, got {prose_title:?}"
    );

    let bolded_p_id = ledger_tokens("- **P-017** · a\n");
    assert!(
        capability_ids(&bolded_p_id).is_empty(),
        "the Pulse id space stays out of the version id space, got {bolded_p_id:?}"
    );

    let prose_only = ledger_tokens(
        "v2-01 mentioned in prose, not declared\n- not an id\n  - v2-02 · indented\n\
         * v2-03 · another bullet\n-v2-04 · no space\n- v2-05 without the separator\n",
    );
    assert!(
        capability_ids(&prose_only).is_empty(),
        "only a column-0 `- ` item opening with the id and its separator declares, got \
         {prose_only:?}"
    );

    let repeated = duplicates(&capability_ids(&ledger_tokens(
        "- v4-01 · a\n- **v4-02** · b\n- v4-01 · a again\n",
    )));
    assert_eq!(
        repeated,
        vec!["v4-01".to_string()],
        "a duplicate id is caught, across the two forms too"
    );
    assert!(duplicates(&["v4-01".to_string(), "v4-02".to_string()]).is_empty());

    let p_id_lines = p_id_positions(&ledger_tokens("- v4-01 · a\n- P-017 · b\n- **P-9** · c\n"));
    assert_eq!(
        p_id_lines,
        vec!["P-017".to_string(), "P-9".to_string()],
        "a P-ID at an id position is caught in either form"
    );

    assert_eq!(
        read_coverage("# Coverage — conductor-0.1.0\n\nno ledger\n"),
        Coverage::NoLedger,
        "a `no ledger` file is skipped by its line"
    );
    let empty_ledger = ledger_tokens(
        "# Coverage — x\n\nverified 0/0 · deferred 0 · planned 0 · implemented 0 · unclaimed 0\n",
    );
    assert!(
        capability_ids(&empty_ledger).is_empty(),
        "a ledger file with zero capability lines yields no id, which the declares arm fails"
    );

    assert!(!is_pulse_p_id("v2-01"));
    assert!(!is_pulse_p_id("P-"), "a bare prefix is not a P-ID");
    assert!(!is_pulse_p_id("P-01a"), "digits only");

    assert!(is_version_capability_id("v2-01"));
    assert!(is_version_capability_id("v3-11"));
    assert!(
        is_version_capability_id("v10-7"),
        "the shape is not bounded to one digit per side"
    );
    assert!(!is_version_capability_id("v"), "a bare prefix is not an id");
    assert!(!is_version_capability_id("v2-"), "digits on both sides");
    assert!(!is_version_capability_id("v-01"), "digits on both sides");
    assert!(!is_version_capability_id("v2-0a"), "digits only");
    assert!(
        !is_version_capability_id("P-017"),
        "the two id spaces stay disjoint"
    );
}
