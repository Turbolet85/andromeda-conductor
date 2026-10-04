//! Every committed real-model capture, walked from disk and held to the keyed elision.

use crate::*;

// ---- every committed capture -----------------------------------------------------------------------

/// The committed real-model captures — every `conductor-0.3.0/chunks/*/evidence/rm-capture*.txt` —
/// pinned by count, so a walk that finds nothing can never pass.
const COMMITTED_CAPTURES: usize = 14;

/// Every committed real-model capture, repo-relative and sorted.
fn committed_captures() -> Vec<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let chunks = "conductor-0.3.0/chunks";
    let mut names = Vec::new();
    for chunk in std::fs::read_dir(root.join(chunks)).expect("the chunks dir is readable") {
        let chunk = chunk.expect("a chunk entry").file_name();
        let evidence = format!("{chunks}/{}/evidence", chunk.to_string_lossy());
        let Ok(files) = std::fs::read_dir(root.join(&evidence)) else {
            continue;
        };
        for file in files {
            let file = file.expect("an evidence entry").file_name();
            let file = file.to_string_lossy();
            if file.starts_with("rm-capture") && file.ends_with(".txt") {
                names.push(format!("{evidence}/{file}"));
            }
        }
    }
    names.sort();
    names
}

/// How many `fingerprint_hex=` values in `text` are not the placeholder: the value is the ASCII
/// alphanumeric run directly after the `=`, and an empty run is not one.
pub(super) fn un_elided_keyed_values(text: &str) -> usize {
    text.split("fingerprint_hex=")
        .skip(1)
        .filter(|rest| {
            rest.bytes()
                .next()
                .is_some_and(|b| b.is_ascii_alphanumeric())
        })
        .count()
}

#[test]
fn every_committed_capture_carries_no_un_elided_fingerprint_value() {
    // No committed capture keeps a `fingerprint_hex` value, whatever its characters, and the capture's
    // own elision has nothing left to do on any of them. A failure names the file and the count, never
    // the value.
    let names = committed_captures();
    assert_eq!(names.len(), COMMITTED_CAPTURES, "{names:#?}");
    let failures: Vec<String> = names
        .iter()
        .filter_map(|name| {
            let text = committed(name);
            let values = un_elided_keyed_values(&text);
            let fixed = elide_fingerprints(&text) == text;
            (values > 0 || !fixed).then(|| {
                format!(
                    "{name}: {values} un-elided keyed values, a fixed point of the elision: {fixed}"
                )
            })
        })
        .collect();
    assert!(failures.is_empty(), "{failures:#?}");
}
