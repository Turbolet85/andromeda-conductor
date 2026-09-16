//! The scenario-assertion audit gate — the committed catalog agrees with the committed audit ledger
//! on both axes, in both directions (test-plan §6; `verification-matrix.json#v3-06`).
//!
//! Three kinds of arm, and the negative ones are the point. The POSITIVE arm asserts
//! `check_scenario_audit` over the REAL committed pair. The NEGATIVE arms mutate that REAL ledger —
//! dropping each row in turn, then planting a row that no longer describes the catalog, then a row
//! naming no scenario at all — so each of the six conditions is proven able to fail rather than
//! assumed to be. The CONTROL arms pin the two properties the gate's own correctness rests on: that
//! the tier set is closed at the type level, and that the retired-gloss sweep sees a gloss the plain
//! single-line form misses.
//!
//! Paths resolve from `CARGO_MANIFEST_DIR`, never a CWD-relative default: a cargo test binary runs
//! with its CWD at the package root, where neither `contracts/` nor `scenarios/` exists.

use std::path::{Path, PathBuf};

use conductor_core::{
    CapabilityManifest, LiveAssertion, OverTier, Scenario, ScenarioAuditLedger,
    carries_retired_gloss, check_scenario_audit, load_catalog,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn ledger() -> ScenarioAuditLedger {
    ScenarioAuditLedger::load(&repo_root().join("contracts/scenario-audit-ledger.toml"))
        .expect("the committed scenario audit ledger loads")
}

fn catalog() -> Vec<Scenario> {
    let manifest = CapabilityManifest::load(&repo_root().join("contracts/pulse-capabilities.toml"))
        .expect("the committed capability manifest loads");
    load_catalog(&repo_root().join("scenarios"), &manifest).expect("the committed catalog loads")
}

#[test]
fn the_committed_catalog_matches_the_committed_ledger() {
    let ledger = ledger();
    let catalog = catalog();

    // Vacuity guards: a gate whose subject is empty passes without asserting anything.
    assert!(
        !catalog.is_empty(),
        "the catalog is empty — the gate would pass vacuously"
    );
    assert!(
        !ledger.live_assertion.is_empty(),
        "the ledger pins no live assertion — the gate's assertion axis would pass vacuously"
    );
    assert!(
        !ledger.over_tier.is_empty(),
        "the ledger pins no over-tier scenario — the gate's tier axis would pass vacuously"
    );

    check_scenario_audit(&ledger, &catalog)
        .expect("the committed catalog agrees with the committed ledger on both axes");
}

#[test]
fn dropping_any_pinned_row_fails_the_gate() {
    let ledger = ledger();
    let catalog = catalog();

    for dropped in &ledger.live_assertion {
        let mut reduced = ledger.clone();
        reduced
            .live_assertion
            .retain(|a| a.scenario != dropped.scenario);
        let rendered = check_scenario_audit(&reduced, &catalog)
            .expect_err("a live assertion the ledger does not pin must fail the gate")
            .to_string();
        assert!(
            rendered.contains(&dropped.scenario),
            "the audit message must name the unpinned scenario {}, got: {rendered}",
            dropped.scenario
        );
    }

    for dropped in &ledger.over_tier {
        let mut reduced = ledger.clone();
        reduced.over_tier.retain(|o| o.scenario != dropped.scenario);
        let rendered = check_scenario_audit(&reduced, &catalog)
            .expect_err("an over-tier scenario the ledger does not pin must fail the gate")
            .to_string();
        assert!(
            rendered.contains(&dropped.scenario),
            "the audit message must name the unpinned scenario {}, got: {rendered}",
            dropped.scenario
        );
    }
}

#[test]
fn a_pinned_row_that_no_longer_describes_the_catalog_fails_the_gate() {
    let catalog = catalog();

    // Pin rot is what stops a ledger becoming a permanent excuse: the subject exists, but the
    // property the row claims about it is no longer true.
    let fits_its_tier = catalog
        .iter()
        .find(|s| !ledger().over_tier.iter().any(|o| o.scenario == s.name))
        .expect("some committed scenario fits its declared tier")
        .name
        .clone();

    let mut rotted = ledger();
    rotted.over_tier.push(OverTier {
        scenario: fits_its_tier.clone(),
        reason: "synthetic pin-rot row".into(),
    });
    let rendered = check_scenario_audit(&rotted, &catalog)
        .expect_err("a scenario pinned over-tier that fits its tier must fail the gate")
        .to_string();
    assert!(
        rendered.contains(&fits_its_tier),
        "the audit message must name the rotted pin {fits_its_tier}, got: {rendered}"
    );

    let declares_no_check = catalog
        .iter()
        .find(|s| s.expected.is_empty())
        .expect("some committed scenario declares no check")
        .name
        .clone();

    let mut rotted = ledger();
    rotted.live_assertion.push(LiveAssertion {
        scenario: declares_no_check.clone(),
        ground: "synthetic pin-rot row".into(),
        discriminates: true,
        weakness: String::new(),
    });
    let rendered = check_scenario_audit(&rotted, &catalog)
        .expect_err("a scenario pinned as a live assertion that declares none must fail the gate")
        .to_string();
    assert!(
        rendered.contains(&declares_no_check),
        "the audit message must name the rotted pin {declares_no_check}, got: {rendered}"
    );
}

#[test]
fn a_pinned_row_naming_no_scenario_fails_the_gate() {
    let catalog = catalog();
    const ABSENT: &str = "no-such-scenario";

    let mut orphaned = ledger();
    orphaned.over_tier.push(OverTier {
        scenario: ABSENT.into(),
        reason: "synthetic lost-subject row".into(),
    });
    let rendered = check_scenario_audit(&orphaned, &catalog)
        .expect_err("an over-tier pin naming no scenario must fail the gate")
        .to_string();
    assert!(
        rendered.contains(ABSENT),
        "the audit message must name the lost subject {ABSENT}, got: {rendered}"
    );

    let mut orphaned = ledger();
    orphaned.live_assertion.push(LiveAssertion {
        scenario: ABSENT.into(),
        ground: "synthetic lost-subject row".into(),
        discriminates: true,
        weakness: String::new(),
    });
    let rendered = check_scenario_audit(&orphaned, &catalog)
        .expect_err("a live-assertion pin naming no scenario must fail the gate")
        .to_string();
    assert!(
        rendered.contains(ABSENT),
        "the audit message must name the lost subject {ABSENT}, got: {rendered}"
    );
}

#[test]
fn the_tier_set_is_closed_at_the_type_level() {
    // The closed set needs no corpus scan: an out-of-set tier cannot survive deserialization, so a
    // fourth `SloTier` variant would break this arm rather than silently widening the ladder.
    let minted = r#"
name = "out-of-set-tier"
p_ids = ["P-005"]
seed = 1
slo_tier = "<45s"
jitter_ms = 0

[[phases]]
name = "only"
gap_ms = 1000
"#;
    assert!(
        Scenario::from_toml_str(minted).is_err(),
        "a tier outside the closed set must fail to deserialize"
    );
}

#[test]
fn the_sweep_sees_a_wrapped_gloss_the_plain_form_misses() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/wrapped-gloss.toml");
    let text = std::fs::read_to_string(&fixture).expect("the wrapped-gloss control fixture loads");

    assert!(
        carries_retired_gloss(&text),
        "the comment-marker-stripped sweep must detect a gloss wrapped across two comment lines"
    );
    assert!(
        !text.lines().any(|line| line.contains("MCP round-trip")),
        "the fixture must carry the gloss WRAPPED — a plain single-line search finding it would \
         mean the control no longer proves the comment-strip-and-join is load-bearing"
    );
}

#[test]
fn no_committed_scenario_carries_a_retired_gloss() {
    let catalog_dir = repo_root().join("scenarios");
    let files = conductor_core::scenario_files(&catalog_dir).expect("the catalog enumerates");
    assert!(!files.is_empty(), "no scenario files — a vacuous sweep");

    let carrying: Vec<String> = files
        .iter()
        .filter(|path| {
            std::fs::read_to_string(path)
                .map(|text| carries_retired_gloss(&text))
                .unwrap_or(false)
        })
        .map(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string()
        })
        .collect();

    assert!(
        carrying.is_empty(),
        "committed scenarios still carry a retired latency gloss: {}",
        carrying.join(", ")
    );
}
