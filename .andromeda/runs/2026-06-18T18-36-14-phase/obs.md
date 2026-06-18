# obs extract

## Relevance
partial — this chunk extends the emission primitives (conductor-emit) to support multi-service topology, which is instrumentable per §1 and foundational to critical-path spans; it does NOT add a new surface or scenario, so it contributes span-tree patterns only.

## Constraints
- Raw OTLP message construction (conductor-emit) is instrumentable per §1 table; instrumentation at the gRPC-call boundary only, not Pulse-side state — §1
- Span naming convention is `{module}.{operation}` (e.g., `emit.batch`); no high-cardinality names per-service-instance; multi-service partitioning must be deterministic under seed — §4 Span naming convention & §2 Agent-readable invariants
- Heartbeat / metrics: N/A for this chunk (it is a message-construction primitive, not a long-running surface) — §3 & §5
- No W3C `traceparent` on self-obs log lines (self-obs is plain `tracing` JSON; product OTLP traces carry W3C context but are NOT self-obs) — §3 Correlation; §1 note
- All fields scrubbed per redaction model: no absolute host-file paths (per 2026-06-15-log-error-boundary-redaction amendment) — §6 Log conformance check

## Patterns to follow
- Span tree builder (existing `span_tree.rs`) extended to cross-service parent linkage: a span in service A can parent a span in service B under one shared `trace_id`, realized in raw OTLP field assignment — reuse the `{module}.{operation}` naming
- Deterministic service partitioning and ID generation: all randomized choices (service assignment, span/trace ID generation) must be seeded functions (`ChaCha8Rng` precedent) — scope "Determinism" & §2
- `ResourceSpans` per-service partitioning within one `ExportTraceServiceRequest` carrying ≥2 distinct `service.name` Resource attributes — builds on existing message.rs `Resource` + request construction, no new serialization

## Anti-patterns to avoid
- No unstructured cross-service trace context propagation (e.g., HTTP header `traceparent` shims); W3C semantics realized in OTLP fields (`trace_id`, `parent_span_id`) only — §3 Correlation
- High-cardinality span names (e.g., per `service.name` instance, per endpoint, per user ID) — §4 & amendment 2026-06-18-severity-logs (bounded span-name set)
- No OTel SDK initialization or use for self-obs (behavioral invariant per amendment 2026-06-17-raw-otlp-message-scaffold) — if tests require instrumentation, use plain `tracing` spans + JSON logs only

## Contract bindings
**obs ↔ tests harness:** the multi-service trace structure (distinct `service.name`s, shared `trace_id`, cross-service parent linkage) is PRODUCT OTLP emission, not self-obs — tests assert the structure via gRPC-boundary inspection. Any self-obs lines the builder emits follow §3 Log format JSON schema and carry `service.name` + `service.version` + `deployment.environment` + `run_id` identity fields.

## Acceptance criteria contributions
- "(obs) Multi-service trace builder emits ≥2 distinct `service.name` ResourceSpans under one `ExportTraceServiceRequest` with one shared `trace_id`." — scope Acceptance intent
- "(obs) Cross-service parent→child `parent_span_id` linkage valid (downstream-service span's `parent_span_id` references an upstream-service span `span_id`)." — §4 Must-trace paths
- "(obs) Builder deterministic: same scenario+seed ⇒ same service topology + ID assignment shape (via seeded RNG)." — §2 Agent-readable invariants
- "(obs) Refused transport (gRPC failure) ⇒ typed `EmitError` at call boundary; if logged, structured `tracing::error!` event (not panic, no unlogged fault)." — §7 Error Capture & Reporting

## Relevant amendment history
**2026-06-17-raw-otlp-message-scaffold** — clarified "no OTel SDK for self-observation" as behavioral (no SDK initialization/use); noted opentelemetry-proto's transitive SDK crates are dormant. Applies: the multi-service builder uses opentelemetry-proto's raw types as PRODUCT emission; the behavioral invariant (no SDK init) is not violated.
