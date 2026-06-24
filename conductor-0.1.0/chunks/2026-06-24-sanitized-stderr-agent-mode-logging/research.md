# Codebase Research — 2026-06-24-sanitized-stderr-agent-mode-logging

## Scope
- **Depth:** deep (mature codebase, cross-seam: conductor-core obs + conductor-cli edge) · **Reads:** 11 · **Globs/Greps:** 6 · **code-graph:** queried (call-edges empty for these symbols → direct-grep adoption trace authoritative)

## Files inspected
- `crates/conductor-core/src/obs.rs` (full) — `init_observability(default_service_name, run_id) -> ServiceIdentity` **hardcodes `std::io::stderr` as the MakeWriter sink** (line 62); `build_subscriber<W: MakeWriter>(identity, make_writer, filter)` is already **generic over the writer** (line 70) — a file sink drops in with no layer change. `JsonObsLayer::on_event` already redacts at the processor stage (`redact_value` on `target`/str/error/debug fields; `is_allowlisted` drops non-allowlisted). Emits **flat JSON** to the sink (not pretty) in BOTH modes today.
- `crates/conductor-core/src/redact.rs` (full) — `sanitize_error(&dyn Error) -> String`: Display-only (never Debug → no struct dump / `{:?}` backtrace), `redact_value`'d, whitespace-collapsed to one line. **Its doc comment (line 98) explicitly defers the `error:` / `hint:` wrapping to "Epoch 8" — i.e. THIS chunk** — and keeps the core half anyhow-free. `redact_value` masks absolute host-path tokens only (drive-letter / `/home/` / `/Users/` / `%APPDATA%` / `~/` / `.cargo` / `.rustup`), preserving repo-relative + `::` module paths.
- `crates/conductor-core/src/lib.rs` (full) — public exports: `init_observability`, `mint_run_id`, `now_rfc3339`, `ServiceIdentity`, `sanitize_error`, `redact_value`. `build_subscriber` is **private** (obs.rs-internal). Doc: the init is "the side-effecting entrypoint … so the CLI and Tauri shells initialize logging identically."
- `crates/conductor-cli/src/main.rs` (full) — bootstrap order is `mint_run_id()` → `init_observability(...)` → **`Cli::parse()`** (lines 23-25): the subscriber is installed BEFORE args are parsed, so `--agent-mode` can't currently select the sink → **ordering must change**. The anyhow edge (line 29) is `eprintln!("error: {}", sanitize_error(&*err))` — **`error:` only; no contextual detail, no `hint:`, no `--debug` gate**.
- `crates/conductor-cli/src/cli.rs` (full) — top-level `Cli` carries only `command`; the five verbs are flag-local. **No global flags** → `--agent-mode` + `--debug`/`-v` go here as `#[arg(global = true)]` on `Cli`.
- `crates/conductor-cli/src/pause.rs` (full) — `CliResolver::select(spinner) -> Self` (line 71) is the isatty gate (`stdin().is_terminal() && stdout().is_terminal()` → Interactive, else `Headless::proceed()`). Existing test `select_off_tty_is_headless` (line 104). **The PREREQ override lands here.**
- `crates/conductor-cli/src/commands/run.rs` (full) — `CliResolver::select(None)` at line 19; threads `&resolver` into `execute_scenario`. Call-site #1 for the agent-mode bool.
- `crates/conductor-cli/src/commands/suite.rs` (full) — `CliResolver::select(Some(progress.clone()))` at line 25. Call-site #2.
- `crates/conductor-cli/src/pipeline.rs` (full) — `execute_scenario(pf, scenario, run_id, resolver: &CliResolver)` already takes `&CliResolver`; the operator-checklist hold (empty-`expected` path, line 136-145) is the only `resolve_hold` await — unreachable in CI (Blocked-first). No change needed beyond the resolver the callers pass.
- `crates/conductor-cli/src/paths.rs` (full) — `Paths::resolve()` reads `CONDUCTOR_{SCENARIOS,RUNS,CONTRACT}_DIR` via `resolve_handle(base, var, default)` → `conductor_core::resolve_under` (rejects traversal/absolute escape). **The agent-log path must reuse this `resolve_under` guard**, not raw-join. `Paths::resolve()` runs inside `dispatch` (AFTER current obs-init).
- `crates/conductor-cli/src/commands/mod.rs` (full) — `persist`/`print_record`/`exit_code` helpers; `exit_code` is non-zero only on a hard `Fail` (test-plan §1).
- `crates/conductor-cli/tests/cli_smoke.rs` (full) — `assert_cmd` E2E; `conductor(dir)` forces Blocked via `ANDROMEDA_PULSE_DATA_DIR="pulse;injection"`. `run_with_unknown_target_is_an_error` (line 94) asserts `.failure()` only — **extend it to assert the `error:`/`hint:` shape + no host-path leak**. `piped_coverage_output_carries_no_ansi_escapes` (line 165) is the ANSI-strip precedent.

## Graph impact (direct-grep adoption trace — code-graph `calls_m` returned empty for these symbols)
- **`init_observability`** — TWO callers: `conductor-cli/main.rs:24` (`"conductor"`) **and `conductor-tauri/main.rs:4` (`"conductor-tauri", None`)**. A signature change ripples to BOTH → the Tauri stub must keep compiling (pass the stderr/dev sink; its own `logs/conductor-tauri.jsonl` file sink is Epoch-9, not this chunk).
- **`CliResolver::select`** — 2 callers: `run.rs:19`, `suite.rs:25`. Both gain the agent-mode bool.
- **`sanitize_error`** — `cli/main.rs:29` (the edge I wrap) + `core/scenario.rs:109` (internal toml-error sanitize — **unaffected**, I don't change `sanitize_error`'s signature).
- **`execute_scenario`** — `run.rs:20`, `suite.rs:28`; already `&CliResolver`, no change.

## Patterns detected
- **Generic-writer subscriber** (`obs.rs:70`): `build_subscriber<W: MakeWriter>` — the file sink is a `MakeWriter` impl; the obs.rs test `SharedBuf` (line 256) is the exact `Arc<Mutex<…>>`-backed `Write + MakeWriter` template to mirror for a `Mutex<File>` agent sink.
- **Testable-pure-core** (session-learnings, render.rs `*_styled(color: bool)`): split the env-reading shell from a pure decision fn so both branches test deterministically — apply to `select` (a pure `resolve_kind(agent_mode, stdin_tty, stdout_tty)`) and the sink choice.
- **Single isatty gate** (`pause.rs:72` + `render::stdout_color()`): one `IsTerminal` primitive drives color AND the resolver; agent-mode is an OR-override on top, never a parallel mechanism.
- **Blocked-spine E2E** (`cli_smoke.rs:26`): injection-metacharacter data-dir forces Blocked hermetically on any host — the harness for the new agent-mode + error-edge E2E tests.
- **Processor-stage redaction already in place** (`obs.rs:175`, `redact.rs`): this chunk REUSES it (no new redaction policy) — the file sink inherits the same redacted layer.

## Conventions to follow
- **`std`-only date/JSON in obs** (`obs.rs`): no new crates for the sink; `Mutex<File>` + `std::io::Write` (the existing hand-rolled convention; obs.rs avoids `regex`/date crates).
- **`resolve_under` for any path handle** (`paths.rs:70`, security-plan §Input Validation): the agent-log path is canonicalized + bounds-checked, never raw-joined.
- **owo-colors + one `stdout_color()` bool** (session-learnings; design.md synthesis note): do NOT add `anstream`/`anstyle` (the distiller's suggestion) — the error edge reuses the existing tty gate; piped stderr carries no ANSI.
- **No nextest retries / `std::time` stamps / Display-not-Debug at the edge** (testing.md, security-plan): unchanged invariants.

## New files to create
- (none expected) — all changes land in existing files. (A new `crates/conductor-cli/src/` module for the error-edge formatter is OPTIONAL if `main.rs` grows; default is to keep it inline in `main.rs` + a small helper.)

## Files to modify
- `crates/conductor-core/src/obs.rs` — parameterize the sink: `init_observability(default_service_name, run_id, sink: ObsSink)` where `ObsSink { Stderr, AgentFile(PathBuf) }`; `build_subscriber` stays generic (dispatch the writer by sink). + unit test: file sink writes redacted JSON with `run_id`.
- `crates/conductor-core/src/lib.rs` — export `ObsSink`.
- `crates/conductor-cli/src/main.rs` — reorder `Cli::parse()` BEFORE `init_observability`; resolve `agent_mode` (flag OR pre-set `CONDUCTOR_AGENT_MODE` env) → `std::env::set_var("CONDUCTOR_AGENT_MODE","1")` when the flag is set → pick `ObsSink` → init; rewrite the anyhow edge to the `error:` / (contextual) / `hint:` shape, `--debug` surfacing the full `{:?}` chain to stderr only.
- `crates/conductor-cli/src/cli.rs` — add `#[arg(long, global = true)] agent_mode: bool` + `#[arg(long, short = 'v', global = true)] debug: bool` on `Cli`.
- `crates/conductor-cli/src/pause.rs` — `select(spinner, agent_mode: bool)`; force `Headless` when `agent_mode`; pure `resolve_kind` helper + new unit test `select_agent_mode_is_headless`; update the existing `select_off_tty_is_headless` call.
- `crates/conductor-cli/src/commands/run.rs` + `suite.rs` — thread the `agent_mode` bool into `select(...)`.
- `crates/conductor-tauri/src/main.rs` — update the `init_observability("conductor-tauri", None, ObsSink::Stderr)` call (keep-compiling 1-liner; its file sink is Epoch-9).
- `crates/conductor-cli/tests/cli_smoke.rs` — new E2E: `--agent-mode` writes `logs/agent-latest.jsonl` (JSON + `run_id`) and keeps the Blocked envelope; extend `run_with_unknown_target_is_an_error` to assert `error:` + `hint:` on stderr with no host path.
- `scripts/agent-run.{sh,ps1}` — surface `--agent-mode` on the optional scenario leg (`conductor run "$SCENARIO" --agent-mode` or export `CONDUCTOR_AGENT_MODE=1`) so the release-gate path exercises the agent sink. (Confirm parity .sh ↔ .ps1.)

## Open questions (resolve at P4)
1. **`hint:` strategy** — generic single hint (always `--debug` pointer) vs. category-mapped hints (downcast known `CoreError`/context roots: scenario-not-found, manifest-missing, egress-refused) + generic fallback. (AskUserQuestion — shapes the error-edge surface + test count.)
2. **Agent-log path** — `logs/agent-latest.jsonl` as a **sibling of `runs/`** (`runs_dir.parent()/logs`, moves with `CONDUCTOR_RUNS_DIR`) vs. **fixed at project root** (`./logs`, ignores the runs override) vs. **nested** (`{runs_dir}/logs`). obs-plan §3 says "project root, relative to CONDUCTOR_RUNS_DIR" — internally tense. (AskUserQuestion — affects CI artifact path + agent log discovery.)
3. **`init_observability` shape** — parameterize the shared fn (Option A, updates the Tauri caller now) vs. add `init_observability_with_sink` + keep the 2-arg wrapper (Option B, zero Tauri ripple). Leaning A (one honest entry; Tauri change is trivial + Epoch-9 will want its own file sink) — decided in the plan, not asked.
