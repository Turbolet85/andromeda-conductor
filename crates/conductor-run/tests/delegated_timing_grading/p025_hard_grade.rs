//! The P-025 hard grade under the measurement contract's rule, over synthetic lines.

use super::*;

// ---- The P-025 hard grade, proven over SYNTHETIC lines before the leg fires ----
// The rule is `contracts/pulse-p025-measurement-contract.md` §The grading rule. Lines are in
// Pulse's on-disk shape at HEAD `226554a`: the hue leaf's `duration_ms` + `severity_tier`, and
// `interpretation.incident.created`'s allowlisted `created` / `deduped` / `severity` /
// `priority_tier`.

fn synthetic_hue_tier(duration_ms: f64, tier: &str, at: &str) -> String {
    format!(
        r#"{{"fields":{{"deployment.environment":"production","duration_ms":{duration_ms},"service.name":"com.andromeda.pulse","severity_tier":"{tier}"}},"level":"INFO","message":"constellation hue update latency recorded","target":"metric.constellation.hue_update_ms","timestamp":"{at}"}}"#
    )
}

fn synthetic_incident_created(created: bool, at: &str) -> String {
    let deduped = !created;
    format!(
        r#"{{"fields":{{"created":{created},"deduped":{deduped},"deployment.environment":"production","priority_tier":"autonomous","service.name":"com.andromeda.pulse","severity":"error"}},"level":"INFO","message":"incident producer outcome","target":"{INCIDENT_CREATED_TARGET}","timestamp":"{at}"}}"#
    )
}

/// A leg window from 10:01:00.000Z (phase-2 start) to 10:04:00.000Z (`scenario.run` close).
fn synthetic_window() -> (i64, i64) {
    let at = |s: &str| timestamp_ms(&synthetic_hue_tier(0.0, "none", s)).expect("parses");
    (
        at("2026-09-29T10:01:00.000Z"),
        at("2026-09-29T10:04:00.000Z"),
    )
}

/// Arm (i): in-window samples at or under the budget grade `Ok` with the WORST of them, and
/// the inclusive boundary holds at exactly 2000ms.
#[test]
fn p025_in_window_samples_within_budget_grade_ok_with_the_worst() {
    let bound = bounds()[0];
    let lines = vec![
        synthetic_hue_tier(640.0, "autonomous", "2026-09-29T10:01:12.000Z"),
        synthetic_hue_tier(2_000.0, "autonomous", "2026-09-29T10:02:30.000Z"),
        synthetic_hue_tier(510.0, "suggested", "2026-09-29T10:03:40.000Z"),
    ];
    assert_eq!(
        grade_in_window(&lines, &bound, synthetic_window()),
        Ok(2_000.0)
    );
}

/// Arm (ii): one in-window sample over the budget is a hard `Err` carrying the value — a
/// breach is a measurement, never rescued by a fast neighbour.
#[test]
fn p025_an_in_window_sample_over_budget_is_a_hard_err_carrying_its_value() {
    let bound = bounds()[0];
    let lines = vec![
        synthetic_hue_tier(700.0, "autonomous", "2026-09-29T10:01:12.000Z"),
        synthetic_hue_tier(2_400.0, "autonomous", "2026-09-29T10:02:30.000Z"),
    ];
    let err = grade_in_window(&lines, &bound, synthetic_window())
        .expect_err("2400ms is over the 2000ms budget");
    assert!(err.contains("2400"), "the reason carries the value: {err}");
    assert!(err.contains("P-025"), "the reason names the bound: {err}");
}

/// Arm (iii): a window holding no hue sample is UNGRADED, never met — even when the capture
/// holds samples outside it — and the reason names P-025 and the window.
#[test]
fn p025_an_empty_leg_window_is_ungraded_never_met() {
    let bound = bounds()[0];
    let window = synthetic_window();
    let lines = vec![
        synthetic_hue_tier(900.0, "autonomous", "2026-09-29T10:00:10.000Z"),
        synthetic_hue_tier(900.0, "none", "2026-09-29T10:06:00.000Z"),
    ];
    let err = grade_in_window(&lines, &bound, window).expect_err("an empty window must not pass");
    assert!(err.contains("UNGRADED"), "the reason says ungraded: {err}");
    assert!(err.contains("P-025"), "the reason names the bound: {err}");
    assert!(
        err.contains(&window.0.to_string()) && err.contains(&window.1.to_string()),
        "the reason names the window: {err}"
    );
}

/// Arm (iv): a stale remembered-tier sample (clamped to exactly 60000ms, `none`) that fires when
/// a hidden service reappears lands before phase-2 start, so the window excludes it and it
/// cannot breach the grade.
#[test]
fn p025_a_stale_sixty_second_sample_before_phase_two_is_excluded() {
    let bound = bounds()[0];
    let lines = vec![
        synthetic_hue_tier(60_000.0, "none", "2026-09-29T10:00:59.999Z"),
        synthetic_hue_tier(820.0, "autonomous", "2026-09-29T10:01:40.000Z"),
    ];
    assert_eq!(
        grade_in_window(&lines, &bound, synthetic_window()),
        Ok(820.0)
    );
    // The control: the same stale sample one millisecond later is inside and breaches.
    let inside = vec![synthetic_hue_tier(
        60_000.0,
        "none",
        "2026-09-29T10:01:00.000Z",
    )];
    assert!(grade_in_window(&inside, &bound, synthetic_window()).is_err());
}

/// Arm (v): the anchor is the NEAREST preceding `created=true` line; a `created=false` dedupe
/// between them opens nothing and is skipped, and no preceding creation is `None`.
#[test]
fn p025_the_anchor_is_the_nearest_preceding_fresh_incident_not_a_dedupe() {
    let bound = bounds()[0];
    let lines = vec![
        synthetic_incident_created(true, "2026-09-29T10:00:05.000Z"),
        synthetic_incident_created(true, "2026-09-29T10:01:30.000Z"),
        synthetic_incident_created(false, "2026-09-29T10:01:30.900Z"),
        // Start instant 10:01:31.550 − 1.200 = 10:01:30.350, 350ms after the fresh incident.
        synthetic_hue_tier(1_200.0, "autonomous", "2026-09-29T10:01:31.550Z"),
        synthetic_hue_tier(0.0, "none", "2026-09-29T10:03:50.000Z"),
    ];
    let rises = tiered_hue_samples_in_window(&lines, &bound, synthetic_window());
    assert_eq!(rises.len(), 1, "the `none` fall is not a rise: {rises:?}");
    // Anchoring on the dedupe would read 550ms; on the earlier fresh incident, 85350ms.
    let error = incident_anchor_error_ms(&rises[0], &lines).expect("a fresh incident precedes");
    assert_eq!(error, 350.0);
    assert!(error <= P025_ANCHOR_TOLERANCE_MS);

    let only_dedupe = vec![synthetic_incident_created(
        false,
        "2026-09-29T10:01:30.900Z",
    )];
    assert_eq!(incident_anchor_error_ms(&rises[0], &only_dedupe), None);
}
