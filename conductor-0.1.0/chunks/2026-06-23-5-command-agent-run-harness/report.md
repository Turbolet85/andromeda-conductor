# Report — 2026-06-23-5-command-agent-run-harness

**Chunk:** 5-command agent-run harness — boot=preflight · run=nextest+scenarios · status=runs.db/JSONL · cleanup=idempotent · logs=journal; .sh + .ps1 parity over the conductor-cli verbs
**Date:** 2026-06-23
**Commits:** none since last_wrap (this wrap creates the chunk commit)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-cli/src/commands/preflight.rs` — the `conductor preflight [--json]` verb handler.
  - MOD `crates/conductor-cli/src/pipeline.rs` — `readiness()` + `unreachable_state()` (surface the full `ReadyState`).
  - MOD `crates/conductor-cli/src/cli.rs` (Preflight clap variant) · `main.rs` (dispatch) · `commands/mod.rs` (wire module) · `tests/cli_smoke.rs` (preflight E2E + 4-verb help).
  - MOD `scripts/agent-run.sh` + `scripts/agent-run.ps1` — `run` stage flags · `run_id` validation · status/logs latest-default · parity.
- **Symbols / APIs:**
  - NEW CLI subcommand `conductor preflight [--json]` (the `agent-run boot` entrypoint); the verb surface goes 3→4 verbs.
  - NEW crate-internal `pipeline::readiness(&Path) -> anyhow::Result<ReadyState>` (pub, bin-internal) · `commands::preflight(bool, &Paths) -> anyhow::Result<ExitCode>` · private `pipeline::unreachable_state(&ContractManifest, &str) -> ReadyState`.
  - NO new external API / IPC method / HTTP endpoint / port / socket (conductor-cli is a bin; reuses the hardened `ReadbackClient::connect` + `run_preflight` UNCHANGED — no new subprocess-spawn code).
  - Shell surface: `agent-run.{sh,ps1} run [--unit|--integration|--e2e]`; `status`/`logs` default to the latest run; `cleanup`/`status`/`logs` validate `run_id` (`^[0-9A-Za-z._-]+$`).
- **Crates / modules:** no crate added/removed; new module `conductor-cli::commands::preflight`.
- **Dependencies:** NONE added or bumped (`Cargo.toml` + `Cargo.lock` untouched; `cargo audit` exit 0).
- **Schema / config:** none. The emitted readiness JSON is the pre-existing `ReadyState` shape (arch §Standard Contracts); no new schema, no new env var (consumes existing `ANDROMEDA_PULSE_DATA_DIR` / `CONDUCTOR_CONTRACT_MANIFEST` / `CONDUCTOR_PREFLIGHT_TIMEOUT`).
- **Coverage of new surfaces:**
  - `conductor preflight [--json]` (new CLI verb) → validation {reuses hardened data-dir injection-reject ✓} · instrumentation {run_id-tagged tracing JSON via the existing `verify.readback.preflight` span + an info line; no new span name ✓} · PII {`data_dir` masked via `conductor_core::redact_value` ✓} · tests {cli_smoke E2E unit/e2e ✓} · a11y {ASCII `[PASS]`/`[BLOCKED]` prefix, never color-alone ✓} · tokens {n/a — cli}
  - `agent-run.{sh,ps1}` stage-flags + run_id-validation (new shell surface) → validation {`run_id` charset-checked before any `sqlite3`/path-join — no string-concat SQL on raw input ✓} · instrumentation {n/a — shell wraps the binary} · PII {n/a} · tests {smoke: `bash -n`, ps1 parse, `boot` + `run --unit` executed ✓; shell isn't nextest-covered} · a11y {ASCII output ✓} · tokens {n/a}

## Deviations from intent
1. **`pipeline::preflight()` left UNCHANGED** (plan: "have preflight() reuse readiness()"). `preflight()` retains the `ReadbackClient` for scenario reuse while `readiness()` discards it — full reuse would double-spawn the sidecar or need an awkward client-returning signature. Added `readiness()` alongside (~6 lines of connect/manifest logic shared by duplication). Justification: lower blast radius — the run/suite/report cli_smoke tests pass unchanged.
2. **`UNREACHABLE_PRECONDITION` string duplicated** in `pipeline.rs` (the plan anticipated this). conductor-verify's const + spawn helpers are `pub(crate)`; reusing them is out-of-scope (a verify edit). The literal matches arch §Standard Contracts (cited inline). Follow-up (deferred, out-of-scope): expose a `conductor_verify::readiness(...)` real-sidecar sibling of `preflight_boot` to retire the duplication.
3. **`.ps1` polish beyond the plan** — error paths use `[Console]::Error.WriteLine` (Write-Error under `$ErrorActionPreference='Stop'` throws, masking the intended exit code) + `$env`→`$envelope` (the former collides with the `$env:` provider). Within the parity/agent-parseable intent.

## Decisions & corrections
- **P4 decision (user):** `conductor preflight` is a go/no-go gate — exit 0 iff `ready:true`, **non-zero on `ready:false`** (test-plan §3); the full `{ready,…}` JSON prints either way. CI-safe (CI dogfoods `run`, not `boot`). This intentionally differs from `conductor run`'s "Blocked → exit 0" — preflight is a gate, not a verdict reporter.
- **P4 decision (user):** stage flags `--unit/--integration/--e2e` implemented thin now (per test-plan §9; default = the full bundled gate), not deferred.
- **Intent-incompleteness (resolved at phase-P5):** scope.md was amended to name the `conductor preflight` verb — the harness BUILDS the verb (ch1 never shipped it), not just shell glue.
- **Boundary held:** ch3 (owo-colors/indicatif rich rendering), ch4 (isatty operator-pause / inquire), ch5 (agent-mode JSON-to-file + the `scenario.run` root span) are explicitly OUT of scope; the design/a11y/obs extracts' forward-looking content maps there.
- **Technique reused:** the hermetic forced-unreachable E2E (`ANDROMEDA_PULSE_DATA_DIR=pulse;injection`) now also covers `conductor preflight --json` (deterministic ready:false on any host).

## Outcome
- **Acceptance criteria: all met.** Five commands at `.sh`/`.ps1` parity; `boot` reports readiness without emitting scenarios; `preflight --json` emits the arch readiness shape (ready:false + canonical precondition) and exits non-zero hermetically; `run --unit` dispatches; `cleanup` idempotent + run_id-validated; status never color-alone; no host-path leak.
- **Gates green:** `cargo nextest run -p conductor-cli` 8/8 · `cargo nextest run --workspace --profile ci` 374/374 (373→374; goldens unchanged) · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo test --workspace --doc` ok · `cargo audit` exit 0.
- **Smoke ✓ (boot-path changed):** `preflight --json` → ready:false JSON + exit 1; `agent-run.sh boot` wrapper (timeout→coreutils); `agent-run.ps1` parses; `run --unit` → 374/374. Clean stdout(JSON)/stderr(tracing) split.
