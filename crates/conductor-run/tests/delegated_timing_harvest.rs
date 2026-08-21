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
        let coords: Vec<_> = b.iter().map(|x| (x.p_id, x.target, x.field, x.budget_ms)).collect();
        assert_eq!(
            coords,
            vec![
                ("P-025", "metric.constellation.hue_update_ms", "duration_ms", 2_000.0),
                ("P-027", "metric.constellation.discovery_ms", "duration_ms", 5_000.0),
                ("P-037", "metric.report.render_ms", "value", 2_000.0),
                ("P-045", "metric.findings.counter_refresh_ms", "duration_ms", 1_000.0),
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
        let mistaken = DelegatedBound { field: "duration_ms", ..real };
        let lines = vec![synthetic(real.target, "value", 1_500.0)];
        assert_eq!(grade(&lines, &real), Ok(1_500.0));
        assert!(grade(&lines, &mistaken).is_err(), "the mis-paired field must find nothing");
    }

    /// Absence grades UNGRADED, never met — the degrade direction that keeps an unfired observable
    /// from reading as a satisfied budget.
    #[test]
    fn a_bound_with_no_observation_is_ungraded_not_met() {
        for b in bounds() {
            let err = grade(&[], &b).expect_err("no observation must not pass");
            assert!(err.contains("UNGRADED"), "the reason must say ungraded: {err}");
            assert!(err.contains(b.p_id), "the reason must name its capability: {err}");
        }
    }

    /// A budget is broken by its WORST observation, not rescued by a fast neighbour.
    #[test]
    fn the_worst_observation_grades_the_budget() {
        let b = bounds()[3];
        let ok = vec![synthetic(b.target, b.field, 400.0), synthetic(b.target, b.field, 900.0)];
        assert_eq!(grade(&ok, &b), Ok(900.0));

        let breached =
            vec![synthetic(b.target, b.field, 400.0), synthetic(b.target, b.field, 1_400.0)];
        let err = grade(&breached, &b).expect_err("1400ms is over the 1000ms budget");
        assert!(err.contains("1400"), "the reason must carry the measured value: {err}");
    }

    /// The boundary is inclusive: a bound claiming "≤1s" is met at exactly 1s.
    #[test]
    fn the_budget_boundary_is_inclusive() {
        let b = bounds()[3];
        assert_eq!(grade(&[synthetic(b.target, b.field, 1_000.0)], &b), Ok(1_000.0));
        assert!(grade(&[synthetic(b.target, b.field, 1_000.1)], &b).is_err());
    }

    /// Another leaf's line never grades this bound — the targets are matched exactly, so the
    /// constellation pair cannot cross-contaminate despite sharing a `duration_ms` field.
    #[test]
    fn a_different_leaf_does_not_grade_this_bound() {
        let hue = bounds()[0];
        let discovery = bounds()[1];
        let lines = vec![synthetic(discovery.target, discovery.field, 4_000.0)];
        assert!(grade(&lines, &hue).is_err(), "a discovery line must not grade the hue bound");
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

        assert_eq!(grade(&harvest_since(&path, 2).expect("read"), &b), Ok(1_200.0));
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

    /// The P-025 disproof, pinned. Both legs measured ~36s against a 2s budget, and neither sample
    /// is the scenario's: `halo-hue-encoding.toml` declares ZERO `[phases.emission]` blocks, so it
    /// drives no error-shaped stream and causes no tier change of its own — both samples carry
    /// `severity_tier: "autonomous"` and follow the preflight canary's incident by under a second.
    ///
    /// The mechanism is structural, not a slow run. The fire site computes
    /// `now - item.last_seen_unix_nano`, i.e. how STALE the service's telemetry was when its hue
    /// changed — not how long the hue update took. A service that stops emitting before the tier
    /// flips (the canary storms, then goes quiet for the 90s canary-poll floor) therefore reports
    /// Pulse's own incident-formation latency: L2 cue -> L3 digest (20-60s cadence) -> L4. No <=2s
    /// budget can be met on that path. The same formula reads 702ms for `discovery_ms` precisely
    /// because discovery fires while `last_seen` is still fresh.
    #[test]
    fn p025_hue_update_is_measured_over_budget_by_a_staleness_mechanism() {
        let bound = bounds()[0];
        let err = grade(&hue_lines(), &bound).expect_err("both legs measured far over the budget");
        assert!(err.contains("35581") || err.contains("36704"), "the reason carries a measured value: {err}");

        let worst = observations(&hue_lines(), &bound)
            .into_iter()
            .fold(0.0_f64, f64::max);
        assert!(worst > 17.0 * bound.budget_ms, "measured {worst}ms against a {}ms budget", bound.budget_ms);
    }
}
