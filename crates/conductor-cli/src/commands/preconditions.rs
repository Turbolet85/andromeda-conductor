//! `conductor preconditions [--json]` — probe the live-Pulse preconditions before a leg is scheduled.
//!
//! A go/no-go gate like [`crate::commands::preflight`]: every subject satisfied exits 0, any unmet
//! subject exits non-zero, so a caller gates with `if conductor preconditions; then …`. It is NOT a
//! preflight — it fires no canary storm and emits no OTLP, so it primes none of the SUT state a
//! preflight's canary would (architecture §Established Decisions [Read-Back Dependency Posture]).
//!
//! The result is a harness fact, never a verdict: no `Verdict`, no `ReportState`, no per-P-ID row.
//! Every subject is named on both paths — a gate whose failure reports only a count names no defect
//! (test-plan §11), and the exit code alone cannot separate a pass from a skip.

use std::process::ExitCode;

use conductor_core::now_rfc3339;
use conductor_run as pipeline;

use crate::render;

pub async fn preconditions(json: bool) -> anyhow::Result<ExitCode> {
    let status = pipeline::observe_preconditions().await;

    if json {
        println!(
            "{}",
            serde_json::to_string(&serde_json::json!({
                "satisfied": status.is_satisfied(),
                "unmet": status.unmet(),
                "checked_at": now_rfc3339(),
            }))?
        );
    } else if let Some(caption) = render::preconditions_caption(&status) {
        println!("{caption}");
    } else {
        println!("[PRECONDITION] every live-Pulse precondition is satisfied");
    }

    Ok(if status.is_satisfied() { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}
