# obs extract

## Relevance
Partial — the chunk is a Tauri 2 GUI scaffold (frameless window shell); obs involvement is limited to the one API touch (`ObsSink` extension) and the Tauri backend self-obs sink routing.

## Constraints
- Per obs-plan §3 (Observability Harness Contract) — Tauri backend self-obs logs route to `logs/conductor-tauri.jsonl` via extended `ObsSink`; no OTel SDK init (§3: "NONE initialized or used for self-observation").
- Per obs-plan §3 (Service identity) — Tauri service identity must emit `service.name = "conductor-tauri"` (overrideable by `$CONDUCTOR_SERVICE_NAME` env).
- Per obs-plan §3 (Logging stack) — `tracing` 0.1.x + `tracing-subscriber` JSON formatter; dual sink (backend file `logs/conductor-tauri.jsonl` + stderr in dev mode); redaction at processor stage via `conductor-core::redact`.
- Per obs-plan §11 Anti-Patterns (Spans) — do NOT add W3C trace context (`tracing-opentelemetry` / `traceparent`) for self-observation; correlation is the `run_id` field only.
- Per obs-plan §11 Anti-Patterns (Universal) — do NOT export self-observation OTLP to `:4317` or `:4318`; self-obs is stdout/file/console JSON only; browser frontend produces NO OTel spans at all.
- Per obs-plan §11 Anti-Patterns (Logs) — do NOT leak absolute host paths or internal struct names; redaction layer masks host-file paths → `<redacted>`; internal struct names kept out by field-name allowlist + `Display`-not-`Debug` at `anyhow` edge.

## Patterns to follow
- Per obs-plan §3 (Service identity) — `service.name` hardcoded "conductor-tauri" or runtime override via `$CONDUCTOR_SERVICE_NAME`; `service.version` from `env!("CARGO_PKG_VERSION")`; `deployment.environment` from `$CONDUCTOR_ENV` env var.
- Per obs-plan §3 (Bootstrap phases) — the logger-stack and service-identity-wire are already installed (reused from `2026-06-15-structured-logging-stack`); Tauri backend extends the file sink only (`ObsSink` variant or parameterization, P4 implementation choice).
- Per obs-plan §4 (Span naming) — if/when Tauri commands are instrumented in later chunks, use `tauri.command.*` naming pattern (low-cardinality, bounded set); no high-cardinality names.

## Anti-patterns to avoid
- Do NOT initialize an OTel SDK for Tauri backend self-observation (breaks `current_thread` determinism and pollutes the PRODUCT stream).
- Do NOT add W3C trace context propagation (no `traceparent`, no `tracing-opentelemetry`) — correlation is `run_id` field only; both-surface parity is verified by `runs.db` envelope comparison, NOT trace correlation.
- Do NOT export self-observation OTLP anywhere (neither `:4317` PRODUCT stream nor `:4318` dead port); self-obs is file/stdout/console JSON only.

## Contract bindings
- **obs ↔ conductor_core::ObsSink** — the one cross-crate touch (per scope §Boundaries); CLI caller (`conductor-cli/src/main.rs`) must keep compiling after the `ObsSink` extension for Tauri sink selection.

## Acceptance criteria contributions
- (obs) Tauri backend `tracing` JSON logs route to `logs/conductor-tauri.jsonl` via extended `conductor_core::ObsSink`; redaction stays at processor stage; no new redaction policy added.
- (obs) Service identity on every Tauri JSON line: `service.name = "conductor-tauri"` (or `$CONDUCTOR_SERVICE_NAME` override); `service.version` from manifest; `deployment.environment` from `$CONDUCTOR_ENV` (default "local").
- (obs) Zero OTel SDK init for Tauri backend self-obs (determinism preservation); self-obs is `tracing`-to-file-JSON only.
- (obs) No W3C trace context / `traceparent` / `tracing-opentelemetry` in Tauri command handlers (future chunks); correlation and parity are `run_id`/`runs.db` envelope comparison.

## Relevant amendment history
- **2026-06-15-structured-logging-stack** — self-obs log line schema clarified: foundational line carries `timestamp_ms`, `level`, `target`, service-identity, `run_id` on every line; envelope/result fields appear only on scenario-result records. Custom `tracing-subscriber` layer (stock `fmt().json()` can't emit constant identity fields flat).
- **2026-06-24-sanitized-stderr-agent-mode-logging** — agent-mode reworded as read-only `CONDUCTOR_AGENT_MODE` trigger (read `flag || env-set`, never write); `ObsSink::Stderr` + new file sink (`ObsSink` variant) pattern established; redaction reuses unchanged processor-stage field-allowlist (no new redaction policy).