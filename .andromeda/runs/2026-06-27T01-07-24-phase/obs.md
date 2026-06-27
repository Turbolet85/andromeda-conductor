# obs extract

## Relevance
Partial — surfaces existing observability data (RunRecord envelope) via new Tauri command handlers; no new instrumentation triggers or envelope extensions required, but Tauri IPC boundary instrumentation is mandatory.

## Constraints
- Per §3 Harness Contract (Tauri backend): `#[tracing::instrument]` on all `#[tauri::command]` handlers (run-report query + operator-checklist query) — spans inherit parent `tracing` context (no OTel SDK)
- Per §1 Telemetry surfaces (Tauri backend): RunRecord surface is IPC-internal; envelope carries `run_id` for correlation (not W3C `traceparent`)
- Per §3 Log format JSON schema: RunRecord envelope is locked at 11 fields (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints); this chunk deserializes, not modifies
- Per §6 Boundary-call wrappers: Tauri command handlers are synchronous IPC (simpler logging than MCP/gRPC boundaries); parent Tauri backend sink applies service identity + run_id automatically
- Per §4 Span naming: follow `{module}.{operation}` pattern; if new spans are added, use `tauri.command.{action_name}` (§1 Telemetry surfaces shows Tauri handler pattern)

## Patterns to follow
- Per §3 Tauri backend instrumentation (IPC-internal): command-handler span is root of any core-operation children; the IPC envelope carries `run_id` as the correlation key (no W3C trace propagation needed for single-process boundary)
- Per §3 Bootstrap phases / Log format: self-obs log lines already carry service-identity + run_id via parent Tauri backend sink; no new constants needed
- Per §1 Telemetry surfaces / Tauri backend: reuse existing `#[derive(Serialize)]` on conductor-report types (RunRecord, OperatorChecklist model); deserialization via `#[tauri::command]` return type is the pattern (carried from coverage-matrix-view)

## Anti-patterns to avoid
- Per §11 Spans (bounded set): do not invent span names outside `{module}.{operation}` pattern (no per-scenario, per-fingerprint, or per-P-ID variants — the query is a single op)
- Per §11 Logs: do not re-serialize or transform RunRecord fields; field allowlist + redaction already applied at report-generation time (Epoch 6) — surfacing as-is preserves allowlist integrity
- Per §11 PII Scrubbing: do not add new redaction logic in the Tauri layer; redaction is terminal at report.generate (§3 field-allowlist scope); the envelope is already scrubbed

## Contract bindings
- **Tauri command → conductor-core boundary (IPC):** envelope carries `run_id` as correlation key; parent `tracing` span context available via `tracing::Span::current()` (single-threaded `current_thread` runtime)
- **RunRecord type sourcing:** `conductor-core` + `conductor-report` (re-export); `#[derive(Serialize)]` already applied, no TS re-authoring of shape (per §3 data-sourcing pattern carried from ch6)
- **OperatorChecklist type sourcing:** scenario model (conductor-core); same Serialize pattern applies

## Acceptance criteria contributions
- "(obs) Tauri run-report query command spanned with `#[tracing::instrument]` at handler boundary; span name follows `{module}.{operation}` pattern (e.g., `tauri.command.get_run_report`)."
- "(obs) RunRecord envelope fields (all 11: journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) pass through deserialization unmodified — no field transformation or scrubbing at Tauri layer."
- "(obs) Operator-checklist query command similarly instrumented; correlation via parent `run_id` (from enclosing scenario context, if applicable)."
- "(obs) Service identity (service.name / service.version / deployment.environment) inherited from parent Tauri backend sink; no new sink configuration needed."

## Relevant amendment history
- **2026-06-16-emission-journal-writer:** RunRecord envelope now 11 fields with `read_back_observed_at` added (ISO-8601, null until read-back, null for blocked rows). Obs §3 schema realigned to test-plan owner (§3 Log format JSON schema block); this chunk surfaces the canonical 11-field shape via Tauri deserialization.
- **2026-06-24-sanitized-stderr-agent-mode-logging:** Tauri backend logging sink clarified as read-only CONDUCTOR_AGENT_MODE trigger (agent mode = flag || env-set, never written). No impact on command-handler instrumentation; parent sink configuration carries forward.
