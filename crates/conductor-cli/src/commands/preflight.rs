//! `conductor preflight [--json]` — the MCP readiness gate (the `agent-run boot` entrypoint).
//!
//! A serialize-and-exit over [`crate::pipeline::readiness`]: a go/no-go gate (test-plan §3) — `ready`
//! exits 0, a Blocked precondition exits non-zero. A harness fault (e.g. a missing contract manifest)
//! surfaces at the binary's sanitized `Err` edge. Status is never color-alone — the ASCII `[PASS]` /
//! `[BLOCKED]` prefix carries the state (color is ch3).

use std::process::ExitCode;

use crate::paths::Paths;
use crate::pipeline;

pub async fn preflight(json: bool, paths: &Paths) -> anyhow::Result<ExitCode> {
    let state = pipeline::readiness(&paths.manifest_path).await?;
    if json {
        println!("{}", serde_json::to_string(&state)?);
    } else if state.ready {
        println!("[PASS] preflight ready");
    } else {
        let precondition = state.blocked_precondition.as_deref().unwrap_or("not ready");
        println!("[BLOCKED] preflight — {precondition}");
    }
    Ok(if state.ready { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}
