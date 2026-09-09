//! Loopback latency-shaping egress test — a tonic `TraceService` stub on an ephemeral port (never
//! `:4317`, reserved for the Epoch-4 port-occupier). Asserts seeded per-operation durations reach
//! the collector and distinct operations keep distinct profiles end-to-end; a refused transport is
//! a typed `EmitError`.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use conductor_emit::{EmitError, LatencyOp, LatencyProfile, TraceEmitter, latency_trace_request};
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
    trace_service_server::{TraceService, TraceServiceServer},
};
use opentelemetry_proto::tonic::trace::v1::Span;
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

fn median_duration(spans: &[Span], name: &str) -> u64 {
    let mut ds: Vec<u64> = spans
        .iter()
        .filter(|s| s.name == name)
        .map(|s| s.end_time_unix_nano - s.start_time_unix_nano)
        .collect();
    ds.sort_unstable();
    ds[ds.len() / 2]
}

#[tokio::test(flavor = "current_thread")]
async fn exports_per_operation_latency_to_loopback_stub() {
    let (addr, captured) = start_stub().await;

    let fast = LatencyProfile::new(10, 30, 60).unwrap();
    let slow = LatencyProfile::new(1000, 3000, 6000).unwrap();
    let ops = [
        LatencyOp {
            operation: "cache_get",
            profile: fast,
            samples: 60,
        },
        LatencyOp {
            operation: "db_query",
            profile: slow,
            samples: 60,
        },
    ];

    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(latency_trace_request("conductor", 424_242, &ops))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");
    let spans = &received.resource_spans[0].scope_spans[0].spans;
    assert_eq!(spans.iter().filter(|s| s.name == "cache_get").count(), 60);
    assert_eq!(spans.iter().filter(|s| s.name == "db_query").count(), 60);
    assert!(median_duration(spans, "cache_get") < median_duration(spans, "db_query"));
}

#[tokio::test(flavor = "current_thread")]
async fn refused_transport_surfaces_emit_error() {
    let result = TraceEmitter::connect("http://127.0.0.1:1").await;
    assert!(matches!(result, Err(EmitError::Transport(_))));
}
