//! Live-leg evidence harvest for the four delegated timing budgets (P-025, P-027, P-037, P-045).
//!
//! Pulse delegated these bounds to Conductor and, at SUT HEAD `f0c38f5`, emits each one as a
//! `tracing` event behind its OWN exact allowlist leaf. None of them reaches an MCP read-back
//! surface — Pulse's 8 tools read the corpus and the in-memory buffer, never its own self-obs
//! stream — so, exactly as the storm / baseline / restart / pii / connection / severity harvests do,
//! the bounds grade HERE, over a live-leg capture of `{data_dir}/logs/agent-latest.jsonl.<date>`.
//!
//! Two measured facts shape everything below.
//!
//!   * The FIELD NAME IS NOT UNIFORM. The three leaves added for this delegation carry the duration
//!     in `duration_ms`; the pre-existing `metric.report.render_ms` carries it in `value`. A reader
//!     that assumes one field reads three of four and silently records the fourth as absent — which
//!     is indistinguishable from a leg where it never fired. `bounds()` pins the field per target
//!     and `render_ms_is_not_carried_in_duration_ms` is the negative test that keeps it honest.
//!   * `budget_ms` IS THE WRONG INSTRUMENT and is deliberately not used. It narrows the deadline on
//!     `read_back_observed_at − journal_emitted_at` (`conductor-verify/src/slo.rs`), i.e. Conductor's
//!     own MCP round-trip. These four budgets bound a duration measured INSIDE Pulse, so grading one
//!     against the other would measure Conductor's read-back speed and label it Pulse's render time.
//!
//! ABSENCE IS NEVER A PASS: a bound with no observation grades `Err`, never a satisfied budget —
//! the degrade direction the read-back freshness work settled.
//!
//! MEASURED 2026-08-21 over four live legs (fresh data dir + `pulse-app` restart per leg, window
//! open) against Pulse HEAD `f0c38f5`. THREE budgets are met and asserted below; the fourth is
//! disproved and recorded:
//!
//!   * P-027 constellation discovery — 702.4ms against 5000ms, at `discovered_count: 3` (the
//!     scenario's own three-service topology, which is what attributes it away from the canary).
//!   * P-037 report render — 0-1ms against 2000ms across four samples, every one `degraded_mode`.
//!   * P-045 counter refresh — 269 samples in leg D's window alone (median 1.9ms, max 7.0ms), zero
//!     over the 1000ms budget; ~1000 more across the other three legs, likewise none over.
//!   * P-025 hue update — 35581ms and 36705ms against 2000ms, on two independent legs. NOT a slow
//!     run: the observable measures staleness rather than update latency, and the scenario drives no
//!     tier change of its own. Pinned as a disproof by
//!     `p025_hue_update_is_measured_over_budget_by_a_staleness_mechanism`, never as a pass.
//!
//! TEST-ONLY affordance (the storm-harvest precedent): nothing here is wired into the run path and
//! nothing harvested reaches a Conductor artifact.

use std::path::Path;

/// One delegated bound: which Pulse target carries it, which field holds the duration, and the
/// budget in milliseconds. The field is part of the identity, not a detail — see the module doc.
#[derive(Debug, Clone, Copy, PartialEq)]
struct DelegatedBound {
    p_id: &'static str,
    target: &'static str,
    field: &'static str,
    budget_ms: f64,
}

/// The four bounds Pulse delegated, as measured at SUT HEAD `f0c38f5`.
fn bounds() -> [DelegatedBound; 4] {
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

fn harvest_since(path: &Path, skip_lines: usize) -> std::io::Result<Vec<String>> {
    let body = std::fs::read_to_string(path)?;
    Ok(body.lines().skip(skip_lines).map(str::to_owned).collect())
}

fn is_target(line: &str, target: &str) -> bool {
    line.contains(&format!("\"target\":\"{target}\""))
}

fn num_field(line: &str, key: &str) -> Option<f64> {
    let tail = line.split(&format!("\"{key}\":")).nth(1)?;
    let end = tail.find([',', '}'])?;
    tail[..end].trim().parse().ok()
}

/// Every observation of one bound in the capture. A line that is not this target, or that lacks the
/// bound's own duration field, is SKIPPED rather than fatal — a harvest reads another process's
/// output, so the verdict/error wall applies to evidence too.
fn observations(lines: &[String], bound: &DelegatedBound) -> Vec<f64> {
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
fn grade(lines: &[String], bound: &DelegatedBound) -> Result<f64, String> {
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
/// `ServiceRegistryEntry.last_seen_unix_nano`, which is what makes it P-025's mechanism witness.
const LIFECYCLE_TICK_TARGET: &str = "triage.lifecycle.tick";

/// How far a hue sample's reported duration may sit from its offset to the preceding tick.
///
/// The UI polls `services.list_with_states` every 1000ms (`use-service-constellation.ts:22`) and the
/// sample is recorded over TauRPC after that poll observes the refreshed value, so the reported age
/// trails the tick by up to one poll plus IPC.
const TICK_OFFSET_TOLERANCE_MS: f64 = 1_100.0;

/// Milliseconds since the Unix epoch for a capture line's `timestamp` field, or `None` when the line
/// carries no parseable stamp — a harvest reads another process's output, so a malformed line is
/// skipped rather than fatal (the same posture `observations` takes).
fn timestamp_ms(line: &str) -> Option<i64> {
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
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
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
struct HueSample {
    duration_ms: f64,
    at_ms: i64,
}

/// The hue samples inside `[start_ms, end_ms]`, the window that attributes them to the scenario.
///
/// The leaf carries NO service identifier (Pulse's allowlist for it is exactly
/// `duration_ms` + `severity_tier`, "Aggregate-only"), and `severity_tier` cannot discriminate either
/// — a dot's tier is incident-derived and incidents form only from Autonomous cues, so the scenario
/// reaches the same band the canary does. Attribution is therefore TEMPORAL: the window opens at the
/// scenario's own sustained phase, by which time preflight readiness has already required (and
/// observed) the canary's incident, so the canary's flip and its sample precede `scenario.run`.
fn hue_samples_in_window(
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

/// Every lifecycle-tick instant in the capture, ascending.
fn tick_times_ms(lines: &[String]) -> Vec<i64> {
    let mut ticks: Vec<i64> = lines
        .iter()
        .filter(|line| is_target(line, LIFECYCLE_TICK_TARGET))
        .filter_map(|line| timestamp_ms(line))
        .collect();
    ticks.sort_unstable();
    ticks
}

/// How far a sample sits from the most recent tick at or before it — the quantity the reported
/// duration should equal, because the tick is what stamped `last_seen`.
fn tick_offset_ms(sample: &HueSample, ticks: &[i64]) -> Option<f64> {
    ticks
        .iter()
        .copied()
        .filter(|t| *t <= sample.at_ms)
        .max()
        .map(|t| (sample.at_ms - t) as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A SYNTHETIC line in Pulse's on-disk shape — used to exercise the parser, never presented as
    /// leg evidence. The real captures land in this module as verbatim `leg_*_lines()` functions
    /// once the legs run (module doc, STATUS).
    fn synthetic(target: &str, field: &str, value: f64) -> String {
        format!(
            r#"{{"fields":{{"deployment.environment":"production","{field}":{value},"service.name":"com.andromeda.pulse"}},"level":"INFO","message":"sample","target":"{target}","timestamp":"2026-08-21T09:00:27.965Z"}}"#
        )
    }

    /// The delegation's own coordinates, pinned so a Pulse-side rename cannot drift past unnoticed:
    /// four bounds, four distinct targets, and the budgets Pulse delegated.
    #[test]
    fn the_four_delegated_bounds_are_pinned_to_their_targets_and_budgets() {
        let b = bounds();
        let coords: Vec<_> = b
            .iter()
            .map(|x| (x.p_id, x.target, x.field, x.budget_ms))
            .collect();
        assert_eq!(
            coords,
            vec![
                (
                    "P-025",
                    "metric.constellation.hue_update_ms",
                    "duration_ms",
                    2_000.0
                ),
                (
                    "P-027",
                    "metric.constellation.discovery_ms",
                    "duration_ms",
                    5_000.0
                ),
                ("P-037", "metric.report.render_ms", "value", 2_000.0),
                (
                    "P-045",
                    "metric.findings.counter_refresh_ms",
                    "duration_ms",
                    1_000.0
                ),
            ]
        );
        let mut targets: Vec<_> = b.iter().map(|x| x.target).collect();
        targets.sort_unstable();
        targets.dedup();
        assert_eq!(targets.len(), 4, "each bound must key on its own leaf");
    }

    /// The field divergence, stated positively: P-037's duration rides `value`.
    #[test]
    fn render_ms_is_carried_in_value() {
        let b = bounds()[2];
        let lines = vec![synthetic(b.target, "value", 1_500.0)];
        assert_eq!(grade(&lines, &b), Ok(1_500.0));
    }

    /// The same divergence as a NEGATIVE test — the plausible mis-pairing pinned so it cannot be
    /// re-introduced. Reading P-037 with `duration_ms` finds nothing, and "nothing" is precisely
    /// what a leg where the operator never opened the Report also looks like: the two are
    /// indistinguishable downstream, which is why the field belongs to the bound's identity.
    #[test]
    fn render_ms_is_not_carried_in_duration_ms() {
        let real = bounds()[2];
        let mistaken = DelegatedBound {
            field: "duration_ms",
            ..real
        };
        let lines = vec![synthetic(real.target, "value", 1_500.0)];
        assert_eq!(grade(&lines, &real), Ok(1_500.0));
        assert!(
            grade(&lines, &mistaken).is_err(),
            "the mis-paired field must find nothing"
        );
    }

    /// Absence grades UNGRADED, never met — the degrade direction that keeps an unfired observable
    /// from reading as a satisfied budget.
    #[test]
    fn a_bound_with_no_observation_is_ungraded_not_met() {
        for b in bounds() {
            let err = grade(&[], &b).expect_err("no observation must not pass");
            assert!(
                err.contains("UNGRADED"),
                "the reason must say ungraded: {err}"
            );
            assert!(
                err.contains(b.p_id),
                "the reason must name its capability: {err}"
            );
        }
    }

    /// A budget is broken by its WORST observation, not rescued by a fast neighbour.
    #[test]
    fn the_worst_observation_grades_the_budget() {
        let b = bounds()[3];
        let ok = vec![
            synthetic(b.target, b.field, 400.0),
            synthetic(b.target, b.field, 900.0),
        ];
        assert_eq!(grade(&ok, &b), Ok(900.0));

        let breached = vec![
            synthetic(b.target, b.field, 400.0),
            synthetic(b.target, b.field, 1_400.0),
        ];
        let err = grade(&breached, &b).expect_err("1400ms is over the 1000ms budget");
        assert!(
            err.contains("1400"),
            "the reason must carry the measured value: {err}"
        );
    }

    /// The boundary is inclusive: a bound claiming "≤1s" is met at exactly 1s.
    #[test]
    fn the_budget_boundary_is_inclusive() {
        let b = bounds()[3];
        assert_eq!(
            grade(&[synthetic(b.target, b.field, 1_000.0)], &b),
            Ok(1_000.0)
        );
        assert!(grade(&[synthetic(b.target, b.field, 1_000.1)], &b).is_err());
    }

    /// Another leaf's line never grades this bound — the targets are matched exactly, so the
    /// constellation pair cannot cross-contaminate despite sharing a `duration_ms` field.
    #[test]
    fn a_different_leaf_does_not_grade_this_bound() {
        let hue = bounds()[0];
        let discovery = bounds()[1];
        let lines = vec![synthetic(discovery.target, discovery.field, 4_000.0)];
        assert!(
            grade(&lines, &hue).is_err(),
            "a discovery line must not grade the hue bound"
        );
        assert_eq!(grade(&lines, &discovery), Ok(4_000.0));
    }

    /// The pre-leg slice, which is load-bearing on a real capture: `pulse-app` writes its own boot
    /// output before the OTLP receiver opens (~2.2k lines even on a fresh data dir), so a harvest
    /// that skips nothing mixes boot output into the leg window.
    #[test]
    fn harvest_since_skips_the_pre_leg_prefix() {
        let dir = std::env::temp_dir().join("conductor-delegated-timing-harvest-slice");
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("agent-latest.jsonl.2026-08-21");
        let b = bounds()[0];
        let body = format!(
            "boot-a\nboot-b\n{}\n",
            synthetic(b.target, b.field, 1_200.0)
        );
        std::fs::write(&path, body).expect("write");

        assert_eq!(
            grade(&harvest_since(&path, 2).expect("read"), &b),
            Ok(1_200.0)
        );
        assert!(
            grade(&harvest_since(&path, 3).expect("read"), &b).is_err(),
            "slicing past the sample leaves the bound ungraded"
        );
        std::fs::remove_file(&path).ok();
    }

    // ---- Live-leg evidence, 2026-08-21 ----
    // Four legs, each on a fresh data dir with `pulse-app` restarted and its window open, against
    // Pulse at HEAD `f0c38f5`. Every line below is VERBATIM from that leg's capture, sliced to the
    // leg window by a pre-leg line count taken after readiness and before the first dispatch.

    /// VERBATIM from the leg B capture (run `2026-08-21T18-42-31-734`, pre-leg 6747). The
    /// `discovered_count: 3` is what ATTRIBUTES this sample to `service-constellation-discovery`'s
    /// own second phase — its three-service topology — rather than to the preflight canary, whose
    /// discovery line in the same window carries `discovered_count: 1` at 703.98ms.
    fn leg_b_discovery_line() -> String {
        r#"{"fields":{"deployment.environment":"production","discovered_count":3,"duration_ms":702.4326171875,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"constellation discovery latency recorded","target":"metric.constellation.discovery_ms","timestamp":"2026-08-21T18:43:26.650Z"}"#.to_owned()
    }

    #[test]
    fn p027_constellation_discovery_meets_its_five_second_budget() {
        let bound = bounds()[1];
        assert_eq!(grade(&[leg_b_discovery_line()], &bound), Ok(702.4326171875));
    }

    /// VERBATIM from the leg C captures (runs `2026-08-21T18-45-01-029` and
    /// `2026-08-21T18-50-22-415`). Both carry `degraded_mode: true` because deterministic L4 leaves
    /// the interpretation pending, so the six-section report built here is the DEGRADED form — a
    /// cheaper build than a fully-populated one. At 0-1ms against a 2000ms budget the caveat cannot
    /// change the verdict, but it is what the samples measure.
    fn leg_c_render_lines() -> Vec<String> {
        [
            r#"{"fields":{"degraded_mode":true,"deployment.environment":"production","section_count":6,"service.name":"com.andromeda.pulse","service.version":"0.1.0","value":1},"level":"INFO","message":"report render latency sample","target":"metric.report.render_ms","timestamp":"2026-08-21T18:45:46.431Z"}"#,
            r#"{"fields":{"degraded_mode":true,"deployment.environment":"production","section_count":6,"service.name":"com.andromeda.pulse","service.version":"0.1.0","value":0},"level":"INFO","message":"report render latency sample","target":"metric.report.render_ms","timestamp":"2026-08-21T18:51:08.429Z"}"#,
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    #[test]
    fn p037_report_render_meets_its_two_second_budget() {
        let bound = bounds()[2];
        assert_eq!(grade(&leg_c_render_lines(), &bound), Ok(1.0));
    }

    /// VERBATIM from the leg D capture (run `2026-08-21T18-53-35-135`, pre-leg 3549) — the SLOWEST
    /// of the 269 samples in that leg's window (n=269, min 1.3ms, median 1.9ms, max 7.0ms, zero
    /// samples over the budget). Pinning the worst sample is what makes the assertion meaningful:
    /// the budget is broken by the worst observation, never rescued by a fast neighbour.
    fn leg_d_counter_worst_line() -> String {
        r#"{"fields":{"deployment.environment":"production","duration_ms":7.0,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"findings counter refresh latency recorded","target":"metric.findings.counter_refresh_ms","timestamp":"2026-08-21T18:55:03.299Z"}"#.to_owned()
    }

    #[test]
    fn p045_counter_refresh_meets_its_one_second_budget() {
        let bound = bounds()[3];
        assert_eq!(grade(&[leg_d_counter_worst_line()], &bound), Ok(7.0));
    }

    /// VERBATIM hue samples from legs A and B (runs `2026-08-21T18-38-48-967` and
    /// `2026-08-21T18-42-31-734`). P-025 is RECORDED here, not asserted green — the measurement
    /// disproved the premise its budget rested on, and the test pins the disproof so it cannot
    /// silently drift back to an assumed pass.
    fn hue_lines() -> Vec<String> {
        [
            r#"{"fields":{"deployment.environment":"production","duration_ms":35581.440673828125,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"autonomous"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-08-21T18:39:35.309Z"}"#,
            r#"{"fields":{"deployment.environment":"production","duration_ms":36704.983642578125,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"autonomous"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-08-21T18:43:17.652Z"}"#,
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    /// The P-025 disproof, pinned — and the ORIGIN of the mechanism the successor test pins.
    ///
    /// Both legs measured ~36s against a 2s budget, and neither sample was the scenario's: at that
    /// time `halo-hue-encoding.toml` declared ZERO `[phases.emission]` blocks, so it drove no stream
    /// and caused no tier change of its own — both samples carry `severity_tier: "autonomous"` and
    /// follow the preflight canary's incident by under a second. (The scenario has since been
    /// re-driven; these lines stay as the record of what the un-driven shape measured.)
    ///
    /// THREE fire-site terms explain it, and only the first was known when this test was written.
    /// (1) STALENESS: the site computes `now - item.last_seen_unix_nano`, i.e. how stale the
    /// service's telemetry was when its hue changed, not how long the update took.
    /// (2) SLOWEST-WINS: it loops every dot whose tier changed in the render pass and emits the
    /// MAXIMUM staleness with THAT dot's tier, so a same-pass canary change reports the canary.
    /// (3) QUANTIZATION: `last_seen_unix_nano` has no ingest-path writer — steady-state it is
    /// written only by the 15s lifecycle tick, and only when the service was seen <1s before it. So
    /// the observable reports `t_sample - t_last_refreshing_tick`, U(0, 15s) at a randomly-timed
    /// flip and INDEPENDENT of the dispatch rate.
    ///
    /// Term (3) is why no arm here asserts a sample UNDER 2000ms: at 15s quantization a sub-2s
    /// reading is a tick coincidence (~13% of flips), never attainment. The bound is unmeasurable
    /// through this leaf until the SUT changes the writer.
    #[test]
    fn p025_hue_update_is_recorded_over_budget_never_asserted_as_a_pass() {
        let bound = bounds()[0];
        let err = grade(&hue_lines(), &bound).expect_err("both legs measured far over the budget");
        assert!(
            err.contains("35581") || err.contains("36704"),
            "the reason carries a measured value: {err}"
        );

        let worst = observations(&hue_lines(), &bound)
            .into_iter()
            .fold(0.0_f64, f64::max);
        assert!(
            worst > 17.0 * bound.budget_ms,
            "measured {worst}ms against a {}ms budget",
            bound.budget_ms
        );
    }

    // ---- The re-driven P-025 selection + mechanism pin ----
    // The live capture rides the scenario's own leg; the helpers below are exercised here against
    // SYNTHETIC lines in Pulse's on-disk shape, so the selection rule and the tick arithmetic are
    // pinned independently of when that leg runs.

    fn synthetic_hue(duration_ms: f64, at: &str) -> String {
        format!(
            r#"{{"fields":{{"deployment.environment":"production","duration_ms":{duration_ms},"service.name":"com.andromeda.pulse","severity_tier":"autonomous"}},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"{at}"}}"#
        )
    }

    fn synthetic_tick(at: &str) -> String {
        format!(
            r#"{{"fields":{{"deployment.environment":"production","service.name":"com.andromeda.pulse"}},"level":"INFO","message":"lifecycle heartbeat","target":"{LIFECYCLE_TICK_TARGET}","timestamp":"{at}"}}"#
        )
    }

    #[test]
    fn a_capture_timestamp_parses_to_epoch_millis() {
        // 1970-01-01T00:00:00.000Z is the origin, and a millisecond is a millisecond.
        assert_eq!(
            timestamp_ms(&synthetic_tick("1970-01-01T00:00:00.000Z")),
            Some(0)
        );
        assert_eq!(
            timestamp_ms(&synthetic_tick("1970-01-01T00:00:00.250Z")),
            Some(250)
        );
        // A leap day, so the civil conversion is exercised rather than assumed.
        let a = timestamp_ms(&synthetic_tick("2024-02-28T23:59:59.000Z")).expect("parses");
        let b = timestamp_ms(&synthetic_tick("2024-02-29T23:59:59.000Z")).expect("parses");
        assert_eq!(b - a, 86_400_000, "2024 is a leap year — Feb 29 exists");
        assert_eq!(timestamp_ms("no timestamp here"), None);
    }

    /// The window is what attributes a sample to the scenario, because the leaf carries no service
    /// identifier and `severity_tier` is identical for the canary and the subject.
    #[test]
    fn the_window_selects_the_scenarios_own_samples_and_excludes_the_canarys() {
        let bound = bounds()[0];
        let lines = vec![
            // The canary's flip: before `scenario.run`, which preflight readiness guarantees.
            synthetic_hue(35_000.0, "2026-09-07T10:00:00.000Z"),
            // The subject's flip, inside its own sustained phase.
            synthetic_hue(4_200.0, "2026-09-07T10:02:30.000Z"),
            // After read-back closes the window.
            synthetic_hue(58_000.0, "2026-09-07T10:30:00.000Z"),
        ];
        let start = timestamp_ms(&synthetic_tick("2026-09-07T10:01:00.000Z")).expect("parses");
        let end = timestamp_ms(&synthetic_tick("2026-09-07T10:05:00.000Z")).expect("parses");

        let picked = hue_samples_in_window(&lines, &bound, (start, end));

        assert_eq!(picked.len(), 1, "exactly the in-window sample: {picked:?}");
        assert_eq!(picked[0].duration_ms, 4_200.0);
    }

    /// THE MECHANISM PIN. `last_seen_unix_nano` is written only by the 15s lifecycle tick, so a hue
    /// sample's reported duration is its own offset to the preceding tick — not a render latency.
    /// This is the property that makes the ≤2s budget unmeasurable through this leaf, and asserting
    /// it is what turns a 36s reading from an unexplained number into a verified consequence.
    #[test]
    fn a_hue_sample_reports_its_offset_to_the_preceding_lifecycle_tick() {
        let bound = bounds()[0];
        // Ticks 15s apart, and a flip 9.4s after the second one.
        let lines = vec![
            synthetic_tick("2026-09-07T10:02:00.000Z"),
            synthetic_tick("2026-09-07T10:02:15.000Z"),
            synthetic_hue(9_400.0, "2026-09-07T10:02:24.400Z"),
            synthetic_tick("2026-09-07T10:02:30.000Z"),
        ];
        let start = timestamp_ms(&synthetic_tick("2026-09-07T10:01:00.000Z")).expect("parses");
        let end = timestamp_ms(&synthetic_tick("2026-09-07T10:05:00.000Z")).expect("parses");

        let ticks = tick_times_ms(&lines);
        assert_eq!(ticks.len(), 3, "every tick line is a witness: {ticks:?}");

        let samples = hue_samples_in_window(&lines, &bound, (start, end));
        assert_eq!(samples.len(), 1);

        let offset = tick_offset_ms(&samples[0], &ticks).expect("a tick precedes the sample");
        assert!(
            (samples[0].duration_ms - offset).abs() <= TICK_OFFSET_TOLERANCE_MS,
            "reported {}ms vs {offset}ms since the preceding tick — beyond the {TICK_OFFSET_TOLERANCE_MS}ms poll+IPC tolerance",
            samples[0].duration_ms
        );
        // The tick 15s LATER must not be the one chosen — only a tick at or before the sample can
        // have stamped the value it read.
        assert_eq!(offset, 9_400.0);
    }

    /// Absence is never a pass, and the window is where a re-driven leg can produce one: if the
    /// subject's flip fell outside it, the harvest must say so rather than grade the canary.
    #[test]
    fn an_empty_window_yields_no_samples_rather_than_a_satisfied_budget() {
        let bound = bounds()[0];
        let lines = vec![synthetic_hue(35_000.0, "2026-09-07T10:00:00.000Z")];
        let start = timestamp_ms(&synthetic_tick("2026-09-07T10:01:00.000Z")).expect("parses");
        let end = timestamp_ms(&synthetic_tick("2026-09-07T10:05:00.000Z")).expect("parses");

        assert!(hue_samples_in_window(&lines, &bound, (start, end)).is_empty());
        // And with no tick before it, the mechanism cannot be pinned — absent, never zero.
        let orphan = HueSample {
            duration_ms: 1.0,
            at_ms: start,
        };
        assert_eq!(tick_offset_ms(&orphan, &[]), None);
    }

    // ---- Live-leg evidence, 2026-09-07 (the re-driven leg) ----
    // Leg H of the operator-gated `run --live` suite, run `2026-09-07T07-42-45-582`, pre-leg 50243,
    // against Pulse at HEAD `83d4060` with the compact-widget window open. `halo-hue-encoding` drove
    // 360 dispatches at 2/s across two phases (`emission_count: 360`, `timeline.execute` spanning
    // 183.8s), so its service was emitting CONTINUOUSLY through the tier flip.

    /// VERBATIM — the ONLY hue sample inside leg H's window (phase-2 start `1788767041678` through
    /// `scenario.run` close `1788767195445`). The capture held 7 hue samples after the pre-leg
    /// baseline; the other 6 fall outside the window and belong to the canary or to post-leg tier
    /// changes.
    fn leg_h_hue_line() -> String {
        r#"{"fields":{"deployment.environment":"production","duration_ms":14525.947021484377,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_tier":"autonomous"},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"2026-09-07T07:44:07.658Z"}"#.to_owned()
    }

    /// VERBATIM — the lifecycle tick immediately preceding that sample. Tick spacing across the
    /// capture measured n=76, min 14986ms, median 15000ms, max 15013ms.
    fn leg_h_preceding_tick_line() -> String {
        r#"{"fields":{"deployment.environment":"production","service.name":"com.andromeda.pulse","service.version":"0.1.0","services_active":0,"services_archived":0,"services_bootstrapping":2,"services_dormant":0,"services_quiet":0,"services_silent":0,"services_unknown":0,"tracked_services_total":2},"level":"INFO","message":"lifecycle heartbeat tick","target":"triage.lifecycle.tick","timestamp":"2026-09-07T07:43:53.131Z"}"#.to_owned()
    }

    /// THE RE-DRIVEN MEASUREMENT — and the disproof completed on the SUBJECT's own sample.
    ///
    /// The 2026-08-21 legs could be dismissed as never having driven anything: the scenario declared
    /// no emission, so both samples were the canary's and the service had gone quiet. This leg
    /// removes that explanation. `halo-hue-encoding` emitted 2 dispatches per second THROUGH the
    /// tier flip, so at the moment its dot changed tier the service had been seen ~0.5s earlier —
    /// and the observable still reported **14525.9ms** against a 2000ms budget.
    ///
    /// The reason is the quantization, and this is what pins it: the sample's duration equals its
    /// offset to the PRECEDING lifecycle tick (14527ms) to within 1.1ms. `last_seen_unix_nano` is
    /// stamped by that 15s tick, never by ingest, so a flip landing 14.5s after a tick reports
    /// 14.5s no matter how recently a span arrived. Meeting 2000ms requires the flip to land inside
    /// the first 2s of a 15s window — a ~13% coincidence, which is why NO arm here asserts a pass.
    #[test]
    fn p025_the_re_driven_leg_measures_tick_quantization_not_update_latency() {
        let bound = bounds()[0];
        let lines = vec![leg_h_preceding_tick_line(), leg_h_hue_line()];
        let window = (1_788_767_041_678_i64, 1_788_767_195_445_i64);

        let samples = hue_samples_in_window(&lines, &bound, window);
        assert_eq!(
            samples.len(),
            1,
            "leg H's own sample, canary excluded: {samples:?}"
        );
        let sample = samples[0];

        let ticks = tick_times_ms(&lines);
        let offset = tick_offset_ms(&sample, &ticks).expect("a tick precedes the sample");

        // THE MECHANISM: the reported duration IS the offset to the tick that stamped `last_seen`.
        assert!(
            (sample.duration_ms - offset).abs() <= TICK_OFFSET_TOLERANCE_MS,
            "reported {}ms vs {offset}ms since the preceding tick",
            sample.duration_ms
        );
        assert_eq!(offset, 14_527.0, "the measured tick offset, pinned");

        // RECORDED, never asserted as a pass: the subject's own sample is over budget, on a leg
        // where the subject never stopped emitting.
        assert!(
            sample.duration_ms > bound.budget_ms,
            "the re-driven sample is recorded over budget: {}ms vs {}ms",
            sample.duration_ms,
            bound.budget_ms
        );
        assert!((sample.duration_ms - 14_525.947_021_484_377).abs() < 1e-6);
    }
}
