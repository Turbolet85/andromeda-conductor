# obs extract

## Relevance
Partial — the chunk touches the Tauri backend instrumentation (command handlers for list/start/stop) and cross-surface state correlation (run_id envelope carry via Tauri IPC), but the core scenario-execution instrumentation (timeline/emit/verify/report spans) is unchanged and already implemented.

## Constraints
1. Per obs-plan §3: `#[tracing::instrument]` on each `#[tauri::command]` handler (`list_scenarios`, `list_suites`, `start_run`, `stop_run`); span nesting: command handler = parent, core operations = children (plain `tracing` JSON render, no OTel SDK).
2. Per obs-plan §3 Correlation: the Tauri command envelope carries the `run_id` (correlation key for the span hierarchy), NOT a W3C `traceparent`; cross-surface parity (CLI vs Tauri) is verified by `runs.db` envelope comparison (same seed ⇒ same verdict/state), NOT trace correlation.
3. Per obs-plan §3 Logging stack (Tauri backend): file `logs/conductor-tauri.jsonl` + stderr (dev only); no network OTLP exporter (recursion guard — Conductor IS an observer, must stay stdout/file-only).
4. Per obs-plan §4 Span naming: bounded set (`scenario.run`, `timeline.execute*`, `emit.batch`, `verify.readback*`, `report.generate`, `db.insert_run`, `tauri.command.*`); no high-cardinality span names (no per-scenario-id, per-P-ID, per-run-id as span name component).
5. Per obs-plan §11 Anti-Patterns (Universal): NEVER export self-observation OTLP to `:4317` or `:4318`; self-obs is stdout/file/console JSON only.
6. Per obs-plan §3 Harness Contract: service.name = `"conductor-tauri"` (compile-time `env!("CARGO_PKG_NAME")`); service.version and deployment.environment inherited from CLI (shared env var resolution).

## Patterns to follow
1. Per obs-plan §4 Must-trace: `tauri.command.start_scenario` (or `tauri.command.start_run` — naming per convention) as root span with attributes `run_id`, `seed`, `scenario`, `p_ids`; child spans (core path) inherit the same `tracing` context (no SDK, just plain `tracing::Span::current()` + JSON formatter render).
2. Per obs-plan §1 Critical paths: Path 7 (Both-surface parity) requires identical log JSON schema (`journal_emitted_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`) on both CLI and Tauri runs; the envelope carries `run_id` for matching runs by `(scenario, seed)`.
3. Per obs-plan §6 Log format: JSONL with required fields on every line; list/start/stop command logs are boundary events (info level: method name + latency_ms + result status + error if any).

## Anti-patterns to avoid
1. NEVER spawn an OTel SDK in the Tauri backend (determinism break + recipe violation); the command handlers use `#[tracing::instrument]` (plain spans → JSON lines), never OTel SDK init.
2. NEVER add W3C `traceparent` context propagation between command handler and core runs (obs-plan §3: correlation is `run_id` field, not distributed tracing); the core path already runs deterministically on the `current_thread` runtime with `tracing::Span::current()` inherited.
3. NEVER export self-observation OTLP from the Tauri frontend (recursion + determinism guard per obs-plan §1 Tauri surface): Tauri backend exporter is file/stderr only; Tauri frontend produces console JSON only (no OTel JS SDK, no network `:4317` / `:4318` export).

## Contract bindings
- **Tauri IPC ↔ core boundary (obs-plan §3 ipc-internal):** the command envelope carries `run_id` as the correlation key; the span hierarchy (command handler parent → core operations child) is rendered as plain JSON log events, no OTel SDK between them.
- **CLI vs Tauri parity gate (obs-plan §4 Critical Path 7):** both surfaces produce identical JSONL (same schema, same `(scenario, seed)` matching logic); the `runs.db` envelope comparison verifies parity (no distributed-trace correlation needed).

## Acceptance criteria contributions
1. (obs) Tauri command handlers (`list_scenarios`, `list_suites`, `start_run`, `stop_run`) are instrumented with `#[tracing::instrument]` spans; each span logs its method name + latency_ms + result status (OK / error) at info level to `logs/conductor-tauri.jsonl`.
2. (obs) A `start_run` call sets the `run_id` on the command-handler span and propagates it (via `tracing::Span::current()`) to all child core operations (timeline/emit/verify/report); both-surface parity (CLI vs Tauri for the same seed) is verified by matching rows in `runs.db` by `(scenario, seed)` + asserting identical `verdict`/`state`.
3. (obs) No OTel SDK initialization in Tauri command handlers or the frontend; self-observation remains tracing JSON only (file `logs/conductor-tauri.jsonl` + stderr in dev).
4. (obs) The run_id envelope field is preserved and logged on every JSONL line; the Tauri command handler's span carries `run_id`, `seed`, `scenario`, `p_ids` attributes per obs-plan §4 must-trace (Scenario 1 / Path 7).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3 Harness Contract): clarified that self-obs log lines carry a base set (`timestamp_ms`, `level`, `target`, service-identity, `run_id`) on every line; the Run-report envelope (verdict/latency/state) is added at report-generation time by the seam (Epoch 6, not this chunk). Implementation uses a custom `tracing-subscriber` layer. — This chunk inherits that foundation for the Tauri backend file sink; no new redaction policy needed.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§3 agent-mode flag): agent mode is triggered by `--agent-mode` CLI flag OR `CONDUCTOR_AGENT_MODE` env, a **read-only trigger** Conductor reads (never writes). Observable mode is identical. — The Tauri backend observes `CONDUCTOR_ENV` + `CONDUCTOR_SERVICE_NAME` from the parent shell; no new env var needed for Tauri.
