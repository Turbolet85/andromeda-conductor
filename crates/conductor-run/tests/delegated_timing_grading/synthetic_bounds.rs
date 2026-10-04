//! The four delegated bounds' unit tests over synthetic lines in Pulse's on-disk shape.

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
