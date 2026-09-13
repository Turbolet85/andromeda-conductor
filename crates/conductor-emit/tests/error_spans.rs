//! Loopback gRPC integration for multi-span error traces — a capturing `TraceService` stub on an
//! ephemeral port (never `:4317`, reserved for the Epoch-4 port-occupier fault). The received
//! protobuf must carry `Status.Code=ERROR` at the expected span with well-formed parent/child
//! linkage. Determinism discipline: loopback stubs live only in tests.

mod common;

use common::start_stub;
use conductor_emit::{DEFAULT_SERVICE_NAME, ErrorPlacement, TraceEmitter, error_trace_request};
use opentelemetry_proto::tonic::trace::v1::status::StatusCode;

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
