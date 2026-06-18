//! OTLP raw-type emission primitives (opentelemetry-proto + tonic), gRPC egress to `:4317`.
//!
//! The Epoch-3 emission seam: hand-built raw OTLP messages — trace ([`trace_request`],
//! [`error_trace_request`], [`exception_trace_request`], [`latency_trace_request`],
//! [`rate_trace_request`], [`service_topology_request`], [`pii_trace_request`]) and log records
//! with controlled `SeverityNumber` ([`severity_logs_request`]) or an embedded PII corpus
//! ([`pii_logs_request`]) — shipped over tonic gRPC
//! [`TraceEmitter`] / [`LogsEmitter`] to Pulse's loopback ingest.
//! Transport/collector faults are typed [`EmitError`] values (`Result::Err`), never verification
//! verdicts (the verdict/error wall).

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

pub use client::{LogsEmitter, TraceEmitter, DEFAULT_OTLP_ENDPOINT};
pub use error::EmitError;
pub use exception::{exception_trace_request, fingerprint, ExceptionSpec, FingerprintVariant, Frame};
pub use latency::{latency_trace_request, LatencyOp, LatencyProfile};
pub use logs::{severity_logs_request, Severity};
pub use message::{trace_request, DEFAULT_SERVICE_NAME};
pub use pii::{pii_logs_request, pii_trace_request, PiiCategory, PiiCorpus};
pub use rate::{rate_trace_request, RateCurve};
pub use span_tree::{error_trace_request, ErrorPlacement};
pub use topology::{service_topology_request, ServiceTopology};
