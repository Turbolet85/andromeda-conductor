# Scope — Structured logging stack

**Marker:** 2026-06-15-structured-logging-stack
**Version:** conductor-0.1.0 · **Epoch:** 1 — Foundation (chunk 5/9)
**Working-route line:** _Structured logging stack — tracing + tracing-subscriber JSON (no OTel SDK), service-identity fields, std::panic::set_hook capture_

## What it builds
The self-observation logging foundation for Conductor: a `tracing` + `tracing-subscriber`
stack that emits structured **JSON** self-observation logs to stdout (and/or a file), and is
explicitly **NOT** an OpenTelemetry SDK. It establishes:

- **A subscriber init surface** — a runtime-agnostic entrypoint that installs the global
  `tracing` subscriber, callable identically from the headless `conductor-cli` path (the
  release gate) and the optional `conductor-tauri` shell (headless-drivable core, thin shells).
- **JSON-formatted log lines** — machine-parseable self-obs ground truth (the agent-parseable
  log stream), per the obs-plan log JSON schema (§3/§6).
- **Service-identity fields** — `service.name` (+ version / identity fields) stamped on every
  line so logs are attributable to Conductor the harness.
- **`run_id` on every line** — the cross-cutting correlation key the obs invariant mandates
  ("every line carrying `run_id`"), threaded so all self-obs for a run shares it.
- **`std::panic::set_hook` capture** — a panic hook that routes every panic into the structured
  log stream, satisfying the **zero-unlogged-panics** invariant.

## Boundaries / non-goals
- **Self-observation ONLY.** This path emits NO OTLP and uses NO OTel SDK. The only OTLP is the
  PRODUCT fault stream to Pulse `127.0.0.1:4317` (Epoch 3 — Emission primitives), a wholly
  separate egress. Conflating the two is the precise anti-pattern this chunk guards against.
- **Redaction is the NEXT chunk.** Field-allowlist redaction + `anyhow`-edge sanitization (no
  host-paths / internal struct-names / stack-traces leaking into logs or artifacts) belongs to
  _Log + error-boundary redaction_. This chunk stands up the stack; the allowlist/sanitization
  layer is composed on top afterward. Build the seam so it can attach, but do not implement it here.
- **File sink deferred (lean scope — P4 decision).** This chunk emits JSON to **stderr** and parameterizes the
  subscriber's writer so the file sink (`logs/agent-latest.jsonl` under `CONDUCTOR_RUNS_DIR`) + `--agent-mode`
  selection attach when the Epoch 8 agent-run harness lands (it owns the runs dir + the clap flag). No network
  sink / log shipping ever — stdout/stderr + local file only (local-only tool, no cloud).
- **Not the emission journal or run-report envelope** — the per-run JSONL emission journal and
  the Markdown/`runs.db` run report are distinct artifacts (report seam / Epoch 6 + the
  Emission-journal chunk), with their own schema. Those wall-clock stamps come from
  `std::time`, never tokio's virtual clock — relevant only insofar as self-obs must not be
  confused with them.

## Surfaces / contracts touched
- **Obs invariant** "Self-obs never exports OTLP … `tracing` JSON to stdout/file (no OTel SDK),
  every line carrying `run_id`; zero unlogged panics (`std::panic::set_hook`)" — this chunk is
  its primary realization.
- **Obs-plan §3 / §6** — the log JSON schema (field set, service-identity, line shape).
- **A shared init surface** both binaries call identically — candidate home is the
  runtime-agnostic core (`conductor-core`) so the CLI and Tauri shells stay thin; exact crate /
  module path to be fixed by codebase research + the plan.
- **New dependency edges** — `tracing 0.1.44` + `tracing-subscriber 0.3.23` (JSON feature) into
  the owning crate(s); must keep `Cargo.lock` committed + un-drifted and the cargo-audit/deny
  gate green (the supply-chain invariant from the prior chunk).

## Definition of done (intent level)
Initializing the stack produces JSON log lines carrying service-identity + `run_id`; a panic is
captured as a structured line rather than an unlogged crash; no OTel SDK is pulled in; the
regression gate (`cargo nextest run --workspace`, clippy, audit/deny) stays green with the lock
un-drifted.
