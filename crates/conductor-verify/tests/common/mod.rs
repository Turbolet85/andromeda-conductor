//! Shared in-process Pulse stand-in for the read-back seam tests: a hand-rolled line-delimited
//! JSON-RPC server over a duplex stream that emits Pulse's RAW `tools/call` result shapes (no MCP
//! `{content:[…]}` envelope). This is the fidelity the rmcp-server stub used to hide. Per
//! `.claude/rules/testing.md`: mock Pulse over an in-process stub; never fake its reaction as a verdict.
#![allow(dead_code)]

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

/// Configurable stub behavior, one knob per readiness leg.
pub struct StubConfig {
    pub version: String,
    pub tools: Vec<String>,
    pub canary_in_corpus: bool,
    /// When set, `query_incident_list` returns a JSON-RPC ERROR (corpus unavailable) — exercises the
    /// call-error-vs-empty distinction the preflight catch-all fix introduces.
    pub query_errors: bool,
    pub canary: String,
    /// The fingerprint `retrieve_telemetry_slice` reports in `fingerprint_refs`. NOT the canary's
    /// carrier — Pulse populates that field from the L4 model's `evidence_refs`, never from its own
    /// computed fingerprint; kept because the per-check extraction still reads it.
    pub canary_fingerprint: String,
    /// The `opened_at_unix_nano` each listed incident reports — the canary leg's carrier. `None` omits
    /// the field entirely, which must read as not-fresh rather than as satisfied.
    pub opened_at_unix_nano: Option<i64>,
    /// The `markdown` body `retrieve_report` returns — the per-check extraction grades substring
    /// checks against it (Pulse's six-section Diagnostic Report).
    pub report_markdown: String,
    /// The `degraded_mode` flag `retrieve_report` REPORTS. Pulse computes it (`parsed_l4.is_none()`)
    /// and returns it; it is never an argument Conductor can pass.
    pub report_degraded: bool,
    /// Number of `span_refs` `retrieve_telemetry_slice` reports — the evidence count `CountAtLeast`
    /// checks grade against.
    pub span_ref_count: usize,
    /// When set, every `tools/call` result is a well-formed JSON value of the WRONG shape — the
    /// readers must degrade to empty rather than panicking.
    pub malformed_results: bool,
}

impl Default for StubConfig {
    fn default() -> Self {
        Self {
            version: "2024-11-05".to_string(),
            tools: [
                "query_incident_list",
                "retrieve_report",
                "retrieve_telemetry_slice",
                "mark_incident_resolved",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            canary_in_corpus: true,
            query_errors: false,
            canary: "conductor-canary-7f3a".to_string(),
            canary_fingerprint: "0123456789abcdef".to_string(),
            opened_at_unix_nano: Some(i64::MAX),
            report_markdown: "## Diagnostic Report\nRetryStorm detected on checkout-service.\n"
                .to_string(),
            report_degraded: false,
            span_ref_count: 1,
            malformed_results: false,
        }
    }
}

/// Serve the stub over a duplex half until the client closes it. Drives the same line-delimited
/// JSON-RPC contract Pulse's hand-rolled sidecar speaks: raw `result` payloads, notifications ignored.
pub async fn serve_stub<S>(io: S, config: StubConfig)
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let (reader, mut writer) = tokio::io::split(io);
    let mut lines = BufReader::new(reader).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        if line.trim().is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let Some(id) = req.get("id").cloned() else {
            continue; // notification — no response
        };
        let method = req.get("method").and_then(Value::as_str).unwrap_or("");
        let tool = req.pointer("/params/name").and_then(Value::as_str);
        let calls_query = method == "tools/call" && tool == Some("query_incident_list");
        let calls_slice = method == "tools/call" && tool == Some("retrieve_telemetry_slice");
        let calls_report = method == "tools/call" && tool == Some("retrieve_report");

        let resp = if config.query_errors && calls_query {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32603, "message": "incident corpus unavailable" },
            })
        } else {
            let result = match method {
                "initialize" => json!({
                    "protocolVersion": config.version,
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "stub-pulse", "version": "0.0.0" },
                }),
                "tools/list" => {
                    let tools: Vec<Value> =
                        config.tools.iter().map(|n| json!({ "name": n })).collect();
                    json!({ "tools": tools })
                }
                // Ahead of every per-tool arm: the knob means EVERY `tools/call` result is a
                // well-formed JSON value of the wrong shape, incident list included.
                "tools/call" if config.malformed_results => json!({ "unexpected": "shape" }),
                "tools/call" if calls_query => {
                    if config.canary_in_corpus {
                        let mut item = json!({
                            "id": 1,
                            "status": "active",
                            "severity": "high",
                            "title": config.canary,
                        });
                        if let Some(opened) = config.opened_at_unix_nano {
                            item["opened_at_unix_nano"] = json!(opened);
                        }
                        json!({ "items": [item], "total": 1, "next_cursor": Value::Null })
                    } else {
                        json!({ "items": [], "total": 0, "next_cursor": Value::Null })
                    }
                }
                "tools/call" if calls_report => json!({
                    "markdown": config.report_markdown,
                    "degraded_mode": config.report_degraded,
                }),
                "tools/call" if calls_slice => {
                    let span_refs: Vec<Value> =
                        (0..config.span_ref_count).map(|i| json!(format!("span-{i}"))).collect();
                    json!({
                        "incident_id": 1,
                        "span_refs": span_refs,
                        "fingerprint_refs": [config.canary_fingerprint],
                        "timestamps_unix_nano": [0],
                    })
                }
                "tools/call" => json!({ "ok": true }),
                _ => json!({}),
            };
            json!({ "jsonrpc": "2.0", "id": id, "result": result })
        };

        let Ok(mut out) = serde_json::to_string(&resp) else {
            break;
        };
        out.push('\n');
        if writer.write_all(out.as_bytes()).await.is_err() || writer.flush().await.is_err() {
            break;
        }
    }
}
