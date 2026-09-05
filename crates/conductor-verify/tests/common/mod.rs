//! Shared in-process Pulse stand-in for the read-back seam tests: a hand-rolled line-delimited
//! JSON-RPC server over a duplex stream that emits Pulse's RAW `tools/call` result shapes (no MCP
//! `{content:[…]}` envelope). This is the fidelity the rmcp-server stub used to hide. Per
//! `.claude/rules/testing.md`: mock Pulse over an in-process stub; never fake its reaction as a verdict.
#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

/// The tool name carried by a stale decoy response, so a client that pairs it to the wrong request
/// says so in its own return value rather than merely reading a different line.
pub const DECOY_TOOL: &str = "stale-decoy-tool";

/// The wall-clock ceiling a read-back call may take before the test calls it a hang.
///
/// A mutation that stops a response ever pairing (an inverted id guard) or a request ever being
/// written blocks `read_line` forever, so the test binary hangs and the runner reports TIMEOUT —
/// the assertion that would have failed never gets to speak (`.claude/rules/testing.md` 2026-09-03).
/// Held well under cargo-mutants' own per-test timeout (auto = 5x the baseline, floored at 20s): a
/// bound at that floor races the harness and the kill is not credited.
const READ_BACK_BOUND: Duration = Duration::from_secs(5);

/// Await `fut` under [`READ_BACK_BOUND`], turning a hang into a named failure.
pub async fn bounded<F: std::future::Future>(fut: F) -> F::Output {
    tokio::time::timeout(READ_BACK_BOUND, fut)
        .await
        .expect("read-back call hung past the bound (a mutated id guard or unwritten request)")
}

/// One line the stub read off the wire, in arrival order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WireEntry {
    Request { id: i64, method: String },
    Notification { method: String },
}

/// What the stub observed on the wire. Cloneable, because `StubConfig` moves into [`serve_stub`] and
/// the test needs to keep its end — and because for a notification there is nothing else to observe:
/// the client discards `notify`'s result at its single call site by design.
#[derive(Clone, Default)]
pub struct WireLog(Arc<Mutex<Vec<WireEntry>>>);

impl WireLog {
    pub fn entries(&self) -> Vec<WireEntry> {
        self.0.lock().expect("wire log lock").clone()
    }

    pub fn request_ids(&self) -> Vec<i64> {
        self.entries()
            .into_iter()
            .filter_map(|entry| match entry {
                WireEntry::Request { id, .. } => Some(id),
                WireEntry::Notification { .. } => None,
            })
            .collect()
    }

    fn push(&self, entry: WireEntry) {
        self.0.lock().expect("wire log lock").push(entry);
    }
}

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
    /// When set, every line the stub reads is recorded in arrival order. The stub ECHOES the id it
    /// received, so it answers any id sequence equally well — the recorded sequence is the only
    /// witness that ids advance at all.
    pub wire_log: Option<WireLog>,
    /// Emit a stale `id: 1` decoy response immediately BEFORE the real answer to the Nth request
    /// (1-based, notifications excluded). A client whose ids advance skips it on id mismatch; one
    /// whose id never leaves 1 pairs it to a later request.
    pub decoy_before_nth_request: Option<u32>,
    /// When set, `mark_incident_resolved` answers with a JSON-RPC ERROR carrying Pulse's own
    /// declined reason. This arm exists ONLY here: Pulse guards the write on
    /// `updated_unix_nano <= ?2` and its dispatch stamps `now` fresh on every call, so the decline
    /// is reachable live only against a future-stamped row. Stub-proven, and recorded as such.
    pub resolve_declines: bool,
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
            wire_log: None,
            decoy_before_nth_request: None,
            resolve_declines: false,
        }
    }
}

async fn write_line<W>(writer: &mut W, value: &Value) -> std::io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    let mut out = serde_json::to_string(value).map_err(std::io::Error::other)?;
    out.push('\n');
    writer.write_all(out.as_bytes()).await?;
    writer.flush().await
}

/// Serve the stub over a duplex half until the client closes it. Drives the same line-delimited
/// JSON-RPC contract Pulse's hand-rolled sidecar speaks: raw `result` payloads, notifications ignored.
pub async fn serve_stub<S>(io: S, config: StubConfig)
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let (reader, mut writer) = tokio::io::split(io);
    let mut lines = BufReader::new(reader).lines();
    let mut request_index: u32 = 0;
    while let Ok(Some(line)) = lines.next_line().await {
        if line.trim().is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let method_name = req
            .get("method")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let Some(id) = req.get("id").cloned() else {
            if let Some(log) = &config.wire_log {
                log.push(WireEntry::Notification {
                    method: method_name,
                });
            }
            continue; // notification — no response
        };
        request_index += 1;
        if let Some(log) = &config.wire_log {
            let observed = id.as_i64().unwrap_or(i64::MIN);
            log.push(WireEntry::Request {
                id: observed,
                method: method_name.clone(),
            });
        }
        if config.decoy_before_nth_request == Some(request_index) {
            let decoy = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": { "tools": [{ "name": DECOY_TOOL }] },
            });
            if write_line(&mut writer, &decoy).await.is_err() {
                break;
            }
        }
        let method = method_name.as_str();
        let tool = req.pointer("/params/name").and_then(Value::as_str);
        let calls_query = method == "tools/call" && tool == Some("query_incident_list");
        let calls_slice = method == "tools/call" && tool == Some("retrieve_telemetry_slice");
        let calls_report = method == "tools/call" && tool == Some("retrieve_report");
        let calls_resolve = method == "tools/call" && tool == Some("mark_incident_resolved");

        let resp = if config.query_errors && calls_query {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32603, "message": "incident corpus unavailable" },
            })
        } else if config.resolve_declines && calls_resolve {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32603,
                    "message": "incident changed concurrently; resolution not applied",
                },
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
                        // `incident_id` is the LIVE key — measured 2026-09-01 by driving the real
                        // sidecar by hand at Pulse HEAD `83d4060`. The stub said `id` until then,
                        // and a reader that accepted only `id` therefore passed every stub test and
                        // read an empty list against the live corpus.
                        let mut item = json!({
                            "incident_id": 1,
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
                    let span_refs: Vec<Value> = (0..config.span_ref_count)
                        .map(|i| json!(format!("span-{i}")))
                        .collect();
                    json!({
                        "incident_id": 1,
                        "span_refs": span_refs,
                        "fingerprint_refs": [config.canary_fingerprint],
                        "timestamps_unix_nano": [0],
                    })
                }
                // Pulse's applied shape echoes the row it wrote, so the id is read back from the
                // request rather than fixed — a caller that resolves the wrong incident is then
                // visible in the result instead of being answered agreeably.
                "tools/call" if calls_resolve => {
                    let requested = req
                        .pointer("/params/arguments/incident_id")
                        .and_then(Value::as_i64)
                        .unwrap_or(1);
                    json!({ "resolved": true, "incident_id": requested })
                }
                "tools/call" => json!({ "ok": true }),
                _ => json!({}),
            };
            json!({ "jsonrpc": "2.0", "id": id, "result": result })
        };

        if write_line(&mut writer, &resp).await.is_err() {
            break;
        }
    }
}
