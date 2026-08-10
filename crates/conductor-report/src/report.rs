//! The per-run Markdown run report.
//!
//! Renders a run's [`RunRecord`]s into a deterministic Markdown document at `runs/<run_id>.md` — the
//! human-facing sibling of the JSONL [`JournalWriter`](crate::JournalWriter) and the `runs.db`
//! [`RunsDb`](crate::RunsDb), over the same envelope shape (arch §Standard Contracts). Each check
//! renders its verdict-first [`Lamp`], and a blocked row shows identity + SLO tier only — the five
//! never-measured fields collapse to an em-dash, never `null` (the blocked-row null rule). The render
//! is a pure function of its inputs (no clock read), so the artifact is deterministic; a write fault
//! is a harness fault ([`ReportError`] → `Err`), never a verification verdict (the verdict/error
//! wall). `CONDUCTOR_RUNS_DIR` resolution happens at the cli edge — the writer is handed an
//! already-resolved `runs_dir`.

use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};

use conductor_core::{EnvelopeStatus, Lamp, RunRecord, Verdict};

/// The render of a measurement field that a blocked row never carries (the blocked-row null rule).
const ABSENT: &str = "—";

/// A harness fault from the run-report writer — never a verification outcome (the verdict/error
/// wall). `#[non_exhaustive]` so later report-seam chunks extend the fault surface.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReportError {
    /// Creating the runs directory or writing the report file failed (incl. a pre-existing report —
    /// `<run_id>.md` is opened `create_new`, so a repeat run_id is a loud `Err`, never a clobber).
    #[error("run report io error: {0}")]
    Io(#[from] std::io::Error),
}

/// The per-run Markdown run report.
pub struct RunReport;

impl RunReport {
    /// Render the run's `records` to `<runs_dir>/<run_id>.md`, returning the written path.
    ///
    /// `runs_dir` is the already-resolved artifact directory (the cli edge applies
    /// `CONDUCTOR_RUNS_DIR`). The file is opened `create_new`, so a pre-existing `<run_id>.md` is an
    /// `Err`, never a silent clobber (run_id-stemmed, never overwritten).
    pub fn write(
        runs_dir: &Path,
        run_id: &str,
        records: &[RunRecord],
        envelope: &EnvelopeStatus,
    ) -> Result<PathBuf, ReportError> {
        fs::create_dir_all(runs_dir)?;
        let path = runs_dir.join(format!("{run_id}.md"));
        let mut file = OpenOptions::new().write(true).create_new(true).open(&path)?;
        file.write_all(Self::render(run_id, records, envelope).as_bytes())?;
        Ok(path)
    }

    /// Render the run report to a Markdown string — a pure function of its inputs (no clock, no IO),
    /// so the artifact is byte-identical for a fixed record set.
    pub fn render(run_id: &str, records: &[RunRecord], envelope: &EnvelopeStatus) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "# Run report `{run_id}`");
        let _ = writeln!(out);
        if let Some(banner) = envelope_banner(envelope) {
            let _ = writeln!(out, "{banner}");
            let _ = writeln!(out);
        }
        let _ = writeln!(out, "{}", summary_line(records));
        for rec in records {
            let _ = write!(out, "\n{}", scenario_block(rec));
        }
        out
    }
}

/// The run-level envelope banner, or `None` for an in-envelope run (which needs no caveat).
///
/// Markdown has no color channel, so the signal is carried by the always-rendered label plus
/// emphasis — the color-free counterpart of the CLI's ASCII prefix (design-system §Iconography).
/// It sits above the check tally because it qualifies every row beneath it: it is a statement about
/// whether this run could be evidence at all, not a verdict on any check.
fn envelope_banner(envelope: &EnvelopeStatus) -> Option<String> {
    let cause = envelope.cause()?;
    Some(format!("> **[{}]** {cause}", envelope.label()))
}

/// The run-level summary: the shared seed (when uniform), the check count, and the per-lamp tally.
fn summary_line(records: &[RunRecord]) -> String {
    let mut line = String::new();
    if let Some(seed) = uniform_seed(records) {
        let _ = write!(line, "**Seed** `{seed}` · ");
    }
    let _ = write!(line, "**Checks** {}", records.len());
    for (lamp, n) in lamp_counts(records) {
        if n > 0 {
            let _ = write!(line, " · {n} {}", lamp.status_prefix());
        }
    }
    line
}

/// One per-scenario detail section, led by the verdict-first lamp; measurement fields collapse to
/// `ABSENT` for a blocked row.
fn scenario_block(rec: &RunRecord) -> String {
    let lamp = Lamp::for_record(rec);
    let p_ids = rec.p_ids.iter().map(|p| format!("`{}`", p.0)).collect::<Vec<_>>().join(", ");
    let mut block = String::new();
    let _ = writeln!(block, "## {} {}", lamp.status_prefix(), rec.scenario);
    let _ = writeln!(block, "- **P-IDs** {p_ids}");
    let _ = writeln!(block, "- **Verdict** {} · **State** {}", opt_verdict(rec.verdict), wire(serde_json::to_value(rec.state)));
    let _ = writeln!(block, "- **Latency** {} · **SLO tier** `{}`", latency(rec.latency_ms), wire(serde_json::to_value(rec.slo_tier)));
    let _ = writeln!(block, "- **Emitted** {} · **Observed** {}", instant(&rec.journal_emitted_at), instant(&rec.read_back_observed_at));
    let _ = writeln!(block, "- **Fingerprints** {}", fingerprints(&rec.fingerprints));
    block
}

/// The run's seed when every record shares it (the run invariant), else `None` (omitted).
fn uniform_seed(records: &[RunRecord]) -> Option<u64> {
    let (first, rest) = records.split_first()?;
    rest.iter().all(|r| r.seed == first.seed).then_some(first.seed)
}

/// The per-lamp tally in fixed render order (deterministic — no map iteration).
fn lamp_counts(records: &[RunRecord]) -> [(Lamp, usize); 6] {
    let mut counts = [
        (Lamp::Pass, 0usize),
        (Lamp::Fail, 0),
        (Lamp::Hold, 0),
        (Lamp::Manual, 0),
        (Lamp::Residual, 0),
        (Lamp::Blocked, 0),
    ];
    for rec in records {
        let lamp = Lamp::for_record(rec);
        for slot in &mut counts {
            if slot.0 == lamp {
                slot.1 += 1;
                break;
            }
        }
    }
    counts
}

/// A verdict's canonical wire spelling, or `ABSENT` when the row carries none (blocked).
fn opt_verdict(verdict: Option<Verdict>) -> String {
    match verdict {
        Some(v) => wire(serde_json::to_value(v)),
        None => ABSENT.to_string(),
    }
}

/// Latency in milliseconds, or `ABSENT` for a never-measured row.
fn latency(ms: Option<i64>) -> String {
    ms.map_or_else(|| ABSENT.to_string(), |n| format!("{n} ms"))
}

/// An RFC-3339 instant in an inline-code span, or `ABSENT` when absent.
fn instant(value: &Option<String>) -> String {
    match value {
        Some(s) => format!("`{s}`"),
        None => ABSENT.to_string(),
    }
}

/// The fingerprint list: `ABSENT` for a never-measured row (`None`), `(none)` for a measured row
/// with no fingerprints (`Some([])`), else the back-ticked fingerprints.
fn fingerprints(fps: &Option<Vec<String>>) -> String {
    match fps {
        None => ABSENT.to_string(),
        Some(v) if v.is_empty() => "(none)".to_string(),
        Some(v) => v.iter().map(|f| format!("`{f}`")).collect::<Vec<_>>().join(", "),
    }
}

/// Take an enum's bare serde wire string (e.g. `SloTier` → `<5s`) so the report spelling stays
/// identical to the JSONL / runs.db wire form (the `#[serde(rename)]` is the single source of truth).
fn wire(value: serde_json::Result<serde_json::Value>) -> String {
    match value {
        Ok(serde_json::Value::String(s)) => s,
        Ok(other) => other.to_string(),
        Err(_) => ABSENT.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::TempDir;
    use conductor_core::{PId, ReportState, SloTier, Verdict};

    /// The default run standing — most render assertions are about check rows, not the envelope.
    const OK_ENV: EnvelopeStatus = EnvelopeStatus::InEnvelope;

    fn suspect() -> EnvelopeStatus {
        EnvelopeStatus::EnvironmentSuspect(
            "scenario \"activity-floor\" runs 3900s, over the proven-good envelope ceiling of 600s"
                .to_string(),
        )
    }

    fn measured(scenario: &str, verdict: Verdict, state: ReportState) -> RunRecord {
        RunRecord::measured(
            "2026-06-16T21-10-06-abc",
            424242,
            scenario,
            vec![PId("P-009".to_string()), PId("P-010".to_string())],
            verdict,
            state,
            "2026-06-16T21:10:06Z",
            "2026-06-16T21:10:07Z",
            1840,
            SloTier::Tier5s,
            vec!["fp-1".to_string()],
        )
    }

    fn blocked(scenario: &str) -> RunRecord {
        RunRecord::blocked(
            "2026-06-16T21-10-06-abc",
            424242,
            scenario,
            vec![PId("P-003".to_string())],
            SloTier::Tier20s,
        )
    }

    #[test]
    fn calibration_renders_hold_not_manual() {
        let md = RunReport::render(
            "R",
            &[measured("severity-choice", Verdict::CalibrationRegion, ReportState::ManualCheck)], &OK_ENV,
        );
        assert!(md.contains("## [HOLD] severity-choice"), "{md}");
        assert!(!md.contains("[MANUAL]"), "{md}");
    }

    #[test]
    fn known_residual_renders_residual_even_with_a_verdict() {
        let md = RunReport::render(
            "R",
            &[measured("degraded-report", Verdict::Fail, ReportState::KnownResidual)], &OK_ENV,
        );
        assert!(md.contains("## [RESIDUAL] degraded-report"), "{md}");
        assert!(!md.contains("[FAIL]"), "{md}");
    }

    #[test]
    fn pass_and_fail_render_their_prefixes() {
        let md = RunReport::render(
            "R",
            &[
                measured("ok", Verdict::Pass, ReportState::Pass),
                measured("bad", Verdict::Fail, ReportState::Fail),
            ], &OK_ENV,
        );
        assert!(md.contains("## [PASS] ok"), "{md}");
        assert!(md.contains("## [FAIL] bad"), "{md}");
    }

    #[test]
    fn blocked_row_shows_identity_and_tier_with_measurement_fields_em_dashed() {
        let md = RunReport::render("R", &[blocked("port-occupier")], &OK_ENV);
        assert!(md.contains("## [BLOCKED] port-occupier"), "{md}");
        // identity + slo_tier + state stay populated
        assert!(md.contains("`P-003`"), "{md}");
        assert!(md.contains("`<20s`"), "{md}");
        assert!(md.contains("**State** Blocked"), "{md}");
        // the five never-measured fields collapse to the em-dash, never the literal `null`
        assert!(md.contains("**Verdict** —"), "{md}");
        assert!(md.contains("**Latency** —"), "{md}");
        assert!(md.contains("**Emitted** — · **Observed** —"), "{md}");
        assert!(md.contains("**Fingerprints** —"), "{md}");
        assert!(!md.to_lowercase().contains("null"), "{md}");
    }

    #[test]
    fn measured_empty_fingerprints_distinct_from_blocked() {
        let rec = RunRecord::measured(
            "no-fp",
            1,
            "no-fp",
            vec![PId("P-001".to_string())],
            Verdict::Pass,
            ReportState::Pass,
            "2026-06-16T21:10:06Z",
            "2026-06-16T21:10:07Z",
            10,
            SloTier::Tier5s,
            vec![],
        );
        let md = RunReport::render("R", &[rec], &OK_ENV);
        assert!(md.contains("**Fingerprints** (none)"), "{md}");
    }

    #[test]
    fn summary_line_counts_lamps_and_shows_uniform_seed() {
        let md = RunReport::render(
            "R",
            &[
                measured("ok", Verdict::Pass, ReportState::Pass),
                measured("hold", Verdict::CalibrationRegion, ReportState::ManualCheck),
                blocked("blk"),
            ], &OK_ENV,
        );
        assert!(md.contains("**Checks** 3"), "{md}");
        assert!(md.contains("1 [PASS]"), "{md}");
        assert!(md.contains("1 [HOLD]"), "{md}");
        assert!(md.contains("1 [BLOCKED]"), "{md}");
        assert!(md.contains("**Seed** `424242`"), "{md}");
    }

    #[test]
    fn an_in_envelope_run_renders_no_banner() {
        let md = RunReport::render("R", &[measured("ok", Verdict::Pass, ReportState::Pass)], &OK_ENV);
        assert!(!md.contains("ENVIRONMENT-SUSPECT"), "an in-envelope run needs no caveat: {md}");
        assert!(!md.contains("IN-ENVELOPE"), "nor a redundant all-clear: {md}");
    }

    #[test]
    fn a_suspect_run_banners_the_breach_above_the_tally() {
        let md = RunReport::render("R", &[measured("ok", Verdict::Pass, ReportState::Pass)], &suspect());
        assert!(md.contains("[ENVIRONMENT-SUSPECT]"), "the label carries the signal: {md}");
        assert!(md.contains("activity-floor"), "the cause names the breaching scenario: {md}");
        assert!(md.contains("3900s"), "the cause names the realized duration: {md}");
        assert!(
            md.find("ENVIRONMENT-SUSPECT") < md.find("**Checks**"),
            "the banner qualifies the rows beneath it, so it precedes the tally: {md}"
        );
    }

    /// The qualifier is orthogonal to the closed `ReportState` set — a suspect run's check rows keep
    /// their own states, and nothing renders as Fail on account of the envelope.
    #[test]
    fn the_banner_does_not_alter_any_check_state() {
        let records = [
            measured("ok", Verdict::Pass, ReportState::Pass),
            blocked("blk"),
        ];
        let plain = RunReport::render("R", &records, &OK_ENV);
        let flagged = RunReport::render("R", &records, &suspect());

        assert!(flagged.contains("## [PASS] ok"), "{flagged}");
        assert!(flagged.contains("## [BLOCKED] blk"), "{flagged}");
        assert!(!flagged.contains("[FAIL]"), "an envelope breach is never a Fail: {flagged}");
        for line in plain.lines().filter(|l| l.starts_with("## ")) {
            assert!(flagged.contains(line), "check row changed under the banner: {line}");
        }
    }

    #[test]
    fn the_banner_leaks_no_host_path_or_struct_name() {
        let md = RunReport::render("R", &[blocked("blk")], &suspect());
        for leak in ["C:\\", "/Users/", "/home/", "EnvelopeStatus", "LoadEnvelope", "RunRecord"] {
            assert!(!md.contains(leak), "leaked {leak:?}: {md}");
        }
    }

    #[test]
    fn render_is_deterministic() {
        let records =
            [measured("ok", Verdict::Pass, ReportState::Pass), blocked("blk")];
        assert_eq!(RunReport::render("R", &records, &OK_ENV), RunReport::render("R", &records, &OK_ENV));
    }

    #[test]
    fn no_host_paths_or_struct_names_leak() {
        let md = RunReport::render(
            "2026-06-16T21-10-06-abc",
            &[measured("ok", Verdict::Pass, ReportState::Pass), blocked("blk")], &OK_ENV,
        );
        for leak in
            ["C:\\", "/Users/", "/home/", "RunRecord", "RunReport", "Lamp", "ReportState", "RunsDb"]
        {
            assert!(!md.contains(leak), "leaked {leak:?}: {md}");
        }
    }

    #[test]
    fn write_creates_run_id_stemmed_file_matching_render() {
        let dir = TempDir::new().unwrap();
        let run_id = "2026-06-16T21-10-06-abc";
        let records = [measured("ok", Verdict::Pass, ReportState::Pass)];
        let path = RunReport::write(dir.path(), run_id, &records, &OK_ENV).unwrap();
        assert_eq!(path, dir.path().join("2026-06-16T21-10-06-abc.md"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), RunReport::render(run_id, &records, &OK_ENV));
    }

    #[test]
    fn write_never_overwrites_an_existing_report() {
        let dir = TempDir::new().unwrap();
        let run_id = "2026-06-16T21-10-06-abc";
        let records = [measured("ok", Verdict::Pass, ReportState::Pass)];
        RunReport::write(dir.path(), run_id, &records, &OK_ENV).unwrap();
        // a second write for the same run_id is a loud Err (create_new), never a clobber
        assert!(matches!(
            RunReport::write(dir.path(), run_id, &records, &OK_ENV),
            Err(ReportError::Io(_))
        ));
    }
}
