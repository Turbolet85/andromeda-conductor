# design extract

## No domain coverage

This chunk is test-authoring plus mutation-gate tooling inside `crates/conductor-emit` (`exception.rs` scrubber, `latency.rs` percentile math), `scripts/mutation-roster.toml` and `scripts/mutation-gate.py` — it renders no webview or CLI surface (no `owo-colors`/`indicatif`/`comfy-table` in `conductor-emit/src/`, and the percentile accessors are read only by `conductor-run/src/dispatch.rs` / `conductor-core/src/phase_spec.rs`, not by any output path), so no token, typography, motion, iconography or component-pattern mandate from design-system.md applies.
