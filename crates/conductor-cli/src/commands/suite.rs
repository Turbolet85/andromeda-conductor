//! `conductor suite [--filter] [--seed]` — run the catalog over one runtime + one preflight gate.

use std::process::ExitCode;

use crate::paths::Paths;
use crate::pause::CliResolver;
use crate::pipeline;
use crate::render;

use super::{exit_code, persist};

pub async fn suite(
    filter: Option<&str>,
    seed: Option<u64>,
    paths: &Paths,
    run_id: &str,
) -> anyhow::Result<ExitCode> {
    let scenarios = paths.load_all_scenarios(filter, seed)?;
    if scenarios.is_empty() {
        anyhow::bail!("no scenarios found to run");
    }

    let preflight = pipeline::preflight(&paths.manifest_path).await?;
    let progress = render::spinner(scenarios.len());
    let resolver = CliResolver::select(Some(progress.clone()));
    let mut records = Vec::with_capacity(scenarios.len());
    for scenario in &scenarios {
        let record = pipeline::execute_scenario(&preflight, scenario, run_id, &resolver).await?;
        progress.inc(1);
        records.push(record);
    }
    progress.finish_and_clear();

    persist(&paths.runs_dir, run_id, &records)?;
    println!("{}", render::results_table(&records));
    Ok(exit_code(&records))
}
