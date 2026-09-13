//! Loopback gRPC integration for multi-service topology traces — a capturing `TraceService` stub on
//! an ephemeral port (never `:4317`, reserved for the Epoch-4 port-occupier fault). The received
//! protobuf must carry ≥2 distinct `service.name` ResourceSpans joined by one shared `trace_id` with
//! cross-service parent/child linkage. Determinism discipline: loopback stubs live only in tests.

mod common;

use common::start_stub;
use conductor_emit::{
    EmitError, ErrorPlacement, ServiceTopology, TraceEmitter, service_topology_request,
};
use opentelemetry_proto::tonic::common::v1::any_value;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, status::StatusCode};

fn service_name(rs: &ResourceSpans) -> String {
    let kv = rs
        .resource
        .as_ref()
        .unwrap()
        .attributes
        .iter()
        .find(|kv| kv.key == "service.name")
        .unwrap();
    match kv.value.as_ref().unwrap().value.as_ref().unwrap() {
        any_value::Value::StringValue(s) => s.clone(),
        _ => panic!("service.name not a string"),
    }
}

#[tokio::test(flavor = "current_thread")]
async fn ships_multi_service_topology_to_loopback_stub() {
    let (addr, captured) = start_stub().await;

    let topology = ServiceTopology::new(&["gateway", "api", "db"]).expect("≥2 distinct services");
    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emitter
        .export(service_topology_request(
            &topology,
            42,
            Some(ErrorPlacement::DeepChild { depth: 2 }),
            "downstream failure",
        ))
        .await
        .expect("export to loopback stub");

    let received = captured
        .lock()
        .unwrap()
        .clone()
        .expect("stub received a request");

    // ≥2 distinct service.name ResourceSpans, in topology order
    let names: Vec<String> = received.resource_spans.iter().map(service_name).collect();
    assert_eq!(names, vec!["gateway", "api", "db"]);

    // one span per service, in order
    let spans: Vec<_> = received
        .resource_spans
        .iter()
        .map(|rs| &rs.scope_spans[0].spans[0])
        .collect();
    assert_eq!(spans.len(), 3);

    // one shared trace_id; root unparented; cross-service parent linkage across the chain
    let trace_id = &spans[0].trace_id;
    assert!(spans.iter().all(|s| &s.trace_id == trace_id));
    assert!(spans[0].parent_span_id.is_empty());
    assert!(
        spans
            .windows(2)
            .all(|w| w[1].parent_span_id == w[0].span_id)
    );

    // the deep error sits on the downstream service; upstream services are OK
    let leaf = spans[2].status.as_ref().unwrap();
    assert_eq!(leaf.code, StatusCode::Error as i32);
    assert_eq!(leaf.message, "downstream failure");
    assert!(
        spans[..2]
            .iter()
            .all(|s| s.status.as_ref().unwrap().code == StatusCode::Ok as i32)
    );
}

#[tokio::test(flavor = "current_thread")]
async fn refused_transport_surfaces_emit_error() {
    // Nothing listens on :1 — connect must fail as a typed EmitError, never panic.
    let result = TraceEmitter::connect("http://127.0.0.1:1").await;
    assert!(matches!(result, Err(EmitError::Transport(_))));
}
