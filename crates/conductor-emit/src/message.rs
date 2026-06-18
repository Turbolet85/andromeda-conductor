//! Hand-built raw OTLP trace messages (opentelemetry-proto structs, not the SDK exporter).
//!
//! Assembles a well-formed `ExportTraceServiceRequest` field-by-field so the later Epoch-3 fault
//! chunks have byte-level control over the fields they perturb (status, severity, fingerprint
//! identity, root-vs-child placement); this module owns the shared raw-struct primitives
//! ([`span`], [`service_resource`], status builders) over which [`crate::span_tree`] composes
//! multi-span error traces (architecture §Established Decisions — OTLP Emission Strategy).

use std::time::{SystemTime, UNIX_EPOCH};

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::common::v1::{any_value, AnyValue, KeyValue};
use opentelemetry_proto::tonic::resource::v1::Resource;
use opentelemetry_proto::tonic::trace::v1::{
    span::{Event, SpanKind}, status::StatusCode, ResourceSpans, ScopeSpans, Span, Status,
};

/// Default `service.name` resource attribute stamped on emitted spans.
pub const DEFAULT_SERVICE_NAME: &str = "conductor";

/// Build a single well-formed OTLP trace export request: one `OK` span under `service_name`.
pub fn trace_request(service_name: &str, span_name: &str) -> ExportTraceServiceRequest {
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(service_resource(service_name)),
            scope_spans: vec![ScopeSpans {
                spans: vec![ok_span(span_name)],
                ..Default::default()
            }],
            ..Default::default()
        }],
    }
}

pub(crate) fn service_resource(service_name: &str) -> Resource {
    Resource {
        attributes: vec![string_kv("service.name", service_name)],
        ..Default::default()
    }
}

/// A string-valued OTLP [`KeyValue`] attribute — the only attribute value-type Conductor emits.
pub(crate) fn string_kv(key: &str, value: &str) -> KeyValue {
    KeyValue {
        key: key.to_string(),
        value: Some(AnyValue {
            value: Some(any_value::Value::StringValue(value.to_string())),
        }),
        ..Default::default()
    }
}

/// Build a span with caller-supplied identity, linkage, and status — no span events.
/// `parent_span_id` is empty for a root span. Delegates to [`span_with_events`].
pub(crate) fn span(
    name: &str,
    trace_id: Vec<u8>,
    span_id: Vec<u8>,
    parent_span_id: Vec<u8>,
    status: Status,
) -> Span {
    span_with_events(name, trace_id, span_id, parent_span_id, status, Vec::new())
}

/// As [`span`], but carrying span [`Event`]s (e.g. an OTel `exception` event). Timestamps are
/// wall-clock (`std::time`): the seeded determinism lives in the identity bytes, not the clock
/// (architecture §Cross-cutting Patterns — Determinism discipline).
pub(crate) fn span_with_events(
    name: &str,
    trace_id: Vec<u8>,
    span_id: Vec<u8>,
    parent_span_id: Vec<u8>,
    status: Status,
    events: Vec<Event>,
) -> Span {
    let now = unix_nanos();
    Span {
        trace_id,
        span_id,
        parent_span_id,
        name: name.to_string(),
        kind: SpanKind::Internal as i32,
        start_time_unix_nano: now,
        end_time_unix_nano: now,
        status: Some(status),
        events,
        ..Default::default()
    }
}

/// As [`span`], but with a caller-supplied duration: `end_time = start_time + duration_nanos` (the
/// seeded latency-shaping output — [`crate::latency`]). `start` is wall-clock `unix_nanos()`, so the
/// seed governs the duration, not the absolute stamps (architecture §Cross-cutting Patterns).
pub(crate) fn timed_span(
    name: &str,
    trace_id: Vec<u8>,
    span_id: Vec<u8>,
    parent_span_id: Vec<u8>,
    status: Status,
    duration_nanos: u64,
) -> Span {
    let start = unix_nanos();
    Span {
        trace_id,
        span_id,
        parent_span_id,
        name: name.to_string(),
        kind: SpanKind::Internal as i32,
        start_time_unix_nano: start,
        end_time_unix_nano: start.saturating_add(duration_nanos),
        status: Some(status),
        ..Default::default()
    }
}

pub(crate) fn ok_status() -> Status {
    Status {
        code: StatusCode::Ok as i32,
        ..Default::default()
    }
}

pub(crate) fn error_status(message: &str) -> Status {
    Status {
        code: StatusCode::Error as i32,
        message: message.to_string(),
    }
}

fn ok_span(name: &str) -> Span {
    span(name, vec![1; 16], vec![1; 8], Vec::new(), ok_status())
}

pub(crate) fn unix_nanos() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_one_span_under_service_name() {
        let req = trace_request(DEFAULT_SERVICE_NAME, "baseline");
        assert_eq!(req.resource_spans.len(), 1);
        let rs = &req.resource_spans[0];
        let attrs = &rs.resource.as_ref().unwrap().attributes;
        let svc = attrs
            .iter()
            .find(|kv| kv.key == "service.name")
            .expect("service.name attribute present");
        assert_eq!(
            svc.value.as_ref().unwrap().value,
            Some(any_value::Value::StringValue("conductor".to_string()))
        );
        assert_eq!(rs.scope_spans.len(), 1);
        assert_eq!(rs.scope_spans[0].spans.len(), 1);
    }

    #[test]
    fn span_is_well_formed_ok_status() {
        let req = trace_request(DEFAULT_SERVICE_NAME, "baseline");
        let span = &req.resource_spans[0].scope_spans[0].spans[0];
        assert_eq!(span.name, "baseline");
        assert_eq!(span.trace_id.len(), 16);
        assert_eq!(span.span_id.len(), 8);
        assert_eq!(span.kind, SpanKind::Internal as i32);
        assert_eq!(span.status.as_ref().unwrap().code, StatusCode::Ok as i32);
        assert!(span.end_time_unix_nano >= span.start_time_unix_nano);
    }
}
