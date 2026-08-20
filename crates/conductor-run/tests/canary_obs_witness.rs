//! The wire-shape witness, alone in its own test binary.
//!
//! `init_observability` installs a PROCESS-GLOBAL subscriber (first install wins), so a per-test temp
//! file cannot isolate it: any sibling test emitting `emit.batch` lines concurrently lands them in
//! whichever file won the install. Measured on the shared binary — the test passed alone and under
//! `--test-threads=1`, and failed only alongside its four `emit_canary_storm` siblings. `cargo test`
//! gives each `tests/*.rs` its own process and nextest is process-per-test, so a private binary is the
//! isolation both runners honour.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use conductor_core::{ObsSink, init_observability};
use conductor_emit::TraceEmitter;
use conductor_run::{CANARY_STORM_COUNT, canary_spec, emit_canary_storm};
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
    trace_service_server::{TraceService, TraceServiceServer},
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// The `run_id` this test stamps on its own self-obs stream — the discriminator the count filters on.
const WITNESS_RUN_ID: &str = "canary-witness";

type Traces = Arc<Mutex<Vec<ExportTraceServiceRequest>>>;

#[derive(Clone, Default)]
struct Capture {
    traces: Traces,
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

async fn start_stub() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let capture = Capture::default();
    tokio::spawn(async move {
        Server::builder()
            .add_service(TraceServiceServer::new(capture))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .ok();
    });
    addr
}

/// The witness exists to answer a question a zero could not. If it were dropped by the field
/// allowlist or filtered out by level, it would reproduce that exact failure one layer down — so it is
/// read back out of a real self-observation artifact this test produced, never asserted in isolation.
#[tokio::test(flavor = "current_thread")]
async fn the_wire_shape_witness_reaches_the_self_obs_artifact() {
    // The leading `info` is load-bearing: a bare `conductor_emit=debug` directive filters every OTHER
    // target out, silencing the rest of the self-obs stream. This is the filter the live-leg recipe
    // prescribes, so the test proves the configuration an operator will actually run.
    //
    // SAFETY: this test owns its process under both runners — nextest is process-per-test and this is
    // the only test in this binary, so nothing else is reading the environment concurrently, which is
    // the race edition 2024 makes `set_var` unsafe for. The write also precedes the subscriber install.
    unsafe { std::env::set_var("RUST_LOG", "info,conductor_emit=debug") };

    let dir = assert_fs::TempDir::new().expect("temp dir");
    let log = dir.path().join("obs.jsonl");
    init_observability("conductor", Some(WITNESS_RUN_ID.to_string()), ObsSink::File(log.clone()));

    let addr = start_stub().await;
    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emit_canary_storm(&mut emitter, &canary_spec("ConductorCanary_witness"), 11)
        .await
        .expect("emit the canary storm");

    let contents = std::fs::read_to_string(&log).expect("self-obs artifact written");
    // Scoped by this test's own `run_id`: the subscriber is process-global, so the count must not be
    // perturbable by any line a future sibling in this binary emits.
    let witness: Vec<&str> = contents
        .lines()
        .filter(|l| l.contains("wire shape:") && l.contains(WITNESS_RUN_ID))
        .collect();

    assert_eq!(
        witness.len() as u64,
        CANARY_STORM_COUNT,
        "expected one witness line per exported batch, got:\n{contents}"
    );

    for line in &witness {
        for field in [
            "timestamp_ms",
            "level",
            "target",
            "service.name",
            "service.version",
            "deployment.environment",
            "run_id",
        ] {
            assert!(
                line.contains(&format!("\"{field}\"")),
                "witness line is missing the self-obs base field {field}: {line}"
            );
        }
        assert!(line.contains("spans=1"), "{line}");
        assert!(line.contains("spans_missing_ids=0"), "{line}");
        assert!(line.contains("event_names=[exception]"), "{line}");
        assert!(
            line.contains("event_attr_keys=[exception.message,exception.stacktrace,exception.type]"),
            "the witness must name every key the receiver extracts: {line}"
        );
    }
}
