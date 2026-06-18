//! Loopback gRPC integration for the PII payload corpus — capturing `TraceService` + `LogsService`
//! stubs on ephemeral ports (never `:4317`, reserved for the Epoch-4 port-occupier fault). Asserts
//! each of the seven P-047 categories reaches the wire: as span attributes + an `exception` event on
//! the trace path, and in log-record body + attributes on the logs path (P-035 / P-047 / P-048).

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use conductor_emit::{
    pii_logs_request, pii_trace_request, EmitError, LogsEmitter, PiiCategory, PiiCorpus,
    TraceEmitter, DEFAULT_SERVICE_NAME,
};
use opentelemetry_proto::tonic::collector::logs::v1::{
    logs_service_server::{LogsService, LogsServiceServer},
    ExportLogsServiceRequest, ExportLogsServiceResponse,
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_server::{TraceService, TraceServiceServer},
    ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use opentelemetry_proto::tonic::common::v1::{any_value, KeyValue};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Clone, Default)]
struct CapturingTraceService {
    last: Arc<Mutex<Option<ExportTraceServiceRequest>>>,
}

#[tonic::async_trait]
impl TraceService for CapturingTraceService {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        *self.last.lock().unwrap() = Some(request.into_inner());
        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}

#[derive(Clone, Default)]
struct CapturingLogsService {
    last: Arc<Mutex<Option<ExportLogsServiceRequest>>>,
}

#[tonic::async_trait]
impl LogsService for CapturingLogsService {
    async fn export(
        &self,
        request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        *self.last.lock().unwrap() = Some(request.into_inner());
        Ok(Response::new(ExportLogsServiceResponse::default()))
    }
}

async fn start_trace_stub() -> (SocketAddr, Arc<Mutex<Option<ExportTraceServiceRequest>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = CapturingTraceService::default();
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

async fn start_logs_stub() -> (SocketAddr, Arc<Mutex<Option<ExportLogsServiceRequest>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = CapturingLogsService::default();
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

fn string_value(kv: &KeyValue) -> String {
    match kv.value.as_ref().and_then(|v| v.value.as_ref()) {
        Some(any_value::Value::StringValue(s)) => s.clone(),
        _ => String::new(),
    }
}

#[tokio::test(flavor = "current_thread")]
async fn ships_pii_corpus_across_spans_and_exceptions() {
    let (addr, captured) = start_trace_stub().await;
    let corpus = PiiCorpus::seeded(42);
    let categories = PiiCategory::all();

    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(pii_trace_request(DEFAULT_SERVICE_NAME, &corpus, &categories))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");
    let spans = &received.resource_spans[0].scope_spans[0].spans;
    assert_eq!(spans.len(), 2);

    // every string value reachable on the trace wire: span attributes + exception-event attributes
    let mut wire: Vec<String> = Vec::new();
    for span in spans {
        wire.extend(span.attributes.iter().map(string_value));
        for event in &span.events {
            wire.extend(event.attributes.iter().map(string_value));
        }
    }
    for category in categories {
        let value = corpus.value(category);
        assert!(
            wire.iter().any(|w| w.contains(value)),
            "trace wire missing {category:?}: {value}"
        );
    }
}

#[tokio::test(flavor = "current_thread")]
async fn ships_pii_corpus_through_logs() {
    let (addr, captured) = start_logs_stub().await;
    let corpus = PiiCorpus::seeded(42);
    let categories = PiiCategory::all();

    let mut emitter = LogsEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(pii_logs_request(DEFAULT_SERVICE_NAME, &corpus, &categories))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");
    let records = &received.resource_logs[0].scope_logs[0].log_records;
    assert_eq!(records.len(), 7);

    // every string value reachable on the logs wire: record body + record attributes
    let mut wire: Vec<String> = Vec::new();
    for record in records {
        if let Some(any_value::Value::StringValue(s)) =
            record.body.as_ref().and_then(|b| b.value.as_ref())
        {
            wire.push(s.clone());
        }
        wire.extend(record.attributes.iter().map(string_value));
    }
    for category in categories {
        let value = corpus.value(category);
        assert!(
            wire.iter().any(|w| w.contains(value)),
            "logs wire missing {category:?}: {value}"
        );
    }
}

#[tokio::test(flavor = "current_thread")]
async fn refused_transport_surfaces_emit_error() {
    // Nothing listens on :1 — connect must fail as a typed EmitError, never panic.
    assert!(matches!(
        TraceEmitter::connect("http://127.0.0.1:1").await,
        Err(EmitError::Transport(_))
    ));
    assert!(matches!(
        LogsEmitter::connect("http://127.0.0.1:1").await,
        Err(EmitError::Transport(_))
    ));
}
