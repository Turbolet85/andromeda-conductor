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
    pair_canary_attempts(&refs, None)
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

// ---- the pairing across the emission instant -------------------------------------------------------
// The 2026-10-07 sixth series' three shapes around the scenario's emission instant, built from the
// recorded stamps and line kinds in the recorded file order, fields only. Where an outcome line and
// the next prompt share a stamp the outcome comes first, as on all four such pairs in the three
// recorded windows.

const D1_INSTANT_MS: i64 = 1_791_441_166_355;
const D2_INSTANT_MS: i64 = 1_791_441_771_853;
const D3_INSTANT_MS: i64 = 1_791_442_340_721;

fn tier_2_tick(t: &str) -> [serde_json::Value; 2] {
    [
        log(
            t,
            DIGEST_TICK,
            serde_json::json!({ "mode": "tier2", "cue_present": true }),
        ),
        log(
            t,
            DIGEST_ASSEMBLE,
            serde_json::json!({
                "mode": "tier2",
                "cue_kind": "error_rate_spike",
                "cue_priority_tier": "suggested",
            }),
        ),
    ]
}

fn deduped(t: &str) -> serde_json::Value {
    log(
        t,
        "interpretation.incident.created",
        serde_json::json!({ "created": false, "deduped": true }),
    )
}

fn paired_at(lines: &[serde_json::Value], emission_ms: i64) -> Vec<String> {
    let refs: Vec<&serde_json::Value> = lines.iter().collect();
    pair_canary_attempts(&refs, Some(emission_ms))
}

/// d1: the third canary storm ticked 4 ms before the instant and its prompt assembled 1 ms after it;
/// the scenario's own digest ticked 5 s later.
fn d1_shape() -> Vec<serde_json::Value> {
    let [tick, assemble] = cue_tick("2026-10-08T06:32:46.351Z", "tier1", "retry_storm");
    let [scenario, scenario_assemble] =
        cue_tick("2026-10-08T06:32:51.361Z", "tier1", "retry_storm");
    vec![
        tick,
        assemble,
        prompt("2026-10-08T06:32:46.356Z"),
        scenario,
        scenario_assemble,
        parse_ok("2026-10-08T06:32:52.134Z"),
        deduped("2026-10-08T06:32:52.168Z"),
        prompt("2026-10-08T06:32:52.168Z"),
        parse_ok("2026-10-08T06:32:58.062Z"),
        created("2026-10-08T06:32:58.120Z"),
    ]
}

/// d2: the tick 2 ms before the instant, the prompt 2 ms after it, and a tier-2 cue tick 730 ms
/// after it.
fn d2_shape() -> Vec<serde_json::Value> {
    let [tick, assemble] = cue_tick("2026-10-08T06:42:51.851Z", "tier1", "retry_storm");
    let [tier2, tier2_assemble] = tier_2_tick("2026-10-08T06:42:52.583Z");
    let [scenario, scenario_assemble] =
        cue_tick("2026-10-08T06:42:56.859Z", "tier1", "retry_storm");
    vec![
        tick,
        assemble,
        prompt("2026-10-08T06:42:51.855Z"),
        tier2,
        tier2_assemble,
        scenario,
        scenario_assemble,
        parse_ok("2026-10-08T06:42:58.178Z"),
        deduped("2026-10-08T06:42:58.211Z"),
        prompt("2026-10-08T06:42:58.211Z"),
        parse_ok("2026-10-08T06:43:04.085Z"),
        created("2026-10-08T06:43:04.143Z"),
        prompt("2026-10-08T06:43:04.143Z"),
    ]
}

/// d3: two storms with their inferences and one tier-2 cue tick before the instant, the third storm's
/// tick 11 ms after it.
fn d3_shape() -> Vec<serde_json::Value> {
    let [tick1, assemble1] = cue_tick("2026-10-08T06:49:20.714Z", "tier1", "retry_storm");
    let [tick2, assemble2] = cue_tick("2026-10-08T06:50:50.721Z", "tier1", "retry_storm");
    let [tier2, tier2_assemble] = tier_2_tick("2026-10-08T06:51:58.582Z");
    let [tick3, assemble3] = cue_tick("2026-10-08T06:52:20.732Z", "tier1", "retry_storm");
    let [scenario, scenario_assemble] =
        cue_tick("2026-10-08T06:52:25.725Z", "tier1", "retry_storm");
    vec![
        tick1,
        assemble1,
        prompt("2026-10-08T06:49:20.718Z"),
        parse_ok("2026-10-08T06:49:25.902Z"),
        created("2026-10-08T06:49:25.965Z"),
        tick2,
        assemble2,
        prompt("2026-10-08T06:50:50.725Z"),
        parse_ok("2026-10-08T06:50:56.484Z"),
        deduped("2026-10-08T06:50:56.511Z"),
        tier2,
        tier2_assemble,
        tick3,
        assemble3,
        prompt("2026-10-08T06:52:20.736Z"),
        scenario,
        scenario_assemble,
        parse_ok("2026-10-08T06:52:26.736Z"),
        deduped("2026-10-08T06:52:26.769Z"),
        prompt("2026-10-08T06:52:26.769Z"),
        parse_ok("2026-10-08T06:52:32.603Z"),
        created("2026-10-08T06:52:32.661Z"),
    ]
}

#[test]
fn a_canary_tick_just_before_the_instant_reads_the_inference_stamped_after_it_d1() {
    let out = paired_at(&d1_shape(), D1_INSTANT_MS);
    assert_eq!(out.len(), 2, "{out:?}");
    assert!(
        out[0].starts_with(&format!("{CANARY}{SURFACED} t=2026-10-08T06:32:46.351Z ")),
        "{out:?}"
    );
    assert!(
        out[0].ends_with(" parse=ok created=false deduped=true skip_reason=none"),
        "{out:?}"
    );
    assert_eq!(out[1], "canary other cue-bearing digests: 0 ()", "{out:?}");
}

#[test]
fn a_canary_tick_just_before_the_instant_reads_the_inference_stamped_after_it_d2() {
    // The tier-2 tick is stamped after the instant, so it stays out of the closing count.
    let out = paired_at(&d2_shape(), D2_INSTANT_MS);
    assert_eq!(out.len(), 2, "{out:?}");
    assert!(
        out[0].starts_with(&format!("{CANARY}{SURFACED} t=2026-10-08T06:42:51.851Z ")),
        "{out:?}"
    );
    assert!(
        out[0].ends_with(" parse=ok created=false deduped=true skip_reason=none"),
        "{out:?}"
    );
    assert_eq!(out[1], "canary other cue-bearing digests: 0 ()", "{out:?}");
}

#[test]
fn the_scenario_s_own_storm_digest_never_prints_as_a_canary() {
    for (shape, instant, scenario_tick) in [
        (d1_shape(), D1_INSTANT_MS, "t=2026-10-08T06:32:51.361Z "),
        (d2_shape(), D2_INSTANT_MS, "t=2026-10-08T06:42:56.859Z "),
    ] {
        let out = paired_at(&shape, instant);
        assert!(
            out.iter().all(|line| !line.contains(scenario_tick)),
            "{out:?}"
        );
        // The shape does carry that digest, parsed `ok` with an incident created: with no instant
        // every tick is selected and it prints.
        let whole = paired(&shape);
        assert!(
            whole.iter().any(|line| {
                line.starts_with(&format!("{CANARY}{SURFACED} {scenario_tick}"))
                    && line.ends_with(" parse=ok created=true deduped=false skip_reason=none")
            }),
            "{whole:?}"
        );
    }
}

#[test]
fn a_canary_tick_stamped_after_the_instant_gets_no_line_d3() {
    let out = paired_at(&d3_shape(), D3_INSTANT_MS);
    assert_eq!(out.len(), 3, "{out:?}");
    assert!(
        out[0].starts_with(&format!("{CANARY}{SURFACED} t=2026-10-08T06:49:20.714Z ")),
        "{out:?}"
    );
    assert!(
        out[0].ends_with(" parse=ok created=true deduped=false skip_reason=none"),
        "{out:?}"
    );
    assert!(
        out[1].starts_with(&format!("{CANARY}{SURFACED} t=2026-10-08T06:50:50.721Z ")),
        "{out:?}"
    );
    assert!(
        out[1].ends_with(" parse=ok created=false deduped=true skip_reason=none"),
        "{out:?}"
    );
    assert_eq!(
        out[2], "canary other cue-bearing digests: 1 (error_rate_spike)",
        "{out:?}"
    );
}

#[test]
fn a_tick_the_instant_cannot_place_is_never_selected() {
    // A clock with no date does not parse as a stamp; the second tick is stamped exactly at d1's
    // instant.
    let [unplaced, unplaced_assemble] = cue_tick("06:32:40.000", "tier1", "retry_storm");
    let [at_instant, at_instant_assemble] =
        cue_tick("2026-10-08T06:32:46.355Z", "tier1", "retry_storm");
    let lines = [
        unplaced,
        unplaced_assemble,
        prompt("2026-10-08T06:32:40.010Z"),
        parse_ok("2026-10-08T06:32:45.000Z"),
        at_instant,
        at_instant_assemble,
        prompt("2026-10-08T06:32:46.360Z"),
        parse_ok("2026-10-08T06:32:52.000Z"),
    ];
    let out = paired_at(&lines, D1_INSTANT_MS);
    assert_eq!(out, ["canary other cue-bearing digests: 0 ()"], "{out:?}");
    // With no instant the whole window is selected, as when the scenario never emitted.
    let whole = paired(&lines);
    assert_eq!(whole.len(), 3, "{whole:?}");
    assert!(
        whole[..2].iter().all(|line| line.starts_with(CANARY)),
        "{whole:?}"
    );
}

#[test]
fn the_pre_fix_window_read_the_same_shapes_as_a_pipeline_fault() {
    // What the sixth series' d1 and d2 captures printed for the third storm: the pairing was handed
    // only the lines stamped before the instant, so the canary's prompt was never read.
    for (shape, instant, tick) in [
        (d1_shape(), D1_INSTANT_MS, "t=2026-10-08T06:32:46.351Z "),
        (d2_shape(), D2_INSTANT_MS, "t=2026-10-08T06:42:51.851Z "),
    ] {
        let before: Vec<serde_json::Value> = shape
            .into_iter()
            .filter(|line| {
                line["timestamp"]
                    .as_str()
                    .and_then(real_model_common::iso_ms)
                    .is_some_and(|at| at < instant)
            })
            .collect();
        let out = paired(&before);
        assert!(
            out[0].starts_with(&format!("{CANARY}{PIPELINE_FAULT} {tick}")),
            "{out:?}"
        );
        assert!(
            out[0].ends_with(" parse=none created=none deduped=none skip_reason=none"),
            "{out:?}"
        );
    }
}

#[test]
fn the_capture_prints_pulse_s_no_incident_outcome() {
    let capture = include_str!("../real_model_live.rs");
    assert!(capture.contains("\"interpretation.incident.skipped\""));
    assert!(capture.contains("canary_attempts(&window, emission_ms)"));
}
