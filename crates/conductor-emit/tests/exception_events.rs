//! Loopback gRPC integration for exception span events — a capturing `TraceService` stub on an
//! ephemeral port (never `:4317`, reserved for the Epoch-4 port-occupier fault). The received
//! protobuf must carry the OTel `exception` event (type/message/stacktrace) on an ERROR span.
//! Determinism discipline: loopback stubs live only in tests.

mod common;

use common::start_stub;
use conductor_emit::{
    DEFAULT_SERVICE_NAME, ExceptionSpec, Frame, TraceEmitter, exception_trace_request,
};
use opentelemetry_proto::tonic::trace::v1::status::StatusCode;

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
