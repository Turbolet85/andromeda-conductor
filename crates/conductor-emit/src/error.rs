//! Harness-fault error type for the emission seam — the verdict/error-wall half.
//!
//! OTLP egress faults (a refused/invalid gRPC transport, a collector-returned `tonic::Status`)
//! are Conductor's own harness failures and ride in `Result::Err` — never a verification outcome
//! (mirrors [`conductor_core::CoreError`]). `#[non_exhaustive]` so later Epoch-3 chunks extend the
//! fault surface.

/// A harness fault from the OTLP emission seam — never a verification verdict.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum EmitError {
    /// The gRPC channel to the collector could not be established (refused or invalid endpoint).
    #[error("OTLP transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    /// The collector returned a gRPC error status for an export request.
    #[error("OTLP export rejected: {0}")]
    Status(#[from] tonic::Status),
}
