# obs extract

## Relevance
Relevant — this chunk is the primary realization of the obs invariant (self-obs never exports OTLP; structured JSON logging via `tracing`; `run_id` on every line; zero unlogged panics).

## Constraints
1. **No OTel SDK for self-observation** (obs-plan §11 Anti-Patterns): `opentelemetry-proto` is the PRODUCT fault stream only; self-obs is `tracing` + `tracing-subscriber` JSON to stdout/file exclusively (obs-plan §1 "no OTel SDK or exporter for self-observation"; §3 "NONE — creator-explicit anti-pattern")
2. **Wall-clock timestamps only** (obs-plan §1 scope §32 + §11 anti-pattern): `std::time::SystemTime` / `std::time::Instant`, never tokio virtual clock; preserves journal-relative SLO math
3. **JSONL schema binding contract** (obs-plan §3 §6 §6): every line must carry `journal_emitted_at` (ISO-8601), `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints` (agent-parseable; no absolute paths or internal struct names per redaction layer requirement)
4. **Service identity fields stamped at every line** (obs-plan §3): `service.name` (hardcoded `"conductor"` or env `$CONDUCTOR_SERVICE_NAME`), `service.version` (compile-time `env!("CARGO_PKG_VERSION")`), `deployment.environment` (runtime `$CONDUCTOR_ENV`, default `"local"`)
5. **`run_id` correlation key on every line** (scope "the cross-cutting correlation key"; obs-plan §6 §11 anti-pattern): no W3C `traceparent`; span context is `tracing` hierarchy rendered inline by JSON subscriber
6. **Panic capture via `std::panic::set_hook()`** (scope "zero-unlogged-panics invariant"; obs-plan §10 SLO Invariant): every panic routed into structured log stream, JSON-serialized backtrace, converted to `anyhow::Error` at binary edge
7. **Dual sink (CLI), file sink (Tauri)** (obs-plan §3 §6): CLI = stderr pretty (dev) + file `logs/agent-latest.jsonl` (agent mode); Tauri backend = file `logs/conductor-tauri.jsonl` + stderr (dev); frontend = console JSON (no network export per recursion guard)

## Patterns to follow
1. **Subscriber init entrypoint** (scope "runtime-agnostic entrypoint callable identically from both binaries"): a public fn in shared core (`conductor-core` candidate) that CLI and Tauri handlers call identically; establishes global `tracing` subscriber at process startup
2. **Structured field logging** (obs-plan §6): all log events via `tracing` macros (`info!`, `debug!`, etc.) with structured fields (not string interpolation); JSON subscriber renders fields as map keys in JSONL
3. **Span hierarchy via `#[tracing::instrument]`** (obs-plan §4): annotate must-trace functions with span open/close tied to phase boundaries; span nesting becomes parent-child hierarchy in JSON log events
4. **`RUST_LOG` environment variable gating** (obs-plan §6): per-module base levels (conductor-cli = info, conductor-timeline = debug opt-in, etc.); JSON subscriber respects `RUST_LOG` filter
5. **Run-id threading via context** (scope "threaded so all self-obs for a run shares it"): pass `run_id` as span field at root scenario span; child spans inherit it; every JSON line carries it via `tracing` context

## Anti-patterns to avoid
1. **No OTel SDK init or exporter in this chunk** (obs-plan §11): no `opentelemetry::global::set_text_map_propagator()`, no meter provider, no tracer provider; `tracing-opentelemetry` bridge is forbidden (re-introduces SDK)
2. **No unstructured stderr text** (obs-plan §11 anti-pattern): all output MUST be structured JSON per-line or removed; raw `eprintln!()` / `println!()` bypasses agent parsing
3. **No multi-line stack traces without one-line serialization** (obs-plan §11): panic hook must JSON-serialize backtrace to a single field, no raw `std::backtrace::Backtrace::capture().to_string()` spilled to stderr

## Contract bindings
- **Tests harness contract** (focus-guide cross-domain binding §2): tests consume structured log format (schema per obs-plan §3 §6) + status endpoint readiness fields; this chunk's JSONL output is the binding surface
- **Binary entry points** (conductor-cli `main()` + conductor-tauri startup): must call the subscriber init before any scenario logic; both binaries bind to the same init surface to guarantee parity
- **Redaction layer seam** (scope "Build the seam so it can attach"): subscriber JSON formatter must be composable with a downstream filter layer that applies field-allowlist + path/name sanitization at processor stage (not sink stage); do not hardcode scrubbing rules here

## Acceptance criteria contributions
1. "(obs) Subscriber init completes before any scenario execution; all self-obs log lines are valid JSONL with schema fields (journal_emitted_at / run_id / seed / scenario / p_ids / verdict / state / latency_ms / slo_tier / fingerprints)."
2. "(obs) Service-identity fields stamped on every line: service.name / service.version / deployment.environment; service.name overrideable via $CONDUCTOR_SERVICE_NAME env var (hardcoded default = 'conductor')."
3. "(obs) Panic captured by std::panic::set_hook() routes into tracing::error! JSON log event (no unstructured backtrace to stderr); exit code = 1 on panic at CLI binary edge."
4. "(obs) No OTel SDK / exporter present in Cargo.toml dependencies for self-observation (opentelemetry-proto remains for PRODUCT only); cargo-audit / cargo-deny gates remain green."

## Relevant amendment history
(none) — amendment file does not exist (normal for fresh project; first amendment will be created on first plan modification)