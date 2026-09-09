//! OTLP raw-type emission primitives (opentelemetry-proto + tonic), gRPC egress to `:4317`.
//!
//! The Epoch-3 emission seam: hand-built raw OTLP messages — trace ([`trace_request`],
//! [`error_trace_request`], [`exception_trace_request`], [`latency_trace_request`],
//! [`rate_trace_request`], [`service_topology_request`], [`pii_trace_request`]) and log records
//! with controlled `SeverityNumber` ([`severity_logs_request`]) or an embedded PII corpus
//! ([`pii_logs_request`]) — shipped over tonic gRPC
//! [`TraceEmitter`] / [`LogsEmitter`] to Pulse's loopback ingest.
//! Transport/collector faults are typed [`EmitError`] values (`Result::Err`), never verification
//! verdicts (the verdict/error wall). The pre-emission egress-liveness gate is [`probe_egress`].

mod client;
mod error;
mod exception;
mod latency;
mod logs;
mod message;
mod pii;
mod rate;
mod span_tree;
mod topology;

pub use client::{
    DEFAULT_CONNECT_TIMEOUT, DEFAULT_OTLP_ENDPOINT, LogsEmitter, TraceEmitter, probe_egress,
};
pub use error::EmitError;
pub use exception::{
    ExceptionSpec, FingerprintVariant, Frame, exception_trace_request, fingerprint,
};
pub use latency::{LatencyOp, LatencyProfile, latency_trace_request};
pub use logs::{Severity, severity_logs_request};
pub use message::{DEFAULT_SERVICE_NAME, trace_request};
pub use pii::{PiiCategory, PiiCorpus, pii_logs_request, pii_trace_request};
pub use rate::{RateCurve, rate_trace_request};
pub use span_tree::{ErrorPlacement, error_trace_request};
pub use topology::{ServiceTopology, service_topology_request};
