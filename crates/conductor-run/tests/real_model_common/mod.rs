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
/// comparison is byte equality over one normal form: the rule text itself, never a digest of it.
pub fn rule_section(src: &str) -> Option<String> {
    let normalized = src.replace("\r\n", "\n");
    let lines: Vec<&str> = normalized.split('\n').collect();
    let begin = lines.iter().position(|l| l.trim() == RULE_BEGIN)?;
    let end = begin + lines[begin..].iter().position(|l| l.trim() == RULE_END)?;
    Some(lines[begin..=end].join("\n"))
}

use serde_json::Value;

/// The canary line grammar the harvest's rule reads — byte-identical to its constants, which the
/// harvest's `the_capture_prints_every_token_the_rule_reads` holds this source to.
pub const CANARY: &str = "canary: ";
pub const SURFACED: &str = "surfaced";
pub const DISMISSED: &str = "dismissed";
pub const PIPELINE_FAULT: &str = "pipeline-fault";

pub const DIGEST_TICK: &str = "digest.runtime.cadence_tick";
pub const DIGEST_ASSEMBLE: &str = "digest.assemble.request";
const PROMPT: &str = "interpretation.prompt.assemble";
const PARSE: &str = "interpretation.json.parse";
const CREATED: &str = "interpretation.incident.created";
const ERROR: &str = "interpretation.inference.error";
const SKIPPED: &str = "interpretation.inference.skipped";
/// Pulse's no-incident outcome for a parsed generation (from Pulse `a2addb3`) — a different line from
/// `SKIPPED`, which is a pipeline skip before inference.
const INCIDENT_SKIPPED: &str = "interpretation.incident.skipped";

fn target(line: &Value) -> &str {
    line.get("target")
        .and_then(Value::as_str)
        .unwrap_or_default()
}

fn field(line: &Value, key: &str) -> String {
    match line.get("fields").and_then(|f| f.get(key)) {
        Some(Value::String(s)) => s.clone(),
        Some(other) => other.to_string(),
        None => String::new(),
    }
}

/// One `canary:` line per cue-bearing digest whose cue is a retry storm, in Pulse's log order, plus a
/// closing count of the cue-bearing digests of other kinds (counted, never classified).
///
/// Pulse's inference is serial and its lines carry no digest identity, so the pairing follows the
/// inference, never the cadence: the digest's inference is the FIRST prompt assembly after its tick
/// and before the next retry-storm tick, and its outcome is what lies between that prompt and the
/// next one. `surfaced` when an incident outcome follows its parse `ok` (Pulse logs one on creation
/// and on dedupe alike); `dismissed` when its parse `ok` has none; `pipeline-fault` when there is no
/// prompt, a skip before it, no parse `ok`, or an inference error. Pairing up to the next tick of ANY
/// kind was measured wrong at drives b1 and b2 (2026-09-29): a tier-2 tick a few seconds after the
/// canary's closed its segment before the parse, reading a surfacing and a dismissal as faults.
/// Stated limit: a digest Pulse's queue replaces before inference pairs with the replacing one.
///
/// Each line ends with its inference's `skip_reason` — Pulse's own stated cause of a no-incident
/// outcome — or `none` when the segment logs none. Recorded only: the rule reads the first token.
pub fn canary_attempts(lines: &[&Value]) -> Vec<String> {
    let cue_ticks: Vec<(usize, String)> = lines
        .iter()
        .enumerate()
        .filter(|(_, v)| target(v) == DIGEST_TICK && field(v, "cue_present") == "true")
        .map(|(at, _)| {
            let kind = lines[at + 1..]
                .iter()
                .find(|v| target(v) == DIGEST_ASSEMBLE)
                .map(|v| field(v, "cue_kind"))
                .unwrap_or_default();
            (at, kind)
        })
        .collect();
    let mut out = Vec::new();
    let mut other_kinds: Vec<String> = Vec::new();
    for (k, (at, kind)) in cue_ticks.iter().enumerate() {
        if kind != "retry_storm" {
            other_kinds.push(if kind.is_empty() {
                "unknown".to_string()
            } else {
                kind.clone()
            });
            continue;
        }
        let bound = cue_ticks[k + 1..]
            .iter()
            .find(|(_, kind)| kind == "retry_storm")
            .map_or(lines.len(), |(next, _)| *next);
        let before_bound = &lines[at + 1..bound];
        let prompt = before_bound.iter().position(|v| target(v) == PROMPT);
        let skipped_first = before_bound[..prompt.unwrap_or(before_bound.len())]
            .iter()
            .any(|v| target(v) == SKIPPED);
        let inference: &[&Value] = match prompt {
            Some(p) => {
                let after = &lines[at + 1 + p + 1..];
                let end = after
                    .iter()
                    .position(|v| target(v) == PROMPT)
                    .unwrap_or(after.len());
                &after[..end]
            }
            None => &[],
        };
        let first = |name: &str| inference.iter().find(|v| target(v) == name);
        let parse = first(PARSE).map(|v| field(v, "parse_outcome"));
        let faulted = skipped_first || first(ERROR).is_some() || first(SKIPPED).is_some();
        let outcome = first(CREATED);
        let skip_reason =
            first(INCIDENT_SKIPPED).map_or("none".to_string(), |v| field(v, "skip_reason"));
        let token = match (prompt, parse.as_deref(), faulted, outcome) {
            (Some(_), Some("ok"), false, Some(_)) => SURFACED,
            (Some(_), Some("ok"), false, None) => DISMISSED,
            _ => PIPELINE_FAULT,
        };
        let tier = lines[at + 1..]
            .iter()
            .find(|v| target(v) == DIGEST_ASSEMBLE)
            .map(|v| field(v, "cue_priority_tier"))
            .unwrap_or_default();
        out.push(format!(
            "{CANARY}{token} t={} cue_kind={kind} cue_priority_tier={tier} parse={} created={} deduped={} skip_reason={skip_reason}",
            lines[*at]
                .get("timestamp")
                .and_then(Value::as_str)
                .unwrap_or("unknown"),
            parse.as_deref().unwrap_or("none"),
            outcome.map_or("none".to_string(), |v| field(v, "created")),
            outcome.map_or("none".to_string(), |v| field(v, "deduped")),
        ));
    }
    out.push(format!(
        "canary other cue-bearing digests: {} ({})",
        other_kinds.len(),
        other_kinds.join(",")
    ));
    out
}

/// What the capture prints in place of a fingerprint.
const FINGERPRINT_PLACEHOLDER: &str = "<fingerprint>";

/// The key the capture renders Pulse's storm fingerprint under.
const FINGERPRINT_KEY: &str = "fingerprint_hex=";

/// Replace every fingerprint with `<fingerprint>`, by two rules. Keyed: the value after each
/// `fingerprint_hex=` — the ASCII alphanumeric run directly after the `=` — whatever its characters,
/// because a keyed value is a fingerprint even when its prefix happens to be all digits. Unkeyed: every
/// fingerprint-shaped token, a run of eight or more lowercase hex digits holding at least one letter,
/// with no ASCII alphanumeric on either side; an unkeyed all-digit run is a nanosecond stamp or a seed
/// and is never one, and a `det-` prefix before a ref survives, so the canned-evidence witness still
/// reads. The committed captures carry no fingerprint (security-plan §Input Validation, the real-model
/// capture ingest row). Idempotent: an existing placeholder and an empty keyed value are left as they
/// are.
pub fn elide_fingerprints(text: &str) -> String {
    let text = &elide_keyed_fingerprints(text);
    let bytes = text.as_bytes();
    let is_hex = |b: u8| b.is_ascii_digit() || (b'a'..=b'f').contains(&b);
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let mut copied = 0;
    while i < bytes.len() {
        let bounded_before = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
        if bounded_before && is_hex(bytes[i]) {
            let end = i + bytes[i..].iter().take_while(|b| is_hex(**b)).count();
            let bounded_after = end == bytes.len() || !bytes[end].is_ascii_alphanumeric();
            let run = &bytes[i..end];
            if bounded_after && run.len() >= 8 && run.iter().any(|b| b.is_ascii_lowercase()) {
                out.push_str(&text[copied..i]);
                out.push_str(FINGERPRINT_PLACEHOLDER);
                copied = end;
            }
            i = end;
        } else {
            i += 1;
        }
    }
    out.push_str(&text[copied..]);
    out
}

/// The keyed rule of [`elide_fingerprints`].
fn elide_keyed_fingerprints(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(FINGERPRINT_KEY) {
        let value_at = at + FINGERPRINT_KEY.len();
        out.push_str(&rest[..value_at]);
        let after = &rest[value_at..];
        let run = after.bytes().take_while(u8::is_ascii_alphanumeric).count();
        if run > 0 {
            out.push_str(FINGERPRINT_PLACEHOLDER);
        }
        rest = &after[run..];
    }
    out.push_str(rest);
    out
}

/// What the capture prints in place of the live data dir's workspace key.
pub const WORKSPACE_KEY_PLACEHOLDER: &str = "<workspace-key>";

/// The line Pulse's `## Project Context` renders the workspace key on.
const WORKSPACE_LINE: &str = "workspace=";

/// Mask the live data dir's workspace key — its basename — in a block the capture prints, replacing it
/// with [`WORKSPACE_KEY_PLACEHOLDER`].
///
/// First, every line whose trimmed start is `workspace=` keeps that prefix and gets its whole value
/// replaced, whatever it holds: the raw key, a scrubber rendering such as `[redacted: credit_card]`
/// (which carries no key text to match), or anything else. Then, when `key` is non-empty, every
/// occurrence of it whose neighbours are outside `[A-Za-z0-9_.-]` (or the text's edge) is replaced,
/// which covers each `## Previously Seen` entry's `({workspace})` suffix. Idempotent.
pub fn mask_workspace_key(text: &str, key: Option<&str>) -> String {
    let mut masked = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        let content = line.trim_end_matches(['\r', '\n']);
        let indent = content.len() - content.trim_start().len();
        if content[indent..].starts_with(WORKSPACE_LINE) {
            masked.push_str(&content[..indent + WORKSPACE_LINE.len()]);
            masked.push_str(WORKSPACE_KEY_PLACEHOLDER);
        } else {
            masked.push_str(content);
        }
        masked.push_str(&line[content.len()..]);
    }
    let Some(key) = key.filter(|k| !k.is_empty()) else {
        return masked;
    };
    let is_key_char = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-');
    let mut out = String::with_capacity(masked.len());
    let (mut copied, mut from) = (0, 0);
    while let Some(found) = masked[from..].find(key) {
        let at = from + found;
        let end = at + key.len();
        let bounded = masked[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !is_key_char(c))
            && masked[end..].chars().next().is_none_or(|c| !is_key_char(c));
        if bounded {
            out.push_str(&masked[copied..at]);
            out.push_str(WORKSPACE_KEY_PLACEHOLDER);
            copied = end;
            from = end;
        } else {
            from = at + masked[at..].chars().next().map_or(1, char::len_utf8);
        }
    }
    out.push_str(&masked[copied..]);
    out
}

/// How a rendered report shows the workspace key, for the capture's fields-only witness — the value
/// itself is never returned: `absent` (no `workspace=` line), `unknown-key` (the capture could not
/// derive the key), `verbatim` (the value is the key, or a path whose last component is) or `scrubbed`
/// (anything else). Pulse stamps a PATH — its detected workspace root, else the data dir
/// (andromeda-pulse `crates/workspace-detector/src/contract.rs` `workspace_key`) — so the leaf is what
/// the capture's key can match.
pub fn workspace_rendering(report: &str, key: Option<&str>) -> &'static str {
    let Some(value) = report
        .lines()
        .find_map(|l| l.trim_start().strip_prefix(WORKSPACE_LINE))
    else {
        return "absent";
    };
    match key.filter(|k| !k.is_empty()) {
        None => "unknown-key",
        Some(key) if value.trim().rsplit(['/', '\\']).next() == Some(key) => "verbatim",
        Some(_) => "scrubbed",
    }
}

/// The most ids one attribution sweep probes.
pub const SWEEP_BOUND: i64 = 64;

/// The ids one attribution sweep probes: [`SWEEP_BOUND`] of them, starting `SWEEP_BOUND - 1` below the
/// highest incident id seen so far — active or swept — floored at 1, or from 1 when none is seen.
///
/// Anchored on the HIGHEST id seen, never the lowest ACTIVE one: an incident that auto-resolved before
/// a poll is off the active list but still readable by id, and an anchor at the lowest active id skips
/// every such incident below it (measured at drive a3, 2026-09-29: only incident 6 was active at the
/// first poll, so the scenario's incident below it was never read). Stated limit: past
/// [`SWEEP_BOUND`] incidents, an id above the highest seen is reached only once one at or above it
/// has been seen.
pub fn sweep_window(highest_seen: Option<i64>) -> std::ops::Range<i64> {
    let start = highest_seen.map_or(1, |h| h.saturating_sub(SWEEP_BOUND - 1).max(1));
    start..start.saturating_add(SWEEP_BOUND)
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
