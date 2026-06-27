//! Harness-fault error type for the verification seam — the verdict/error-wall half.
//!
//! The read-back client raises a [`VerifyError`] (`Result::Err`) only for a *harness* fault: a
//! rejected data-dir, a sidecar that will not spawn, a transport that drops mid-call, a malformed
//! response, or a JSON-RPC error from the server. A protocol-version difference or an absent tool is
//! NOT an error here — it is reported as data the preflight gate maps to a `Blocked` verdict, the
//! same wall `conductor_faults::FaultError` and `conductor_core::CoreError` draw. `#[non_exhaustive]`
//! so later verification chunks extend the surface.

/// A harness fault raised by the MCP read-back client — never a verification verdict.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum VerifyError {
    /// The Pulse data-dir carried a shell/injection metacharacter and was refused before spawn.
    #[error("ANDROMEDA_PULSE_DATA_DIR rejected: {reason}")]
    DataDirRejected { reason: String },
    /// The sidecar process could not be spawned from its fixed program path.
    #[error("could not spawn the andromeda-pulse-mcp sidecar")]
    Spawn(#[source] std::io::Error),
    /// The read-back transport failed (stdio write/read error, or the sidecar closed its stream).
    #[error("MCP read-back transport failed")]
    Transport(#[source] std::io::Error),
    /// A request could not be serialized / the session was misused — a client-side protocol fault.
    #[error("MCP read-back protocol error: {reason}")]
    Protocol { reason: String },
    /// A response line could not be decoded as JSON (or exceeded the size bound).
    #[error("MCP read-back response decode failed: {reason}")]
    Decode { reason: String },
    /// The server returned a JSON-RPC error response. `message` is the server's text (sanitize at the
    /// edge before surfacing); the `Display` shows only the code, so the error is artifact-safe by default.
    #[error("MCP read-back returned a JSON-RPC error (code {code})")]
    JsonRpc { code: i64, message: String },
    /// The pinned MCP contract manifest could not be read, parsed, or failed its bounds check.
    /// `reason` is pre-sanitized (no host path) so it is safe at the operator edge.
    #[error("MCP contract manifest invalid: {reason}")]
    Manifest { reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_dir_rejected_displays_its_reason() {
        let err = VerifyError::DataDirRejected {
            reason: "metacharacter '$'".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "ANDROMEDA_PULSE_DATA_DIR rejected: metacharacter '$'"
        );
    }

    #[test]
    fn spawn_displays_its_message_and_chains_the_io_source() {
        let err = VerifyError::Spawn(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "program not found",
        ));
        assert_eq!(
            err.to_string(),
            "could not spawn the andromeda-pulse-mcp sidecar"
        );
        // the underlying io::Error is preserved as the chained source (the verdict/error wall).
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn jsonrpc_error_display_shows_only_the_code_not_the_server_message() {
        let err = VerifyError::JsonRpc {
            code: -32603,
            message: "incident corpus unavailable at C:/secret".to_string(),
        };
        let shown = err.to_string();
        assert!(shown.contains("-32603"), "{shown}");
        assert!(!shown.contains("secret"), "server message must not leak in Display: {shown}");
    }
}
