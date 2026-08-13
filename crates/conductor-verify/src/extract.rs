//! Per-check read-back extraction — Pulse's raw corpus-tool payloads → the observed values
//! [`crate::evaluate_check`] grades.
//!
//! The comparison layer is unchanged: this module only supplies what it reads. One pass over the
//! corpus composes an [`Observation`] from the three read-back tools, and
//! [`Observation::observed_for`] hands each check the slice its [`ComparisonKind`] needs —
//! substring kinds grade against the composed text, [`ComparisonKind::CountAtLeast`] against the
//! evidence count. `ExpectedCheck` carries no tool selector (and the scenario model is not this
//! chunk's to change), so the mapping is DERIVED from the kind rather than declared.
//!
//! Every outcome is a value, never a `Result::Err` (the verdict/error wall): a call failure or an
//! empty corpus becomes an [`Outcome`] variant the run seam maps to `Blocked`. The empty case is
//! deliberately NOT a graded observation — an `Absent` check against an empty string passes
//! trivially, and a false pass-as-empty is exactly what the read-back gate exists to prevent
//! (security-plan §Security Anti-Patterns → Input).
//!
//! Shapes are Pulse's, verified against `crates/mcp-server/src/tools.rs`: `query_incident_list`
//! takes no arguments and returns `{items:[{incident_id,status,severity,title,…}],total,…}`;
//! `retrieve_report` takes `{incident_id}` and returns `{markdown, degraded_mode}` — `degraded_mode`
//! is COMPUTED and returned by Pulse, never a mode Conductor can request; `retrieve_telemetry_slice`
//! takes `{incident_id}` and returns `{span_refs,fingerprint_refs,…}`.

use std::borrow::Cow;

use conductor_core::{ComparisonKind, redact_value};
use serde_json::Value;

use crate::client::ReadbackClient;
use crate::error::VerifyError;

/// What one read-back pass observed across the corpus's incidents.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Observation {
    /// The composed text every substring comparison grades against — the union across incidents, so
    /// a token leaked by ANY incident fails an `Absent` scrub check.
    pub text: String,
    /// The telemetry evidence Pulse attached, summed across incidents (`span_refs`). The sample-count
    /// floors grade against this; a floor it cannot satisfy routes to the calibration region rather
    /// than hard-failing (architecture §Timing-Tolerance Model).
    pub evidence_count: usize,
    /// Whether ANY incident's report came back under Pulse's degraded mode — the pre-accepted
    /// residual (architecture §Standard Contracts).
    pub degraded: bool,
    /// The fingerprint references across incidents — the fidelity carrier (Pulse scrubs titles).
    pub fingerprints: Vec<String>,
}

impl Observation {
    /// The observed value this check's [`ComparisonKind`] reads.
    ///
    /// `CountAtLeast` parses its observed value as an integer, so it is handed the evidence count as
    /// a bare decimal; every other kind is a substring/equality test over the composed text.
    pub fn observed_for(&self, kind: ComparisonKind) -> Cow<'_, str> {
        match kind {
            ComparisonKind::CountAtLeast => Cow::Owned(self.evidence_count.to_string()),
            ComparisonKind::Exact | ComparisonKind::Contains | ComparisonKind::Absent => {
                Cow::Borrowed(self.text.as_str())
            }
        }
    }
}

/// One read-back pass's outcome — a value in every case (the verdict/error wall).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The corpus held incidents and they were read.
    Observed(Observation),
    /// `query_incident_list` returned no incidents — nothing to grade against.
    EmptyCorpus,
    /// A read-back call / transport / JSON-RPC error, carrying its redacted reason.
    CallFailed(String),
}

/// Read the corpus once and compose the observation every check in the scenario grades against.
#[tracing::instrument(name = "verify.readback.observe", skip_all)]
pub async fn observe(client: &ReadbackClient) -> Outcome {
    let list = match client.query_incident_list(None).await {
        Ok(value) => value,
        Err(e) => return Outcome::CallFailed(call_error_reason(&e)),
    };
    let ids = incident_ids(&list);
    if ids.is_empty() {
        return Outcome::EmptyCorpus;
    }

    let mut observation = Observation { text: list_text(&list), ..Observation::default() };
    for id in &ids {
        let args = Some(serde_json::json!({ "incident_id": id }));
        match client.retrieve_report(args.clone()).await {
            Ok(report) => {
                if let Some(markdown) = report.get("markdown").and_then(Value::as_str) {
                    push_segment(&mut observation.text, markdown);
                }
                observation.degraded |= report
                    .get("degraded_mode")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
            }
            Err(e) => return Outcome::CallFailed(call_error_reason(&e)),
        }
        match client.retrieve_telemetry_slice(args).await {
            Ok(slice) => {
                observation.evidence_count += string_array(&slice, "span_refs").len();
                observation.fingerprints.extend(fingerprint_refs(&slice));
            }
            Err(e) => return Outcome::CallFailed(call_error_reason(&e)),
        }
    }

    tracing::info!(count = ids.len(), "read-back observed the incident corpus");
    if observation.degraded {
        // The flag is carried in the message, not a field: `degraded` is not on the self-obs
        // field-name allowlist and a non-allowlisted field is dropped at the processor stage
        // (obs-plan §11).
        tracing::warn!("read-back served under degraded mode — a pre-accepted residual");
    }
    Outcome::Observed(observation)
}

/// The readable per-incident fields of a `query_incident_list` result — status, severity and title,
/// the only per-incident signal available without a second call.
fn list_text(list: &Value) -> String {
    let mut text = String::new();
    let Some(items) = list.get("items").and_then(Value::as_array) else {
        return text;
    };
    for item in items {
        for key in ["status", "severity", "title"] {
            if let Some(value) = item.get(key).and_then(Value::as_str) {
                push_segment(&mut text, value);
            }
        }
    }
    text
}

/// Append a segment, keeping a separator so two segments cannot form a token neither contained.
fn push_segment(text: &mut String, segment: &str) {
    if !text.is_empty() {
        text.push('\n');
    }
    text.push_str(segment);
}

/// The incident ids in a `query_incident_list` result (`{items:[{id,…}]}`; tolerant of the stub's
/// `incident_id` item key).
pub(crate) fn incident_ids(list: &Value) -> Vec<i64> {
    list.get("items")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|it| it.get("id").or_else(|| it.get("incident_id")).and_then(Value::as_i64))
                .collect()
        })
        .unwrap_or_default()
}

/// The fingerprint reference strings in a `retrieve_telemetry_slice` result (`{fingerprint_refs:[…]}`).
pub(crate) fn fingerprint_refs(slice: &Value) -> Vec<String> {
    string_array(slice, "fingerprint_refs")
}

/// The string entries of a named array field — absent or malformed degrades to empty, never an error.
fn string_array(value: &Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|entries| entries.iter().filter_map(|e| e.as_str().map(str::to_string)).collect())
        .unwrap_or_default()
}

/// The redacted reason for a read-back call error — `JsonRpc` hides its server message behind
/// `Display`, so surface it (redacted) only here for the precondition string.
pub(crate) fn call_error_reason(e: &VerifyError) -> String {
    match e {
        VerifyError::JsonRpc { message, .. } => redact_value(message).into_owned(),
        other => redact_value(&other.to_string()).into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn observation() -> Observation {
        Observation {
            text: "active\nRetryStorm detected\nuser.email".to_string(),
            evidence_count: 12,
            degraded: false,
            fingerprints: vec!["abc".to_string()],
        }
    }

    #[test]
    fn count_at_least_reads_the_evidence_count_as_a_bare_decimal() {
        let o = observation();
        let observed = o.observed_for(ComparisonKind::CountAtLeast);
        assert_eq!(observed, "12");
        // The whole point: `compare` parses this, and a composed text blob never would.
        assert_eq!(observed.trim().parse::<i64>().unwrap(), 12);
    }

    #[test]
    fn substring_kinds_read_the_composed_text() {
        let o = observation();
        for kind in [ComparisonKind::Exact, ComparisonKind::Contains, ComparisonKind::Absent] {
            assert_eq!(o.observed_for(kind), o.text.as_str());
        }
    }

    #[test]
    fn list_text_composes_status_severity_and_title() {
        let list = json!({
            "items": [ { "incident_id": 1, "status": "active", "severity": "Suggested", "title": "t" } ],
            "total": 1,
        });
        let text = list_text(&list);
        assert!(text.contains("active"));
        assert!(text.contains("Suggested"));
        assert!(text.contains("t"));
    }

    #[test]
    fn readers_degrade_to_empty_on_a_malformed_shape() {
        for malformed in [json!({}), json!({ "items": "nope" }), json!([]), Value::Null] {
            assert!(incident_ids(&malformed).is_empty());
            assert!(fingerprint_refs(&malformed).is_empty());
            assert!(list_text(&malformed).is_empty());
        }
    }

    #[test]
    fn incident_ids_accepts_either_item_key() {
        assert_eq!(incident_ids(&json!({ "items": [{ "id": 7 }] })), vec![7]);
        assert_eq!(incident_ids(&json!({ "items": [{ "incident_id": 9 }] })), vec![9]);
    }

    #[test]
    fn segments_are_separated_so_no_token_spans_two_of_them() {
        let mut text = String::new();
        push_segment(&mut text, "Retry");
        push_segment(&mut text, "Storm");
        assert!(!text.contains("RetryStorm"));
    }
}
