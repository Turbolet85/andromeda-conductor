//! Live-leg evidence harvest for the incident resolve-lifecycle (P-075) — the read-back fidelity
//! Conductor CAN assert against a deterministic-L4 Pulse.
//!
//! WHY THIS EXISTS AT ALL. Payload-level content fidelity is unattainable in this mode: every
//! L4-authored field (`title` / `severity` / `fingerprint` / `evidence_refs`) is a fixture constant,
//! so no read-back field varies with what Conductor emitted (arch §Read-Back Dependency Posture,
//! measured 2026-08-16). A SECOND axis was checked at Pulse HEAD `83d4060` and closed the same way:
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

use conductor_run::{
    AUTO_RESOLVE_IDLE_SECONDS, LifecycleObservation, LifecycleVerdict, attribute_by_liveness,
    evaluate_lifecycle, select_resolve_target,
};

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
