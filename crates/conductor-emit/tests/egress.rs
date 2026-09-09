//! Loopback gRPC egress tests — a tonic `TraceService` stub on an ephemeral port (never `:4317`,
//! which is reserved for the Epoch-4 port-occupier fault). Determinism discipline: loopback stubs
//! live only in tests; the live `:4317` egress is a local/operator gate, not CI.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use conductor_emit::{
    DEFAULT_CONNECT_TIMEOUT, DEFAULT_SERVICE_NAME, EmitError, TraceEmitter, probe_egress,
    trace_request,
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
    trace_service_server::{TraceService, TraceServiceServer},
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Clone, Default)]
struct CapturingService {
    last: Arc<Mutex<Option<ExportTraceServiceRequest>>>,
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

async fn start_stub() -> (SocketAddr, Arc<Mutex<Option<ExportTraceServiceRequest>>>) {
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

#[tokio::test(flavor = "current_thread")]
async fn exports_well_formed_request_to_loopback_stub() {
    let (addr, captured) = start_stub().await;

    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(trace_request(DEFAULT_SERVICE_NAME, 1, "scaffold-span"))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");
    let rs = &received.resource_spans[0];
    let attrs = &rs.resource.as_ref().unwrap().attributes;
    assert!(attrs.iter().any(|kv| kv.key == "service.name"));
    assert_eq!(rs.scope_spans[0].spans[0].name, "scaffold-span");
}

#[tokio::test(flavor = "current_thread")]
async fn refused_transport_surfaces_emit_error() {
    // Nothing listens on :1 — connect must fail as a typed EmitError, never panic.
    let result = TraceEmitter::connect("http://127.0.0.1:1").await;
    assert!(matches!(result, Err(EmitError::Transport(_))));
}

#[tokio::test(flavor = "current_thread")]
async fn probe_egress_ok_against_connectable_stub() {
    let (addr, _captured) = start_stub().await;
    probe_egress(format!("http://{addr}"))
        .await
        .expect("probe a connectable loopback stub");
}

#[tokio::test(flavor = "current_thread")]
async fn probe_egress_refused_surfaces_emit_error() {
    // Nothing listens on :1 — the liveness probe must fail as a typed EmitError, never hang or panic.
    let result = probe_egress("http://127.0.0.1:1").await;
    assert!(matches!(result, Err(EmitError::Transport(_))));
}

#[test]
fn default_connect_timeout_is_bounded_fail_fast() {
    assert!(DEFAULT_CONNECT_TIMEOUT > Duration::ZERO);
    assert!(DEFAULT_CONNECT_TIMEOUT <= Duration::from_secs(10));
}
