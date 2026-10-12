//! `conductor scenarios` — list the scenario catalogue (name · P-IDs · SLO tier); runs nothing.

use std::process::ExitCode;

use crate::paths::Paths;
use crate::render;

pub fn scenarios(paths: &Paths) -> anyhow::Result<ExitCode> {
    let summaries = paths.list_scenarios()?;
    if summaries.is_empty() {
        println!("No scenarios found.");
        return Ok(ExitCode::SUCCESS);
    }
    println!("{}", render::scenarios_table(&summaries));
    println!("{}", render::scenarios_rollup(summaries.len()));
    Ok(ExitCode::SUCCESS)
}
