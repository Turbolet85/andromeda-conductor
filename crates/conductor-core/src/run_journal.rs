//! Read-only run-journal accessor — the persisted `runs/<run_id>.jsonl` ground truth, shared by the
//! CLI `report` verb and the Tauri run-report view (DRY; the IO-in-core sibling of
//! [`crate::scenario_files`]). Listing + parsing only; a malformed line or unreadable journal is a
//! harness fault ([`CoreError::Config`] → `Err`), never a verification outcome (the verdict/error wall).

use std::path::Path;

use crate::{CoreError, RunRecord};

/// The newest run's id — the lexicographically-greatest `<run_id>.jsonl` stem (run_ids sort by time).
/// An unreadable / absent runs directory yields `None` (no runs yet), not an error.
pub fn latest_run_id(runs_dir: &Path) -> crate::Result<Option<String>> {
    let mut latest: Option<String> = None;
    let Ok(entries) = std::fs::read_dir(runs_dir) else {
        return Ok(None);
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|x| x.to_str()) != Some("jsonl") {
            continue;
        }
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            && latest.as_deref().is_none_or(|current| stem > current)
        {
            latest = Some(stem.to_string());
        }
    }
    Ok(latest)
}

/// Read + parse every record in `runs/<run_id>.jsonl` (blank lines skipped) into the run-report
/// envelope, in journal order. A read or per-line parse failure is a harness fault.
pub fn read_run_journal(runs_dir: &Path, run_id: &str) -> crate::Result<Vec<RunRecord>> {
    let path = runs_dir.join(format!("{run_id}.jsonl"));
    let text = std::fs::read_to_string(&path)
        .map_err(|e| CoreError::Config(format!("read run journal: {e}")))?;
    let mut records = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let record = serde_json::from_str::<RunRecord>(line)
            .map_err(|e| CoreError::Config(format!("parse journal line: {e}")))?;
        records.push(record);
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::*;
    use crate::{PId, ReportState, SloTier, Verdict};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, process-and-thread-unique temp directory (conductor-core has no `assert_fs` dev-dep;
    /// the `std::process::id()` + atomic suffix keeps parallel nextest workers from colliding).
    fn temp_runs_dir() -> PathBuf {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("conductor-core-run-journal-{}-{n}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn measured(run_id: &str) -> RunRecord {
        RunRecord::measured(
            run_id,
            424242,
            "error-baseline-spike",
            vec![PId("P-009".to_string())],
            Verdict::Pass,
            ReportState::Pass,
            "2026-06-16T21:10:06Z",
            "2026-06-16T21:10:07Z",
            1840,
            SloTier::Tier5s,
            vec!["fp-1".to_string()],
        )
    }

    fn write_journal(dir: &Path, run_id: &str, body: &str) {
        std::fs::write(dir.join(format!("{run_id}.jsonl")), body).unwrap();
    }

    #[test]
    fn latest_run_id_is_none_for_absent_dir() {
        let dir = temp_runs_dir().join("does-not-exist");
        assert_eq!(latest_run_id(&dir).unwrap(), None);
    }

    #[test]
    fn latest_run_id_picks_the_greatest_jsonl_stem_ignoring_non_journals() {
        let dir = temp_runs_dir();
        for stem in ["2026-06-16T20-00-00-a", "2026-06-16T21-00-00-b", "2026-06-16T19-00-00-c"] {
            write_journal(&dir, stem, &serde_json::to_string(&measured(stem)).unwrap());
        }
        std::fs::write(dir.join("runs.db"), "not a journal").unwrap();
        assert_eq!(latest_run_id(&dir).unwrap().as_deref(), Some("2026-06-16T21-00-00-b"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn read_run_journal_parses_records_in_order_and_skips_blank_lines() {
        let dir = temp_runs_dir();
        let records = vec![
            measured("2026-06-16T21-10-06-r"),
            RunRecord::blocked(
                "2026-06-16T21-10-06-r",
                7,
                "port-occupier",
                vec![PId("P-003".to_string())],
                SloTier::Tier20s,
            ),
        ];
        let body = format!(
            "{}\n\n{}\n",
            serde_json::to_string(&records[0]).unwrap(),
            serde_json::to_string(&records[1]).unwrap()
        );
        write_journal(&dir, "2026-06-16T21-10-06-r", &body);
        assert_eq!(read_run_journal(&dir, "2026-06-16T21-10-06-r").unwrap(), records);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn read_run_journal_missing_file_is_a_harness_fault() {
        let dir = temp_runs_dir();
        assert!(matches!(read_run_journal(&dir, "no-such-run"), Err(CoreError::Config(_))));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn read_run_journal_malformed_line_is_a_harness_fault() {
        let dir = temp_runs_dir();
        write_journal(&dir, "bad", "{not valid json}");
        assert!(matches!(read_run_journal(&dir, "bad"), Err(CoreError::Config(_))));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
