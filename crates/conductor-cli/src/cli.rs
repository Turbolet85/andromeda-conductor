//! The clap verb surface — `conductor run` / `suite` / `report`.

use clap::{Parser, Subcommand};

/// Scenario-driven OTLP fault-injection + verification harness for a live Pulse instance.
#[derive(Parser)]
#[command(name = "conductor", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run one scenario by name or Pulse P-ID.
    Run {
        /// Scenario name (e.g. `error-baseline-spike`) or Pulse P-ID (e.g. `P-009`).
        target: String,
        /// Override the scenario seed (takes precedence over `CONDUCTOR_SEED`).
        #[arg(long)]
        seed: Option<u64>,
    },
    /// Run the scenario catalog (optionally filtered by name substring).
    Suite {
        /// Only run scenarios whose name contains this substring.
        #[arg(long)]
        filter: Option<String>,
        /// Override every scenario's seed (takes precedence over `CONDUCTOR_SEED`).
        #[arg(long)]
        seed: Option<u64>,
    },
    /// Render a persisted run's report from its JSONL journal.
    Report {
        /// The run_id to render; the newest run when omitted.
        run_id: Option<String>,
    },
}
