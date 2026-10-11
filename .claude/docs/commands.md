# Commands Reference

_Complete command reference from `.andromeda/architecture.md` + `Cargo.toml` + the test/obs harness contracts. The 3–5 most common commands live in CLAUDE.md §Workflow; this file has the rest._

## Build
- `cargo build` — debug build of the workspace.
- `cargo build --release` — local release binary (`conductor-cli`); the source-of-truth release gate. **Never without `cargo audit` green.**

## Agent harness (source of truth — `scripts/agent-run.{sh,ps1}`)
- `agent-run.sh boot` — MCP preflight readiness gate (`conductor preflight --json`): protocol `2024-11-05` + required-tool presence + the run-contract terms + the canary round-trip. `ready:false` ⇒ dependent scenarios `Blocked` under whichever of the gate's FIVE named preconditions applies (version mismatch · missing tool · no incident opened after the canary storm · app-sidecar workspace-key agreement · unmet run-contract terms) — never a generic string. The wrapper's wall-clock budget is DERIVED per invocation from `contracts/pulse-run-contract.toml` (`warmup_ms/1000 + min_canary_poll_seconds + margin`); a missing term is a hard exit 2, and a lowered `CONDUCTOR_PREFLIGHT_TIMEOUT` clamps UP to the contract floor, never down. **`boot` writes NO run artifacts** — no journal, no `runs.db` row, no `agent-latest.jsonl` refresh; a criterion needing those requires a SCENARIO leg.
- `agent-run.sh run [--unit|--integration|--live [real-model]]` — nextest + doctest + clippy + scenario runs. `--live` is the operator-gated live-Pulse suite (needs a running `pulse-app`; never a CI gate, never reachable from a bare `run`) — it refuses at exit 1 naming each unmet precondition rather than firing a leg. `--live real-model` runs the ONE real-model leg instead (`real-model-interpretation`, led by `conductor preconditions --for real-model-interpretation`; needs `pulse-app` under the real-model posture — deterministic L4 absent or falsy; its preflight canary is three storms 90 s apart, and each invocation is one drive of a record the posture contract fixes before its first drive — a counted drive of a pre-stated series, or a drive of its one 2026-10-07 capture run, which states no verdict); an unknown selector exits 2. Scenario leg: `SCENARIO=<name|P-ID> [SEED=<n>] agent-run.sh run` — `--seed` is appended ONLY when `SEED` is explicitly set (otherwise the TOML-declared seed governs); a P-ID several scenarios name is refused before any load, naming them, so name the scenario where several share one.
- `agent-run.sh status <run_id>` — read the Run-report envelope from `runs/<run_id>.jsonl` / `runs.db`.
- `agent-run.sh cleanup <run_id>` — remove run artifacts (`rm -f` in the shells) then `conductor cleanup <run_id>` for the run's rows in **all three `runs.db` tables** (`runs`, `run_check`, `run_envelope` — teardown covers every table a run writes, or cleanup leaves orphans) + release `:4317`; idempotent (file-absence is not an error; a zero-row `DELETE` succeeds). The shells issue no SQL — the deletes are rusqlite bound-parameter statements inside the binary (2026-09-06).
- `agent-run.sh logs <run_id>` — tail the emission journal + sanitized stderr.

## Scenario runs (`conductor-cli`)
- `conductor run <scenario|P-ID> --seed <s>` — drive one scenario, its gate evaluated under the scenario's declared L4 posture (a P-ID named by several scenarios is refused before any load — `error:` names them, `hint:` says to pass one; exit 1).
- `conductor preconditions [--for <SCENARIO>] [--json]` — the non-mutating live-Pulse probe; `--for` grades the handles under that scenario's declared L4 posture (a P-ID is refused, exit 2).
- `conductor suite --seed <s>` — drive the full catalog, over the SUT capability manifest's accepted set (preflight first).
- `conductor report <run_id>` — re-print a stored run report (pipe-friendly).
- `conductor cleanup <run_id>` — remove one run's rows from all three `runs.db` tables in a single transaction (rusqlite bound parameters); idempotent, prints the removed row count. The teardown half of `agent-run.sh cleanup`.
- Env: `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, `CONDUCTOR_SEED`. Live read-back needs Pulse's `mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED` + matching `ANDROMEDA_PULSE_DATA_DIR`.

## Testing
- `cargo nextest run --workspace --profile ci` — unit + integration (JUnit XML).
- `cargo nextest run -p conductor-<seam>` — one seam. `cargo nextest run -E 'test(<name>)'` — one test.
- `cargo test --workspace --doc` — doctests (nextest skips these).
- `cargo llvm-cov nextest --lcov --output-path lcov.info --fail-under-lines 60` — coverage gate.

## Linting & quality
- `cargo clippy --workspace --all-targets -- -D warnings` — lint gate.
- `cargo fmt` / `cargo fmt --check` — format / check.
- `cargo check --workspace` — type/compile check.

## Supply chain
- `cargo audit` — RustSec advisory gate; the BARE form is the gate (`.github/workflows/ci.yml`). `--deny warnings` exits 1 by construction against the `deny.toml`-adjudicated allowed set (17 as of 2026-09-07: 16 `unmaintained` + 1 `unsound`) — never add it.
- `cargo deny check advisories bans sources licenses` — superset gate (`deny.toml`).
- `cargo update` — refresh `Cargo.lock` (review major bumps); keep it committed + un-drifted.

## Database (no migration framework — raw SQL)
- N/A — `runs.db` schema is fixed on first write (rusqlite, bundled SQLite). No migrate/seed/reset commands; `cleanup` removes rows.

## Git & release
- Conventional commits (`feat:`/`fix:`/`chore:`/`refactor:`/`docs:`/`test:`). Feature branch per task; never force-push main.

## Troubleshooting
- `cargo clean` — clean build artifacts (note: `bundled` SQLite recompiles from C → longer cold build).
- `cargo modules generate tree` / `cargo tree` — module/dependency graph. `cargo public-api --simplified --workspace` — API surface.
