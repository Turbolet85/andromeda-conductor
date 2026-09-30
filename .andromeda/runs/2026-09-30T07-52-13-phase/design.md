# design extract

## No domain coverage
The chunk changes only the operator/local dev instrument `scripts/mutation-gate.py`, its roster TOML and fixture tallies, with no production Rust change. It renders nothing on the desktop-webview surface, and the design-system §Surface: cli governs `conductor-cli` / `agent-run` output, not dev tooling. The plan names no mutation gate (a grep for `mutation` in design-system.md returns 0 hits), so it holds no tokens, typography, motion or component pattern that apply.
