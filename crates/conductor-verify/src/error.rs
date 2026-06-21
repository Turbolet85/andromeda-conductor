//! Harness-fault error type for the verification seam — the verdict/error-wall half.
//!
//! The read-back client raises a [`VerifyError`] (`Result::Err`) only for a *harness* fault: a
//! rejected data-dir, a sidecar that will not spawn, a failed `initialize` handshake, or a transport
//! that drops mid-call. A protocol-version difference or an absent tool is NOT an error here — it is
//! reported as data the later preflight gate maps to a `Blocked` verdict, the same wall
//! `conductor_faults::FaultError` and `conductor_core::CoreError` draw. `#[non_exhaustive]` so later
//! Epoch-5 verification chunks extend the surface.

use rmcp::service::{ClientInitializeError, ServiceError};

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
    /// The MCP `initialize` handshake failed (transport refused, or the server errored during init).
    /// Boxed: the inner rmcp error is large, and an unboxed variant bloats every `Result` (clippy
    /// `result_large_err`).
    #[error("MCP initialize handshake failed")]
    Initialize(#[source] Box<ClientInitializeError>),
    /// A read-back tool call (or tool listing) failed at the transport/service layer.
    #[error("MCP read-back call failed")]
    Call(#[source] Box<ServiceError>),
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
}
