//! Loopback gRPC integration for multi-span error traces — a capturing `TraceService` stub on an
//! ephemeral port (never `:4317`, reserved for the Epoch-4 port-occupier fault). The received
//! protobuf must carry `Status.Code=ERROR` at the expected span with well-formed parent/child
//! linkage. Determinism discipline: loopback stubs live only in tests.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use conductor_emit::{DEFAULT_SERVICE_NAME, ErrorPlacement, TraceEmitter, error_trace_request};
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
    trace_service_server::{TraceService, TraceServiceServer},
};
use opentelemetry_proto::tonic::trace::v1::status::StatusCode;
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
async fn ships_deep_child_error_trace_to_loopback_stub() {
    let (addr, captured) = start_stub().await;

    let depth = 3;
    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(error_trace_request(
            DEFAULT_SERVICE_NAME,
            42,
            ErrorPlacement::DeepChild { depth },
            "downstream failure",
        ))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");
    let spans = &received.resource_spans[0].scope_spans[0].spans;
    assert_eq!(spans.len(), depth + 1);

    // root has no parent; every span shares the one trace_id
    assert!(spans[0].parent_span_id.is_empty());
    let trace_id = &spans[0].trace_id;
    assert!(spans.iter().all(|s| &s.trace_id == trace_id));
    // parent/child linkage holds across the chain
    assert!(
        spans
            .windows(2)
            .all(|w| w[1].parent_span_id == w[0].span_id)
    );
    // the ERROR sits on the deep leaf; ancestors are OK
    let leaf = &spans[depth];
    assert_eq!(leaf.status.as_ref().unwrap().code, StatusCode::Error as i32);
    assert_eq!(leaf.status.as_ref().unwrap().message, "downstream failure");
    assert!(
        spans[..depth]
            .iter()
            .all(|s| s.status.as_ref().unwrap().code == StatusCode::Ok as i32)
    );
}

#[tokio::test(flavor = "current_thread")]
async fn ships_root_error_trace_to_loopback_stub() {
    let (addr, captured) = start_stub().await;

    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(error_trace_request(
            DEFAULT_SERVICE_NAME,
            42,
            ErrorPlacement::Root,
            "boom",
        ))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");
    let spans = &received.resource_spans[0].scope_spans[0].spans;
    assert_eq!(spans.len(), 1);
    assert!(spans[0].parent_span_id.is_empty());
    assert_eq!(
        spans[0].status.as_ref().unwrap().code,
        StatusCode::Error as i32
    );
}
