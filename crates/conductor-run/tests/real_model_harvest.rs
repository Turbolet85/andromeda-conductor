//! Live-leg harvest for the real-model interpretation leg (`verification-matrix.json#v3-09`).
//!
//! The graded half of the capture/grade split: `real_model_live.rs` (feature `live-pulse`) PRINTS what
//! the one drive produced, and this target — in the DEFAULT suite — grades it. Each graded capture is a
//! committed evidence file pinned here by the sha256 of its LF-normalized content, and grading reads the
//! file only after its digest matches: no capture text sits in test source (security-plan §Security
//! Anti-Patterns → Data Protection).
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
//! PINNED CAPTURE — the one graded drive, 2026-09-23, graded from the copy whose storm prefix is elided
//! (`ELIDED_CAPTURE`), against andromeda-pulse HEAD `83d4060` under the
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

mod evidence_pin;
mod real_model_common;
mod real_model_grading;

use std::collections::BTreeSet;
use std::path::Path;

use conductor_core::{ReportState, Verdict};
use evidence_pin::{check_digest, committed, pinned};
mod real_model_series;

use real_model_common::{
    DIGEST_ASSEMBLE, DIGEST_TICK, SWEEP_BOUND, WORKSPACE_KEY_PLACEHOLDER,
    canary_attempts as pair_canary_attempts, elide_fingerprints, envelope_record, mask_host_paths,
    mask_workspace_key, rule_section, sweep_window, workspace_rendering,
};
use real_model_grading::series_2026_09_29::series_capture;
use real_model_grading::series_2026_09_30::capture_2026_09_30;
use real_model_grading::series_2026_10_01::capture_2026_10_01;
use real_model_grading::series_2026_10_06::capture_2026_10_06;
use real_model_series::{
    Drive, EVIDENCE, EVIDENCE_2026_09_30, EVIDENCE_2026_10_01, EVIDENCE_2026_10_06, SERIES,
    SERIES_2026_09_30, SERIES_2026_10_01, SERIES_2026_10_06,
};

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

// ---- digest pins over committed evidence -----------------------------------------------------------
// Every capture this harvest grades is a committed evidence file held by the sha256 of its LF-normalized
// content; grading reads the file only after its digest matches. No capture text sits in test source
// (security-plan §Security Anti-Patterns → Data Protection).

/// The block the capture printed after its rule record, from its first line to its last.
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

// ---- the 2026-09-23 capture (step 14) --------------------------------------------------------------

/// The 2026-09-23 drive's capture as graded and pinned: [`FROZEN_CAPTURE`] passed once through
/// `elide_fingerprints` (its storm prefix), otherwise byte-identical. The original has carried the same
/// bytes since it was elided in place on 2026-10-02 under the founder's ruling.
const ELIDED_CAPTURE: &str = "conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/rm-capture-2026-09-22-elided.txt";
const ELIDED_CAPTURE_SHA256: &str =
    "d57c2613698a3182862f370cb49968c185f606911cf50d33d4841ec62bb9c8b1";

/// The original capture, frozen save once: elided in place on 2026-10-02 under the founder's ruling, so
/// it no longer keeps its storm prefix.
const FROZEN_CAPTURE: &str =
    "conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt";

/// What the rule measured on the one drive.
const MEASURED: Grade = Grade::NoAttributableIncident;

/// The graded 2026-09-23 capture, digest-checked.
fn committed_capture() -> String {
    pinned(ELIDED_CAPTURE, ELIDED_CAPTURE_SHA256)
}

#[test]
fn the_pinned_capture_grades_as_the_drive_measured() {
    // The rule graded it; nobody re-judged it.
    let committed = committed_capture();
    let block = capture_block(&committed);
    assert_eq!(grade(block), MEASURED);
    assert_eq!(row(grade(block)), (None, ReportState::Blocked));
}

#[test]
fn the_pinned_capture_witnesses_the_real_model_and_a_clear_launch() {
    // B1 and the launch witness: a canned or fouled launch is never graded. This route read no
    // report, so the evidence half of B1 is vacuous here; the operative witnesses are the whole-log
    // model mode and the envelope's zero `det-` refs.
    let committed = committed_capture();
    let block = capture_block(&committed);
    assert!(real_model_witnessed(block));
    assert!(launch_cwd_clear(block));
    assert!(
        section_body(block, "## Evidence").is_none(),
        "no report was read on this route"
    );
    assert!(
        block
            .lines()
            .any(|l| l == "envelope fingerprints: 0, det- prefixed: 0")
    );
}

#[test]
fn the_pinned_envelope_carries_the_eleven_keys_and_the_closed_sets() {
    let committed = committed_capture();
    let block = capture_block(&committed);
    let Some(record) = envelope_record(block, ELIDED_CAPTURE) else {
        assert_eq!(
            route(block),
            Route::PreflightBlocked,
            "only a preflight-blocked capture may carry no envelope"
        );
        return;
    };
    assert_eq!(record.scenario, "real-model-interpretation");
    assert_eq!(record.state, ReportState::Blocked);
    assert_eq!(record.verdict, None);
}

#[test]
fn the_pinned_trace_carries_its_route_s_witness_set() {
    let committed = committed_capture();
    let block = capture_block(&committed);
    assert_eq!(route(block), Route::PreflightBlocked);
    assert!(trace_conforms(block));
}

#[test]
fn the_pinned_capture_is_blocked_on_every_further_grade() {
    // The drive never emitted, so there is no attributed report to grade: Blocked with that reason on
    // all three, never a panic, and no canary line (the capture predates them).
    let committed = committed_capture();
    let block = capture_block(&committed);
    for outcome in [structure(block), steps(block), retrieval(block)] {
        assert_eq!(outcome, Outcome::Blocked("no attributable incident"));
        assert_eq!(outcome_row(outcome), (None, ReportState::Blocked));
    }
    assert_eq!(canary_attempts(block), CanaryAttempts::default());
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
fn the_elided_capture_matches_its_pinned_digest() {
    let text = committed(ELIDED_CAPTURE);
    assert_eq!(
        check_digest(ELIDED_CAPTURE, &text, ELIDED_CAPTURE_SHA256),
        Ok(())
    );
}

#[test]
fn the_frozen_capture_is_elided_in_place_and_equals_the_graded_copy() {
    // The original, elided in place, is byte-identical to the graded copy, and the capture's own elision
    // has nothing left to do on it. The copy stays the pinned form.
    let frozen = committed(FROZEN_CAPTURE);
    let copy = committed(ELIDED_CAPTURE);
    assert_eq!(frozen, copy);
    assert_eq!(elide_fingerprints(&frozen), frozen);
}

#[test]
fn a_one_byte_change_to_a_pinned_capture_fails_its_digest() {
    // The pin can fail: one ASCII byte flipped in memory, and the check names the file, never the text.
    let drive = &SERIES[SERIES.len() - 1];
    let name = format!("{EVIDENCE}/{}", drive.file);
    let text = committed(&name);
    assert_eq!(check_digest(&name, &text, drive.sha256), Ok(()));
    let mut bytes = text.clone().into_bytes();
    let at = (bytes.len() / 2..bytes.len())
        .find(|&i| bytes[i].is_ascii_alphanumeric())
        .expect("an ASCII byte to flip");
    bytes[at] ^= 0x01;
    let tampered = String::from_utf8(bytes).expect("an ASCII flip keeps UTF-8");
    let error = check_digest(&name, &tampered, drive.sha256).expect_err("the tampered copy fails");
    assert!(error.contains(&name), "the error names the file: {error}");
    for line in text.lines().filter(|l| l.trim().len() >= 16) {
        assert!(!error.contains(line), "the error carries capture text");
    }
}

/// The real-model test sources outside `real_model_grading/`.
const SOURCES: [&str; 4] = [
    include_str!("real_model_harvest.rs"),
    include_str!("real_model_series/mod.rs"),
    include_str!("real_model_common/mod.rs"),
    include_str!("real_model_live.rs"),
];

/// Every module under `real_model_grading/`, by file name. A module missing here is unscanned, which
/// `the_capture_text_arm_scans_every_grading_module` holds against the directory.
const GRADING_MODULES: [(&str, &str); 11] = [
    (
        "canary_pairing.rs",
        include_str!("real_model_grading/canary_pairing.rs"),
    ),
    (
        "capture_population.rs",
        include_str!("real_model_grading/capture_population.rs"),
    ),
    (
        "capture_tokens.rs",
        include_str!("real_model_grading/capture_tokens.rs"),
    ),
    ("mod.rs", include_str!("real_model_grading/mod.rs")),
    (
        "scrub_and_sweep.rs",
        include_str!("real_model_grading/scrub_and_sweep.rs"),
    ),
    (
        "series_2026_09_29.rs",
        include_str!("real_model_grading/series_2026_09_29.rs"),
    ),
    (
        "series_2026_09_30.rs",
        include_str!("real_model_grading/series_2026_09_30.rs"),
    ),
    (
        "series_2026_10_01.rs",
        include_str!("real_model_grading/series_2026_10_01.rs"),
    ),
    (
        "series_2026_10_06.rs",
        include_str!("real_model_grading/series_2026_10_06.rs"),
    ),
    (
        "witnesses.rs",
        include_str!("real_model_grading/witnesses.rs"),
    ),
    (
        "workspace_mask.rs",
        include_str!("real_model_grading/workspace_mask.rs"),
    ),
];

/// Every pinned capture this harvest grades, labelled, each digest-checked.
fn graded_captures() -> Vec<(String, String)> {
    let mut captures = vec![(ELIDED_CAPTURE.to_string(), committed_capture())];
    for drive in &SERIES {
        captures.push((drive.label.to_string(), series_capture(drive)));
    }
    for drive in &SERIES_2026_09_30 {
        captures.push((drive.label.to_string(), capture_2026_09_30(drive)));
    }
    for drive in &SERIES_2026_10_01 {
        captures.push((
            format!("2026-10-01 {}", drive.label),
            capture_2026_10_01(drive),
        ));
    }
    for drive in &SERIES_2026_10_06 {
        captures.push((
            format!("2026-10-06 {}", drive.label),
            capture_2026_10_06(drive),
        ));
    }
    captures
}

/// A capture block's report-section lines long enough to be prose: 40 characters or more, never a `## `
/// header.
fn report_lines(block: &str) -> impl Iterator<Item = &str> {
    block
        .lines()
        .skip_while(|l| !l.starts_with("## "))
        .take_while(|l| *l != END_OF_SECTIONS)
        .filter(|l| l.chars().count() >= 40 && !l.starts_with("## "))
}

/// The label of the first capture whose report prose sits in `sources`, or how many lines were checked.
fn capture_text_in(sources: &str, captures: &[(String, String)]) -> Result<usize, String> {
    let mut checked = 0;
    for (label, text) in captures {
        for line in report_lines(capture_block(text)) {
            checked += 1;
            if sources.contains(line) {
                return Err(label.clone());
            }
        }
    }
    Ok(checked)
}

/// The scanned sources, with `planted` appended to the grading module named `into`.
fn scanned_sources(into: &str, planted: &str) -> String {
    let mut sources = SOURCES.concat();
    for (name, text) in GRADING_MODULES {
        sources.push_str(text);
        if name == into {
            sources.push_str(planted);
        }
    }
    sources
}

#[test]
fn no_committed_capture_text_sits_in_test_source() {
    // Every report-section line of a pinned capture long enough to be prose is absent from the real-model
    // test sources: the four above and every grading module. The count guard keeps the arm from passing
    // over captures that rendered no report at all.
    match capture_text_in(&scanned_sources("", ""), &graded_captures()) {
        Ok(checked) => assert!(checked > 0, "no report line was checked"),
        Err(label) => panic!("{label}: a report line sits in test source"),
    }
}

// andromeda:walks-tree — lists the grading modules' dir, so an unlisted module file moves the result.
#[test]
fn the_capture_text_arm_scans_every_grading_module() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/real_model_grading");
    let on_disk: BTreeSet<String> = std::fs::read_dir(&dir)
        .expect("the grading modules' dir is readable")
        .map(|entry| entry.expect("a grading module entry").file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".rs"))
        .collect();
    let scanned: BTreeSet<String> = GRADING_MODULES
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect();
    assert_eq!(on_disk, scanned);
}

#[test]
fn a_report_line_planted_in_a_grading_module_is_caught() {
    // The inverse control: a report line read from a committed capture at run time, appended to an
    // in-memory copy of one grading module, is caught — and the same sources without it are clean.
    let captures = graded_captures();
    let (label, planted) = captures
        .iter()
        .find_map(|(label, text)| {
            report_lines(capture_block(text))
                .next()
                .map(|line| (label.clone(), format!("\n{line}\n")))
        })
        .expect("a capture rendered a report line");
    assert_eq!(
        capture_text_in(&scanned_sources("witnesses.rs", &planted), &captures),
        Err(label)
    );
    assert!(capture_text_in(&scanned_sources("witnesses.rs", ""), &captures).is_ok_and(|n| n > 0));
}
