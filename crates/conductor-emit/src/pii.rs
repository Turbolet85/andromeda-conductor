//! PII payload corpus: a seeded generator of synthetic, structurally-valid values across the seven
//! P-047 categories (email, JWT, bearer token, API key, credit-card PAN, SSN, secret `key=value`),
//! plus builders that embed the corpus across all three OTLP signal types — span attributes and an
//! `exception` span event ([`pii_trace_request`]) and log-record body + attributes
//! ([`pii_logs_request`]). The downstream `pii-scrub` scenario reads these back via MCP and asserts
//! Pulse scrubbed each category while preserving surrounding structure (P-035 / P-047 / P-048);
//! scrub verification is NOT this module's concern. Values are synthetic fixtures by construction —
//! never host- or env-derived — and a deterministic function of the seed (architecture §Established
//! Decisions — Determinism RNG): the same seed reproduces the same corpus.

use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::common::v1::{any_value, AnyValue};
use opentelemetry_proto::tonic::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use opentelemetry_proto::tonic::trace::v1::span::Event;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans};
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

use crate::message::{
    error_status, ok_status, service_resource, span_with_attributes, span_with_events, string_kv,
    unix_nanos,
};
use crate::span_tree::gen_id;

const ALNUM: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const B64URL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
/// Standard base64url of `{"alg":"HS256","typ":"JWT"}` — a real JWT header segment.
const JWT_HEADER: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

/// The seven P-047 PII categories Pulse's scrubber must detect and redact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PiiCategory {
    /// An email address (`local@example.com`).
    Email,
    /// A JSON Web Token (three base64url segments).
    Jwt,
    /// An HTTP `Bearer` authorization token.
    Bearer,
    /// A provider-style secret API key.
    ApiKey,
    /// A Luhn-valid credit-card PAN.
    CreditCard,
    /// A US Social Security Number (`NNN-NN-NNNN`).
    Ssn,
    /// A secret-like `key=value` pair.
    SecretKeyValue,
}

impl PiiCategory {
    /// All seven categories in discriminant order (the [`PiiCorpus`] storage order).
    pub fn all() -> [PiiCategory; 7] {
        [
            PiiCategory::Email,
            PiiCategory::Jwt,
            PiiCategory::Bearer,
            PiiCategory::ApiKey,
            PiiCategory::CreditCard,
            PiiCategory::Ssn,
            PiiCategory::SecretKeyValue,
        ]
    }

    /// The attribute/field key this category's value is carried under — a label only; Pulse detects
    /// PII by value shape, not by key name.
    pub fn field_key(self) -> &'static str {
        match self {
            PiiCategory::Email => "user.email",
            PiiCategory::Jwt => "session.jwt",
            PiiCategory::Bearer => "auth.header",
            PiiCategory::ApiKey => "service.api_key",
            PiiCategory::CreditCard => "payment.pan",
            PiiCategory::Ssn => "user.ssn",
            PiiCategory::SecretKeyValue => "config.secret",
        }
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// A seeded corpus holding one structurally-valid synthetic value per [`PiiCategory`]. Built from a
/// single `u64` seed, which it retains so the emission builders derive matching span identity from
/// it; the same seed reproduces the same corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiiCorpus {
    seed: u64,
    values: [String; 7],
}

impl PiiCorpus {
    /// Generate the corpus deterministically from `seed`. Array-literal element order is the draw
    /// order, so it matches [`PiiCategory::all`] / [`PiiCategory::index`].
    pub fn seeded(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let values = [
            generate(PiiCategory::Email, &mut rng),
            generate(PiiCategory::Jwt, &mut rng),
            generate(PiiCategory::Bearer, &mut rng),
            generate(PiiCategory::ApiKey, &mut rng),
            generate(PiiCategory::CreditCard, &mut rng),
            generate(PiiCategory::Ssn, &mut rng),
            generate(PiiCategory::SecretKeyValue, &mut rng),
        ];
        Self { seed, values }
    }

    /// The seed this corpus was generated from (the emission builders reuse it for span identity).
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// The synthetic value for `category`.
    pub fn value(&self, category: PiiCategory) -> &str {
        &self.values[category.index()]
    }
}

/// Build an OTLP trace embedding the corpus across two signal carriers: a root span carrying the
/// selected categories' values as span attributes (the "spans" carrier), and a child span carrying
/// an OTel `exception` event whose message + stacktrace embed the same values (the "exceptions"
/// carrier). Span identity is seeded from [`PiiCorpus::seed`]; `start`/`end` stamps are wall-clock.
pub fn pii_trace_request(
    service_name: &str,
    corpus: &PiiCorpus,
    categories: &[PiiCategory],
) -> ExportTraceServiceRequest {
    let mut rng = ChaCha8Rng::seed_from_u64(corpus.seed());
    let trace_id = gen_id::<16>(&mut rng).to_vec();
    let root_id = gen_id::<8>(&mut rng).to_vec();
    let child_id = gen_id::<8>(&mut rng).to_vec();

    let attributes = categories
        .iter()
        .map(|c| string_kv(c.field_key(), corpus.value(*c)))
        .collect();
    let root = span_with_attributes(
        "pii.attributes",
        trace_id.clone(),
        root_id.clone(),
        Vec::new(),
        ok_status(),
        attributes,
    );
    let child = span_with_events(
        "pii.exception",
        trace_id,
        child_id,
        root_id,
        error_status("pii corpus exception"),
        vec![pii_exception_event(corpus, categories)],
    );

    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(service_resource(service_name)),
            scope_spans: vec![ScopeSpans {
                spans: vec![root, child],
                ..Default::default()
            }],
            ..Default::default()
        }],
    }
}

/// Build an OTLP logs request with one [`LogRecord`] per selected category, each embedding the
/// category's value in BOTH the record body and an attribute (the "logs" carrier).
pub fn pii_logs_request(
    service_name: &str,
    corpus: &PiiCorpus,
    categories: &[PiiCategory],
) -> ExportLogsServiceRequest {
    let log_records = categories
        .iter()
        .map(|c| pii_log_record(*c, corpus.value(*c)))
        .collect();
    ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(service_resource(service_name)),
            scope_logs: vec![ScopeLogs {
                log_records,
                ..Default::default()
            }],
            ..Default::default()
        }],
    }
}

fn pii_exception_event(corpus: &PiiCorpus, categories: &[PiiCategory]) -> Event {
    let message = categories
        .iter()
        .map(|c| corpus.value(*c))
        .collect::<Vec<_>>()
        .join("; ");
    let mut stacktrace = String::new();
    for c in categories {
        stacktrace.push_str(&format!(
            "at emit::pii::handler (pii.rs:1) -- {}\n",
            corpus.value(*c)
        ));
    }
    Event {
        time_unix_nano: unix_nanos(),
        name: "exception".to_string(),
        attributes: vec![
            string_kv("exception.type", "PiiLeak"),
            string_kv("exception.message", &message),
            string_kv("exception.stacktrace", &stacktrace),
        ],
        ..Default::default()
    }
}

fn pii_log_record(category: PiiCategory, value: &str) -> LogRecord {
    let now = unix_nanos();
    LogRecord {
        time_unix_nano: now,
        observed_time_unix_nano: now,
        body: Some(AnyValue {
            value: Some(any_value::Value::StringValue(format!(
                "request payload: {value}"
            ))),
        }),
        attributes: vec![string_kv(category.field_key(), value)],
        ..Default::default()
    }
}

fn generate(category: PiiCategory, rng: &mut ChaCha8Rng) -> String {
    match category {
        PiiCategory::Email => format!("user{:08}@example.com", rng.next_u32() % 100_000_000),
        PiiCategory::Jwt => format!(
            "{}.{}.{}",
            JWT_HEADER,
            charset_run(rng, B64URL, 24),
            charset_run(rng, B64URL, 43)
        ),
        PiiCategory::Bearer => format!("Bearer {}", charset_run(rng, B64URL, 32)),
        PiiCategory::ApiKey => format!("sk_live_{}", charset_run(rng, ALNUM, 24)),
        PiiCategory::CreditCard => luhn_pan(rng),
        PiiCategory::Ssn => {
            let area = 100 + (rng.next_u32() % 500);
            let group = 1 + (rng.next_u32() % 99);
            let serial = 1 + (rng.next_u32() % 9999);
            format!("{area:03}-{group:02}-{serial:04}")
        }
        PiiCategory::SecretKeyValue => format!("password={}", charset_run(rng, ALNUM, 16)),
    }
}

fn charset_run(rng: &mut ChaCha8Rng, charset: &[u8], len: usize) -> String {
    let mut bytes = vec![0u8; len];
    rng.fill_bytes(&mut bytes);
    bytes
        .iter()
        .map(|b| charset[*b as usize % charset.len()] as char)
        .collect()
}

/// A 16-digit Luhn-valid PAN: a fixed Visa-style leading `4`, 14 seeded digits, and the computed
/// Luhn check digit — so a card-number detector recognizes it (scope acceptance).
fn luhn_pan(rng: &mut ChaCha8Rng) -> String {
    let mut digits = [0u8; 16];
    digits[0] = 4;
    for d in digits[1..15].iter_mut() {
        *d = (rng.next_u32() % 10) as u8;
    }
    digits[15] = luhn_check_digit(&digits[..15]);
    digits.iter().map(|d| (b'0' + *d) as char).collect()
}

/// The Luhn check digit for `payload` (the digits preceding the check position).
fn luhn_check_digit(payload: &[u8]) -> u8 {
    let mut sum = 0u32;
    for (i, &d) in payload.iter().rev().enumerate() {
        let mut v = u32::from(d);
        if i % 2 == 0 {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        sum += v;
    }
    ((10 - (sum % 10)) % 10) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_proto::tonic::common::v1::KeyValue;

    fn string_value(kv: &KeyValue) -> String {
        match kv.value.as_ref().and_then(|v| v.value.as_ref()) {
            Some(any_value::Value::StringValue(s)) => s.clone(),
            _ => String::new(),
        }
    }

    fn luhn_valid(num: &str) -> bool {
        let digits: Vec<u32> = num.chars().filter_map(|c| c.to_digit(10)).collect();
        let mut sum = 0;
        for (i, &d) in digits.iter().rev().enumerate() {
            let mut v = d;
            if i % 2 == 1 {
                v *= 2;
                if v > 9 {
                    v -= 9;
                }
            }
            sum += v;
        }
        sum % 10 == 0
    }

    #[test]
    fn exposes_exactly_seven_distinct_categories() {
        let all = PiiCategory::all();
        assert_eq!(all.len(), 7);
        let mut slots: Vec<usize> = all.iter().map(|c| c.index()).collect();
        slots.sort_unstable();
        slots.dedup();
        assert_eq!(slots.len(), 7, "categories index to 7 distinct slots");
    }

    #[test]
    fn each_category_value_is_structurally_valid() {
        let corpus = PiiCorpus::seeded(7);
        for category in PiiCategory::all() {
            let v = corpus.value(category);
            match category {
                PiiCategory::Email => {
                    assert!(v.ends_with("@example.com") && v.starts_with("user"), "{v}");
                }
                PiiCategory::Jwt => {
                    assert_eq!(v.split('.').count(), 3, "JWT has three segments: {v}");
                    assert!(v.split('.').all(|s| !s.is_empty()), "{v}");
                }
                PiiCategory::Bearer => assert!(v.starts_with("Bearer "), "{v}"),
                PiiCategory::ApiKey => assert!(v.starts_with("sk_live_") && v.len() > 8, "{v}"),
                PiiCategory::CreditCard => {
                    assert_eq!(v.len(), 16, "{v}");
                    assert!(v.chars().all(|c| c.is_ascii_digit()), "{v}");
                    assert!(luhn_valid(v), "PAN must be Luhn-valid: {v}");
                }
                PiiCategory::Ssn => {
                    let parts: Vec<&str> = v.split('-').collect();
                    assert_eq!(parts.len(), 3, "{v}");
                    assert_eq!(
                        (parts[0].len(), parts[1].len(), parts[2].len()),
                        (3, 2, 4),
                        "{v}"
                    );
                    assert!(v.chars().all(|c| c.is_ascii_digit() || c == '-'), "{v}");
                }
                PiiCategory::SecretKeyValue => assert!(v.contains('='), "{v}"),
            }
        }
    }

    #[test]
    fn same_seed_reproduces_identical_corpus() {
        assert_eq!(PiiCorpus::seeded(42), PiiCorpus::seeded(42));
    }

    #[test]
    fn different_seeds_diverge() {
        let a = PiiCorpus::seeded(1);
        let b = PiiCorpus::seeded(2);
        assert_ne!(a, b);
        // the first and last draws both move with the seed — proves it threads the whole stream,
        // not just the first value.
        assert_ne!(a.value(PiiCategory::Email), b.value(PiiCategory::Email));
        assert_ne!(
            a.value(PiiCategory::SecretKeyValue),
            b.value(PiiCategory::SecretKeyValue)
        );
    }

    /// Per-span seeded shape (excludes wall-clock): ids, linkage, status, attribute + event KVs.
    type SpanShape = (
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        i32,
        Vec<(String, String)>,
        Vec<(String, String)>,
    );

    fn trace_shape(req: &ExportTraceServiceRequest) -> Vec<SpanShape> {
        req.resource_spans[0].scope_spans[0]
            .spans
            .iter()
            .map(|s| {
                let attrs = s
                    .attributes
                    .iter()
                    .map(|kv| (kv.key.clone(), string_value(kv)))
                    .collect();
                let event_attrs = s
                    .events
                    .iter()
                    .flat_map(|e| e.attributes.iter())
                    .map(|kv| (kv.key.clone(), string_value(kv)))
                    .collect();
                (
                    s.trace_id.clone(),
                    s.span_id.clone(),
                    s.parent_span_id.clone(),
                    s.status.as_ref().unwrap().code,
                    attrs,
                    event_attrs,
                )
            })
            .collect()
    }

    #[test]
    fn trace_request_shape_is_seed_deterministic() {
        let corpus = PiiCorpus::seeded(99);
        let a = pii_trace_request("svc", &corpus, &PiiCategory::all());
        let b = pii_trace_request("svc", &corpus, &PiiCategory::all());
        assert_eq!(trace_shape(&a), trace_shape(&b));

        let other = pii_trace_request("svc", &PiiCorpus::seeded(100), &PiiCategory::all());
        assert_ne!(trace_shape(&a), trace_shape(&other));
    }

    #[test]
    fn trace_carries_span_attributes_and_an_exception_event() {
        let corpus = PiiCorpus::seeded(7);
        let req = pii_trace_request("svc", &corpus, &PiiCategory::all());
        let spans = &req.resource_spans[0].scope_spans[0].spans;
        assert_eq!(spans.len(), 2);

        let root = &spans[0];
        assert!(root.parent_span_id.is_empty());
        assert_eq!(root.attributes.len(), 7);
        for category in PiiCategory::all() {
            assert!(
                root.attributes
                    .iter()
                    .any(|kv| kv.key == category.field_key()
                        && string_value(kv) == corpus.value(category)),
                "span attributes missing {category:?}"
            );
        }

        let child = &spans[1];
        assert_eq!(child.parent_span_id, root.span_id);
        assert_eq!(child.events.len(), 1);
        let event = &child.events[0];
        assert_eq!(event.name, "exception");
        let attr = |key: &str| {
            event
                .attributes
                .iter()
                .find(|kv| kv.key == key)
                .map(string_value)
                .unwrap_or_default()
        };
        let message = attr("exception.message");
        let stacktrace = attr("exception.stacktrace");
        for category in PiiCategory::all() {
            let v = corpus.value(category);
            assert!(message.contains(v), "exception.message missing {category:?}");
            assert!(
                stacktrace.contains(v),
                "exception.stacktrace missing {category:?}"
            );
        }
    }

    #[test]
    fn logs_request_embeds_each_value_in_body_and_attribute() {
        let corpus = PiiCorpus::seeded(7);
        let cats = PiiCategory::all();
        let req = pii_logs_request("svc", &corpus, &cats);
        let records = &req.resource_logs[0].scope_logs[0].log_records;
        assert_eq!(records.len(), 7);
        for (record, category) in records.iter().zip(cats) {
            let value = corpus.value(category);
            let body = match record.body.as_ref().and_then(|b| b.value.as_ref()) {
                Some(any_value::Value::StringValue(s)) => s.clone(),
                _ => String::new(),
            };
            assert!(body.contains(value), "log body missing {category:?}");
            assert!(
                record
                    .attributes
                    .iter()
                    .any(|kv| kv.key == category.field_key() && string_value(kv) == value),
                "log attribute missing {category:?}"
            );
        }
    }
}
