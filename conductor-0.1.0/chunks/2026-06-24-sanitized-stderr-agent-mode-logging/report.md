# Report — 2026-06-24-sanitized-stderr-agent-mode-logging

**Chunk:** sanitized stderr + agent-mode logging — `--agent-mode` flag forces JSON-only self-obs sink to `logs/agent-latest.jsonl` + Headless resolver (never-block) + `error:`/`hint:` sanitized stderr edge; closes Epoch 8 (conductor-cli)
**Date:** 2026-06-24
**Commits:** none yet — this chunk is uncommitted; wrap commits it in P7 (last_wrap was `2026-06-23-isatty-gated-operator-pause`)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-core/src/{obs.rs, lib.rs}` · `crates/conductor-cli/src/{cli.rs, main.rs, paths.rs, pause.rs, render.rs, commands/run.rs, commands/suite.rs}` · `crates/conductor-cli/tests/cli_smoke.rs` · `crates/conductor-tauri/src/main.rs` · `scripts/{agent-run.sh, agent-run.ps1}`
- **Symbols / APIs:**
  - NEW public: `conductor_core::ObsSink { Stderr, AgentFile(PathBuf) }` (exported from `lib.rs`); `conductor_cli::render::error_block(error_msg, hint) -> String`; `conductor_cli::paths::agent_log_path() -> anyhow::Result<PathBuf>`.
  - CHANGED public: `conductor_core::init_observability(name, run_id, sink: ObsSink)` — gained the 3rd `sink` arg (was 2-arg). `CliResolver::select(spinner, agent_mode: bool)` — gained `agent_mode` (was 1-arg). `commands::run`/`commands::suite` gained `agent_mode: bool`.
  - NEW internal: `obs::resolve_writer`/`open_agent_file`/`ObsWriter`/`ObsWriterGuard` (file-sink writer); `cli::pause::resolve_kind` (pure decision) + `Kind`; `cli::main::{obs_sink, hint_for}`; `render::{stderr_color, error_block_styled}` + `HINT_GREY` const.
  - CLI flags: `--agent-mode` (global) · `--debug`/`-v` (global) on `conductor`.
- **Crates / modules:** no crate added/removed. Changed: `conductor-core` (obs sink), `conductor-cli` (cli/main/paths/pause/render/commands), `conductor-tauri` (1-line init call).
- **Dependencies:** NONE added/bumped — the agent-file sink is `std`-only (`Arc<Mutex<File>>` + `OpenOptions`). `Cargo.lock` un-drifted.
- **Schema / config:** no migrations. New env handle `CONDUCTOR_AGENT_MODE` (a **read-only trigger** — `agent_mode = --agent-mode flag || env-set`; main never writes it). New on-disk artifact `logs/agent-latest.jsonl` (the self-obs JSON stream in agent mode; a SEPARATE artifact + schema from the emission journal `runs/<run_id>.jsonl`), resolved as a sibling of the runs dir via `resolve_under`.
- **Coverage of new surfaces:**
  - `--agent-mode` flag → agent-file self-obs sink → validation n/a (bool flag) · instrumentation = IS the self-obs sink (dual-sink select) · PII redacted✓ (the file sink inherits the existing processor-stage field-allowlist + `redact_value` — verified by `agent_file_sink_writes_redacted_json_to_the_file`) · tests unit (obs ×2) + e2e (`agent_mode_routes_self_obs_to_the_log_file_not_stderr`) · a11y n/a (cli) · tokens n/a.
  - `error:` / `hint:` stderr edge → validation n/a · instrumentation n/a (the operator error edge) · PII redacted✓ (`sanitize_error` Display-only + host-path scrub; the full chain is `--debug`-gated to stderr only, never artifacts) · tests unit (render `error_block` ×2) + e2e (`run_with_unknown_target_is_a_sanitized_error_with_a_hint`) · a11y n/a (cli — ASCII `error:`/`hint:` labels never color-alone) · tokens design-token✓ (`lamp_code(Lamp::Fail)`=203 / `HINT_GREY`=246 xterm, tty-gated on stderr).

## Deviations from intent
1. **`commands/mod.rs` not edited** (plan listed "`commands/mod.rs` / dispatch path"). `agent_mode` is threaded through the dispatch in `main.rs`; `mod.rs`'s `pub use` re-exports are signature-agnostic, so it needed no change. In-scope; the plan offered the OR.
2. **`hint_for` matches the anyhow-chain substrings, not a `CoreError` downcast** (plan: "downcast to `CoreError` where typed, else a narrow context-string check"). `CoreError` has only `Config`/`Validation`; the scenario-not-found / manifest / `:4317` faults are anyhow **contexts**, not `CoreError` variants — so the "narrow context-string check" branch is the correct one (matched against Conductor's own stable context strings). Anticipated by the plan.
3. **D4 (pre-decided P4): `CONDUCTOR_AGENT_MODE` is a read-only trigger, not written by `main`** — avoids edition-2024 `unsafe std::env::set_var`. obs-plan §3 phrasing is "`--agent-mode` … sets `CONDUCTOR_AGENT_MODE=1`"; the observable mode is identical (scripts export it OR pass the flag; main reads `flag || env`). Recorded for the drift check — obs-plan §3 may want a wording amendment.

(Not a deviation — explicitly planned: dev mode stays **JSON-to-stderr**, not pretty-print; agent mode adds only the file redirection. obs-plan §3 says "pretty-print in dev" but the `2026-06-15-structured-logging-stack` impl emits JSON to stderr in both modes; pretty-print was never built and is out of scope.)

## Decisions & corrections
- **D1** one honest obs entry point — `init_observability` gains the `sink` param (both cli + tauri callers updated), not a parallel `…_with_sink` fn. Epoch-9 Tauri will reuse the same door for its `logs/conductor-tauri.jsonl`.
- **D2** (user-decided P4) category-mapped `hint:` + generic `--debug` fallback (vs generic-only).
- **D3** (user-decided P4) agent-log path = sibling of `runs/` (moves with `CONDUCTOR_RUNS_DIR`) vs fixed-at-root / nested.
- **D4** (decided P4) read-only `CONDUCTOR_AGENT_MODE` trigger — no edition-2024 `unsafe set_var`.
- **Enum-dispatch writer over a non-generic sink:** `build_subscriber<W: MakeWriter>` is generic, but `set_global_default` takes ONE concrete subscriber — the two sinks (stderr vs file) are different `W` types, so they're unified by an `ObsWriter { Stderr | File(Arc<Mutex<File>>) }` enum that impls `MakeWriter` (dispatching per-line via an `ObsWriterGuard`). Mirrors the `2026-06-23` enum-dispatch-over-non-object-safe-trait `CliResolver` precedent.
- **Infallible init:** an `AgentFile` that can't be opened falls back to `ObsWriter::Stderr` — startup logging never blocks the run (tested `agent_file_sink_falls_back_to_stderr_when_unopenable`).
- **Testable-pure-core reused:** `resolve_kind(agent_mode, stdin_tty, stdout_tty)` (pause) + `error_block_styled(…, color)` (render) follow the established `render::*_styled(color)` pattern so both arms test deterministically without a pty / tty.

## Outcome
- **All acceptance criteria met.** `--agent-mode` routes self-obs JSON to `logs/agent-latest.jsonl` (no stderr JSON) with inherited redaction; `error:`/`hint:` sanitized edge with `--debug`-only chain; `select(…, agent_mode=true)` → Headless; self-obs ≠ emission journal; piped stderr no-ANSI; `conductor-tauri` compiles.
- **Gates green (commands run):** `cargo nextest run --workspace --profile ci` → **395/395** (388→395, +7) · `cargo test --workspace --doc` ok · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo audit` exit 0 (only the known RUSTSEC-2025-0119 allowed-warning) · `cargo deny check` exit 0 · `Cargo.lock` un-drifted.
- **Smoke:** boot-path changed (cli bootstrap + harness scripts) → covered by the real-binary `cli_smoke.rs` E2E (`assert_cmd` spawns the actual `conductor`): agent-mode file sink, error-edge stderr, Blocked envelope all asserted on the real binary. The supplementary manual invocation was declined by the user.
