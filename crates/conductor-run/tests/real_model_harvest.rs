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
//! workspace-key divergence. Recorded, and never replaced (plan D1).
//!
//! EXTENDED 2026-09-29, before the drive series (`contracts/pulse-real-model-leg-posture.md`, The drive
//! series): the rule gains the P-031 / P-034 / P-044 grades and the canary's attempts, appended below
//! the 2026-09-23 rule so that rule stays a byte-exact prefix (`rule_predates_the_drive`).

mod real_model_common;

use std::collections::BTreeSet;
use std::path::Path;

use conductor_core::{ENVELOPE_KEYS_SORTED, ReportState, RunRecord, Verdict};
mod real_model_series;

use real_model_common::{
    DIGEST_ASSEMBLE, DIGEST_TICK, SWEEP_BOUND, canary_attempts as pair_canary_attempts,
    elide_fingerprints, mask_host_paths, rule_section, sweep_window,
};
use real_model_series::{EVIDENCE, SERIES};

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

// Extended 2026-09-29, before the drive series (contracts/pulse-real-model-leg-posture.md, The drive
// series (f)): three further grades over the same capture, and the canary's attempts. Everything above
// this comment is the 2026-09-23 rule, byte-identical; the rank-1 rule is unchanged.

const CORPUS_ROWS: &str = "creating digest corpus retrieval rows: ";
const CANARY: &str = "canary: ";
const SURFACED: &str = "surfaced";
const DISMISSED: &str = "dismissed";
const PIPELINE_FAULT: &str = "pipeline-fault";

/// The six P-031 sections, in the order Pulse's serializer renders them.
const P031_SECTIONS: [&str; 6] = [
    "## Symptom",
    "## Timeline",
    "## Hypotheses",
    "## Investigation Steps",
    "## Evidence",
    "## Project Context",
];
/// Pulse's empty placeholders for the two narrative sections.
const NO_SYMPTOM: &str = "_No symptom narrative available._";
const NO_TIMELINE: &str = "_No timeline narrative available._";
const NO_PRIOR: &str = "no prior same-scope incident to retrieve";

/// What one of the further grades decides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Pass,
    ManualCheck,
    /// A deterministic SUT fault: the serializer's section structure broke.
    Fail,
    /// Nothing to grade, with the named reason.
    Blocked(&'static str),
}

/// The posture's outcome table for the further grades, as `(verdict, state)`.
fn outcome_row(outcome: Outcome) -> (Option<Verdict>, ReportState) {
    match outcome {
        Outcome::Pass => (Some(Verdict::Pass), ReportState::Pass),
        Outcome::ManualCheck => (Some(Verdict::CalibrationRegion), ReportState::ManualCheck),
        Outcome::Fail => (Some(Verdict::Fail), ReportState::Fail),
        Outcome::Blocked(_) => (None, ReportState::Blocked),
    }
}

/// The shared precondition: one attributed report, read back — the rank-1 rule's own attribution.
fn attributed_report(capture: &str) -> Result<(), &'static str> {
    match grade(capture) {
        Grade::ReadBackFailed => Err("read-back failed"),
        Grade::NoAttributableIncident => Err("no attributable incident"),
        Grade::Identified | Grade::NotIdentified | Grade::NoRankedHypotheses => Ok(()),
    }
}

/// The attributed line's `degraded_mode` token, as Pulse's `retrieve_report` returned it, reads `true`.
fn degraded(capture: &str) -> bool {
    capture
        .lines()
        .filter_map(|l| l.strip_prefix(ATTRIBUTED))
        .any(|l| l.split_whitespace().any(|t| t == "degraded_mode=true"))
}

/// P-031 Report Structure: the six sections in order, and Symptom and Timeline each carrying a
/// narrative rather than its placeholder.
fn structure(capture: &str) -> Outcome {
    if let Err(reason) = attributed_report(capture) {
        return Outcome::Blocked(reason);
    }
    if degraded(capture) {
        return Outcome::Blocked("report degraded");
    }
    let positions: Option<Vec<usize>> = P031_SECTIONS
        .iter()
        .map(|header| capture.find(&format!("\n{header}\n")))
        .collect();
    if !positions.is_some_and(|p| p.windows(2).all(|w| w[0] < w[1])) {
        return Outcome::Fail;
    }
    let narrative = |header: &str, placeholder: &str| {
        section_body(capture, header)
            .is_some_and(|b| !b.trim().is_empty() && b.trim() != placeholder)
    };
    if narrative("## Symptom", NO_SYMPTOM) && narrative("## Timeline", NO_TIMELINE) {
        Outcome::Pass
    } else {
        Outcome::ManualCheck
    }
}

/// P-034 Suggested Investigation Steps: at least one numbered entry, as Pulse renders `1. {step}`.
fn steps(capture: &str) -> Outcome {
    if let Err(reason) = attributed_report(capture) {
        return Outcome::Blocked(reason);
    }
    if degraded(capture) {
        return Outcome::Blocked("report degraded");
    }
    let Some(body) = section_body(capture, "## Investigation Steps") else {
        return Outcome::Blocked("no Investigation Steps section");
    };
    if body.lines().any(numbered_entry) {
        Outcome::Pass
    } else {
        Outcome::ManualCheck
    }
}

fn numbered_entry(line: &str) -> bool {
    let digits = line.bytes().take_while(u8::is_ascii_digit).count();
    digits > 0 && line[digits..].starts_with(". ")
}

/// P-044 Retrieval-Augmented Interpretation, by inference chain: `## Previously Seen` lists an
/// incident opened BEFORE the attributed one, AND the creating digest's corpus retrieval returned at
/// least one row. Stated limit: this reaches the model's INPUT, never whether the model used it.
fn retrieval(capture: &str) -> Outcome {
    if let Err(reason) = attributed_report(capture) {
        return Outcome::Blocked(reason);
    }
    let opened = capture
        .lines()
        .find_map(|l| l.strip_prefix(ATTRIBUTED))
        .and_then(|l| field(l, "opened_at_unix_nano"));
    let prior = section_body(capture, "## Previously Seen").is_some_and(|body| {
        body.lines()
            .filter_map(previously_seen_opened)
            .any(|seen| opened.is_some_and(|attributed| seen < attributed))
    });
    let rows = capture
        .lines()
        .find_map(|l| l.strip_prefix(CORPUS_ROWS))
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(0);
    if prior && rows >= 1 {
        Outcome::Pass
    } else {
        Outcome::Blocked(NO_PRIOR)
    }
}

/// The open stamp of a `- incident #{id} @ {opened_unix_nano} — {title} ({workspace})` entry.
fn previously_seen_opened(line: &str) -> Option<i64> {
    let (_, after) = line.strip_prefix("- incident #")?.split_once(" @ ")?;
    after.split_whitespace().next()?.parse().ok()
}

/// The canary's attempts, counted from its `canary:` lines by their first token.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct CanaryAttempts {
    surfaced: u32,
    dismissed: u32,
    pipeline_fault: u32,
}

fn canary_attempts(capture: &str) -> CanaryAttempts {
    let mut attempts = CanaryAttempts::default();
    for token in capture
        .lines()
        .filter_map(|l| l.strip_prefix(CANARY))
        .filter_map(|rest| rest.split_whitespace().next())
    {
        match token {
            SURFACED => attempts.surfaced += 1,
            DISMISSED => attempts.dismissed += 1,
            PIPELINE_FAULT => attempts.pipeline_fault += 1,
            _ => {}
        }
    }
    attempts
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

// ---- the further grades' arms (P-031, P-034, P-044, the canary) -----------------------------------
// Pulse's report sections and placeholders, transcribed at andromeda-pulse HEAD `f15536b` (the file is
// byte-identical at `e98d838`, the drives' HEAD)
// (`crates/interpretation/src/markdown.rs:137-234`).

const NO_STEPS: &str = "_No suggested investigation steps produced._";

/// Pulse's six sections as `serialize_report` renders them, from the given bodies.
fn six(symptom: &str, timeline: &str, hypotheses: &str, steps: &str) -> String {
    format!(
        "## Symptom\n\n{symptom}\n\n## Timeline\n\n{timeline}\n\n## Hypotheses\n{hypotheses}\
         ## Investigation Steps\n\n{steps}\n\n## Evidence\n\n- `ref-1`\n\n\
         ## Project Context\n\nworkspace=<host-path>\n\n"
    )
}

/// A non-degraded report whose model supplied every narrative.
fn full_report() -> String {
    six(
        "The conductor service raised one repeated exception.",
        "A burst began at the emission instant.",
        &ranked(&[entry(
            "high",
            "The conductor service is in a retry storm.",
            "",
        )]),
        "1. Inspect the conductor worker's retry loop.\n   - _Expected yield:_ the retried call",
    )
}

/// Pulse's `## Previously Seen` subsection listing one incident opened at `opened_ms`.
fn previously_seen(opened_ms: i64) -> String {
    format!(
        "## Previously Seen\n\n- incident #3 @ {} — conductor retry storm (<host-path>)\n\n",
        opened_ms * 1_000_000
    )
}

/// A synthetic capture whose attributed report renders `sections` (from its first `## ` header),
/// opened at `opened_ms`, with `degraded` as Pulse returned it and `rows` as the creating digest's
/// corpus retrieval rows.
fn report_capture(opened_ms: i64, degraded: bool, sections: &str, rows: &str) -> String {
    format!(
        "run_id: 2026-09-29T10-00-00-000\n\
         {EMISSION_INSTANT}{EMITTED_MS}\n\
         {TRACE}spans=emit.batch,scenario.run,timeline.execute,verify.readback.observe \
         wire_shape_lines=36 retrieve_report_witness=true\n\
         {ATTRIBUTED}incident_id=7 opened_at_unix_nano={opened_ns} degraded_mode={degraded} pickup_ms={pickup}\n\
         {sections}{END_OF_SECTIONS}\n\
         {INFERENCE_MODE}real\n\
         {LAUNCH_CWD}false\n\
         {CORPUS_ROWS}{rows}\n",
        opened_ns = opened_ms * 1_000_000,
        pickup = opened_ms - EMITTED_MS,
    )
}

const OPENED_MS: i64 = EMITTED_MS + 31_000;

#[test]
fn p031_all_six_in_order_with_both_narratives_passes() {
    let capture = report_capture(OPENED_MS, false, &full_report(), "0");
    assert_eq!(structure(&capture), Outcome::Pass);
    assert_eq!(
        outcome_row(structure(&capture)),
        (Some(Verdict::Pass), ReportState::Pass)
    );
    assert_eq!(
        grade(&capture),
        Grade::Identified,
        "the fixture also satisfies the rank-1 rule"
    );
}

#[test]
fn p031_a_narrative_placeholder_is_manual_check() {
    for sections in [
        six(
            NO_SYMPTOM,
            "A burst began.",
            &ranked(&[entry("high", "x", "")]),
            "1. y",
        ),
        six(
            "A burst.",
            NO_TIMELINE,
            &ranked(&[entry("high", "x", "")]),
            "1. y",
        ),
    ] {
        let capture = report_capture(OPENED_MS, false, &sections, "0");
        assert_eq!(structure(&capture), Outcome::ManualCheck);
        assert_eq!(
            outcome_row(structure(&capture)),
            (Some(Verdict::CalibrationRegion), ReportState::ManualCheck)
        );
    }
}

#[test]
fn p031_a_degraded_report_is_blocked() {
    let sections = six(
        "detail text",
        NO_TIMELINE,
        &format!("\n{DEGRADED_NOTICE}\n\n"),
        DEGRADED_NOTICE,
    );
    let capture = report_capture(OPENED_MS, true, &sections, "0");
    assert_eq!(structure(&capture), Outcome::Blocked("report degraded"));
    assert_eq!(
        outcome_row(structure(&capture)),
        (None, ReportState::Blocked)
    );
}

#[test]
fn p031_a_section_missing_or_out_of_order_fails() {
    let full = full_report();
    let swapped = full
        .replace("## Symptom\n", "## TEMP\n")
        .replace("## Timeline\n", "## Symptom\n")
        .replace("## TEMP\n", "## Timeline\n");
    let missing = full.replace("## Evidence\n\n- `ref-1`\n\n", "");
    for sections in [swapped, missing] {
        let capture = report_capture(OPENED_MS, false, &sections, "0");
        assert_eq!(structure(&capture), Outcome::Fail);
        assert_eq!(
            outcome_row(structure(&capture)),
            (Some(Verdict::Fail), ReportState::Fail)
        );
    }
}

#[test]
fn p031_no_attributable_incident_is_blocked() {
    let capture = report_capture(EMITTED_MS - 5_000, false, &full_report(), "0");
    assert_eq!(
        structure(&capture),
        Outcome::Blocked("no attributable incident")
    );
    let failed = format!("{READ_BACK_FAILED}x\n");
    assert_eq!(structure(&failed), Outcome::Blocked("read-back failed"));
}

#[test]
fn p034_a_numbered_entry_passes() {
    let capture = report_capture(OPENED_MS, false, &full_report(), "0");
    assert_eq!(steps(&capture), Outcome::Pass);
    assert_eq!(
        outcome_row(steps(&capture)),
        (Some(Verdict::Pass), ReportState::Pass)
    );
}

#[test]
fn p034_the_empty_placeholder_is_manual_check() {
    let sections = six("A.", "B.", &ranked(&[entry("high", "x", "")]), NO_STEPS);
    let capture = report_capture(OPENED_MS, false, &sections, "0");
    assert_eq!(steps(&capture), Outcome::ManualCheck);
    assert_eq!(
        outcome_row(steps(&capture)),
        (Some(Verdict::CalibrationRegion), ReportState::ManualCheck)
    );
}

#[test]
fn p034_the_degraded_notice_is_blocked() {
    let sections = six(
        "detail",
        NO_TIMELINE,
        &format!("\n{DEGRADED_NOTICE}\n\n"),
        DEGRADED_NOTICE,
    );
    let capture = report_capture(OPENED_MS, true, &sections, "0");
    assert_eq!(steps(&capture), Outcome::Blocked("report degraded"));
}

#[test]
fn p034_an_absent_section_is_blocked() {
    let sections = full_report().replace(
        "## Investigation Steps\n\n1. Inspect the conductor worker's retry loop.\n   - _Expected yield:_ the retried call\n\n",
        "",
    );
    let capture = report_capture(OPENED_MS, false, &sections, "0");
    assert_eq!(
        steps(&capture),
        Outcome::Blocked("no Investigation Steps section")
    );
    assert_eq!(structure(&capture), Outcome::Fail);
}

#[test]
fn p034_a_numbered_line_needs_digits_then_a_dot_and_a_space() {
    assert!(numbered_entry("1. step"));
    assert!(numbered_entry("12. step"));
    assert!(!numbered_entry("1.step"));
    assert!(!numbered_entry(". step"));
    assert!(!numbered_entry("- 1. step"));
}

#[test]
fn p044_a_prior_incident_and_retrieved_rows_pass() {
    let sections = format!(
        "{}{}",
        full_report(),
        previously_seen(EMITTED_MS - 86_400_000)
    );
    let capture = report_capture(OPENED_MS, false, &sections, "2");
    assert_eq!(retrieval(&capture), Outcome::Pass);
    assert_eq!(
        outcome_row(retrieval(&capture)),
        (Some(Verdict::Pass), ReportState::Pass)
    );
}

#[test]
fn p044_an_entry_opened_after_the_attributed_incident_is_never_retrieval() {
    let sections = format!("{}{}", full_report(), previously_seen(OPENED_MS + 1_000));
    let capture = report_capture(OPENED_MS, false, &sections, "2");
    assert_eq!(retrieval(&capture), Outcome::Blocked(NO_PRIOR));
}

#[test]
fn p044_zero_or_unknown_rows_are_blocked() {
    let sections = format!(
        "{}{}",
        full_report(),
        previously_seen(EMITTED_MS - 86_400_000)
    );
    for rows in ["0", "unknown"] {
        let capture = report_capture(OPENED_MS, false, &sections, rows);
        assert_eq!(retrieval(&capture), Outcome::Blocked(NO_PRIOR), "{rows}");
        assert_eq!(
            outcome_row(retrieval(&capture)),
            (None, ReportState::Blocked)
        );
    }
}

#[test]
fn p044_no_previously_seen_section_is_blocked() {
    let capture = report_capture(OPENED_MS, false, &full_report(), "5");
    assert_eq!(retrieval(&capture), Outcome::Blocked(NO_PRIOR));
}

#[test]
fn the_canary_s_three_tokens_are_counted() {
    let capture = format!(
        "{NO_EMISSION}\n\
         {CANARY}{SURFACED} t=2026-09-29T10:00:00.000Z cue_kind=retry_storm cue_priority_tier=tier_1 parse=ok created=true deduped=false\n\
         {CANARY}{DISMISSED} t=2026-09-29T10:01:30.000Z cue_kind=retry_storm cue_priority_tier=tier_1 parse=ok created=none deduped=none\n\
         {CANARY}{PIPELINE_FAULT} t=2026-09-29T10:03:00.000Z cue_kind=retry_storm cue_priority_tier=tier_1 parse=none created=none deduped=none\n\
         {CANARY}{DISMISSED} t=2026-09-29T10:04:30.000Z cue_kind=retry_storm cue_priority_tier=tier_1 parse=ok created=none deduped=none\n\
         canary other cue-bearing digests: 1 (service_went_silent)\n"
    );
    assert_eq!(
        canary_attempts(&capture),
        CanaryAttempts {
            surfaced: 1,
            dismissed: 2,
            pipeline_fault: 1,
        }
    );
    assert_eq!(
        canary_attempts("no canary lines at all\n"),
        CanaryAttempts::default()
    );
}

#[test]
fn every_row_of_the_further_grades_is_reachable() {
    let full = report_capture(OPENED_MS, false, &full_report(), "0");
    let manual = report_capture(
        OPENED_MS,
        false,
        &six(NO_SYMPTOM, "B.", &ranked(&[entry("high", "x", "")]), "1. y"),
        "0",
    );
    let failed = report_capture(
        OPENED_MS,
        false,
        &full_report().replace("## Evidence\n", "## Evidence moved\n"),
        "0",
    );
    let reached: BTreeSet<String> = [
        structure(&full),
        structure(&manual),
        structure(&failed),
        structure(&format!("{READ_BACK_FAILED}x\n")),
    ]
    .iter()
    .map(|outcome| format!("{:?}", outcome_row(*outcome)))
    .collect();
    assert_eq!(reached.len(), 4, "P-031's four rows");
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

// ---- the capture's attribution sweep --------------------------------------------------------------

#[test]
fn the_sweep_reads_an_incident_that_resolved_below_the_lowest_active_one() {
    // Drive a3, 2026-09-29: at the capture's first poll only incident 6 was active — the canary's (4)
    // and the one formed after the scenario's storm (5, inferred from the dense row ids) had
    // auto-resolved at 16:41:34Z. A window anchored at the lowest ACTIVE id starts at 6 and never
    // reads either; this one must.
    let active_at_first_poll = [6_i64];
    let window = sweep_window(active_at_first_poll.iter().copied().max());
    for resolved in [4, 5] {
        assert!(
            window.contains(&resolved),
            "incident {resolved} resolved before the poll and sits below the lowest active id; window {window:?}"
        );
    }
    assert!(window.contains(&6), "the active incident is read too");
}

#[test]
fn the_sweep_window_is_bounded_and_floored_at_one() {
    assert_eq!(sweep_window(None), 1..1 + SWEEP_BOUND);
    assert_eq!(sweep_window(Some(1)), 1..1 + SWEEP_BOUND);
    let high = sweep_window(Some(200));
    assert_eq!(high, 200 - (SWEEP_BOUND - 1)..201);
    assert_eq!(high.end - high.start, SWEEP_BOUND);
}

// ---- the capture's canary pairing ------------------------------------------------------------------
// Pulse log lines in the shapes measured at the 2026-09-29 series, fields only.

fn log(t: &str, target: &str, fields: serde_json::Value) -> serde_json::Value {
    serde_json::json!({ "timestamp": t, "target": target, "fields": fields })
}

fn cue_tick(t: &str, mode: &str, kind: &str) -> [serde_json::Value; 2] {
    [
        log(
            t,
            DIGEST_TICK,
            serde_json::json!({ "mode": mode, "cue_present": true }),
        ),
        log(
            t,
            DIGEST_ASSEMBLE,
            serde_json::json!({ "mode": mode, "cue_kind": kind, "cue_priority_tier": "autonomous" }),
        ),
    ]
}

fn prompt(t: &str) -> serde_json::Value {
    log(
        t,
        "interpretation.prompt.assemble",
        serde_json::json!({ "prompt_version": "v2.2" }),
    )
}

fn parse_ok(t: &str) -> serde_json::Value {
    log(
        t,
        "interpretation.json.parse",
        serde_json::json!({ "parse_outcome": "ok" }),
    )
}

fn created(t: &str) -> serde_json::Value {
    log(
        t,
        "interpretation.incident.created",
        serde_json::json!({ "created": true, "deduped": false }),
    )
}

fn paired(lines: &[serde_json::Value]) -> Vec<String> {
    let refs: Vec<&serde_json::Value> = lines.iter().collect();
    pair_canary_attempts(&refs)
}

#[test]
fn a_tier_2_tick_before_the_parse_does_not_turn_a_dismissal_into_a_fault() {
    // b2 storm 1: its prompt assembled at once, a tier-2 error-rate tick came 3 s later, and its parse
    // `ok` followed with no incident outcome.
    let [tick, assemble] = cue_tick("17:20:21.152", "tier1", "retry_storm");
    let [tier2, tier2_assemble] = cue_tick("17:20:24.244", "tier2", "error_rate_spike");
    let lines = [
        tick,
        assemble,
        prompt("17:20:21.182"),
        tier2,
        tier2_assemble,
        parse_ok("17:20:28.180"),
        prompt("17:20:28.180"),
        parse_ok("17:20:35.510"),
    ];
    let out = paired(&lines);
    assert!(
        out[0].starts_with(&format!("{CANARY}{DISMISSED} ")),
        "{out:?}"
    );
    assert_eq!(
        out[1],
        "canary other cue-bearing digests: 1 (error_rate_spike)"
    );
}

#[test]
fn a_tier_2_tick_before_the_parse_does_not_turn_a_surfacing_into_a_fault() {
    // b1 storm 1: a tier-2 tick 0.95 s after the canary's, then its parse `ok` and an incident.
    let [tick, assemble] = cue_tick("17:02:13.285", "tier1", "retry_storm");
    let [tier2, tier2_assemble] = cue_tick("17:02:14.232", "tier2", "error_rate_spike");
    let lines = [
        tick,
        assemble,
        prompt("17:02:13.296"),
        tier2,
        tier2_assemble,
        parse_ok("17:02:18.030"),
        created("17:02:18.058"),
        prompt("17:02:18.060"),
    ];
    assert!(paired(&lines)[0].starts_with(&format!("{CANARY}{SURFACED} ")));
}

#[test]
fn an_earlier_digest_s_outcome_is_never_the_canary_s() {
    // The canary's digest queued behind a running inference: that inference's parse and incident land
    // after the canary's tick and before the canary's own prompt.
    let [tick, assemble] = cue_tick("16:00:10.000", "tier1", "retry_storm");
    let lines = [
        prompt("16:00:08.000"),
        tick,
        assemble,
        parse_ok("16:00:12.000"),
        created("16:00:12.020"),
        prompt("16:00:12.030"),
        parse_ok("16:00:16.000"),
    ];
    assert!(paired(&lines)[0].starts_with(&format!("{CANARY}{DISMISSED} ")));
}

#[test]
fn an_inference_error_or_a_skip_is_a_pipeline_fault() {
    // a1's first fire: every inference errored `model_not_configured`.
    let [tick, assemble] = cue_tick("15:57:36.798", "tier1", "retry_storm");
    let errored = [
        tick.clone(),
        assemble.clone(),
        prompt("15:57:36.810"),
        log(
            "15:57:36.810",
            "interpretation.inference.error",
            serde_json::json!({ "error_category": "model_not_configured" }),
        ),
    ];
    assert!(paired(&errored)[0].starts_with(&format!("{CANARY}{PIPELINE_FAULT} ")));
    // A digest skipped in backoff assembles no prompt at all.
    let skipped = [
        tick,
        assemble,
        log(
            "15:57:40.000",
            "interpretation.inference.skipped",
            serde_json::json!({ "reason": "backoff_active" }),
        ),
    ];
    assert!(paired(&skipped)[0].starts_with(&format!("{CANARY}{PIPELINE_FAULT} ")));
}

#[test]
fn the_next_canary_storm_s_prompt_is_never_the_previous_one_s() {
    // Storm 1's digest never reached inference; storm 2's did. Storm 1 is a fault, never storm 2's parse.
    let [tick1, assemble1] = cue_tick("17:00:00.000", "tier1", "retry_storm");
    let [tick2, assemble2] = cue_tick("17:01:30.000", "tier1", "retry_storm");
    let lines = [
        tick1,
        assemble1,
        tick2,
        assemble2,
        prompt("17:01:30.010"),
        parse_ok("17:01:35.000"),
    ];
    let out = paired(&lines);
    assert!(
        out[0].starts_with(&format!("{CANARY}{PIPELINE_FAULT} ")),
        "{out:?}"
    );
    assert!(
        out[1].starts_with(&format!("{CANARY}{DISMISSED} ")),
        "{out:?}"
    );
}

#[test]
fn a_fingerprint_is_elided_and_a_stamp_a_seed_and_a_det_prefix_are_not() {
    let fp = ["12dcd67b", "34e41302", "ed9cd723", "dd1e28cf"].concat();
    assert_eq!(
        elide_fingerprints(&format!("- `{fp}` and fingerprint_hex={}", &fp[..8])),
        "- `<fingerprint>` and fingerprint_hex=<fingerprint>"
    );
    for kept in [
        "opened_at_unix_nano=1790702606754859000",
        "\"seed\":4317033",
        "run_id: 2026-09-29T17-19-35-933",
        "a deadbee word",
        "prefix12dcd67b",
    ] {
        assert_eq!(elide_fingerprints(kept), kept);
    }
    assert_eq!(
        elide_fingerprints("- `det-span-9f2c4a7e1b6d0358`"),
        "- `det-span-<fingerprint>`",
        "the canned-evidence prefix survives, so the witness still reads it"
    );
}

// ---- producer/grader agreement --------------------------------------------------------------------

#[test]
fn the_capture_prints_every_token_the_rule_reads() {
    // The capture is feature-gated, so this default-suite target cannot call it; it can read its
    // source — its own file and the shared module whose canary pairing it prints through. Each grammar
    // literal the rule keys on must appear there verbatim.
    let capture = [
        include_str!("real_model_live.rs"),
        include_str!("real_model_common/mod.rs"),
    ]
    .concat();
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
        CORPUS_ROWS,
        CANARY,
        SURFACED,
        DISMISSED,
        PIPELINE_FAULT,
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
fn the_pinned_capture_is_blocked_on_every_further_grade() {
    // The drive never emitted, so there is no attributed report to grade: Blocked with that reason on
    // all three, never a panic, and no canary line (the capture predates them).
    for outcome in [
        structure(PINNED_CAPTURE),
        steps(PINNED_CAPTURE),
        retrieval(PINNED_CAPTURE),
    ] {
        assert_eq!(outcome, Outcome::Blocked("no attributable incident"));
        assert_eq!(outcome_row(outcome), (None, ReportState::Blocked));
    }
    assert_eq!(canary_attempts(PINNED_CAPTURE), CanaryAttempts::default());
}

#[test]
fn rule_predates_the_drive() {
    // The 2026-09-23 capture's pre-leg rule record against this file's rule. The rule was EXTENDED on
    // 2026-09-29, before the drive series, by appending below that rule's last line — so the recorded
    // rule, less its end marker, is a byte-exact prefix of the current one: P-033's rule is unedited,
    // and any edit above the extension fails here.
    let recorded =
        rule_section(&committed_capture()).expect("the capture opens with the rule record");
    let current =
        rule_section(include_str!("real_model_harvest.rs")).expect("this file carries its rule");
    let body = recorded
        .strip_suffix("// ---- rule: end ----")
        .expect("the recorded rule closes on its end marker");
    assert!(
        current.starts_with(body),
        "the 2026-09-23 rule was edited after its drive, not extended"
    );
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

// ---- the 2026-09-29 drive series (step 10) ---------------------------------------------------------
// Six captures against andromeda-pulse `e98d838` under the real-model posture on one long-lived data
// dir, recorded one row each in `evidence/attempt-ledger.md`. Graded by the rule; nobody re-judges them.

/// A series drive's committed capture, read workspace-root anchored.
fn committed_series_capture(file: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(EVIDENCE)
        .join(file);
    std::fs::read_to_string(&path)
        .expect("the committed series capture is readable")
        .replace("\r\n", "\n")
}

/// The block the capture printed after its rule record, the 2026-09-23 pin's cut.
fn capture_block(committed: &str) -> &str {
    let start = committed
        .find("\nreal-model capture\n")
        .expect("the capture block opens")
        + 1;
    let end = committed[start..]
        .find("\n.\n")
        .map(|at| start + at + 1)
        .expect("libtest's mark closes the capture block");
    &committed[start..end]
}

#[test]
fn each_series_block_equals_its_committed_capture() {
    for drive in &SERIES {
        let committed = committed_series_capture(drive.file);
        assert_eq!(capture_block(&committed), drive.block, "{}", drive.label);
    }
}

#[test]
fn each_series_drive_recorded_the_current_rule_before_it_fired() {
    // The rule was not touched between the series and now, so every drive's pre-leg record equals it.
    let current =
        rule_section(include_str!("real_model_harvest.rs")).expect("this file carries its rule");
    for drive in &SERIES {
        let recorded = rule_section(&committed_series_capture(drive.file))
            .expect("the capture opens with the rule record");
        assert_eq!(
            recorded, current,
            "{}: the rule moved after the drive",
            drive.label
        );
    }
}

#[test]
fn the_series_captures_carry_no_fingerprint() {
    // Elided once after the series by a mirror of the capture's own rule; this is the Rust rule
    // finding nothing left in any committed capture, the ledger and the pins included.
    let ledger = committed_series_capture("attempt-ledger.md");
    assert_eq!(elide_fingerprints(&ledger), ledger, "the attempt ledger");
    for drive in &SERIES {
        let committed = committed_series_capture(drive.file);
        assert_eq!(elide_fingerprints(&committed), committed, "{}", drive.label);
        assert_eq!(
            elide_fingerprints(drive.block),
            drive.block,
            "{}",
            drive.label
        );
    }
}

/// What the rule measured on each drive: its route, its rank-1 grade, the three further grades and
/// the canary tokens the capture printed (b1's and b2's `pipeline-fault` are the pairing artifacts
/// the ledger corrects from Pulse's own log; the tokens are recorded as printed).
fn measured(label: &str) -> (Route, Grade, [Outcome; 3], CanaryAttempts) {
    let none = Outcome::Blocked("no attributable incident");
    let tokens = |surfaced, dismissed, pipeline_fault| CanaryAttempts {
        surfaced,
        dismissed,
        pipeline_fault,
    };
    match label {
        "a1-pipeline-fault" => (
            Route::PreflightBlocked,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(0, 0, 1),
        ),
        "a1" => (
            Route::PreflightBlocked,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(0, 1, 0),
        ),
        "a2" => (
            Route::EmittedNoReadBack,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(1, 0, 0),
        ),
        "a3" => (
            Route::ReadBack,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(1, 0, 0),
        ),
        "b1" => (
            Route::ReadBack,
            Grade::NoAttributableIncident,
            [none; 3],
            tokens(0, 1, 1),
        ),
        "b2" => (
            Route::ReadBack,
            Grade::NotIdentified,
            [Outcome::Pass; 3],
            tokens(1, 0, 1),
        ),
        other => panic!("no measurement recorded for {other}"),
    }
}

#[test]
fn each_series_drive_grades_as_the_ledger_records() {
    for drive in &SERIES {
        let (route_, grade_, further, tokens) = measured(drive.label);
        let block = drive.block;
        assert_eq!(route(block), route_, "{}", drive.label);
        assert_eq!(grade(block), grade_, "{}", drive.label);
        assert_eq!(
            [structure(block), steps(block), retrieval(block)],
            further,
            "{}",
            drive.label
        );
        assert_eq!(canary_attempts(block), tokens, "{}", drive.label);
        assert!(
            trace_conforms(block),
            "{}: the trace witness set",
            drive.label
        );
    }
}

#[test]
fn every_series_drive_witnesses_the_real_model_and_a_clear_launch() {
    for drive in &SERIES {
        assert!(real_model_witnessed(drive.block), "{}", drive.label);
        assert!(launch_cwd_clear(drive.block), "{}", drive.label);
    }
}

#[test]
fn the_series_envelopes_carry_the_eleven_keys() {
    for drive in &SERIES {
        let Some(line) = drive
            .block
            .lines()
            .find_map(|l| l.strip_prefix("envelope: "))
            .filter(|rest| rest.starts_with('{'))
        else {
            continue;
        };
        let value: serde_json::Value = serde_json::from_str(line).expect("the envelope parses");
        let mut keys: Vec<&str> = value
            .as_object()
            .expect("the envelope is an object")
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(keys, ENVELOPE_KEYS_SORTED, "{}", drive.label);
        let record: RunRecord = serde_json::from_str(line).expect("the closed sets");
        assert_eq!(record.scenario, "real-model-interpretation");
        assert_eq!(record.verdict, None, "{}: declare-only", drive.label);
    }
}

#[test]
fn v3_09_is_not_met_by_the_series() {
    // D1 (posture contract, The drive series (a)): met only if at least one drive is graded AND every
    // graded drive reads Identified. A drive is graded when it attributed an incident on the read-back
    // route. b2 alone did, and it reads NotIdentified — recorded, never replaced.
    let graded: Vec<&str> = SERIES
        .iter()
        .filter(|d| route(d.block) == Route::ReadBack && attributed_report(d.block).is_ok())
        .map(|d| d.label)
        .collect();
    assert_eq!(graded, ["b2"]);
    let b2 = SERIES
        .iter()
        .find(|d| d.label == "b2")
        .expect("b2 is pinned");
    assert_eq!(grade(b2.block), Grade::NotIdentified);
    assert_eq!(
        row(grade(b2.block)),
        (Some(Verdict::CalibrationRegion), ReportState::ManualCheck)
    );
    let met = !graded.is_empty()
        && SERIES
            .iter()
            .filter(|d| graded.contains(&d.label))
            .all(|d| grade(d.block) == Grade::Identified);
    assert!(!met, "the series does not meet v3-09");
}
