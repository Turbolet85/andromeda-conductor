//! The clap verb surface — `conductor run` / `suite` / `report` / `preflight` / `coverage`.

use clap::{Parser, Subcommand};

/// Scenario-driven OTLP fault-injection + verification harness for a live Pulse instance.
#[derive(Parser)]
#[command(name = "conductor", version, about)]
pub struct Cli {
    /// Agent mode: route self-obs JSON to `logs/agent-latest.jsonl` and never block on an operator pause.
    #[arg(long, global = true)]
    pub agent_mode: bool,
    /// Print the full (unsanitized) error chain to stderr on failure.
    #[arg(long, short = 'v', global = true)]
    pub debug: bool,
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
    /// Run the MCP readiness preflight gate (the `agent-run boot` entrypoint).
    Preflight {
        /// Emit the full readiness result as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Render the capability coverage matrix (the definition-of-done classification).
    Coverage {
        /// Regenerate `coverage-matrix.md` at the repo root.
        #[arg(long)]
        write: bool,
    },
    /// Probe the live-Pulse preconditions without firing a preflight canary.
    Preconditions {
        /// Emit the full precondition result as JSON.
        #[arg(long)]
        json: bool,
        /// Probe for the scenario the next leg drives: its declared L4 posture decides how the
        /// deterministic-L4 handle is graded. A scenario NAME — a P-ID can name several scenarios.
        #[arg(long = "for", value_name = "SCENARIO", value_parser = scenario_name)]
        for_scenario: Option<String>,
    },
    /// Remove a run's `runs.db` rows across every table it wrote (the harness `cleanup` teardown).
    Cleanup {
        /// The run_id whose rows are removed.
        run_id: String,
    },
}

/// `--for` takes a scenario name only. A P-ID resolves in the catalog's unsorted directory order and
/// can match more than one scenario — two name P-018 — so the probe would grade the posture of
/// whichever file the filesystem listed first.
fn scenario_name(value: &str) -> Result<String, String> {
    let pid_shaped = value
        .strip_prefix("P-")
        .is_some_and(|rest| rest.len() == 3 && rest.bytes().all(|b| b.is_ascii_digit()));
    if pid_shaped {
        Err(format!(
            "{value} is a P-ID; --for takes a scenario name, because a P-ID can name several scenarios"
        ))
    } else {
        Ok(value.to_string())
    }
}
