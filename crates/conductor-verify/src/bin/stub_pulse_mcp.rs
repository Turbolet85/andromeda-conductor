//! Test-support stub: a hand-rolled line-delimited JSON-RPC server over stdio that mimics Pulse's
//! RAW `tools/call` result shapes (no MCP `{content:[…]}` envelope) + a canary incident on
//! `query_incident_list`. Spawned as a real child by `tests/preflight_spawn.rs` so the read-back
//! client's `connect_command` stdio path is exercised end to end. Built only under the `stub-server`
//! feature — never part of a release build.

use conductor_verify::{QUERY_INCIDENT_LIST, READBACK_TOOLS};
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// Must match `tests/preflight_spawn.rs`'s expected canary marker.
const CANARY: &str = "conductor-canary-7f3a";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    let mut stdout = tokio::io::stdout();
    while let Ok(Some(line)) = lines.next_line().await {
        if line.trim().is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        // Notifications (no id) get no response — matches Pulse's sidecar.
        let Some(id) = req.get("id").cloned() else {
            continue;
        };
        let method = req.get("method").and_then(Value::as_str).unwrap_or("");
        let resp = json!({ "jsonrpc": "2.0", "id": id, "result": stub_result(method, &req) });
        let Ok(mut out) = serde_json::to_string(&resp) else {
            continue;
        };
        out.push('\n');
        if stdout.write_all(out.as_bytes()).await.is_err() || stdout.flush().await.is_err() {
            break;
        }
    }
}

/// Pulse's RAW result shapes (the un-enveloped payload as the JSON-RPC `result`).
fn stub_result(method: &str, req: &Value) -> Value {
    match method {
        "initialize" => json!({
            "protocolVersion": "2024-11-05",
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "stub-pulse", "version": "0.0.0" },
        }),
        "tools/list" => {
            let tools: Vec<Value> = READBACK_TOOLS.iter().map(|n| json!({ "name": n })).collect();
            json!({ "tools": tools })
        }
        "tools/call" => {
            let name = req.pointer("/params/name").and_then(Value::as_str).unwrap_or("");
            if name == QUERY_INCIDENT_LIST {
                json!({
                    "items": [ {
                        "incident_id": 1,
                        "status": "active",
                        "severity": "high",
                        "title": CANARY,
                        "opened_at_unix_nano": 0,
                    } ],
                    "total": 1,
                    "next_cursor": Value::Null,
                })
            } else {
                json!({ "ok": true })
            }
        }
        _ => json!({}),
    }
}
