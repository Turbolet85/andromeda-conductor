# Commands Reference

_Complete command reference from `.andromeda/architecture.md` + `Cargo.toml` + the test/obs harness contracts. The 3–5 most common commands live in CLAUDE.md §Workflow; this file has the rest._

## Build
- `cargo build` — debug build of the workspace.
- `cargo build --release` — local release binary (`conductor-cli`); the source-of-truth release gate. **Never without `cargo audit` green.**
- `cargo tauri build` — optional Tauri 2 GUI bundle (~3 MB; convenience only).

## Agent harness (source of truth — `scripts/agent-run.{sh,ps1}`)
- `agent-run.sh boot` — MCP preflight readiness gate (`conductor preflight --json`): protocol `2024-11-05` + tool presence + data-dir canary. `ready:false` ⇒ dependent scenarios `Blocked`.
- `agent-run.sh run [--unit|--integration|--e2e]` — nextest + doctest + clippy + scenario runs.
- `agent-run.sh status <run_id>` — read the Run-report envelope from `runs/<run_id>.jsonl` / `runs.db`.
- `agent-run.sh cleanup <run_id>` — remove run artifacts + `runs.db` row + release `:4317`; idempotent.
- `agent-run.sh logs <run_id>` — tail the emission journal + sanitized stderr.

## Scenario runs (`conductor-cli`)
- `conductor run <scenario|P-ID> --seed <s>` — drive one scenario.
- `conductor suite --seed <s>` — drive the full catalog, over the SUT capability manifest's accepted set (preflight first).
- `conductor report <run_id>` — re-print a stored run report (pipe-friendly).
- Env: `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, `CONDUCTOR_SEED`. Live read-back needs Pulse's `mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED` + matching `ANDROMEDA_PULSE_DATA_DIR`.

## Testing
- `cargo nextest run --workspace --profile ci` — unit + integration (JUnit XML).
- `cargo nextest run -p conductor-<seam>` — one seam. `cargo nextest run -E 'test(<name>)'` — one test.
- `cargo test --workspace --doc` — doctests (nextest skips these).
- `cargo llvm-cov nextest --lcov --output-path lcov.info --fail-under-lines 60` — coverage gate.
- Webview E2E: `wdio run` (tauri-driver, Linux + xvfb only).

## Linting & quality
- `cargo clippy --workspace --all-targets -- -D warnings` — lint gate.
- `cargo fmt` / `cargo fmt --check` — format / check.
- `cargo check --workspace` — type/compile check.

## Supply chain
- `cargo audit --deny warnings` — RustSec advisory gate.
- `cargo deny check advisories bans sources licenses` — superset gate (`deny.toml`).
- `cargo update` — refresh `Cargo.lock` (review major bumps); keep it committed + un-drifted.

## Database (no migration framework — raw SQL)
- N/A — `runs.db` schema is fixed on first write (rusqlite, bundled SQLite). No migrate/seed/reset commands; `cleanup` removes rows.

## Git & release
- Conventional commits (`feat:`/`fix:`/`chore:`/`refactor:`/`docs:`/`test:`). Feature branch per task; never force-push main.

## Troubleshooting
- `cargo clean` — clean build artifacts (note: `bundled` SQLite recompiles from C → longer cold build).
- `cargo modules generate tree` / `cargo tree` — module/dependency graph. `cargo public-api --simplified --workspace` — API surface.
