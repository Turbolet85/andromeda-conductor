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
- **Format:** JSONL, one object per line, via a small custom `tracing-subscriber` JSON layer (stock `fmt().json()` cannot emit constant identity fields flat at the top level). Two record shapes: the **self-obs base line** (every line) carries `timestamp_ms` (epoch millis, `std::time`), `level`, `target`, service-identity, `run_id`; the **Run-report envelope** (scenario-result record — emission journal `runs/<run_id>.jsonl`, report seam) carries `journal_emitted_at` (ISO-8601), `read_back_observed_at` (ISO-8601), `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints` (+ scenario extras like `degraded_mode_response`). The report seam also writes a **per-check `CheckRecord`** line onto the same journal (`run_id`, `scenario`, `check_index`, `kind`, `verdict`, `state`, `latency_ms`, `deadline_ms`, `budget_ms`) — a finer grain beneath the envelope, mirrored into the `run_check` table; the field allowlist governs the self-obs line only, so neither report-seam shape passes through it. The self-obs line comes in **two variants over that one base set**: the **event line**, and the **span-lifecycle line** adding `span` (a bounded span name), `span_event` (`new` | `close`), an optional `parent` (absent on a root), and the span's own allowlisted attributes on `new` — this is how a span becomes observable at all (the layer implements `on_event` + `on_new_span` + `on_close`).
- **`run_id` is the correlation key on every line** — there is no W3C `trace_id`/`traceparent` (no OTel SDK). Cross-surface parity is asserted by comparing the `runs.db` envelope (same seed ⇒ same verdict/state), not by trace correlation.
- **Sinks:** cli → stderr (dev) / `logs/agent-latest.jsonl` (`--agent-mode`); Tauri backend → `logs/conductor-tauri.jsonl`; Tauri frontend → browser `console.log` JSON only (NO browser OTLP exporter — recursion guard).
- Never log in a hot path at `info` (use `trace`/`debug` gated by `RUST_LOG=info,{crate}=debug` — a bare per-target directive REPLACES the default and silences every other target, so always pair it with the global level). Never emit multi-line stack traces — serialize to one field.

## Service identity
- `service.name` = `"conductor"` / `"conductor-tauri"` / `"conductor-ui"` (override `$CONDUCTOR_SERVICE_NAME`); `service.version` = `env!("CARGO_PKG_VERSION")`; `deployment.environment` = `$CONDUCTOR_ENV` (default `local`). Emit as flat per-line JSON fields, NOT OTel resource attributes.

## Spans (plain `tracing`, rendered as JSON events — no exported traces)
- Bounded span-name set: `scenario.run`, `timeline.execute*`, `emit.batch`, `emit.logs_batch`, `verify.readback*`, `report.generate`, `db.insert_run`, `fault.{silence,ramp,port_occupier}`, `tauri.command.*`. Never high-cardinality span names (per-user/per-trace/per-input).
- **Fault spans open where the fault actually is, at `info` like every sibling span** (obs-plan §4): `fault.silence`/`fault.ramp` come from `conductor-timeline`'s per-phase observer hook (`run_timeline_observed`/`PhaseWindow`) with `conductor-run` supplying the fault semantics and the `std::time` journal basis — silence and ramp are declarative phase data, NOT calls into `conductor-faults`. Create them, never ENTER them: entering re-parents that phase's `emit.batch` onto the fault span. `fault.port_occupier` is the one that lives in `conductor-faults` (RAII: opens at `occupy()`, closes at `release()`/`Drop`), carries `fault_type` + `port` only, and IS a child of `timeline.execute` — `conductor-run`'s fault-phase guard drives it for a fault-declared phase's window (measured live 2026-08-19), the RAII release being the scheduler's boundary drop.
- Cover the 7 must-trace critical paths (obs-plan §4); close every span at its phase boundary; instrument MCP read-back / rusqlite / tonic egress with a manual client span (these seams aren't auto-instrumented).

## Redaction & panics
- Field-allowlist + value scrub at the tracing-subscriber processor stage (`conductor-core::redact`), not just the sink: the allowlist DROPS non-allowlisted (incl. Debug-dumped) field names; the scrub masks absolute host-FILE paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) → `<redacted>` — NOT `::`-type tokens. The allowlisted `target` module path is preserved; internal struct names are kept out by the allowlist + `Display`-not-`Debug` at the `anyhow` edge.
- Zero unlogged panics: `std::panic::set_hook()` → `tracing::error!(panic=…)` (one-line JSON backtrace) → `anyhow::Error` at the binary edge. NEVER a retry-once policy (masks failures).

## Hard bans
- NEVER introduce an OTel SDK / exporter / `tracing-opentelemetry` / `traceparent` for self-observation (breaks `current_thread` determinism + pollutes the PRODUCT OTLP stream). The ban is on *initializing/using* an SDK for self-obs — a dormant `opentelemetry_sdk` pulled *transitively* by `opentelemetry-proto` (the PRODUCT proto lib) and never initialized is not a violation (a `default-features = false` trim is a tracked follow-up).
- NEVER export self-obs OTLP — not to `:4317` (the PRODUCT stream) and not to `:4318` (unused/dead). NEVER add Sentry/Datadog/Grafana-only consumption.
- NEVER stamp the journal from tokio's virtual clock; NEVER leak host paths / struct names into logs / `runs.db` / report.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-06-17: The Run-report envelope JSONL schema is OWNED by test-plan §3 — obs-plan §3 + a11y-plan §3 reproduce it verbatim and CAN drift; implement/verify a field against the owner (test-plan §3), not a reproduction. (emission-journal-writer shipped obs-plan §3's 10-field outlier, missing `read_back_observed_at`; the envelope is 11 fields, `read_back_observed_at` second after `journal_emitted_at`.)
- 2026-06-26: Instrument a `#[tauri::command]` with a MANUAL `let _span = tracing::info_span!("tauri.command.<name>").entered();` at the top of the body — NOT the `#[tracing::instrument]` ATTRIBUTE. The attribute does not stack cleanly with `#[tauri::command]` (the command macro rewrites the fn signature for IPC arg-extraction, so instrument wraps the wrong shape). The manual span yields the same bounded `tauri.command.*` name (obs §4) and lets you log allowlisted fields (`count`, `latency_ms` — both already in the `conductor-core::redact` allowlist) on an `info!` boundary line. Errors returned to the webview go through `conductor_core::sanitize_error(&e)` (Display-not-Debug + host-path scrub), never the raw error. `conductor-tauri` gains a direct `tracing` dep for this (already a workspace dep — no new `Cargo.lock` package). (scenario-suite-picker-start-stop chunk)
