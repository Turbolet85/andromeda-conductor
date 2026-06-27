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
        let calls_query = method == "tools/call"
            && req.pointer("/params/name").and_then(Value::as_str) == Some("query_incident_list");

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
                "tools/call" if calls_query => {
                    if config.canary_in_corpus {
                        json!({
                            "items": [ {
                                "incident_id": 1,
                                "status": "active",
                                "severity": "high",
                                "title": config.canary,
                                "opened_at_unix_nano": 0,
                            } ],
                            "total": 1,
                            "next_cursor": Value::Null,
                        })
                    } else {
                        json!({ "items": [], "total": 0, "next_cursor": Value::Null })
                    }
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
