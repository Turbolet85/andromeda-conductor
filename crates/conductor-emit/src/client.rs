//! OTLP/gRPC egress — ships raw trace requests to the loopback collector via tonic.
//!
//! A runtime-agnostic async client (the caller owns the runtime, matching `conductor_timeline`):
//! it connects a tonic [`Channel`] to the collector and exports `ExportTraceServiceRequest`s. A
//! refused/invalid transport is an [`EmitError`] (`Result::Err`) — the OTLP-egress liveness surface
//! at this layer — and a collector-returned `tonic::Status` is a typed input, never a panic (the
//! verdict/error wall).

use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_client::TraceServiceClient, ExportTraceServiceRequest,
};
use tonic::transport::{Channel, Endpoint};

use crate::error::EmitError;

/// Default OTLP/gRPC egress target — Pulse's loopback ingest (`:4318` is unused).
pub const DEFAULT_OTLP_ENDPOINT: &str = "http://127.0.0.1:4317";

/// A thin OTLP/gRPC trace emitter over a tonic [`Channel`].
pub struct TraceEmitter {
    client: TraceServiceClient<Channel>,
}

impl TraceEmitter {
    /// Connect to `endpoint` (e.g. [`DEFAULT_OTLP_ENDPOINT`]). A refused or malformed endpoint
    /// surfaces as [`EmitError::Transport`].
    pub async fn connect(endpoint: impl Into<String>) -> Result<Self, EmitError> {
        let channel = Endpoint::from_shared(endpoint.into())?.connect().await?;
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
