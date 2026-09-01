//! The live leg that mints the resolve-lifecycle capture — operator/local only, never a CI gate.
//!
//! Gated behind `--features live-pulse` (the `stub-server` precedent, test-plan §5 Drivers): a
//! default `nextest` / `clippy` / release never compiles it, so a host with no Pulse cannot red the
//! suite. The feature declares no dependency, so it adds nothing to `Cargo.lock`.
//!
//! Firing form — the whole env block in ONE paste, because a partial set fails SILENTLY as a ~0s
//! `[BLOCKED]` that reads exactly like a genuine SUT-side gate failure:
//!
//! ```text
//! PATH=/d/dev/projects/andromeda-pulse/target/release:$PATH \
//! ANDROMEDA_PULSE_DATA_DIR=<the live Pulse's dir> \
//! ANDROMEDA_PULSE_MCP_ENABLED=true \
//! ANDROMEDA_PULSE_L4_DETERMINISTIC=true \
//!   cargo test -p conductor-run --features live-pulse --test lifecycle_live -- --nocapture
//! ```
//!
//! TWO measured facts shape the pacing, both from this chunk's first leg attempt.
//!
//!   * INCIDENT FORMATION TOOK ~110s from storm to `created:true` — L3's digest cadence with L4
//!     behind it. A 120s poll budget landed its last sample ~4s before the incident appeared and
//!     reported an empty active set for a pipeline that had worked perfectly (12 spans ingested,
//!     zero `reject_reason`, `storms_detected_total: 2`, `severity_hint: "autonomous"`). Absence at
//!     the end of a short window is not absence.
//!   * THE CONTROL MUST BE KEPT ALIVE. Pulse auto-resolves an idle incident (120s idle, 30s resolver
//!     tick), and A idles the moment its storm stops — so waiting for B would let A resolve itself
//!     and destroy the control. A's identity is therefore re-emitted while B forms. That is not
//!     gaming the assertion: a control surviving BECAUSE it still has traffic is exactly what
//!     separates "Conductor's write removed B" from "the auto-resolver swept the set".
//!
//! It PRINTS its observation rather than asserting a pinned expectation: the capture is the
//! deliverable, and the grading lives in `lifecycle_harvest.rs` over the frozen lines.
#![cfg(feature = "live-pulse")]

use std::time::Duration;

use conductor_emit::{DEFAULT_OTLP_ENDPOINT, ExceptionSpec, TraceEmitter, exception_trace_request};
use conductor_run::{
    CANARY_SERVICE_NAME, attribute_by_liveness, canary_spec, canary_storm_seed, emit_canary_storm,
    probe_resolve_lifecycle,
};
use conductor_verify::ReadbackClient;

fn now_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}

async fn active_ids(client: &ReadbackClient) -> Vec<i64> {
    let list = client.query_incident_list(None).await.expect("query_incident_list");
    // RAW dump: a reader that extracts nothing is indistinguishable from an empty corpus unless the
    // wire value itself is on the record.
    println!("LEG raw query_incident_list -> {list}");
    list.get("items")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|i| i.get("incident_id").or_else(|| i.get("id")).and_then(|v| v.as_i64()))
                .collect()
        })
        .unwrap_or_default()
}

/// One keep-alive emission of an existing identity — enough to refresh the incident's idle clock
/// without re-storming.
async fn keep_alive(traces: &mut TraceEmitter, spec: &ExceptionSpec, tick: u64) {
    let _ = traces
        .export(exception_trace_request(CANARY_SERVICE_NAME, canary_storm_seed(now_nanos() as u64, tick), spec))
        .await;
}

/// Poll the active set until it reaches `want`, refreshing `alive` (when given) each tick so an
/// already-formed control does not idle out while a second incident is still forming.
async fn poll_until(
    client: &ReadbackClient,
    traces: &mut TraceEmitter,
    alive: Option<&ExceptionSpec>,
    want: usize,
    ticks: u64,
    label: &str,
) -> Vec<i64> {
    let mut seen = active_ids(client).await;
    for tick in 1..=ticks {
        if seen.len() >= want {
            break;
        }
        if let Some(spec) = alive {
            keep_alive(traces, spec, tick).await;
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
        seen = active_ids(client).await;
        println!("LEG {label} t+{}s: active={seen:?}", tick * 10);
    }
    seen
}

#[tokio::test(flavor = "current_thread")]
async fn resolve_lifecycle_liveness_attributed_leg() {
    let data_dir = std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").map(std::path::PathBuf::from);
    let client = ReadbackClient::connect(data_dir).await.expect("live sidecar connects");
    let mut traces = TraceEmitter::connect(DEFAULT_OTLP_ENDPOINT).await.expect("OTLP egress");

    println!("LEG active-set at open: {:?}", active_ids(&client).await);

    let marker = format!("ConductorLifecycle_{}", now_nanos());
    let spec = canary_spec(&marker);
    emit_canary_storm(&mut traces, &spec, now_nanos() as u64).await.expect("storm emits");
    println!("LEG storm emitted: {marker}");

    // Keep the identity fresh each tick: a refreshed incident cannot be idle, which is the whole
    // attribution mechanism below.
    let before = poll_until(&client, &mut traces, Some(&spec), 1, 40, "form").await;
    println!("LEG active-set before resolve: {before:?}");

    if before.is_empty() {
        println!("LEG OUTCOME: no incident formed within the window — nothing to resolve, recorded as measured");
        return;
    }

    // LIVENESS ATTRIBUTION. Pulse auto-resolves only an IDLE incident (120s idle, 30s resolver
    // tick), so an incident refreshed moments ago cannot be swept by the resolver. Emit once more,
    // stamp the instant, and resolve immediately: if the id leaves the active set while its last
    // emission is seconds old, the auto-resolver is excluded by construction and Conductor's write
    // is the only remaining cause. This replaces the two-incident control, which is UNATTAINABLE —
    // Pulse dedupes a new incident against any OPEN one regardless of fingerprint, so a second
    // concurrent incident cannot be formed at all (measured this leg: `created=false deduped=true`).
    keep_alive(&mut traces, &spec, 999).await;
    let freshened_at = std::time::Instant::now();
    let resolve = *before.last().expect("a non-empty active set");

    let observation =
        probe_resolve_lifecycle(&client, resolve).await.expect("the lifecycle write round-trips");
    let idle_at_resolve = freshened_at.elapsed().as_secs_f64();

    println!("LEG resolved={resolve}");
    println!("LEG observation: {observation:?}");
    println!("LEG idle_seconds_at_resolve={idle_at_resolve:.1} (auto-resolve needs 120s idle)");
    println!("LEG verdict: {:?}", attribute_by_liveness(&observation, idle_at_resolve));

    assert!(!observation.before.is_empty(), "the leg observed an active set to write against");
    if !observation.after.contains(&resolve) && idle_at_resolve < 120.0 {
        println!(
            "LEG PROVEN-BY-LIVENESS: incident {resolve} left the active set {idle_at_resolve:.1}s \
             after its last emission — far inside the 120s idle window the auto-resolver requires, \
             so the removal is attributable to Conductor's mark_incident_resolved write"
        );
    }
}
