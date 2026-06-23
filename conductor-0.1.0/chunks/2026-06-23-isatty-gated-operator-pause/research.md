# Codebase Research — 2026-06-23-isatty-gated-operator-pause

## Scope
- **Depth:** moderate · **Reads:** 11 (pause.rs, render.rs, pipeline.rs, suite.rs, run.rs, commands/mod.rs, cli.rs, main.rs, cli_smoke.rs, conductor-cli Cargo.toml, workspace Cargo.toml) · **Globs/Greps:** 2 · **Code-graph queries:** 3 (impact + collision + crate-edge)

## Files inspected
- `crates/conductor-core/src/pause.rs` (full) — the Epoch-5 core API this chunk implements against: `HoldPoint{scenario,p_id,step,prompt,allow_no_go}`, `Decision{Go,NoGo}` (+ `.label()`), `HoldResolution`, `trait PauseResolver { fn resolve(&self,&HoldPoint)->impl Future<Output=Decision> }`, `HeadlessResolver{proceed(),abort(),new(d)}`, `resolve_hold(resolver,hold)->HoldResolution`. **The trait returns `impl Future` ⇒ NOT object-safe** (no `&dyn PauseResolver`); dispatch must be generic or an enum.
- `crates/conductor-cli/src/pipeline.rs` (full) — **the integration point already exists.** `execute_scenario` at :101; on a ready gate + empty `expected` (operator-checklist) it builds a `HoldPoint` (:134) and resolves it via a **hardcoded** `resolve_hold(&HeadlessResolver::proceed(), &hold)` (:141), logs the decision (:142), and returns `manual_record(...)` (:143). A non-ready gate returns `Blocked` early (:106-114) **before** the hold — so the hold is reachable only with a live Pulse / rmcp stub.
- `crates/conductor-cli/src/render.rs` (full) — the ch3 render seam to reuse: `stdout_color()` (:83, `IsTerminal` + `NO_COLOR` + `TERM≠dumb`), `lamp_code(Lamp::Hold)=179` amber (:38), `paint_styled` (:89), `spinner(len)` (:67, draw-target hidden when stderr is not a tty), `Lamp::status_prefix()` → `[HOLD]` (test :230). No hold/phase-line renderer exists yet.
- `crates/conductor-cli/src/commands/suite.rs` (full) — owns the spinner: `render::spinner(n)` (:23), loops `execute_scenario` + `progress.inc(1)` (:26-28), `progress.finish_and_clear()` (:30). The `ProgressBar` to freeze on hold lives **here**, not in `execute_scenario`.
- `crates/conductor-cli/src/commands/run.rs` (full) — single scenario, **no spinner**; calls `execute_scenario` directly (:18).
- `crates/conductor-cli/src/commands/mod.rs` (full) — `persist` / `print_record` / `exit_code` helpers; `exit_code` is non-zero only on `Lamp::Fail` (:40-46) — Blocked/ManualCheck stay 0.
- `crates/conductor-cli/src/cli.rs` + `src/main.rs` (full) — the clap verb surface + `current_thread` dispatch; no `--agent-mode` flag exists yet (that is ch5). `init_observability("conductor", run_id)` is already wired (main.rs:23).
- `crates/conductor-cli/tests/cli_smoke.rs` (full) — the E2E pattern: `assert_cmd` + `ANDROMEDA_PULSE_DATA_DIR="pulse;injection"` forces read-back unreachable ⇒ `Blocked` (so existing run/suite E2E never reach the hold; `assert_cmd` pipes stdout ⇒ non-tty ⇒ headless auto-selected). `blocked_state()` reads the JSONL envelope.
- `crates/conductor-cli/Cargo.toml` + root `Cargo.toml` — deps inherit from `[workspace.dependencies]`; `indicatif="0.17"`, `owo-colors="4"`, `comfy-table="7"` already there. `inquire` absent.

## Graph impact (from the code-graph query)
- **`execute_scenario`** — exactly **2 callers**: `commands/run/run() @ run.rs:17` and `commands/suite/suite() @ suite.rs:25`. Adding a `resolver` parameter touches only these two sites.
- **`resolve_hold` / `HeadlessResolver`** — the only non-test caller is `pipeline.rs:141`; all other references are `conductor-core/tests/operator_pause.rs` (core's own tests, untouched).
- **Collision check** — `PromptResolver` / `CliResolver` / `select_resolver` return `rows: 0` → the names are free.
- **`crate_edges WHERE to_crate='conductor-cli'`** → `rows: 0` → conductor-cli is a **leaf bin**; every change here has zero cross-crate blast radius.

## Patterns detected
- **tty-gate primitive** (`render.rs:83` `stdout_color()`; `:68` `stderr().is_terminal()`): `std::io::IsTerminal` is the established tty selector. The resolver selection must reuse the SAME primitive — gate the interactive path on `std::io::stdin().is_terminal() && std::io::stdout().is_terminal()` (the prompt reads stdin, draws on stdout).
- **Resolver already injected via `resolve_hold`** (`pipeline.rs:141`): the change is to swap the hardcoded `HeadlessResolver::proceed()` for a caller-supplied resolver — DI, not new orchestration.
- **Spinner suspend** (indicatif 0.17): `ProgressBar::suspend(f)` clears the bar, runs `f` (the prompt), redraws — the mechanism to freeze the heartbeat at its current count during the `inquire` prompt.
- **Verdict-first lamp / amber** (`render.rs:32-41`): `lamp_code(Lamp::Hold)=179` is the amber the design/layout extracts name (`ANSI 179`); the hold phase-line reuses `paint_styled(_, 179, color)` + the `[HOLD]` prefix.

## Conventions to follow
- **`*_styled(…, color)` split for testability** (`render.rs:89-96`): public render fns delegate to a `_styled` core taking an explicit `color: bool`, so both plain + colored paths are asserted deterministically. A new hold-line renderer must follow this (e.g. `hold_line_styled(&HoldPoint, color)`).
- **Status never color-alone** (`render.rs:8-9`, CLAUDE.md invariant): the `[HOLD]` ASCII prefix is always present; color is the tty-gated overlay (`renders_leak_no_host_paths_or_struct_names` + `status_line_plain_keeps_prefix_without_escapes` are the precedent assertions).
- **Redaction at the artifact edge** (`pause.rs:143` `resolve_hold` already `redact_value`s the prompt into the `HoldResolution`): the rendered prompt to the operator comes from `HoldPoint::prompt` (in-memory, un-redacted by design per pause.rs:52-53) — but any TRACING emit must use the redacted form; the on-terminal prompt is human-only, not an artifact.
- **`[workspace.dependencies]` with a one-line rationale comment** (root `Cargo.toml:59-62`): add `inquire` there with a comment, then `inquire.workspace = true` in conductor-cli (mirrors the ch3 render-dep block).

## New files to create
- `crates/conductor-cli/src/pause.rs` — the CLI resolver seam: `enum CliResolver { Interactive(PromptResolver), Headless(HeadlessResolver) }` impl-ing `PauseResolver` by match-dispatch; `PromptResolver` holding `Option<ProgressBar>` (the spinner to suspend) + the `inquire::Confirm` logic; `CliResolver::select(spinner: Option<ProgressBar>) -> CliResolver` (the isatty gate). Unit tests for the gate + dispatch (no real TTY).
- `crates/conductor-cli/tests/operator_pause.rs` (optional, or fold into `cli_smoke.rs`) — E2E: a piped `run`/`suite` never blocks (already Blocked-first, but assert no hang + exit 0); a unit/integration assertion that `select()` returns `Headless` off-tty.

## Files to modify
- `crates/conductor-cli/src/render.rs` — add `hold_line(&HoldPoint) -> String` (+ `hold_line_styled`) reusing `[HOLD]`/amber-179; export the spinner handle type if needed.
- `crates/conductor-cli/src/pipeline.rs` — `execute_scenario` gains a `resolver: &CliResolver` param (or generic `R: PauseResolver`); swap `&HeadlessResolver::proceed()` at :141 for it; print the `[HOLD]` phase-line before resolving.
- `crates/conductor-cli/src/commands/run.rs` — build `CliResolver::select(None)`, pass to `execute_scenario`.
- `crates/conductor-cli/src/commands/suite.rs` — build `CliResolver::select(Some(progress.clone()))`, pass to `execute_scenario` (so the prompt suspends the live spinner).
- `crates/conductor-cli/src/main.rs` — `mod pause;`.
- `crates/conductor-cli/Cargo.toml` + root `Cargo.toml` — add `inquire` (workspace dep + member).
- `deny.toml` / `Cargo.lock` — re-audit; add a JUSTIFIED `inquire` transitive ignore/allow only if `cargo deny` flags one.

## Open questions
1. **`execute_scenario` param shape** — concrete `&CliResolver` (simplest; both callers pass it; lives in same crate) vs generic `<R: PauseResolver>` (decouples pipeline from the CLI enum, eases stub injection). Lean concrete; resolve at P4. (The trait is NOT dyn-safe, so `&dyn` is out either way.)
2. **inquire-error → Decision mapping** — `resolve` is infallible (`-> Decision`); an `InquireError` (operator Esc/Ctrl-C) must collapse to a `Decision`. Proposed: `Ok(true)→Go`, `Ok(false)→NoGo`, `Err(canceled/interrupted)→NoGo` when `allow_no_go` else `Go`. Confirm at P4.
3. **Spinner freeze fidelity** — `ProgressBar::suspend` clears-then-redraws (heartbeat pauses at its count) vs the design extract's "stop in place (not hide)". `suspend` is the available indicatif primitive and preserves the count; accept it as the faithful CLI mirror (the literal titlebar-freeze is Epoch-9 Tauri). Note in plan.
