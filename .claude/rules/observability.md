---
paths:
  - "crates/**/src/**/*.rs"
  - "scripts/agent-run.*"
---

# Observability Rules

Path-scoped rules for self-observation (structured logging) across the seam crates. Conductor's self-obs is `tracing` JSON only — there is NO OTel SDK for self-observation (the only OTLP is the PRODUCT fault stream emitted AT Pulse).

**Authoritative source:** `.andromeda/obs-plan.md` — §3 Harness Contract, §6 Log Coverage, §10 SLO, §11 Anti-Patterns. The log JSON schema is owned by `.andromeda/test-plan.md` §3 — obs aligns to it, never the reverse.

## Logging
- **Library:** `tracing` + `tracing-subscriber` JSON formatter (`fmt().json().flatten_event(true)`); init at `main`/Tauri startup before any scenario logic.
- **Format:** JSONL, one object per line, via a small custom `tracing-subscriber` JSON layer (stock `fmt().json()` cannot emit constant identity fields flat at the top level). Two record shapes: the **self-obs base line** (every line) carries `timestamp_ms` (epoch millis, `std::time`), `level`, `target`, service-identity, `run_id`; the **Run-report envelope** (scenario-result record — emission journal `runs/<run_id>.jsonl`, report seam) carries `journal_emitted_at` (ISO-8601), `read_back_observed_at` (ISO-8601), `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints` (+ scenario extras like `bypass_triggered`).
- **`run_id` is the correlation key on every line** — there is no W3C `trace_id`/`traceparent` (no OTel SDK). Cross-surface parity is asserted by comparing the `runs.db` envelope (same seed ⇒ same verdict/state), not by trace correlation.
- **Sinks:** cli → stderr (dev) / `logs/agent-latest.jsonl` (`--agent-mode`); Tauri backend → `logs/conductor-tauri.jsonl`; Tauri frontend → browser `console.log` JSON only (NO browser OTLP exporter — recursion guard).
- Never log in a hot path at `info` (use `trace`/`debug` gated by `RUST_LOG=conductor_timeline=debug`). Never emit multi-line stack traces — serialize to one field.

## Service identity
- `service.name` = `"conductor"` / `"conductor-tauri"` / `"conductor-ui"` (override `$CONDUCTOR_SERVICE_NAME`); `service.version` = `env!("CARGO_PKG_VERSION")`; `deployment.environment` = `$CONDUCTOR_ENV` (default `local`). Emit as flat per-line JSON fields, NOT OTel resource attributes.

## Spans (plain `tracing`, rendered as JSON events — no exported traces)
- Bounded span-name set: `scenario.run`, `timeline.execute*`, `emit.batch`, `verify.readback*`, `report.generate`, `db.insert_run`, `fault.{silence,ramp,port_occupier}`, `tauri.command.*`. Never high-cardinality span names (per-user/per-trace/per-input).
- Cover the 7 must-trace critical paths (obs-plan §4); close every span at its phase boundary; instrument MCP read-back / rusqlite / tonic egress with a manual client span (these seams aren't auto-instrumented).

## Redaction & panics
- Field-allowlist + value scrub at the tracing-subscriber processor stage (`conductor-core::redact`), not just the sink: the allowlist DROPS non-allowlisted (incl. Debug-dumped) field names; the scrub masks absolute host-FILE paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) → `<redacted>` — NOT `::`-type tokens. The allowlisted `target` module path is preserved; internal struct names are kept out by the allowlist + `Display`-not-`Debug` at the `anyhow` edge.
- Zero unlogged panics: `std::panic::set_hook()` → `tracing::error!(panic=…)` (one-line JSON backtrace) → `anyhow::Error` at the binary edge. NEVER a retry-once policy (masks failures).

## Hard bans
- NEVER introduce an OTel SDK / exporter / `tracing-opentelemetry` / `traceparent` for self-observation (breaks `current_thread` determinism + pollutes the PRODUCT OTLP stream).
- NEVER export self-obs OTLP — not to `:4317` (the PRODUCT stream) and not to `:4318` (unused/dead). NEVER add Sentry/Datadog/Grafana-only consumption.
- NEVER stamp the journal from tokio's virtual clock; NEVER leak host paths / struct names into logs / `runs.db` / report.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-06-17: The Run-report envelope JSONL schema is OWNED by test-plan §3 — obs-plan §3 + a11y-plan §3 reproduce it verbatim and CAN drift; implement/verify a field against the owner (test-plan §3), not a reproduction. (emission-journal-writer shipped obs-plan §3's 10-field outlier, missing `read_back_observed_at`; the envelope is 11 fields, `read_back_observed_at` second after `journal_emitted_at`.)
