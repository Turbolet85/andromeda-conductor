//! Digest pins over committed evidence, shared by the harvests that grade a live round from files.
//! A `tests/` subdirectory module, so it is never a test target of its own.
//!
//! Every capture a harvest grades through here is a committed evidence file held by the sha256 of its
//! LF-normalized content; grading reads the file only after its digest matches, and no capture text
//! sits in test source (security-plan §Security Anti-Patterns → Data Protection). Errors name the
//! repo-relative file and both digests, never the text.
#![allow(dead_code)]

use std::path::Path;

/// The sha256 of `text`, lower hex.
pub fn sha256_hex(text: &str) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Hold a committed file's LF-normalized text to its pinned digest.
pub fn check_digest(name: &str, text: &str, expected: &str) -> Result<(), String> {
    let actual = sha256_hex(text);
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "{name}: sha256 {actual} does not match its pinned {expected}"
        ))
    }
}

/// A committed file, read workspace-root anchored (a test binary's cwd is its own crate) and
/// LF-normalized, so a digest is a property of the content and never of `core.autocrlf`.
pub fn committed(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{name}: committed evidence unreadable: {}", e.kind()))
        .replace("\r\n", "\n")
}

/// A committed capture whose digest matches its pin — the only way a grader reads one.
pub fn pinned(name: &str, sha256: &str) -> String {
    let text = committed(name);
    check_digest(name, &text, sha256).unwrap_or_else(|reason| panic!("{reason}"));
    text
}
