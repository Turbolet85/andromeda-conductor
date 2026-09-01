# Commands Reference

_Complete command reference from `.andromeda/architecture.md` + `Cargo.toml` + the test/obs harness contracts. The 3–5 most common commands live in CLAUDE.md §Workflow; this file has the rest._

## Build
- `cargo build` — debug build of the workspace.
- `cargo build --release` — local release binary (`conductor-cli`); the source-of-truth release gate. **Never without `cargo audit` green.**
- `cargo tauri build` — optional Tauri 2 GUI bundle (~3 MB; convenience only).

## Agent harness (source of truth — `scripts/agent-run.{sh,ps1}`)
- `agent-run.sh boot` — MCP preflight readiness gate (`conductor preflight --json`): protocol `2024-11-05` + required-tool presence + the run-contract terms + the canary round-trip. `ready:false` ⇒ dependent scenarios `Blocked` under whichever of the gate's FIVE named preconditions applies (version mismatch · missing tool · no incident opened after the canary storm · app-sidecar workspace-key agreement · unmet run-contract terms) — never a generic string. The wrapper's wall-clock budget is DERIVED per invocation from `contracts/pulse-run-contract.toml` (`warmup_ms/1000 + min_canary_poll_seconds + margin`); a missing term is a hard exit 2, and a lowered `CONDUCTOR_PREFLIGHT_TIMEOUT` clamps UP to the contract floor, never down. **`boot` writes NO run artifacts** — no journal, no `runs.db` row, no `agent-latest.jsonl` refresh; a criterion needing those requires a SCENARIO leg.
- `agent-run.sh run [--unit|--integration|--e2e]` — nextest + doctest + clippy + scenario runs. Scenario leg: `SCENARIO=<name|P-ID> [SEED=<n>] agent-run.sh run` — `--seed` is appended ONLY when `SEED` is explicitly set (otherwise the TOML-declared seed governs).
- `agent-run.sh status <run_id>` — read the Run-report envelope from `runs/<run_id>.jsonl` / `runs.db`.
- `agent-run.sh cleanup <run_id>` — remove run artifacts + the run's rows in **all three `runs.db` tables** (`runs`, `run_check`, `run_envelope` — teardown covers every table a run writes, or cleanup leaves orphans) + release `:4317`; idempotent (file-absence is not an error; a zero-row `DELETE` succeeds).
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
- Webview E2E: `agent-run.{sh,ps1} run --e2e` → ensure-frontend → `cargo build --release -p conductor-tauri --features tauri/custom-protocol` (the FEATURE, not the profile, embeds the bundle) → `wdio run` (tauri-driver). Headless under `xvfb` on Linux CI; headful on the Windows host when `CONDUCTOR_MSEDGEDRIVER` names a driver — unset or not-a-file ⇒ the leg skips at exit 0 with the fetch recipe. macOS has no WebDriver.

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
