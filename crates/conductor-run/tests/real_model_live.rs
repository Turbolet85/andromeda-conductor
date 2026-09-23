//! The real-model interpretation capture — operator/local only, never a CI gate.
//!
//! Gated behind `--features live-pulse` (the `live_suite.rs` precedent): a default `nextest` /
//! `clippy` / release never compiles it, and the feature declares no dependency. It reads what
//! `scripts/agent-run.{sh,ps1} run --live real-model` produced — the leg's frozen self-obs, the leg's
//! journal and Pulse's own log — re-reads the attributed incident over MCP, and PRINTS. It asserts
//! nothing beyond I/O plus one arithmetic check that runs LAST, after every line is out. The grading
//! lives in `real_model_harvest.rs`, over the printed lines pinned as literals.
//!
//! FIRING FORM — the harness fires it, never by hand. `run --live real-model` first records the rule
//! with the ignored `rule_record` below (before the leg), then runs the leg, then runs exactly
//!
//! ```text
//! cargo test -q -p conductor-run --features live-pulse --test real_model_live -- --nocapture
//! ```
//!
//! with stdout appended to `runs/live-suite/rm-capture.txt`, stderr to `rm-capture.err`, and RUST_LOG
//! removed. It inherits Conductor's own live environment: a PATH resolving `andromeda-pulse-mcp`,
//! `ANDROMEDA_PULSE_DATA_DIR` naming the live `pulse-app`'s dir, `ANDROMEDA_PULSE_MCP_ENABLED=true`,
//! deterministic L4 absent or falsy. The operator's `pulse-app` launch is the posture document's
//! (`contracts/pulse-real-model-leg-posture.md`, The launch posture).
//!
//! STOP FORM — the MCP sidecar this spawns exits when its client drops. After an interrupted capture,
//! take the census, and stop only a row absent from the pre-leg census, by PID, after reading its
//! parent and start time — never by image name. `pulse-app` is the operator's.

#![cfg(feature = "live-pulse")]

mod capture_paths;
mod real_model_common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use conductor_core::redact_value;
use conductor_emit::{ExceptionSpec, Frame, fingerprint};
use conductor_verify::{ReadbackClient, VerifyError};
use real_model_common::{mask_host_paths, rule_section};
use serde_json::{Value, json};

// The line grammar the harvest's rule reads — byte-identical to its constants, which the harvest's
// `the_capture_prints_every_token_the_rule_reads` holds this source to.
const EMISSION_INSTANT: &str = "emission_instant_ms: ";
const NO_EMISSION: &str = "emission: none";
const TRACE: &str = "trace: ";
const ATTRIBUTED: &str = "attributed: ";
const AMBIGUOUS: &str = "attribution: ambiguous";
const READ_BACK_FAILED: &str = "read-back failed: ";
const END_OF_SECTIONS: &str = "-- end of report sections --";
const INFERENCE_MODE: &str = "pulse-log inference_mode: ";
const LAUNCH_CWD: &str = "pulse-log workspace basename carries conductor: ";

const POLL_INTERVAL: Duration = Duration::from_secs(10);
const POLL_BOUND: Duration = Duration::from_secs(600);
/// Consecutive not-found ids that end one id sweep (ids are dense corpus row ids).
const SWEEP_MISSES: u32 = 3;
/// The most ids one sweep probes.
const SWEEP_BOUND: i64 = 64;

#[test]
#[ignore = "run once by `run --live real-model` before the leg fires, never by the capture invocation"]
fn rule_record() {
    let section = rule_section(include_str!("real_model_harvest.rs"))
        .expect("the harvest carries its rule markers");
    println!("rule record: the grading rule as committed before the drive");
    println!("{section}");
}

#[tokio::test(flavor = "current_thread")]
async fn capture_the_real_model_leg() {
    let flush = FlushOnDrop;
    emit("real-model capture");

    // (a) The leg's own self-obs, frozen by the harness after the leg.
    let leg = read_jsonl(&runs_dir().join("live-suite").join("rm.jsonl"), "rm.jsonl");
    let run_id = leg
        .iter()
        .find_map(|v| v.get("run_id")?.as_str().map(str::to_owned));
    emit(&format!("run_id: {}", run_id.as_deref().unwrap_or("none")));
    let leg_start_ms = leg.iter().find_map(|v| v.get("timestamp_ms")?.as_i64());
    let emission_ms = leg
        .iter()
        .find(|v| is_span_open(v, "timeline.execute"))
        .and_then(|v| v.get("timestamp_ms")?.as_i64());
    match emission_ms {
        Some(ms) => emit(&format!("{EMISSION_INSTANT}{ms}")),
        None => {
            for message in leg
                .iter()
                .filter_map(|v| v.get("message")?.as_str())
                .filter(|m| m.starts_with("preflight blocked") || m.starts_with("scenario blocked"))
            {
                emit(&format!("preflight: {message}"));
            }
            emit(NO_EMISSION);
        }
    }
    emit(&trace_summary(&leg));

    // (b) The leg's envelope, its model-written fingerprints elided.
    print_envelope(run_id.as_deref());

    // (c) Attribution by the scenario's own cue fingerprint, read by id.
    let pickup_ms = match emission_ms {
        Some(ms) => attribute(ms).await,
        None => {
            emit("attribution: none (the scenario never emitted)");
            None
        }
    };

    // (d) Pulse's own log.
    print_pulse_witnesses(leg_start_ms, emission_ms);

    // B3, last: the pickup figure (digest pickup, inference excluded) cannot be negative. Every
    // line is written before the check, so a failing check cannot truncate the capture.
    if let Some(pickup) = pickup_ms {
        emit(&format!("pickup check: pickup_ms >= 0: {}", pickup >= 0));
    }
    drop(flush);
    if let Some(pickup) = pickup_ms {
        assert!(
            pickup >= 0,
            "the attributed incident opened before the emission instant"
        );
    }
}

/// Everything the capture prints, written in ONE call when the test ends. libtest prints its own
/// progress mark for the ignored `rule_record` while this test runs, and with `--nocapture` a mark
/// written between two lines would split a grammar line; one locked write cannot interleave.
static OUTPUT: Mutex<String> = Mutex::new(String::new());

/// Writes [`OUTPUT`] on drop — on a panic too, so an unexpected fault never loses the drive's capture.
/// The leading newline puts libtest's mark on a line of its own.
struct FlushOnDrop;

impl Drop for FlushOnDrop {
    fn drop(&mut self) {
        let text = std::mem::take(&mut *OUTPUT.lock().unwrap_or_else(PoisonError::into_inner));
        print!("\n{text}");
    }
}

/// Record one line through both scrubs: `redact_value`, then the mask for what it misses.
fn emit(line: &str) {
    emit_block(&format!("{line}\n"));
}

/// Record a multi-line block through both scrubs, verbatim otherwise.
fn emit_block(text: &str) {
    OUTPUT
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .push_str(&mask_host_paths(&redact_value(text)));
}

/// The runs dir, resolved under the workspace root through the guard; a rejected handle fails with
/// a path-free reason, and `FlushOnDrop` still writes the scrubbed buffer on the unwind.
fn runs_dir() -> PathBuf {
    capture_paths::runs_dir_from(
        &capture_paths::workspace_root(),
        std::env::var("CONDUCTOR_RUNS_DIR").ok().as_deref(),
    )
    .unwrap_or_else(|reason| panic!("{reason}"))
}

/// Every JSON line of a file; an unreadable file prints its NAME and error kind, never its path.
fn read_jsonl(path: &Path, name: &str) -> Vec<Value> {
    match std::fs::read_to_string(path) {
        Ok(body) => body
            .lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect(),
        Err(e) => {
            emit(&format!("{name}: unreadable ({:?})", e.kind()));
            Vec::new()
        }
    }
}

fn is_span_open(line: &Value, span: &str) -> bool {
    line.get("span").and_then(Value::as_str) == Some(span)
        && line.get("span_event").and_then(Value::as_str) == Some("new")
}

/// The leg's trace witnesses: the distinct span names it opened, the `debug` wire-shape lines
/// (`conductor-emit` `TraceEmitter::export`), and the `retrieve_report` key-set witness line.
fn trace_summary(leg: &[Value]) -> String {
    let mut spans: Vec<&str> = leg
        .iter()
        .filter(|v| v.get("span_event").and_then(Value::as_str) == Some("new"))
        .filter_map(|v| v.get("span")?.as_str())
        .collect();
    spans.sort_unstable();
    spans.dedup();
    let wire = leg
        .iter()
        .filter(|v| {
            v.get("level")
                .and_then(Value::as_str)
                .is_some_and(|l| l.eq_ignore_ascii_case("debug"))
                && str_field(v, "message").starts_with("wire shape:")
        })
        .count();
    let retrieve_report = leg
        .iter()
        .any(|v| str_field(v, "message").contains("read-back shape: retrieve_report"));
    format!(
        "{TRACE}spans={} wire_shape_lines={wire} retrieve_report_witness={retrieve_report}",
        spans.join(",")
    )
}

fn print_envelope(run_id: Option<&str>) {
    let Some(run_id) = run_id.filter(|id| {
        !id.is_empty()
            && id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
    }) else {
        emit("envelope: none (no usable run_id)");
        return;
    };
    let journal = read_jsonl(
        &runs_dir().join(format!("{run_id}.jsonl")),
        "the leg's journal",
    );
    // The NEWEST envelope line: `seed` is envelope-only, and per-check records ride the same journal.
    let Some(mut envelope) = journal
        .into_iter()
        .rev()
        .find(|v| v.get("seed").is_some() && v.get("check_index").is_none())
    else {
        emit("envelope: none");
        return;
    };
    // Each element is a raw, model-written evidence ref (free text up to 256 chars, never scrubbed),
    // unioned over every active incident — so it is counted, and never committed.
    let (count, det) = match envelope
        .get_mut("fingerprints")
        .and_then(Value::as_array_mut)
    {
        Some(items) => {
            let det = items
                .iter()
                .filter(|f| f.as_str().is_some_and(|s| s.starts_with("det-")))
                .count();
            let count = items.len();
            for item in items.iter_mut() {
                *item = Value::from("<model-text>");
            }
            (count, det)
        }
        None => (0, 0),
    };
    emit(&format!("envelope: {envelope}"));
    emit(&format!(
        "envelope fingerprints: {count}, det- prefixed: {det}"
    ));
}

/// The scenario's cue fingerprint: the dispatcher's base exception (every `identical` occurrence is
/// the base unchanged), through Pulse's own derivation as `conductor-emit` transcribes it. The
/// dispatcher keeps its base private, so this is a transcription of
/// `conductor-run/src/dispatch.rs` `base_exception()`, held to it by the harvest's
/// `the_capture_computes_the_dispatcher_s_own_exception`.
fn scenario_cue_fingerprint() -> String {
    fingerprint(&ExceptionSpec::new(
        "ValueError",
        "conductor synthetic exception",
        vec![
            Frame::new("conductor::worker::handle", "src/worker.rs", 42),
            Frame::new("conductor::worker::parse", "src/worker.rs", 17),
        ],
    ))
}

/// One incident the polls have seen.
#[derive(Debug, Default)]
struct Candidate {
    opened_ns: Option<i64>,
    carries_scenario_fingerprint: Option<bool>,
    seen_active: bool,
}

/// Poll every 10 s for at most 600 s. An incident is attributable when its read-back carries the
/// scenario's cue fingerprint AND it opened at or after the emission instant — the fingerprint names
/// the scenario's storm, the open time excludes a prior leg's incident on a reused dir. Returns the
/// pickup figure when exactly one is attributable.
async fn attribute(emission_ms: i64) -> Option<i64> {
    let data_dir = std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").map(PathBuf::from);
    let client = match ReadbackClient::connect(data_dir).await {
        Ok(client) => client,
        Err(e) => {
            emit(&format!("{READ_BACK_FAILED}{}", reason(&e)));
            return None;
        }
    };
    let scenario_fp = scenario_cue_fingerprint();
    emit(&format!("scenario cue fingerprint: {scenario_fp}"));
    let emitted_ns = emission_ms.saturating_mul(1_000_000);

    let started = Instant::now();
    let mut candidates: BTreeMap<i64, Candidate> = BTreeMap::new();
    let mut polls = 0u32;
    let attributable = loop {
        polls += 1;
        if let Err(e) = poll_once(&client, &scenario_fp, &mut candidates).await {
            emit(&format!("{READ_BACK_FAILED}{}", reason(&e)));
            return None;
        }
        let hits: Vec<i64> = candidates
            .iter()
            .filter(|(_, c)| {
                c.carries_scenario_fingerprint == Some(true)
                    && c.opened_ns.is_some_and(|o| o >= emitted_ns)
            })
            .map(|(id, _)| *id)
            .collect();
        if !hits.is_empty() || started.elapsed() >= POLL_BOUND {
            break hits;
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    };
    emit(&format!(
        "polls: {polls} over {} s",
        started.elapsed().as_secs()
    ));
    // Identity only, never text, for every incident seen — the graded one included.
    for (id, c) in &candidates {
        emit(&format!(
            "incident: incident_id={id} opened_at_unix_nano={} carries_scenario_fingerprint={} seen_active={}",
            c.opened_ns.map_or("unknown".to_string(), |o| o.to_string()),
            c.carries_scenario_fingerprint
                .map_or("unknown".to_string(), |b| b.to_string()),
            c.seen_active
        ));
    }
    match attributable.as_slice() {
        [] => {
            emit("attribution: none within 600s");
            None
        }
        [id] => {
            let opened_ns = candidates[id].opened_ns?;
            print_attributed(&client, *id, opened_ns, emission_ms).await
        }
        _ => {
            emit(AMBIGUOUS);
            None
        }
    }
}

/// One poll: the active list, then an id sweep read BY ID — `retrieve_telemetry_slice` and
/// `retrieve_report` apply no active filter, so an incident resolved before this poll is still found.
async fn poll_once(
    client: &ReadbackClient,
    scenario_fp: &str,
    candidates: &mut BTreeMap<i64, Candidate>,
) -> Result<(), VerifyError> {
    let list = client.query_incident_list(None).await?;
    for item in list
        .get("items")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        // The live key is `incident_id`; `id` is tolerated as the shared extractor tolerates it.
        let Some(id) = item
            .get("incident_id")
            .or_else(|| item.get("id"))
            .and_then(Value::as_i64)
        else {
            continue;
        };
        let candidate = candidates.entry(id).or_default();
        candidate.seen_active = true;
        if candidate.opened_ns.is_none() {
            candidate.opened_ns = item.get("opened_at_unix_nano").and_then(Value::as_i64);
        }
    }

    let first = candidates
        .iter()
        .find(|(_, c)| c.seen_active)
        .map_or(1, |(id, _)| *id);
    let mut misses = 0;
    for id in first..first.saturating_add(SWEEP_BOUND) {
        if misses >= SWEEP_MISSES {
            break;
        }
        if candidates
            .get(&id)
            .is_some_and(|c| c.carries_scenario_fingerprint.is_some() && c.opened_ns.is_some())
        {
            misses = 0;
            continue;
        }
        let slice = match client
            .retrieve_telemetry_slice(Some(json!({ "incident_id": id })))
            .await
        {
            Ok(slice) => slice,
            Err(e) if not_found(&e) => {
                misses += 1;
                continue;
            }
            Err(e) => return Err(e),
        };
        misses = 0;
        let carries = slice
            .get("fingerprint_refs")
            .and_then(Value::as_array)
            .is_some_and(|refs| refs.iter().any(|r| r.as_str() == Some(scenario_fp)));
        let candidate = candidates.entry(id).or_default();
        candidate.carries_scenario_fingerprint = Some(carries);
        if candidate.opened_ns.is_none() {
            let report = client
                .retrieve_report(Some(json!({ "incident_id": id })))
                .await?;
            candidate.opened_ns = report
                .get("markdown")
                .and_then(Value::as_str)
                .and_then(opened_from_report);
        }
    }
    Ok(())
}

/// Read the attributed report by id and print its identity line, `## Hypotheses` and `## Evidence`.
async fn print_attributed(
    client: &ReadbackClient,
    id: i64,
    opened_ns: i64,
    emission_ms: i64,
) -> Option<i64> {
    let report = match client
        .retrieve_report(Some(json!({ "incident_id": id })))
        .await
    {
        Ok(report) => report,
        Err(e) => {
            emit(&format!("{READ_BACK_FAILED}{}", reason(&e)));
            return None;
        }
    };
    let markdown = report
        .get("markdown")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let degraded = report
        .get("degraded_mode")
        .and_then(Value::as_bool)
        .map_or("unknown".to_string(), |d| d.to_string());
    // Units: Pulse stamps nanoseconds, Conductor's self-obs milliseconds. Pulse stamps `opened_at`
    // at digest PICKUP, before inference, so this measures pickup and never formation.
    let pickup_ms = opened_ns.div_euclid(1_000_000) - emission_ms;
    emit(&format!(
        "{ATTRIBUTED}incident_id={id} opened_at_unix_nano={opened_ns} degraded_mode={degraded} pickup_ms={pickup_ms}"
    ));
    emit_block(&report_section(
        markdown,
        "## Hypotheses",
        "## Investigation Steps",
    ));
    emit_block(&report_section(
        markdown,
        "## Evidence",
        "## Project Context",
    ));
    emit(END_OF_SECTIONS);
    emit(&format!(
        "pickup: {pickup_ms} ms (Pulse opened_at, ns, less Conductor's emission instant, ms): \
         digest pickup, inference excluded"
    ));
    Some(pickup_ms)
}

/// One section of Pulse's rendered report, header line included, up to the next section's header.
fn report_section(markdown: &str, header: &str, next: &str) -> String {
    let opener = format!("{header}\n");
    let Some(start) = markdown
        .find(&format!("\n{opener}"))
        .map(|at| at + 1)
        .or_else(|| markdown.starts_with(&opener).then_some(0))
    else {
        return format!("{header}\n\n(absent from the report)\n\n");
    };
    let rest = &markdown[start..];
    let end = rest
        .find(&format!("\n{next}\n"))
        .map_or(rest.len(), |at| at + 1);
    rest[..end].to_string()
}

/// The `- **Opened (unix-nano):** N` header line of a rendered report.
fn opened_from_report(markdown: &str) -> Option<i64> {
    markdown
        .lines()
        .find_map(|l| l.strip_prefix("- **Opened (unix-nano):** "))
        .and_then(|v| v.trim().parse().ok())
}

fn not_found(e: &VerifyError) -> bool {
    matches!(e, VerifyError::JsonRpc { message, .. } if message.contains("incident not found"))
}

/// A read-back error as printable text — the server message included, scrubbed at `emit`.
fn reason(e: &VerifyError) -> String {
    match e {
        VerifyError::JsonRpc { code, message } => format!("JSON-RPC error {code}: {message}"),
        other => other.to_string(),
    }
}

/// (d) Pulse's own log: whole-file witnesses (the model mode, the launch basename, the bootstrap
/// override, uptime) and the leg window's L4 witnesses, fields only — never `message`.
fn print_pulse_witnesses(leg_start_ms: Option<i64>, emission_ms: Option<i64>) {
    let (file, lines) = match pulse_log() {
        Ok(found) => found,
        Err(reason) => {
            emit(&format!("pulse-log: none ({reason})"));
            emit(&format!("{INFERENCE_MODE}absent"));
            emit(&format!("{LAUNCH_CWD}unknown"));
            return;
        }
    };
    emit(&format!("pulse-log file: {file}"));
    let target = |v: &Value| str_field(v, "target").to_string();
    let fields_of = |v: &Value, key: &str| v.get("fields")?.get(key).map(render_value);

    let mut modes: Vec<String> = Vec::new();
    for mode in lines
        .iter()
        .filter(|v| target(v) == "interpretation.model.load")
        .filter_map(|v| fields_of(v, "inference_mode"))
    {
        if modes.last() != Some(&mode) {
            modes.push(mode);
        }
    }
    emit(&format!(
        "{INFERENCE_MODE}{}",
        if modes.is_empty() {
            "absent".to_string()
        } else {
            modes.join(",")
        }
    ));

    // The basename itself is never printed: only whether it carries `conductor` (a case-insensitive
    // substring test, stricter than the rule's whole-word one).
    let basenames: Vec<String> = lines
        .iter()
        .filter(|v| target(v) == "app.boot.workspace_key")
        .filter_map(|v| fields_of(v, "workspace_root_basename"))
        .collect();
    let carries = if basenames.is_empty() {
        "unknown"
    } else if basenames
        .iter()
        .any(|b| b.to_ascii_lowercase().contains("conductor"))
    {
        "true"
    } else {
        "false"
    };
    emit(&format!("{LAUNCH_CWD}{carries}"));

    let overrides = lines
        .iter()
        .filter(|v| target(v) == "triage.baseline.bootstrap_window.override")
        .count();
    emit(&format!(
        "pulse-log bootstrap_window.override (whole file): {overrides}"
    ));

    let stamp = |v: &Value| v.get("timestamp").and_then(Value::as_str).and_then(iso_ms);
    if let (Some(boot_ms), Some(emitted)) = (lines.first().and_then(stamp), emission_ms) {
        emit(&format!(
            "pulse-log uptime at emission: {} ms (the log's first line to Conductor's emission instant)",
            emitted - boot_ms
        ));
    }

    let window: Vec<&Value> = lines
        .iter()
        .filter(|v| match (leg_start_ms, stamp(v)) {
            (Some(start), Some(at)) => at >= start,
            (None, _) => true,
            (Some(_), None) => false,
        })
        .collect();
    emit(&format!(
        "pulse-log window: {} lines since the leg's first self-obs line",
        window.len()
    ));
    for (name, keys) in [
        (
            "interpretation.prompt.assemble",
            &["prompt_version", "token_count", "duration_ms"][..],
        ),
        (
            "interpretation.json.parse",
            &["parse_outcome", "output_bytes", "duration_ms"][..],
        ),
        (
            "interpretation.incident.created",
            &["created", "deduped", "severity", "priority_tier"][..],
        ),
        (
            "interpretation.inference.error",
            &["error_category", "recovery_action", "model_tier"][..],
        ),
        (
            "interpretation.inference.skipped",
            &["reason", "model_tier", "backoff_seconds_remaining"][..],
        ),
        (
            "triage.pattern.storm.detected",
            &["severity_hint", "occurrence_count", "fingerprint_hex"][..],
        ),
    ] {
        let hits: Vec<&&Value> = window.iter().filter(|v| target(v) == name).collect();
        emit(&format!("pulse-log {name}: {}", hits.len()));
        for hit in hits {
            let rendered: Vec<String> = keys
                .iter()
                .filter_map(|k| fields_of(hit, k).map(|value| format!("{k}={value}")))
                .collect();
            emit(&format!(
                "pulse-log   {name} t={} {}",
                hit.get("timestamp")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
                rendered.join(" ")
            ));
        }
    }
    let heartbeats = window.iter().filter(|v| target(v) == "ingest.tick").count();
    emit(&format!(
        "pulse-log heartbeat ingest.tick (15 s): {heartbeats}"
    ));

    // The digest that created the scenario's incident: the last prompt assembly before the first
    // incident creation at or after the emission instant.
    let created_at = emission_ms.and_then(|emitted| {
        window.iter().position(|v| {
            target(v) == "interpretation.incident.created"
                && fields_of(v, "created").as_deref() == Some("true")
                && stamp(v).is_some_and(|at| at >= emitted)
        })
    });
    let version = created_at.and_then(|at| {
        window[..at]
            .iter()
            .rev()
            .find(|v| target(v) == "interpretation.prompt.assemble")
            .and_then(|v| fields_of(v, "prompt_version"))
    });
    emit(&format!(
        "creating digest prompt_version: {}",
        version.as_deref().unwrap_or("unknown")
    ));
}

/// The newest `agent-latest.jsonl.*` under the live data dir's `logs/`, by modification time, with
/// its file name. The data dir is canonicalized and must be a directory before `logs/` is joined;
/// every failure is a path-free reason.
fn pulse_log() -> Result<(String, Vec<Value>), String> {
    const UNREADABLE: &str = "no readable agent-latest.jsonl.* under ANDROMEDA_PULSE_DATA_DIR logs";
    let logs = capture_paths::pulse_logs_dir_from(
        std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").as_deref(),
    )?;
    let newest = std::fs::read_dir(logs)
        .map_err(|_| UNREADABLE.to_string())?
        .flatten()
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("agent-latest.jsonl.")
        })
        .max_by_key(|e| e.metadata().and_then(|m| m.modified()).ok())
        .ok_or_else(|| UNREADABLE.to_string())?;
    let body = std::fs::read_to_string(newest.path()).map_err(|_| UNREADABLE.to_string())?;
    let lines = body
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    Ok((newest.file_name().to_string_lossy().into_owned(), lines))
}

/// A top-level string field of a JSON line, empty when absent.
fn str_field<'a>(line: &'a Value, key: &str) -> &'a str {
    line.get(key).and_then(Value::as_str).unwrap_or_default()
}

fn render_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// `YYYY-MM-DDTHH:MM:SS[.fff…]Z` → epoch milliseconds (Pulse's log stamp; UTC).
fn iso_ms(stamp: &str) -> Option<i64> {
    let num = |range: std::ops::Range<usize>| stamp.get(range)?.parse::<i64>().ok();
    let (year, month, day) = (num(0..4)?, num(5..7)?, num(8..10)?);
    let (hour, minute, second) = (num(11..13)?, num(14..16)?, num(17..19)?);
    let fraction = stamp
        .get(19..)?
        .strip_prefix('.')
        .map(|f| f.trim_end_matches('Z'))
        .unwrap_or("");
    let millis = format!("{fraction:0<3}")
        .get(..3)
        .and_then(|m| m.parse::<i64>().ok())?;
    // Days from the civil date (Howard Hinnant's algorithm), 1970-01-01 = day 0.
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some((((days * 24 + hour) * 60 + minute) * 60 + second) * 1_000 + millis)
}
