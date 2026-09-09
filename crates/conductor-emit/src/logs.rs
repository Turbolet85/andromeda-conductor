//! Raw OTLP log records with caller-controlled `SeverityNumber` — the logs analogue of the trace
//! primitives (P-007). Drives Pulse's severity-boundary hard signal: the OTel WARN(13–16)→ERROR
//! (17–20) cut, where `SeverityNumber >= 17` is error-grade. Severity is spec-controlled, not
//! seed-derived; wall-clock stamps come from `std::time` (architecture §Cross-cutting Patterns —
//! Determinism discipline), so the determinism contract governs the `(severity, body)` shape only.

use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use opentelemetry_proto::tonic::common::v1::{AnyValue, any_value};
use opentelemetry_proto::tonic::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};

use crate::message::{service_resource, unix_nanos};

/// Canonical OTel `SeverityText` short-names, indexed by `SeverityNumber - 1` (over `1..=24`).
const SEVERITY_TEXT: [&str; 24] = [
    "TRACE", "TRACE2", "TRACE3", "TRACE4", "DEBUG", "DEBUG2", "DEBUG3", "DEBUG4", "INFO", "INFO2",
    "INFO3", "INFO4", "WARN", "WARN2", "WARN3", "WARN4", "ERROR", "ERROR2", "ERROR3", "ERROR4",
    "FATAL", "FATAL2", "FATAL3", "FATAL4",
];

/// Benign fixed log body — never a host path or struct name (security-plan §Error Handling).
const LOG_BODY: &str = "conductor severity probe";

/// An OTel `SeverityNumber` paired with its canonical `SeverityText`. Constructed only from a legal
/// number (`1..=24`), so an emitted record can never carry a mismatched or out-of-range pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Severity {
    number: i32,
}

impl Severity {
    /// Build from a `SeverityNumber` in `1..=24`; `None` if out of range.
    pub fn new(number: i32) -> Option<Self> {
        (1..=24).contains(&number).then_some(Self { number })
    }

    /// The OTel `SeverityNumber` (`1..=24`).
    pub fn number(self) -> i32 {
        self.number
    }

    /// The canonical `SeverityText` derived from the number (e.g. `16 -> "WARN4"`, `17 -> "ERROR"`).
    pub fn text(self) -> &'static str {
        SEVERITY_TEXT[(self.number - 1) as usize]
    }
}

/// Build one OTLP log-export request: one `LogRecord` per `severity`, each under `service_name`
/// carrying its number + matching text. Pass severities straddling 17 (e.g. `16` and `17`) to
/// exercise the WARN→ERROR boundary in a single request.
pub fn severity_logs_request(
    service_name: &str,
    severities: &[Severity],
) -> ExportLogsServiceRequest {
    let log_records = severities.iter().map(|s| log_record(*s)).collect();
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

fn log_record(severity: Severity) -> LogRecord {
    let now = unix_nanos();
    LogRecord {
        time_unix_nano: now,
        observed_time_unix_nano: now,
        severity_number: severity.number(),
        severity_text: severity.text().to_string(),
        body: Some(AnyValue {
            value: Some(any_value::Value::StringValue(LOG_BODY.to_string())),
        }),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_proto::tonic::logs::v1::SeverityNumber;

    /// Project a request to the determinism-governed shape — `(number, text, body)`, no wall-clock.
    fn shape(req: &ExportLogsServiceRequest) -> Vec<(i32, String, Option<String>)> {
        req.resource_logs[0].scope_logs[0]
            .log_records
            .iter()
            .map(|r| {
                let body = r.body.as_ref().and_then(|b| match b.value.as_ref() {
                    Some(any_value::Value::StringValue(s)) => Some(s.clone()),
                    _ => None,
                });
                (r.severity_number, r.severity_text.clone(), body)
            })
            .collect()
    }

    #[test]
    fn severity_text_matches_otel_short_names() {
        assert_eq!(Severity::new(1).unwrap().text(), "TRACE");
        assert_eq!(Severity::new(13).unwrap().text(), "WARN");
        assert_eq!(Severity::new(16).unwrap().text(), "WARN4");
        assert_eq!(Severity::new(17).unwrap().text(), "ERROR");
        assert_eq!(Severity::new(24).unwrap().text(), "FATAL4");
    }

    #[test]
    fn severity_numbers_match_the_otel_enum() {
        assert_eq!(
            Severity::new(16).unwrap().number(),
            SeverityNumber::Warn4 as i32
        );
        assert_eq!(
            Severity::new(17).unwrap().number(),
            SeverityNumber::Error as i32
        );
    }

    #[test]
    fn out_of_range_severity_is_rejected() {
        assert!(Severity::new(0).is_none());
        assert!(Severity::new(25).is_none());
        assert!(Severity::new(1).is_some());
        assert!(Severity::new(24).is_some());
    }

    #[test]
    fn builds_a_record_per_severity_across_the_boundary() {
        let warn = Severity::new(16).unwrap();
        let error = Severity::new(17).unwrap();
        let req = severity_logs_request("svc", &[warn, error]);

        let rl = &req.resource_logs[0];
        let svc = rl
            .resource
            .as_ref()
            .unwrap()
            .attributes
            .iter()
            .find(|kv| kv.key == "service.name")
            .expect("service.name present");
        assert_eq!(
            svc.value.as_ref().unwrap().value,
            Some(any_value::Value::StringValue("svc".to_string()))
        );

        let records = &rl.scope_logs[0].log_records;
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].severity_number, 16);
        assert_eq!(records[0].severity_text, "WARN4");
        assert_eq!(records[1].severity_number, 17);
        assert_eq!(records[1].severity_text, "ERROR");
        assert_ne!(records[0].severity_number, records[1].severity_number);
    }

    #[test]
    fn same_inputs_reproduce_identical_shape() {
        let sevs = [Severity::new(16).unwrap(), Severity::new(17).unwrap()];
        let a = severity_logs_request("svc", &sevs);
        let b = severity_logs_request("svc", &sevs);
        assert_eq!(shape(&a), shape(&b));
    }
}
