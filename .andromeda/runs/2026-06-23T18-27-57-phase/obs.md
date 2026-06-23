# obs extract

## Relevance
Partial — the harness orchestrates CLI surface (observability hooks) + reads run artifacts (logs/runs.db) but does not implement new instrumentation depth.

## Constraints
1. Per §3 Observability Harness Contract, `boot` command emits readiness state without scenario emission or mutation — surfaces **Blocked** distinctly if MCP `initialize` gate or OTLP `:4317` liveness fails.
2. Per §4 Critical Path 1 (Headless deterministic scenario run), all five commands must preserve `run_id` as the correlation key; no distributed tracing or W3C `traceparent` — each harness action tags its output with `run_id`.
3. Per §6 Log Coverage, the run-report envelope (emission journal + runs.db) carries 11 required fields; `status` and `logs` commands must honor this binding contract when reading artifacts.
4. Per §3 Logging stack, harness output (both `.sh` and `.ps1`) follows the dual-sink rule: stderr pretty-print in dev, file `logs/agent-latest.jsonl` in agent mode; harness itself logs via `tracing` JSON on parent stdout (no ANSI color-coding; `--agent-mode` flag forces JSON-only).
5. Per §1 Instrumentation scope / CLI entity, `conductor-cli` is the boundary — harness must not duplicate logging; pass-through stdout/stderr of the binary to the operator or piped consumer (tests consume structured JSON).
6. Per §9 CI Integration, artifact handling surfaces log files to CI; harness `logs` command must render JSONL per jq-parseable schema, never lossy text transformation.
7. Per amendment 2026-06-16-emission-journal-writer, the Run-report envelope now carries 11 fields including `read_back_observed_at`; harness reads and respects this in `status` queries.

## Patterns to follow
1. **5-command discipline** — `boot` (probe readiness, surface **Blocked** + named precondition); `run` (nextest gate, then scenarios, deterministic exit 0); `status` / `logs` (artifact-read, no live Pulse); `cleanup` (idempotent, no-op when nothing held).
2. **run_id correlation without distributed tracing** — harness operations carry `run_id` on every output line; parity verification is `runs.db` envelope match (same seed ⇒ same verdict/state), not trace correlation.
3. **Agent-parseable output** — deterministic exit codes + ASCII `[PASS]/[FAIL]/[HOLD]/[BLOCKED]` prefixes on status lines (no color-alone); structured JSONL schema respected in `logs` output.

## Anti-patterns to avoid
1. Spawning or managing Pulse process — harness only probes readiness via MCP preflight + OTLP liveness, never starts/kills Pulse.
2. Re-implementing core scenario logic in shell — harness is orchestration over `conductor` binary and `cargo nextest`, not engine re-implementation.
3. Lossy log transformation — `logs` command outputs raw JSONL; any filtering/aggregation must preserve schema compliance + jq parseability.

## Contract bindings
- **Tests harness contract** (test-plan §3) — harness `boot/run/status/cleanup/logs` vocabulary + exit-code contract binds to tests' CI workflow; `run` verb must gate on `cargo nextest` pass before scenarios (tests consume JSON status envelope + artifact logs).
- **CLI binary contract** (conductor-cli ch1) — harness shells out to `conductor run` / `suite` / `report`; consumes their exit-code + stdout JSONL; must honor exit-code semantics.
- **Run artifact schema** (obs §3 + §6) — `status` and `logs` commands read `runs.db` + `runs/<run_id>.jsonl`; must respect 11-field envelope binding contract from amendment 2026-06-16.

## Acceptance criteria contributions
1. **(obs) Boot readiness gate surfaces Blocked state distinctly** — `boot` command runs MCP `initialize` preflight + OTLP `:4317` liveness check; if either fails, exits with **Blocked** marker + named precondition (not silent downgrade to pass/fail).
2. **(obs) Harness output is agent-parseable** — all exit codes deterministic; `status` lines carry ASCII `[PASS]/[FAIL]/[HOLD]/[BLOCKED]` prefixes; `logs` output is raw JSONL conforming to 11-field envelope schema.
3. **(obs) Artifact reads preserve run_id correlation** — `status` and `logs` commands read `runs.db` + JSONL without mutation; preserve `run_id` field on every output line for cross-surface parity verification.
4. **(obs) Cleanup is idempotent** — `cleanup` command can run multiple times without error; safe when nothing is held; releases port-occupier fault if present, prunes ephemeral artifacts per policy.

## Relevant amendment history
1. **2026-06-16-emission-journal-writer** — Run-report envelope now carries 11 fields including `read_back_observed_at` (ISO-8601; null until read-back); harness `status` and `logs` commands must read and respect this envelope in `runs.db` + JSONL journal queries.
