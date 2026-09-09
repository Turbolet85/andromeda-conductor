//! A minimal line-delimited JSON-RPC 2.0 client session over a child/stdio transport.
//!
//! Pulse's `andromeda-pulse-mcp` is hand-rolled JSON-RPC (`crates/mcp-server/src/jsonrpc.rs`) that
//! returns the RAW tool payload as the `result` field — it does NOT wrap `tools/call` results in the
//! MCP `{content:[…]}` envelope. rmcp's typed client deserializes every result into the typed
//! `ServerResult` enum and rejects the raw shape (`UnexpectedResponse`), so the read-back client
//! speaks JSON-RPC directly here and hands back the raw [`serde_json::Value`] for the caller to read
//! (architecture §Established Decisions [MCP Read-Back Client] reversal — Pulse is itself hand-rolled).

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, Lines};

use crate::error::VerifyError;

/// Soft bound on a single response line — a malformed/oversized line from the child is a typed
/// `Decode` error, never an unbounded read driving the run out of memory (security-plan §Input
/// Validation; serde_json itself recursion-limits nested decoding).
const MAX_LINE_BYTES: usize = 16 * 1024 * 1024;

/// One JSON-RPC stdio session: an id-correlated request/response channel over a boxed transport
/// (the spawned child's stdio, or an in-process duplex in tests).
pub(crate) struct JsonRpcSession {
    writer: Box<dyn AsyncWrite + Unpin + Send>,
    reader: Lines<BufReader<Box<dyn AsyncRead + Unpin + Send>>>,
    next_id: i64,
}

impl JsonRpcSession {
    pub(crate) fn new(
        reader: Box<dyn AsyncRead + Unpin + Send>,
        writer: Box<dyn AsyncWrite + Unpin + Send>,
    ) -> Self {
        Self {
            writer,
            reader: BufReader::new(reader).lines(),
            next_id: 1,
        }
    }

    /// Send a request and return the raw `result` value (`Null` if the response omits it). A
    /// JSON-RPC `error` response is a typed [`VerifyError::JsonRpc`]; transport/decode faults are
    /// their own variants — never a panic (the verdict/error wall).
    pub(crate) async fn request(
        &mut self,
        method: &str,
        params: Value,
    ) -> Result<Value, VerifyError> {
        let id = self.next_id;
        self.next_id += 1;
        let req = json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params });
        self.write_message(&req).await?;
        loop {
            let line = self.read_line().await?;
            let msg: Value = serde_json::from_str(&line).map_err(|e| VerifyError::Decode {
                reason: e.to_string(),
            })?;
            // Skip notifications / unrelated ids; the response we await carries our id.
            if msg.get("id") != Some(&json!(id)) {
                continue;
            }
            if let Some(err) = msg.get("error") {
                let code = err.get("code").and_then(Value::as_i64).unwrap_or(0);
                let message = err
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
                    .to_string();
                return Err(VerifyError::JsonRpc { code, message });
            }
            return Ok(msg.get("result").cloned().unwrap_or(Value::Null));
        }
    }

    /// Send a JSON-RPC notification (no id, no response expected) — used for the post-`initialize`
    /// `notifications/initialized` handshake step (MCP hygiene; Pulse tolerates its absence).
    pub(crate) async fn notify(&mut self, method: &str, params: Value) -> Result<(), VerifyError> {
        let n = json!({ "jsonrpc": "2.0", "method": method, "params": params });
        self.write_message(&n).await
    }

    async fn write_message(&mut self, msg: &Value) -> Result<(), VerifyError> {
        let mut line = serde_json::to_string(msg).map_err(|e| VerifyError::Protocol {
            reason: e.to_string(),
        })?;
        line.push('\n');
        self.writer
            .write_all(line.as_bytes())
            .await
            .map_err(VerifyError::Transport)?;
        self.writer.flush().await.map_err(VerifyError::Transport)
    }

    async fn read_line(&mut self) -> Result<String, VerifyError> {
        match self
            .reader
            .next_line()
            .await
            .map_err(VerifyError::Transport)?
        {
            Some(line) if line.len() > MAX_LINE_BYTES => Err(VerifyError::Decode {
                reason: "response line exceeds size bound".to_string(),
            }),
            Some(line) => Ok(line),
            None => Err(VerifyError::Transport(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "sidecar closed stdout",
            ))),
        }
    }
}
