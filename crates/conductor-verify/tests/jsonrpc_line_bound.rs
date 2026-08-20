//! Both sides of the read-back per-line size bound.
//!
//! `.claude/rules/security.md` REQUIRES a soft per-line size bound on the sidecar's stdout — no
//! unbounded read. The bound is a value, not merely a guard, so both sides are asserted: a line AT the
//! limit is accepted (which pins the constant itself), a line one byte OVER yields the typed
//! `VerifyError::Decode` (which pins the guard and its comparison). Driven through the public
//! `connect_transport` seam over an in-process duplex — the reader is crate-private by design.

use conductor_verify::{ReadbackClient, VerifyError};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// Mirror of the crate-private `jsonrpc::MAX_LINE_BYTES`. A drift here is the point: if the shipped
/// bound moves, the at-limit leg stops being at the limit and these tests say so.
const MAX_LINE_BYTES: usize = 16 * 1024 * 1024;

/// A well-formed `initialize` response padded to EXACTLY `len` bytes (excluding the newline).
fn initialize_response_of_len(len: usize) -> String {
    let head = r#"{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","pad":""#;
    let tail = r#""}}"#;
    let pad = len - head.len() - tail.len();
    format!("{head}{}{tail}", "p".repeat(pad))
}

/// Serve one response line over a duplex, then drain whatever else the client writes (the best-effort
/// `notifications/initialized`) so the client never blocks on a full pipe.
fn serve_line(line: String) -> tokio::io::DuplexStream {
    let (client, server) = tokio::io::duplex(64 * 1024);
    tokio::spawn(async move {
        let (read_half, mut write_half) = tokio::io::split(server);
        let mut lines = BufReader::new(read_half).lines();
        if lines.next_line().await.ok().flatten().is_some() {
            let _ = write_half.write_all(line.as_bytes()).await;
            let _ = write_half.write_all(b"\n").await;
            let _ = write_half.flush().await;
        }
        while let Ok(Some(_)) = lines.next_line().await {}
    });
    client
}

#[tokio::test(flavor = "current_thread")]
async fn a_line_at_the_size_bound_is_accepted() {
    let line = initialize_response_of_len(MAX_LINE_BYTES);
    assert_eq!(line.len(), MAX_LINE_BYTES, "the leg must sit exactly ON the bound");

    let client = ReadbackClient::connect_transport(serve_line(line))
        .await
        .expect("a line at exactly MAX_LINE_BYTES is within the bound");

    assert_eq!(
        client.negotiated_protocol_version(),
        Some("2024-11-05"),
        "the at-limit response must be parsed, not merely tolerated"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn a_line_one_byte_over_the_size_bound_is_a_typed_decode_error() {
    let line = initialize_response_of_len(MAX_LINE_BYTES + 1);
    assert_eq!(line.len(), MAX_LINE_BYTES + 1, "the leg must sit one byte OVER the bound");

    // `ReadbackClient` is not `Debug`, so unwrap the error by matching rather than `expect_err`.
    let err = match ReadbackClient::connect_transport(serve_line(line)).await {
        Ok(_) => panic!("a line over MAX_LINE_BYTES must be rejected"),
        Err(e) => e,
    };

    assert!(
        matches!(err, VerifyError::Decode { .. }),
        "an oversized line is a typed Decode fault, never a panic or a silent truncation: {err:?}"
    );
}
