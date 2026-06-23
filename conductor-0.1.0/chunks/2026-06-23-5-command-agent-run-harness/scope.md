# Scope — 5-command agent-run harness

**Marker:** `2026-06-23-5-command-agent-run-harness`
**Version:** conductor-0.1.0 · Epoch 8 (CLI surface) · ch2/5
**Working-route intent:** "5-command agent-run harness — boot=preflight, run=nextest+scenarios, status=runs.db/JSONL read, cleanup=idempotent, logs=journal (.sh + .ps1)"

## What it builds
Flesh out the `scripts/agent-run.{sh,ps1}` **skeleton** (from `2026-06-16-base-ci-agent-run-harness-skeleton`) into the real **5-command agent-driven harness** — the headless **source-of-truth entrypoint and release gate** an agent (or operator) drives Conductor through. Each of the five subcommands becomes a thin, deterministic orchestration over the already-built `conductor-cli` binary (run/suite/report verbs, ch1) + the cargo test runner + the on-disk run artifacts. No engine logic is re-implemented in shell.

The five commands (the discipline):

- **boot** — run the readiness **preflight**: the MCP `initialize` gate (protocol `2024-11-05` · required-tool presence · data-dir canary) + the OTLP `:4317` egress liveness check; surface `ready` or the distinct **Blocked** state with its named precondition. No scenario emission, no mutation.
- **run** — the **release gate**: `cargo nextest run --workspace --profile ci` (+ the doctest pass nextest skips) **then** drive the scenario suite via `conductor run`/`suite`; machine-parseable result + deterministic exit code. This is the CI-green path `agent-run.sh run` already exercises in ch1 — formalized into the disciplined verb.
- **status** — read the latest run's state from `runs.db` (+ the per-run JSONL journal) and render it; **no mutation, no live Pulse required**.
- **cleanup** — **idempotent** teardown: release the `:4317` port-occupier if held, prune ephemeral per-run artifacts under the runs dir per policy; safe to run repeatedly and when nothing is running.
- **logs** — surface the per-run JSONL emission journal + the self-observation log lines for the latest (or a named) run.

Both `.sh` (bash/POSIX) and `.ps1` (PowerShell) carry the **same** five-command surface at parity — the Windows dev host and the CI Linux runner must drive Conductor identically.

**Note — the `conductor preflight` verb (surfaced at planning):** the skeleton's `boot` already shells out to `conductor preflight --json`, but ch1 shipped only `run`/`suite`/`report` — that verb does not exist yet. So this chunk's build **includes the thin `conductor preflight [--json]` verb** (a serialize-and-exit over the already-built readiness gate — reusing `ReadbackClient::connect` + `run_preflight` UNCHANGED), not just shell glue. It stays orchestration over the engine: a CLI-crate addition with **zero core/seam model change** (the `conductor-verify` readiness gate is consumed as-is). Per the P4 decision, `conductor preflight` is a **go/no-go gate** — exit 0 iff `ready:true`, non-zero on a `ready:false` Blocked precondition (test-plan §3); a genuine harness fault (bad manifest) is a separate non-zero `Err` path.

## Boundaries (what it does NOT do)
- Does **not** re-implement engine logic — it shells out to the `conductor` binary and `cargo nextest`; the harness is orchestration + artifact-read only.
- Does **not** manage the Pulse process (no start/stop of Pulse) — scope law; `boot` only *probes* readiness, never launches/kills the SUT beyond the existing hardened MCP sidecar spawn the preflight already owns.
- Does **not** open any inbound listener of Conductor's own — the `:4317` bind stays the port-occupier fault's sole deliberate bind; `cleanup` only *releases* a held occupier, never binds.
- Does **not** add a new engine dependency or change any core/seam model (zero model change expected, mirroring ch1).
- Adds **no** scenarios (no scenario without a P-ID; none introduced here).

## Surfaces / contracts it touches
- **5-command discipline** — `test-plan §3` (the test harness) + `.claude/rules/verification-harness.md`; the `boot/run/status/cleanup/logs` vocabulary + exit-code contract is what agents key off.
- **conductor-cli verbs** — invokes `conductor run` / `suite` / `report` (ch1) as subprocesses; consumes their exit-code + stdout contract.
- **Run artifacts** — reads `runs/runs.db` (SQLite index) + `runs/<run_id>.jsonl` journal for `status`/`logs`; honors `CONDUCTOR_RUNS_DIR`.
- **Readiness gate** — `boot` exercises the MCP preflight (`2024-11-05` · tools · canary) + OTLP egress liveness; **Blocked** surfaced distinctly, never silently downgraded to pass/fail.
- **Env handles** — `CONDUCTOR_*` (`RUNS_DIR` / `SCENARIOS_DIR` / `CONTRACT_MANIFEST` / `SEED`) + Pulse-side `ANDROMEDA_PULSE_MCP_ENABLED` / `ANDROMEDA_PULSE_DATA_DIR` consumed by `boot`.
- **Output discipline** — agent-parseable: deterministic exit codes + machine-readable status lines; **status is never color-alone** (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` ASCII prefixes on the CLI path).
- **Self-observation** — every harness action carries `run_id`; self-obs is `tracing` JSON to stdout/file (no OTLP from the harness itself).

## Acceptance intent (val-1 anchor)
The five subcommands exist and behave per the discipline in **both** `.sh` and `.ps1`:
- `boot` reports readiness (or **Blocked** + precondition) **without** emitting scenarios;
- `run` gates on `nextest` **then** drives scenarios, exit 0 on success (the ch1 CI-green path, formalized);
- `status` + `logs` read artifacts **without** a live Pulse;
- `cleanup` is **idempotent** (repeat-safe, no-op when nothing is held).

Source-of-truth check: `scripts/agent-run.sh run` exit 0 in CI (as ch1 achieves), plus the four other verbs wired with parity between `.sh` and `.ps1`. Zero core/seam model change.
