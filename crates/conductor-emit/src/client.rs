//! OTLP/gRPC egress — ships raw trace + log requests to the loopback collector via tonic.
//!
//! A runtime-agnostic async client (the caller owns the runtime, matching `conductor_timeline`):
//! it connects a tonic [`Channel`] to the collector and exports `ExportTraceServiceRequest`s /
//! `ExportLogsServiceRequest`s. A refused/invalid transport is an [`EmitError`] (`Result::Err`) —
//! the OTLP-egress liveness surface at this layer, made explicit by the [`probe_egress`] gate — and
//! a collector-returned `tonic::Status` is a typed input, never a panic (the verdict/error wall).

use std::time::Duration;

use opentelemetry_proto::tonic::collector::logs::v1::{
    logs_service_client::LogsServiceClient, ExportLogsServiceRequest,
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_client::TraceServiceClient, ExportTraceServiceRequest,
};
use tonic::transport::{Channel, Endpoint};

use crate::error::EmitError;

/// Default OTLP/gRPC egress target — Pulse's loopback ingest (`:4318` is unused).
pub const DEFAULT_OTLP_ENDPOINT: &str = "http://127.0.0.1:4317";

/// Bounded connect timeout for OTLP/gRPC egress — fail-fast, real wall-clock (`std::time`),
/// never tokio's virtual clock; a connect bound, not an SLO tier.
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

/// Build the bounded egress [`Endpoint`] for `endpoint`, applying [`DEFAULT_CONNECT_TIMEOUT`].
/// A malformed endpoint surfaces as [`EmitError::Transport`].
fn egress_endpoint(endpoint: impl Into<String>) -> Result<Endpoint, EmitError> {
    Ok(Endpoint::from_shared(endpoint.into())?.connect_timeout(DEFAULT_CONNECT_TIMEOUT))
}

/// OTLP-egress liveness probe — confirm Pulse's loopback gRPC ingest is connectable before emission.
///
/// Connects a tonic [`Channel`] to `endpoint` (e.g. [`DEFAULT_OTLP_ENDPOINT`]) with the bounded
/// [`DEFAULT_CONNECT_TIMEOUT`], then drops it — no OTLP message is exported. A refused, unreachable,
/// or timed-out transport surfaces as [`EmitError::Transport`] (`Result::Err`) — a harness fault,
/// never a verification verdict (the verdict/error wall). Orchestrating the gate into a run is the
/// CLI's job.
pub async fn probe_egress(endpoint: impl Into<String>) -> Result<(), EmitError> {
    let _ = egress_endpoint(endpoint)?.connect().await?;
    Ok(())
}

/// A thin OTLP/gRPC trace emitter over a tonic [`Channel`].
pub struct TraceEmitter {
    client: TraceServiceClient<Channel>,
}

impl TraceEmitter {
    /// Connect to `endpoint` (e.g. [`DEFAULT_OTLP_ENDPOINT`]) with the bounded
    /// [`DEFAULT_CONNECT_TIMEOUT`]. A refused or malformed endpoint surfaces as [`EmitError::Transport`].
    pub async fn connect(endpoint: impl Into<String>) -> Result<Self, EmitError> {
        let channel = egress_endpoint(endpoint)?.connect().await?;
        Ok(Self::from_channel(channel))
    }

    /// Build an emitter over an already-established [`Channel`] (the in-process test path).
    pub fn from_channel(channel: Channel) -> Self {
        Self {
            client: TraceServiceClient::new(channel),
        }
    }

    /// Export one trace request. A collector-returned error status surfaces as [`EmitError::Status`].
    #[tracing::instrument(name = "emit.batch", skip_all, fields(emission_count = count_spans(&request)))]
    pub async fn export(&mut self, request: ExportTraceServiceRequest) -> Result<(), EmitError> {
        self.client.export(request).await?;
        Ok(())
    }
}

fn count_spans(request: &ExportTraceServiceRequest) -> u64 {
    request
        .resource_spans
        .iter()
        .flat_map(|rs| rs.scope_spans.iter())
        .map(|ss| ss.spans.len() as u64)
        .sum()
}

/// A thin OTLP/gRPC logs emitter over a tonic [`Channel`] — the logs sibling of [`TraceEmitter`].
pub struct LogsEmitter {
    client: LogsServiceClient<Channel>,
}

impl LogsEmitter {
    /// Connect to `endpoint` (e.g. [`DEFAULT_OTLP_ENDPOINT`]) with the bounded
    /// [`DEFAULT_CONNECT_TIMEOUT`]. A refused or malformed endpoint surfaces as [`EmitError::Transport`].
    pub async fn connect(endpoint: impl Into<String>) -> Result<Self, EmitError> {
        let channel = egress_endpoint(endpoint)?.connect().await?;
        Ok(Self::from_channel(channel))
    }

    /// Build an emitter over an already-established [`Channel`] (the in-process test path).
    pub fn from_channel(channel: Channel) -> Self {
        Self {
            client: LogsServiceClient::new(channel),
        }
    }

    /// Export one logs request. A collector-returned error status surfaces as [`EmitError::Status`].
    #[tracing::instrument(name = "emit.logs_batch", skip_all, fields(record_count = count_records(&request)))]
    pub async fn export(&mut self, request: ExportLogsServiceRequest) -> Result<(), EmitError> {
        self.client.export(request).await?;
        Ok(())
    }
}

fn count_records(request: &ExportLogsServiceRequest) -> u64 {
    request
        .resource_logs
        .iter()
        .flat_map(|rl| rl.scope_logs.iter())
        .map(|sl| sl.log_records.len() as u64)
        .sum()
}
