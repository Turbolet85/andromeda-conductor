# Scope — conductor run/suite/report verbs

**Chunk:** `2026-06-23-conductor-run-suite-report-verbs`
**Epoch:** 8 (CLI surface) · chunk 1/5 — **opens Epoch 8; the first real `conductor-cli` build-out**
**Working-route intent:** _"conductor run/suite/report verbs — clap CLI over current_thread bootstrap"_
**Seam:** `conductor-cli` (the `conductor` binary — headless source of truth + release gate)

## What it builds
The clap-derive command surface for the `conductor` binary + its `#[tokio::main(flavor="current_thread")]`
bootstrap + the anyhow binary edge, wiring the already-built engine seams (timeline · emit · verify · report)
behind three scenario-execution verbs:

- **`conductor run <scenario|P-ID> --seed <s>`** — resolve one scenario from `scenarios/` (serde+garde via
  `Scenario::from_toml_str`), drive the seeded `conductor-timeline` on the current_thread runtime, emit its
  OTLP stream via `conductor-emit` to `127.0.0.1:4317` (gated by the egress liveness check), run MCP read-back
  verification via `conductor-verify` when the preflight gate is ready (else `Blocked`), classify into the
  `Verdict`/`ReportState` envelope, and persist it through `conductor-report` (JSONL journal + `runs.db` row +
  Markdown report). `--seed` overrides `CONDUCTOR_SEED` / the scenario's own seed.
- **`conductor suite [--filter …]`** — run a set of scenarios (all of `scenarios/`, or a filtered subset)
  sequentially over ONE current_thread runtime + ONE preflight gate, aggregating the per-scenario envelopes;
  exit non-zero on any hard `Fail`.
- **`conductor report [<run_id>]`** — render an existing run's canonical envelope from the persisted artifacts
  (`runs.db` / JSONL) to stdout (read-only; the latest run when `<run_id>` is omitted).

Exit-code discipline (test-plan §3 / verification-harness): exit 0 = all `Pass`; non-zero ONLY on a hard
`Fail` (nextest-aligned). `Blocked` / `ManualCheck` / `KnownResidual` / `CalibrationRegion` are reported
envelope states, NOT non-zero exits.

## Boundaries
- **Functional skeleton, not polished output** — output here is plain + minimal; the
  owo-colors/indicatif/comfy-table line-oriented rendering + coverage table is **ch3**
  (`Line-oriented output rendering`). ch1 prints the envelope in a simple, test-anchorable form with the ASCII
  status prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) per the status-never-color-alone invariant.
- **Headless-default operator-pause** — `conductor run`/`suite` use the shipped headless never-block default
  (`2026-06-21-operator-pause-orchestration`); the isatty-gated `inquire` interactive pause + spinner mirror is
  **ch4**.
- **Basic error edge, not the agent-mode log sink** — anyhow at the binary edge with sanitized `Display` (no
  host paths / struct names); the `error:` / `hint:` stderr format + `--agent-mode` JSON-to-file journal sink is
  **ch5**.
- **No shell harness wiring** — the 5-command `scripts/agent-run.{sh,ps1}` (boot/run/status/cleanup/logs) that
  CALLS these verbs is **ch2**. ch1 builds the verbs; ch2 orchestrates them from the shell.
- **`preflight` / `boot` verb deferred to ch2** (boundary to confirm at P4) — the chunk title names
  run/suite/report. `conductor run`/`suite` CONSUME the existing preflight gate
  (`2026-06-21-preflight-readiness-gate`) to decide ready-vs-`Blocked`; whether a standalone
  `conductor preflight --json` subcommand lands here or with ch2's `boot` is a P4 scope question (lean: ch2,
  since `boot=preflight` is ch2's first responsibility).
- **Live execution is operator-gated** — a full `conductor run` needs a live Pulse on `:4317` (egress) + the
  `mcp-server` feature (read-back); per the established gate, CI/tests use `assert_cmd` + the rmcp stub /
  no-target paths, and the live leg stays local/operator-only. ch1 does NOT add a live-Pulse CI gate.
- **Coarse live emission, faithful bridges deferred (P4 decision — intent refinement)** — research surfaced that
  the seams are mutually independent, so two content bridges are unbuilt and live-only: (1) `EmissionSpec`
  carries only a coarse `Signal` class (never wired to the rich emit builders), and (2) there is no
  per-`ExpectedCheck` read-back observed-extraction. The P4 AskUserQuestion resolved to **coarse live + full
  Blocked path**: ch1 wires the FULL orchestration with COARSE `Signal`-class emission + a basic read-back; the
  faithful per-scenario emission + per-check extraction are deferred to **Epoch-10** ("Live-Pulse E2E proof",
  where they are MCP-verified). The CI-tested spine is the no-Pulse → `Blocked` path + the three verbs +
  persist/report.
- **No new engine logic** — the timeline / emit / verify / report seams are WIRED, not extended; no new
  `conductor-core` types, no new emission primitive, no new verdict logic. New code is CLI-crate-local (clap
  structs + the run/suite/report handlers + the bootstrap + the env-handle canonicalize edge).
- **Scope law** — Conductor opens no new inbound listener (the only deliberate bind remains the Epoch-4 `:4317`
  port-occupier fault); the CLI is a gRPC/MCP client + an artifact reader/writer.

## Surfaces / contracts touched
- `crates/conductor-cli/Cargo.toml` — add `clap` (derive), `tokio` (`current_thread`/`macros`/`time`),
  `anyhow`, `tracing`, plus the engine-seam deps it wires (`conductor-timeline`, `conductor-emit`,
  `conductor-verify`, `conductor-report`) — currently only `conductor-core`. The CLI bin is allowed to depend
  on every library seam (forbidden cross-seam library→library edges still won't compile).
- `crates/conductor-cli/src/main.rs` — replace the init-only stub with the clap `Cli`/`Commands` parser +
  `#[tokio::main(flavor="current_thread")]` + dispatch; keep `conductor_core::init_observability("conductor", …)`
  FIRST, before any scenario logic.
- (likely) new CLI-crate modules — e.g. `src/cli.rs` (clap structs) + `src/commands/{run,suite,report}.rs`
  (handlers); exact module shape decided at P4.
- The Run-report envelope contract (arch §Standard Contracts / test-plan §3) — run/suite PRODUCE it, report
  CONSUMES it. No schema change.
- `CONDUCTOR_*` env handles — `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SEED` (CLI `--seed`
  takes precedence); `std::fs::canonicalize` + bounds-check at the cli edge before any read/write (security
  §Input Validation).
- `crates/conductor-cli/tests/cli_smoke.rs` — extend the `assert_cmd` smoke into run/suite/report E2E coverage
  (brand anchors, exit codes; rmcp stub for the read-back leg; no live Pulse).

## Definition of done (acceptance intent)
- `conductor --help` lists `run`, `suite`, `report` (+ global flags); `conductor <verb> --help` documents each;
  a clap parse error exits non-zero with a sanitized message.
- The binary runs on `#[tokio::main(flavor="current_thread")]` (determinism invariant — no multi-thread
  work-stealing on the CLI path); `init_observability` is called before any scenario logic.
- `conductor run <P-ID> --seed <s>` drives the full timeline→emit→(verify | `Blocked`)→classify→persist
  pipeline and writes the canonical envelope to the JSONL journal + `runs.db` + Markdown report (run_id-stemmed,
  never overwritten); exit 0 on `Pass`, non-zero only on a hard `Fail`.
- `conductor suite` runs ≥2 scenarios over one runtime + one preflight, aggregates envelopes, exits non-zero on
  any hard `Fail`.
- `conductor report [<run_id>]` renders the persisted envelope read-only (latest when omitted).
- Output carries the ASCII status prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`); errors are sanitized at the
  anyhow edge (no host paths / struct names).
- `assert_cmd` E2E covers help / exit-codes / the three verbs (rmcp stub for read-back; no live-Pulse CI
  dependency); gates green (workspace nextest, clippy `-D`, doctest); `agent-run.sh run` exit 0; goldens
  UNCHANGED.
- Epoch-8 deferrals stated in the plan: pretty rendering (ch3), interactive pause (ch4), agent-mode log sink +
  `error:`/`hint:` format (ch5), shell harness (ch2), `preflight` verb (ch2, pending P4).
