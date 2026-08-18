//! Live-leg evidence harvest for the statistical-anomaly family (P-009..P-012).
//!
//! Neither of the family's authored read-back checks can grade under deterministic L4: the composed
//! text cannot carry a cue kind (an open incident's report renders the degraded branch, which names
//! no cue), and the evidence count reads `span_refs` = `Incident.evidence_refs.span_ids`, a field no
//! producer populates. The surface that does carry the family's reaction is Pulse's
//! `triage.cue.emit` line — kind · priority · scope · magnitude · absolute_value ·
//! persistence_seconds · confidence · suppression_bypassed — so the live proof grades there, exactly
//! as the fingerprint family's harvest does on `triage.pattern.storm.detected`.
//!
//! TEST-ONLY affordance (the storm-harvest precedent): nothing here is wired into the run path and
//! nothing harvested reaches a Conductor artifact. On Windows the capture is
//! `{data_dir}/logs/agent-latest.jsonl.<date>`, sliced to the leg window by a pre-leg line count.
//!
//! The canary-fingerprint equality comparator also lives here: `emit_canary` logs Conductor's
//! computed 32-hex fingerprint on `message`, Pulse's storm line carries an 8-hex prefix of its own
//! derivation, and prefix equality on a live leg is the transcription-equality proof the 2026-08-17
//! alignment could only assert at unit tier (Conductor-vs-Conductor). A mismatch here is a
//! TRANSCRIPTION defect against architecture §Read-Back Dependency Posture, never a scenario failure.

use std::path::Path;

/// One `triage.cue.emit` observation.
#[derive(Debug, Clone, PartialEq)]
struct CueEmitted {
    kind: String,
    priority: String,
    scope: String,
    magnitude: f64,
    absolute_value: f64,
    persistence_seconds: u64,
    confidence: f64,
    suppression_bypassed: bool,
}

/// Slice a harvested log to the leg window. `skip_lines` is the pre-leg line count; the six-item
/// live recipe requires a FRESH data dir, under which 0 is correct by construction.
fn harvest_since(path: &Path, skip_lines: usize) -> std::io::Result<Vec<String>> {
    let body = std::fs::read_to_string(path)?;
    Ok(body.lines().skip(skip_lines).map(str::to_owned).collect())
}

/// Pull every cue-emitted observation out of harvested lines, in order. A line that does not parse,
/// is not this target, or is missing a field is SKIPPED rather than fatal — a harvest is
/// other-process output (the verdict/error wall applied to evidence).
fn parse_cue_emitted(lines: &[String]) -> Vec<CueEmitted> {
    lines
        .iter()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|v| v.get("target").and_then(|t| t.as_str()) == Some("triage.cue.emit"))
        .filter_map(|v| {
            let f = v.get("fields")?;
            Some(CueEmitted {
                kind: f.get("kind")?.as_str()?.to_owned(),
                priority: f.get("priority")?.as_str()?.to_owned(),
                scope: f.get("scope")?.as_str()?.to_owned(),
                magnitude: f.get("magnitude")?.as_f64()?,
                absolute_value: f.get("absolute_value")?.as_f64()?,
                persistence_seconds: f.get("persistence_seconds")?.as_u64()?,
                confidence: f.get("confidence")?.as_f64()?,
                suppression_bypassed: f.get("suppression_bypassed")?.as_bool()?,
            })
        })
        .collect()
}

/// The family's candidate check: a cue of `kind` fired at all. Existence alone proves the sample
/// floor — Pulse's evaluator skips below-floor snapshots before a cue can exist.
fn cue_fired<'a>(observed: &'a [CueEmitted], kind: &str) -> Result<&'a CueEmitted, String> {
    observed
        .iter()
        .find(|c| c.kind == kind)
        .ok_or_else(|| format!("no {kind} cue in the harvest window"))
}

/// Baseline convergence made concrete: the live baseline-relative ratio crossed the spec multiplier
/// with the confidence the sample accumulation implies (confidence = samples/100, capped at 1.0).
fn convergence_witness(cue: &CueEmitted) -> Result<(), String> {
    if cue.magnitude < 3.0 {
        return Err(format!("magnitude {} below the 3.0x spec multiplier", cue.magnitude));
    }
    if cue.confidence < 0.7 {
        return Err(format!("confidence {} below the 0.7 floor witness", cue.confidence));
    }
    Ok(())
}

/// The tier the shipped magnitudes reach: Suggested (Autonomous needs >=5x with confidence >=0.9).
fn suggested_tier(cue: &CueEmitted) -> Result<(), String> {
    if cue.priority == "suggested" {
        Ok(())
    } else {
        Err(format!("cue fired at priority {:?}, not suggested", cue.priority))
    }
}

/// The P-057 dual-condition witness the spike carries for free: 0.35 absolute error rate crosses the
/// 0.05 absolute-bypass arm, so the cue must arrive with the bypass flag already set.
fn absolute_bypass_witness(cue: &CueEmitted) -> Result<(), String> {
    if cue.suppression_bypassed {
        Ok(())
    } else {
        Err("the cue did not carry the dual-condition bypass flag".to_owned())
    }
}

/// The message prefix `emit_canary` logs its computed fingerprint under.
const CANARY_FP_WITNESS_PREFIX: &str = "canary fingerprint computed ";

/// Conductor's own computed canary fingerprint, recovered from its self-obs stream (flat JSON lines;
/// `message` is a top-level field there, unlike Pulse's nested `fields`).
fn conductor_canary_fingerprint(lines: &[String]) -> Option<String> {
    lines
        .iter()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter_map(|v| v.get("message").and_then(|m| m.as_str()).map(str::to_owned))
        .find_map(|m| m.strip_prefix(CANARY_FP_WITNESS_PREFIX).map(str::to_owned))
        .filter(|fp| fp.len() == 32 && fp.chars().all(|c| c.is_ascii_hexdigit()))
}

/// Pulse's storm-line fingerprints (the 8-hex prefix of its own derivation), in order.
fn pulse_storm_fingerprints(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|v| {
            v.get("target").and_then(|t| t.as_str()) == Some("triage.pattern.storm.detected")
        })
        .filter_map(|v| v.get("fields")?.get("fingerprint_hex")?.as_str().map(str::to_owned))
        .collect()
}

/// The transcription-equality check: Pulse's 8-hex storm prefix must be the head of Conductor's
/// 32-hex computed value for the same canary exception.
fn canary_prefix_equality(conductor_fp: &str, pulse_prefix: &str) -> Result<(), String> {
    if conductor_fp.len() != 32 {
        return Err(format!("conductor fingerprint is {} chars, not 32", conductor_fp.len()));
    }
    if pulse_prefix.len() != 8 {
        return Err(format!("pulse prefix is {} chars, not 8", pulse_prefix.len()));
    }
    if &conductor_fp[..8] == pulse_prefix {
        Ok(())
    } else {
        Err(format!(
            "derivations diverge: conductor {}.. vs pulse {}",
            &conductor_fp[..8],
            pulse_prefix
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Source-derived line shape (`andromeda-pulse crates/triage/src/cue/emitter.rs:194-205` at HEAD
    /// `efabe8e`), mirroring the captured storm-line format — to be re-pinned VERBATIM from the live
    /// leg's capture, per the storm-harvest precedent.
    fn cue_line(kind: &str, priority: &str, magnitude: f64, confidence: f64, bypassed: bool) -> String {
        format!(
            r#"{{"fields":{{"absolute_value":0.35,"confidence":{confidence},"kind":"{kind}","magnitude":{magnitude},"persistence_seconds":100,"priority":"{priority}","scope":"service","suppression_bypassed":{bypassed}}},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-18T00:00:00.000Z"}}"#
        )
    }

    fn other_line() -> String {
        r#"{"fields":{"rows_ingested":15},"level":"INFO","message":"","target":"buffer.tick","timestamp":"2026-08-18T00:00:00.000Z"}"#.to_owned()
    }

    /// Verbatim shape of Conductor's flat self-obs line carrying the canary witness.
    fn witness_line(fp: &str) -> String {
        format!(
            r#"{{"timestamp_ms":1755500000000,"level":"INFO","target":"conductor_run","service.name":"conductor","service.version":"0.1.0","deployment.environment":"local","run_id":"2026-08-18T00-00-00-000","message":"canary fingerprint computed {fp}"}}"#
        )
    }

    /// Verbatim storm-line shape from the 2026-08-15 capture (shared with `storm_harvest.rs`).
    fn storm_line(fp: &str) -> String {
        format!(
            r#"{{"fields":{{"cue_kind":"retry_storm","fingerprint_hex":"{fp}","occurrence_count":10,"severity_hint":"autonomous","window_seconds":30}},"level":"INFO","message":"","target":"triage.pattern.storm.detected","timestamp":"2026-08-18T00:00:00.000Z"}}"#
        )
    }

    #[test]
    fn the_parser_reads_the_cue_emit_line_shape() {
        let lines = vec![other_line(), cue_line("error_rate_spike", "suggested", 3.5, 0.7, true)];
        let observed = parse_cue_emitted(&lines);
        assert_eq!(
            observed,
            vec![CueEmitted {
                kind: "error_rate_spike".to_owned(),
                priority: "suggested".to_owned(),
                scope: "service".to_owned(),
                magnitude: 3.5,
                absolute_value: 0.35,
                persistence_seconds: 100,
                confidence: 0.7,
                suppression_bypassed: true,
            }]
        );
    }

    #[test]
    fn the_error_spike_cue_satisfies_candidate_convergence_tier_and_bypass() {
        let observed =
            parse_cue_emitted(&[cue_line("error_rate_spike", "suggested", 3.5, 0.7, true)]);
        let cue = cue_fired(&observed, "error_rate_spike").expect("cue fired");
        assert_eq!(convergence_witness(cue), Ok(()));
        assert_eq!(suggested_tier(cue), Ok(()));
        assert_eq!(absolute_bypass_witness(cue), Ok(()));
    }

    #[test]
    fn the_latency_cue_satisfies_candidate_and_convergence() {
        let observed =
            parse_cue_emitted(&[cue_line("latency_regression", "suggested", 3.0, 1.0, true)]);
        let cue = cue_fired(&observed, "latency_regression").expect("cue fired");
        assert_eq!(convergence_witness(cue), Ok(()));
        assert_eq!(suggested_tier(cue), Ok(()));
    }

    #[test]
    fn a_below_multiplier_cue_fails_the_convergence_witness() {
        let observed = parse_cue_emitted(&[cue_line("error_rate_spike", "curious", 2.5, 1.0, false)]);
        let err = convergence_witness(&observed[0]).unwrap_err();
        assert!(err.contains("below the 3.0x"), "{err}");
    }

    #[test]
    fn a_low_confidence_cue_fails_the_convergence_witness() {
        let observed = parse_cue_emitted(&[cue_line("error_rate_spike", "curious", 3.5, 0.4, false)]);
        let err = convergence_witness(&observed[0]).unwrap_err();
        assert!(err.contains("below the 0.7"), "{err}");
    }

    #[test]
    fn an_empty_window_reports_the_missing_cue_kind() {
        let err = cue_fired(&parse_cue_emitted(&[other_line()]), "error_rate_spike").unwrap_err();
        assert!(err.contains("no error_rate_spike cue"), "{err}");
    }

    #[test]
    fn a_non_suggested_cue_fails_the_tier_witness() {
        let observed = parse_cue_emitted(&[cue_line("error_rate_spike", "curious", 3.5, 0.9, true)]);
        let err = suggested_tier(&observed[0]).unwrap_err();
        assert!(err.contains("not suggested"), "{err}");
    }

    #[test]
    fn a_malformed_line_is_skipped_not_fatal() {
        let ragged = vec![
            "not json at all".to_owned(),
            r#"{"target":"triage.cue.emit"}"#.to_owned(),
            cue_line("error_rate_spike", "suggested", 3.5, 0.7, true),
        ];
        assert_eq!(parse_cue_emitted(&ragged).len(), 1);
    }

    #[test]
    fn the_window_slice_drops_pre_leg_lines() {
        let dir = assert_fs::TempDir::new().unwrap();
        let path = dir.path().join("agent-latest.jsonl.2026-08-18");
        let body =
            format!("{}\n{}\n", other_line(), cue_line("error_rate_spike", "suggested", 3.5, 0.7, true));
        std::fs::write(&path, body).unwrap();

        assert_eq!(parse_cue_emitted(&harvest_since(&path, 0).unwrap()).len(), 1);
        assert!(parse_cue_emitted(&harvest_since(&path, 2).unwrap()).is_empty());
    }

    #[test]
    fn the_conductor_witness_line_yields_the_32_hex_fingerprint() {
        let fp = "4a7f2b91c6e05d3849b1e7a2c5f08d63";
        let lines = vec![other_line(), witness_line(fp)];
        assert_eq!(conductor_canary_fingerprint(&lines), Some(fp.to_owned()));
    }

    #[test]
    fn a_malformed_witness_value_is_rejected() {
        let lines = vec![witness_line("not-a-fingerprint")];
        assert_eq!(conductor_canary_fingerprint(&lines), None);
    }

    #[test]
    fn the_pulse_storm_prefix_is_recovered_from_the_harvest() {
        let lines = vec![other_line(), storm_line("4a7f2b91")];
        assert_eq!(pulse_storm_fingerprints(&lines), vec!["4a7f2b91".to_owned()]);
    }

    #[test]
    fn the_canary_prefix_equality_holds_for_a_matching_pair() {
        assert_eq!(
            canary_prefix_equality("4a7f2b91c6e05d3849b1e7a2c5f08d63", "4a7f2b91"),
            Ok(())
        );
    }

    #[test]
    fn a_diverging_prefix_fails_the_equality_check() {
        let err =
            canary_prefix_equality("4a7f2b91c6e05d3849b1e7a2c5f08d63", "ffffffff").unwrap_err();
        assert!(err.contains("derivations diverge"), "{err}");
    }

    /// VERBATIM from the 2026-08-18 leg A capture (pulse-leg-a slice, the last suggested cue at
    /// 18:50:31) — the parser is pinned to Pulse's ACTUAL line, superseding the source-derived
    /// shape above for the live-proof claim. Late-window values: the baseline genuinely converged
    /// (magnitude ~3.09 against the authored 3.5x recipe) with confidence saturated at 1.0.
    fn captured_error_spike_line() -> String {
        r#"{"fields":{"absolute_value":0.2724857716035038,"confidence":1.0,"deployment.environment":"production","kind":"error_rate_spike","magnitude":3.090646116188503,"persistence_seconds":102,"priority":"suggested","scope":"service","service.name":"com.andromeda.pulse","service.version":"0.1.0","suppression_bypassed":true},"level":"INFO","message":"attention cue emitted","target":"triage.cue.emit","timestamp":"2026-08-18T18:50:31.109Z"}"#.to_owned()
    }

    /// VERBATIM pair from the same leg: Conductor's computed 32-hex witness and Pulse's 8-hex storm
    /// prefix for the SAME canary exception — they matched live (`bf2c0bf8`), which is the
    /// transcription-equality proof the 2026-08-17 alignment could only assert Conductor-vs-Conductor.
    fn captured_witness_line() -> String {
        r#"{"deployment.environment":"local","level":"INFO","message":"canary fingerprint computed bf2c0bf81e6c18189b4e48ca2cf46112","run_id":"2026-08-18T18-47-32-786","service.name":"conductor","service.version":"0.1.0","target":"conductor_run","timestamp_ms":1787078852996}"#.to_owned()
    }

    fn captured_storm_line() -> String {
        r#"{"fields":{"cue_kind":"retry_storm","deployment.environment":"production","fingerprint_hex":"bf2c0bf8","occurrence_count":10,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_hint":"autonomous","window_seconds":30},"level":"INFO","message":"","target":"triage.pattern.storm.detected","timestamp":"2026-08-18T18:48:18.095Z"}"#.to_owned()
    }

    #[test]
    fn the_leg_a_capture_satisfies_every_error_spike_predicate() {
        let observed = parse_cue_emitted(&[captured_error_spike_line()]);
        let cue = cue_fired(&observed, "error_rate_spike").expect("cue fired");
        assert_eq!(convergence_witness(cue), Ok(()));
        assert_eq!(suggested_tier(cue), Ok(()));
        assert_eq!(absolute_bypass_witness(cue), Ok(()));
    }

    #[test]
    fn the_leg_a_capture_proves_the_canary_prefix_equality() {
        let conductor =
            conductor_canary_fingerprint(&[captured_witness_line()]).expect("witness present");
        let pulse = pulse_storm_fingerprints(&[captured_storm_line()]);
        assert_eq!(pulse.len(), 1);
        assert_eq!(canary_prefix_equality(&conductor, &pulse[0]), Ok(()));
    }
}
