//! Pins the committed coverage-lamp fixture the webview `--e2e` arm is seeded with.
//!
//! The fixture's RECORD CONTENT is a committed artifact (`tests/fixtures/lamps-journal.jsonl`), not a
//! journal hand-written inline in `wdio.conf.ts`, so it is reviewable — and this test round-trips it
//! through `conductor_core::read_run_journal`, the production reader `run_report` single-sources,
//! performing the SAME copy-then-read the wdio seed performs.
//!
//! It asserts the fixture's SEMANTICS, not merely that it parses: a change that leaves the file
//! readable but no longer collision-bearing fails HERE, loudly, instead of leaving the e2e green over
//! a fixture that stopped testing anything.

use std::collections::BTreeMap;

use conductor_core::{Lamp, read_run_journal};

/// The run-id stem the seed copies the fixture to — `read_run_journal` resolves `<run_id>.jsonl`.
/// `wdio.conf.ts` copies to this same stem; a divergence breaks the e2e, not this test.
const FIXTURE_RUN_ID: &str = "lamps-fixture";

/// The collided P-ID: named by two records whose lamps differ, so the worst-lamp-wins join has
/// something to decide. `Blocked` outranks `Pass`, so the coverage row must render Blocked.
const COLLIDED_P_ID: &str = "P-019";

fn seeded_journal() -> (assert_fs::TempDir, Vec<conductor_core::RunRecord>) {
    let dir = assert_fs::TempDir::new().unwrap();
    let fixture = format!("{}/tests/fixtures/lamps-journal.jsonl", env!("CARGO_MANIFEST_DIR"));
    std::fs::copy(&fixture, dir.path().join(format!("{FIXTURE_RUN_ID}.jsonl")))
        .expect("the committed fixture copies into the runs dir the way the wdio seed copies it");
    let records = read_run_journal(dir.path(), FIXTURE_RUN_ID)
        .expect("the fixture parses through the production journal reader");
    (dir, records)
}

#[test]
fn the_fixture_carries_a_collision_whose_lamps_differ() {
    let (_dir, records) = seeded_journal();
    assert!(
        records.len() >= 3,
        "the e2e needs a populated row, a collided row and an untouched P-ID: {} records",
        records.len()
    );

    let mut lamps_by_p_id: BTreeMap<&str, Vec<Lamp>> = BTreeMap::new();
    for record in &records {
        for p_id in &record.p_ids {
            lamps_by_p_id.entry(p_id.0.as_str()).or_default().push(Lamp::for_record(record));
        }
    }

    let collided = lamps_by_p_id
        .get(COLLIDED_P_ID)
        .unwrap_or_else(|| panic!("{COLLIDED_P_ID} is the collision the join is asserted on"));
    assert_eq!(
        collided.len(),
        2,
        "{COLLIDED_P_ID} must be named by exactly two records: {collided:?}"
    );
    assert_ne!(
        collided[0], collided[1],
        "a collision whose lamps agree decides nothing — the join would pass under either rule"
    );
    assert!(
        collided.contains(&Lamp::Blocked) && collided.contains(&Lamp::Pass),
        "the e2e expects Blocked to beat Pass on this row: {collided:?}"
    );
}

#[test]
fn the_fixture_leaves_a_classified_capability_unmentioned() {
    let (_dir, records) = seeded_journal();
    let named: Vec<&str> =
        records.iter().flat_map(|r| r.p_ids.iter().map(|p| p.0.as_str())).collect();

    let unmentioned = conductor_core::coverage_matrix()
        .iter()
        .find(|row| !named.contains(&row.p_id))
        .map(|row| row.p_id);

    assert!(
        unmentioned.is_some(),
        "the e2e asserts a not-yet-run cell, so some classified capability must go unmentioned"
    );
}
