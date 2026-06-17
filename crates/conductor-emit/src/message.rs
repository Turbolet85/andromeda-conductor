//! Hand-built raw OTLP trace messages (opentelemetry-proto structs, not the SDK exporter).
//!
//! Assembles a well-formed `ExportTraceServiceRequest` field-by-field so the later Epoch-3 fault
//! chunks have byte-level control over the fields they perturb (status, severity, fingerprint
//! identity, root-vs-child placement); this scaffold emits the unperturbed `OK` base
//! (architecture §Established Decisions — OTLP Emission Strategy).

use std::time::{SystemTime, UNIX_EPOCH};

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::common::v1::{any_value, AnyValue, KeyValue};
use opentelemetry_proto::tonic::resource::v1::Resource;
use opentelemetry_proto::tonic::trace::v1::{
    span::SpanKind, status::StatusCode, ResourceSpans, ScopeSpans, Span, Status,
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

fn service_resource(service_name: &str) -> Resource {
    Resource {
        attributes: vec![KeyValue {
            key: "service.name".to_string(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(service_name.to_string())),
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn ok_span(name: &str) -> Span {
    let now = unix_nanos();
    Span {
        trace_id: vec![1; 16],
        span_id: vec![1; 8],
        name: name.to_string(),
        kind: SpanKind::Internal as i32,
        start_time_unix_nano: now,
        end_time_unix_nano: now,
        status: Some(Status {
            code: StatusCode::Ok as i32,
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn unix_nanos() -> u64 {
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
