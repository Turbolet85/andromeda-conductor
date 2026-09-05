//! Wire assertions for the preflight canary storm: what Conductor actually puts on the socket.
//!
//! The canary is the payload whose fate is in question — Pulse logged nine spans received and zero
//! fingerprints tracked. Nothing proved what left this side, so these tests capture the REAL emission
//! loop (`emit_canary_storm`) against a loopback tonic `TraceService` stub on an ephemeral port —
//! never `:4317`, which is reserved for the port-occupier fault, and never a live Pulse (test-plan §2
//! Integration row + §11).
//!
//! The assertions mirror what Pulse's appender requires of each span
//! (`crates/buffer/src/appender.rs`): a non-empty `trace_id`/`span_id` (a span missing either is
//! skipped outright) and an `exception` event carrying `exception.type` / `exception.message` /
//! `exception.stacktrace`, since a `None`/empty `exception.type` yields no fingerprint at all.
//!
//! They also mirror what Pulse's span STORE requires: `spans` is `PRIMARY KEY (trace_id, span_id)`
//! (`crates/buffer/src/schema.rs`), so a repeated identity is rejected at the receiver rather than
//! stored — a loss an events-intact assertion cannot see, because every span still arrives.

mod common;

use std::collections::BTreeSet;

use common::start_trace_stub as start_stub;
use conductor_emit::{TraceEmitter, fingerprint, trace_request};
use conductor_run::{
    CANARY_SERVICE_NAME, CANARY_STORM_COUNT, canary_spec, canary_warmup_seed, emit_canary_storm,
};
use opentelemetry_proto::tonic::common::v1::any_value;
use opentelemetry_proto::tonic::trace::v1::Span;

/// Drive the real canary emission loop at the stub and return every span the stub received.
async fn captured_storm_spans(marker: &str, base: u64) -> Vec<Span> {
    let (addr, traces) = start_stub().await;
    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");

    let spec = canary_spec(marker);
    emit_canary_storm(&mut emitter, &spec, base).await.expect("emit the canary storm");

    let received = traces.lock().unwrap().clone();
    received
        .iter()
        .flat_map(|r| r.resource_spans.iter())
        .flat_map(|rs| rs.scope_spans.iter())
        .flat_map(|ss| ss.spans.iter())
        .cloned()
        .collect()
}

fn event_attr(span: &Span, key: &str) -> Option<String> {
    let event = span.events.iter().find(|e| e.name == "exception")?;
    event.attributes.iter().find(|kv| kv.key == key).and_then(|kv| {
        match kv.value.as_ref()?.value.as_ref()? {
            any_value::Value::StringValue(s) => Some(s.clone()),
            _ => None,
        }
    })
}

#[tokio::test(flavor = "current_thread")]
async fn storm_reaches_the_wire_with_its_exception_events_intact() {
    let spans = captured_storm_spans("ConductorCanary_wire", 424_242).await;

    assert_eq!(
        spans.len() as u64,
        CANARY_STORM_COUNT,
        "every canary occurrence must reach the collector"
    );

    for span in &spans {
        assert!(!span.trace_id.is_empty(), "an empty trace_id makes the receiver skip the span");
        assert!(!span.span_id.is_empty(), "an empty span_id makes the receiver skip the span");

        let names: Vec<&str> = span.events.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"exception"), "span carried no exception event: {names:?}");

        for key in ["exception.type", "exception.message", "exception.stacktrace"] {
            let value = event_attr(span, key);
            assert!(
                value.as_deref().is_some_and(|v| !v.is_empty()),
                "exception event is missing a non-empty {key}"
            );
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn storm_carries_distinct_span_identity_under_one_fingerprint() {
    let marker = "ConductorCanary_identity";
    let spans = captured_storm_spans(marker, 7_777).await;

    let span_ids: BTreeSet<Vec<u8>> = spans.iter().map(|s| s.span_id.clone()).collect();
    assert_eq!(
        span_ids.len() as u64,
        CANARY_STORM_COUNT,
        "each occurrence must carry a distinct span identity"
    );

    // Pulse fingerprints on exception.type + the normalized stacktrace, so an identical pair across
    // occurrences is what makes the storm count as ONE recurring fault rather than N unrelated ones.
    let types: BTreeSet<Option<String>> =
        spans.iter().map(|s| event_attr(s, "exception.type")).collect();
    let stacks: BTreeSet<Option<String>> =
        spans.iter().map(|s| event_attr(s, "exception.stacktrace")).collect();
    assert_eq!(types.len(), 1, "the storm must carry ONE exception.type");
    assert_eq!(stacks.len(), 1, "the storm must carry ONE stacktrace");
    assert_eq!(types.into_iter().next().flatten().as_deref(), Some(marker));

    assert_eq!(
        fingerprint(&canary_spec(marker)),
        fingerprint(&canary_spec(marker)),
        "the fingerprint is a pure function of content"
    );
}

/// The WHOLE canary emission — warm-up pre-roll and counted storm — through one collector. The two
/// legs are built by different emitters off one `base`, so only a union assertion can catch them
/// sharing an identity; a collision costs the storm occurrences it believes it sent, and every span
/// still arrives, so no events-intact check would notice.
#[tokio::test(flavor = "current_thread")]
async fn the_whole_canary_emission_carries_unique_span_identity() {
    const WARMUP_EMISSIONS: u32 = 3;
    let base = 9_001_u64;

    let (addr, traces) = start_stub().await;
    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");

    for i in 0..WARMUP_EMISSIONS {
        let seed = canary_warmup_seed(base, i);
        emitter
            .export(trace_request(CANARY_SERVICE_NAME, seed, "canary-warmup"))
            .await
            .expect("emit a warm-up span");
    }
    let spec = canary_spec("ConductorCanary_union");
    emit_canary_storm(&mut emitter, &spec, base).await.expect("emit the canary storm");

    let received = traces.lock().unwrap().clone();
    let spans: Vec<Span> = received
        .iter()
        .flat_map(|r| r.resource_spans.iter())
        .flat_map(|rs| rs.scope_spans.iter())
        .flat_map(|ss| ss.spans.iter())
        .cloned()
        .collect();

    let expected = u64::from(WARMUP_EMISSIONS) + CANARY_STORM_COUNT;
    assert_eq!(spans.len() as u64, expected, "every emitted span must reach the collector");

    for span in &spans {
        assert_eq!(span.trace_id.len(), 16, "trace_id must be 16 bytes to satisfy the receiver");
        assert_eq!(span.span_id.len(), 8, "span_id must be 8 bytes to satisfy the receiver");
    }

    let identities: BTreeSet<(Vec<u8>, Vec<u8>)> =
        spans.iter().map(|s| (s.trace_id.clone(), s.span_id.clone())).collect();
    assert_eq!(
        identities.len() as u64,
        expected,
        "warm-up and storm must never share a (trace_id, span_id) — the receiver keys on that pair"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn storm_is_attributed_to_the_canary_service() {
    let (addr, traces) = start_stub().await;
    let mut emitter = TraceEmitter::connect(format!("http://{addr}"))
        .await
        .expect("connect to loopback stub");
    emit_canary_storm(&mut emitter, &canary_spec("ConductorCanary_service"), 1)
        .await
        .expect("emit the canary storm");

    // Pulse keys the storm detector's rolling window per service name, and drops a row whose service
    // is empty (`record_occurrence` returns early) — so the resource attribute is load-bearing.
    let received = traces.lock().unwrap().clone();
    for request in &received {
        for rs in &request.resource_spans {
            let attrs = &rs.resource.as_ref().expect("resource present").attributes;
            let service = attrs
                .iter()
                .find(|kv| kv.key == "service.name")
                .and_then(|kv| match kv.value.as_ref()?.value.as_ref()? {
                    any_value::Value::StringValue(s) => Some(s.clone()),
                    _ => None,
                });
            assert_eq!(service.as_deref(), Some(CANARY_SERVICE_NAME));
        }
    }
}
