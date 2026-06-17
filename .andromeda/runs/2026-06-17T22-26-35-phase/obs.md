# obs extract

## Relevance
partial — raw OTLP message scaffold is foundational infrastructure that enables instrumentation; does not itself implement scenario instrumentation or log/span emission, which are Epoch-3+ responsibilities.

## Constraints
- Per obs-plan §1 Instrumentation scope: `conductor-emit` is **Instrumentable** as the raw OTLP message construction and gRPC egress boundary; telemetry surfaces as spans around the gRPC client call and the tonic `Status` outcome, not the transport itself.
- Per obs-plan §3 OTel SDK init: ZERO OTel SDK for Conductor self-observation (creator mandate). `opentelemetry-proto 0.32.0` is the PRODUCT (fault telemetry emitted AT Pulse), not self-instrumentation. Self-observation remains **structured tracing logs only**.
- Per obs-plan §4 Span Coverage: `emit.batch` is a must-trace span child of `timeline.execute`; hand-constructed raw OTLP structs route through tonic with span wrapping the client call, **NOT the internal protobuf construction**. Span attributes: `emission_count`, `p_id_count`.
- Per obs-plan §1 Obs tier: Minimal (0). No metrics backend, no OTel exporter, no HTTP framework auto-instrumentation. Determinism preservation (tokio `current_thread` runtime, seed-reproducible seeding) constrains background batch exporters and async telemetry tasks.
- Per obs-plan §3 Logging stack: `tracing` 0.1.x + `tracing-subscriber` JSON formatter; no exporter or remote sink; tonic client call must log via `tracing` JSON, not OTel metrics.
- Per obs-plan §9 CI Integration: all telemetry artifact outputs (logs, JSON envelopes) must be agent-parseable; `Cargo.lock` committed + `cargo-audit` + `cargo-deny` green before chunk lands.

## Patterns to follow
- Span naming: `{module}.{operation}` pattern (e.g., `emit.batch`); avoid high-cardinality names.
- Boundary-call logging (per §6): gRPC emit span must log method, batch index, emission count, result status (OK/error).
- Verdict/error wall (per scope definition-of-done): transport refusal surfaces as `EmitError` (thiserror enum), typed harness input; tonic `Status` never panics.
- Log-level discipline: gRPC emit outcomes log at `info` level (§6 per-module log levels: `conductor-emit` = info base).

## Anti-patterns to avoid
- **No OTel SDK init anywhere** (creator mandate §6). Do NOT add `opentelemetry`, `opentelemetry-otlp`, or exporter crates.
- **No background-task batch exporters.** Tonic client calls must be synchronous or bounded in the `current_thread` runtime; no async spawn that breaks seed determinism.
- **No hand-dumping of raw protobuf bytes** to stdout/files outside the controlled JSON boundary. Emit stack produces structured `tracing` JSON only; raw OTLP egress to Pulse `:4317` is the PRODUCT, not a debugging surface.
- **No W3C trace context propagation** in gRPC requests. Conductor is stateless; only the PRODUCT fault stream crosses the network.

## Contract bindings
- **obs ↔ tests harness (obs-plan §3 + focus-guide cross-domain binding):** the `emit.batch` span and JSONL lines feed the emission-journal writer (next chunk, Epoch-3 phase-2), which produces the JSONL run-report envelope (binding contract from tests §5); obs CI gate (§9) validates the schema.
- **obs ↔ timeline (obs-plan §4 critical paths):** `emit.batch` is a child span of `timeline.execute` in all 7 must-trace scenarios; timeline integration point deferred to later Epoch-3 chunks (phase-2+), but the span hierarchy and timing discipline must be compatible with determinism preservation.

## Acceptance criteria contributions
- **(obs) gRPC emit logging:** emit.batch span wraps the tonic `TraceServiceClient.export()` call with structured fields (batch_index, emission_count, p_id_count) and logs method name + latency_ms + result status at info level.
- **(obs) Transport error surface:** `EmitError` (thiserror) enum surfaces tonic connection/status errors; no panic on transport fault.
- **(obs) No-OTel-SDK gate:** verify `Cargo.toml` + `Cargo.lock` contain zero OTel exporter or OTel SDK crates; cargo-audit + cargo-deny green; no `opentelemetry-otlp` / `opentelemetry::global::*` calls anywhere in conductor-emit.
- **(obs) Determinism preservation:** tonic client call respects tokio `current_thread` runtime invariant (no async spawn for batch export; any await must be a seam call, not a background task).

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** clarified that self-obs log lines carry base fields (`timestamp_ms`, `level`, `target`, service-identity, `run_id`) and the Run-report envelope is a separate record produced by the report seam (Epoch 6). D-obs-stack cleared: tracing JSON, no OTel SDK. Cascaded; obs-plan §3 reconciled.
- **2026-06-15-log-error-boundary-redaction:** redaction model is host-file-path anchor (absolute paths → `<redacted>`) + allowlist + Display-at-edge; `target` module paths preserved. Relevant if emit stack logs error details; no struct-name token redaction.
- **2026-06-16-emission-journal-writer:** Run-report envelope schema gained `read_back_observed_at` field (ISO-8601; null until MCP read-back). Emit-layer logs do not produce this field (it is populated downstream by verify seam in Epoch 5); raw OTLP fixture tests may parse it as null or omit it on emit-only runs (non-Epoch-5 fixtures).