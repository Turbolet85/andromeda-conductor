//! The run-journal conformance gate — every `runs/<run_id>.jsonl` row schema-complete and
//! host-path-free, red on violation (obs-plan §9/§10, which reserved this gate as "not-yet-built").
//!
//! Four properties per line. SHAPE is the typed parse via `conductor_core::classify_journal_line`,
//! the same rule the production reader uses. KEY PRESENCE is a separate `serde_json::Value`
//! key-set check, because a typed parse cannot see an absent `Option` key: serde reads a missing
//! `Option` field as `None`, so the envelope's five nullable keys can all be gone from a line that
//! deserializes cleanly. CLOSED SETS ride the typed parse (`verdict` / `state` are enums, so a
//! foreign value fails the parse). HOST-PATH FREEDOM is `conductor_core::redact_value` — the one
//! definition of a host path (obs-plan §11 single-location ownership), never a second regex.
//!
//! The required key NAMES are derived from freshly serialized reference records rather than
//! hand-listed, so they cannot drift from the structs; the arity pins below are what make a field
//! added or removed to the contract fail HERE, loudly.
//!
//! Subjects: the committed lamps fixture, a journal produced in-test through the production writer
//! `conductor_run::persist`, and — when `CONDUCTOR_RUNS_DIR` is set — every journal in that
//! directory. That last is how the CI step reuses this gate over a real produced artifact instead of
//! re-listing the schema in `jq`. Unset, the gate asserts over its own subjects only.

use std::collections::BTreeSet;
use std::path::Path;

use conductor_core::{
    CheckRecord, ComparisonKind, EnvelopeStatus, JournalLine, PId, ReportState, RunRecord, SloTier,
    Verdict, classify_journal_line, redact_value,
};

/// The contract's arity (arch §Standard Contracts): eleven envelope fields, nine check-record keys.
const ENVELOPE_FIELDS: usize = 11;
const CHECK_FIELDS: usize = 9;

fn reference_envelope() -> RunRecord {
    RunRecord::measured(
        "2026-09-06T00-00-00-ref",
        424242,
        "reference",
        vec![PId("P-009".to_string())],
        Verdict::Pass,
        ReportState::Pass,
        "2026-09-06T00:00:00Z",
        "2026-09-06T00:00:01Z",
        1000,
        SloTier::Tier5s,
        vec!["fp-1".to_string()],
    )
}

fn reference_check(run_id: &str) -> CheckRecord {
    CheckRecord {
        run_id: run_id.to_string(),
        scenario: "reference".to_string(),
        check_index: 0,
        kind: ComparisonKind::Contains,
        verdict: Verdict::Pass,
        state: ReportState::Pass,
        latency_ms: 1000,
        deadline_ms: 5000,
        budget_ms: None,
    }
}

fn keys(value: &serde_json::Value) -> BTreeSet<String> {
    value
        .as_object()
        .expect("a journal row is a JSON object")
        .keys()
        .cloned()
        .collect()
}

fn envelope_keys() -> BTreeSet<String> {
    keys(&serde_json::to_value(reference_envelope()).unwrap())
}

fn check_keys() -> BTreeSet<String> {
    keys(&serde_json::to_value(reference_check("r")).unwrap())
}

/// Judge one line against the four properties. `Err` carries what was observed, so a failure names
/// the defect rather than reporting a count.
fn conformance(line: &str) -> Result<(), String> {
    let value: serde_json::Value =
        serde_json::from_str(line).map_err(|e| format!("not parseable as JSON: {e}"))?;
    if !value.is_object() {
        return Err("not a JSON object".to_string());
    }

    // (a) shape + (c) closed sets — a foreign `verdict`/`state` fails the typed parse.
    let required = match classify_journal_line(line) {
        Ok(JournalLine::Envelope(_)) => envelope_keys(),
        Ok(JournalLine::Check(_)) => check_keys(),
        Err(e) => return Err(format!("conforms to neither registered shape: {e}")),
    };

    // (b) key presence — extra keys are allowed (obs-plan §3 keeps the extension point open).
    let present = keys(&value);
    let missing: Vec<&String> = required.difference(&present).collect();
    if !missing.is_empty() {
        return Err(format!("keys absent from the row: {missing:?}"));
    }

    // (d) host-path freedom, over every string the row carries at either depth.
    for (key, field) in value.as_object().unwrap() {
        let strings = field
            .as_str()
            .map(|s| vec![s])
            .or_else(|| {
                field
                    .as_array()
                    .map(|a| a.iter().filter_map(|i| i.as_str()).collect())
            })
            .unwrap_or_default();
        for s in strings {
            if redact_value(s).as_ref() != s {
                return Err(format!("field `{key}` carries an absolute host path"));
            }
        }
    }
    Ok(())
}

fn assert_journal_conforms(path: &Path, origin: &str) -> usize {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{origin} is readable: {e}"));
    let mut rows = 0;
    for (index, line) in text.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        if let Err(why) = conformance(line) {
            panic!("{origin} line {}: {why}", index + 1);
        }
        rows += 1;
    }
    rows
}

/// The arity pin — a field added to or removed from either contract shape fails here, naming the
/// count, before any subject is judged against a set that silently moved.
#[test]
fn the_two_registered_shapes_carry_their_contract_arity() {
    assert_eq!(
        envelope_keys().len(),
        ENVELOPE_FIELDS,
        "the run-report envelope is eleven fields (arch §Standard Contracts): {:?}",
        envelope_keys()
    );
    assert_eq!(
        check_keys().len(),
        CHECK_FIELDS,
        "the per-check record is nine keys (arch §Standard Contracts): {:?}",
        check_keys()
    );
}

#[test]
fn the_committed_fixture_journal_conforms() {
    let path = format!(
        "{}/tests/fixtures/lamps-journal.jsonl",
        env!("CARGO_MANIFEST_DIR")
    );
    let rows = assert_journal_conforms(Path::new(&path), "the committed lamps journal");
    assert!(rows > 0, "the fixture has rows to judge");
}

/// The production writer's own output, carrying BOTH registered shapes plus the rows whose nulls a
/// naive completeness check would reject: a Blocked row (five measurement fields null) and a
/// governed `CalibrationRegion`/`ManualCheck` co-occurrence (a11y-plan §6 crosswalk).
#[test]
fn a_journal_written_by_the_production_writer_conforms_across_both_shapes() {
    let dir = assert_fs::TempDir::new().unwrap();
    let run_id = "2026-09-06T00-00-00-gate";
    let records = vec![
        reference_envelope(),
        RunRecord::blocked(
            run_id,
            7,
            "blocked-row",
            vec![PId("P-003".to_string())],
            SloTier::Tier20s,
        ),
        RunRecord::measured(
            run_id,
            424242,
            "calibration-row",
            vec![PId("P-008".to_string())],
            Verdict::CalibrationRegion,
            ReportState::ManualCheck,
            "2026-09-06T00:00:00Z",
            "2026-09-06T00:00:01Z",
            1000,
            SloTier::Tier90s,
            Vec::new(),
        ),
    ];
    conductor_run::persist(
        dir.path(),
        run_id,
        &records,
        &[reference_check(run_id)],
        &EnvelopeStatus::InEnvelope,
    )
    .expect("the production writer persists the subject");

    let path = dir.path().join(format!("{run_id}.jsonl"));
    let rows = assert_journal_conforms(&path, "a production-written journal");
    assert_eq!(
        rows,
        records.len() + 1,
        "both registered shapes reached the journal — three envelopes and one check record"
    );
}

/// The CI step's arm: judge whatever the pointed-at runs dir holds. Unset — every ordinary suite
/// run — this is a no-op, so the gate never depends on host-local residue.
#[test]
fn every_journal_in_a_pointed_at_runs_dir_conforms() {
    let Ok(target) = std::env::var("CONDUCTOR_RUNS_DIR") else {
        return;
    };
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&target);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        panic!("CONDUCTOR_RUNS_DIR names {target}, which is not a readable directory");
    };
    let mut journals = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|x| x.to_str()) == Some("jsonl") {
            // The origin names the handle + the file STEM, never the resolved path: this gate's own
            // failure output is subject to the rule it enforces (security-plan §Anti-Patterns →
            // Logging), and `path.display()` here is an absolute host path.
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("<journal>");
            assert_journal_conforms(&path, &format!("{target}/{name}"));
            journals += 1;
        }
    }
    assert!(
        journals > 0,
        "CONDUCTOR_RUNS_DIR names {target}, which holds no journal to judge — the producing leg \
         never fired, and a silent pass over an empty directory would prove nothing"
    );
}

// ---- negative arms: a gate never exercised against a violation is not known to fail ----

fn envelope_line() -> serde_json::Value {
    serde_json::to_value(reference_envelope()).unwrap()
}

fn without(key: &str) -> String {
    let mut value = envelope_line();
    value
        .as_object_mut()
        .unwrap()
        .remove(key)
        .expect("the key is there to remove");
    value.to_string()
}

/// Caught by the typed parse: a REQUIRED key is not `Option`, so serde refuses the line.
#[test]
fn a_row_missing_a_required_key_fails_the_gate() {
    let why = conformance(&without("seed")).unwrap_err();
    assert!(
        why.contains("neither registered shape") && why.contains("seed"),
        "the failure names the missing required key: {why}"
    );
}

/// Caught ONLY by the key-set check. serde deserializes an absent `Option` field as `None`, so this
/// line parses cleanly into a `RunRecord` — an arm that removed a required key instead would never
/// exercise the presence property at all.
#[test]
fn a_row_missing_an_optional_key_fails_the_gate() {
    let line = without("latency_ms");
    assert!(
        serde_json::from_str::<RunRecord>(&line).is_ok(),
        "precondition: the typed parse ACCEPTS this line, which is why presence needs its own check"
    );
    let why = conformance(&line).unwrap_err();
    assert!(
        why.contains("keys absent") && why.contains("latency_ms"),
        "the failure names the absent optional key: {why}"
    );
}

#[test]
fn a_row_carrying_an_absolute_host_path_fails_the_gate() {
    let mut value = envelope_line();
    value.as_object_mut().unwrap().insert(
        "scenario".to_string(),
        serde_json::Value::String("C:\\Users\\turbo\\runs\\leak".to_string()),
    );
    let why = conformance(&value.to_string()).unwrap_err();
    assert!(
        why.contains("scenario") && why.contains("host path"),
        "the failure names the leaking field: {why}"
    );
}

#[test]
fn a_conformant_row_passes_every_arm() {
    conformance(&envelope_line().to_string()).expect("the reference envelope conforms");
    conformance(
        &serde_json::to_value(reference_check("r"))
            .unwrap()
            .to_string(),
    )
    .expect("the reference check record conforms");
}
