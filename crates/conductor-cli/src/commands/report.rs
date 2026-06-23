//! `conductor report [<run_id>]` — render a persisted run's report from its JSONL journal.

use std::path::Path;
use std::process::ExitCode;

use anyhow::Context as _;
use conductor_core::RunRecord;

use crate::paths::Paths;
use crate::render;

pub fn report(run_id: Option<&str>, paths: &Paths) -> anyhow::Result<ExitCode> {
    let run_id = match run_id {
        Some(id) => id.to_string(),
        None => latest_run_id(&paths.runs_dir)?.context("no runs found to report")?,
    };
    let records = read_journal(&paths.runs_dir, &run_id)?;
    println!("Run report {}", render::paint(&run_id, render::ID_CYAN));
    println!();
    println!("{}", render::results_table(&records));
    Ok(ExitCode::SUCCESS)
}

fn read_journal(runs_dir: &Path, run_id: &str) -> anyhow::Result<Vec<RunRecord>> {
    let path = runs_dir.join(format!("{run_id}.jsonl"));
    let text = std::fs::read_to_string(&path).context("read run journal")?;
    let mut records = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        records.push(serde_json::from_str::<RunRecord>(line).context("parse journal line")?);
    }
    Ok(records)
}

/// The newest run's id — the lexicographically greatest `<run_id>.jsonl` stem (run_ids sort by time).
fn latest_run_id(runs_dir: &Path) -> anyhow::Result<Option<String>> {
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
