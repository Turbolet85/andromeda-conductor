# obs extract

## Relevance
Partial — adds a new Tauri command surface and IPC signal path (instrumentation) but does not introduce live hold-emission logic or new critical paths; the observable operation (operator decision → run resume/abort) is bounded within existing scenario execution.

## Constraints
- **§1 Instrumentation scope:** `conductor-tauri` surface is instrumentable; new `#[tauri::command]` handlers require a MANUAL `let _span = tracing::info_span!("tauri.command.<name>").entered();` (per obs §4 + 2026-06-26 rule: the `#[tracing::instrument]` ATTRIBUTE does not stack cleanly with `#[tauri::command]`).
- **§3 Service identity:** `service.name` hardcoded `"conductor-tauri"`; `service.version` via `env!("CARGO_PKG_VERSION")`; `deployment.environment` from `$CONDUCTOR_ENV` env var (default `"local"`).
- **§3 Logging stack:** dual sink for Tauri backend (file `logs/conductor-tauri.jsonl` + stderr in dev); `--agent-mode` forces JSON-only to file (read-only `CONDUCTOR_AGENT_MODE` env trigger per amendment 2026-06-24).
- **§1 & §3 Correlation model:** IPC envelope carries **`run_id`** as correlation key (NOT W3C traceparent or trace_id); command-handler span is parent of core-operation spans; cross-surface parity verified by runs.db envelope comparison (same seed ⇒ same verdict/state), not trace propagation.
- **§3 Log format JSON schema (amended 2026-06-16):** Every log line carries `run_id`, `service.name`, `service.version`, `deployment.environment`; Run-report envelope on scenario-result events includes 11 fields (`journal_emitted_at`, `read_back_observed_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`).
- **§3 Redaction:** errors returned to the webview go through `conductor_core::sanitize_error` (Display-not-Debug + host-path scrub); allowlisted log fields only (`count`/`latency_ms` already in the redact allowlist); any new field (e.g. `decision`) must be allowlist-safe and never carry a host path.

## Patterns to follow
- **§4 Span naming:** bounded `tauri.command.*` set (e.g., `tauri.command.resolve_operator_hold`); never high-cardinality span names.
- **§4 Span attributes:** `run_id` in context; an allowlist-safe `decision` (Go/NoGo) field on the resolution boundary line if logged.
- **§3 IPC-internal:** span nesting parent = command handler, child = core operations (plain `tracing` span hierarchy rendered as JSON log events — no OTel SDK, no W3C trace context).

## Anti-patterns to avoid
- **§3 Correlation invariant:** Do NOT propagate W3C `traceparent` or generate `trace_id` for the IPC or hold-resolution operation (no OTel SDK, no distributed tracing in Conductor self-obs).
- **§3 No frontend OTel export (recursion guard):** The Tauri frontend may log the hold-open state to console, but any such logs are console-JSON-only, never exported as OTLP at `:4318` or elsewhere.
- **§1 Hold-emission boundary:** The chunk does NOT emit `HoldPoint` into the scenario (live-Pulse hold logic deferred to Epoch 10); thus, no new critical path is added. A seeded/constructed hold used for demonstration must carry the `run_id` and not pollute the verdict envelope prematurely.

## Contract bindings
- **obs ↔ tests harness:** structured log format binding contract from tests (§3 notes this); the new Tauri command should emit logs matching the schema (service.name, run_id, level, timestamp, message fields).
- **obs ↔ a11y harness (Epoch-9 Desktop a11y chunk, ch9):** the operator-pause dialog's `onProceed`/`onAbort` event→command→decision flow should log the decision outcome; no new a11y schema required (scope defers a11y integration to ch9).

## Acceptance criteria contributions
- "(obs) Resolve `#[tauri::command]` carries a manual `tauri.command.<name>` span with `run_id` in context; the resolution boundary `info!` line carries an allowlist-safe `decision` (Go/NoGo) field."
- "(obs) Hold-resolution boundary log appears in `logs/conductor-tauri.jsonl` (agent mode) / stderr (dev) with `service.name="conductor-tauri"`; no host path / struct name leaks (sanitize_error / allowlist)."
- "(obs) No OTLP export from Tauri frontend or IPC layer; hold-signal backend→frontend carries `run_id` in envelope only."

## Relevant amendment history
- **2026-06-26-live-counter-channel-stream / scenario-suite-picker-start-stop (§4 instrumentation note):** A `#[tauri::command]` is instrumented with a MANUAL `tracing::info_span!("tauri.command.<name>").entered()` (NOT the `#[tracing::instrument]` attribute, which the command macro's signature rewrite breaks); errors to the webview go through `sanitize_error`; `count`/`latency_ms` are already allowlisted. The resolve command follows this exact pattern.
- **2026-06-24-sanitized-stderr-agent-mode-logging (§3 logging stack):** `--agent-mode` is a read-only trigger (`agent_mode = flag || env-set`; Conductor reads, never writes). The new Tauri command handler respects this: JSON-only to `logs/conductor-tauri.jsonl` in agent mode; JSON + stderr in dev.
- **2026-06-16-emission-journal-writer (§3 envelope schema):** Run-report envelope now 11 fields including `read_back_observed_at`. The hold-resolution must not prematurely pollute or reshape this envelope.
