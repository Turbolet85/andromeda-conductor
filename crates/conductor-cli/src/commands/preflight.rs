//! `conductor preflight [--json]` — the MCP readiness gate (the `agent-run boot` entrypoint).
//!
//! A serialize-and-exit over [`crate::pipeline::readiness`]: a go/no-go gate (test-plan §3) — `ready`
//! exits 0, a Blocked precondition exits non-zero. A harness fault (e.g. a missing contract manifest)
//! surfaces at the binary's sanitized `Err` edge. Status is never color-alone — the ASCII `[PASS]` /
//! `[BLOCKED]` prefix carries the state (color is ch3).

use std::process::ExitCode;

use conductor_core::Lamp;

use crate::paths::Paths;
use crate::pipeline;
use crate::render;

pub async fn preflight(json: bool, paths: &Paths) -> anyhow::Result<ExitCode> {
    let state = pipeline::readiness(&paths.manifest_path).await?;
    if json {
        println!("{}", serde_json::to_string(&state)?);
    } else if state.ready {
        println!(
            "{} preflight ready",
            render::paint(Lamp::Pass.status_prefix(), render::lamp_code(Lamp::Pass))
        );
    } else {
        let precondition = state.blocked_precondition.as_deref().unwrap_or("not ready");
        println!(
            "{} preflight — {precondition}",
            render::paint(Lamp::Blocked.status_prefix(), render::lamp_code(Lamp::Blocked))
        );
    }
    Ok(if state.ready { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}
