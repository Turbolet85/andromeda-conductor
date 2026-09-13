//! Loopback latency-shaping egress test — a tonic `TraceService` stub on an ephemeral port (never
//! `:4317`, reserved for the Epoch-4 port-occupier). Asserts seeded per-operation durations reach
//! the collector and distinct operations keep distinct profiles end-to-end; a refused transport is
//! a typed `EmitError`.

mod common;

use common::start_stub;
use conductor_emit::{EmitError, LatencyOp, LatencyProfile, TraceEmitter, latency_trace_request};
use opentelemetry_proto::tonic::trace::v1::Span;

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
