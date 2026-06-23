# Codebase Research — 2026-06-23-5-command-agent-run-harness

## Scope
- **Depth:** deep (CLI surface + the verify preflight seam) · **Reads:** 13 · **Globs/Greps:** 5

## Files inspected
- `scripts/agent-run.sh` (full) — the skeleton already sketches all 5 commands. `boot` = `timeout 30 cargo run -q -p conductor-cli --bin conductor -- preflight --json`. `run` = `nextest --profile ci` + `test --doc` + `clippy -D` (+ optional `SCENARIO=/SEED=` leg). `status` = `tail -n1 runs/<id>.jsonl | jq -e {…}`. `cleanup` = `rm -f` jsonl/md + `sqlite3 … "DELETE … WHERE run_id = '$RUN_ID'"` (**string-interpolated**). `logs` = `cat runs/<id>.jsonl`. Stage flags `--unit/--integration/--e2e` are named in the comment but **not implemented**.
- `scripts/agent-run.ps1` (full) — parity mirror: same 5 commands, same `preflight --json` call, `ConvertFrom-Json` status, `Remove-Item` cleanup.
- `crates/conductor-cli/src/cli.rs` (full) — clap `Commands` = `Run`/`Suite`/`Report` ONLY. **No `Preflight` variant** → `conductor preflight` is an unrecognized subcommand today; `boot` is broken until it lands.
- `crates/conductor-cli/src/main.rs` (full) — `#[tokio::main(current_thread)]`; mints `run_id`, `init_observability`, dispatches the 3 verbs; the error arm prints `error: {sanitize_error}` + `FAILURE`. `run_id` is NOT printed to stdout.
- `crates/conductor-cli/src/pipeline.rs` (full) — `preflight(manifest_path) -> Preflight{client, ready}`: `ReadbackClient::connect(data_dir)` → on `Err` returns `ready:false` (logs only, **drops detail**); on `Ok` loads the manifest + `run_preflight` → keeps only `.ready`. **The full `ReadyState` the boot verb must serialize is discarded.**
- `crates/conductor-cli/src/commands/{mod,run,suite,report}.rs` — `mod.rs`: `persist` (journal+db+md), `print_record` → `"{lamp.status_prefix()} {scenario}"` (no color; color is ch3), `exit_code` → non-zero only on a hard `Fail`. `report.rs`: `latest_run_id(runs_dir)` = the newest `*.jsonl` stem (run_ids sort by time) — the latest-default pattern `status`/`logs` should mirror.
- `crates/conductor-cli/src/paths.rs` (full) — `Paths{scenarios_dir, runs_dir, manifest_path}` resolved via `resolve_under` (traversal-rejected). `manifest_path` defaults `contracts/mcp-contract.toml`. The preflight verb reads the manifest from here.
- `crates/conductor-cli/tests/cli_smoke.rs` (full) — 7 `assert_cmd` E2E cases. The **hermetic forced-unreachable** template (`ANDROMEDA_PULSE_DATA_DIR=pulse;injection` → deterministic Blocked, no sidecar spawn) is reusable verbatim for the preflight test. `help_lists_the_three_verbs` asserts `contains(run ∧ suite ∧ report)`.
- `crates/conductor-verify/src/preflight.rs` (full) — **`ReadyState` (`#[derive(Serialize)]`) IS the arch readiness JSON shape**: `ready`, `negotiated_protocol_version`, `expected_protocol_version`, `required_tools`, `data_dir`, `canary_round_trip`, `blocked_precondition`, `checked_at` + `report_state()`. `run_preflight(client, manifest, canary, data_dir) -> Result<ReadyState>` always `Ok` (MCP errors fold into Blocked legs). `preflight_boot(transport, …)` is the injected-transport sibling (drives the stub child test); its doc names "the Epoch-8 `conductor preflight` verb" as a driver, but it takes a transport the CLI cannot build. `UNREACHABLE_PRECONDITION` const = the named unreachable precondition.
- `crates/conductor-verify/src/client.rs` (connect path) — `ReadbackClient::connect(Option<PathBuf>)` does the FULL hardened spawn (`resolve_data_dir` → `build_command` → `TokioChildProcess::new` → `connect_transport`); a single `Err(VerifyError)` covers data-dir-rejected / spawn-fail / initialize-fail — i.e. every "unreachable" cause.
- `crates/conductor-verify/src/spawn.rs` (full) — the hardened-spawn internals (`resolve_data_dir`/`build_command`/`PULSE_MCP_PROGRAM`/`DATA_DIR_ENV`) are `pub(crate)` → the CLI MUST reuse `ReadbackClient::connect`, never rebuild the transport (the no-new-boundary rule).
- `crates/conductor-verify/src/lib.rs` (full) — exports `ReadyState`, `run_preflight`, `preflight_boot`, `CanaryMarker`, `ContractManifest`, `ReadbackClient`.
- `.github/workflows/ci.yml` (run step) — CI dogfoods **`.\scripts\agent-run.ps1 run`** (pwsh, fail-fast), NOT `boot`. So preflight/boot exit codes do not gate CI; `run` (no flag = full gate) MUST stay green.
- `.andromeda/test-plan.md` §3 + §9 — §3 boot: stdout `{"ready":true}`; exit `0 on ready:true`, **non-zero on a not-ready/handshake failure**; status/cleanup envelope contract. §9 stage table: `run --unit` → `cargo nextest run --workspace --profile ci`; `run --integration` → rmcp-stub + tauri::test mock + in-mem/TempDir runs.db; `run --e2e` → cli (assert_cmd) + webview.

## Graph impact (from the code-graph)
- **Cold-graph note:** not queried via `code-graph.py` — the change footprint is small + already pinpointed by the targeted reads. It is additive: one new clap verb (`Preflight`) + one new `commands/preflight.rs` + a `ReadyState`-surfacing refactor of `pipeline::preflight`, whose only callers are `commands/run.rs` + `commands/suite.rs` (both keep working off the `ready` bool).

## Patterns detected
- **Hermetic forced-unreachable E2E** (`crates/conductor-cli/tests/cli_smoke.rs:23`): an injection-metachar `ANDROMEDA_PULSE_DATA_DIR` makes `ReadbackClient::connect` `Err` before any spawn → deterministic Blocked on any host. The preflight verb test reuses it.
- **Latest-run default** (`crates/conductor-cli/src/commands/report.rs:33`): newest `*.jsonl` stem; `status`/`logs` mirror it so they are usable without the (un-surfaced) minted run_id.
- **Verdict/error wall** (`crates/conductor-cli/src/pipeline.rs:40`): connect-`Err` → a Blocked *value*, never `Result::Err`; only a bad/missing contract manifest is a harness `Err`.
- **ASCII status prefix, color deferred to ch3** (`crates/conductor-cli/src/commands/mod.rs:31`): `print_record` emits `[LAMP] scenario`, no ANSI — the preflight human line follows suit.

## Conventions to follow
- **Reuse `ReadbackClient::connect` + `run_preflight` UNCHANGED**; add NO spawn/transport code to the CLI (spawn is `pub(crate)`; the no-new-boundary playbook rule from the ch1 session).
- `serde_json::to_string(&ready_state)` for `--json` — the struct already serializes to the arch readiness shape (no hand-built JSON).
- Stage flags are **shell-side** (`run`'s `$2`); no Rust change for them.
- Operator-supplied `run_id` is normally filesystem-safe (`mint_run_id`), but the shell ops that interpolate it (the `sqlite3` DELETE, path joins) validate `^[0-9A-Za-z._-]+$` first — no string-concatenated SQL on unvalidated input (security-plan §Input Validation).
- Exit non-zero only on a hard `Fail` (the `run`/scenario path) or a harness fault; the preflight *gate* exit code follows test-plan §3 (pinned by the P4 decision).

## New files to create
- `crates/conductor-cli/src/commands/preflight.rs` — the `conductor preflight [--json]` handler: obtain the `ReadyState` from the pipeline, emit it as JSON (`--json`) or a human `[PASS]`/`[BLOCKED]` line + the precondition, and return the gate exit code.

## Files to modify
- `crates/conductor-cli/src/cli.rs` — add `Preflight { #[arg(long)] json: bool }` to `Commands`.
- `crates/conductor-cli/src/main.rs` — dispatch `Commands::Preflight`.
- `crates/conductor-cli/src/commands/mod.rs` — `mod preflight; pub use preflight::preflight;`.
- `crates/conductor-cli/src/pipeline.rs` — surface the full `ReadyState`: add a thin `readiness(manifest_path) -> anyhow::Result<ReadyState>` (reuses `ReadbackClient::connect`; on `Err` synthesizes the unreachable `ReadyState` mirroring `preflight_boot`'s Err arm — `UNREACHABLE_PRECONDITION` + `CanaryOutcome::Skipped`; on `Ok` loads the manifest + `run_preflight`). Have `preflight()` reuse it (derive `ready`, keep the client for scenario reuse) so run/suite are unaffected.
- `crates/conductor-cli/tests/cli_smoke.rs` — add preflight verb test(s) (hermetic `ready:false` JSON + exit code) + update `help_lists_the_three_verbs` to the four-verb surface.
- `scripts/agent-run.sh` + `scripts/agent-run.ps1` — implement `run` stage flags (`--unit/--integration/--e2e`, default = full); `run_id` charset validation in `cleanup` (+ `status`/`logs`); `status`/`logs` default-to-latest when run_id omitted. Keep `.sh` ⇄ `.ps1` at strict parity.

## Open questions
- **Preflight exit code on a clean `ready:false`** (a Blocked precondition, not a harness fault): non-zero = gate semantics (test-plan §3 "exit 0 on ready:true, non-zero otherwise"), vs always-0 = "Blocked is not a hard Fail" (the `conductor run` rule, agent reads `.ready` from JSON). → **P4 AskUserQuestion.**
- **Stage-flag depth**: implement the thin `--unit/--integration/--e2e` partition now (per §9; the skeleton already promises it) vs defer to the Epoch-10 CI-quality-gate chunk and ship `run` as the bundled gate only. → **P4 AskUserQuestion.**
