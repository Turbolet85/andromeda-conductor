//! The P-025 window selection and the retired instrument's tick-offset pin, over synthetic lines.

use super::*;

// ---- The P-025 window selection + the RETIRED instrument's mechanism pin ----
// The helpers below are exercised against SYNTHETIC lines in Pulse's on-disk shape, so the
// window selection (still the grade's attribution) and the retired tick arithmetic (Pulse HEAD
// `83d4060`) are pinned independently of any leg.

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

/// THE RETIRED INSTRUMENT'S MECHANISM PIN (Pulse HEAD `83d4060`). `last_seen_unix_nano` is written
/// only by the 15s lifecycle tick, so that leaf's reported duration was its own offset to the
/// preceding tick — not a render latency. This property is why the retired instrument could not
/// grade the ≤2s budget, and asserting it is what turned a 36s reading from an unexplained number
/// into a verified consequence.
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
