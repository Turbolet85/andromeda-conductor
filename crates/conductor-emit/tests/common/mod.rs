//! The loopback OTLP trace collector every emission test drives its egress at.
//!
//! A tonic `TraceService` stub on an ephemeral `127.0.0.1:0` — never `:4317`, which is reserved for
//! the port-occupier fault, and never a live Pulse (`.claude/rules/testing.md` 2026-06-17). Shared as
//! a MODULE, so each test binary still compiles its own copy and keeps the per-process isolation the
//! runner's per-test process model gives it (test-plan §11).
#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
    trace_service_server::{TraceService, TraceServiceServer},
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// The most recent export the stub received.
pub type LastRequest = Arc<Mutex<Option<ExportTraceServiceRequest>>>;

#[derive(Clone, Default)]
pub struct CapturingService {
    last: LastRequest,
}

#[tonic::async_trait]
impl TraceService for CapturingService {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        *self.last.lock().unwrap() = Some(request.into_inner());
        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}

pub async fn start_stub() -> (SocketAddr, LastRequest) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = CapturingService::default();
    let captured = Arc::clone(&svc.last);
    tokio::spawn(async move {
        Server::builder()
            .add_service(TraceServiceServer::new(svc))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });
    (addr, captured)
}
