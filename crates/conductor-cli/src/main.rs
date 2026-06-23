//! `conductor` — headless agent-run CLI (source of truth + release gate).
//!
//! The composition root: a `current_thread` tokio bootstrap behind the `run` / `suite` / `report`
//! verbs that wire the engine seams (timeline · emit · verify · report) end-to-end. anyhow lives only
//! at this edge; a harness fault renders as a single sanitized line (architecture §Conventions).

use std::process::ExitCode;

use clap::Parser;
use conductor_core::{init_observability, mint_run_id, sanitize_error};

mod cli;
mod commands;
mod paths;
mod pipeline;
mod render;

use cli::{Cli, Commands};

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let run_id = mint_run_id();
    let _identity = init_observability("conductor", Some(run_id.clone()));
    let cli = Cli::parse();
    match dispatch(cli, &run_id).await {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {}", sanitize_error(&*err));
            ExitCode::FAILURE
        }
    }
}

async fn dispatch(cli: Cli, run_id: &str) -> anyhow::Result<ExitCode> {
    let paths = paths::Paths::resolve()?;
    match cli.command {
        Commands::Run { target, seed } => commands::run(&target, seed, &paths, run_id).await,
        Commands::Suite { filter, seed } => commands::suite(filter.as_deref(), seed, &paths, run_id).await,
        Commands::Report { run_id: requested } => commands::report(requested.as_deref(), &paths),
        Commands::Preflight { json } => commands::preflight(json, &paths).await,
        Commands::Coverage { write } => commands::coverage(write),
    }
}
