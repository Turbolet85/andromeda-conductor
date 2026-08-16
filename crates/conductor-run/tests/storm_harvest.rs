//! Live-leg evidence harvest for the fingerprint-storm family (P-017 / P-018).
//!
//! Identity cannot be graded through MCP read-back under deterministic L4: `fingerprint_refs` is
//! populated from the L4 model's `evidence_refs`, which the deterministic fixture pins to `[]`, and
//! Pulse's own computed fingerprint lands in a `span_events` column no MCP tool reads. The one
//! surface that carries identity is Pulse's `triage.pattern.storm.detected` log line, which emits
//! `severity_hint` + `occurrence_count` + an 8-char `fingerprint_hex` prefix.
//!
//! So this is a TEST-ONLY affordance, deliberately not a shipped seam: the live leg is an
//! operator gate by construction and Conductor's product never needs to read Pulse's logs. Nothing
//! here is wired into the run path, and nothing harvested reaches a Conductor artifact.
//!
//! On Windows the capture is NOT a console tee — `pulse-app` is a GUI-subsystem binary that never
//! attaches to a console — so the sink is `{data_dir}/logs/agent-latest.jsonl.<date>`, sliced to the
//! leg window by a pre-leg line count.

use std::path::Path;

/// One `triage.pattern.storm.detected` observation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct StormDetected {
    cue_kind: String,
    severity_hint: String,
    occurrence_count: u64,
    fingerprint_hex: String,
}

/// The harvest analogue of the retired `Contains "RetryStorm"` read-back check. Pulse names the cue
/// on its own detected line; the scenario's `[[expected]]` token was a guess at how that name would
/// surface in report text, and under deterministic L4 it never does.
const RETRY_STORM_CUE: &str = "retry_storm";

/// Slice a harvested Pulse log to the leg window. `skip_lines` is the pre-leg line count; the
/// six-item live recipe requires a FRESH data dir, under which 0 is correct by construction.
fn harvest_since(path: &Path, skip_lines: usize) -> std::io::Result<Vec<String>> {
    let body = std::fs::read_to_string(path)?;
    Ok(body.lines().skip(skip_lines).map(str::to_owned).collect())
}

/// Pull every storm-detected observation out of harvested lines, in order.
///
/// Pulse writes its fields nested under `fields`. A line that does not parse, is not this target, or
/// is missing a field is SKIPPED rather than fatal — a harvest is other-process output, so a
/// malformed tail must never panic the assertion (the verdict/error wall applied to evidence).
fn parse_storm_detected(lines: &[String]) -> Vec<StormDetected> {
    lines
        .iter()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|v| v.get("target").and_then(|t| t.as_str()) == Some("triage.pattern.storm.detected"))
        .filter_map(|v| {
            let f = v.get("fields")?;
            Some(StormDetected {
                cue_kind: f.get("cue_kind")?.as_str()?.to_owned(),
                severity_hint: f.get("severity_hint")?.as_str()?.to_owned(),
                occurrence_count: f.get("occurrence_count")?.as_u64()?,
                fingerprint_hex: f.get("fingerprint_hex")?.as_str()?.to_owned(),
            })
        })
        .collect()
}

/// P-017 through the storm: every occurrence the detector counted carried ONE fingerprint.
fn single_fingerprint(observed: &[StormDetected]) -> Result<String, String> {
    let mut fps: Vec<&str> = observed.iter().map(|o| o.fingerprint_hex.as_str()).collect();
    fps.sort_unstable();
    fps.dedup();
    match fps.as_slice() {
        [] => Err("no storm-detected line in the harvest window".to_owned()),
        [one] => Ok((*one).to_owned()),
        many => Err(format!("the storm carried {} distinct fingerprints: {many:?}", many.len())),
    }
}

/// The harvest analogue of `Contains "RetryStorm"`: the storm surfaced, and Pulse named the cue.
fn retry_storm_surfaced(observed: &[StormDetected]) -> Result<(), String> {
    if observed.is_empty() {
        return Err("no storm-detected line in the harvest window".to_owned());
    }
    match observed.iter().find(|o| o.cue_kind != RETRY_STORM_CUE) {
        Some(o) => Err(format!("a storm line named cue_kind {:?}, not the retry storm", o.cue_kind)),
        None => Ok(()),
    }
}

/// The harvest analogue of `Absent "RetryStorm"`: sub-floor fingerprints never aggregate, so NO storm
/// line exists in the window. Unlike the read-back `Absent` this replaces, the absence is meaningful —
/// a merged fingerprint would have crossed the floor and written a line here.
fn no_storm_surfaced(observed: &[StormDetected]) -> Result<(), String> {
    match observed.first() {
        None => Ok(()),
        Some(o) => Err(format!(
            "sub-floor fingerprints aggregated into a storm: {} at occurrence_count {}",
            o.fingerprint_hex, o.occurrence_count
        )),
    }
}

/// P-018 tier ladder: `suggested` fires, then `autonomous` at or past the Autonomous threshold.
fn tier_ladder_reached_autonomous(observed: &[StormDetected]) -> Result<(), String> {
    const AUTONOMOUS_THRESHOLD: u64 = 10;
    if !observed.iter().any(|o| o.severity_hint == "suggested") {
        return Err("no suggested cue in the harvest window".to_owned());
    }
    let auto = observed
        .iter()
        .find(|o| o.severity_hint == "autonomous")
        .ok_or_else(|| "no autonomous cue in the harvest window".to_owned())?;
    if auto.occurrence_count < AUTONOMOUS_THRESHOLD {
        return Err(format!(
            "autonomous fired at occurrence_count {}, below the {AUTONOMOUS_THRESHOLD} threshold",
            auto.occurrence_count
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verbatim shape from a real capture
    /// (`2026-08-15-canary-spans-pulse-fingerprints/evidence/trio-and-storm.jsonl`) — the parser is
    /// pinned to Pulse's ACTUAL line, not to an invented one.
    fn detected_line(fp: &str, count: u64, hint: &str) -> String {
        format!(
            r#"{{"fields":{{"cue_kind":"retry_storm","deployment.environment":"production","fingerprint_hex":"{fp}","occurrence_count":{count},"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity_hint":"{hint}","window_seconds":30}},"level":"INFO","message":"","target":"triage.pattern.storm.detected","timestamp":"2026-08-16T08:18:33.916Z"}}"#
        )
    }

    fn other_line() -> String {
        r#"{"fields":{"rows_ingested":15},"level":"INFO","message":"","target":"buffer.tick","timestamp":"2026-08-16T08:18:30.000Z"}"#.to_owned()
    }

    fn storm_harvest() -> Vec<String> {
        vec![
            other_line(),
            detected_line("6074a716", 5, "suggested"),
            other_line(),
            detected_line("6074a716", 10, "autonomous"),
        ]
    }

    #[test]
    fn the_parser_reads_pulses_real_storm_line_shape() {
        let observed = parse_storm_detected(&storm_harvest());
        assert_eq!(
            observed,
            vec![
                StormDetected {
                    cue_kind: "retry_storm".to_owned(),
                    severity_hint: "suggested".to_owned(),
                    occurrence_count: 5,
                    fingerprint_hex: "6074a716".to_owned(),
                },
                StormDetected {
                    cue_kind: "retry_storm".to_owned(),
                    severity_hint: "autonomous".to_owned(),
                    occurrence_count: 10,
                    fingerprint_hex: "6074a716".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn a_single_fingerprint_storm_satisfies_p017() {
        assert_eq!(single_fingerprint(&parse_storm_detected(&storm_harvest())), Ok("6074a716".to_owned()));
    }

    /// The defect the scenario re-shape exists to prevent: a mixed-variant storm splitting across two
    /// fingerprints must FAIL P-017, not silently pass on whichever fired first.
    #[test]
    fn two_fingerprints_fail_p017() {
        let split = vec![
            detected_line("6074a716", 5, "suggested"),
            detected_line("ff001122", 5, "suggested"),
        ];
        let err = single_fingerprint(&parse_storm_detected(&split)).unwrap_err();
        assert!(err.contains("2 distinct fingerprints"), "{err}");
    }

    #[test]
    fn the_tier_ladder_reaches_autonomous_past_the_threshold() {
        assert_eq!(tier_ladder_reached_autonomous(&parse_storm_detected(&storm_harvest())), Ok(()));
    }

    #[test]
    fn a_suggested_only_storm_fails_the_tier_ladder() {
        let only = vec![detected_line("6074a716", 6, "suggested")];
        let err = tier_ladder_reached_autonomous(&parse_storm_detected(&only)).unwrap_err();
        assert!(err.contains("no autonomous cue"), "{err}");
    }

    /// `fingerprint-distinct`'s guard: three sub-floor fingerprints raise no cue at all, so the
    /// harvest window is empty of storm lines and P-017's distinctness half holds.
    #[test]
    fn a_sub_floor_harvest_carries_no_storm_line() {
        let quiet = vec![other_line(), other_line()];
        let observed = parse_storm_detected(&quiet);
        assert!(observed.is_empty());
        assert_eq!(no_storm_surfaced(&observed), Ok(()));
        assert!(single_fingerprint(&observed).unwrap_err().contains("no storm-detected line"));
    }

    /// The harvest analogue of the retired `Contains "RetryStorm"`: Pulse names the cue on its own
    /// line, which is gradeable where the degraded read-back report is not.
    #[test]
    fn the_storm_surfaces_a_retry_storm_cue() {
        assert_eq!(retry_storm_surfaced(&parse_storm_detected(&storm_harvest())), Ok(()));
    }

    #[test]
    fn an_empty_window_fails_the_retry_storm_surface_check() {
        let err = retry_storm_surfaced(&parse_storm_detected(&[other_line()])).unwrap_err();
        assert!(err.contains("no storm-detected line"), "{err}");
    }

    /// The retired read-back `Absent` passed vacuously on text that could never carry the token; the
    /// harvest analogue actually fails when sub-floor fingerprints aggregate.
    #[test]
    fn an_aggregating_sub_floor_window_fails_the_absence_check() {
        let merged = vec![detected_line("6074a716", 12, "autonomous")];
        let err = no_storm_surfaced(&parse_storm_detected(&merged)).unwrap_err();
        assert!(err.contains("aggregated into a storm"), "{err}");
    }

    /// A harvest is another process's output: a truncated or non-JSON tail is skipped, never fatal.
    #[test]
    fn a_malformed_line_is_skipped_not_fatal() {
        let ragged = vec![
            "not json at all".to_owned(),
            r#"{"target":"triage.pattern.storm.detected"}"#.to_owned(),
            detected_line("6074a716", 10, "autonomous"),
        ];
        assert_eq!(parse_storm_detected(&ragged).len(), 1);
    }

    #[test]
    fn the_window_slice_drops_pre_leg_lines() {
        let dir = assert_fs::TempDir::new().unwrap();
        let path = dir.path().join("agent-latest.jsonl.2026-08-16");
        let body = format!("{}\n{}\n", other_line(), detected_line("6074a716", 10, "autonomous"));
        std::fs::write(&path, body).unwrap();

        assert_eq!(parse_storm_detected(&harvest_since(&path, 0).unwrap()).len(), 1);
        assert!(parse_storm_detected(&harvest_since(&path, 2).unwrap()).is_empty());
    }
}
