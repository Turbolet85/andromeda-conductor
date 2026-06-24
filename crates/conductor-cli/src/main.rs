//! `conductor` — headless agent-run CLI (source of truth + release gate).
//!
//! The composition root: a `current_thread` tokio bootstrap behind the `run` / `suite` / `report`
//! verbs that wire the engine seams (timeline · emit · verify · report) end-to-end. anyhow lives only
//! at this edge; a harness fault renders as a single sanitized line (architecture §Conventions).

use std::process::ExitCode;

use clap::Parser;
use conductor_core::{init_observability, mint_run_id, sanitize_error, ObsSink};

mod cli;
mod commands;
mod paths;
mod pause;
mod pipeline;
mod render;

use cli::{Cli, Commands};

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    // Parse before `init_observability` so `--agent-mode` selects the sink; clap exits via its own
    // (non-panicking) error path on bad args, so the panic hook still brackets all real logic.
    let cli = Cli::parse();
    let debug = cli.debug;
    let agent_mode = cli.agent_mode || std::env::var_os("CONDUCTOR_AGENT_MODE").is_some();
    let run_id = mint_run_id();
    let _identity = init_observability("conductor", Some(run_id.clone()), obs_sink(agent_mode));
    match dispatch(cli, &run_id, agent_mode).await {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", render::error_block(&sanitize_error(&*err), hint_for(&err)));
            if debug {
                // The operator opted into the full (unsanitized) chain — stderr only, never artifacts.
                eprintln!("{err:?}");
            }
            ExitCode::FAILURE
        }
    }
}

/// The self-obs sink for this invocation: `logs/agent-latest.jsonl` in agent mode (falling back to
/// stderr if the path can't be resolved), else stderr.
fn obs_sink(agent_mode: bool) -> ObsSink {
    if agent_mode {
        paths::agent_log_path().map(ObsSink::File).unwrap_or(ObsSink::Stderr)
    } else {
        ObsSink::Stderr
    }
}

/// Map a harness fault to an actionable one-line hint (design-system §cli "Error output"). Matches on
/// Conductor's own stable anyhow context markers; an unrecognized fault points at `--debug`.
fn hint_for(err: &anyhow::Error) -> &'static str {
    let chain = err.chain().map(|e| e.to_string()).collect::<Vec<_>>().join(" ");
    if chain.contains("no scenario matches") {
        "list scenarios in scenarios/, or pass a P-ID like P-009"
    } else if chain.contains("contract manifest") {
        "ensure contracts/mcp-contract.toml exists (or set CONDUCTOR_CONTRACT_MANIFEST)"
    } else if chain.contains(":4317") {
        "ensure a Pulse instance is listening on 127.0.0.1:4317"
    } else {
        "re-run with --debug for the full error chain"
    }
}

async fn dispatch(cli: Cli, run_id: &str, agent_mode: bool) -> anyhow::Result<ExitCode> {
    let paths = paths::Paths::resolve()?;
    match cli.command {
        Commands::Run { target, seed } => commands::run(&target, seed, &paths, run_id, agent_mode).await,
        Commands::Suite { filter, seed } => {
            commands::suite(filter.as_deref(), seed, &paths, run_id, agent_mode).await
        }
        Commands::Report { run_id: requested } => commands::report(requested.as_deref(), &paths),
        Commands::Preflight { json } => commands::preflight(json, &paths).await,
        Commands::Coverage { write } => commands::coverage(write),
    }
}
