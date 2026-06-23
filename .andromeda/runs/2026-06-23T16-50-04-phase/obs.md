# obs extract

## Relevance
Partial — CLI surface adds logging instrumentation and log-format conformance but defers pretty rendering, agent-mode JSON sink, and advanced telemetry surfaces to later chapters.

## Constraints
1. No OTel SDK for self-observation; `tracing` + `tracing-subscriber` JSON formatter only (obs-plan §3 OTel SDK init)
2. CLI must initialize observability stack BEFORE scenario logic: `conductor_core::init_observability("conductor", …)` first in `main()` (obs-plan §3 Bootstrap phases / Service identity)
3. JSONL log format matches the Run-report envelope schema (11 fields: journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) per obs-plan §6 Log Coverage (amended 2026-06-16)
4. Log file sink: stdout in dev mode, `logs/agent-latest.jsonl` in `--agent-mode` (obs-plan §3 Log file location)
5. Span naming convention: `{module}.{operation}` pattern (e.g., `scenario.run`, `timeline.execute`, `emit.batch`); no high-cardinality names (obs-plan §4 Span naming convention)
6. Exit-code discipline: 0 on all `Pass` verdicts; non-zero ONLY on hard `Fail`; `Blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` are reported states, not exit errors (scope.md §Definition of done)

## Patterns to follow
1. Instrument `scenario.run` (root span) with attributes: `run_id`, `seed`, `scenario`, `p_ids` (array); child spans for `timeline.execute`, `emit.batch`, `verify.readback`, `report.generate`, `db.insert_run` close at phase boundaries (obs-plan §4 Headless scenario path)
2. Error boundary: use anyhow edge with sanitized `Display` (no host paths / struct names); raw error display deferred to agent-mode `error:`/`hint:` format in ch5 (scope.md §Basic error edge)
3. Prefix output with ASCII status (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) per status-never-color-alone invariant; full pretty rendering deferred to ch3 (scope.md §Functional skeleton)
4. `#[tracing::instrument]` on CLI command handlers and core entry points; spans nest under CLI handler parent (obs-plan §4 Table: Manual instrumentation / CLI)

## Anti-patterns to avoid
1. NO OTel SDK init or batched exporter (determinism breakage on `current_thread` runtime) — use `tracing` spans only (obs-plan §3 OTel SDK init)
2. NO W3C `traceparent` or `trace_id` propagation; correlation within a run is via `run_id` field, not distributed tracing (obs-plan §3 Correlation / obs-plan §1 CLI surface notes)
3. NO unbounded span names derived from user input (e.g., per-P-ID spans, per-scenario-name spans); use fixed `{module}.{operation}` names only (obs-plan §4 Span naming convention)
4. NO TTY auto-detection for ANSI color gating (deferred to ch5 sink design); ch1 prints plain status + sanitized error messages only (scope.md §Basic error edge)

## Contract bindings
- Run-report envelope contract (arch §Standard Contracts / test-plan §3): run/suite PRODUCE the 11-field JSONL envelope; report CONSUMES it. Binding: shape must match per obs-plan §3 Log format JSON schema (amended 2026-06-16, now includes `read_back_observed_at`).
- Test harness contract (obs-plan §3 Bootstrap phases / notes on binding): downstream test assertions consume the structured log format and exit-code discipline; CLI format must match tests' binding schema.

## Acceptance criteria contributions
1. (obs) CLI logs in JSONL per run-report envelope schema (11 fields: journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints); output to `logs/agent-latest.jsonl` in `--agent-mode`, stdout in dev.
2. (obs) Root `scenario.run` span + child spans (`timeline.execute`, `emit.batch`, `verify.readback`, `report.generate`, `db.insert_run`) instrumented with `#[tracing::instrument]`; attributes match obs-plan §4 span attribute table.
3. (obs) Exit-code discipline: 0 on all `Pass` verdicts; non-zero ONLY on hard `Fail`; `Blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` reported as envelope states, not exit codes.
4. (obs) Error output sanitized at anyhow edge (no absolute host paths, no internal struct names); ASCII status prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) on output.

## Relevant amendment history
**2026-06-16-emission-journal-writer:** Run-report envelope schema now 11 fields (added `read_back_observed_at` ISO-8601 after `journal_emitted_at`, null until read-back, null for blocked rows). Applies directly: this chunk's JSONL output must carry the updated envelope (obs-plan §3 §6 all reproductions realigned).

**2026-06-15-log-error-boundary-redaction:** Redaction model clarified — scrub absolute host-file paths (drive-letter, `/home`, `/Users`, etc.) → `<redacted>`; struct-name guard is allowlist + `Display`-not-`Debug` at edge, NOT `::`-token redaction. Applies: error messages use `anyhow::Error` `Display` (not `Debug`); any log field scrubbing deferred to ch5 agent-mode sink.

**2026-06-15-structured-logging-stack:** Self-obs log line carries base set (timestamp_ms, level, target, service-identity, run_id) on every line; envelope/result fields (verdict, latency_ms, etc.) appear only on scenario-result events. Applies: CLI logs initialize via `conductor_core::init_observability` (foundation chunk); this chunk wires the engine seams and PRODUCES the envelope on completion, but the service-identity base and `run_id` are constants from the observability stack initialization.