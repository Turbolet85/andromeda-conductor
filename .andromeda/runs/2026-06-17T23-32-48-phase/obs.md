# obs extract

## Relevance
Relevant — chunk adds first ERROR-status emission primitive (P-005, P-008) with span-tree assembly; falls under critical-path instrumentation and error-capture trigger.

## Constraints
1. Per obs-plan §1 Obs tier (Minimal), no OTel SDK for self-observation; only raw `opentelemetry-proto` types for PRODUCT emission (conductor-emit → Pulse) — per §3 OTel SDK init behavioral invariant (dormant transitive SDK acceptable; never initialized for self-obs).
2. Per §4 Span / Trace Coverage, ERROR-status span must use `{module}.{operation}` naming (e.g., `emit.error_status` / `emit.span_tree`); avoid high-cardinality span names per naming convention.
3. Per §4 Must-trace paths (Critical Path 1: Headless deterministic run), spans in the error-emission path must carry attributes `verdict`, `state`, and include parent/child linkage via `trace_id` + `parent_span_id` (NOT W3C `traceparent` — correlation within run is `run_id` field).
4. Per §3 Log format JSON schema (binding contract from tests), error-path boundary calls must log: `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`, `journal_emitted_at` (ISO-8601, wall-clock `std::time::SystemTime`). No absolute host paths or internal struct names (redaction per §11).
5. Per §1 Service identity, logs must carry compile-time `env!("CARGO_PKG_NAME")` = `conductor-emit` + `env!("CARGO_PKG_VERSION")` + `$CONDUCTOR_ENV` (default `local`); every JSON line includes service identity.
6. Per §4 Fault-injection spans (chaos-instrumentation trigger), error-status construction is NOT a fault span — it is a primitive builder; fault spans wrap fault *application* (later chunks). This chunk's spans are emission primitives only.

## Patterns to follow
1. Deterministic ID generation for `trace_id` and `span_id` — derived from seed input (no RNG or clock entropy) so same scenario+seed yields same trace shape; passed to builder rather than owned globally.
2. Span status construction via OTel `Status { code: STATUS_CODE_ERROR, message }` (plus OK / UNSET) — byte-controllable for later fault injection; stored on `Span.status` field before egress.
3. Multi-span trace assembly: single root span (empty `parent_span_id`) + N descendants with correct parent linkage; shipped as `ExportTraceServiceRequest` → `ResourceSpans → ScopeSpans → Span[]` over existing tonic `TraceServiceClient` to `:4317` (reuse egress, no new transport).

## Anti-patterns to avoid
1. Do not initialize OTel SDK for self-observation (behavioral invariant §3 / §1); emit self-obs via structured `tracing` logs only (JSONL + file/stdout sink), never OTLP exporter.
2. Do not use W3C `trace_id`/`traceparent` for correlation within a run (§3 Correlation, §4); use `run_id` field on every log line instead.
3. Do not emit exception span events (`exception.type` / `message` / `stacktrace` fields) or fingerprint identity (scope.md boundary: "No exception events…"). Error signal is `Status` field only.

## Contract bindings
- Tests harness binding (obs-plan §3): structured JSONL log schema (§6) must match run-report envelope shape (11 fields: journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints); per amendment 2026-06-16, `read_back_observed_at` is required (null until read-back; null for blocked rows).

## Acceptance criteria contributions
1. (obs) Span-tree assembly produces well-formed `ExportTraceServiceRequest` with correct `trace_id` sharing, `parent_span_id` linkage (root = empty, children = parent's span_id), and deterministic ID derivation (seed input, no entropy).
2. (obs) ERROR-status construction: span carries `Status { code: STATUS_CODE_ERROR, message }` (plus OK / UNSET variants) as byte-controllable field, verifiable at emission and in loopback stub assertions.
3. (obs) Root-vs-child placement selector works; error signal appears at requested span depth in tree (root or deep child), confirmed by loopback test assertions.
4. (obs) Log boundary calls (`emit.error_status`, `emit.span_tree`) include required schema fields (run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints, journal_emitted_at) + service identity (service.name, service.version, deployment.environment); no absolute paths or internal struct names (redaction per §11).

## Relevant amendment history
- **2026-06-17-raw-otlp-message-scaffold**: Clarified no-SDK invariant as behavioral (span-tree assembly uses raw `opentelemetry-proto` types only; OTel SDK never initialized for self-obs). Dormant transitive SDK (opentelemetry-proto's default features) noted acceptable; follow-up recorded to evaluate `default-features = false` — does not block this chunk (per §3 OTel SDK init, transitive presence is not a violation).
