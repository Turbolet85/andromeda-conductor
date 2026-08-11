# design extract

## No domain coverage

This chunk is pure backend `tracing` instrumentation — adding `scenario.run` / `report.generate` / `db.insert_run` spans and their attributes to machine JSON sinks (`logs/agent-latest.jsonl`, `logs/conductor-tauri.jsonl`, both `ObsSink::File`/`Stderr`, never stdout per `crates/conductor-core/src/obs.rs:56-59`) — with no rendered surface: no tokens, typography, motion, iconography, or component patterns are touched, and the scope explicitly excludes any `Verdict`/`ReportState`/run-report-envelope change that the design plan's status tier would govern.
