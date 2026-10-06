//! The capture's host-path mask and its attribution sweep window.

use crate::*;

// ---- the host-path mask ---------------------------------------------------------------------------
// The forms are BUILT, never spelled: this file's hygiene gate greps it for exactly these shapes.

fn drive(tail: &str) -> String {
    format!("{}{}{}{tail}", 'D', ':', '\\')
}

#[test]
fn the_mask_covers_every_form_redact_value_misses() {
    let long_path = format!("{}{}", "\\\\?\\", drive("data"));
    let msys = format!("/{}/dev/pulse", 'd');
    let cases = [
        (format!("see `{}`", drive("data")), "see `<host-path>`"),
        (format!("({})", drive("data")), "(<host-path>)"),
        (
            format!("path={} next", drive("data")),
            "path=<host-path> next",
        ),
        (format!("at {long_path} end"), "at <host-path> end"),
        (format!("under {msys}"), "under <host-path>"),
    ];
    for (input, want) in &cases {
        let scrubbed = conductor_core::redact_value(input);
        assert_eq!(mask_host_paths(&scrubbed), *want, "{input}");
    }
}

#[test]
fn the_mask_covers_the_named_roots() {
    for root in [
        format!("/{}/dev", "home"),
        format!("/{}/dev", "Users"),
        format!("%{}%", "APPDATA"),
        format!("/{}/dev", "tmp"),
        format!("/{}/{}/dev", "var", "tmp"),
    ] {
        assert_eq!(
            mask_host_paths(&format!("x {root}/pulse y")),
            "x <host-path> y",
            "{root}"
        );
    }
}

#[test]
fn the_mask_leaves_a_url_a_repo_path_and_a_span_ref_alone() {
    for clean in [
        "http://127.0.0.1:4317/v1/traces",
        "crates/conductor-run/tests/real_model_harvest.rs",
        "span:a/b/c",
    ] {
        assert_eq!(mask_host_paths(clean), clean);
    }
}

// ---- the capture's attribution sweep --------------------------------------------------------------

#[test]
fn the_sweep_reads_an_incident_that_resolved_below_the_lowest_active_one() {
    // Drive a3, 2026-09-29: at the capture's first poll only incident 6 was active — the canary's (4)
    // and the one formed after the scenario's storm (5, inferred from the dense row ids) had
    // auto-resolved at 16:41:34Z. A window anchored at the lowest ACTIVE id starts at 6 and never
    // reads either; this one must.
    let active_at_first_poll = [6_i64];
    let window = sweep_window(active_at_first_poll.iter().copied().max());
    for resolved in [4, 5] {
        assert!(
            window.contains(&resolved),
            "incident {resolved} resolved before the poll and sits below the lowest active id; window {window:?}"
        );
    }
    assert!(window.contains(&6), "the active incident is read too");
}

#[test]
fn the_sweep_window_is_bounded_and_floored_at_one() {
    assert_eq!(sweep_window(None), 1..1 + SWEEP_BOUND);
    assert_eq!(sweep_window(Some(1)), 1..1 + SWEEP_BOUND);
    let high = sweep_window(Some(200));
    assert_eq!(high, 200 - (SWEEP_BOUND - 1)..201);
    assert_eq!(high.end - high.start, SWEEP_BOUND);
}
