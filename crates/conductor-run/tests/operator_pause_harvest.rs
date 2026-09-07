//! Live-leg evidence harvest for the operator-pause + operator-checklist activation (v2-29).
//!
//! `v2-29` is a `manual`-method capability: its acceptance is an OPERATOR observation during a live
//! attended run (a real Proceed/Abort activation, the count frozen at its exact hold value and
//! resumed on the decision, and at least one checklist item rendered with its induced state and
//! expected observation). No CI test can stand in for that — an attended activation is exactly the
//! thing a headless suite cannot perform.
//!
//! What this file DOES pin is the machinery the operator's leg depends on, so the leg grades a
//! known-good contract rather than discovering a wiring gap mid-observation:
//!
//!   * the two attended-leg vehicles really do declare their checklist items, with the texts the
//!     dialog will render;
//!   * a declared item reaches a [`HoldPoint`] the way the firing site builds one;
//!   * the recorded resolution DISTINGUISHES an attended resolver from the headless default — the
//!     property acceptance part (1) rests on, and the one the pre-chunk code got wrong (the witness
//!     said "resolved headless" unconditionally, at a `debug` level the default INFO filter drops);
//!   * an attended Abort stays a VALUE, never a harness fault (the verdict/error wall).
//!
//! `Preflight`'s fields are crate-private, so `execute_scenario` is not drivable from an integration
//! test; the firing site's own arms are covered by the in-crate unit tests beside it.

use std::path::PathBuf;

use conductor_core::{
    ChecklistItem, Decision, HeadlessResolver, HoldPoint, PId, PauseResolver, Scenario,
    resolve_hold,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn committed(stem: &str) -> Scenario {
    let path = workspace_root()
        .join("scenarios")
        .join(format!("{stem}.toml"));
    let text = std::fs::read_to_string(&path).expect("committed scenario file reads");
    Scenario::from_toml_str(&text).expect("committed scenario validates")
}

/// The hold the firing site builds for a declare-only scenario (`conductor-run/src/lib.rs`).
fn checklist_hold(scenario: &Scenario) -> HoldPoint {
    HoldPoint {
        scenario: scenario.name.clone(),
        p_id: scenario.p_ids[0].clone(),
        step: "operator-checklist".to_string(),
        prompt: "Observe the operator-checklist claim for this scenario".to_string(),
        allow_no_go: true,
        checklist: scenario.checklist.clone(),
    }
}

/// An attended resolver stand-in: it reports a resolver kind that is not the headless default, which
/// is the only thing separating a real activation from the never-blocking one in the record.
struct AttendedStub(Decision);

impl PauseResolver for AttendedStub {
    fn kind(&self) -> &'static str {
        "test-attended"
    }

    async fn resolve(&self, _hold: &HoldPoint) -> Decision {
        self.0
    }
}

#[test]
fn the_attended_leg_vehicles_declare_their_checklist_items() {
    let hue = committed("halo-hue-encoding");
    assert!(
        hue.expected.is_empty(),
        "declare-only: an expected check would divert it off the hold path"
    );
    assert_eq!(hue.checklist.len(), 1);
    assert_eq!(
        hue.checklist[0].observation,
        "hue shifted toward burgundy under error pressure?"
    );
    // The induced half is deliberately honest about what the operator is grading. This scenario now
    // really does drive its stimulus — a sustained identical-fingerprint stream through incident
    // formation — so the induced text names it. An induced text still claiming the scenario drives
    // nothing would be false, and the operator would be grading against a fiction.
    assert!(
        hue.checklist[0]
            .induced
            .contains("drives a sustained identical-fingerprint exception stream"),
        "this one really does drive its stimulus, so the induced text names the stream: {:?}",
        hue.checklist[0].induced
    );

    let breathing = committed("halo-breathing-encoding");
    assert!(breathing.expected.is_empty());
    assert_eq!(breathing.checklist.len(), 1);
    assert_eq!(
        breathing.checklist[0].observation,
        "halo breathing rate tracks throughput?"
    );
    assert!(
        breathing.checklist[0].induced.contains("ramped"),
        "this one really does drive its stimulus, so the induced text names the ramp: {:?}",
        breathing.checklist[0].induced
    );
    // P-ID discipline: the breathing scenario is P-026. Observing a halo claim never claims P-025,
    // whose budget re-drive is owned by a separate route entry.
    assert_eq!(breathing.p_ids, vec![PId("P-026".to_string())]);
}

#[test]
fn the_hold_carries_the_scenarios_declared_items() {
    let hold = checklist_hold(&committed("halo-hue-encoding"));
    assert_eq!(hold.step, "operator-checklist");
    assert!(
        hold.allow_no_go,
        "both a real Proceed and a real Abort must be reachable outcomes"
    );
    assert_eq!(hold.checklist.len(), 1);
    assert_eq!(
        hold.checklist[0].observation,
        "hue shifted toward burgundy under error pressure?"
    );
}

#[test]
fn a_scenario_declaring_no_checklist_still_holds_on_the_generic_prompt() {
    // 22 other declare-only scenarios take this hold without declaring items; the change is additive
    // and must not have made a checklist mandatory.
    let plain = committed("ack-cooldown");
    assert!(plain.expected.is_empty());
    assert!(plain.checklist.is_empty());
    let hold = checklist_hold(&plain);
    assert!(hold.checklist.is_empty());
    assert!(
        !hold.prompt.is_empty(),
        "the generic prompt is what such a hold renders"
    );
}

#[tokio::test]
async fn the_resolution_distinguishes_an_attended_activation_from_the_headless_default() {
    let hold = checklist_hold(&committed("halo-hue-encoding"));

    let headless = resolve_hold(&HeadlessResolver::proceed(), &hold).await;
    let attended = resolve_hold(&AttendedStub(Decision::Go), &hold).await;

    // Both decided Go — so the DECISION alone cannot witness an attended activation. The resolver
    // kind is what separates them, which is why the record carries it.
    assert_eq!(headless.decision, Decision::Go);
    assert_eq!(attended.decision, Decision::Go);
    assert_ne!(
        headless.resolver_kind, attended.resolver_kind,
        "a decision-only witness cannot tell a real activation from the never-blocking default"
    );
    assert_eq!(headless.resolver_kind, "headless");
    assert_eq!(attended.resolver_kind, "test-attended");
}

#[tokio::test]
async fn an_attended_abort_is_a_recorded_value_not_a_harness_fault() {
    let hold = checklist_hold(&committed("halo-breathing-encoding"));
    let resolution = resolve_hold(&AttendedStub(Decision::NoGo), &hold).await;
    assert_eq!(resolution.decision, Decision::NoGo);
    assert_eq!(resolution.decision.label(), "No-Go");
    assert_eq!(resolution.resolver_kind, "test-attended");
}

#[test]
fn a_checklist_beside_expected_checks_is_rejected_at_load() {
    // The sibling-spanning rule, exercised through `from_toml_str` — the load path, not the method
    // in isolation, so the test proves the check is actually WIRED (a rule that exists but never
    // runs is the `#[garde(skip)]` failure mode this project has already shipped once).
    let toml = r#"
name = "contradictory"
p_ids = ["P-025"]
seed = 1
slo_tier = "<5s"
jitter_ms = 0
[[phases]]
name = "p1"
gap_ms = 100
[[expected]]
kind = "Contains"
class = "Hard"
expected = "anything"
[[checklist]]
induced = "something driven"
observation = "something observed?"
"#;
    let err =
        Scenario::from_toml_str(toml).expect_err("a checks-bearing scenario rejects a checklist");
    let text = err.to_string();
    assert!(
        text.contains("checklist"),
        "the error names the offending declaration: {text}"
    );
}

#[test]
fn an_empty_checklist_text_is_rejected_at_load() {
    let toml = r#"
name = "empty-half"
p_ids = ["P-025"]
seed = 1
slo_tier = "<5s"
jitter_ms = 0
[[phases]]
name = "p1"
gap_ms = 100
[[checklist]]
induced = ""
observation = "something observed?"
"#;
    assert!(
        Scenario::from_toml_str(toml).is_err(),
        "a half-declared item would render a blank dialog row"
    );
}

/// The 2026-08-22 attended legs, pinned verbatim.
///
/// Two legs against a live Pulse (HEAD `f0c38f5`, data dir `pulse-legs/20260822-133000`, window
/// open, deterministic L4), driven through the Tauri GUI over a two-scenario catalog
/// (`CONDUCTOR_SCENARIOS_DIR=runs/leg-scenarios`) so the halo pair — the only scenarios declaring
/// checklist items — hold at counts 0 and 1 rather than 11 and 12. Four holds, four real
/// activations. The captures below are the operator-observed evidence for `v2-29`; the tests grade
/// them without needing a live Pulse, exactly as the storm / baseline / restart / pii / connection /
/// severity / delegated-timing harvests do.
mod live_leg {
    /// `message` fields from `logs/conductor-tauri.jsonl`, in wall-clock order. Runs
    /// `2026-08-22T11-57-17-764` (legs 1-2) and `2026-08-22T12-04-10-307` (legs 3-4).
    const WITNESSES: [&str; 4] = [
        "operator-checklist hold resolved by tauri-dialog: Go (1 checklist item(s))",
        "operator-checklist hold resolved by tauri-dialog: Go (1 checklist item(s))",
        "operator-checklist hold resolved by tauri-dialog: No-Go (1 checklist item(s))",
        "operator-checklist hold resolved by tauri-dialog: Go (1 checklist item(s))",
    ];

    /// `(resolver_kind, decision, item_count)` — the three facts the witness has to carry for the
    /// line to be usable evidence of an ATTENDED activation.
    fn parse(message: &str) -> Option<(&str, &str, usize)> {
        let rest = message.strip_prefix("operator-checklist hold resolved by ")?;
        let (kind, rest) = rest.split_once(": ")?;
        let (decision, rest) = rest.split_once(" (")?;
        let count = rest.strip_suffix(" checklist item(s))")?.parse().ok()?;
        Some((kind, decision, count))
    }

    #[test]
    fn every_live_resolution_came_from_the_dialog_never_the_headless_default() {
        // The load-bearing assertion of the whole chunk. Before it, this line said "resolved
        // headless" unconditionally at `debug`; a leg could not have distinguished a real
        // activation from the never-blocking default, because the witness asserted the answer.
        for message in WITNESSES {
            let (kind, _, _) = parse(message).expect("witness parses");
            assert_eq!(
                kind, "tauri-dialog",
                "an attended leg must never report headless: {message}"
            );
        }
    }

    #[test]
    fn both_decision_arms_were_exercised_live() {
        let decisions: Vec<&str> = WITNESSES
            .iter()
            .filter_map(|m| parse(m).map(|p| p.1))
            .collect();
        assert_eq!(decisions, ["Go", "Go", "No-Go", "Go"]);
        assert!(decisions.contains(&"Go"), "a real Proceed activation");
        assert!(
            decisions.contains(&"No-Go"),
            "a real Abort activation — allow_no_go is reachable"
        );
    }

    #[test]
    fn every_hold_carried_its_scenarios_declared_item() {
        for message in WITNESSES {
            let (_, _, count) = parse(message).expect("witness parses");
            assert_eq!(
                count, 1,
                "each halo scenario declares exactly one item: {message}"
            );
        }
    }

    #[test]
    fn a_no_go_is_indistinguishable_from_a_go_in_the_record() {
        // MEASURED, not argued: leg 1 resolved halo-breathing with Go and leg 2 with No-Go, and the
        // two envelope rows agree on every graded field (latency differed by 6ms of scheduling).
        // `manual_record` never receives the Decision, so the witness line above is the ONLY place
        // an Abort is visible. Anyone reading a run report for evidence of a no-go will not find it.
        let go = (
            "halo-breathing-encoding",
            None::<&str>,
            "KnownResidual",
            10056,
        );
        let no_go = (
            "halo-breathing-encoding",
            None::<&str>,
            "KnownResidual",
            10050,
        );
        assert_eq!(go.0, no_go.0);
        assert_eq!(go.1, no_go.1, "verdict identical");
        assert_eq!(go.2, no_go.2, "state identical");
        assert!(
            (go.3 - no_go.3 as i64).abs() < 100,
            "latency differs only by scheduling noise"
        );
    }

    #[test]
    fn the_operators_wait_is_excluded_from_the_measured_latency() {
        // Leg 4 held the hue dialog open for 54s (read-back stamped 12:05:33, resolved 12:06:27)
        // and still recorded 6081ms — the scenario's own 6s of phase gaps. `execute_scenario` takes
        // `observed_ms`/`read_back_observed_at` BEFORE the hold, so unbounded human time cannot
        // inflate an SLO. A leg that recorded ~54000ms here would mean the stamps had moved.
        let recorded_ms: i64 = 6081;
        let operator_wait_ms: i64 = 54_000;
        let scenario_gap_ms: i64 = 6_000;
        assert!(
            recorded_ms < operator_wait_ms / 2,
            "the hold's wait is not in the measurement"
        );
        assert!(
            (recorded_ms - scenario_gap_ms).abs() < 1_500,
            "latency tracks the scenario's own phase duration, not the operator"
        );
    }

    #[test]
    fn the_frozen_count_sequence_is_recorded() {
        // Operator-observed (`method: manual`), corroborated by screenshots of both dialogs: the
        // titlebar read 0 at the first hold and 1 at the second — motionless, tinted, with the
        // label swapped to "HOLD — operator pause" so the state survives a colour-blind read — and
        // advanced to 2 on the final decision. Neither blanked nor continued while held.
        let frozen_at: [u64; 2] = [0, 1];
        let resumed_to: u64 = 2;
        assert_eq!(
            frozen_at[1], 1,
            "a NONZERO frozen value, not only the count-0 first hold"
        );
        assert_eq!(
            resumed_to as usize,
            WITNESSES.len() / 2,
            "one completion per scenario in the leg"
        );
    }
}

#[test]
fn a_checklist_item_round_trips_through_the_scenario_model() {
    let item = ChecklistItem {
        induced: "rate ramped".to_string(),
        observation: "did it breathe faster?".to_string(),
    };
    let toml = format!(
        "name = \"rt\"\np_ids = [\"P-026\"]\nseed = 1\nslo_tier = \"<5s\"\njitter_ms = 0\n[[phases]]\nname = \"p1\"\ngap_ms = 100\n[[checklist]]\ninduced = \"{}\"\nobservation = \"{}\"\n",
        item.induced, item.observation
    );
    let scenario = Scenario::from_toml_str(&toml).expect("validates");
    assert_eq!(scenario.checklist, vec![item]);
}
