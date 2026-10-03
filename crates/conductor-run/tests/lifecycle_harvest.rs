//! Live-leg evidence harvest for the incident resolve-lifecycle (P-075) — the read-back fidelity
//! Conductor CAN assert against a deterministic-L4 Pulse.
//!
//! WHY THIS EXISTS AT ALL. Every L4-authored field (`title` / `severity` / `fingerprint` /
//! `evidence_refs`) is a fixture constant in this mode, so as measured 2026-08-16 the read-back
//! carried no payload-varying field (arch §Read-Back Dependency Posture). That was true of Pulse
//! `efabe8e` and is not of `83d4060`, where `fingerprint_refs` also carries the triggering cue's
//! computed fingerprint (measured 2026-09-10) — a payload-identity axis this harvest does not grade,
//! so its runtime-state grading below is unchanged. A SECOND axis was checked at Pulse HEAD `83d4060` and closed the same way:
//! `incident_events` — the corpus table that persists created/resolved lifecycle events, and the one
//! table whose content is NOT L4-authored — is written by `crates/triage/src/incident/persistence.rs`
//! and read only corpus-side; there are ZERO references to it in `crates/mcp-server`, whose eight
//! wire tools are pinned at `crates/mcp-server/src/tools.rs:44-51`. No MCP tool surfaces it at any
//! width, so it is recorded as a Pulse 0.4.0 residual candidate rather than built against.
//!
//! What survives is RUNTIME-STATE fidelity: Conductor writes through `mark_incident_resolved` and
//! observes the consequence through `query_incident_list`. It depends on no L4-authored field, which
//! is exactly why it holds where payload fidelity does not. Pulse asserts the same pairing in-process
//! (`crates/mcp-server/src/tools.rs:934`, "Re-read: status is now resolved, no longer in the active
//! list"); the live leg asserts it across the wire.
//!
//! THE TWO-INCIDENT CONTROL IS UNATTAINABLE WITHIN ONE DEDUPE TUPLE — measured, not assumed. The plan
//! called for driving a SECOND incident and resolving only one, so the spared control would attribute
//! the change. Pulse forecloses that for storms sharing one identity: the incident producer dedupes a
//! new incident against an OPEN one carrying the same `(kind, scope, scope_id)` TUPLE — the workspace
//! scopes the candidate set, the tuple is the key (`pulse-app/src/inference_runtime.rs:811`). Every
//! storm this leg drove shared one tuple, so no second incident could form. Measured this chunk at
//! HEAD `83d4060` — a storm raised while incident 4 was open logged `created=false deduped=true`, and
//! across the whole leg every `created=true` occurred with the active set EMPTY. The clinching
//! independent observation: incident 7 formed at 16:43:23, the same second incident 6 was resolved. A
//! CROSS-SCOPE control (two incidents under DIFFERENT tuples) is possible and untested — a weighable
//! route option, never a retirement. So `LifecycleVerdict::Proven` is unreachable as driven and kept
//! for the stronger form.
//!
//! WHAT REPLACES IT IS LIVENESS. Pulse auto-resolves only an IDLE incident (120s idle, 30s resolver
//! tick), so an incident whose telemetry is seconds old cannot be the resolver's to take. Resolving
//! a freshly-refreshed incident and watching it leave the active set therefore excludes the resolver
//! by construction. `a_single_incident_drop_is_not_attributable` still pins the mis-pairing that has
//! no liveness evidence behind it — the hazard test-plan §6 requires be pinned for severity-lifecycle.
//!
//! THE DECLINED ARM IS STUB-ONLY, AND NOT BECAUSE IT IS AWKWARD. Pulse guards the write on
//! `UPDATE … WHERE id = ?5 AND updated_unix_nano <= ?2` and returns `DeclinedStale` when zero rows
//! match (`crates/corpus/src/contract.rs:652-659`); its dispatch stamps `now = current_unix_nanos()`
//! fresh on every call, so the predicate can fail only against a future-stamped row. The arm is
//! unreachable through the MCP surface by any ordinary means, and is proven in
//! `conductor-verify/tests/readback.rs` against the stub instead — recorded as stub-proven, never as
//! a live claim for an arm that never fired.
//!
//! MEASURED 2026-09-01 against a live Pulse at HEAD `83d4060` (release binaries, deterministic L4,
//! fresh data dir, `pulse-app` spawned directly). The verbatim capture is pinned in `leg_*()` below.
//! Incident 6 was resolved through `mark_incident_resolved` 0.0s after its last emission and left
//! the active set immediately; Pulse's own corpus records it active 16:42:38 → 16:43:23, a 45s life
//! against a 120s idle threshold — so the auto-resolver could not have taken it.
//!
//! ONE READING TRAP COST FOUR LEGS AND IS PINNED BELOW. The live wire carries the item key
//! `incident_id`; the in-process stub carried `id`. A reader accepting only `id` extracts nothing
//! from a populated response and returns an empty list — indistinguishable downstream from an empty
//! corpus, and green against every stub test. Three incidents were active for 131–142s each while
//! the leg reported `[]`. Only dumping the RAW wire value separated them, which is why
//! `active_ids` prints it and why `the_live_item_key_is_incident_id` pins the shape here.
//!
//! TEST-ONLY affordance: nothing here is wired into the run path, and nothing harvested reaches a
//! Conductor artifact.

use std::collections::HashMap;

use conductor_run::{
    AUTO_RESOLVE_IDLE_SECONDS, LifecycleObservation, LifecycleVerdict, attribute_by_liveness,
    evaluate_lifecycle, select_resolve_target,
};

mod evidence_pin;

/// The RAW `query_incident_list` result the live sidecar returned, verbatim from the 2026-09-01 leg.
/// Pinned because the item key is the thing that silently broke four earlier legs.
fn leg_raw_active_list() -> &'static str {
    r#"{"items":[{"incident_id":6,"opened_at_unix_nano":1788280958251622800,"severity":"error","status":"active","title":"Deterministic verification incident"}],"next_cursor":null,"total":1}"#
}

/// The leg's measured observation and the idle age of the resolved incident at the write instant.
fn leg_observation() -> (LifecycleObservation, f64) {
    (
        LifecycleObservation {
            before: vec![6],
            resolved: 6,
            after: vec![],
        },
        0.0,
    )
}

/// The active set as `query_incident_list` reports it, reduced to the ids the probe reads.
fn observation(before: &[i64], resolved: i64, after: &[i64]) -> LifecycleObservation {
    LifecycleObservation {
        before: before.to_vec(),
        resolved,
        after: after.to_vec(),
    }
}

#[test]
fn the_older_incident_is_held_back_as_the_control() {
    // A is the preflight canary's (already open when the scenario's own storm forms); B is the
    // scenario's. Resolving the newer one keeps the canary as the control.
    assert_eq!(select_resolve_target(&[1, 2]), Some((2, 1)));
    assert_eq!(select_resolve_target(&[1, 2, 3]), Some((3, 1)));
}

#[test]
fn a_single_incident_offers_no_control_to_hold() {
    assert_eq!(select_resolve_target(&[1]), None);
    assert_eq!(select_resolve_target(&[]), None);
}

/// The leg's whole point: the resolved id is gone AND the control survived.
#[test]
fn a_spared_control_attributes_the_drop_to_conductors_write() {
    let seen = observation(&[1, 2], 2, &[1]);
    assert_eq!(
        evaluate_lifecycle(&seen, 1),
        LifecycleVerdict::Proven { control: 1 }
    );
}

/// THE NEGATIVE TEST. One incident in, none out, is exactly what Pulse's 120s idle auto-resolve
/// produces — so it must NOT grade as proof. Without this the leg would pass on evidence that says
/// nothing about whether Conductor's write did anything.
#[test]
fn a_single_incident_drop_is_not_attributable() {
    let seen = observation(&[2], 2, &[]);
    assert_eq!(evaluate_lifecycle(&seen, 2), LifecycleVerdict::NoControl);
}

/// Two incidents in, none out: the write landed, but the auto-resolver also took the control, so the
/// leg cannot claim the drop. Distinct from `NoControl` — a control was held, it just did not survive.
#[test]
fn losing_the_control_too_is_unattributable_not_proven() {
    let seen = observation(&[1, 2], 2, &[]);
    assert_eq!(
        evaluate_lifecycle(&seen, 1),
        LifecycleVerdict::Unattributable
    );
}

/// A write Pulse accepted but that left the incident active is a FAILED resolve, never a pass.
#[test]
fn an_incident_still_active_after_the_write_is_not_proven() {
    let seen = observation(&[1, 2], 2, &[1, 2]);
    assert_eq!(evaluate_lifecycle(&seen, 1), LifecycleVerdict::StillActive);
}

/// Absence is never a pass: an empty before-set cannot grade, and must not fall through to `Proven`
/// on the strength of an empty after-set matching it.
#[test]
fn an_empty_active_set_grades_no_control_not_proven() {
    let seen = observation(&[], 1, &[]);
    assert_eq!(evaluate_lifecycle(&seen, 1), LifecycleVerdict::NoControl);
}

// ---- the live capture, 2026-09-01, Pulse HEAD 83d4060 ----

/// The live item key is `incident_id`. A reader accepting only `id` returns an empty list from THIS
/// exact payload — the silent degrade that cost four legs.
#[test]
fn the_live_item_key_is_incident_id() {
    let raw = leg_raw_active_list();
    assert!(raw.contains(r#""incident_id":6"#), "live item key: {raw}");
    assert!(
        !raw.contains(r#""id":6"#),
        "the live wire does NOT carry a bare `id` — a reader keyed on it extracts nothing: {raw}"
    );
}

/// THE LEG'S CLAIM. Incident 6 left the active set 0.0s after its last emission — three orders
/// inside the 120s the auto-resolver requires — so Conductor's `mark_incident_resolved` write is the
/// only cause that remains.
#[test]
fn the_live_leg_resolve_is_attributed_by_liveness() {
    let (seen, idle) = leg_observation();
    assert_eq!(seen.before, vec![6], "one active incident before the write");
    assert_eq!(
        seen.after,
        Vec::<i64>::new(),
        "the resolved id left the active set"
    );
    assert!(
        idle < AUTO_RESOLVE_IDLE_SECONDS,
        "the incident was NOT idle when resolved"
    );
    assert_eq!(
        attribute_by_liveness(&seen, idle),
        LifecycleVerdict::ProvenByLiveness { idle_seconds: 0.0 }
    );
}

/// The same disappearance with a STALE incident proves nothing — the auto-resolver produces exactly
/// this. Liveness is the whole attribution, so its absence must downgrade the verdict.
#[test]
fn the_same_drop_without_liveness_is_unattributable() {
    let (seen, _) = leg_observation();
    assert_eq!(
        attribute_by_liveness(&seen, AUTO_RESOLVE_IDLE_SECONDS + 1.0),
        LifecycleVerdict::Unattributable
    );
}

/// The two-incident control is unreachable against Pulse as measured, so nothing may claim it: a
/// single-incident before-set can never grade `Proven` on the control path.
#[test]
fn the_control_path_cannot_grade_a_single_incident_leg() {
    let (seen, _) = leg_observation();
    assert_eq!(evaluate_lifecycle(&seen, 6), LifecycleVerdict::NoControl);
}

// ---- the P-075 round, 2026-10-02, Pulse S 03ec944 (round-request assertions 1 and 2) ----
//
// `tests/p075_round_live.rs` prints one block of `p075-round:` lines (integers, booleans, closed
// words); the committed copy is graded here from its file, behind its digest pin. An absent sample is
// UNGRADED, never met (round-request §Grading posture).

/// One assertion's grade at its measured value.
#[derive(Debug, Clone, PartialEq)]
enum RoundGrade {
    Pass,
    Fail(String),
    Ungraded(String),
}

/// Every `key=value` token of the capture's `p075-round:` lines.
fn round_fields(capture: &str) -> HashMap<String, String> {
    capture
        .lines()
        .filter_map(|line| line.strip_prefix("p075-round: "))
        .flat_map(str::split_whitespace)
        .filter_map(|token| token.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn field<'a>(fields: &'a HashMap<String, String>, key: &str) -> &'a str {
    fields.get(key).map_or("absent", String::as_str)
}

/// Assertion 1 — read-back content fidelity: Conductor's fingerprint is a member of the incident's
/// `fingerprint_refs`, the incident opened after the storm's emission instant, and `retrieve_report`
/// returned `degraded_mode: false`. A storm that met an already-open incident deduped into it, and
/// a dedupe never writes `fingerprint_hashes` (Pulse `inference_runtime.rs:830-858` at S), so a
/// non-empty set at open makes the reading UNGRADED rather than a FAIL.
fn grade_assertion_1(fields: &HashMap<String, String>) -> RoundGrade {
    let open = field(fields, "active_at_open");
    if open != "0" {
        return RoundGrade::Ungraded(format!("active_at_open={open}"));
    }
    if field(fields, "incident_id") == "none" {
        return RoundGrade::Ungraded("no incident opened after the storm".to_string());
    }
    let wanted = [
        ("fingerprint_in_refs", "true"),
        ("opened_after_emission", "true"),
        ("degraded_mode", "false"),
    ];
    if let Some((key, _)) = wanted.iter().find(|(k, _)| field(fields, k) == "absent") {
        return RoundGrade::Ungraded(format!("{key} absent"));
    }
    let failed: Vec<String> = wanted
        .iter()
        .filter(|(k, want)| field(fields, k) != *want)
        .map(|(k, _)| format!("{k}={}", field(fields, k)))
        .collect();
    if failed.is_empty() {
        RoundGrade::Pass
    } else {
        RoundGrade::Fail(failed.join(" "))
    }
}

/// Assertion 2 — runtime-state fidelity: `mark_incident_resolved` applied and the incident left the
/// active set while its telemetry was fresh, the attribution `attribute_by_liveness` grades.
fn grade_assertion_2(fields: &HashMap<String, String>) -> RoundGrade {
    match field(fields, "verdict") {
        "absent" | "NoControl" => {
            RoundGrade::Ungraded(format!("verdict={}", field(fields, "verdict")))
        }
        "ProvenByLiveness" if field(fields, "resolved_left_active_set") == "true" => {
            RoundGrade::Pass
        }
        other => RoundGrade::Fail(format!(
            "verdict={other} resolved_left_active_set={}",
            field(fields, "resolved_left_active_set")
        )),
    }
}

/// Assertion 7 — the incident's lifecycle events read back through `retrieve_incident_events`.
/// BEFORE the resolve: the first event is `created` and none is `resolved`. AFTER it: the first is
/// still `created`, the last is `resolved`, and that event's stamp lies inside the resolve call's
/// request→response window, inclusive at both ends (Pulse stamps `now` inside the call). Every event
/// in both reads sits in the four-kind vocabulary. A truncated read could hide the last event, so it
/// is UNGRADED rather than graded on a partial list.
fn grade_assertion_7(fields: &HashMap<String, String>) -> RoundGrade {
    let required = [
        "before_first",
        "before_has_resolved",
        "after_first",
        "after_last",
        "before_outside_vocabulary",
        "after_outside_vocabulary",
    ];
    if let Some(key) = required.iter().find(|k| field(fields, k) == "absent") {
        return RoundGrade::Ungraded(format!("{key} absent"));
    }
    if let Some(key) = ["before_truncated", "after_truncated"]
        .iter()
        .find(|k| field(fields, k) == "true")
    {
        return RoundGrade::Ungraded(format!("{key}=true"));
    }
    let wanted = [
        ("before_first", "created"),
        ("before_has_resolved", "false"),
        ("after_first", "created"),
        ("after_last", "resolved"),
        ("before_outside_vocabulary", "0"),
        ("after_outside_vocabulary", "0"),
    ];
    let mut failed: Vec<String> = wanted
        .iter()
        .filter(|(k, want)| field(fields, k) != *want)
        .map(|(k, _)| format!("{k}={}", field(fields, k)))
        .collect();
    if field(fields, "after_last") == "resolved" {
        for key in ["resolved_minus_sent_ns", "received_minus_resolved_ns"] {
            let inside = field(fields, key).parse::<i64>().is_ok_and(|ns| ns >= 0);
            if !inside {
                failed.push(format!("{key}={}", field(fields, key)));
            }
        }
    }
    if failed.is_empty() {
        RoundGrade::Pass
    } else {
        RoundGrade::Fail(failed.join(" "))
    }
}

/// A capture in the leg's print format, every graded field set to the passing value.
fn synthetic_round(overrides: &[(&str, &str)]) -> HashMap<String, String> {
    let mut capture = String::from(
        "\np075-round: active_at_open=0\n\
         p075-round: incident_id=1 opened_after_emission=true opened_minus_emitted_ms=41250\n\
         p075-round: fingerprint_refs=4 det_members=3 fingerprint_in_refs=true\n\
         p075-round: degraded_mode=false\n\
         p075-round: before_total=1 before_truncated=false before_first=created before_has_resolved=false before_sequence=created before_outside_vocabulary=0\n\
         p075-round: resolve_before=1 resolved_left_active_set=true idle_ms=812 verdict=ProvenByLiveness\n\
         p075-round: after_total=2 after_truncated=false after_first=created after_last=resolved after_sequence=created,resolved after_outside_vocabulary=0 resolve_window_ns=4210000 resolved_minus_sent_ns=2105000 received_minus_resolved_ns=2105000\n\
         p075-round: end\n",
    );
    for (key, value) in overrides {
        let start = capture
            .find(&format!("{key}="))
            .expect("the key is in the format");
        let end = capture[start..]
            .find([' ', '\n'])
            .map_or(capture.len(), |at| start + at);
        capture.replace_range(start..end, &format!("{key}={value}"));
    }
    round_fields(&capture)
}

#[test]
fn round_a_full_capture_parses_every_field() {
    let fields = synthetic_round(&[]);
    assert_eq!(field(&fields, "fingerprint_refs"), "4");
    assert_eq!(field(&fields, "det_members"), "3");
    assert_eq!(field(&fields, "idle_ms"), "812");
    assert_eq!(field(&fields, "missing"), "absent");
}

#[test]
fn round_assertion_1_passes_only_when_all_three_hold() {
    assert_eq!(grade_assertion_1(&synthetic_round(&[])), RoundGrade::Pass);
}

#[test]
fn round_assertion_1_a_fingerprint_missing_from_the_refs_is_a_fail() {
    assert_eq!(
        grade_assertion_1(&synthetic_round(&[("fingerprint_in_refs", "false")])),
        RoundGrade::Fail("fingerprint_in_refs=false".to_string())
    );
}

#[test]
fn round_assertion_1_a_degraded_report_is_a_fail() {
    assert_eq!(
        grade_assertion_1(&synthetic_round(&[("degraded_mode", "true")])),
        RoundGrade::Fail("degraded_mode=true".to_string())
    );
}

#[test]
fn round_assertion_1_an_incident_open_at_the_start_is_ungraded_never_a_fail() {
    assert_eq!(
        grade_assertion_1(&synthetic_round(&[
            ("active_at_open", "1"),
            ("fingerprint_in_refs", "false"),
        ])),
        RoundGrade::Ungraded("active_at_open=1".to_string())
    );
}

#[test]
fn round_assertion_1_no_incident_is_ungraded() {
    assert_eq!(
        grade_assertion_1(&synthetic_round(&[("incident_id", "none")])),
        RoundGrade::Ungraded("no incident opened after the storm".to_string())
    );
}

#[test]
fn round_assertion_1_an_absent_report_reading_is_ungraded_never_met() {
    assert_eq!(
        grade_assertion_1(&synthetic_round(&[("degraded_mode", "absent")])),
        RoundGrade::Ungraded("degraded_mode absent".to_string())
    );
}

#[test]
fn round_assertion_2_passes_on_a_liveness_attributed_resolve() {
    assert_eq!(grade_assertion_2(&synthetic_round(&[])), RoundGrade::Pass);
}

#[test]
fn round_assertion_2_a_still_active_incident_is_a_fail() {
    assert_eq!(
        grade_assertion_2(&synthetic_round(&[
            ("verdict", "StillActive"),
            ("resolved_left_active_set", "false"),
        ])),
        RoundGrade::Fail("verdict=StillActive resolved_left_active_set=false".to_string())
    );
}

#[test]
fn round_assertion_2_a_stale_resolve_is_a_fail_not_a_pass() {
    assert_eq!(
        grade_assertion_2(&synthetic_round(&[("verdict", "Unattributable")])),
        RoundGrade::Fail("verdict=Unattributable resolved_left_active_set=true".to_string())
    );
}

#[test]
fn round_assertion_2_no_resolve_is_ungraded() {
    assert_eq!(
        grade_assertion_2(&synthetic_round(&[("verdict", "absent")])),
        RoundGrade::Ungraded("verdict=absent".to_string())
    );
    assert_eq!(
        grade_assertion_2(&synthetic_round(&[("verdict", "NoControl")])),
        RoundGrade::Ungraded("verdict=NoControl".to_string())
    );
}

#[test]
fn round_assertion_7_passes_on_the_canonical_capture() {
    assert_eq!(grade_assertion_7(&synthetic_round(&[])), RoundGrade::Pass);
}

#[test]
fn round_assertion_7_each_failing_shape_is_a_fail_naming_its_key() {
    for (key, value) in [
        ("before_first", "unknown"),
        ("before_has_resolved", "true"),
        ("after_first", "active"),
        ("after_last", "active"),
        ("resolved_minus_sent_ns", "-1"),
        ("received_minus_resolved_ns", "-1"),
        ("before_outside_vocabulary", "1"),
        ("after_outside_vocabulary", "1"),
    ] {
        assert_eq!(
            grade_assertion_7(&synthetic_round(&[(key, value)])),
            RoundGrade::Fail(format!("{key}={value}")),
            "{key}={value}"
        );
    }
}

/// The window bound is inclusive at both ends: Pulse stamps `now` inside the call, so a stamp equal
/// to either edge is inside it.
#[test]
fn round_assertion_7_a_stamp_on_either_window_edge_passes() {
    assert_eq!(
        grade_assertion_7(&synthetic_round(&[("resolved_minus_sent_ns", "0")])),
        RoundGrade::Pass
    );
    assert_eq!(
        grade_assertion_7(&synthetic_round(&[("received_minus_resolved_ns", "0")])),
        RoundGrade::Pass
    );
}

/// With `resolved` last, an offset that could not be computed is a FAIL — the event is there and its
/// placement is unproven. With anything else last, the offsets are never read.
#[test]
fn round_assertion_7_offsets_are_read_only_when_resolved_is_last() {
    assert_eq!(
        grade_assertion_7(&synthetic_round(&[("resolved_minus_sent_ns", "absent")])),
        RoundGrade::Fail("resolved_minus_sent_ns=absent".to_string())
    );
    assert_eq!(
        grade_assertion_7(&synthetic_round(&[
            ("after_last", "active"),
            ("resolved_minus_sent_ns", "absent"),
            ("received_minus_resolved_ns", "absent"),
        ])),
        RoundGrade::Fail("after_last=active".to_string())
    );
}

#[test]
fn round_assertion_7_an_absent_field_is_ungraded_never_met() {
    assert_eq!(
        grade_assertion_7(&synthetic_round(&[("after_last", "absent")])),
        RoundGrade::Ungraded("after_last absent".to_string())
    );
}

#[test]
fn round_assertion_7_a_truncated_read_is_ungraded_never_met() {
    assert_eq!(
        grade_assertion_7(&synthetic_round(&[("after_truncated", "true")])),
        RoundGrade::Ungraded("after_truncated=true".to_string())
    );
}

/// The round's committed P-075 leg capture (2026-10-02, Pulse S `03ec944`, deterministic L4, fresh
/// data dir, the leg fired first on the launch). Run ids, pre-leg counts and the censuses are in
/// `evidence/round-ledger.md`.
const ROUND_CAPTURE: &str =
    "conductor-0.3.0/chunks/2026-10-02-p-075-assert-round-against-pulse/evidence/p075-leg.txt";
const ROUND_CAPTURE_SHA256: &str =
    "85773fb075f5bf7ac199613c9db5f0523f38a7f11a181595721515aace7a4e72";

fn round_capture() -> HashMap<String, String> {
    round_fields(&evidence_pin::pinned(ROUND_CAPTURE, ROUND_CAPTURE_SHA256))
}

/// ROUND-REQUEST ASSERTION 1, AS MEASURED: Conductor's 32-hex fingerprint is a member of incident
/// 1's `fingerprint_refs` (4 refs, 3 of them the `det-*` constants), the incident opened 46 ms after
/// the storm's emission instant on an empty active set, and `retrieve_report` returned
/// `degraded_mode: false`.
#[test]
fn p075_round_assertion_1_read_back_content_fidelity() {
    let fields = round_capture();
    assert_eq!(field(&fields, "fingerprint_refs"), "4");
    assert_eq!(field(&fields, "det_members"), "3");
    assert_eq!(grade_assertion_1(&fields), RoundGrade::Pass);
}

/// ROUND-REQUEST ASSERTION 2, AS MEASURED: `mark_incident_resolved` removed incident 1 from the
/// active set 12 ms after its last emission — far inside the 120 s the auto-resolver requires.
#[test]
fn p075_round_assertion_2_runtime_state_fidelity() {
    let fields = round_capture();
    assert_eq!(field(&fields, "resolve_before"), "1");
    assert_eq!(field(&fields, "idle_ms"), "12");
    assert_eq!(grade_assertion_2(&fields), RoundGrade::Pass);
}

#[test]
fn p075_round_capture_digest_pin_fails_on_a_tampered_byte() {
    let text = evidence_pin::committed(ROUND_CAPTURE);
    assert!(evidence_pin::check_digest(ROUND_CAPTURE, &text, ROUND_CAPTURE_SHA256).is_ok());
    let tampered = text.replacen("degraded_mode=false", "degraded_mode=falsE", 1);
    assert_ne!(tampered, text, "the tamper landed");
    assert!(evidence_pin::check_digest(ROUND_CAPTURE, &tampered, ROUND_CAPTURE_SHA256).is_err());
}

// ---- the P-075 re-round, 2026-10-03, Pulse S2 cdb6c1e (round-request assertions 1, 2 and 7) ----
//
// One fresh `pulse-app` launch on the Linux host (deterministic L4, `WEBKIT_DISABLE_DMABUF_RENDERER=1`
// — the launch-posture deviation recorded in `evidence/round-ledger.md`), the leg fired first on it.

const REROUND_CAPTURE: &str =
    "conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/evidence/p075-leg.txt";
const REROUND_CAPTURE_SHA256: &str =
    "aebc50ee41f3993929788bd424dcfae40cfaab26480efe29381ad93062aaafac";

fn reround_capture() -> HashMap<String, String> {
    round_fields(&evidence_pin::pinned(
        REROUND_CAPTURE,
        REROUND_CAPTURE_SHA256,
    ))
}

/// ROUND-REQUEST ASSERTION 1, AS MEASURED AT S2: Conductor's fingerprint is a member of incident 1's
/// `fingerprint_refs` (4 refs, 3 of them `det-*`), the incident opened 30 ms after the storm's
/// emission instant on an empty active set, and `retrieve_report` returned `degraded_mode: false`.
#[test]
fn p075_reround_assertion_1_read_back_content_fidelity() {
    let fields = reround_capture();
    assert_eq!(field(&fields, "active_at_open"), "0");
    assert_eq!(field(&fields, "fingerprint_refs"), "4");
    assert_eq!(field(&fields, "det_members"), "3");
    assert_eq!(field(&fields, "opened_minus_emitted_ms"), "30");
    assert_eq!(grade_assertion_1(&fields), RoundGrade::Pass);
}

/// ROUND-REQUEST ASSERTION 2, AS MEASURED AT S2: `mark_incident_resolved` removed incident 1 from the
/// active set 0 ms (sub-millisecond) after its last emission — far inside the auto-resolver's 120 s.
#[test]
fn p075_reround_assertion_2_runtime_state_fidelity() {
    let fields = reround_capture();
    assert_eq!(field(&fields, "resolve_before"), "1");
    assert_eq!(field(&fields, "idle_ms"), "0");
    assert_eq!(grade_assertion_2(&fields), RoundGrade::Pass);
}

/// ROUND-REQUEST ASSERTION 7, AS MEASURED AT S2: before the resolve the incident's events read
/// `created` alone; after it, `created,resolved`, the `resolved` stamp 63 395 ns after the request
/// was sent and 224 316 ns before the response arrived, inside a 287 711 ns window. No event read
/// `unknown`, and neither read was truncated.
#[test]
fn p075_reround_assertion_7_incident_events_read_back() {
    let fields = reround_capture();
    assert_eq!(field(&fields, "before_sequence"), "created");
    assert_eq!(field(&fields, "after_sequence"), "created,resolved");
    assert_eq!(field(&fields, "resolve_window_ns"), "287711");
    assert_eq!(field(&fields, "resolved_minus_sent_ns"), "63395");
    assert_eq!(field(&fields, "received_minus_resolved_ns"), "224316");
    assert_eq!(grade_assertion_7(&fields), RoundGrade::Pass);
}

#[test]
fn p075_reround_capture_digest_pin_fails_on_a_tampered_byte() {
    let text = evidence_pin::committed(REROUND_CAPTURE);
    assert!(evidence_pin::check_digest(REROUND_CAPTURE, &text, REROUND_CAPTURE_SHA256).is_ok());
    let tampered = text.replacen("after_last=resolved", "after_last=resolveD", 1);
    assert_ne!(tampered, text, "the tamper landed");
    assert!(
        evidence_pin::check_digest(REROUND_CAPTURE, &tampered, REROUND_CAPTURE_SHA256).is_err()
    );
}

#[test]
fn round_a_tampered_byte_fails_the_digest_pin() {
    let text = "p075-round: end\n";
    let pin = evidence_pin::sha256_hex(text);
    assert!(evidence_pin::check_digest("synthetic", text, &pin).is_ok());
    assert!(evidence_pin::check_digest("synthetic", "p075-round: enD\n", &pin).is_err());
}
