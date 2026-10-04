### Correlation (no distributed tracing)

- **No W3C trace context anywhere** — no OTel SDK generates `trace_id`/`traceparent`; a local single-process harness doesn't need it. Within a run, the `tracing` span hierarchy + the `run_id` field correlate the lines.
- **HTTP:** N/A (no HTTP server).
- **gRPC outbound (conductor-emit → Pulse):** no context propagated. **The only OTLP Conductor speaks is the PRODUCT fault stream to Pulse on `:4317`; self-observation NEVER exports OTLP at all** (no SDK, no exporter, no `:4318` — `:4318` is unused per arch, exporting there is both a recursion trap and a dead port). Self-obs is stderr/file JSON only.
- **IPC (Tauri command → conductor-core):** the envelope carries the **`run_id`** (correlation), not a `traceparent`; the command-handler `tracing` span is the parent of the core-operation spans.
- **Internal async (tokio `current_thread`):** `tracing::Span::current()` within the single-threaded runtime preserves span context automatically; no cross-task boundary.
