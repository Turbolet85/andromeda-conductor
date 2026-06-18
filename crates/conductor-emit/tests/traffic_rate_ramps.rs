//! Loopback traffic-rate egress test — a tonic `TraceService` stub on an ephemeral port (never
//! `:4317`, reserved for the Epoch-4 port-occupier). Asserts the shaped ramp + breathing streams
//! reach the collector with exactly the seeded per-window span totals (P-026 emission side).

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use conductor_emit::{rate_trace_request, RateCurve, TraceEmitter};
use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_server::{TraceService, TraceServiceServer},
    ExportTraceServiceRequest, ExportTraceServiceResponse,
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

fn received_span_count(captured: &Arc<Mutex<Option<ExportTraceServiceRequest>>>) -> usize {
    captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request")
        .resource_spans[0]
        .scope_spans[0]
        .spans
        .len()
}

#[tokio::test(flavor = "current_thread")]
async fn exports_ramp_and_breathing_totals_to_loopback_stub() {
    let (addr, captured) = start_stub().await;
    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");

    let seed = 424_242;

    let ramp = RateCurve::ramp(5, 60, 24).unwrap();
    let ramp_total: usize = ramp.window_counts(seed).iter().map(|&c| c as usize).sum();
    emitter
        .export(rate_trace_request("conductor", seed, &ramp, "tick"))
        .await
        .expect("export ramp to loopback stub");
    assert_eq!(received_span_count(&captured), ramp_total);

    let breathing = RateCurve::breathing(40, 20, 8, 48).unwrap();
    let breathing_total: usize = breathing.window_counts(seed).iter().map(|&c| c as usize).sum();
    emitter
        .export(rate_trace_request("conductor", seed, &breathing, "tick"))
        .await
        .expect("export breathing to loopback stub");
    assert_eq!(received_span_count(&captured), breathing_total);
}
