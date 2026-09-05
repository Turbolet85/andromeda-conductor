//! The loopback OTLP collector every wire test drives its emission at.
//!
//! A tonic `TraceService` + `LogsService` stub on an ephemeral `127.0.0.1:0` — never `:4317`, which
//! is reserved for the port-occupier fault, and never a live Pulse (`.claude/rules/testing.md`
//! 2026-06-17). Shared as a MODULE, so each test binary still compiles its own copy and keeps the
//! per-process isolation the self-obs subscriber needs (test-plan §11).
#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use opentelemetry_proto::tonic::collector::logs::v1::{
    ExportLogsServiceRequest, ExportLogsServiceResponse,
    logs_service_server::{LogsService, LogsServiceServer},
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
    trace_service_server::{TraceService, TraceServiceServer},
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub type Traces = Arc<Mutex<Vec<ExportTraceServiceRequest>>>;
pub type Logs = Arc<Mutex<Vec<ExportLogsServiceRequest>>>;

/// What the stub received, in arrival order.
#[derive(Clone, Default)]
pub struct Capture {
    pub traces: Traces,
    pub logs: Logs,
}

#[tonic::async_trait]
impl TraceService for Capture {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        self.traces.lock().unwrap().push(request.into_inner());
        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}

#[tonic::async_trait]
impl LogsService for Capture {
    async fn export(
        &self,
        request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        self.logs.lock().unwrap().push(request.into_inner());
        Ok(Response::new(ExportLogsServiceResponse::default()))
    }
}

/// Start the stub and return its address plus both capture handles.
pub async fn start_stub() -> (SocketAddr, Traces, Logs) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let capture = Capture::default();
    let (traces, logs) = (Arc::clone(&capture.traces), Arc::clone(&capture.logs));
    tokio::spawn(async move {
        Server::builder()
            .add_service(TraceServiceServer::new(capture.clone()))
            .add_service(LogsServiceServer::new(capture))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .ok();
    });
    (addr, traces, logs)
}

/// Start the stub for a trace-only test, discarding the logs handle.
pub async fn start_trace_stub() -> (SocketAddr, Traces) {
    let (addr, traces, _logs) = start_stub().await;
    (addr, traces)
}
