//! Live-leg evidence harvest for the operator-gated live suite.
//!
//! The graded half of the capture/grade split: `live_suite.rs` (feature `live-pulse`) PRINTS what a
//! real `agent-run run --live` produced, and this target — which runs in the DEFAULT suite — grades
//! the frozen lines. It pins them as LITERALS, the `severity_harvest.rs` convention: no harvest reads
//! a path at grading time, and a default-suite test could not read the version-workspace `evidence/`
//! tree anyway. `evidence/` keeps the full per-leg copies as the human-readable record.
//!
//! Captured 2026-09-06 against a live Pulse at HEAD `83d4060` (deterministic L4 + MCP enabled, fresh
//! data dir). Three legs, each with a predicted outcome, all met:
//!
//!   b1  `degraded-mode-report` — gate ready, read-back finds b1's own canary incident still open, so
//!       the graded route runs: `ManualCheck`, `latency_ms` recorded.
//!   b2  the SAME scenario fired immediately, inside Pulse's 120s idle window: its canary dedupes
//!       against b1's still-open incident, no fresh incident forms, and the gate goes
//!       not-ready-but-CONNECTED — the only state that reaches the readiness-gate line.
//!   a   `auto-resolve-idle-window` after a 150s quiet window: a fresh canary forms, then idles for
//!       the whole 165s silent scenario, so read-back finds an EMPTY active set.
//!
//! These grade the three `conductor-run` mutation survivors that are accepted-deliberate because no
//! HERMETIC test can reach their sites (test-plan §12 classes B and C). Landing them does NOT retire
//! those acceptances — a live leg is never a CI gate — it buys the behavioural coverage the accepted
//! class cannot.

use serde_json::Value;

/// Leg b2's readiness-gate line — observable 3's PRESENT arm.
fn leg_b2_gate_blocked_line() -> &'static str {
    r#"{"deployment.environment":"local","level":"INFO","message":"preflight blocked: readiness gate not satisfied","run_id":"2026-09-06T09-12-02-060","service.name":"conductor","service.version":"0.1.0","target":"conductor_run::canary","timestamp_ms":1788686056807}"#
}

/// Leg a's auto-resolve line — observable 1's route discriminator. Only the `AutoResolved` arm emits
/// it, which is what separates that route from the degraded-read-back route b1 took.
fn leg_a_auto_resolved_line() -> &'static str {
    r#"{"deployment.environment":"local","level":"INFO","message":"declare-only read-back empty: no active incident outlived the emission window","run_id":"2026-09-06T09-16-47-440","service.name":"conductor","service.version":"0.1.0","target":"conductor_run::execute","timestamp_ms":1788686418592}"#
}

/// Leg a's persisted envelope — the `AutoResolved` arm's row.
///
/// Captured with the scenario's idle window at 165 s. A second live run the same day did NOT reach this
/// arm — the canary incident was still active at read-back — because 165 s left only 15 s over Pulse's
/// 150 s worst case (120 s idle + up to a full 30 s observer tick). The scenario now declares 200 s. These
/// lines stay pinned as the record of what the arm looks like when it IS reached; they are not a claim that
/// 165 s reaches it reliably, and the 200 s value is not yet live-proven.
fn leg_a_envelope() -> &'static str {
    r#"{"fingerprints":[],"journal_emitted_at":"2026-09-06T09:17:33Z","latency_ms":165050,"p_ids":["P-022"],"read_back_observed_at":"2026-09-06T09:20:18Z","run_id":"2026-09-06T09-16-47-440","scenario":"auto-resolve-idle-window","seed":4317122,"slo_tier":"<90s","state":"KnownResidual","verdict":null}"#
}

/// Leg b1's persisted envelope — observable 2's subject, and observable 1's contrast.
fn leg_b1_envelope() -> &'static str {
    r#"{"fingerprints":["det-span-9f2c4a7e1b6d0358","det-template-0007","det-fingerprint-4a7f2b91c6e05d3849b1e7a2c5f08d63","84093098717f14f2527418611dba052c"],"journal_emitted_at":"2026-09-06T09:11:55Z","latency_ms":6045,"p_ids":["P-053"],"read_back_observed_at":"2026-09-06T09:12:01Z","run_id":"2026-09-06T09-11-09-325","scenario":"degraded-mode-report","seed":4317053,"slo_tier":"<20s","state":"ManualCheck","verdict":null}"#
}

/// Per-leg presence of the readiness-gate line, as measured over the frozen captures
/// (74 / 233 / 66 self-obs lines respectively).
const GATE_LINE_PRESENCE: [(&str, bool); 3] = [("b1", false), ("b2", true), ("a", false)];

fn parse(line: &str) -> Value {
    serde_json::from_str(line).expect("a frozen capture line parses")
}

fn seconds_of(rfc3339: &str) -> i64 {
    let (h, m, s) = (&rfc3339[11..13], &rfc3339[14..16], &rfc3339[17..19]);
    h.parse::<i64>().unwrap() * 3600 + m.parse::<i64>().unwrap() * 60 + s.parse::<i64>().unwrap()
}

/// Observable 1. The envelope carries NO `degraded` field — `RunRecord`'s eleven fields have none and
/// `degraded_mode_response` exists nowhere in the workspace — so the persisted proxy is the state
/// mapping: `manual_record` sets `state_for(observation, ManualCheck)`, which returns `KnownResidual`
/// iff `observation.degraded`. The mutation site deletes the `degraded` field initializer on the
/// `AutoResolved` arm, which would land this row on the default arm instead.
#[test]
fn the_auto_resolve_arm_lands_known_residual_with_its_own_route_line() {
    let env = parse(leg_a_envelope());
    assert_eq!(
        env["scenario"], "auto-resolve-idle-window",
        "the pinned envelope is leg a's"
    );
    assert_eq!(
        env["state"], "KnownResidual",
        "the AutoResolved arm sets degraded, so state maps to the residual"
    );
    assert!(
        env["verdict"].is_null(),
        "a declare-only scenario grades nothing"
    );
    assert_eq!(
        env["fingerprints"].as_array().expect("fingerprints").len(),
        0,
        "an empty active set carries no fingerprints"
    );

    let line = parse(leg_a_auto_resolved_line());
    assert_eq!(
        line["message"],
        "declare-only read-back empty: no active incident outlived the emission window",
        "the route discriminator's text is what separates AutoResolved from the degraded read-back"
    );
    assert_eq!(line["target"], "conductor_run::execute");
    assert_eq!(
        line["run_id"], env["run_id"],
        "the line and the envelope belong to the same run"
    );
}

/// Observable 1's other half: the mapping DISCRIMINATES rather than being constant under
/// deterministic L4. The same declare-only shape on leg b1 took the graded route and landed
/// `ManualCheck`, so `KnownResidual` above is a measurement, not a foregone conclusion.
#[test]
fn the_graded_route_lands_manual_check_on_the_same_declare_only_shape() {
    let b1 = parse(leg_b1_envelope());
    let a = parse(leg_a_envelope());
    assert_eq!(b1["state"], "ManualCheck");
    assert_eq!(a["state"], "KnownResidual");
    assert_ne!(
        b1["state"], a["state"],
        "the two routes must not collapse to one state"
    );
}

/// Observable 2. `latency_ms` is `read_back_observed_at − journal_emitted_at` — a SUBTRACTION. The
/// two persisted instants are second-truncated (`now_rfc3339` formats from `as_secs()`) while
/// `latency_ms` is millisecond-grained, so the equality is bounded by one second, never exact. The
/// mutation sites replace `-` with `+` or `/`, whose outputs miss by far more than that bound.
#[test]
fn the_manual_path_latency_is_the_bounded_difference_of_its_two_instants() {
    let env = parse(leg_b1_envelope());
    let latency = env["latency_ms"].as_i64().expect("latency_ms");
    let emitted = seconds_of(
        env["journal_emitted_at"]
            .as_str()
            .expect("journal_emitted_at"),
    );
    let observed = seconds_of(
        env["read_back_observed_at"]
            .as_str()
            .expect("read_back_observed_at"),
    );

    let difference_ms = (observed - emitted) * 1000;
    assert!(
        (latency - difference_ms).abs() < 1000,
        "latency_ms {latency} is not the bounded difference of its instants ({difference_ms}ms)"
    );

    // The mutants, at the measured values: a sum reads the two epochs added, a quotient reads ~1.
    let sum_ms = (observed + emitted) * 1000;
    assert!(
        (latency - sum_ms).abs() >= 1000,
        "a sum would be indistinguishable from the difference"
    );
    assert!(
        observed != 0 && (latency - observed / emitted.max(1)).abs() >= 1000,
        "a quotient would be indistinguishable"
    );
}

/// Observable 3, BOTH arms. The mutation site deletes the `!` in `if !state.ready`, inverting the
/// guard — which a one-arm assertion would pass, so the absent arm is graded too.
#[test]
fn the_readiness_gate_line_marks_the_not_ready_leg_and_only_it() {
    assert_eq!(
        GATE_LINE_PRESENCE,
        [("b1", false), ("b2", true), ("a", false)],
        "only the deduped leg reached a not-ready-but-connected gate"
    );

    let line = parse(leg_b2_gate_blocked_line());
    assert_eq!(
        line["message"],
        "preflight blocked: readiness gate not satisfied"
    );
    assert_eq!(line["target"], "conductor_run::canary");
}

/// The exact-string rule. `canary.rs` carries three other `preflight blocked:` lines — two for the
/// unreachable read-back path and one for a failed canary emission — and the connect arms return
/// EARLY, so a prefix match would report a sidecar that never connected as a not-ready gate.
#[test]
fn the_gate_line_is_not_confused_with_the_other_preflight_blocked_lines() {
    let message = parse(leg_b2_gate_blocked_line())["message"]
        .as_str()
        .expect("message")
        .to_owned();
    assert!(message.starts_with("preflight blocked: "));
    for other in [
        "preflight blocked: MCP read-back path unreachable",
        "preflight blocked: canary emission failed",
    ] {
        assert_ne!(
            message, other,
            "the gate line must be matched exactly, never by its prefix"
        );
    }
}
