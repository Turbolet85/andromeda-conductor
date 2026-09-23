//! Live-leg harvest for the real-model interpretation leg (`verification-matrix.json#v3-09`).
//!
//! The graded half of the capture/grade split: `real_model_live.rs` (feature `live-pulse`) PRINTS what
//! the one drive produced, and this target — in the DEFAULT suite — grades it. It reads no path at
//! grading time: the capture is pinned here as literals, the `live_suite_harvest.rs` convention.
//!
//! THE RULE is the span between the two marker lines below. It was written before the drive, and the
//! capture's `rule_record` prints it into `evidence/rm-capture.txt` BEFORE the leg fires, so the
//! committed capture proves the rule was not edited after the model answered. Everything outside the
//! markers is fixture, pin or test. The rule grades whatever happened; nobody re-judges an outcome.
//!
//! STATED LIMIT (fixed with the rule): the digest's cue line reaches the model verbatim —
//! `[autonomous] retry_storm — retry_storm scope_id=conductor` (andromeda-pulse
//! `crates/triage/src/digest/assembler.rs:664-673`, inserted whole by `interpretation/src/prompt.rs`)
//! — and that line itself satisfies the rule. So `Identified` means the real, non-canned model carried
//! the cue's scope and kind into rank 1, n=1: never inference of an unstated cause, and never a choice
//! among competing causes.
//!
//! The synthetic arms render Pulse's report format verbatim — the hypothesis entry
//! (`crates/interpretation/src/markdown.rs:153-171`), the degraded notice (`:109`) and the empty
//! placeholder (`:158`) — transcribed at andromeda-pulse HEAD `83d4060`.
//!
//! PINNED CAPTURE — the one graded drive, 2026-09-23, against andromeda-pulse HEAD `83d4060` under the
//! real-model posture: deterministic L4 absent (`interpretation.model.load` read `real`), a fresh data
//! dir, the default bootstrap window (no override line), the workspace basename clear of
//! `conductor`, and the model on its PRIMARY tier (Llama-3.2-3B-Instruct-Q4_K_M, GPU). Tier: `<90s`,
//! the honest bucket, graded here and never on it. Pickup figure: NONE — the scenario never emitted.
//!
//! The rule graded it `NoAttributableIncident` → `(None, Blocked)`, route `PreflightBlocked`, and it
//! is a MODEL-SIDE Blocked (plan B2): the leg window held exactly one cue-bearing digest — the preflight
//! canary's tier-1 `retry_storm` at the Autonomous band (Pulse's `digest.runtime.cadence_tick` read
//! `cue_present: true`) — the real model answered it (`interpretation.inference.request` `success`,
//! 4379 ms, parse `ok`) and did not surface it: no `interpretation.incident.created` followed, and the
//! creation predicate's remaining exits (`Dismiss`, severity `None`, a resolution-summary flag) are the
//! model's own output fields. Every later digest was a cue-less tier-3 baseline; no inference error, no
//! skip, and Pulse formed no incident at all, so the sidecar's empty corpus was the truth rather than a
//! workspace-key divergence. Recorded, never re-driven (plan D1).

mod real_model_common;

use std::collections::BTreeSet;
use std::path::Path;

use conductor_core::{ENVELOPE_KEYS_SORTED, ReportState, RunRecord, Verdict};
use real_model_common::{mask_host_paths, rule_section};

// ---- rule: begin ----
// The grading rule for the real-model interpretation leg, fixed before the drive
// (contracts/pulse-real-model-leg-posture.md, The grading rule; plan 2026-09-22 steps 11 and D1-D3).

/// The capture's line grammar — what `real_model_live.rs` prints and this rule reads.
const EMISSION_INSTANT: &str = "emission_instant_ms: ";
const NO_EMISSION: &str = "emission: none";
const TRACE: &str = "trace: ";
const ATTRIBUTED: &str = "attributed: ";
const AMBIGUOUS: &str = "attribution: ambiguous";
const READ_BACK_FAILED: &str = "read-back failed: ";
const END_OF_SECTIONS: &str = "-- end of report sections --";
const INFERENCE_MODE: &str = "pulse-log inference_mode: ";
const LAUNCH_CWD: &str = "pulse-log workspace basename carries conductor: ";

/// Pulse's hypothesis entry opener, and the close of its confidence label: the statement is the text
/// after the FIRST close, so a close inside the statement stays part of it.
const ENTRY_OPEN: &str = "- **(";
const LABEL_CLOSE: &str = ")** ";

/// What the rule decides for one capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grade {
    /// The rank-1 statement names `conductor` as a whole word and a retry token.
    Identified,
    /// A ranked hypothesis exists and rank 1 does not name both facets.
    NotIdentified,
    /// The attributed report ranks nothing: degraded, or parsed but empty.
    NoRankedHypotheses,
    /// No single incident is attributable to the scenario's emission.
    NoAttributableIncident,
    /// The read-back call itself failed.
    ReadBackFailed,
}

/// The posture's outcome table, as `(verdict, state)`.
fn row(grade: Grade) -> (Option<Verdict>, ReportState) {
    match grade {
        Grade::Identified => (Some(Verdict::Pass), ReportState::Pass),
        Grade::NotIdentified => (Some(Verdict::CalibrationRegion), ReportState::ManualCheck),
        Grade::NoRankedHypotheses | Grade::NoAttributableIncident | Grade::ReadBackFailed => {
            (None, ReportState::Blocked)
        }
    }
}

/// Grade a capture. Precedence: a failed read-back, then attribution (exactly one attributed
/// incident, opened at or after the emission instant), then the attributed report's rank 1.
fn grade(capture: &str) -> Grade {
    let lines: Vec<&str> = capture.lines().collect();
    if lines.iter().any(|l| l.starts_with(READ_BACK_FAILED)) {
        return Grade::ReadBackFailed;
    }
    let attributed: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|l| l.starts_with(ATTRIBUTED))
        .collect();
    let [line] = attributed.as_slice() else {
        return Grade::NoAttributableIncident;
    };
    if lines.iter().any(|l| l.starts_with(AMBIGUOUS)) {
        return Grade::NoAttributableIncident;
    }
    let emitted_ns = lines
        .iter()
        .find_map(|l| l.strip_prefix(EMISSION_INSTANT))
        .and_then(|v| v.trim().parse::<i64>().ok())
        .and_then(|ms| ms.checked_mul(1_000_000));
    let opened_ns = field(line, "opened_at_unix_nano");
    match (emitted_ns, opened_ns) {
        (Some(emitted), Some(opened)) if opened >= emitted => {}
        _ => return Grade::NoAttributableIncident,
    }
    match section_body(capture, "## Hypotheses").and_then(rank1_statement) {
        None => Grade::NoRankedHypotheses,
        Some(statement) if identifies_cause(statement) => Grade::Identified,
        Some(_) => Grade::NotIdentified,
    }
}

/// The value of a `key=value` token on a capture line.
fn field(line: &str, key: &str) -> Option<i64> {
    line.split_whitespace()
        .find_map(|token| token.strip_prefix(key)?.strip_prefix('='))
        .and_then(|v| v.parse().ok())
}

/// The body of a report section as the capture printed it: after the `header` line, up to the next
/// `## ` line or the end-of-sections marker.
fn section_body<'a>(capture: &'a str, header: &str) -> Option<&'a str> {
    let open = format!("\n{header}\n");
    let start = capture.find(&open)? + open.len();
    let rest = &capture[start..];
    let end = ["\n## ".to_string(), format!("\n{END_OF_SECTIONS}")]
        .iter()
        .filter_map(|terminator| rest.find(terminator.as_str()))
        .min()
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

/// The rank-1 hypothesis statement: the first entry's text after its label close, up to its
/// justification sub-bullet, the next entry or the section's blank-line close. A statement may carry a
/// newline (Pulse bounds only its length), so it is never cut at its first line end.
fn rank1_statement(hypotheses: &str) -> Option<&str> {
    let entry = &hypotheses[hypotheses.find(ENTRY_OPEN)? + ENTRY_OPEN.len()..];
    let statement = &entry[entry.find(LABEL_CLOSE)? + LABEL_CLOSE.len()..];
    let end = ["\n  - ", "\n- **(", "\n\n"]
        .iter()
        .filter_map(|terminator| statement.find(terminator))
        .min()
        .unwrap_or(statement.len());
    Some(statement[..end].trim_end())
}

/// Identified iff the statement, lowercased (ASCII), names `conductor` as a whole word AND a retry
/// token. Stated limits, fixed here: negation is not read; only the hyphenated canary identity is
/// separated from `conductor` (a space-separated `conductor canary` passes); `retried` is not a retry
/// token; a hyphen or underscore neighbour rejects `conductor` wherever it stands.
fn identifies_cause(statement: &str) -> bool {
    let lower = statement.to_ascii_lowercase();
    names_conductor(&lower) && names_retry(&lower)
}

/// `conductor` with neither neighbour in `[a-z0-9_-]`.
fn names_conductor(lower: &str) -> bool {
    let bytes = lower.as_bytes();
    let joins = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-';
    lower.match_indices("conductor").any(|(at, word)| {
        let before = at.checked_sub(1).map(|i| bytes[i]);
        let after = bytes.get(at + word.len()).copied();
        !before.is_some_and(joins) && !after.is_some_and(joins)
    })
}

/// A maximal run of ASCII letters equal to `retry`, `retries` or `retrying` — the tokens v3-09's
/// acceptance names. `retry_storm` splits into `retry` and `storm`; `non-retryable` yields none.
fn names_retry(lower: &str) -> bool {
    lower
        .split(|c: char| !c.is_ascii_alphabetic())
        .any(|run| matches!(run, "retry" | "retries" | "retrying"))
}

/// B1: the capture witnesses the REAL model — Pulse's `interpretation.model.load` read `real` over
/// the whole log, and the attributed report's evidence carries no deterministic-fixture (`det-`) ref.
/// A capture failing this is an operator-launch fault, never a graded outcome.
fn real_model_witnessed(capture: &str) -> bool {
    let mode_real = capture
        .lines()
        .any(|l| l.strip_prefix(INFERENCE_MODE) == Some("real"));
    let canned_evidence = section_body(capture, "## Evidence").is_some_and(|e| e.contains("`det-"));
    mode_real && !canned_evidence
}

/// Plan D-8: `pulse-app`'s cwd basename reaches the prompt as `PROJECT:`, so a launch whose basename
/// carries `conductor` puts the word in front of the model outside the cue. Such a capture is an
/// operator-launch fault, never graded.
fn launch_cwd_clear(capture: &str) -> bool {
    capture
        .lines()
        .any(|l| l.strip_prefix(LAUNCH_CWD) == Some("false"))
}

/// Which way the leg went, read from the capture's own lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Route {
    /// The scenario never emitted: the gate (or the posture check) blocked it.
    PreflightBlocked,
    /// It emitted, and no `retrieve_report` read-back witness followed.
    EmittedNoReadBack,
    /// Everything else: the full chain.
    ReadBack,
}

fn route(capture: &str) -> Route {
    if capture.lines().any(|l| l == NO_EMISSION) {
        return Route::PreflightBlocked;
    }
    match trace_field(capture, "retrieve_report_witness") {
        Some("true") => Route::ReadBack,
        _ => Route::EmittedNoReadBack,
    }
}

/// A `key=value` token of the capture's `trace:` line, as text.
fn trace_field<'a>(capture: &'a str, key: &str) -> Option<&'a str> {
    capture
        .lines()
        .find_map(|l| l.strip_prefix(TRACE))?
        .split_whitespace()
        .find_map(|token| token.strip_prefix(key)?.strip_prefix('='))
}

/// Whether the `trace:` line carries its route's witness set, so no outcome forces a test edit.
fn trace_conforms(capture: &str) -> bool {
    let spans: BTreeSet<&str> = trace_field(capture, "spans")
        .map(|s| s.split(',').filter(|n| !n.is_empty()).collect())
        .unwrap_or_default();
    let wire: u64 = trace_field(capture, "wire_shape_lines")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let retrieve_report = trace_field(capture, "retrieve_report_witness") == Some("true");
    let chain = ["scenario.run", "timeline.execute", "emit.batch"]
        .iter()
        .all(|s| spans.contains(s));
    match route(capture) {
        Route::PreflightBlocked => !spans.contains("timeline.execute"),
        Route::EmittedNoReadBack => chain && wire >= 1 && !retrieve_report,
        Route::ReadBack => {
            chain
                && spans.iter().any(|s| s.starts_with("verify.readback"))
                && wire >= 1
                && retrieve_report
        }
    }
}
// ---- rule: end ----

// ---- fixtures -------------------------------------------------------------------------------------

/// Pulse's degraded notice, verbatim (`markdown.rs:109`). It carries a retry token itself.
const DEGRADED_NOTICE: &str = "_Interpretation pending — hypotheses and investigation steps will populate as soon as the next L4 inference cycle completes for this incident. See diagnostics for retry options._";

/// Pulse's parsed-but-empty placeholder, verbatim (`markdown.rs:158`).
const NO_HYPOTHESES: &str = "_No ranked hypotheses produced._";

/// The emission instant every synthetic capture shares.
const EMITTED_MS: i64 = 1_790_150_400_000;

/// One rendered hypothesis entry, in `serialize_report`'s shape (`markdown.rs:160-171`).
fn entry(confidence: &str, statement: &str, justification: &str) -> String {
    let mut out = format!("{ENTRY_OPEN}{confidence}{LABEL_CLOSE}{statement}");
    if !justification.is_empty() {
        out.push_str("\n  - ");
        out.push_str(justification);
    }
    out.push('\n');
    out
}

/// A `## Hypotheses` body as Pulse renders it: a blank line under the header, the entries, and a
/// blank line closing the section.
fn ranked(entries: &[String]) -> String {
    format!("\n{}\n", entries.concat())
}

/// A synthetic capture with one attributed incident opened at `opened_ms`, whose report's
/// `## Hypotheses` body is `hypotheses`.
fn capture_with(opened_ms: i64, hypotheses: &str) -> String {
    format!(
        "run_id: 2026-09-23T10-00-00-000\n\
         {EMISSION_INSTANT}{EMITTED_MS}\n\
         {TRACE}spans=emit.batch,scenario.run,timeline.execute,verify.readback.observe \
         wire_shape_lines=36 retrieve_report_witness=true\n\
         {ATTRIBUTED}incident_id=2 opened_at_unix_nano={opened_ns} degraded_mode=false pickup_ms={pickup}\n\
         ## Hypotheses\n{hypotheses}## Evidence\n\n- `ref-1`\n\n{END_OF_SECTIONS}\n\
         {INFERENCE_MODE}real\n\
         {LAUNCH_CWD}false\n",
        opened_ns = opened_ms * 1_000_000,
        pickup = opened_ms - EMITTED_MS,
    )
}

/// A capture whose attributed report ranks `statement` first, with no justification.
fn rank1(statement: &str) -> String {
    capture_with(
        EMITTED_MS + 31_000,
        &ranked(&[entry("high", statement, "")]),
    )
}

// ---- the rule's arms ------------------------------------------------------------------------------

#[test]
fn a01_rank1_naming_both_facets_is_identified_and_passes() {
    let capture =
        rank1("The conductor service is caught in a retry storm of one repeated exception.");
    assert_eq!(grade(&capture), Grade::Identified);
    assert_eq!(
        row(grade(&capture)),
        (Some(Verdict::Pass), ReportState::Pass)
    );
}

#[test]
fn a02_both_facets_only_at_rank_2_is_a_miss_never_a_pass() {
    let capture = capture_with(
        EMITTED_MS + 31_000,
        &ranked(&[
            entry("high", "Latency on an unrelated dependency.", ""),
            entry("medium", "The conductor service is in a retry storm.", ""),
        ]),
    );
    assert_eq!(grade(&capture), Grade::NotIdentified);
    assert_eq!(
        row(grade(&capture)),
        (Some(Verdict::CalibrationRegion), ReportState::ManualCheck)
    );
}

#[test]
fn a03_the_degraded_notice_is_blocked_though_it_carries_a_retry_token() {
    assert!(
        names_retry(&DEGRADED_NOTICE.to_ascii_lowercase()),
        "the notice itself carries `retry`, so this arm is not vacuous"
    );
    let capture = capture_with(EMITTED_MS + 31_000, &format!("\n{DEGRADED_NOTICE}\n\n"));
    assert_eq!(grade(&capture), Grade::NoRankedHypotheses);
    assert_eq!(row(grade(&capture)), (None, ReportState::Blocked));
}

#[test]
fn a04_the_empty_placeholder_is_blocked() {
    let capture = capture_with(EMITTED_MS + 31_000, &format!("\n{NO_HYPOTHESES}\n\n"));
    assert_eq!(grade(&capture), Grade::NoRankedHypotheses);
    assert_eq!(row(grade(&capture)), (None, ReportState::Blocked));
}

#[test]
fn a05_the_canary_identity_with_retry_is_not_identified() {
    assert_eq!(
        grade(&rank1("The conductor-canary service is in a retry storm.")),
        Grade::NotIdentified
    );
}

#[test]
fn a06_conductor_without_a_retry_token_is_not_identified() {
    assert_eq!(
        grade(&rank1(
            "The conductor service is raising repeated exceptions."
        )),
        Grade::NotIdentified
    );
}

#[test]
fn a07_retry_without_conductor_is_not_identified() {
    assert_eq!(
        grade(&rank1("A downstream service is in a retry storm.")),
        Grade::NotIdentified
    );
}

#[test]
fn a08_both_facets_only_in_the_justification_is_not_identified() {
    let capture = capture_with(
        EMITTED_MS + 31_000,
        &ranked(&[entry(
            "high",
            "An exception burst is under way.",
            "the conductor service shows a retry storm",
        )]),
    );
    assert_eq!(grade(&capture), Grade::NotIdentified);
}

#[test]
fn a09_case_is_not_read() {
    assert_eq!(
        grade(&rank1("Conductor is stuck in a RETRY loop.")),
        Grade::Identified
    );
}

#[test]
fn a10_an_incident_opened_before_the_emission_instant_is_never_attributed() {
    // The canary mis-pairing: the canary's incident opens before the scenario emits. Alone, it grades
    // Blocked however well its report reads.
    let capture = capture_with(
        EMITTED_MS - 5_000,
        &ranked(&[entry(
            "high",
            "The conductor service is in a retry storm.",
            "",
        )]),
    );
    assert_eq!(grade(&capture), Grade::NoAttributableIncident);
    assert_eq!(row(grade(&capture)), (None, ReportState::Blocked));
}

#[test]
fn a11_two_attributable_incidents_are_blocked() {
    let single = rank1("The conductor service is in a retry storm.");
    let doubled = single.replace(
        &format!("{ATTRIBUTED}incident_id=2"),
        &format!(
            "{ATTRIBUTED}incident_id=3 opened_at_unix_nano={} degraded_mode=false pickup_ms=40000\n{AMBIGUOUS}\n{ATTRIBUTED}incident_id=2",
            (EMITTED_MS + 40_000) * 1_000_000
        ),
    );
    assert_eq!(grade(&doubled), Grade::NoAttributableIncident);
    assert_eq!(row(grade(&doubled)), (None, ReportState::Blocked));
}

#[test]
fn a12_a_read_back_failure_is_blocked() {
    let capture = format!(
        "{EMISSION_INSTANT}{EMITTED_MS}\n{READ_BACK_FAILED}MCP read-back returned a JSON-RPC error (code -32603)\n"
    );
    assert_eq!(grade(&capture), Grade::ReadBackFailed);
    assert_eq!(row(grade(&capture)), (None, ReportState::Blocked));
}

#[test]
fn a13_a_second_label_close_inside_the_statement_stays_in_it() {
    let statement = "The conductor service)** is in a retry storm.";
    let capture = rank1(statement);
    assert_eq!(
        section_body(&capture, "## Hypotheses").and_then(rank1_statement),
        Some(statement)
    );
    assert_eq!(grade(&capture), Grade::Identified);
}

#[test]
fn a14_a_statement_carrying_a_newline_is_extracted_whole() {
    let statement = "The conductor service is failing\nbehind a retry storm.";
    let capture = rank1(statement);
    assert_eq!(
        section_body(&capture, "## Hypotheses").and_then(rank1_statement),
        Some(statement),
        "graded over both lines, never cut at the first line end"
    );
    assert_eq!(grade(&capture), Grade::Identified);
}

#[test]
fn a15_stated_limit_a_space_separated_canary_identity_passes() {
    assert_eq!(
        grade(&rank1("The conductor canary is in a retry storm.")),
        Grade::Identified
    );
}

#[test]
fn a16_stated_limit_negation_is_not_read() {
    assert_eq!(
        grade(&rank1("This is not a retry storm on conductor.")),
        Grade::Identified
    );
}

#[test]
fn a17_stated_limit_no_retry_still_names_the_token() {
    assert_eq!(
        grade(&rank1("There is no retry pattern on conductor.")),
        Grade::Identified
    );
}

#[test]
fn a18_non_retryable_carries_no_retry_token() {
    assert_eq!(
        grade(&rank1(
            "The conductor service raised a non-retryable error."
        )),
        Grade::NotIdentified
    );
}

#[test]
fn a19_stated_limit_retried_is_not_a_retry_token() {
    assert_eq!(
        grade(&rank1("The conductor service retried the same call.")),
        Grade::NotIdentified
    );
}

#[test]
fn a20_stated_limit_a_hyphen_neighbour_rejects_conductor() {
    assert_eq!(
        grade(&rank1("A conductor-side retry storm.")),
        Grade::NotIdentified
    );
}

#[test]
fn a21_stated_limit_an_underscore_neighbour_rejects_conductor() {
    assert_eq!(
        grade(&rank1("The _conductor_ worker is in a retry storm.")),
        Grade::NotIdentified
    );
}

#[test]
fn the_cue_line_itself_satisfies_the_rule() {
    // D2's stated limit, as an arm: the line the model reads verbatim already names both facets.
    assert!(identifies_cause(
        "[autonomous] retry_storm — retry_storm scope_id=conductor"
    ));
}

#[test]
fn every_row_of_the_posture_table_is_reachable() {
    let all = [
        Grade::Identified,
        Grade::NotIdentified,
        Grade::NoRankedHypotheses,
        Grade::NoAttributableIncident,
        Grade::ReadBackFailed,
    ];
    for g in all {
        // Exhaustive: a new grade does not compile until it is placed in `all` and given a row.
        match g {
            Grade::Identified
            | Grade::NotIdentified
            | Grade::NoRankedHypotheses
            | Grade::NoAttributableIncident
            | Grade::ReadBackFailed => {}
        }
    }
    let reached: BTreeSet<String> = [
        rank1("The conductor service is in a retry storm."),
        rank1("A downstream service is in a retry storm."),
        capture_with(EMITTED_MS + 31_000, &format!("\n{NO_HYPOTHESES}\n\n")),
        format!("{READ_BACK_FAILED}x\n"),
        format!("{EMISSION_INSTANT}{EMITTED_MS}\nattribution: none within 600s\n"),
    ]
    .iter()
    .map(|capture| format!("{:?}", row(grade(capture))))
    .collect();
    let table: BTreeSet<String> = all.iter().map(|g| format!("{:?}", row(*g))).collect();
    assert_eq!(table.len(), 3, "the posture's three rows");
    assert_eq!(reached, table, "every row is reachable from a capture");
}

// ---- B1 and the launch witness --------------------------------------------------------------------

#[test]
fn a_canned_capture_is_never_the_real_model() {
    let real = rank1("The conductor service is in a retry storm.");
    assert!(real_model_witnessed(&real));

    let canned_mode = real.replace(
        &format!("{INFERENCE_MODE}real"),
        &format!("{INFERENCE_MODE}deterministic"),
    );
    assert!(!real_model_witnessed(&canned_mode));

    // The deterministic fixture's evidence refs (andromeda-pulse deterministic_inference.rs:68-72).
    let canned_evidence = real.replace("- `ref-1`", "- `det-span-9f2c4a7e1b6d0358`");
    assert!(!real_model_witnessed(&canned_evidence));
}

#[test]
fn a_launch_whose_basename_carries_conductor_is_not_clear() {
    let clear = rank1("The conductor service is in a retry storm.");
    assert!(launch_cwd_clear(&clear));
    let fouled = clear.replace(&format!("{LAUNCH_CWD}false"), &format!("{LAUNCH_CWD}true"));
    assert!(!launch_cwd_clear(&fouled));
}

// ---- routes ---------------------------------------------------------------------------------------

#[test]
fn each_route_carries_its_own_trace_witness_set() {
    let read_back = rank1("The conductor service is in a retry storm.");
    assert_eq!(route(&read_back), Route::ReadBack);
    assert!(trace_conforms(&read_back));

    let no_read_back = read_back.replace(
        "retrieve_report_witness=true",
        "retrieve_report_witness=false",
    );
    assert_eq!(route(&no_read_back), Route::EmittedNoReadBack);
    assert!(trace_conforms(&no_read_back));

    let blocked = format!(
        "{NO_EMISSION}\n{TRACE}spans=scenario.run,verify.readback.preflight wire_shape_lines=12 \
         retrieve_report_witness=false\n"
    );
    assert_eq!(route(&blocked), Route::PreflightBlocked);
    assert!(trace_conforms(&blocked));
}

#[test]
fn a_read_back_route_missing_its_emission_span_does_not_conform() {
    let broken =
        rank1("The conductor service is in a retry storm.").replace("spans=emit.batch,", "spans=");
    assert_eq!(route(&broken), Route::ReadBack);
    assert!(!trace_conforms(&broken));
}

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

// ---- producer/grader agreement --------------------------------------------------------------------

#[test]
fn the_capture_prints_every_token_the_rule_reads() {
    // The capture is feature-gated, so this default-suite target cannot call it; it can read its
    // source. Each grammar literal the rule keys on must appear there verbatim.
    let capture = include_str!("real_model_live.rs");
    for token in [
        EMISSION_INSTANT,
        NO_EMISSION,
        TRACE,
        ATTRIBUTED,
        AMBIGUOUS,
        READ_BACK_FAILED,
        END_OF_SECTIONS,
        INFERENCE_MODE,
        LAUNCH_CWD,
    ] {
        assert!(
            capture.contains(&format!("\"{token}\"")),
            "the capture prints {token:?}"
        );
    }
    for key in [
        "opened_at_unix_nano=",
        "spans=",
        "wire_shape_lines=",
        "retrieve_report_witness=",
    ] {
        assert!(capture.contains(key), "the capture prints the {key} field");
    }
}

#[test]
fn the_capture_computes_the_dispatcher_s_own_exception() {
    // Attribution keys on the scenario's cue fingerprint, which the capture computes from a
    // transcription of the dispatcher's private base exception. Drift in either copy would attribute
    // nothing; both must carry the same four literals.
    let capture = include_str!("real_model_live.rs");
    let dispatch = include_str!("../src/dispatch.rs");
    for literal in [
        r#""ValueError","#,
        r#""conductor synthetic exception","#,
        r#"Frame::new("conductor::worker::handle", "src/worker.rs", 42)"#,
        r#"Frame::new("conductor::worker::parse", "src/worker.rs", 17)"#,
    ] {
        assert!(dispatch.contains(literal), "dispatch.rs carries {literal}");
        assert!(capture.contains(literal), "the capture carries {literal}");
    }
}

#[test]
fn the_rule_section_is_extractable_and_bounded_by_its_markers() {
    let section = rule_section(include_str!("real_model_harvest.rs")).expect("markers present");
    assert!(section.starts_with("// ---- rule: begin ----"));
    assert!(section.ends_with("// ---- rule: end ----"));
    assert!(section.contains("fn identifies_cause"));
    assert!(
        !section.contains("fn a01_"),
        "the arms sit outside the rule"
    );
}

// ---- the pinned capture (step 14) ------------------------------------------------------------------

/// The one drive's capture block, verbatim from `evidence/rm-capture.txt` — everything the capture
/// test printed after the rule record, from its first line to its last.
const PINNED_CAPTURE: &str = r##"real-model capture
run_id: 2026-09-23T07-39-39-845
preflight: preflight blocked: pulse-app and the spawned MCP sidecar must resolve the same incident workspace key — the sidecar keys on ANDROMEDA_PULSE_DATA_DIR, pulse-app on its detected workspace root — or Pulse raised no incident for the canary
preflight: preflight blocked: readiness gate not satisfied
emission: none
trace: spans=db.insert_run,emit.batch,report.generate,scenario.run,verify.readback.call_tool,verify.readback.connect,verify.readback.connect_command,verify.readback.list_tools,verify.readback.preflight wire_shape_lines=15 retrieve_report_witness=false
envelope: {"fingerprints":null,"journal_emitted_at":null,"latency_ms":null,"p_ids":["P-018"],"read_back_observed_at":null,"run_id":"2026-09-23T07-39-39-845","scenario":"real-model-interpretation","seed":4317033,"slo_tier":"<90s","state":"Blocked","verdict":null}
envelope fingerprints: 0, det- prefixed: 0
attribution: none (the scenario never emitted)
pulse-log file: agent-latest.jsonl.2026-09-23
pulse-log inference_mode: real
pulse-log workspace basename carries conductor: false
pulse-log bootstrap_window.override (whole file): 0
pulse-log window: 189975 lines since the leg's first self-obs line
pulse-log interpretation.prompt.assemble: 4
pulse-log   interpretation.prompt.assemble t=2026-09-23T07:39:41.910Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-23T07:40:25.028Z prompt_version=v2.2 token_count=6312 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-23T07:40:41.917Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log   interpretation.prompt.assemble t=2026-09-23T07:41:41.899Z prompt_version=v2.2 token_count=6322 duration_ms=0
pulse-log interpretation.json.parse: 4
pulse-log   interpretation.json.parse t=2026-09-23T07:39:45.586Z parse_outcome=ok output_bytes=395 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-23T07:40:29.407Z parse_outcome=ok output_bytes=1192 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-23T07:40:45.233Z parse_outcome=ok output_bytes=393 duration_ms=0
pulse-log   interpretation.json.parse t=2026-09-23T07:41:45.784Z parse_outcome=ok output_bytes=1098 duration_ms=0
pulse-log interpretation.incident.created: 0
pulse-log interpretation.inference.error: 0
pulse-log interpretation.inference.skipped: 0
pulse-log triage.pattern.storm.detected: 2
pulse-log   triage.pattern.storm.detected t=2026-09-23T07:40:24.991Z severity_hint=suggested occurrence_count=5 fingerprint_hex=8cb9c5d5
pulse-log   triage.pattern.storm.detected t=2026-09-23T07:40:24.999Z severity_hint=autonomous occurrence_count=10 fingerprint_hex=8cb9c5d5
pulse-log heartbeat ingest.tick (15 s): 44
creating digest prompt_version: unknown
"##;

/// What the rule measured on the one drive.
const MEASURED: Grade = Grade::NoAttributableIncident;

/// The committed capture, read workspace-root anchored (a test binary's cwd is its own crate).
fn committed_capture() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt",
    );
    std::fs::read_to_string(&path)
        .expect("the committed capture is readable")
        .replace("\r\n", "\n")
}

#[test]
fn the_pinned_capture_grades_as_the_drive_measured() {
    // The rule graded it; nobody re-judged it.
    assert_eq!(grade(PINNED_CAPTURE), MEASURED);
    assert_eq!(row(grade(PINNED_CAPTURE)), (None, ReportState::Blocked));
}

#[test]
fn the_pinned_capture_witnesses_the_real_model_and_a_clear_launch() {
    // B1 and the launch witness: a canned or fouled launch is never graded. This route read no
    // report, so the evidence half of B1 is vacuous here; the operative witnesses are the whole-log
    // model mode and the envelope's zero `det-` refs.
    assert!(real_model_witnessed(PINNED_CAPTURE));
    assert!(launch_cwd_clear(PINNED_CAPTURE));
    assert!(
        section_body(PINNED_CAPTURE, "## Evidence").is_none(),
        "no report was read on this route"
    );
    assert!(
        PINNED_CAPTURE
            .lines()
            .any(|l| l == "envelope fingerprints: 0, det- prefixed: 0")
    );
}

#[test]
fn the_pinned_envelope_carries_the_eleven_keys_and_the_closed_sets() {
    let envelope = PINNED_CAPTURE
        .lines()
        .find_map(|l| l.strip_prefix("envelope: "))
        .filter(|rest| rest.starts_with('{'));
    let Some(line) = envelope else {
        assert_eq!(
            route(PINNED_CAPTURE),
            Route::PreflightBlocked,
            "only a preflight-blocked capture may carry no envelope"
        );
        return;
    };
    // A typed parse cannot prove a nullable key PRESENT, so the key set is checked on the value.
    let value: serde_json::Value = serde_json::from_str(line).expect("the pinned envelope parses");
    let mut keys: Vec<&str> = value
        .as_object()
        .expect("the envelope is an object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(keys, ENVELOPE_KEYS_SORTED);
    let record: RunRecord = serde_json::from_str(line).expect("the closed verdict/state/tier sets");
    assert_eq!(record.scenario, "real-model-interpretation");
    assert_eq!(record.state, ReportState::Blocked);
    assert_eq!(record.verdict, None);
}

#[test]
fn the_pinned_trace_carries_its_route_s_witness_set() {
    assert_eq!(route(PINNED_CAPTURE), Route::PreflightBlocked);
    assert!(trace_conforms(PINNED_CAPTURE));
}

#[test]
fn rule_predates_the_drive() {
    // The capture's pre-leg rule record against this file's rule: equal, or the rule moved after the
    // model answered.
    let recorded =
        rule_section(&committed_capture()).expect("the capture opens with the rule record");
    let current =
        rule_section(include_str!("real_model_harvest.rs")).expect("this file carries its rule");
    assert_eq!(recorded, current, "the rule was edited after the drive");
}

#[test]
fn pinned_literals_equal_the_committed_capture() {
    let committed = committed_capture();
    let start = committed
        .find("\nreal-model capture\n")
        .expect("the capture block opens")
        + 1;
    let end = committed[start..]
        .find("\n.\n")
        .map(|at| start + at + 1)
        .expect("libtest's mark closes the capture block");
    assert_eq!(&committed[start..end], PINNED_CAPTURE);
}
