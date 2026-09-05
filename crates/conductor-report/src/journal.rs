//! The per-run JSONL emission-journal writer.
//!
//! [`JournalWriter`] appends one [`RunRecord`] per scenario check to `runs/<run_id>.jsonl` — the
//! append-mostly, agent-parseable ground truth the journal-relative SLO math reads (arch
//! §Journal-relative ground truth). The file is run_id-stemmed and never truncated; a write fault is
//! a harness fault ([`JournalError`] → `Err`), never a verification verdict (the verdict/error
//! wall). `CONDUCTOR_RUNS_DIR` resolution happens at the cli edge — the writer is handed an
//! already-resolved `runs_dir`.

use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use conductor_core::{CheckRecord, RunRecord};

/// A harness fault from the journal writer — never a verification outcome (the verdict/error wall).
/// `#[non_exhaustive]` so later report-seam chunks extend the fault surface.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum JournalError {
    /// Opening, writing, or flushing the journal file failed.
    #[error("journal io error: {0}")]
    Io(#[from] std::io::Error),
    /// Serializing a record to JSON failed.
    #[error("journal serialize error: {0}")]
    Serialize(#[from] serde_json::Error),
}

/// Appends [`RunRecord`]s to a single run's `<runs_dir>/<run_id>.jsonl`, one JSON object per line.
///
/// Opens the file create+append — never truncating a prior run's journal (the millisecond-precise
/// `run_id` already keys a fresh file per run) — and holds it open for the run. [`append`] flushes
/// each line so a concurrent reader (the `status` command) always sees a durable, line-complete
/// journal; throughput is an explicit non-goal.
///
/// [`append`]: JournalWriter::append
pub struct JournalWriter {
    writer: BufWriter<File>,
}

impl JournalWriter {
    /// Open the journal for `run_id` under `runs_dir` (created if absent), ready to append.
    ///
    /// `runs_dir` is the already-resolved artifact directory (the cli edge applies
    /// `CONDUCTOR_RUNS_DIR` + `resolve_under`).
    pub fn create(runs_dir: &Path, run_id: &str) -> Result<Self, JournalError> {
        fs::create_dir_all(runs_dir)?;
        let path = runs_dir.join(format!("{run_id}.jsonl"));
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    /// Append one `record` as a single JSON line, then flush.
    pub fn append(&mut self, record: &RunRecord) -> Result<(), JournalError> {
        serde_json::to_writer(&mut self.writer, record)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }

    /// Append one per-check record as a single JSON line, then flush.
    ///
    /// A distinct record shape from the eleven-field envelope, on the same journal: the envelope
    /// rows carry the scenario grain, these carry the check grain behind them. Only measured checks
    /// are appended — a blocked or declare-only scenario contributes none.
    pub fn append_check(&mut self, check: &CheckRecord) -> Result<(), JournalError> {
        serde_json::to_writer(&mut self.writer, check)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::TempDir;
    use conductor_core::{PId, ReportState, SloTier, Verdict};

    fn measured(run_id: &str) -> RunRecord {
        RunRecord {
            journal_emitted_at: Some("2026-06-16T21:10:06Z".to_string()),
            read_back_observed_at: Some("2026-06-16T21:10:07Z".to_string()),
            run_id: run_id.to_string(),
            seed: 424242,
            scenario: "error-baseline-spike".to_string(),
            p_ids: vec![PId("P-009".to_string())],
            verdict: Some(Verdict::Pass),
            state: ReportState::Pass,
            latency_ms: Some(1840),
            slo_tier: SloTier::Tier5s,
            fingerprints: Some(vec!["fp-1".to_string()]),
        }
    }

    fn read_lines(dir: &TempDir, run_id: &str) -> Vec<String> {
        let path = dir.path().join(format!("{run_id}.jsonl"));
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn appends_run_id_stemmed_parseable_jsonl() {
        let dir = TempDir::new().unwrap();
        let run_id = "2026-06-16T21-10-06-abc";
        let mut w = JournalWriter::create(dir.path(), run_id).unwrap();
        w.append(&measured(run_id)).unwrap();

        assert!(dir.path().join("2026-06-16T21-10-06-abc.jsonl").exists());
        let lines = read_lines(&dir, run_id);
        assert_eq!(lines.len(), 1);
        // each line is a complete, parseable JSON object (agent-parseable ground truth)
        let v: serde_json::Value = serde_json::from_str(&lines[0]).unwrap();
        assert_eq!(v["run_id"], serde_json::json!(run_id));
    }

    #[test]
    fn append_never_overwrites_a_prior_record() {
        let dir = TempDir::new().unwrap();
        let run_id = "2026-06-16T21-10-06-keep";
        {
            let mut w = JournalWriter::create(dir.path(), run_id).unwrap();
            w.append(&measured(run_id)).unwrap();
        }
        // re-open the same run's journal and append a second record
        {
            let mut w = JournalWriter::create(dir.path(), run_id).unwrap();
            let mut second = measured(run_id);
            second.scenario = "second-scenario".to_string();
            w.append(&second).unwrap();
        }
        let lines = read_lines(&dir, run_id);
        assert_eq!(lines.len(), 2, "re-open appended, did not truncate");
        let first: serde_json::Value = serde_json::from_str(&lines[0]).unwrap();
        let snd: serde_json::Value = serde_json::from_str(&lines[1]).unwrap();
        assert_eq!(first["scenario"], serde_json::json!("error-baseline-spike"));
        assert_eq!(snd["scenario"], serde_json::json!("second-scenario"));
    }

    #[test]
    fn line_carries_only_owned_schema_keys_no_host_paths() {
        let dir = TempDir::new().unwrap();
        let run_id = "2026-06-16T21-10-06-hyg";
        let mut w = JournalWriter::create(dir.path(), run_id).unwrap();
        w.append(&measured(run_id)).unwrap();
        let line = read_lines(&dir, run_id).remove(0);

        // no absolute host path / internal struct name leaks into the line
        assert!(!line.contains("C:\\"), "{line}");
        assert!(!line.contains("/Users/"), "{line}");
        assert!(!line.contains("RunRecord"), "{line}");
        // keys are exactly the eleven owned-schema names (a stray Debug-dumped field would fail here)
        let obj: serde_json::Value = serde_json::from_str(&line).unwrap();
        let mut keys: Vec<&str> = obj
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "fingerprints",
                "journal_emitted_at",
                "latency_ms",
                "p_ids",
                "read_back_observed_at",
                "run_id",
                "scenario",
                "seed",
                "slo_tier",
                "state",
                "verdict",
            ]
        );
    }

    #[test]
    fn check_line_is_its_own_parseable_shape_beside_the_envelope() {
        let dir = TempDir::new().unwrap();
        let run_id = "2026-06-16T21-10-06-chk";
        let mut w = JournalWriter::create(dir.path(), run_id).unwrap();
        w.append(&measured(run_id)).unwrap();
        w.append_check(&CheckRecord {
            run_id: run_id.to_string(),
            scenario: "error-baseline-spike".to_string(),
            check_index: 0,
            kind: conductor_core::ComparisonKind::Contains,
            verdict: Verdict::Pass,
            state: ReportState::Pass,
            latency_ms: 1840,
            deadline_ms: 2000,
            budget_ms: Some(2000),
        })
        .unwrap();

        let lines = read_lines(&dir, run_id);
        assert_eq!(
            lines.len(),
            2,
            "the check rides the same journal as its scenario row"
        );
        let env: serde_json::Value = serde_json::from_str(&lines[0]).unwrap();
        assert_eq!(
            env.as_object().unwrap().len(),
            11,
            "the envelope shape is untouched"
        );

        let chk: serde_json::Value = serde_json::from_str(&lines[1]).unwrap();
        let mut keys: Vec<&str> = chk
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "budget_ms",
                "check_index",
                "deadline_ms",
                "kind",
                "latency_ms",
                "run_id",
                "scenario",
                "state",
                "verdict",
            ]
        );
        assert_eq!(chk["kind"], serde_json::json!("Contains"));
        assert!(
            !lines[1].contains("CheckRecord"),
            "no internal struct name leaks: {}",
            lines[1]
        );
        assert!(
            !lines[1].contains("C:\\") && !lines[1].contains("/Users/"),
            "{}",
            lines[1]
        );
    }

    #[test]
    fn blocked_record_writes_null_measurement_fields() {
        let dir = TempDir::new().unwrap();
        let run_id = "2026-06-16T21-10-06-blk";
        let mut w = JournalWriter::create(dir.path(), run_id).unwrap();
        w.append(&RunRecord::blocked(
            run_id,
            7,
            "port-occupier",
            vec![PId("P-003".to_string())],
            SloTier::Tier20s,
        ))
        .unwrap();
        let line = read_lines(&dir, run_id).remove(0);
        let v: serde_json::Value = serde_json::from_str(&line).unwrap();
        assert!(v["journal_emitted_at"].is_null());
        assert!(v["read_back_observed_at"].is_null());
        assert!(v["verdict"].is_null());
        assert!(v["latency_ms"].is_null());
        assert!(v["fingerprints"].is_null());
        assert_eq!(v["state"], serde_json::json!("Blocked"));
    }
}
