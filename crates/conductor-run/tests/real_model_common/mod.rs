//! Shared by the real-model capture (`real_model_live.rs`) and its harvest (`real_model_harvest.rs`),
//! and used by both: the grading rule's span extractor and the host-path mask. A `tests/`
//! subdirectory module, so it is never a test target of its own.

/// The grading rule's two marker LINES. Matched as whole lines, so prose that quotes a marker inside
/// a longer line can never open or close a span.
const RULE_BEGIN: &str = "// ---- rule: begin ----";
const RULE_END: &str = "// ---- rule: end ----";

/// The span between the rule's two marker lines, markers included, with line endings normalized to
/// LF. `None` when either marker is absent or they are out of order.
///
/// The capture prints this span before the leg and the harvest compares it with its own, so the
/// comparison is byte equality over one normal form — never a hash, which would need a dependency.
pub fn rule_section(src: &str) -> Option<String> {
    let normalized = src.replace("\r\n", "\n");
    let lines: Vec<&str> = normalized.split('\n').collect();
    let begin = lines.iter().position(|l| l.trim() == RULE_BEGIN)?;
    let end = begin + lines[begin..].iter().position(|l| l.trim() == RULE_END)?;
    Some(lines[begin..=end].join("\n"))
}

/// Mask every host path `conductor_core::redact_value` misses, replacing each with `<host-path>`.
///
/// `redact_value` tests whitespace-separated TOKENS, so a path that does not open its token survives
/// it: a backticked ref, `(D:\…`, `path=D:\…`, the `\\?\D:\…` long-path form, and an MSYS root such
/// as `/d/…`. Applied after it, this catches a drive-letter path (`X:\` or `X:/`, word-anchored so a
/// `scheme://` URL is never matched, the `\\?\` prefix included), an MSYS root (`/x/` whose left
/// neighbour is not `[A-Za-z0-9_./-]`), and `/home/`, `/Users/` or `%APPDATA%`, each up to the next
/// whitespace, backtick, quote or closing bracket. A repo-relative path carries none of these.
pub fn mask_host_paths(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        if host_path_starts_at(&chars, i) {
            while i < chars.len() && !ends_host_path(chars[i]) {
                i += 1;
            }
            out.push_str("<host-path>");
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

fn host_path_starts_at(chars: &[char], i: usize) -> bool {
    let at = |k: usize| chars.get(k).copied();
    let is_word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let long_path_prefix = chars[i..].starts_with(&['\\', '\\', '?', '\\']);
    let drive_at = |k: usize| {
        at(k).is_some_and(|c| c.is_ascii_alphabetic())
            && at(k + 1) == Some(':')
            && matches!(at(k + 2), Some('\\' | '/'))
    };
    let word_boundary = i == 0 || !is_word(chars[i - 1]);
    let msys_neighbour_ok =
        i == 0 || !(is_word(chars[i - 1]) || matches!(chars[i - 1], '.' | '/' | '-'));
    let msys_root = chars[i] == '/'
        && at(i + 1).is_some_and(|c| c.is_ascii_alphabetic())
        && at(i + 2) == Some('/')
        && msys_neighbour_ok;
    let named = ["/home/", "/Users/", "%APPDATA%"]
        .iter()
        .any(|token| token.chars().enumerate().all(|(k, t)| at(i + k) == Some(t)));
    (long_path_prefix && drive_at(i + 4)) || (word_boundary && drive_at(i)) || msys_root || named
}

fn ends_host_path(c: char) -> bool {
    c.is_whitespace() || matches!(c, '`' | '"' | '\'' | ')' | ']' | '}' | '>')
}
