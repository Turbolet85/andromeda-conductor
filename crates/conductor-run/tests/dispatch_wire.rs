//! Per-family wire assertions for the emission dispatcher: what a phase DECLARES is what reaches
//! the collector.
//!
//! Captures against loopback tonic `TraceService` / `LogsService` stubs on ephemeral ports — never
//! `:4317`, which is reserved for the port-occupier fault, and never a live Pulse (test-plan §2
//! Integration row + §11). Each test declares one shape, drives the real
//! `run_timeline_with` + `Dispatcher` path under `start_paused`, and asserts the emitted stream.

mod common;

use common::start_stub;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;

use conductor_core::{
    EmissionShape, EmissionSpec, FingerprintVariantSpec, PId, PhaseSpec, PiiCategorySpec, Scenario,
    Signal, SloTier,
};
use conductor_run::Dispatcher;
use conductor_timeline::{PhaseTimeline, run_timeline_with};
use opentelemetry_proto::tonic::common::v1::any_value;
use opentelemetry_proto::tonic::trace::v1::status::StatusCode;

fn scenario(phase: PhaseSpec) -> Scenario {
    Scenario {
        name: "dispatch-fixture".to_string(),
        p_ids: vec![PId("P-005".to_string())],
        seed: 4242,
        slo_tier: SloTier::Tier5s,
        phases: vec![phase],
        jitter_ms: 0,
        expected: Vec::new(),
        checklist: Vec::new(),
    }
}

fn phase(gap_ms: u64, emission: EmissionSpec) -> PhaseSpec {
    PhaseSpec {
        name: "under-test".to_string(),
        gap_ms,
        emission,
        fault: None,
    }
}

/// Spans across every captured trace request.
fn all_spans(
    reqs: &[ExportTraceServiceRequest],
) -> Vec<opentelemetry_proto::tonic::trace::v1::Span> {
    reqs.iter()
        .flat_map(|r| r.resource_spans.iter())
        .flat_map(|rs| rs.scope_spans.iter())
        .flat_map(|ss| ss.spans.iter().cloned())
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// One dispatched span reduced to identity, linkage and outcome. Every `*_time_unix_nano` field is
/// deliberately absent: those come from `SystemTime::now()`, so the seed governs what is here and
/// the clock governs what is not (test-plan §7; obs-plan §5).
#[derive(Debug)]
#[allow(dead_code)] // read by insta's Debug rendering, never by test code
struct SpanShape {
    batch: usize,
    name: String,
    trace_id: String,
    span_id: String,
    parent_span_id: String,
    status: i32,
    events: Vec<String>,
    attribute_keys: Vec<String>,
}

/// The captured trace stream as an ordered shape-projection — the golden's subject.
fn span_shapes(reqs: &[ExportTraceServiceRequest]) -> Vec<SpanShape> {
    reqs.iter()
        .enumerate()
        .flat_map(|(batch, req)| {
            all_spans(std::slice::from_ref(req))
                .into_iter()
                .map(move |s| SpanShape {
                    batch,
                    name: s.name.clone(),
                    trace_id: hex(&s.trace_id),
                    span_id: hex(&s.span_id),
                    parent_span_id: hex(&s.parent_span_id),
                    status: s.status.as_ref().map_or(0, |st| st.code),
                    events: s.events.iter().map(|e| e.name.clone()).collect(),
                    attribute_keys: s.attributes.iter().map(|kv| kv.key.clone()).collect(),
                })
        })
        .collect()
}

/// The committed `fingerprint-storm` fixture — the same run `conductor-timeline`'s pacing golden
/// freezes one altitude below.
fn storm_fixture() -> Scenario {
    let toml = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/fingerprint-storm.toml"
    ))
    .expect("fixture readable");
    Scenario::from_toml_str(&toml).expect("fixture valid")
}

fn service_names(reqs: &[ExportTraceServiceRequest]) -> Vec<String> {
    reqs.iter()
        .flat_map(|r| r.resource_spans.iter())
        .filter_map(|rs| rs.resource.as_ref())
        .flat_map(|res| res.attributes.iter())
        .filter(|kv| kv.key == "service.name")
        .filter_map(|kv| kv.value.as_ref())
        .filter_map(|v| v.value.as_ref())
        .filter_map(|v| match v {
            any_value::Value::StringValue(s) => Some(s.clone()),
            _ => None,
        })
        .collect()
}

macro_rules! drive_scenario {
    ($scenario:expr) => {{
        let (addr, traces, logs) = start_stub().await;
        let endpoint = format!("http://{}", addr);
        let scenario = $scenario;
        let timeline = PhaseTimeline::from(&scenario);
        let mut dispatcher = Dispatcher::connect(&scenario, &endpoint)
            .await
            .expect("connect stub");
        run_timeline_with(&timeline, scenario.seed, async |point| {
            dispatcher.dispatch(point).await
        })
        .await
        .expect("timeline drives the dispatcher");
        let t = traces.lock().unwrap().clone();
        let l = logs.lock().unwrap().clone();
        (t, l)
    }};
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn a_declared_occurrence_count_is_what_reaches_the_wire() {
    let (traces, _) = drive_scenario!(scenario(phase(
        4000,
        EmissionSpec::shaped(Signal::Traces, 7, EmissionShape::Plain)
    )));
    assert_eq!(traces.len(), 7, "one emission per declared occurrence");
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn a_zero_occurrence_phase_is_a_real_silence_window() {
    let (traces, logs) = drive_scenario!(scenario(phase(
        30_000,
        EmissionSpec::shaped(Signal::Traces, 0, EmissionShape::Plain)
    )));
    assert!(traces.is_empty(), "a silence phase emits nothing");
    assert!(logs.is_empty());
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_error_family_realizes_its_declared_percentage_and_depth() {
    let (traces, _) = drive_scenario!(scenario(phase(
        4000,
        EmissionSpec::shaped(
            Signal::Traces,
            10,
            EmissionShape::Error {
                depth: 2,
                error_percent: 30
            },
        )
    )));
    assert_eq!(traces.len(), 10);
    let errored: Vec<&ExportTraceServiceRequest> = traces
        .iter()
        .filter(|r| {
            all_spans(std::slice::from_ref(*r)).iter().any(|s| {
                s.status
                    .as_ref()
                    .is_some_and(|st| st.code == StatusCode::Error as i32)
            })
        })
        .collect();
    assert_eq!(errored.len(), 3, "30% of 10 emissions carry the error");
    // depth 2 => a root -> child -> child chain, the leaf carrying ERROR.
    let deepest = all_spans(std::slice::from_ref(errored[0])).len();
    assert_eq!(deepest, 3, "depth 2 yields a three-span chain");
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_exception_family_cycles_its_declared_variant_mix() {
    let (traces, _) = drive_scenario!(scenario(phase(
        6000,
        EmissionSpec::shaped(
            Signal::Traces,
            6,
            EmissionShape::Exception {
                variants: vec![
                    FingerprintVariantSpec::Identical,
                    FingerprintVariantSpec::Path,
                    FingerprintVariantSpec::Line,
                ],
            },
        )
    )));
    assert_eq!(traces.len(), 6);
    let spans = all_spans(&traces);
    assert_eq!(spans.len(), 6);
    assert!(
        spans
            .iter()
            .all(|s| s.events.iter().any(|e| e.name == "exception")),
        "every emission carries an exception event"
    );
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_severity_family_reaches_the_logs_collector_with_its_declared_numbers() {
    let (traces, logs) = drive_scenario!(scenario(phase(
        4000,
        EmissionSpec::shaped(
            Signal::Logs,
            4,
            EmissionShape::Severity {
                severities: vec![16, 17]
            },
        )
    )));
    assert!(
        traces.is_empty(),
        "a logs-shaped phase opens no trace batch"
    );
    assert_eq!(logs.len(), 4);
    let numbers: Vec<i32> = logs
        .iter()
        .flat_map(|r| r.resource_logs.iter())
        .flat_map(|rl| rl.scope_logs.iter())
        .flat_map(|sl| sl.log_records.iter())
        .map(|lr| lr.severity_number)
        .collect();
    assert_eq!(
        numbers,
        vec![16, 17, 16, 17],
        "the declared mix cycles across occurrences"
    );
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_latency_family_emits_its_declared_sample_count() {
    let (traces, _) = drive_scenario!(scenario(phase(
        4000,
        EmissionSpec::shaped(
            Signal::Traces,
            2,
            EmissionShape::Latency {
                operation: "checkout".to_string(),
                p50_ms: 100,
                p95_ms: 400,
                p99_ms: 800,
                samples: 60,
            },
        )
    )));
    assert_eq!(traces.len(), 2, "one batch per occurrence");
    let spans = all_spans(&traces);
    assert_eq!(
        spans.len(),
        120,
        "each batch carries its declared sample count"
    );
    assert!(spans.iter().all(|s| s.name == "checkout"));
    assert!(
        spans
            .iter()
            .all(|s| s.end_time_unix_nano >= s.start_time_unix_nano)
    );
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_declared_latency_percentiles_shape_the_emitted_durations() {
    let profile = |p50, p95, p99| {
        scenario(phase(
            2000,
            EmissionSpec::shaped(
                Signal::Traces,
                1,
                EmissionShape::Latency {
                    operation: "checkout".to_string(),
                    p50_ms: p50,
                    p95_ms: p95,
                    p99_ms: p99,
                    samples: 200,
                },
            ),
        ))
    };
    let median_ms = |reqs: &[ExportTraceServiceRequest]| {
        let mut d: Vec<u64> = all_spans(reqs)
            .iter()
            .map(|s| (s.end_time_unix_nano - s.start_time_unix_nano) / 1_000_000)
            .collect();
        d.sort_unstable();
        d[d.len() / 2]
    };

    let (fast, _) = drive_scenario!(profile(10, 30, 60));
    let (slow, _) = drive_scenario!(profile(1000, 3000, 6000));
    assert!(
        median_ms(&fast) < median_ms(&slow),
        "the declared profile drives the realized durations: {} vs {}",
        median_ms(&fast),
        median_ms(&slow)
    );
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_pii_family_embeds_every_declared_category() {
    let (traces, _) = drive_scenario!(scenario(phase(
        4000,
        EmissionSpec::shaped(
            Signal::Traces,
            1,
            EmissionShape::Pii {
                categories: vec![PiiCategorySpec::Email, PiiCategorySpec::Ssn],
            },
        )
    )));
    let keys: Vec<String> = all_spans(&traces)
        .iter()
        .flat_map(|s| {
            s.attributes
                .iter()
                .map(|kv| kv.key.clone())
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(keys.iter().any(|k| k == "user.email"), "{keys:?}");
    assert!(keys.iter().any(|k| k == "user.ssn"), "{keys:?}");
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_rate_family_emits_the_curve_total() {
    let (traces, _) = drive_scenario!(scenario(phase(
        4000,
        EmissionSpec::shaped(
            Signal::Traces,
            1,
            EmissionShape::Ramp {
                from_rate: 5,
                to_rate: 20,
                windows: 10
            },
        )
    )));
    assert_eq!(traces.len(), 1);
    let spans = all_spans(&traces);
    assert!(
        spans.len() > 10,
        "the ramp emits one span per window count, got {}",
        spans.len()
    );
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_topology_family_emits_every_declared_service() {
    let (traces, _) = drive_scenario!(scenario(phase(
        4000,
        EmissionSpec::shaped(
            Signal::Traces,
            1,
            EmissionShape::Topology {
                services: vec!["checkout-api".to_string(), "payments-worker".to_string()],
                error_depth: Some(1),
            },
        )
    )));
    let names = service_names(&traces);
    assert!(names.iter().any(|n| n == "checkout-api"), "{names:?}");
    assert!(names.iter().any(|n| n == "payments-worker"), "{names:?}");
    let spans = all_spans(&traces);
    assert!(
        spans.iter().any(|s| s
            .status
            .as_ref()
            .is_some_and(|st| st.code == StatusCode::Error as i32)),
        "the declared error depth places one ERROR span"
    );
}

/// Whether the declared `error_depth` lands the ERROR span at the trace ROOT (no parent) or on a
/// deeper child. The root-vs-deep split is P-008's whole subject, and the depth's ZERO case is what
/// selects it — so the placement, not merely the presence of an error, is what has to be asserted.
async fn topology_error_is_at_the_root(error_depth: u32) -> bool {
    let (traces, _) = drive_scenario!(scenario(phase(
        4000,
        EmissionSpec::shaped(
            Signal::Traces,
            1,
            EmissionShape::Topology {
                services: vec!["checkout-api".to_string(), "payments-worker".to_string()],
                error_depth: Some(error_depth),
            },
        )
    )));
    let spans = all_spans(&traces);
    let errored: Vec<_> = spans
        .iter()
        .filter(|s| {
            s.status
                .as_ref()
                .is_some_and(|st| st.code == StatusCode::Error as i32)
        })
        .collect();
    assert_eq!(
        errored.len(),
        1,
        "the declared depth places exactly one ERROR span"
    );
    errored[0].parent_span_id.is_empty()
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn a_zero_error_depth_places_the_error_at_the_root_and_a_deeper_one_does_not() {
    assert!(
        topology_error_is_at_the_root(0).await,
        "depth 0 is the ROOT placement — the P-008 root-cause half"
    );
    assert!(
        !topology_error_is_at_the_root(1).await,
        "a non-zero depth places the error on a DEEP CHILD, never the root"
    );
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_same_seed_reproduces_the_same_stream() {
    let shape = || {
        scenario(phase(
            4000,
            EmissionSpec::shaped(
                Signal::Traces,
                5,
                EmissionShape::Exception {
                    variants: vec![FingerprintVariantSpec::Identical],
                },
            ),
        ))
    };
    let ids = |reqs: &[ExportTraceServiceRequest]| {
        all_spans(reqs)
            .iter()
            .map(|s| (s.trace_id.clone(), s.span_id.clone()))
            .collect::<Vec<_>>()
    };
    let (a, _) = drive_scenario!(shape());
    let (b, _) = drive_scenario!(shape());
    assert_eq!(
        ids(&a),
        ids(&b),
        "same scenario + seed => identical stream identity"
    );

    let mut other = shape();
    other.seed = 999;
    let (c, _) = drive_scenario!(other);
    assert_ne!(ids(&a), ids(&c), "a different seed diverges");
}

/// Freeze what the dispatcher actually puts on the wire for the committed storm fixture. The test
/// above proves the stream is stable WITHIN a run; this is the on-disk tripwire that catches a
/// shape→primitive regression ACROSS commits, which no in-run comparison can see.
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_committed_storm_fixture_stream_is_frozen() {
    let scenario = storm_fixture();
    let (traces, logs) = drive_scenario!(scenario);
    assert!(
        logs.is_empty(),
        "an exception-shaped fixture opens no logs batch"
    );
    insta::assert_debug_snapshot!("storm_stream_seed_4317017", span_shapes(&traces));
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_committed_storm_fixture_stream_is_frozen_at_an_alternate_seed() {
    let mut scenario = storm_fixture();
    scenario.seed = 7;
    let (traces, _) = drive_scenario!(scenario);
    insta::assert_debug_snapshot!("storm_stream_seed_7", span_shapes(&traces));
}

/// The committed `error-baseline-spike` fixture — the Error shape's stream golden, covering the
/// majority-plain slots (`dispatch.rs`'s non-error branch) that only occurrence-count tests
/// asserted before the constant-span-identity collision showed what that class of coverage misses.
fn error_baseline_fixture() -> Scenario {
    let toml = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/error-baseline-spike.toml"
    ))
    .expect("fixture readable");
    Scenario::from_toml_str(&toml).expect("fixture valid")
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn the_committed_error_baseline_fixture_stream_is_frozen() {
    let scenario = error_baseline_fixture();
    let (traces, logs) = drive_scenario!(scenario);
    assert!(
        logs.is_empty(),
        "an error-shaped fixture opens no logs batch"
    );
    insta::assert_debug_snapshot!("error_baseline_stream_seed_424242", span_shapes(&traces));
}
