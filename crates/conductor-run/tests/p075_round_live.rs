//! The P-075 round's live leg — operator/local only, never a CI gate.
//!
//! Gated behind `--features live-pulse` (the `lifecycle_live.rs` precedent): a default `nextest` /
//! `clippy` / release never compiles it. It PRODUCES the capture `lifecycle_harvest.rs` grades for
//! round-request assertions 1 (read-back content fidelity), 2 (runtime-state fidelity) and 7 (the
//! incident's lifecycle events read back around the resolve); it grades nothing itself.
//!
//! FIRING FORM — first leg on a FRESH `pulse-app` launch (an incident the storm dedupes into never
//! gains the storm's fingerprint, so the active set must be empty at open), the whole env block in
//! one paste:
//!
//! ```text
//! PATH="<the Pulse checkout>/target/release:$PATH" \
//! ANDROMEDA_PULSE_DATA_DIR=<the live Pulse's dir> \
//! ANDROMEDA_PULSE_MCP_ENABLED=true \
//! ANDROMEDA_PULSE_L4_DETERMINISTIC=true \
//!   cargo test -q -p conductor-run --features live-pulse --test p075_round_live -- --nocapture
//! ```
//!
//! A round's plan resolves the checkout at fire time (`realpath ../andromeda-pulse/target/release`),
//! so no host path is ever authored into the command.
//!
//! STDOUT is the committed capture: one buffered block of `p075-round:` lines carrying integers,
//! booleans and closed words only. The emitted fingerprint is compared IN-PROCESS and only the
//! membership boolean leaves the process — no hex, title, path or workspace key reaches stdout. Event
//! stamps leave only as offsets inside the resolve call's window, never as raw epoch nanoseconds.
//! Raw wire values go to STDERR, which the firing form sends to the gitignored `runs/`.
#![cfg(feature = "live-pulse")]

use std::time::Duration;

use conductor_emit::{
    DEFAULT_OTLP_ENDPOINT, ExceptionSpec, TraceEmitter, exception_trace_request, fingerprint,
};
use conductor_run::{
    CANARY_SERVICE_NAME, LifecycleVerdict, ResolveWindow, attribute_by_liveness, canary_spec,
    canary_storm_seed, emit_canary_storm, probe_resolve_lifecycle_timed,
};
use conductor_verify::ReadbackClient;
use serde_json::{Value, json};

/// Poll ticks while the incident forms: storm → `created:true` took ~110 s on the 2026-09-01 leg,
/// so 40 × 10 s leaves room for L3's digest cadence with L4 behind it.
const FORM_TICKS: u64 = 40;
const TICK: Duration = Duration::from_secs(10);

fn now_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}

/// `(incident_id, opened_at_unix_nano)` for every active item. The live key is `incident_id`;
/// `id` is accepted too (testing.md 2026-09-01: an item-key mismatch reads as an empty corpus).
async fn active_items(client: &ReadbackClient) -> Vec<(i64, Option<i64>)> {
    let list = client
        .query_incident_list(None)
        .await
        .expect("query_incident_list");
    eprintln!("LEG raw query_incident_list -> {list}");
    list.get("items")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|i| {
                    let id = i
                        .get("incident_id")
                        .or_else(|| i.get("id"))
                        .and_then(Value::as_i64)?;
                    Some((id, i.get("opened_at_unix_nano").and_then(Value::as_i64)))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// One emission of the storm's identity — refreshes the incident's idle clock without re-storming.
async fn keep_alive(traces: &mut TraceEmitter, spec: &ExceptionSpec, tick: u64) {
    let _ = traces
        .export(exception_trace_request(
            CANARY_SERVICE_NAME,
            canary_storm_seed(now_nanos() as u64, tick),
            spec,
        ))
        .await;
}

fn verdict_name(verdict: &LifecycleVerdict) -> &'static str {
    match verdict {
        LifecycleVerdict::Proven { .. } => "Proven",
        LifecycleVerdict::ProvenByLiveness { .. } => "ProvenByLiveness",
        LifecycleVerdict::Unattributable => "Unattributable",
        LifecycleVerdict::StillActive => "StillActive",
        LifecycleVerdict::NoControl => "NoControl",
    }
}

fn word(value: Option<bool>) -> String {
    value.map_or_else(|| "absent".to_string(), |v| v.to_string())
}

fn count(value: Option<i64>) -> String {
    value.map_or_else(|| "absent".to_string(), |v| v.to_string())
}

/// Pulse's closed event vocabulary at S2, plus `unknown` (what it renders for any other stored value)
/// and `other` for anything outside both — so no SUT-supplied string reaches stdout verbatim.
fn event_word(kind: &str) -> &'static str {
    match kind {
        "created" => "created",
        "active" => "active",
        "acknowledged" => "acknowledged",
        "resolved" => "resolved",
        "unknown" => "unknown",
        _ => "other",
    }
}

/// One `retrieve_incident_events` read, reduced in-process to what the capture may carry.
struct EventsRead {
    total: Option<i64>,
    truncated: Option<bool>,
    words: Vec<&'static str>,
    last_occurred: Option<i64>,
}

impl EventsRead {
    fn first(&self) -> &'static str {
        self.words.first().copied().unwrap_or("none")
    }

    fn last(&self) -> &'static str {
        self.words.last().copied().unwrap_or("none")
    }

    fn sequence(&self) -> String {
        if self.words.is_empty() {
            "none".to_string()
        } else {
            self.words.join(",")
        }
    }

    fn outside_vocabulary(&self) -> usize {
        self.words
            .iter()
            .filter(|w| matches!(**w, "unknown" | "other"))
            .count()
    }
}

async fn read_events(client: &ReadbackClient, id: i64) -> Option<EventsRead> {
    match client.retrieve_incident_events(id).await {
        Ok(result) => {
            eprintln!("LEG raw retrieve_incident_events -> {result}");
            let events: &[Value] = result
                .get("events")
                .and_then(Value::as_array)
                .map_or(&[], Vec::as_slice);
            Some(EventsRead {
                total: result.get("total").and_then(Value::as_i64),
                truncated: result.get("truncated").and_then(Value::as_bool),
                words: events
                    .iter()
                    .map(|e| event_word(e.get("event_kind").and_then(Value::as_str).unwrap_or("")))
                    .collect(),
                last_occurred: events
                    .last()
                    .and_then(|e| e.get("occurred_unix_nano"))
                    .and_then(Value::as_i64),
            })
        }
        Err(e) => {
            eprintln!("LEG retrieve_incident_events failed: {e}");
            None
        }
    }
}

fn before_line(read: Option<&EventsRead>) -> String {
    match read {
        Some(r) => format!(
            "p075-round: before_total={} before_truncated={} before_first={} before_has_resolved={} \
             before_sequence={} before_outside_vocabulary={}\n",
            count(r.total),
            word(r.truncated),
            r.first(),
            r.words.contains(&"resolved"),
            r.sequence(),
            r.outside_vocabulary()
        ),
        None => "p075-round: before_total=absent before_truncated=absent before_first=absent \
                 before_has_resolved=absent before_sequence=absent before_outside_vocabulary=absent\n"
            .to_string(),
    }
}

/// The after line. The three `_ns` values are window-RELATIVE, so a raw epoch stamp never leaves the
/// process; the offsets are read only when the last event is the `resolved` the call should append.
fn after_line(read: Option<&EventsRead>, window: Option<ResolveWindow>) -> String {
    let span = window.map(|w| w.received_unix_nanos - w.sent_unix_nanos);
    let resolved_at = read
        .filter(|r| r.last() == "resolved")
        .and_then(|r| r.last_occurred);
    let (minus_sent, received_minus) = match (resolved_at, window) {
        (Some(at), Some(w)) => (
            Some(at - w.sent_unix_nanos),
            Some(w.received_unix_nanos - at),
        ),
        _ => (None, None),
    };
    let tail = format!(
        "resolve_window_ns={} resolved_minus_sent_ns={} received_minus_resolved_ns={}",
        count(span),
        count(minus_sent),
        count(received_minus)
    );
    match read {
        Some(r) => format!(
            "p075-round: after_total={} after_truncated={} after_first={} after_last={} \
             after_sequence={} after_outside_vocabulary={} {tail}\n",
            count(r.total),
            word(r.truncated),
            r.first(),
            r.last(),
            r.sequence(),
            r.outside_vocabulary()
        ),
        None => format!(
            "p075-round: after_total=absent after_truncated=absent after_first=absent \
             after_last=absent after_sequence=absent after_outside_vocabulary=absent {tail}\n"
        ),
    }
}

#[tokio::test(flavor = "current_thread")]
async fn p075_round_live_leg() {
    let data_dir = std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").map(std::path::PathBuf::from);
    let client = ReadbackClient::connect(data_dir)
        .await
        .expect("live sidecar connects");
    let mut traces = TraceEmitter::connect(DEFAULT_OTLP_ENDPOINT)
        .await
        .expect("OTLP egress");

    let mut out = String::from("\n");
    let active_at_open = active_items(&client).await.len();
    out.push_str(&format!("p075-round: active_at_open={active_at_open}\n"));

    // The emission instant is stamped BEFORE the storm, so an incident opened after it cannot be a
    // leftover of anything earlier.
    let emitted = now_nanos();
    let spec = canary_spec(&format!("ConductorP075_{}", emitted / 1_000_000));
    let expected = fingerprint(&spec);
    emit_canary_storm(&mut traces, &spec, emitted as u64)
        .await
        .expect("storm emits");

    let mut formed: Option<(i64, i64)> = None;
    for tick in 1..=FORM_TICKS {
        formed = active_items(&client)
            .await
            .into_iter()
            .filter_map(|(id, opened)| opened.map(|o| (id, o)))
            .find(|(_, opened)| *opened > emitted);
        if formed.is_some() {
            break;
        }
        keep_alive(&mut traces, &spec, tick).await;
        tokio::time::sleep(TICK).await;
    }

    let Some((id, opened)) = formed else {
        out.push_str(
            "p075-round: incident_id=none opened_after_emission=absent opened_minus_emitted_ms=absent\n\
             p075-round: fingerprint_refs=0 det_members=0 fingerprint_in_refs=absent\n\
             p075-round: degraded_mode=absent\n",
        );
        out.push_str(&before_line(None));
        out.push_str(
            "p075-round: resolve_before=0 resolved_left_active_set=absent idle_ms=absent verdict=absent\n",
        );
        out.push_str(&after_line(None, None));
        out.push_str("p075-round: end\n");
        print!("{out}");
        return;
    };
    out.push_str(&format!(
        "p075-round: incident_id={id} opened_after_emission=true opened_minus_emitted_ms={}\n",
        (opened - emitted) / 1_000_000
    ));

    let args = Some(json!({ "incident_id": id }));
    let (refs, members, in_refs) = match client.retrieve_telemetry_slice(args.clone()).await {
        Ok(slice) => {
            eprintln!("LEG raw retrieve_telemetry_slice -> {slice}");
            let refs: Vec<String> = slice
                .get("fingerprint_refs")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(str::to_owned))
                        .collect()
                })
                .unwrap_or_default();
            let members = refs.iter().filter(|r| r.starts_with("det-")).count();
            let in_refs = refs.contains(&expected);
            (refs.len(), members, Some(in_refs))
        }
        Err(e) => {
            eprintln!("LEG retrieve_telemetry_slice failed: {e}");
            (0, 0, None)
        }
    };
    out.push_str(&format!(
        "p075-round: fingerprint_refs={refs} det_members={members} fingerprint_in_refs={}\n",
        word(in_refs)
    ));

    let degraded = match client.retrieve_report(args).await {
        Ok(report) => {
            eprintln!(
                "LEG retrieve_report keys -> {:?}",
                report.as_object().map(|o| o.keys().collect::<Vec<_>>())
            );
            report.get("degraded_mode").and_then(Value::as_bool)
        }
        Err(e) => {
            eprintln!("LEG retrieve_report failed: {e}");
            None
        }
    };
    out.push_str(&format!("p075-round: degraded_mode={}\n", word(degraded)));

    let before = read_events(&client, id).await;
    out.push_str(&before_line(before.as_ref()));

    // LIVENESS: refresh the identity, stamp the instant, resolve at once. Pulse auto-resolves only an
    // incident idle for 120 s, so a removal seconds after the last emission is Conductor's write.
    keep_alive(&mut traces, &spec, FORM_TICKS + 1).await;
    let freshened = std::time::Instant::now();
    let probed = probe_resolve_lifecycle_timed(&client, id).await;
    let idle = freshened.elapsed();
    // Read IMMEDIATELY: a deterministic re-emission before the app's 60 s reconcile can append
    // `active` after `resolved`, so nothing may sit between the resolve and this read.
    let after = read_events(&client, id).await;
    let window = match probed {
        Ok((observation, window)) => {
            let verdict = attribute_by_liveness(&observation, idle.as_secs_f64());
            out.push_str(&format!(
                "p075-round: resolve_before={} resolved_left_active_set={} idle_ms={} verdict={}\n",
                observation.before.len(),
                !observation.after.contains(&id),
                idle.as_millis(),
                verdict_name(&verdict)
            ));
            Some(window)
        }
        Err(e) => {
            eprintln!("LEG mark_incident_resolved failed: {e}");
            out.push_str(
                "p075-round: resolve_before=0 resolved_left_active_set=absent idle_ms=absent verdict=absent\n",
            );
            None
        }
    };
    out.push_str(&after_line(after.as_ref(), window));
    out.push_str("p075-round: end\n");
    // One write: libtest's `-q` progress marks share this stdout (testing.md 2026-09-23).
    print!("{out}");
}
