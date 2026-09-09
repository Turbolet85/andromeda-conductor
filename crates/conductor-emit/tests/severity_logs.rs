//! Loopback gRPC logs-egress tests — a tonic `LogsService` stub on an ephemeral port (never `:4317`,
//! which is reserved for the Epoch-4 port-occupier fault). Mirrors `egress.rs` for the logs path:
//! the `SeverityNumber` 17-boundary records survive the round-trip, and a refused transport is a
//! typed `EmitError` (P-007).

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use conductor_emit::{
    DEFAULT_SERVICE_NAME, EmitError, LogsEmitter, Severity, severity_logs_request,
};
use opentelemetry_proto::tonic::collector::logs::v1::{
    ExportLogsServiceRequest, ExportLogsServiceResponse,
    logs_service_server::{LogsService, LogsServiceServer},
};
use opentelemetry_proto::tonic::logs::v1::SeverityNumber;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Clone, Default)]
struct CapturingService {
    last: Arc<Mutex<Option<ExportLogsServiceRequest>>>,
}

#[tonic::async_trait]
impl LogsService for CapturingService {
    async fn export(
        &self,
        request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        *self.last.lock().unwrap() = Some(request.into_inner());
        Ok(Response::new(ExportLogsServiceResponse::default()))
    }
}

async fn start_stub() -> (SocketAddr, Arc<Mutex<Option<ExportLogsServiceRequest>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = CapturingService::default();
    let captured = Arc::clone(&svc.last);
    tokio::spawn(async move {
        Server::builder()
            .add_service(LogsServiceServer::new(svc))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });
    (addr, captured)
}

#[tokio::test(flavor = "current_thread")]
async fn exports_boundary_records_to_loopback_stub() {
    let (addr, captured) = start_stub().await;

    let warn = Severity::new(16).expect("16 is a valid severity");
    let error = Severity::new(17).expect("17 is a valid severity");

    let mut emitter = LogsEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(severity_logs_request(DEFAULT_SERVICE_NAME, &[warn, error]))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");
    let rl = &received.resource_logs[0];
    let attrs = &rl.resource.as_ref().unwrap().attributes;
    assert!(attrs.iter().any(|kv| kv.key == "service.name"));

    let records = &rl.scope_logs[0].log_records;
    assert_eq!(records.len(), 2);
    // the two records straddle the 17-boundary and stay distinguishable across the round-trip
    assert_eq!(records[0].severity_number, SeverityNumber::Warn4 as i32);
    assert_eq!(records[0].severity_text, "WARN4");
    assert_eq!(records[1].severity_number, SeverityNumber::Error as i32);
    assert_eq!(records[1].severity_text, "ERROR");
    assert_ne!(records[0].severity_number, records[1].severity_number);
}

#[tokio::test(flavor = "current_thread")]
async fn refused_transport_surfaces_emit_error() {
    // Nothing listens on :1 — connect must fail as a typed EmitError, never panic.
    let result = LogsEmitter::connect("http://127.0.0.1:1").await;
    assert!(matches!(result, Err(EmitError::Transport(_))));
}
