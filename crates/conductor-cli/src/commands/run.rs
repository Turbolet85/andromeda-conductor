//! `conductor run <scenario|P-ID> [--seed]` — drive one scenario, persist its envelope.

use std::process::ExitCode;

use conductor_run as pipeline;

use crate::paths::Paths;
use crate::pause::CliResolver;

use super::{exit_code, persist, print_record};

pub async fn run(
    target: &str,
    seed: Option<u64>,
    paths: &Paths,
    run_id: &str,
    agent_mode: bool,
) -> anyhow::Result<ExitCode> {
    let scenario = paths.load_scenario(target, seed)?;
    let preflight = pipeline::preflight(&paths.manifest_path).await?;
    let resolver = CliResolver::select(None, agent_mode);
    let records = [pipeline::execute_scenario(&preflight, &scenario, run_id, &resolver).await?];

    persist(&paths.runs_dir, run_id, &records)?;
    print_record(&records[0]);
    Ok(exit_code(&records))
}
