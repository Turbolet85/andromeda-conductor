//! The delegated-timing harvest's grading helpers and its synthetic families, as CHILD modules of
//! `delegated_timing_harvest.rs`. A `tests/` subdirectory module, so it is never a test target of its
//! own; the root reaches these helpers through `pub(crate)`, the children through `use super::*;`.
#![allow(dead_code)]

mod p025_hard_grade;
mod synthetic_bounds;
mod window_selection;

use std::path::Path;

/// One delegated bound: which Pulse target carries it, which field holds the duration, and the
/// budget in milliseconds. The field is part of the identity, not a detail — see the module doc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DelegatedBound {
    pub(crate) p_id: &'static str,
    pub(crate) target: &'static str,
    pub(crate) field: &'static str,
    pub(crate) budget_ms: f64,
}

/// The four bounds Pulse delegated, as measured at SUT HEAD `f0c38f5`.
pub(crate) fn bounds() -> [DelegatedBound; 4] {
    [
        DelegatedBound {
            p_id: "P-025",
            target: "metric.constellation.hue_update_ms",
            field: "duration_ms",
            budget_ms: 2_000.0,
        },
        DelegatedBound {
            p_id: "P-027",
            target: "metric.constellation.discovery_ms",
            field: "duration_ms",
            budget_ms: 5_000.0,
        },
        DelegatedBound {
            p_id: "P-037",
            target: "metric.report.render_ms",
            field: "value",
            budget_ms: 2_000.0,
        },
        DelegatedBound {
            p_id: "P-045",
            target: "metric.findings.counter_refresh_ms",
            field: "duration_ms",
            budget_ms: 1_000.0,
        },
    ]
}

pub(crate) fn harvest_since(path: &Path, skip_lines: usize) -> std::io::Result<Vec<String>> {
    let body = std::fs::read_to_string(path)?;
    Ok(body.lines().skip(skip_lines).map(str::to_owned).collect())
}

pub(crate) fn is_target(line: &str, target: &str) -> bool {
    line.contains(&format!("\"target\":\"{target}\""))
}

pub(crate) fn num_field(line: &str, key: &str) -> Option<f64> {
    let tail = line.split(&format!("\"{key}\":")).nth(1)?;
    let end = tail.find([',', '}'])?;
    tail[..end].trim().parse().ok()
}

/// Every observation of one bound in the capture. A line that is not this target, or that lacks the
/// bound's own duration field, is SKIPPED rather than fatal — a harvest reads another process's
/// output, so the verdict/error wall applies to evidence too.
pub(crate) fn observations(lines: &[String], bound: &DelegatedBound) -> Vec<f64> {
    lines
        .iter()
        .filter(|line| is_target(line, bound.target))
        .filter_map(|line| num_field(line, bound.field))
        .collect()
}

/// Grade one bound over a capture, returning the WORST observation on success.
///
/// The worst sample is what a budget means: a bound claiming "≤2s" is broken by any observation
/// above it, so grading the first or the mean would let a slow render hide behind a fast one. No
/// observation at all is an error — never a pass.
pub(crate) fn grade(lines: &[String], bound: &DelegatedBound) -> Result<f64, String> {
    let seen = observations(lines, bound);
    let worst = seen
        .iter()
        .copied()
        .fold(None::<f64>, |acc, v| Some(acc.map_or(v, |a| a.max(v))))
        .ok_or_else(|| {
            format!(
                "{}: no observation of `{}` carrying `{}` in the capture — the bound is UNGRADED, not met",
                bound.p_id, bound.target, bound.field
            )
        })?;
    if worst > bound.budget_ms {
        return Err(format!(
            "{}: worst `{}` was {worst}ms, over its {}ms budget",
            bound.p_id, bound.target, bound.budget_ms
        ));
    }
    Ok(worst)
}

/// Pulse's per-service lifecycle heartbeat target — the ONLY steady-state writer of
/// `ServiceRegistryEntry.last_seen_unix_nano`, which made it the RETIRED P-025 instrument's mechanism
/// witness at Pulse HEAD `83d4060`. It is off the hue path at `226554a`.
pub(crate) const LIFECYCLE_TICK_TARGET: &str = "triage.lifecycle.tick";

/// How far a RETIRED-instrument hue sample's reported duration may sit from its offset to the
/// preceding tick (Pulse HEAD `83d4060`).
///
/// The UI polls `services.list_with_states` every 1000ms (`use-service-constellation.ts:22`) and the
/// sample is recorded over TauRPC after that poll observes the refreshed value, so the reported age
/// trails the tick by up to one poll plus IPC.
pub(crate) const TICK_OFFSET_TOLERANCE_MS: f64 = 1_100.0;

/// Pulse's backend witness of an incident opening. `created=true` is a fresh incident, whose
/// `opened_at` is the instant a service's tier RISES; `created=false` is a dedupe and opens nothing.
pub(crate) const INCIDENT_CREATED_TARGET: &str = "interpretation.incident.created";

/// How far a rise sample's start instant (`timestamp − duration_ms`) may sit from its incident's
/// creation line — the tolerance Pulse's own leg used (`xtask/src/hue_shift.rs:41`,
/// `ANCHOR_TOLERANCE_MS`, Pulse HEAD `226554a`).
pub(crate) const P025_ANCHOR_TOLERANCE_MS: f64 = 1_000.0;

/// Milliseconds since the Unix epoch for a capture line's `timestamp` field, or `None` when the line
/// carries no parseable stamp — a harvest reads another process's output, so a malformed line is
/// skipped rather than fatal (the same posture `observations` takes).
pub(crate) fn timestamp_ms(line: &str) -> Option<i64> {
    let stamp = line.split("\"timestamp\":\"").nth(1)?.split('"').next()?;
    let (date, rest) = stamp.split_once('T')?;
    let time = rest.strip_suffix('Z')?;
    let mut d = date.split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let (ss, frac) = match t.next()?.split_once('.') {
        Some((s, f)) => {
            let mut millis = f.to_owned();
            millis.truncate(3);
            while millis.len() < 3 {
                millis.push('0');
            }
            (s.parse::<i64>().ok()?, millis.parse::<i64>().ok()?)
        }
        None => (time.rsplit(':').next()?.parse::<i64>().ok()?, 0),
    };
    let days = days_from_civil(y, mo, day);
    Some((days * 86_400 + hh * 3_600 + mm * 60 + ss) * 1_000 + frac)
}

/// Howard Hinnant's `days_from_civil` — days since 1970-01-01 from a proleptic-Gregorian date.
/// Transcribed rather than pulled in as a dependency: the harvest needs one date conversion.
pub(crate) fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// One hue observation with the instant it was logged.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct HueSample {
    pub(crate) duration_ms: f64,
    pub(crate) at_ms: i64,
}

/// The hue samples inside `[start_ms, end_ms]`, the window that attributes them to the scenario.
///
/// The leaf carries NO service identifier (Pulse's allowlist for it is exactly
/// `duration_ms` + `severity_tier`, "Aggregate-only"), and `severity_tier` cannot discriminate either
/// — a dot's tier is incident-derived and incidents form only from Autonomous cues, so the scenario
/// reaches the same band the canary does. Attribution is therefore TEMPORAL: the window opens at the
/// scenario's own sustained phase, by which time preflight readiness has already required (and
/// observed) the canary's incident, so the canary's flip and its sample precede `scenario.run`.
pub(crate) fn hue_samples_in_window(
    lines: &[String],
    bound: &DelegatedBound,
    window: (i64, i64),
) -> Vec<HueSample> {
    lines
        .iter()
        .filter(|line| is_target(line, bound.target))
        .filter_map(|line| {
            Some(HueSample {
                duration_ms: num_field(line, bound.field)?,
                at_ms: timestamp_ms(line)?,
            })
        })
        .filter(|s| s.at_ms >= window.0 && s.at_ms <= window.1)
        .collect()
}

/// Grade one bound over only the capture lines stamped inside `[start_ms, end_ms]` — the same
/// selection `hue_samples_in_window` makes, graded by the same worst-observation, inclusive,
/// absence-is-UNGRADED `grade()` the other three bounds use. The reason names the window, so an
/// UNGRADED leg says which window held nothing.
pub(crate) fn grade_in_window(
    lines: &[String],
    bound: &DelegatedBound,
    window: (i64, i64),
) -> Result<f64, String> {
    let in_window: Vec<String> = lines
        .iter()
        .filter(|line| timestamp_ms(line).is_some_and(|t| t >= window.0 && t <= window.1))
        .cloned()
        .collect();
    grade(&in_window, bound)
        .map_err(|reason| format!("{reason} (leg window {}..={})", window.0, window.1))
}

/// The in-window hue samples whose NEW tier is not `none` — the rises, which an incident opening
/// anchors. A fall to no incident carries `severity_tier: "none"` and anchors to a resolution.
pub(crate) fn tiered_hue_samples_in_window(
    lines: &[String],
    bound: &DelegatedBound,
    window: (i64, i64),
) -> Vec<HueSample> {
    let tiered: Vec<String> = lines
        .iter()
        .filter(|line| !line.contains("\"severity_tier\":\"none\""))
        .cloned()
        .collect();
    hue_samples_in_window(&tiered, bound, window)
}

/// `|(at_ms − duration_ms) − t|` for a hue sample, where `t` is the nearest `created=true`
/// incident-creation line at or before the sample. The sample's start instant is the tier-effective
/// instant Pulse subtracted, so for a rise it should land on the raising incident's opening. A
/// `created=false` dedupe is skipped; `None` when no creation precedes the sample.
pub(crate) fn incident_anchor_error_ms(sample: &HueSample, lines: &[String]) -> Option<f64> {
    lines
        .iter()
        .filter(|line| is_target(line, INCIDENT_CREATED_TARGET))
        .filter(|line| line.contains("\"created\":true"))
        .filter_map(|line| timestamp_ms(line))
        .filter(|t| *t <= sample.at_ms)
        .max()
        .map(|t| ((sample.at_ms as f64 - sample.duration_ms) - t as f64).abs())
}

/// Every lifecycle-tick instant in the capture, ascending — the RETIRED instrument's mechanism
/// witness (Pulse HEAD `83d4060`).
pub(crate) fn tick_times_ms(lines: &[String]) -> Vec<i64> {
    let mut ticks: Vec<i64> = lines
        .iter()
        .filter(|line| is_target(line, LIFECYCLE_TICK_TARGET))
        .filter_map(|line| timestamp_ms(line))
        .collect();
    ticks.sort_unstable();
    ticks
}

/// How far a sample sits from the most recent tick at or before it — the quantity the RETIRED
/// instrument's reported duration equalled at Pulse HEAD `83d4060`, because the tick is what stamped
/// the `last_seen` it read.
pub(crate) fn tick_offset_ms(sample: &HueSample, ticks: &[i64]) -> Option<f64> {
    ticks
        .iter()
        .copied()
        .filter(|t| *t <= sample.at_ms)
        .max()
        .map(|t| (sample.at_ms - t) as f64)
}

/// `halo-hue-encoding`'s `healthy-baseline` phase length — the offset from the one
/// `timeline.execute` start to phase-2 start in `contracts/pulse-p025-measurement-contract.md`
/// §The grading rule.
pub(crate) const P025_PHASE_TWO_OFFSET_MS: i64 = 30_000;

/// The P-025 leg window from a frozen self-obs journal: `[timeline.execute new + 30 000 ms,
/// scenario.run close]`. Keyed on the span AND its `span_event`, since every child line carries
/// `"parent":"<span>"` too (testing.md 2026-10-01). `None` unless exactly one of each is present.
pub(crate) fn p025_window(selfobs: &str) -> Option<(i64, i64)> {
    let stamps = |span: &str, event: &str| -> Vec<i64> {
        selfobs
            .lines()
            .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
            .filter(|v| {
                v.get("span").and_then(serde_json::Value::as_str) == Some(span)
                    && v.get("span_event").and_then(serde_json::Value::as_str) == Some(event)
            })
            .filter_map(|v| v.get("timestamp_ms").and_then(serde_json::Value::as_i64))
            .collect()
    };
    match (
        stamps("timeline.execute", "new").as_slice(),
        stamps("scenario.run", "close").as_slice(),
    ) {
        ([start], [close]) => Some((start + P025_PHASE_TWO_OFFSET_MS, *close)),
        _ => None,
    }
}
