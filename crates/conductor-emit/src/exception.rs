//! Exception span events + path/line-insensitive fingerprint control (P-006, P-017, P-018).
//!
//! Builds an OTLP trace carrying one error span with an OTel `exception` span event
//! (`exception.type` / `exception.message` / `exception.stacktrace`), and computes the fingerprint
//! Conductor expects Pulse to derive from that exception's content. Per the amended P-017 clause (c)
//! the fingerprint is insensitive to source PATH and LINE but sensitive to exception TYPE and stack
//! FRAME functions: identical / path-variant / line-variant ⇒ the same fingerprint; type-variant /
//! frame-variant ⇒ a different one. Span identity is seeded ([`crate::span_tree`]'s `ChaCha8Rng`
//! discipline); the fingerprint is a pure function of content (not the seed) and stable across
//! platforms/versions — so NOT `std::hash::DefaultHasher` (architecture §Established Decisions —
//! Determinism RNG).

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::trace::v1::span::Event;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans};
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;

use crate::message::{error_status, service_resource, span_with_events, string_kv, unix_nanos};
use crate::span_tree::gen_id;

/// Upper bound on the stack-frame count a single exception contributes to the rendered stacktrace
/// and the fingerprint input (bounded output — security-plan §Input Validation).
pub const MAX_FRAMES: usize = 64;

/// One synthetic stack frame. `function` is fingerprint-significant; `file` and `line` are
/// deliberately fingerprint-INSENSITIVE (carried for the rendered stacktrace only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    /// Fully-qualified function/method signature (fingerprint-significant).
    pub function: String,
    /// Source file / module path — relative, never an absolute host path (fingerprint-insensitive).
    pub file: String,
    /// Source line number (fingerprint-insensitive — amended P-017 clause (c)).
    pub line: u32,
}

impl Frame {
    /// Construct a frame from its parts.
    pub fn new(function: impl Into<String>, file: impl Into<String>, line: u32) -> Self {
        Self { function: function.into(), file: file.into(), line }
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
        Self { exception_type: exception_type.into(), message: message.into(), frames }
    }
}

/// A controlled transformation of a base [`ExceptionSpec`], driving Pulse's fingerprinting into a
/// known relationship with the base (amended P-017 clause (c)).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FingerprintVariant {
    /// Identical to the base ⇒ SAME fingerprint.
    Identical,
    /// Same frames + lines, different source PATH per frame ⇒ SAME fingerprint (path-insensitive).
    PathVariant,
    /// Same frames + paths, different LINE per frame ⇒ SAME fingerprint (line-insensitive).
    LineVariant,
    /// Different exception TYPE ⇒ DIFFERENT fingerprint.
    TypeVariant,
    /// Different stack-FRAME function ⇒ DIFFERENT fingerprint.
    FrameVariant,
}

impl FingerprintVariant {
    /// Derive a variant exception from `base` by applying this transformation. The same-fingerprint
    /// variants perturb only fingerprint-insensitive fields (path / line); the different-fingerprint
    /// variants perturb a significant field (type / frame function).
    pub fn derive(self, base: &ExceptionSpec) -> ExceptionSpec {
        let mut spec = base.clone();
        match self {
            FingerprintVariant::Identical => {}
            FingerprintVariant::PathVariant => {
                for (i, frame) in spec.frames.iter_mut().enumerate() {
                    frame.file = format!("alt/module_{i}.rs");
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

/// The fingerprint Conductor expects Pulse to derive from `spec` — a path+line-INSENSITIVE,
/// type+frame-SENSITIVE identity over the exception content. A pure function of content (not the RNG
/// seed), stable across platforms/versions (FNV-1a, never `DefaultHasher`).
pub fn fingerprint(spec: &ExceptionSpec) -> String {
    let mut h = Fnv1a::new();
    h.update(spec.exception_type.as_bytes());
    for frame in spec.frames.iter().take(MAX_FRAMES) {
        h.update(b"\n");
        h.update(frame.function.as_bytes());
    }
    format!("{:016x}", h.finish())
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
            scope_spans: vec![ScopeSpans { spans: vec![root], ..Default::default() }],
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
        out.push_str(&format!("at {} ({}:{})\n", frame.function, frame.file, frame.line));
    }
    out
}

/// FNV-1a (64-bit) — a fixed, platform/version-stable hash (unlike the SipHash-based
/// `DefaultHasher`), so identical exception content yields the same fingerprint on any host.
struct Fnv1a(u64);

impl Fnv1a {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    fn new() -> Self {
        Self(Self::OFFSET)
    }

    fn update(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
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
    fn same_fingerprint_triple_matches_the_base() {
        let b = base();
        let fp = fingerprint(&b);
        for variant in [
            FingerprintVariant::Identical,
            FingerprintVariant::PathVariant,
            FingerprintVariant::LineVariant,
        ] {
            assert_eq!(
                fingerprint(&variant.derive(&b)),
                fp,
                "{variant:?} must keep the fingerprint (amended P-017 clause (c))"
            );
        }
    }

    #[test]
    fn different_fingerprint_variants_diverge_from_the_base() {
        let b = base();
        let fp = fingerprint(&b);
        for variant in [FingerprintVariant::TypeVariant, FingerprintVariant::FrameVariant] {
            assert_ne!(
                fingerprint(&variant.derive(&b)),
                fp,
                "{variant:?} must change the fingerprint"
            );
        }
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
