//! OTLP raw-type emission primitives (opentelemetry-proto + tonic), gRPC egress to `:4317`.
//!
//! The Epoch-3 emission seam: hand-built raw OTLP trace messages ([`trace_request`],
//! [`error_trace_request`], [`exception_trace_request`]) shipped over a tonic gRPC [`TraceEmitter`]
//! to Pulse's loopback ingest.
//! Transport/collector faults are typed [`EmitError`] values (`Result::Err`), never verification
//! verdicts (the verdict/error wall).

mod client;
mod error;
mod exception;
mod message;
mod span_tree;

pub use client::{TraceEmitter, DEFAULT_OTLP_ENDPOINT};
pub use error::EmitError;
pub use exception::{exception_trace_request, fingerprint, ExceptionSpec, FingerprintVariant, Frame};
pub use message::{trace_request, DEFAULT_SERVICE_NAME};
pub use span_tree::{error_trace_request, ErrorPlacement};
