//! The span-landing witness — operator/local only, never a CI gate.
//!
//! Gated behind `--features live-pulse` (the `live_suite.rs` precedent). It drives nothing: it reads
//! two frozen self-obs journals of same-seed `conductor run` drives (`runs/live-suite/span-a.jsonl`,
//! `span-b.jsonl`) and the live `pulse-app`'s own log, and grades whether both drives' spans LANDED
//! in Pulse's span store inside one retention window. Pulse keys that store on `(trace_id, span_id)`
//! and refuses a whole batch carrying a held pair, logging `duckdb.append` with a `reject_reason` and
//! counting `buffer.tick` `append_rejections` — so a replayed identity is visible on that surface.
//!
//! FIRING FORM, after both drives and both freezes:
//!
//! ```text
//! cargo test -q -p conductor-run --features live-pulse --test span_landing_live -- --nocapture
//! ```
//!
//! with `ANDROMEDA_PULSE_DATA_DIR` naming the live `pulse-app`'s dir. It prints ONE summary line of
//! integers — never a Pulse line, a path or a fingerprint — and panics with the same field set on a
//! FAIL. Every guard rejection keeps `capture_paths`' path-free text.

#![cfg(feature = "live-pulse")]

mod capture_paths;

use std::path::Path;

use serde_json::Value;

/// What one frozen drive journal carries: its run and its scenario emission instant.
struct Drive {
    run_id: String,
    emitted_ms: i64,
}

/// The single `run_id` a frozen journal carries and the `timestamp_ms` of its ONE
/// `timeline.execute` span-`new` line — the scenario's emission instant (the canary's `emit.batch`
/// lines precede it).
fn drive(runs: &Path, leg: &str) -> Result<Drive, String> {
    let body = std::fs::read_to_string(runs.join("live-suite").join(format!("span-{leg}.jsonl")))
        .map_err(|e| format!("span-{leg} journal unreadable: {}", e.kind()))?;
    let lines: Vec<Value> = body
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let mut run_ids: Vec<&str> = lines
        .iter()
        .filter_map(|v| v.get("run_id").and_then(Value::as_str))
        .collect();
    run_ids.sort_unstable();
    run_ids.dedup();
    let [run_id] = run_ids.as_slice() else {
        return Err(format!(
            "span-{leg} journal carries {} run_ids, not one",
            run_ids.len()
        ));
    };
    let starts: Vec<i64> = lines
        .iter()
        .filter(|v| {
            v.get("span").and_then(Value::as_str) == Some("timeline.execute")
                && v.get("span_event").and_then(Value::as_str) == Some("new")
        })
        .filter_map(|v| v.get("timestamp_ms").and_then(Value::as_i64))
        .collect();
    let [emitted_ms] = starts.as_slice() else {
        return Err(format!(
            "span-{leg} journal carries {} timeline.execute starts, not one",
            starts.len()
        ));
    };
    Ok(Drive {
        run_id: (*run_id).to_string(),
        emitted_ms: *emitted_ms,
    })
}

/// Every Pulse log line under the live data dir's `logs/` (`agent-latest.jsonl*` — a UTC date change
/// splits the log across files), each with its stamp in epoch ms, in stamp order.
fn pulse_lines() -> Result<Vec<(i64, Value)>, String> {
    const UNREADABLE: &str = "no readable agent-latest.jsonl* under ANDROMEDA_PULSE_DATA_DIR logs";
    let logs = capture_paths::pulse_logs_dir_from(
        std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").as_deref(),
    )?;
    let mut lines = Vec::new();
    for entry in std::fs::read_dir(logs)
        .map_err(|_| UNREADABLE.to_string())?
        .flatten()
    {
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with("agent-latest.jsonl")
        {
            continue;
        }
        let body = std::fs::read_to_string(entry.path()).map_err(|_| UNREADABLE.to_string())?;
        lines.extend(
            body.lines()
                .filter_map(|l| serde_json::from_str::<Value>(l).ok())
                .filter_map(|v| {
                    let ms = iso_ms(v.get("timestamp")?.as_str()?)?;
                    Some((ms, v))
                }),
        );
    }
    if lines.is_empty() {
        return Err(UNREADABLE.to_string());
    }
    lines.sort_by_key(|(ms, _)| *ms);
    Ok(lines)
}

/// `YYYY-MM-DDTHH:MM:SS[.fff…]Z` → epoch milliseconds (Pulse's log stamp; UTC). The full civil date
/// is read, so a UTC date change between the drives cannot fold the difference.
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

fn target(line: &Value) -> &str {
    line.get("target")
        .and_then(Value::as_str)
        .unwrap_or_default()
}

fn field<'a>(line: &'a Value, key: &str) -> Option<&'a Value> {
    line.get("fields")?.get(key)
}

#[test]
fn both_same_seed_drives_land_inside_one_retention_window() {
    let fail = |reason: String| -> ! { panic!("span-landing: FAIL {reason}") };
    let runs = capture_paths::runs_dir_from(
        &capture_paths::workspace_root(),
        std::env::var("CONDUCTOR_RUNS_DIR").ok().as_deref(),
    )
    .unwrap_or_else(|e| fail(e));
    let a = drive(&runs, "a").unwrap_or_else(|e| fail(e));
    let b = drive(&runs, "b").unwrap_or_else(|e| fail(e));
    let pulse = pulse_lines().unwrap_or_else(|e| fail(e));

    let reject_lines = pulse
        .iter()
        .filter(|(_, l)| target(l) == "duckdb.append" && field(l, "reject_reason").is_some())
        .count();
    let last_tick = pulse
        .iter()
        .rev()
        .find(|(_, l)| target(l) == "buffer.tick")
        .map(|(_, l)| l);
    let append_rejections = last_tick
        .and_then(|l| field(l, "append_rejections"))
        .and_then(Value::as_i64);
    let retention_seconds = last_tick
        .and_then(|l| field(l, "retention_window_seconds"))
        .and_then(Value::as_i64);
    let spans_after_b = pulse
        .iter()
        .filter(|(ms, l)| {
            *ms >= b.emitted_ms
                && target(l) == "duckdb.append"
                && field(l, "table_name").and_then(Value::as_str) == Some("spans")
        })
        .count();

    let run_ids = if a.run_id == b.run_id { 1 } else { 2 };
    let delta_ms = b.emitted_ms - a.emitted_ms;
    let retention_ms = retention_seconds.map(|s| s * 1_000);
    let summary = format!(
        "run_ids={run_ids} delta_ms={delta_ms} retention_ms={} reject_lines={reject_lines} \
         append_rejections={} spans_after_b={spans_after_b}",
        retention_ms.map_or_else(|| "absent".to_string(), |v| v.to_string()),
        append_rejections.map_or_else(|| "absent".to_string(), |v| v.to_string()),
    );
    let inside_window = retention_ms.is_some_and(|r| 0 < delta_ms && delta_ms < r);
    let pass = run_ids == 2
        && inside_window
        && reject_lines == 0
        && append_rejections == Some(0)
        && spans_after_b >= 1;
    if !pass {
        fail(summary);
    }
    // One write, leading newline: libtest's `-q` progress marks share this stdout.
    print!("\nspan-landing: PASS {summary}\n");
}
