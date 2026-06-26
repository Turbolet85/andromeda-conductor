# obs extract

## Relevance
Partial — chunk adds Tauri Channel infrastructure + real run execution; requires proper obs instrumentation for command handlers + persistence (CLI↔Tauri parity).

## Constraints
1. Per obs-plan §1 Tauri desktop-webview surface: `#[tracing::instrument]` on `#[tauri::command]` handlers (`start_run`, `stop_run`); IPC envelope carries `run_id` (not W3C traceparent)
2. Per §3 Service identity: Tauri backend `service.name` = `"conductor-tauri"` (compile-time `env!("CARGO_PKG_NAME")`), version from manifest, `deployment.environment` from `$CONDUCTOR_ENV` env var
3. Per §3 Logging stack: backend uses `tracing` + `tracing-subscriber` JSON formatter; dual sink (file `logs/conductor-tauri.jsonl` + stderr dev-only); JSON NDJSON format per binding schema
4. Per §3 Agent-mode flag: read-only trigger (`--agent-mode` flag OR `CONDUCTOR_AGENT_MODE` env), never written by Conductor
5. Per §1 Heartbeat: optional 30s `conductor.tick` via Channel (unstructured counter data for UI, NOT telemetry spans)
6. Per §3 No OTel SDK: behavioral invariant — core-owned `current_thread` runtime preserves determinism; no batch exporters

## Patterns to follow
1. Per §2, §4 Span naming: `{module}.{operation}` pattern (e.g., `tauri.command.start_scenario`, `tauri.command.stop_scenario`) with low-cardinality attributes
2. Per §3 Logging bootstrap: `tracing-subscriber::fmt().json().flatten_event(true).init()` at Tauri startup before any scenario logic
3. Per §3 JSONL envelope persistence: 11-field schema (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) to runs.db + `logs/conductor-tauri.jsonl`
4. Per §1, §3 Frontend JSON-only: browser console `console.log(JSON.stringify(…))` sink; NO OTel JS SDK, NO network OTLP exporter (recursion guard)
5. Per §3 Correlation: `run_id` in IPC envelope + JSONL lines; parity verified by runs.db envelope comparison (same seed ⇒ same verdict/state), NOT W3C trace context

## Anti-patterns to avoid
1. Per §3 Hard bans: DO NOT initialize OTel SDK for self-observation (breaks `current_thread` determinism; opentelemetry-proto transitive presence is acceptable, but uninit)
2. Per §4: DO NOT use high-cardinality span names (no per-user-ID, per-path; no unbounded labels on attributes)
3. Per §3 Correlation: DO NOT propagate W3C `traceparent`; use `run_id` envelope-only correlation

## Contract bindings
- **Tests harness contract** (tests excerpt §3, obs §3 Log format JSON schema) — tests bind to JSONL envelope shape + runs.db verdict/state/latency fields; the 11-field schema is the test-plan's ground truth (obs derives, does not re-author)
- **CLI↔Tauri parity** (test-plan Path 7, scope.md § Boundaries) — both surfaces emit identical verdicts for same seed (verified by runs.db envelope comparison, not trace correlation)

## Acceptance criteria contributions
1. (obs) Tauri `start_run` / `stop_run` command handlers instrumented with `#[tracing::instrument]` spans carrying attributes: `run_id`, `seed`, `scenario`, `p_ids`
2. (obs) JSONL log lines for scenario results emitted with all 11 envelope fields; service identity fields (`service.name`, `service.version`, `deployment.environment`) on every line
3. (obs) Run records persisted to `runs.db` + `logs/conductor-tauri.jsonl` with envelope matching test-plan binding schema (per obs §3 — ground truth owned by tests excerpt §3)
4. (obs) Frontend console JSON only — no OTel JS SDK, no OTLP exporter (recursion guard per §1 Tauri desktop-webview scope)

## Relevant amendment history
1. **2026-06-15-structured-logging-stack** — clarified two record shapes: self-obs base line (every line: timestamp, level, target, service identity, run_id) vs Run-report envelope (scenario-result record with verdict/latency/state); this chunk's persistence uses the envelope shape
2. **2026-06-16-emission-journal-writer** — Run-report envelope revised to 11 fields (added `read_back_observed_at` post-`journal_emitted_at`); directly applicable to Tauri run persistence in this chunk
3. **2026-06-24-sanitized-stderr-agent-mode-logging** — agent-mode flag clarified as read-only trigger (Conductor reads, never writes); relevant for CLI↔Tauri logging parity in this chunk