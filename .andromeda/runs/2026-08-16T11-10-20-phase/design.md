# design extract

## No domain coverage

This chunk is backend-only Rust `tracing` self-observation spans (`fault.silence` / `fault.ramp` / `fault.port_occupier`) written into `logs/agent-latest.jsonl` via `JsonObsLayer` plus a `redact.rs` field allowlist — no rendered surface, no CLI operator-facing output, no tokens/typography/motion/iconography/component patterns are touched (scope.md explicitly states "the a11y/GUI surfaces are untouched").
