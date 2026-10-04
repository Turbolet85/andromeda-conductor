//! The capture's canary pairing over Pulse log lines.

use crate::*;

// ---- the capture's canary pairing ------------------------------------------------------------------
// Pulse log lines in the shapes measured at the 2026-09-29 series, fields only.

fn log(t: &str, target: &str, fields: serde_json::Value) -> serde_json::Value {
    serde_json::json!({ "timestamp": t, "target": target, "fields": fields })
}

fn cue_tick(t: &str, mode: &str, kind: &str) -> [serde_json::Value; 2] {
    [
        log(
            t,
            DIGEST_TICK,
            serde_json::json!({ "mode": mode, "cue_present": true }),
        ),
        log(
            t,
            DIGEST_ASSEMBLE,
            serde_json::json!({ "mode": mode, "cue_kind": kind, "cue_priority_tier": "autonomous" }),
        ),
    ]
}

fn prompt(t: &str) -> serde_json::Value {
    log(
        t,
        "interpretation.prompt.assemble",
        serde_json::json!({ "prompt_version": "v2.2" }),
    )
}

fn parse_ok(t: &str) -> serde_json::Value {
    log(
        t,
        "interpretation.json.parse",
        serde_json::json!({ "parse_outcome": "ok" }),
    )
}

fn created(t: &str) -> serde_json::Value {
    log(
        t,
        "interpretation.incident.created",
        serde_json::json!({ "created": true, "deduped": false }),
    )
}

fn paired(lines: &[serde_json::Value]) -> Vec<String> {
    let refs: Vec<&serde_json::Value> = lines.iter().collect();
    pair_canary_attempts(&refs)
}

#[test]
fn a_tier_2_tick_before_the_parse_does_not_turn_a_dismissal_into_a_fault() {
    // b2 storm 1: its prompt assembled at once, a tier-2 error-rate tick came 3 s later, and its parse
    // `ok` followed with no incident outcome.
    let [tick, assemble] = cue_tick("17:20:21.152", "tier1", "retry_storm");
    let [tier2, tier2_assemble] = cue_tick("17:20:24.244", "tier2", "error_rate_spike");
    let lines = [
        tick,
        assemble,
        prompt("17:20:21.182"),
        tier2,
        tier2_assemble,
        parse_ok("17:20:28.180"),
        prompt("17:20:28.180"),
        parse_ok("17:20:35.510"),
    ];
    let out = paired(&lines);
    assert!(
        out[0].starts_with(&format!("{CANARY}{DISMISSED} ")),
        "{out:?}"
    );
    assert_eq!(
        out[1],
        "canary other cue-bearing digests: 1 (error_rate_spike)"
    );
}

#[test]
fn a_tier_2_tick_before_the_parse_does_not_turn_a_surfacing_into_a_fault() {
    // b1 storm 1: a tier-2 tick 0.95 s after the canary's, then its parse `ok` and an incident.
    let [tick, assemble] = cue_tick("17:02:13.285", "tier1", "retry_storm");
    let [tier2, tier2_assemble] = cue_tick("17:02:14.232", "tier2", "error_rate_spike");
    let lines = [
        tick,
        assemble,
        prompt("17:02:13.296"),
        tier2,
        tier2_assemble,
        parse_ok("17:02:18.030"),
        created("17:02:18.058"),
        prompt("17:02:18.060"),
    ];
    assert!(paired(&lines)[0].starts_with(&format!("{CANARY}{SURFACED} ")));
}

#[test]
fn an_earlier_digest_s_outcome_is_never_the_canary_s() {
    // The canary's digest queued behind a running inference: that inference's parse and incident land
    // after the canary's tick and before the canary's own prompt.
    let [tick, assemble] = cue_tick("16:00:10.000", "tier1", "retry_storm");
    let lines = [
        prompt("16:00:08.000"),
        tick,
        assemble,
        parse_ok("16:00:12.000"),
        created("16:00:12.020"),
        prompt("16:00:12.030"),
        parse_ok("16:00:16.000"),
    ];
    assert!(paired(&lines)[0].starts_with(&format!("{CANARY}{DISMISSED} ")));
}

#[test]
fn an_inference_error_or_a_skip_is_a_pipeline_fault() {
    // a1's first fire: every inference errored `model_not_configured`.
    let [tick, assemble] = cue_tick("15:57:36.798", "tier1", "retry_storm");
    let errored = [
        tick.clone(),
        assemble.clone(),
        prompt("15:57:36.810"),
        log(
            "15:57:36.810",
            "interpretation.inference.error",
            serde_json::json!({ "error_category": "model_not_configured" }),
        ),
    ];
    assert!(paired(&errored)[0].starts_with(&format!("{CANARY}{PIPELINE_FAULT} ")));
    // A digest skipped in backoff assembles no prompt at all.
    let skipped = [
        tick,
        assemble,
        log(
            "15:57:40.000",
            "interpretation.inference.skipped",
            serde_json::json!({ "reason": "backoff_active" }),
        ),
    ];
    assert!(paired(&skipped)[0].starts_with(&format!("{CANARY}{PIPELINE_FAULT} ")));
}

#[test]
fn the_next_canary_storm_s_prompt_is_never_the_previous_one_s() {
    // Storm 1's digest never reached inference; storm 2's did. Storm 1 is a fault, never storm 2's parse.
    let [tick1, assemble1] = cue_tick("17:00:00.000", "tier1", "retry_storm");
    let [tick2, assemble2] = cue_tick("17:01:30.000", "tier1", "retry_storm");
    let lines = [
        tick1,
        assemble1,
        tick2,
        assemble2,
        prompt("17:01:30.010"),
        parse_ok("17:01:35.000"),
    ];
    let out = paired(&lines);
    assert!(
        out[0].starts_with(&format!("{CANARY}{PIPELINE_FAULT} ")),
        "{out:?}"
    );
    assert!(
        out[1].starts_with(&format!("{CANARY}{DISMISSED} ")),
        "{out:?}"
    );
}

#[test]
fn a_dismissal_carries_pulse_s_stated_skip_reason() {
    // From Pulse `a2addb3` every parsed generation logs one of created / deduped / skipped.
    let [tick, assemble] = cue_tick("18:00:00.000", "tier1", "retry_storm");
    let lines = [
        tick,
        assemble,
        prompt("18:00:00.010"),
        parse_ok("18:00:05.000"),
        log(
            "18:00:05.010",
            "interpretation.incident.skipped",
            serde_json::json!({
                "skip_reason": "decision_dismiss",
                "decision": "dismiss",
                "severity": "low",
                "digest_kind": "cue",
            }),
        ),
        prompt("18:00:06.000"),
    ];
    let out = paired(&lines);
    assert!(
        out[0].starts_with(&format!("{CANARY}{DISMISSED} ")),
        "{out:?}"
    );
    assert!(out[0].ends_with(" skip_reason=decision_dismiss"), "{out:?}");
}

#[test]
fn a_surfacing_and_the_pre_fix_dismissal_carry_no_skip_reason() {
    let [tick, assemble] = cue_tick("18:10:00.000", "tier1", "retry_storm");
    let surfaced = [
        tick.clone(),
        assemble.clone(),
        prompt("18:10:00.010"),
        parse_ok("18:10:05.000"),
        created("18:10:05.010"),
        prompt("18:10:06.000"),
    ];
    let out = paired(&surfaced);
    assert!(
        out[0].starts_with(&format!("{CANARY}{SURFACED} ")),
        "{out:?}"
    );
    assert!(out[0].ends_with(" skip_reason=none"), "{out:?}");
    // Before `a2addb3` Pulse logged no outcome line for a dismissal.
    let pre_fix = [
        tick,
        assemble,
        prompt("18:10:00.010"),
        parse_ok("18:10:05.000"),
        prompt("18:10:06.000"),
    ];
    let out = paired(&pre_fix);
    assert!(
        out[0].starts_with(&format!("{CANARY}{DISMISSED} ")),
        "{out:?}"
    );
    assert!(out[0].ends_with(" skip_reason=none"), "{out:?}");
}

#[test]
fn the_capture_prints_pulse_s_no_incident_outcome() {
    assert!(include_str!("../real_model_live.rs").contains("\"interpretation.incident.skipped\""));
}
