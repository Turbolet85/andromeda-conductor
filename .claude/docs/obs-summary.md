# Observability Summary — Conductor

_Distilled from `.andromeda/obs-plan.md`. setup-project Phase 3. wrap-session does not modify._

## Obs tier
**Tier:** Minimal (0).
**Justification:** 8 crates + 2 surfaces; self-observation is structured `tracing` logs only (JSONL); OTLP is the PRODUCT (fault telemetry emitted AT Pulse), not the obs mechanism — a creator-explicit ban on an OTel SDK for self-observation.

## Harness contract (§3) — bound to the test harness
- **Logger:** `tracing` 0.1.44 + `tracing-subscriber` 0.3.23 (`fmt().json().flatten_event(true)`).
- **OTel SDK:** NONE for self-observation (the only OTLP is the PRODUCT fault stream to Pulse `:4317`).
- **Log sink:** cli → `logs/agent-latest.jsonl` (`--agent-mode`) / stderr (dev); Tauri backend → `logs/conductor-tauri.jsonl`; Tauri frontend → browser `console.log` JSON (no network exporter — recursion guard).
- **Log format:** JSONL, one object per line (custom `tracing-subscriber` layer). Two shapes: the **self-obs base line** (every line) — `timestamp_ms` (`std::time`), `level`, `target`, service-identity, `run_id`; the **Run-report envelope** — `journal_emitted_at`, `read_back_observed_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints` (schema owned by test-plan §3). The self-obs line has two variants over its base set: the **event line** and the **span-lifecycle line** (`span`, `span_event` `new`/`close`, optional `parent`, plus the span's allowlisted attributes on `new`).
- **Heartbeat:** N/A for cli (short-lived per-scenario); Tauri backend optional 30s `conductor.tick` via `Channel` (UI state, NOT telemetry).
- **Correlation:** the `run_id` field — NO W3C `trace_id`/`traceparent`. Cross-surface parity = `runs.db` envelope comparison.
- **Status:** disk read (no endpoint) — bound to test-plan §3.

## SLO invariants (§10)
| Metric | SLO | Source |
|---|---|---|
| unlogged panics | zero (`std::panic::set_hook` → `tracing::error` → anyhow edge) | panic hook + CI grep |
| `latency_ms` | ≤ `slo_tier` (`<5s`/`<20s`/`<90s`) | JSON field assertion at report-gen (no histogram backend) |

## Service identity
- `service.name` = `conductor` / `conductor-tauri` / `conductor-ui` (override `$CONDUCTOR_SERVICE_NAME`).
- `service.version` = `env!("CARGO_PKG_VERSION")`; `deployment.environment` = `$CONDUCTOR_ENV` (default `local`).
- Emitted as flat per-line JSON fields — NOT OTel resource attributes.

## Redaction wire
- Field-allowlist (drops non-allowlisted field names) + value scrub at the tracing-subscriber processor stage (`conductor-core::redact`), not just the sink — mask absolute host-FILE paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) → `<redacted>`, NOT `::`-type tokens; the allowlisted `target` module path is preserved.
- No PII by architecture (synthetic-only); the allowlist permits identity + `message` + bounded self-obs + reserved envelope fields; struct names kept out via the allowlist + `Display`-not-`Debug` at the `anyhow` edge.

## Universal anti-patterns
- No OTel SDK / `tracing-opentelemetry` / `traceparent` for self-observation; no network OTLP (recursion guard).
- No proprietary-APM-only or dashboards-only consumption — every signal is agent-readable (JSONL / `runs.db` / report).
- Never stamp the journal from tokio's virtual clock; never skip the `run_id` field; never log in a hot path at `info`.

## Critical decisions
- **OTel SDK = NONE** for self-obs (creator ban) — preserves `current_thread` determinism + keeps the PRODUCT OTLP stream clean.
- **`tracing` JSON** is the only self-obs mechanism; wall-clock from `std::time`.
- **No metrics backend** — perf budget is a `latency_ms`/`slo_tier` JSON field assertion, not a histogram.
- **Error reporting** = `std::panic::set_hook` + `anyhow` edge; no external error-tracking platform.

---

**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.claude/rules/observability.md`.
