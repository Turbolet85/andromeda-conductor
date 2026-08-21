//! Live-leg evidence harvest for the severity-lifecycle family (P-019..P-023, P-059, P-060).
//!
//! None of this family's authored read-back checks could grade under deterministic L4, each for a
//! reason measured at SUT HEAD `efabe8e` and recorded in the scenario TOML headers. The surfaces
//! that DO carry the family's reaction are Pulse's own ledger lines, so the live proof grades here,
//! exactly as the storm / baseline / restart / pii / connection harvests do.
//!
//! Three witnesses carry the P-022 lifecycle, and which one carries what is itself a measurement:
//!
//!   * `incidents.list_active.request` → `item_count` is the AUTO-RESOLVE witness. The obvious
//!     candidate — `triage.incident.auto_resolve.tick` — turns out to carry `evaluated_count` /
//!     `resolved_count` / `duration_ms` as the literal string `"<redacted>"` on the wire, because
//!     Pulse's observer field allowlist does not admit them. So the tick line proves the observer
//!     RAN and can never prove it RESOLVED anything; the active-count transition is what moves.
//!   * `interpretation.incident.created` → `created` / `deduped` is the NEW-NOT-REOPEN witness, and
//!     it doubles as an independent resolution proof: the producer dedups against the ACTIVE set
//!     only, so a same-fingerprint retrigger reads `created=true, deduped=false` if and only if the
//!     earlier incident was no longer active. Had it still been open the line would read
//!     `created=false, deduped=true`.
//!   * `triage.cue.emit` → `priority` is the P-019/P-020 tier witness. The line carries NO service
//!     identity (Pulse's aggregate-only convention bans `scope_id` / `service_name` from it), so
//!     cues are attributed by `persistence_seconds`, which IS the emitting service's sample count:
//!     the preflight canary's is pinned at `CANARY_SAMPLE_COUNT` by its fixed warm-up + storm, and
//!     a scenario's is its own emission count. Without that discriminator the canary's frozen cue —
//!     it stops emitting, and the EWMA is sample-driven, so its last value repeats every tick
//!     forever — would be indistinguishable from the scenario's.
//!
//! TEST-ONLY affordance (the storm-harvest precedent): nothing here is wired into the run path and
//! nothing harvested reaches a Conductor artifact. On Windows the capture is
//! `{data_dir}/logs/agent-latest.jsonl.<date>`, sliced to the leg window by a pre-leg line count —
//! which is NOT zero even on a fresh data dir, because `pulse-app` logs its own boot before the
//! receiver opens (2234 lines on leg A).

use std::path::Path;

/// The preflight canary's sample count: `[incident_formation].warmup_emissions` (3) benign spans
/// plus `CANARY_STORM_COUNT` (12) exceptions. A cue reporting this many samples is the canary's,
/// not the scenario's.
const CANARY_SAMPLE_COUNT: u64 = 3 + conductor_run::CANARY_STORM_COUNT;

/// Pulse's auto-resolution window (`DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS`) plus its observer
/// tick (`DEFAULT_AUTO_RESOLVE_TICK_INTERVAL`): an incident idle at the window boundary is resolved
/// at the NEXT tick, so this sum is the true upper bound on observed resolution latency.
const RESOLVE_BOUND_MS: i64 = (120 + 30) * 1000;

/// One `triage.cue.emit` observation.
#[derive(Debug, Clone, PartialEq)]
struct CueEmitted {
    kind: String,
    priority: String,
    magnitude: f64,
    confidence: f64,
    persistence_seconds: u64,
}

/// One `interpretation.incident.created` producer outcome.
#[derive(Debug, Clone, PartialEq)]
struct IncidentOutcome {
    at_ms: i64,
    created: bool,
    deduped: bool,
    priority_tier: String,
}

/// One `incidents.list_active.request` observation of the in-app active set.
#[derive(Debug, Clone, PartialEq)]
struct ActiveCount {
    at_ms: i64,
    count: u64,
}

/// Slice a harvested log to the leg window. `skip_lines` is the pre-leg line count.
fn harvest_since(path: &Path, skip_lines: usize) -> std::io::Result<Vec<String>> {
    let body = std::fs::read_to_string(path)?;
    Ok(body.lines().skip(skip_lines).map(str::to_owned).collect())
}

/// Milliseconds-of-day from an RFC-3339 stamp's time component. A leg is minutes long and never
/// crosses midnight, so a time-of-day basis is sufficient and avoids a date dependency; a negative
/// delta would mean it did, which the callers treat as a parse failure rather than a measurement.
fn at_ms(line: &str) -> Option<i64> {
    let t = line.split("\"timestamp\":\"").nth(1)?.get(11..23)?;
    let (h, rest) = t.split_once(':')?;
    let (m, rest) = rest.split_once(':')?;
    let (s, ms) = rest.split_once('.')?;
    Some(
        h.parse::<i64>().ok()? * 3_600_000
            + m.parse::<i64>().ok()? * 60_000
            + s.parse::<i64>().ok()? * 1_000
            + ms.parse::<i64>().ok()?,
    )
}

fn is_target(line: &str, target: &str) -> bool {
    line.contains(&format!("\"target\":\"{target}\""))
}

fn num_field(line: &str, key: &str) -> Option<f64> {
    let tail = line.split(&format!("\"{key}\":")).nth(1)?;
    let end = tail.find([',', '}'])?;
    tail[..end].trim().parse().ok()
}

fn str_field(line: &str, key: &str) -> Option<String> {
    Some(line.split(&format!("\"{key}\":\"")).nth(1)?.split('"').next()?.to_owned())
}

fn bool_field(line: &str, key: &str) -> Option<bool> {
    let tail = line.split(&format!("\"{key}\":")).nth(1)?;
    let end = tail.find([',', '}'])?;
    match tail[..end].trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

/// A line that does not parse, is not this target, or is missing a field is SKIPPED rather than
/// fatal — a harvest is other-process output (the verdict/error wall applied to evidence).
fn parse_cues(lines: &[String]) -> Vec<CueEmitted> {
    lines
        .iter()
        .filter(|l| is_target(l, "triage.cue.emit"))
        .filter_map(|l| {
            Some(CueEmitted {
                kind: str_field(l, "kind")?,
                priority: str_field(l, "priority")?,
                magnitude: num_field(l, "magnitude")?,
                confidence: num_field(l, "confidence")?,
                persistence_seconds: num_field(l, "persistence_seconds")? as u64,
            })
        })
        .collect()
}

fn parse_outcomes(lines: &[String]) -> Vec<IncidentOutcome> {
    lines
        .iter()
        .filter(|l| is_target(l, "interpretation.incident.created"))
        .filter_map(|l| {
            Some(IncidentOutcome {
                at_ms: at_ms(l)?,
                created: bool_field(l, "created")?,
                deduped: bool_field(l, "deduped")?,
                priority_tier: str_field(l, "priority_tier")?,
            })
        })
        .collect()
}

fn parse_active_counts(lines: &[String]) -> Vec<ActiveCount> {
    lines
        .iter()
        .filter(|l| is_target(l, "incidents.list_active.request"))
        .filter_map(|l| {
            Some(ActiveCount { at_ms: at_ms(l)?, count: num_field(l, "item_count")? as u64 })
        })
        .collect()
}

/// The cues a SCENARIO drove — every cue whose sample count is not the canary's fixed one.
fn scenario_cues(cues: &[CueEmitted]) -> Vec<&CueEmitted> {
    cues.iter().filter(|c| c.persistence_seconds != CANARY_SAMPLE_COUNT).collect()
}

/// P-022 auto-resolve: the in-app active set must strictly DROP, which only a resolution can do
/// (a creation raises it and nothing else touches it). Returns the drop's instant.
fn auto_resolve_observed(counts: &[ActiveCount]) -> Result<i64, String> {
    counts
        .windows(2)
        .find(|w| w[1].count < w[0].count)
        .map(|w| w[1].at_ms)
        .ok_or_else(|| "no active-count drop: nothing auto-resolved in the leg window".to_owned())
}

/// The instant the active set EMPTIED. A drop alone cannot be attributed to a particular incident —
/// the count carries no identity, and a leg always has the preflight canary's incident open beside
/// the scenario's, so the FIRST drop is generally the canary's. Reaching zero is the attributable
/// witness: every incident open before it, the scenario's included, is resolved by then.
fn all_resolved_at(counts: &[ActiveCount]) -> Result<i64, String> {
    let opened = counts.iter().any(|c| c.count > 0);
    if !opened {
        return Err("no incident was ever open in the leg window".to_owned());
    }
    counts
        .iter()
        .skip_while(|c| c.count == 0)
        .find(|c| c.count == 0)
        .map(|c| c.at_ms)
        .ok_or_else(|| "the active set never emptied".to_owned())
}

/// P-022 timing, asserted HARD: an incident idle from `created_at` is resolved no later than the
/// window plus one observer tick.
fn resolution_within_window(created_at: i64, resolved_at: i64) -> Result<(), String> {
    let elapsed = resolved_at - created_at;
    if elapsed <= 0 {
        return Err(format!("resolution precedes creation by {}ms", -elapsed));
    }
    if elapsed > RESOLVE_BOUND_MS {
        return Err(format!("resolution took {elapsed}ms, over the {RESOLVE_BOUND_MS}ms bound"));
    }
    Ok(())
}

/// P-022 new-not-reopen: a post-resolution same-fingerprint detection creates a SECOND incident.
/// Pulse dedups against the ACTIVE set, so `deduped` staying false across the pair is exactly the
/// evidence that the first incident was gone — the read-back surface cannot say this.
fn new_not_reopen(outcomes: &[IncidentOutcome], resolved_at: i64) -> Result<(), String> {
    let before = outcomes.iter().find(|o| o.at_ms < resolved_at && o.created);
    let after = outcomes.iter().find(|o| o.at_ms > resolved_at && o.created);
    match (before, after) {
        (Some(_), Some(a)) if !a.deduped => Ok(()),
        (Some(_), Some(a)) => {
            Err(format!("the retrigger deduped (deduped={}) — the incident was reopened", a.deduped))
        }
        (Some(_), None) => Err("no incident created after the resolution".to_owned()),
        _ => Err("no incident created before the resolution".to_owned()),
    }
}

/// P-019/P-020 tier: the band a scenario's cues reached. Never hard-failed on the exact value —
/// callers record what was measured (the calibration-region policy).
fn reached_band<'a>(cues: &[&'a CueEmitted], priority: &str) -> Option<&'a CueEmitted> {
    cues.iter().find(|c| c.priority == priority).copied()
}

/// P-059: the resolution summary has no producer under deterministic L4, so its ABSENCE across the
/// whole leg is the measurement. Anything naming it would falsify the premise correction on v2-16.
fn no_resolution_summary(lines: &[String]) -> Result<(), String> {
    match lines.iter().find(|l| l.contains("resolution_summary") || l.contains("ResolutionSummary"))
    {
        Some(l) => Err(format!("a resolution-summary line appeared: {}", &l[..l.len().min(160)])),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// VERBATIM from the 2026-08-21 leg A capture (run `2026-08-21T08-59-21-737`): the scenario's
    /// trigger storm clearing the Autonomous band, the incident it produced, the active-count
    /// transitions across both auto-resolutions, the redacted observer tick, and the retrigger
    /// whose `deduped=false` proves the first incident was gone.
    fn leg_a_lines() -> Vec<String> {
        [
            r#"{"fields":{"cue_kind":"retry_storm","deployment.environment":"production","fingerprint_hex":"12dcd67b","occurrence_count":10,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_hint":"autonomous","window_seconds":30},"level":"INFO","message":"","target":"triage.pattern.storm.detected","timestamp":"2026-08-21T09:00:27.965Z"}"#,
            r#"{"fields":{"created":true,"deduped":false,"deployment.environment":"production","priority_tier":"autonomous","service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"error"},"level":"INFO","message":"incident producer outcome","target":"interpretation.incident.created","timestamp":"2026-08-21T09:00:28.046Z"}"#,
            r#"{"fields":{"deployment.environment":"production","item_count":2,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"incidents.list_active returned","target":"incidents.list_active.request","timestamp":"2026-08-21T09:00:28.188Z"}"#,
            r#"{"fields":{"deployment.environment":"production","item_count":1,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"incidents.list_active returned","target":"incidents.list_active.request","timestamp":"2026-08-21T09:02:15.675Z"}"#,
            r#"{"fields":{"deployment.environment":"production","duration_ms":"<redacted>","evaluated_count":"<redacted>","resolved_count":"<redacted>","service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"incident auto-resolution tick","target":"triage.incident.auto_resolve.tick","timestamp":"2026-08-21T09:02:45.455Z"}"#,
            r#"{"fields":{"deployment.environment":"production","item_count":0,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"incidents.list_active returned","target":"incidents.list_active.request","timestamp":"2026-08-21T09:02:45.671Z"}"#,
            r#"{"fields":{"cue_kind":"retry_storm","deployment.environment":"production","fingerprint_hex":"12dcd67b","occurrence_count":10,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_hint":"autonomous","window_seconds":30},"level":"INFO","message":"","target":"triage.pattern.storm.detected","timestamp":"2026-08-21T09:06:09.625Z"}"#,
            r#"{"fields":{"created":true,"deduped":false,"deployment.environment":"production","priority_tier":"autonomous","service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"error"},"level":"INFO","message":"incident producer outcome","target":"interpretation.incident.created","timestamp":"2026-08-21T09:06:09.695Z"}"#,
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    /// P-022's headline, on Pulse's own ledger: the scenario's incident opened at 09:00:28.046 and
    /// the active set had EMPTIED by 09:02:45.671 — 137.6s, inside the 120s window plus the 30s
    /// tick. The bound is asserted against the emptying rather than the first drop, because the
    /// count carries no incident identity and the first drop (09:02:15.675) is the preflight
    /// canary's own incident resolving on its own schedule, 21s earlier.
    #[test]
    fn leg_a_proves_auto_resolve_within_the_window_and_tick() {
        let lines = leg_a_lines();
        let counts = parse_active_counts(&lines);
        let outcomes = parse_outcomes(&lines);
        let first_drop = auto_resolve_observed(&counts).expect("an active-count drop");
        let emptied_at = all_resolved_at(&counts).expect("the active set emptied");
        assert!(first_drop < emptied_at, "the canary resolves before the set empties");

        let created_at = outcomes.first().expect("a creation").at_ms;
        assert_eq!(resolution_within_window(created_at, emptied_at), Ok(()));
        assert_eq!(emptied_at - created_at, 137_625);
    }

    /// The attribution the predicate pair exists to keep honest: pinning the bound to the first
    /// drop would grade the scenario's creation against the CANARY's resolution — here a 107.6s
    /// delta that passes the bound while measuring the wrong incident.
    #[test]
    fn the_first_drop_is_not_the_scenarios_resolution() {
        let lines = leg_a_lines();
        let counts = parse_active_counts(&lines);
        let created_at = parse_outcomes(&lines).first().expect("a creation").at_ms;
        let first_drop = auto_resolve_observed(&counts).expect("a drop");
        assert_eq!(first_drop - created_at, 107_629);
        assert!(all_resolved_at(&counts).expect("emptied") - first_drop == 29_996);
    }

    #[test]
    fn all_resolved_at_needs_an_incident_and_an_emptying() {
        assert!(all_resolved_at(&[ActiveCount { at_ms: 1, count: 0 }]).is_err());
        assert!(all_resolved_at(&[ActiveCount { at_ms: 1, count: 2 }]).is_err());
        let full = [
            ActiveCount { at_ms: 1_000, count: 0 },
            ActiveCount { at_ms: 2_000, count: 1 },
            ActiveCount { at_ms: 3_000, count: 0 },
        ];
        assert_eq!(all_resolved_at(&full), Ok(3_000));
    }

    /// The retrigger carried the SAME fingerprint (`12dcd67b`, pinned in both storm lines) and
    /// still produced `created=true, deduped=false`. Pulse dedups against the active set, so this
    /// is a second, independent proof that the first incident had resolved.
    #[test]
    fn leg_a_proves_new_not_reopen_on_the_same_fingerprint() {
        let lines = leg_a_lines();
        let storms: Vec<_> =
            lines.iter().filter(|l| is_target(l, "triage.pattern.storm.detected")).collect();
        assert_eq!(storms.len(), 2, "a trigger and a retrigger storm");
        assert!(
            storms.iter().all(|l| l.contains("\"fingerprint_hex\":\"12dcd67b\"")),
            "both storms carry ONE fingerprint — otherwise dedup identity differs and the \
             new-not-reopen claim would rest on distinct incidents"
        );
        assert!(storms.iter().all(|l| l.contains("\"severity_hint\":\"autonomous\"")));

        let counts = parse_active_counts(&lines);
        let resolved_at = auto_resolve_observed(&counts).expect("an active-count drop");
        assert_eq!(new_not_reopen(&parse_outcomes(&lines), resolved_at), Ok(()));
    }

    /// The measurement that redirected this harvest: the observer's own tick line carries its
    /// counters as the literal string `"<redacted>"`, so it can witness that auto-resolution RAN
    /// and never that it resolved anything. Pinning it keeps a future reader from "fixing" the
    /// harvest by grading the obvious field.
    #[test]
    fn the_auto_resolve_tick_counters_are_redacted_on_the_wire() {
        let tick = leg_a_lines()
            .into_iter()
            .find(|l| is_target(l, "triage.incident.auto_resolve.tick"))
            .expect("a tick line");
        assert!(tick.contains(r#""resolved_count":"<redacted>""#));
        assert!(tick.contains(r#""evaluated_count":"<redacted>""#));
        assert_eq!(num_field(&tick, "resolved_count"), None, "not a number to grade");
    }

    /// P-059's premise correction, measured rather than argued: nothing in the leg names a
    /// resolution summary, because nothing constructs the digest kind that would attach one and the
    /// deterministic fixture pins `is_resolution_summary` false.
    #[test]
    fn leg_a_carries_no_resolution_summary() {
        assert_eq!(no_resolution_summary(&leg_a_lines()), Ok(()));
    }

    /// VERBATIM from the 2026-08-21 leg B capture: the first cue the scenario drove (91 samples →
    /// confidence 0.91) and the first that cleared magnitude 5.0 one sample later. Together they
    /// show the Autonomous gate being crossed on the magnitude term while confidence was already
    /// satisfied — which is what the 90-sample baseline was authored to buy.
    fn leg_b_lines() -> Vec<String> {
        [
            r#"{"fields":{"absolute_value":0.0333,"confidence":0.91,"deployment.environment":"production","kind":"error_rate_spike","magnitude":3.33,"persistence_seconds":91,"priority":"suggested","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":false},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-21T09:09:44.602Z"}"#,
            r#"{"fields":{"absolute_value":0.06549111,"confidence":0.92,"deployment.environment":"production","kind":"error_rate_spike","magnitude":6.549111000000001,"persistence_seconds":92,"priority":"autonomous","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":true},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-21T09:09:45.602Z"}"#,
            r#"{"fields":{"absolute_value":0.3339595230843561,"confidence":0.15,"deployment.environment":"production","kind":"error_rate_spike","magnitude":8.511516379456504,"persistence_seconds":15,"priority":"curious","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":true},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-21T09:08:11.612Z"}"#,
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    /// P-019/P-020 at the cue tier: `severity-tier-autonomous` reached the Autonomous band, and it
    /// reached it the way the file was authored to — confidence bought by the baseline (0.92 at 92
    /// samples, i.e. samples/100 exactly), magnitude supplied by the spike.
    #[test]
    fn leg_b_reaches_the_autonomous_band_by_the_authored_mechanism() {
        let cues = parse_cues(&leg_b_lines());
        let scenario = scenario_cues(&cues);
        let autonomous = reached_band(&scenario, "autonomous").expect("an autonomous cue");
        assert!(autonomous.magnitude >= 5.0, "magnitude {}", autonomous.magnitude);
        assert!(autonomous.confidence >= 0.9, "confidence {}", autonomous.confidence);
        assert!(autonomous.persistence_seconds >= 30, "persistence");
        assert_eq!(
            autonomous.confidence,
            autonomous.persistence_seconds as f64 / 100.0,
            "confidence IS samples/100 — the term the baseline phase buys"
        );
        assert_eq!(autonomous.kind, "error_rate_spike");
    }

    /// The canary discriminator, without which leg B's own cue is indistinguishable from the
    /// preflight canary's: the canary stops emitting after its fixed warm-up + storm, and because
    /// the EWMA is sample-driven its cue freezes at exactly that sample count and repeats forever.
    #[test]
    fn canary_cues_are_separable_from_scenario_cues_by_sample_count() {
        let cues = parse_cues(&leg_b_lines());
        assert_eq!(cues.len(), 3);
        let scenario = scenario_cues(&cues);
        assert_eq!(scenario.len(), 2, "the canary's cue is excluded");
        assert!(
            cues.iter().any(|c| c.persistence_seconds == CANARY_SAMPLE_COUNT),
            "the canary's frozen cue is present and reports {CANARY_SAMPLE_COUNT} samples"
        );
        assert!(scenario.iter().all(|c| c.persistence_seconds > CANARY_SAMPLE_COUNT));
    }

    /// VERBATIM first scenario cue from the 2026-08-21 legs C and D. Read them beside leg B's: the
    /// magnitude progression is IDENTICAL across all three (3.33 at the first spike sample), because
    /// the spike shape and the EWMA math are the same. Only the baseline sample count differs, and
    /// the band follows it — which is the controlled experiment the three files were re-shaped to be.
    fn leg_c_suggested_line() -> String {
        r#"{"fields":{"absolute_value":0.0333,"confidence":0.71,"deployment.environment":"production","kind":"error_rate_spike","magnitude":3.33,"persistence_seconds":71,"priority":"suggested","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":false},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-21T09:13:00.896Z"}"#.to_owned()
    }

    fn leg_d_curious_line() -> String {
        r#"{"fields":{"absolute_value":0.0333,"confidence":0.21,"deployment.environment":"production","kind":"error_rate_spike","magnitude":3.33,"persistence_seconds":21,"priority":"curious","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":false},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-21T09:15:07.769Z"}"#.to_owned()
    }

    /// P-019/P-020 across the ladder: three legs, three bands, and the term that moved is
    /// confidence — not magnitude, which was identical. Recorded, never hard-failed on the exact
    /// value (the calibration-region policy): what is asserted is that the authored mechanism is
    /// what selected the band.
    #[test]
    fn the_three_tier_legs_separate_by_confidence_at_equal_magnitude() {
        let b = parse_cues(&leg_b_lines());
        let b = scenario_cues(&b);
        let first_b = b.first().expect("leg B cue");
        let c = parse_cues(&[leg_c_suggested_line()]);
        let d = parse_cues(&[leg_d_curious_line()]);

        assert_eq!(first_b.magnitude, c[0].magnitude, "same spike shape ⇒ same first magnitude");
        assert_eq!(first_b.magnitude, d[0].magnitude);

        assert_eq!(c[0].priority, "suggested");
        assert!((0.7..0.9).contains(&c[0].confidence), "confidence {}", c[0].confidence);
        assert_eq!(d[0].priority, "curious");
        assert!(d[0].confidence < 0.7, "confidence {}", d[0].confidence);
        assert!(
            reached_band(&scenario_cues(&parse_cues(&leg_b_lines())), "autonomous").is_some(),
            "leg B is the only one that bought confidence past 0.9"
        );
        for cue in [&c[0], &d[0]] {
            assert_eq!(cue.confidence, cue.persistence_seconds as f64 / 100.0);
        }
    }

    /// The `route_read_back` `AutoResolved` arm, which shipped unit-pinned but UNEXERCISED live on
    /// 2026-08-20 because the canary's own cues kept its incident alive. Leg E exercised it: an
    /// `ack-cooldown` run emits no incident-forming stimulus at all, so the only incident is the
    /// preflight canary's, it auto-resolves 120s + a tick later, and the 370s scenario outlasts it —
    /// leaving an EMPTY active list at read-back, which a declare-only scenario routes to the
    /// pre-accepted residual rather than `Blocked`. This is Conductor's own line, not Pulse's.
    fn leg_e_auto_resolved_arm_line() -> String {
        r#"{"deployment.environment":"local","level":"INFO","message":"declare-only read-back empty: no active incident outlived the emission window","run_id":"2026-08-21T09-16-01-488","service.name":"conductor","service.version":"0.1.0","target":"conductor_run","timestamp_ms":1787304177606}"#.to_owned()
    }

    #[test]
    fn leg_e_exercised_the_auto_resolve_residual_arm_live() {
        let line = leg_e_auto_resolved_arm_line();
        assert!(line.contains("declare-only read-back empty"));
        assert!(line.contains(r#""target":"conductor_run""#), "Conductor's own arm, not Pulse's");
        assert!(line.contains(r#""run_id":"2026-08-21T09-16-01-488""#));
    }

    #[test]
    fn a_harvest_of_unrelated_lines_yields_no_observations() {
        let noise = vec![r#"{"target":"metric.webgpu.frame_duration_ms","fields":{}}"#.to_owned()];
        assert!(parse_cues(&noise).is_empty());
        assert!(parse_outcomes(&noise).is_empty());
        assert!(parse_active_counts(&noise).is_empty());
        assert!(auto_resolve_observed(&parse_active_counts(&noise)).is_err());
    }

    /// A rising active count is a creation, never a resolution — the predicate must not read one as
    /// the other, or every leg would "prove" auto-resolve simply by opening an incident.
    #[test]
    fn a_rising_active_count_is_not_an_auto_resolve() {
        let rising = vec![
            ActiveCount { at_ms: 1_000, count: 0 },
            ActiveCount { at_ms: 2_000, count: 1 },
            ActiveCount { at_ms: 3_000, count: 2 },
        ];
        assert!(auto_resolve_observed(&rising).is_err());
    }

    #[test]
    fn a_resolution_past_the_window_and_tick_fails_the_bound() {
        assert_eq!(resolution_within_window(0, RESOLVE_BOUND_MS), Ok(()));
        assert!(resolution_within_window(0, RESOLVE_BOUND_MS + 1).is_err());
        assert!(resolution_within_window(5_000, 1_000).is_err());
    }

    /// The negative that keeps `new_not_reopen` honest: had the incident still been active, Pulse
    /// would have deduped, and the predicate must reject that rather than reading a refresh as a
    /// new incident.
    #[test]
    fn a_deduped_retrigger_fails_new_not_reopen() {
        let outcomes = vec![
            IncidentOutcome {
                at_ms: 1_000,
                created: true,
                deduped: false,
                priority_tier: "autonomous".to_owned(),
            },
            IncidentOutcome {
                at_ms: 9_000,
                created: true,
                deduped: true,
                priority_tier: "autonomous".to_owned(),
            },
        ];
        assert!(new_not_reopen(&outcomes, 5_000).is_err());
    }

    #[test]
    fn a_resolution_summary_line_falsifies_the_absence_witness() {
        let lines = vec![r#"{"target":"x","fields":{"kind":"resolution_summary"}}"#.to_owned()];
        assert!(no_resolution_summary(&lines).is_err());
    }

    #[test]
    fn harvest_since_skips_the_pre_leg_prefix() {
        let dir = std::env::temp_dir().join("conductor-severity-harvest-slice");
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("agent-latest.jsonl.2026-08-21");
        std::fs::write(&path, "boot-a\nboot-b\nleg-1\nleg-2\n").expect("write");
        let got = harvest_since(&path, 2).expect("read");
        assert_eq!(got, vec!["leg-1".to_owned(), "leg-2".to_owned()]);
        std::fs::remove_file(&path).ok();
    }
}
