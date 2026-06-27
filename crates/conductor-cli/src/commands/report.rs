//! `conductor report [<run_id>]` — render a persisted run's report from its JSONL journal.

use std::process::ExitCode;

use anyhow::Context as _;

use crate::paths::Paths;
use crate::render;

pub fn report(run_id: Option<&str>, paths: &Paths) -> anyhow::Result<ExitCode> {
    let run_id = match run_id {
        Some(id) => id.to_string(),
        None => conductor_core::latest_run_id(&paths.runs_dir)?.context("no runs found to report")?,
    };
    let records = conductor_core::read_run_journal(&paths.runs_dir, &run_id)?;
    println!("Run report {}", render::paint(&run_id, render::ID_CYAN));
    println!();
    println!("{}", render::results_table(&records));
    Ok(ExitCode::SUCCESS)
}
