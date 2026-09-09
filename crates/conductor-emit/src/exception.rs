//! Exception span events + line-insensitive fingerprint control (P-006, P-017, P-018).
//!
//! Builds an OTLP trace carrying one error span with an OTel `exception` span event
//! (`exception.type` / `exception.message` / `exception.stacktrace`), and recomputes the fingerprint
//! Pulse derives from that exception's content — Pulse's own derivation, over the same preimage the
//! wire carries, so the expectation predicts how the SUT actually groups exceptions.
//!
//! Identity is LINE-insensitive and TYPE/FRAME/PATH-sensitive. Pulse normalizes TOKEN-LEADING: a path
//! token counts as absolute only where a token starts, so a RELATIVE path is preserved in full and is
//! significant at every depth (`src/a.rs` and `src/b/c.rs` are distinct faults), while a token-leading
//! absolute path is stripped to nothing. One bound survives beside it: only the first
//! [`NORMALIZED_FRAMES`] frames reach the preimage.
//!
//! Span identity is seeded ([`crate::span_tree`]'s `ChaCha8Rng` discipline); the fingerprint is a pure
//! function of content (not the seed) and stable across platforms/versions.

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::trace::v1::span::Event;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans};
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;

use crate::message::{error_status, service_resource, span_with_events, string_kv, unix_nanos};
use crate::span_tree::gen_id;

/// Upper bound on the stack-frame count a single exception contributes to the rendered stacktrace
/// (bounded output — security-plan §Input Validation). The FINGERPRINT input is bounded far tighter,
/// by [`NORMALIZED_FRAMES`].
pub const MAX_FRAMES: usize = 64;

/// Frames that reach the fingerprint preimage after normalization — Pulse hashes only this many
/// leading stack lines, so frames past it are invisible to exception identity.
pub const NORMALIZED_FRAMES: usize = 3;

/// Width of the fingerprint Pulse stores and Conductor expects: blake3 truncated to 16 bytes,
/// rendered as 32 lowercase hex chars.
pub const FINGERPRINT_BYTES: usize = 16;

/// One synthetic stack frame. `function` and `file` are fingerprint-significant; `line` is not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    /// Fully-qualified function/method signature (fingerprint-significant).
    pub function: String,
    /// Source file / module path, fingerprint-significant at EVERY depth — normalization preserves a
    /// relative path in full and strips only a TOKEN-LEADING absolute one, to nothing. Conductor's
    /// own frames stay relative (`stacktrace_carries_no_absolute_host_path`).
    pub file: String,
    /// Source line number (fingerprint-insensitive — normalization drops the `:line(:col)` suffix).
    pub line: u32,
}

impl Frame {
    /// Construct a frame from its parts.
    pub fn new(function: impl Into<String>, file: impl Into<String>, line: u32) -> Self {
        Self {
            function: function.into(),
            file: file.into(),
            line,
        }
    }
}

/// A synthetic exception identity: type, message, and an ordered stack (outermost frame first).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionSpec {
    /// The `exception.type` (fingerprint-significant).
    pub exception_type: String,
    /// The `exception.message` (carried on the event; fingerprint-insensitive).
    pub message: String,
    /// The ordered stack frames (at most [`MAX_FRAMES`] contribute to output).
    pub frames: Vec<Frame>,
}

impl ExceptionSpec {
    /// Construct an exception spec.
    pub fn new(
        exception_type: impl Into<String>,
        message: impl Into<String>,
        frames: Vec<Frame>,
    ) -> Self {
        Self {
            exception_type: exception_type.into(),
            message: message.into(),
            frames,
        }
    }
}

/// A controlled transformation of a base [`ExceptionSpec`], driving Pulse's fingerprinting into a
/// known relationship with the base (amended P-017 clause (c)).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FingerprintVariant {
    /// Identical to the base ⇒ SAME fingerprint.
    Identical,
    /// Same frames + lines, a different source path per frame BELOW its leading segment
    /// ⇒ DIFFERENT fingerprint — relative path structure is significant at every depth.
    PathVariant,
    /// Same frames + lines, a different LEADING path segment per frame ⇒ DIFFERENT fingerprint.
    /// Distinct from [`FingerprintVariant::PathVariant`] only in WHERE the path differs; together
    /// the two witness that identity turns on the whole relative path, not one segment of it.
    RelativePathVariant,
    /// Same frames + paths, different LINE per frame ⇒ SAME fingerprint (line-insensitive).
    LineVariant,
    /// Different exception TYPE ⇒ DIFFERENT fingerprint.
    TypeVariant,
    /// Different stack-FRAME function ⇒ DIFFERENT fingerprint.
    FrameVariant,
}

impl FingerprintVariant {
    /// Derive a variant exception from `base` by applying this transformation. The same-fingerprint
    /// variants perturb only a fingerprint-insensitive field (the line); the different-fingerprint
    /// variants perturb a significant one (any part of the path / the type / a frame function).
    pub fn derive(self, base: &ExceptionSpec) -> ExceptionSpec {
        let mut spec = base.clone();
        match self {
            FingerprintVariant::Identical => {}
            // Vary the path BELOW its leading segment. Both path variants stay relative, so
            // `stacktrace_carries_no_absolute_host_path` still holds.
            FingerprintVariant::PathVariant => {
                for (i, frame) in spec.frames.iter_mut().enumerate() {
                    let head = frame
                        .file
                        .split('/')
                        .next()
                        .unwrap_or(&frame.file)
                        .to_owned();
                    frame.file = format!("{head}/variant_{i}/module.rs");
                }
            }
            FingerprintVariant::RelativePathVariant => {
                for (i, frame) in spec.frames.iter_mut().enumerate() {
                    frame.file = format!("alt_{i}/module.rs");
                }
            }
            FingerprintVariant::LineVariant => {
                for frame in &mut spec.frames {
                    frame.line = frame.line.wrapping_add(100);
                }
            }
            FingerprintVariant::TypeVariant => spec.exception_type.push_str("Variant"),
            FingerprintVariant::FrameVariant => {
                if let Some(frame) = spec.frames.first_mut() {
                    frame.function.push_str("_variant");
                }
            }
        }
        spec
    }
}

/// The fingerprint Pulse derives from `spec` — Pulse's own derivation, recomputed here over the same
/// preimage the wire carries, so Conductor's expectation predicts how Pulse actually groups
/// exceptions. blake3 over `exception_type` + a NUL separator + the normalized stacktrace, truncated
/// to [`FINGERPRINT_BYTES`] and rendered lowercase hex. A pure function of content (not the RNG seed).
///
/// One identity narrowing follows from the normalization and is deliberate, not incidental: only the
/// first [`NORMALIZED_FRAMES`] frames contribute. See [`normalize_stacktrace`].
///
/// Source of truth: `andromeda-pulse crates/buffer/src/fingerprint.rs`
/// (`compute_exception_fingerprint`), transcribed at HEAD `efabe8e` — Pulse is the SUT, so its
/// derivation is the fact and this is the expectation of it.
pub fn fingerprint(spec: &ExceptionSpec) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(spec.exception_type.as_bytes());
    hasher.update(b"\0");
    hasher.update(normalize_stacktrace(&render_stacktrace(spec)).as_bytes());
    let hash = hasher.finalize();
    let mut out = String::with_capacity(FINGERPRINT_BYTES * 2);
    for byte in &hash.as_bytes()[..FINGERPRINT_BYTES] {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// Build an OTLP trace export of a single root error span carrying an OTel `exception` span event
/// for `spec`. Span identity (`trace_id` / `span_id`) is a deterministic function of `seed`; the
/// span's `Status.Code` is ERROR with the exception message.
pub fn exception_trace_request(
    service_name: &str,
    seed: u64,
    spec: &ExceptionSpec,
) -> ExportTraceServiceRequest {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let trace_id = gen_id::<16>(&mut rng).to_vec();
    let span_id = gen_id::<8>(&mut rng).to_vec();
    let root = span_with_events(
        "root",
        trace_id,
        span_id,
        Vec::new(),
        error_status(&spec.message),
        vec![exception_event(spec)],
    );
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(service_resource(service_name)),
            scope_spans: vec![ScopeSpans {
                spans: vec![root],
                ..Default::default()
            }],
            ..Default::default()
        }],
    }
}

fn exception_event(spec: &ExceptionSpec) -> Event {
    Event {
        time_unix_nano: unix_nanos(),
        name: "exception".to_string(),
        attributes: vec![
            string_kv("exception.type", &spec.exception_type),
            string_kv("exception.message", &spec.message),
            string_kv("exception.stacktrace", &render_stacktrace(spec)),
        ],
        ..Default::default()
    }
}

fn render_stacktrace(spec: &ExceptionSpec) -> String {
    let mut out = String::new();
    for frame in spec.frames.iter().take(MAX_FRAMES) {
        out.push_str(&format!(
            "at {} ({}:{})\n",
            frame.function, frame.file, frame.line
        ));
    }
    out
}

/// Normalize a rendered stacktrace for fingerprint stability across hosts: keep the first
/// [`NORMALIZED_FRAMES`] non-empty lines, normalize each, join with `\n`.
///
/// The frame bound is what makes the same proximate failure surface hash alike however deep the rest
/// of the stack goes — and it is why frame-sensitivity reaches only that far.
fn normalize_stacktrace(raw: &str) -> String {
    let mut frames: Vec<String> = Vec::with_capacity(NORMALIZED_FRAMES);
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        frames.push(normalize_frame(trimmed));
        if frames.len() >= NORMALIZED_FRAMES {
            break;
        }
    }
    frames.join("\n")
}

/// Strip the host-varying parts of one frame line: TOKEN-LEADING absolute paths (Unix `/…` or
/// Windows `C:\…`), hex memory addresses (`0x…`), and `:line(:col)` suffixes.
///
/// Only an absolute path is stripped, and only where a token starts (see [`is_token_boundary`]); a
/// relative path such as `src/worker.rs` is identity-significant and survives in full. Transcribed
/// from Pulse's `normalize_frame`, which is byte-identical.
fn normalize_frame(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if is_hex_address_start(bytes, i) {
            i = skip_hex_address(bytes, i);
            continue;
        }
        if is_absolute_path_start(bytes, i) {
            i = skip_absolute_path(bytes, i);
            continue;
        }
        if bytes[i] == b':' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
            i = skip_line_number_suffix(bytes, i);
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    while out.ends_with(' ') {
        out.pop();
    }
    out
}

fn is_hex_address_start(bytes: &[u8], i: usize) -> bool {
    i + 2 < bytes.len()
        && bytes[i] == b'0'
        && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X')
        && bytes[i + 2].is_ascii_hexdigit()
}

fn skip_hex_address(bytes: &[u8], mut i: usize) -> usize {
    i += 2;
    while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
        i += 1;
    }
    i
}

/// A path token counts as ABSOLUTE only when it STARTS a token — position 0, or preceded by a
/// non-path byte. Without this precondition the Unix arm fires on every `/`, and since
/// [`is_path_char`] admits `/`, [`skip_absolute_path`] then swallows the remainder of the token so
/// only the leading segment survives, making `src/a.rs` and `src/b/c.rs` one identity.
fn is_token_boundary(bytes: &[u8], i: usize) -> bool {
    i == 0 || !is_path_char(bytes[i - 1])
}

fn is_absolute_path_start(bytes: &[u8], i: usize) -> bool {
    if !is_token_boundary(bytes, i) {
        return false;
    }
    if bytes[i] == b'/' && i + 1 < bytes.len() && is_path_char(bytes[i + 1]) {
        return true;
    }
    i + 2 < bytes.len()
        && bytes[i].is_ascii_alphabetic()
        && bytes[i + 1] == b':'
        && (bytes[i + 2] == b'\\' || bytes[i + 2] == b'/')
}

fn skip_absolute_path(bytes: &[u8], mut i: usize) -> usize {
    if i + 2 < bytes.len()
        && bytes[i].is_ascii_alphabetic()
        && bytes[i + 1] == b':'
        && (bytes[i + 2] == b'\\' || bytes[i + 2] == b'/')
    {
        i += 3;
    } else {
        i += 1;
    }
    while i < bytes.len() && is_path_char(bytes[i]) {
        i += 1;
    }
    i
}

fn is_path_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'/' | b'\\' | b'_' | b'-' | b'.' | b'~')
}

fn skip_line_number_suffix(bytes: &[u8], mut i: usize) -> usize {
    i += 1;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b':' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_proto::tonic::common::v1::any_value;
    use opentelemetry_proto::tonic::trace::v1::status::StatusCode;

    fn base() -> ExceptionSpec {
        ExceptionSpec::new(
            "ValueError",
            "bad input",
            vec![
                Frame::new("conductor::worker::handle", "src/worker.rs", 42),
                Frame::new("conductor::worker::parse", "src/worker.rs", 17),
            ],
        )
    }

    fn string_attr(req: &ExportTraceServiceRequest, key: &str) -> String {
        let event = &req.resource_spans[0].scope_spans[0].spans[0].events[0];
        event
            .attributes
            .iter()
            .find(|kv| kv.key == key)
            .and_then(|kv| kv.value.as_ref())
            .and_then(|v| v.value.as_ref())
            .map(|v| match v {
                any_value::Value::StringValue(s) => s.clone(),
                _ => String::new(),
            })
            .unwrap_or_default()
    }

    /// The seeded shape (identity + status + exception content), excluding wall-clock timestamps.
    fn shape(req: &ExportTraceServiceRequest) -> (Vec<u8>, Vec<u8>, i32, String, String, String) {
        let span = &req.resource_spans[0].scope_spans[0].spans[0];
        (
            span.trace_id.clone(),
            span.span_id.clone(),
            span.status.as_ref().unwrap().code,
            string_attr(req, "exception.type"),
            string_attr(req, "exception.message"),
            string_attr(req, "exception.stacktrace"),
        )
    }

    #[test]
    fn same_fingerprint_variants_match_the_base() {
        let b = base();
        let fp = fingerprint(&b);
        for variant in [
            FingerprintVariant::Identical,
            FingerprintVariant::LineVariant,
        ] {
            assert_eq!(
                fingerprint(&variant.derive(&b)),
                fp,
                "{variant:?} must keep the fingerprint"
            );
        }
    }

    /// Under token-leading normalization a relative path is preserved in full, so BOTH path variants
    /// change identity — they differ only in where the path differs, and together they witness that
    /// the whole relative path is significant, not one segment of it.
    #[test]
    fn both_path_variants_are_fingerprint_significant() {
        let b = base();
        let fp = fingerprint(&b);
        for variant in [
            FingerprintVariant::PathVariant,
            FingerprintVariant::RelativePathVariant,
        ] {
            assert_ne!(
                fingerprint(&variant.derive(&b)),
                fp,
                "{variant:?} must change the fingerprint"
            );
        }
    }

    /// Both path variants stay RELATIVE, so `stacktrace_carries_no_absolute_host_path` keeps holding
    /// for every scenario that drives them.
    #[test]
    fn path_variants_emit_a_changed_relative_path() {
        let b = base();
        for variant in [
            FingerprintVariant::PathVariant,
            FingerprintVariant::RelativePathVariant,
        ] {
            let derived = variant.derive(&b);
            for (before, after) in b.frames.iter().zip(&derived.frames) {
                assert_ne!(
                    before.file, after.file,
                    "the emitted path must actually change"
                );
                assert!(
                    !after.file.starts_with('/'),
                    "must stay relative: {}",
                    after.file
                );
            }
            let st = string_attr(
                &exception_trace_request("svc", 7, &derived),
                "exception.stacktrace",
            );
            assert!(
                !st.contains("C:\\") && !st.contains("/Users/") && !st.contains("/home/"),
                "{st}"
            );
        }
    }

    /// The normalization fact the redraw turns on, pinned directly against Pulse's own axis tests: a
    /// relative path is significant at EVERY depth, and a token-leading absolute path is stripped to
    /// nothing (so it cannot collide with any surviving relative path).
    #[test]
    fn relative_paths_are_significant_at_every_depth() {
        let at = |file: &str| {
            fingerprint(&ExceptionSpec::new(
                "E",
                "m",
                vec![Frame::new("f", file, 1)],
            ))
        };
        assert_ne!(
            at("src/a.rs"),
            at("src/b/c.rs"),
            "differing below the leading segment"
        );
        assert_ne!(
            at("src/a.rs"),
            at("other/a.rs"),
            "differing at the leading segment"
        );
        assert_ne!(
            at("src/a.rs"),
            at("/src/a.rs"),
            "a token-leading absolute is stripped away"
        );
    }

    /// A token-leading absolute path normalizes to nothing, so two DIFFERENT absolute paths collapse
    /// to one identity — the axis Conductor cannot use, because its own frames must stay relative.
    #[test]
    fn token_leading_absolute_paths_normalize_to_the_same_empty_form() {
        assert_eq!(
            normalize_stacktrace("at handler(/usr/lib/thing.rs:10)"),
            "at handler()"
        );
        assert_eq!(
            normalize_stacktrace("at handler(C:\\proj\\thing.rs:10)"),
            "at handler()"
        );
        assert_eq!(
            normalize_stacktrace("at handler(src/b/c.rs:10)"),
            "at handler(src/b/c.rs)"
        );
    }

    #[test]
    fn different_fingerprint_variants_diverge_from_the_base() {
        let b = base();
        let fp = fingerprint(&b);
        for variant in [
            FingerprintVariant::TypeVariant,
            FingerprintVariant::FrameVariant,
        ] {
            assert_ne!(
                fingerprint(&variant.derive(&b)),
                fp,
                "{variant:?} must change the fingerprint"
            );
        }
    }

    /// The second narrowing: only the first `NORMALIZED_FRAMES` frames reach the preimage, so a frame
    /// past that bound is invisible to identity however much it changes.
    #[test]
    fn frames_past_the_normalized_bound_do_not_change_the_fingerprint() {
        let mut deep = base();
        while deep.frames.len() < NORMALIZED_FRAMES {
            deep.frames
                .push(Frame::new("conductor::worker::pad", "src/worker.rs", 1));
        }
        let fp = fingerprint(&deep);

        let mut beyond = deep.clone();
        beyond.frames.push(Frame::new(
            "conductor::worker::invisible",
            "src/other.rs",
            99,
        ));
        assert_eq!(fingerprint(&beyond), fp);
    }

    #[test]
    fn fingerprint_is_pulse_width_lowercase_hex() {
        let fp = fingerprint(&base());
        assert_eq!(fp.len(), FINGERPRINT_BYTES * 2);
        assert!(
            fp.chars()
                .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
        );
    }

    #[test]
    fn fingerprint_is_a_pure_function_of_content() {
        let b = base();
        assert_eq!(fingerprint(&b), fingerprint(&b.clone()));
    }

    #[test]
    fn builds_one_error_span_with_a_well_formed_exception_event() {
        let req = exception_trace_request("svc", 7, &base());
        let spans = &req.resource_spans[0].scope_spans[0].spans;
        assert_eq!(spans.len(), 1);
        let span = &spans[0];
        assert!(span.parent_span_id.is_empty());
        assert_eq!(span.trace_id.len(), 16);
        assert_eq!(span.span_id.len(), 8);
        assert_eq!(span.status.as_ref().unwrap().code, StatusCode::Error as i32);

        assert_eq!(span.events.len(), 1);
        let event = &span.events[0];
        assert_eq!(event.name, "exception");
        let keys: Vec<&str> = event.attributes.iter().map(|kv| kv.key.as_str()).collect();
        assert!(keys.contains(&"exception.type"));
        assert!(keys.contains(&"exception.message"));
        assert!(keys.contains(&"exception.stacktrace"));
    }

    #[test]
    fn same_seed_reproduces_identical_shape() {
        let a = exception_trace_request("svc", 99, &base());
        let b = exception_trace_request("svc", 99, &base());
        assert_eq!(shape(&a), shape(&b));
    }

    #[test]
    fn different_seeds_diverge() {
        let a = exception_trace_request("svc", 1, &base());
        let b = exception_trace_request("svc", 2, &base());
        assert_ne!(shape(&a).0, shape(&b).0);
        assert_ne!(shape(&a).1, shape(&b).1);
    }

    #[test]
    fn stacktrace_carries_no_absolute_host_path() {
        let req = exception_trace_request("svc", 7, &base());
        let st = string_attr(&req, "exception.stacktrace");
        assert!(!st.contains("C:\\"), "{st}");
        assert!(!st.contains("/Users/"), "{st}");
        assert!(!st.contains("/home/"), "{st}");
        assert!(st.contains("conductor::worker::handle"));
    }
}
