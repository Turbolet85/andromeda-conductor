//! Loopback traffic-rate egress test — a tonic `TraceService` stub on an ephemeral port (never
//! `:4317`, reserved for the Epoch-4 port-occupier). Asserts the shaped ramp + breathing streams
//! reach the collector with exactly the seeded per-window span totals (P-026 emission side).

mod common;

use common::{LastRequest, start_stub};
use conductor_emit::{RateCurve, TraceEmitter, rate_trace_request};

fn received_span_count(captured: &LastRequest) -> usize {
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
    let breathing_total: usize = breathing
        .window_counts(seed)
        .iter()
        .map(|&c| c as usize)
        .sum();
    emitter
        .export(rate_trace_request("conductor", seed, &breathing, "tick"))
        .await
        .expect("export breathing to loopback stub");
    assert_eq!(received_span_count(&captured), breathing_total);
}
