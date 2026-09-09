//! Live-leg evidence harvest for the restart-suppression family (P-015/P-016/P-057).
//!
//! Neither of the family's authored read-back checks can grade under deterministic L4 (an open
//! incident's report renders the degraded branch, which names no cue kind), so the live proof
//! grades on Pulse's own tracing lines, exactly as the fingerprint and statistical-anomaly
//! families' harvests do. The witnesses, measured live 2026-08-18 (run `2026-08-18T21-40-46-519`,
//! SUT at HEAD `efabe8e`) and pinned VERBATIM below:
//!
//! - P-015: `triage.pattern.restart_emit` — the RestartDetector dispatching a gap>threshold
//!   restart event on the resume span's arrival (`pattern/detector.rs:144`); the leg carried two
//!   (gap 25s and 26s).
//! - P-016 suppressed half: `triage.cue.suppression_check` lines with `restart_window_active` and
//!   `suppression_bypassed: false` at `persistence_seconds < 30` (measured at 10..13), with NO
//!   matching `triage.cue.emit` — a dropped cue never reaches emit. The tick line's
//!   `cues_suppressed`/`bypass_triggered` counters are REDACTED on the live surface
//!   (`"<redacted>"` — Pulse's default-deny field allowlist predates the chunk-#63 counters;
//!   measured), so drop evidence rides the check-line + the emit absence, never a counter.
//! - P-016 surgical half: the same service's cues PLAIN-KEPT (emitted, not bypassed) once
//!   `persistence_seconds >= 30` (measured at 38..) — the drop→keep crossing. NOTE the measured
//!   semantics: `persistence_seconds` IS the service's cumulative sample count
//!   (`cue/evaluate.rs:55`), not spike duration; the scenario paces ~1 span/s so samples track
//!   seconds, and the crossing exists only in the service's first ~50 samples.
//! - P-057 absolute arm: `triage.cue.suppression_bypass` with `bypass_reason: "absolute"` (Pulse's
//!   own label; ten fired, persistence 20..29) plus a kept young cue carrying the bypass flag.
//! - The relative arm is UNREACHABLE for error cues at the SUT's shipped constants and carries no
//!   predicate: alpha_short/alpha_long = (1/30)/(1/300) = 10 equals the bypass multiplier, and the
//!   0.01 baseline floor caps the transient peak at ~9.7x (three consecutive errors) — recorded as
//!   next-Pulse-visit intake, never graded here.
//! - The corpus-keeper: an `error_rate_spike` cue at priority `autonomous` (measured mag 5.91 at
//!   confidence 0.90, persistence 90) — the Tier-1 incident former.
//!
//! TEST-ONLY affordance (the storm/baseline-harvest precedent): nothing here is wired into the run
//! path and nothing harvested reaches a Conductor artifact. On Windows the capture is
//! `{data_dir}/logs/agent-latest.jsonl.<date>`, sliced to the leg window by a pre-leg line count.

/// One `triage.pattern.restart_emit` observation.
#[derive(Debug, Clone, PartialEq)]
struct RestartEmitted {
    cue_kind: String,
    gap_seconds: u64,
}

/// One per-cue `triage.cue.suppression_check` decision observation.
#[derive(Debug, Clone, PartialEq)]
struct SuppressionCheck {
    cue_kind: String,
    persistence_seconds: u64,
    restart_window_active: bool,
    suppression_bypassed: bool,
}

/// One `triage.cue.suppression_bypass` trigger observation (a cue KEPT because bypass fired).
#[derive(Debug, Clone, PartialEq)]
struct BypassTriggered {
    cue_kind: String,
    bypass_reason: String,
}

/// One `triage.cue.emit` observation (the kept-cue surface, as in `baseline_harvest.rs`).
#[derive(Debug, Clone, PartialEq)]
struct CueEmitted {
    kind: String,
    priority: String,
    persistence_seconds: u64,
    absolute_value: f64,
    magnitude: f64,
    confidence: f64,
    suppression_bypassed: bool,
}

fn parsed(lines: &[String]) -> impl Iterator<Item = serde_json::Value> + '_ {
    lines
        .iter()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
}

fn is_target<'v>(v: &'v serde_json::Value, target: &str) -> Option<&'v serde_json::Value> {
    (v.get("target").and_then(|t| t.as_str()) == Some(target)).then(|| v.get("fields"))?
}

/// A line that does not parse, is not this target, or is missing a field is SKIPPED rather than
/// fatal — a harvest is other-process output (the verdict/error wall applied to evidence).
fn parse_restart_emitted(lines: &[String]) -> Vec<RestartEmitted> {
    parsed(lines)
        .filter_map(|v| {
            let f = is_target(&v, "triage.pattern.restart_emit")?.clone();
            Some(RestartEmitted {
                cue_kind: f.get("cue_kind")?.as_str()?.to_owned(),
                gap_seconds: f.get("gap_seconds")?.as_u64()?,
            })
        })
        .collect()
}

fn parse_suppression_checks(lines: &[String]) -> Vec<SuppressionCheck> {
    parsed(lines)
        .filter_map(|v| {
            let f = is_target(&v, "triage.cue.suppression_check")?.clone();
            Some(SuppressionCheck {
                cue_kind: f.get("cue_kind")?.as_str()?.to_owned(),
                persistence_seconds: f.get("persistence_seconds")?.as_u64()?,
                restart_window_active: f.get("restart_window_active")?.as_bool()?,
                suppression_bypassed: f.get("suppression_bypassed")?.as_bool()?,
            })
        })
        .collect()
}

fn parse_bypass_triggers(lines: &[String]) -> Vec<BypassTriggered> {
    parsed(lines)
        .filter_map(|v| {
            let f = is_target(&v, "triage.cue.suppression_bypass")?.clone();
            Some(BypassTriggered {
                cue_kind: f.get("cue_kind")?.as_str()?.to_owned(),
                bypass_reason: f.get("bypass_reason")?.as_str()?.to_owned(),
            })
        })
        .collect()
}

fn parse_cue_emitted(lines: &[String]) -> Vec<CueEmitted> {
    parsed(lines)
        .filter_map(|v| {
            let f = is_target(&v, "triage.cue.emit")?.clone();
            Some(CueEmitted {
                kind: f.get("kind")?.as_str()?.to_owned(),
                priority: f.get("priority")?.as_str()?.to_owned(),
                persistence_seconds: f.get("persistence_seconds")?.as_u64()?,
                absolute_value: f.get("absolute_value")?.as_f64()?,
                magnitude: f.get("magnitude")?.as_f64()?,
                confidence: f.get("confidence")?.as_f64()?,
                suppression_bypassed: f.get("suppression_bypassed")?.as_bool()?,
            })
        })
        .collect()
}

/// The suppression persistence cutoff, transcribed from the SUT
/// (`DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SECONDS`, `cue/thresholds.rs:143` at HEAD `efabe8e`).
const PERSISTENCE_CUTOFF: u64 = 30;

/// The restart gap threshold, transcribed from the SUT
/// (`DEFAULT_RESTART_GAP_THRESHOLD_SECONDS`, `cue/thresholds.rs:130` at HEAD `efabe8e`).
const GAP_THRESHOLD_SECONDS: u64 = 20;

/// P-015: both gap/resume legs dispatched a restart event, each with a gap over the threshold.
fn restart_witness(events: &[RestartEmitted]) -> Result<(), String> {
    let restarts: Vec<_> = events
        .iter()
        .filter(|e| e.cue_kind == "restart_event")
        .collect();
    if restarts.len() < 2 {
        return Err(format!(
            "{} restart_event emissions in the window; the scenario drives 2",
            restarts.len()
        ));
    }
    match restarts
        .iter()
        .find(|e| e.gap_seconds <= GAP_THRESHOLD_SECONDS)
    {
        Some(e) => Err(format!(
            "restart event with gap {}s, not over the {GAP_THRESHOLD_SECONDS}s threshold",
            e.gap_seconds
        )),
        None => Ok(()),
    }
}

/// P-016 suppressed half: drop evidence — an in-window, not-bypassed check line below the cutoff
/// with no leaked emit (a dropped cue never reaches `cue.emit`). The tick counters cannot
/// corroborate: they read `"<redacted>"` on the live surface (see the header).
fn suppressed_witness(checks: &[SuppressionCheck], emits: &[CueEmitted]) -> Result<(), String> {
    let droppable = checks.iter().any(|c| {
        c.cue_kind == "error_rate_spike"
            && c.restart_window_active
            && !c.suppression_bypassed
            && c.persistence_seconds < PERSISTENCE_CUTOFF
    });
    if !droppable {
        return Err(
            "no in-window, not-bypassed suppression_check below the persistence cutoff".to_owned(),
        );
    }
    match emits.iter().find(|c| {
        c.kind == "error_rate_spike"
            && !c.suppression_bypassed
            && c.persistence_seconds < PERSISTENCE_CUTOFF
    }) {
        Some(c) => Err(format!(
            "a not-bypassed error_rate_spike cue leaked to emit at persistence {} — it should have been dropped",
            c.persistence_seconds
        )),
        None => Ok(()),
    }
}

/// P-057 absolute arm: a bypass trigger with Pulse's own `absolute` reason label, plus the kept
/// young cue it saved.
fn absolute_bypass_witness(
    triggers: &[BypassTriggered],
    emits: &[CueEmitted],
) -> Result<(), String> {
    if !triggers
        .iter()
        .any(|t| t.cue_kind == "error_rate_spike" && t.bypass_reason == "absolute")
    {
        return Err("no suppression_bypass trigger with reason=absolute".to_owned());
    }
    if emits.iter().any(|c| {
        c.kind == "error_rate_spike"
            && c.suppression_bypassed
            && c.persistence_seconds < PERSISTENCE_CUTOFF
    }) {
        Ok(())
    } else {
        Err("no kept error_rate_spike cue below the cutoff carrying the bypass flag".to_owned())
    }
}

/// P-016 surgical half: the same service's spike dropped below the cutoff and PLAIN-kept (not
/// bypassed) at or above it — the drop→keep crossing. The drop side is the suppressed witness's
/// check-line; the keep side must reach `cue.emit`.
fn surgical_crossing_witness(
    checks: &[SuppressionCheck],
    emits: &[CueEmitted],
) -> Result<(), String> {
    let dropped_young = checks.iter().any(|c| {
        c.cue_kind == "error_rate_spike"
            && c.restart_window_active
            && !c.suppression_bypassed
            && c.persistence_seconds < PERSISTENCE_CUTOFF
    });
    if !dropped_young {
        return Err("no dropped-side evidence below the persistence cutoff".to_owned());
    }
    if emits.iter().any(|c| {
        c.kind == "error_rate_spike"
            && !c.suppression_bypassed
            && c.persistence_seconds >= PERSISTENCE_CUTOFF
    }) {
        Ok(())
    } else {
        Err("no plain-kept error_rate_spike cue at or above the persistence cutoff".to_owned())
    }
}

/// The corpus-keeper: an autonomous-tier spike cue (Tier-1 forms the incident from it alone).
fn autonomous_witness(emits: &[CueEmitted]) -> Result<(), String> {
    if emits
        .iter()
        .any(|c| c.kind == "error_rate_spike" && c.priority == "autonomous")
    {
        Ok(())
    } else {
        Err("no autonomous-tier error_rate_spike cue in the harvest window".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------------------------
    // VERBATIM leg captures — run `2026-08-18T21-40-46-519`, fresh data dir, SUT HEAD `efabe8e`
    // (pulse-app.exe built 2026-08-17 23:04). Six lines, one per witness class, byte-identical to
    // `{data_dir}/logs/agent-latest.jsonl.2026-08-18` sliced past the pre-leg line count.
    // ------------------------------------------------------------------------------------------
    const LEG_RESTART_EMIT_1: &str = r#"{"fields":{"cue_kind":"restart_event","deployment.environment":"production","gap_seconds":25,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"","target":"triage.pattern.restart_emit","timestamp":"2026-08-18T21:42:05.653Z"}"#;
    const LEG_RESTART_EMIT_2: &str = r#"{"fields":{"cue_kind":"restart_event","deployment.environment":"production","gap_seconds":26,"service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"","target":"triage.pattern.restart_emit","timestamp":"2026-08-18T21:43:15.126Z"}"#;
    const LEG_CHECK_DROP: &str = r#"{"fields":{"bypass_reason":"none","cue_kind":"error_rate_spike","deployment.environment":"production","persistence_seconds":10,"restart_window_active":true,"service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":false},"level":"INFO","message":"","target":"triage.cue.suppression_check","timestamp":"2026-08-18T21:42:08.225Z"}"#;
    const LEG_BYPASS_ABSOLUTE: &str = r#"{"fields":{"bypass_reason":"absolute","cue_kind":"error_rate_spike","deployment.environment":"production","service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"suppression bypassed by dual-condition","target":"triage.cue.suppression_bypass","timestamp":"2026-08-18T21:42:18.226Z"}"#;
    const LEG_EMIT_YOUNG_BYPASSED: &str = r#"{"fields":{"absolute_value":0.05703347981878583,"confidence":0.2,"deployment.environment":"production","kind":"error_rate_spike","magnitude":5.703347981878583,"persistence_seconds":20,"priority":"curious","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":true},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-18T21:42:18.226Z"}"#;
    const LEG_EMIT_KEPT_PLAIN: &str = r#"{"fields":{"absolute_value":0.04972557186849345,"confidence":0.38,"deployment.environment":"production","kind":"error_rate_spike","magnitude":4.972557186849345,"persistence_seconds":38,"priority":"curious","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":false},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-18T21:42:36.226Z"}"#;
    const LEG_EMIT_AUTONOMOUS: &str = r#"{"fields":{"absolute_value":0.1051559857609772,"confidence":0.9,"deployment.environment":"production","kind":"error_rate_spike","magnitude":5.911348912538991,"persistence_seconds":90,"priority":"autonomous","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":true},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-18T21:43:54.226Z"}"#;

    fn pinned_leg_lines() -> Vec<String> {
        [
            LEG_RESTART_EMIT_1,
            LEG_RESTART_EMIT_2,
            LEG_CHECK_DROP,
            LEG_BYPASS_ABSOLUTE,
            LEG_EMIT_YOUNG_BYPASSED,
            LEG_EMIT_KEPT_PLAIN,
            LEG_EMIT_AUTONOMOUS,
        ]
        .map(str::to_owned)
        .to_vec()
    }

    fn other_line() -> String {
        r#"{"fields":{"rows_ingested":15},"level":"INFO","message":"","target":"buffer.tick","timestamp":"2026-08-18T00:00:00.000Z"}"#.to_owned()
    }

    /// Synthetic variant of the captured `cue.emit` shape, for the negative cases the leg
    /// (correctly) never produced.
    fn cue_line(priority: &str, persistence: u64, abs: f64, mag: f64, bypassed: bool) -> String {
        format!(
            r#"{{"fields":{{"absolute_value":{abs},"confidence":1.0,"deployment.environment":"production","kind":"error_rate_spike","magnitude":{mag},"persistence_seconds":{persistence},"priority":"{priority}","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":{bypassed}}},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-18T00:00:00.000Z"}}"#
        )
    }

    #[test]
    fn the_pinned_leg_lines_parse_and_satisfy_every_witness() {
        let lines = {
            let mut l = pinned_leg_lines();
            l.insert(0, other_line());
            l
        };
        let restarts = parse_restart_emitted(&lines);
        let checks = parse_suppression_checks(&lines);
        let triggers = parse_bypass_triggers(&lines);
        let emits = parse_cue_emitted(&lines);

        assert_eq!(
            restarts.len(),
            2,
            "the leg carried exactly two restart events"
        );
        assert_eq!(restart_witness(&restarts), Ok(()));
        assert_eq!(suppressed_witness(&checks, &emits), Ok(()));
        assert_eq!(absolute_bypass_witness(&triggers, &emits), Ok(()));
        assert_eq!(surgical_crossing_witness(&checks, &emits), Ok(()));
        assert_eq!(autonomous_witness(&emits), Ok(()));
    }

    #[test]
    fn the_pinned_observations_carry_the_measured_values() {
        let lines = pinned_leg_lines();
        assert_eq!(
            parse_restart_emitted(&lines)
                .iter()
                .map(|e| e.gap_seconds)
                .collect::<Vec<_>>(),
            vec![25, 26],
            "both gaps over the 20s threshold, as driven (25s declared + jitter)"
        );
        let checks = parse_suppression_checks(&lines);
        assert_eq!(
            checks,
            vec![SuppressionCheck {
                cue_kind: "error_rate_spike".to_owned(),
                persistence_seconds: 10,
                restart_window_active: true,
                suppression_bypassed: false,
            }],
            "the first dropped cue fired at the service's 10th sample — the warm-up floor"
        );
        let auto = parse_cue_emitted(&lines)
            .into_iter()
            .find(|c| c.priority == "autonomous")
            .expect("the keeper cue is pinned");
        assert!(auto.magnitude >= 5.0 && auto.confidence >= 0.9 && auto.persistence_seconds >= 30);
    }

    #[test]
    fn the_restart_witness_needs_two_over_threshold_crossings() {
        let one = parse_restart_emitted(&[LEG_RESTART_EMIT_1.to_owned()]);
        assert!(
            restart_witness(&one).is_err(),
            "one restart is not the scenario's pair"
        );

        let shallow = parse_restart_emitted(&[
            LEG_RESTART_EMIT_1.to_owned(),
            LEG_RESTART_EMIT_2.replace("\"gap_seconds\":26", "\"gap_seconds\":20"),
        ]);
        assert!(
            restart_witness(&shallow).is_err(),
            "a 20s gap does not cross the >20s threshold"
        );
    }

    #[test]
    fn the_suppressed_witness_fails_on_a_leaked_young_emit_or_missing_drop_evidence() {
        let checks = parse_suppression_checks(&[LEG_CHECK_DROP.to_owned()]);
        let leaked = parse_cue_emitted(&[cue_line("curious", 12, 0.033, 3.3, false)]);
        assert!(
            suppressed_witness(&checks, &leaked).is_err(),
            "a young not-bypassed cue reaching emit means nothing was suppressed"
        );
        let clean = parse_cue_emitted(&[LEG_EMIT_KEPT_PLAIN.to_owned()]);
        assert!(
            suppressed_witness(&[], &clean).is_err(),
            "no check-line drop evidence, no suppressed witness"
        );
    }

    #[test]
    fn the_absolute_arm_requires_the_trigger_and_the_kept_young_cue() {
        let triggers = parse_bypass_triggers(&[LEG_BYPASS_ABSOLUTE.to_owned()]);
        let mature_only = parse_cue_emitted(&[LEG_EMIT_AUTONOMOUS.to_owned()]);
        assert!(
            absolute_bypass_witness(&triggers, &mature_only).is_err(),
            "a mature-sample cue is kept via persistence, not attributably via bypass"
        );
        let young = parse_cue_emitted(&[LEG_EMIT_YOUNG_BYPASSED.to_owned()]);
        assert!(
            absolute_bypass_witness(&[], &young).is_err(),
            "no trigger line, no arm witness"
        );
    }

    #[test]
    fn the_surgical_crossing_needs_the_plain_keep_not_a_bypassed_one() {
        let checks = parse_suppression_checks(&[LEG_CHECK_DROP.to_owned()]);
        let bypass_kept_only = parse_cue_emitted(&[cue_line("curious", 41, 0.086, 8.6, true)]);
        assert!(
            surgical_crossing_witness(&checks, &bypass_kept_only).is_err(),
            "a bypassed keep does not witness the persistence arm"
        );
        assert!(
            surgical_crossing_witness(&[], &parse_cue_emitted(&[LEG_EMIT_KEPT_PLAIN.to_owned()]))
                .is_err(),
            "no drop side, no crossing"
        );
    }

    #[test]
    fn the_corpus_keeper_needs_the_autonomous_tier() {
        let below = parse_cue_emitted(&[LEG_EMIT_KEPT_PLAIN.to_owned()]);
        assert!(
            autonomous_witness(&below).is_err(),
            "a curious cue forms no Tier-1 incident"
        );
    }
}
