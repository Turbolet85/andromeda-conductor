//! Loopback gRPC integration for exception span events — a capturing `TraceService` stub on an
//! ephemeral port (never `:4317`, reserved for the Epoch-4 port-occupier fault). The received
//! protobuf must carry the OTel `exception` event (type/message/stacktrace) on an ERROR span.
//! Determinism discipline: loopback stubs live only in tests.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use conductor_emit::{
    exception_trace_request, ExceptionSpec, Frame, TraceEmitter, DEFAULT_SERVICE_NAME,
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_server::{TraceService, TraceServiceServer},
    ExportTraceServiceRequest, ExportTraceServiceResponse,
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
async fn ships_exception_event_to_loopback_stub() {
    let (addr, captured) = start_stub().await;

    let spec = ExceptionSpec::new(
        "ValueError",
        "bad input",
        vec![Frame::new("conductor::worker::handle", "src/worker.rs", 42)],
    );
    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(exception_trace_request(DEFAULT_SERVICE_NAME, 42, &spec))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");
    let span = &received.resource_spans[0].scope_spans[0].spans[0];
    assert_eq!(span.status.as_ref().unwrap().code, StatusCode::Error as i32);

    assert_eq!(span.events.len(), 1);
    let event = &span.events[0];
    assert_eq!(event.name, "exception");
    let keys: Vec<&str> = event.attributes.iter().map(|kv| kv.key.as_str()).collect();
    assert!(keys.contains(&"exception.type"));
    assert!(keys.contains(&"exception.message"));
    assert!(keys.contains(&"exception.stacktrace"));
}
