//! Artifact-hygiene redaction — the field-name allowlist + value scrubber.
//!
//! Realizes the artifact-hygiene invariant: no absolute host paths reach the self-observation log
//! stream or the operator-facing error edge, and only allowlisted field names appear on a log line
//! (security-plan §Error Handling; obs-plan §11). Two reusable primitives, shared by the `tracing`
//! subscriber ([`crate::obs`]) now and the run-report writers (`runs.db` / JSONL / Markdown report)
//! later: [`redact_value`] masks absolute host-path file references in any string, and
//! [`sanitize_error`] renders an error to a single scrubbed operator-facing line. Std-only — no
//! `regex` (matches the hand-rolled-std convention in [`crate::obs`]).

use std::borrow::Cow;

/// Placeholder substituted for a redacted token — visible, never a silent drop (design-system
/// §Brand Identity: transparency over mystery).
const REDACTED: &str = "<redacted>";

/// Field names permitted on a self-observation JSON line: the 7 flat identity fields, the event
/// `message`, the bounded self-obs event fields, and the reserved Run-report envelope fields
/// (obs-plan §6). A non-allowlisted event field name is dropped at the subscriber's processor
/// stage; an allowlisted field still has its string value scrubbed by [`redact_value`].
const ALLOWLISTED_FIELDS: &[&str] = &[
    // flat identity (obs-plan §3) — also inserted directly by the layer
    "timestamp_ms",
    "level",
    "target",
    "service.name",
    "service.version",
    "deployment.environment",
    "run_id",
    // span-lifecycle record keys (obs-plan §4) — also inserted directly by the layer
    "span",
    "span_event",
    "parent",
    // event message + bounded self-obs event/span fields (obs-plan §6)
    "message",
    "phase",
    "count",
    "panic",
    "location",
    // span attributes carried by the must-trace chain (obs-plan §4 Critical Path 1)
    "row_count",
    "phase_count",
    "emission_count",
    "record_count",
    "mcp_tool",
    // fault-application span attributes (obs-plan §4 Fault-injection spans)
    "fault_type",
    "fault_duration_ms",
    "fault_start_offset_ms",
    "ramp_factor",
    "port",
    // reserved Run-report envelope fields (obs-plan §6; emitted by the Epoch-6 writers)
    "journal_emitted_at",
    "read_back_observed_at",
    "seed",
    "scenario",
    "p_ids",
    "verdict",
    "state",
    "latency_ms",
    "slo_tier",
    "fingerprints",
];

/// Whether `name` may appear on a self-obs line (the field-name allowlist).
pub(crate) fn is_allowlisted(name: &str) -> bool {
    ALLOWLISTED_FIELDS.contains(&name)
}

/// Mask every absolute host-path file reference in `value` with `<redacted>`, preserving the rest
/// of the text and the original whitespace. Returns `Cow::Borrowed` when nothing matches — no
/// allocation on the clean hot path.
///
/// Detection anchors on ABSOLUTE host markers (drive-letter `X:\`/`X:/`, `/home/`, `/Users/`,
/// `/root/`, `%APPDATA%`/`%USERPROFILE%`/`%LOCALAPPDATA%`, `$HOME`, `~/`, `.cargo`/`​.rustup` path
/// segments). A repo-relative source path (`crates/conductor-core/src/obs.rs`) is NOT a host path
/// and is preserved — over-redaction would gut the useful `location`/`target` fields (obs-plan §11;
/// the scope's key correctness edge).
pub fn redact_value(value: &str) -> Cow<'_, str> {
    if !value.split_whitespace().any(is_host_path_token) {
        return Cow::Borrowed(value);
    }
    let mut out = String::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut idx = 0;
    while idx < value.len() {
        let ws_start = idx;
        while idx < value.len() && bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }
        out.push_str(&value[ws_start..idx]);
        if idx >= value.len() {
            break;
        }
        let tok_start = idx;
        while idx < value.len() && !bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }
        let token = &value[tok_start..idx];
        if is_host_path_token(token) {
            out.push_str(REDACTED);
        } else {
            out.push_str(token);
        }
    }
    Cow::Owned(out)
}

/// Render an error to a single scrubbed operator-facing line: its `Display` text (never `Debug`,
/// so no struct-field dump and no `{:?}` backtrace), passed through [`redact_value`], with all
/// whitespace runs collapsed to single spaces (no multi-line stack). The reusable core half of the
/// anyhow-edge sanitization, kept anyhow-free (architecture §Error handling: `anyhow` only at the
/// binary edges) — the cli/tauri edge feeds its `anyhow::Error` (which derefs to `&dyn Error`) and
/// wraps the result in the design-owned `error:` / `hint:` shape (Epoch 8).
pub fn sanitize_error(err: &dyn std::error::Error) -> String {
    let displayed = err.to_string();
    redact_value(&displayed).split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_host_path_token(token: &str) -> bool {
    let bytes = token.as_bytes();
    let drive_absolute = bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/');
    drive_absolute
        || token.contains("/home/")
        || token.contains("/Users/")
        || token.contains("\\Users\\")
        || token.contains("/root/")
        || token.contains("%APPDATA%")
        || token.contains("%USERPROFILE%")
        || token.contains("%LOCALAPPDATA%")
        || token.contains("$HOME")
        || token.starts_with("~/")
        || token.contains(".cargo/")
        || token.contains(".cargo\\")
        || token.contains(".rustup/")
        || token.contains(".rustup\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benign_value_is_borrowed_unchanged() {
        assert!(matches!(
            redact_value("observability initialized"),
            Cow::Borrowed("observability initialized")
        ));
        assert!(matches!(redact_value(""), Cow::Borrowed("")));
    }

    #[test]
    fn repo_relative_source_path_is_kept() {
        // The key correctness edge: a repo-relative source path is NOT a host path.
        let v = "panicked at crates/conductor-core/src/obs.rs:91";
        assert!(matches!(redact_value(v), Cow::Borrowed(_)));
        assert_eq!(redact_value(v), v);
    }

    #[test]
    fn windows_absolute_host_path_is_masked() {
        assert_eq!(
            redact_value("failed to open C:\\Users\\turbo\\data.db for writing"),
            "failed to open <redacted> for writing"
        );
    }

    #[test]
    fn posix_home_paths_are_masked() {
        assert_eq!(redact_value("reading /home/turbo/.config/x"), "reading <redacted>");
        assert_eq!(redact_value("reading /Users/turbo/x"), "reading <redacted>");
        assert_eq!(redact_value("reading /root/secret"), "reading <redacted>");
    }

    #[test]
    fn cargo_and_env_expansion_paths_are_masked() {
        assert_eq!(redact_value("at ~/.cargo/registry/src/foo/lib.rs:42"), "at <redacted>");
        assert_eq!(redact_value("dir %APPDATA%\\andromeda-pulse"), "dir <redacted>");
    }

    #[test]
    fn module_and_type_names_are_not_over_redacted() {
        // `target` / std type names carry `::` but are not host paths — they must survive.
        assert_eq!(redact_value("conductor_core::obs"), "conductor_core::obs");
        assert_eq!(
            redact_value("called Option::unwrap() on a None value"),
            "called Option::unwrap() on a None value"
        );
    }

    #[test]
    fn allowlist_membership() {
        assert!(is_allowlisted("run_id"));
        assert!(is_allowlisted("message"));
        assert!(is_allowlisted("count"));
        assert!(!is_allowlisted("secret_field"));
        assert!(!is_allowlisted("password"));
    }

    #[derive(Debug)]
    struct SyntheticError;

    impl std::fmt::Display for SyntheticError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "open failed at C:\\Users\\turbo\\secret\\corpus.db\n  while loading config"
            )
        }
    }

    impl std::error::Error for SyntheticError {}

    #[test]
    fn sanitize_error_is_single_line_and_host_path_free() {
        let s = sanitize_error(&SyntheticError);
        assert!(!s.contains("C:\\Users"), "host path leaked: {s}");
        assert!(!s.contains('\n'), "multi-line error: {s}");
        assert_eq!(s, "open failed at <redacted> while loading config");
    }
}
