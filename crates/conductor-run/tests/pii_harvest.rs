//! Live-leg evidence harvest for the pii-scrub family (P-035/P-047/P-048).
//!
//! The family's five authored read-back checks were retired to declare-only after measurement: no
//! MCP read-back surface varies with the emitted payload under deterministic L4, so the four
//! `Absent` sentinels passed VACUOUSLY and the `Contains "user.email"` structure marker failed
//! STRUCTURALLY (run `2026-08-19T20-37-25-933`, checks intact on that leg — the fingerprint-storm
//! vacuous-green precedent). The live claim therefore grades here, on Pulse's own tracing lines,
//! exactly as the storm / baseline / restart harvests do. Scrub semantics themselves are
//! byte-verified at SUT source (whole-field `[REDACTED:{category}]` markers at persistence;
//! fingerprint computed over the RAW pre-scrub stacktrace; the `spans` and `log_records` tables
//! store NO attribute columns, so P-048 holds by schema for attributes) — Pulse exposes no
//! external scrub observable (no counter, no per-redaction line; recorded as SUT-visit intake),
//! so what a leg CAN witness is ingestion: every carrier's rows landing intact.
//!
//! Witnesses measured live 2026-08-19 (run `2026-08-19T20-42-57-839`, SUT at HEAD `efabe8e`,
//! fresh data dir) and pinned VERBATIM below:
//!
//! - The two pii trace batches: `duckdb.append` `spans` with `rows_appended: 2` (root
//!   `pii.attributes` + child `pii.exception`), one per occurrence — per-occurrence corpus AND
//!   span identity via the dispatcher's `emission_seed`, so no `(trace_id, span_id)` PK collision.
//! - The two exception events: `span_events` with `rows_appended: 1` per occurrence.
//! - The two log batches: `log_records` with `rows_appended: 7` per occurrence — the PK-collision
//!   fix's live witness: `pii_log_record` stamps each of the seven same-severity records with a
//!   distinct `time_unix_nano` (base + index), so none drops on Pulse's
//!   `(ts_unix_nano, resource_hash, severity_number)` primary key. A value below 7 here means a
//!   same-stamp drop returned.
//! - The preflight canary's storm ladder on the same leg: `triage.pattern.storm.detected` at
//!   `occurrence_count: 5 / suggested` then `10 / autonomous` — the Tier-1 incident former behind
//!   the row's non-Blocked `KnownResidual` landing (verdict `null`, latency 4115 ms, `<5s` held).
//!
//! TEST-ONLY affordance (the storm/baseline/restart-harvest precedent): nothing here is wired into
//! the run path and nothing harvested reaches a Conductor artifact. Capture basis on Windows:
//! `{data_dir}/logs/agent-latest.jsonl.<date>`, sliced to the leg window by a pre-leg line count
//! (3,280 pre-leg / 35,231 post-leg for this run).

use conductor_emit::{PiiCategory, PiiCorpus};

/// One `duckdb.append` observation (Pulse's production `append_table_traced` line).
#[derive(Debug, Clone, PartialEq, Eq)]
struct DuckdbAppend {
    table_name: String,
    rows_appended: u64,
}

/// One `triage.pattern.storm.detected` observation (the canary's cue ladder).
#[derive(Debug, Clone, PartialEq, Eq)]
struct StormDetected {
    cue_kind: String,
    occurrence_count: u64,
    severity_hint: String,
}

fn parsed<'a>(lines: &'a [&'a str]) -> impl Iterator<Item = serde_json::Value> + 'a {
    lines
        .iter()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
}

fn is_target<'v>(v: &'v serde_json::Value, target: &str) -> Option<&'v serde_json::Value> {
    (v.get("target").and_then(|t| t.as_str()) == Some(target)).then(|| v.get("fields"))?
}

/// A line that does not parse, is not this target, or is missing a field is SKIPPED rather than
/// fatal — a harvest is other-process output (the verdict/error wall applied to evidence).
fn parse_duckdb_appends(lines: &[&str]) -> Vec<DuckdbAppend> {
    parsed(lines)
        .filter_map(|v| {
            let f = is_target(&v, "duckdb.append")?.clone();
            Some(DuckdbAppend {
                table_name: f.get("table_name")?.as_str()?.to_owned(),
                rows_appended: f.get("rows_appended")?.as_u64()?,
            })
        })
        .collect()
}

fn parse_storm_detected(lines: &[&str]) -> Vec<StormDetected> {
    parsed(lines)
        .filter_map(|v| {
            let f = is_target(&v, "triage.pattern.storm.detected")?.clone();
            Some(StormDetected {
                cue_kind: f.get("cue_kind")?.as_str()?.to_owned(),
                occurrence_count: f.get("occurrence_count")?.as_u64()?,
                severity_hint: f.get("severity_hint")?.as_str()?.to_owned(),
            })
        })
        .collect()
}

/// The leg's verbatim witness lines, byte-identical to the 2026-08-19 capture (run
/// `2026-08-19T20-42-57-839`): the two pii span batches with their exception events interleaved,
/// the two 7-row log batches, and the canary storm ladder.
const PINNED_LEG_LINES: &[&str] = &[
    r#"{"fields":{"deployment.environment":"production","duration_ms":3,"rows_appended":2,"service.name":"com.andromeda.pulse","service.version":"0.1.0","table_name":"spans"},"level":"INFO","message":"Arrow appender wrote rows","target":"duckdb.append","timestamp":"2026-08-19T20:43:44.999Z"}"#,
    r#"{"fields":{"deployment.environment":"production","duration_ms":3,"rows_appended":1,"service.name":"com.andromeda.pulse","service.version":"0.1.0","table_name":"span_events"},"level":"INFO","message":"Arrow appender wrote rows","target":"duckdb.append","timestamp":"2026-08-19T20:43:45.003Z"}"#,
    r#"{"fields":{"deployment.environment":"production","duration_ms":3,"rows_appended":2,"service.name":"com.andromeda.pulse","service.version":"0.1.0","table_name":"spans"},"level":"INFO","message":"Arrow appender wrote rows","target":"duckdb.append","timestamp":"2026-08-19T20:43:46.034Z"}"#,
    r#"{"fields":{"deployment.environment":"production","duration_ms":2,"rows_appended":1,"service.name":"com.andromeda.pulse","service.version":"0.1.0","table_name":"span_events"},"level":"INFO","message":"Arrow appender wrote rows","target":"duckdb.append","timestamp":"2026-08-19T20:43:46.037Z"}"#,
    r#"{"fields":{"deployment.environment":"production","duration_ms":3,"rows_appended":7,"service.name":"com.andromeda.pulse","service.version":"0.1.0","table_name":"log_records"},"level":"INFO","message":"Arrow appender wrote rows","target":"duckdb.append","timestamp":"2026-08-19T20:43:47.037Z"}"#,
    r#"{"fields":{"deployment.environment":"production","duration_ms":4,"rows_appended":7,"service.name":"com.andromeda.pulse","service.version":"0.1.0","table_name":"log_records"},"level":"INFO","message":"Arrow appender wrote rows","target":"duckdb.append","timestamp":"2026-08-19T20:43:48.036Z"}"#,
    r#"{"fields":{"cue_kind":"retry_storm","deployment.environment":"production","fingerprint_hex":"b5fe50e7","occurrence_count":5,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_hint":"suggested","window_seconds":30},"level":"INFO","message":"","target":"triage.pattern.storm.detected","timestamp":"2026-08-19T20:43:42.977Z"}"#,
    r#"{"fields":{"cue_kind":"retry_storm","deployment.environment":"production","fingerprint_hex":"b5fe50e7","occurrence_count":10,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_hint":"autonomous","window_seconds":30},"level":"INFO","message":"","target":"triage.pattern.storm.detected","timestamp":"2026-08-19T20:43:43.012Z"}"#,
];

/// The per-slot emission seed, transcribed from the dispatcher's `emission_seed`
/// (`crates/conductor-run/src/dispatch.rs:197-201` — private there, so transcribed rather than
/// re-exported) applied to the pii-scrub scenario's TOML-declared seed 4317035. The leg ran with
/// no `SEED` override, so these four slots are exactly the corpora the leg emitted.
fn leg_emission_seed(phase_index: u64, occurrence: u64) -> u64 {
    4_317_035_u64
        .wrapping_mul(0x9e37_79b9_7f4a_7c15)
        .wrapping_add(phase_index.wrapping_mul(0xbf58_476d_1ce4_e5b9))
        .wrapping_add(occurrence.wrapping_mul(0x94d0_49bb_1331_11eb))
}

/// The four corpora the leg emitted: (spans phase 0, logs phase 1) x (occurrence 0, 1).
fn leg_corpora() -> Vec<PiiCorpus> {
    [(0, 0), (0, 1), (1, 0), (1, 1)]
        .into_iter()
        .map(|(phase, occ)| PiiCorpus::seeded(leg_emission_seed(phase, occ)))
        .collect()
}

#[test]
fn pii_span_batches_appended_intact() {
    let appends = parse_duckdb_appends(PINNED_LEG_LINES);
    let span_batches: Vec<&DuckdbAppend> =
        appends.iter().filter(|a| a.table_name == "spans").collect();
    assert_eq!(
        span_batches.len(),
        2,
        "one spans batch per occurrence: {appends:?}"
    );
    assert!(
        span_batches.iter().all(|a| a.rows_appended == 2),
        "each pii trace batch lands both spans (root pii.attributes + child pii.exception) — a \
         value below 2 means a span-identity PK drop: {span_batches:?}"
    );
}

#[test]
fn pii_exception_events_appended() {
    let appends = parse_duckdb_appends(PINNED_LEG_LINES);
    let event_batches: Vec<&DuckdbAppend> = appends
        .iter()
        .filter(|a| a.table_name == "span_events")
        .collect();
    assert_eq!(
        event_batches.len(),
        2,
        "one exception event per occurrence: {appends:?}"
    );
    assert!(
        event_batches.iter().all(|a| a.rows_appended == 1),
        "{event_batches:?}"
    );
}

#[test]
fn pii_log_records_appended_without_pk_collision_drops() {
    let appends = parse_duckdb_appends(PINNED_LEG_LINES);
    let log_batches: Vec<&DuckdbAppend> = appends
        .iter()
        .filter(|a| a.table_name == "log_records")
        .collect();
    assert_eq!(
        log_batches.len(),
        2,
        "one log batch per occurrence: {appends:?}"
    );
    assert!(
        log_batches.iter().all(|a| a.rows_appended == 7),
        "all seven same-severity records land — a value below 7 means a same-nanosecond stamp \
         collided on Pulse's (ts_unix_nano, resource_hash, severity_number) primary key: \
         {log_batches:?}"
    );
}

#[test]
fn canary_storm_ladder_reached_autonomous_on_the_same_leg() {
    let storms = parse_storm_detected(PINNED_LEG_LINES);
    assert_eq!(
        storms.len(),
        2,
        "the ladder fires suggested then autonomous: {storms:?}"
    );
    assert!(
        storms.iter().all(|s| s.cue_kind == "retry_storm"),
        "one fingerprint, one cue family: {storms:?}"
    );
    assert_eq!(
        (storms[0].occurrence_count, storms[0].severity_hint.as_str()),
        (5, "suggested"),
        "the suggested floor fires at 5: {storms:?}"
    );
    assert_eq!(
        (storms[1].occurrence_count, storms[1].severity_hint.as_str()),
        (10, "autonomous"),
        "the autonomous band (the Tier-1 incident former) fires at 10: {storms:?}"
    );
}

#[test]
fn no_corpus_value_or_affix_reaches_a_pinned_line() {
    // The leg's exact emitted values, recomputed from the TOML seed — plus the generator's stable
    // affixes, so a leak is caught even if a recomputation ever drifted from the wire.
    let mut needles: Vec<String> = leg_corpora()
        .iter()
        .flat_map(|corpus| {
            PiiCategory::all()
                .into_iter()
                .map(|c| corpus.value(c).to_owned())
                .collect::<Vec<_>>()
        })
        .collect();
    needles.extend(
        [
            "@example.com",
            "sk_live_",
            "Bearer ",
            "password=",
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
            "ghp_",
        ]
        .map(str::to_owned),
    );
    assert_eq!(needles.len(), 38, "4 corpora x 8 values + 6 stable affixes");
    for line in PINNED_LEG_LINES {
        for needle in &needles {
            assert!(
                !line.contains(needle.as_str()),
                "a committed pin must never carry corpus content (found {needle:?})"
            );
        }
    }
}
