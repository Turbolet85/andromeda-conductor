# obs extract

## Relevance
Relevant — severity-logs chunk adds a logs-record emission primitive (per obs-plan §1 Instrumentation scope: `conductor-emit` instrumentable), which requires instrumentation coverage under Minimal tier.

## Constraints
1. No OTel SDK init for self-observation; instrumentation is `tracing` spans + JSONL logging only (per obs-plan §3 OTel SDK init)
2. `SeverityNumber` and `SeverityText` are spec-controlled inputs, not seed-derived; wall-clock stamps from `std::time::SystemTime`, never tokio virtual clock (per scope §What it builds line 26–27)
3. `LogsEmitter` must mirror `TraceEmitter` structure: loopback gRPC to `:4317`, typed `EmitError` on refused/malformed transport, typed input (tonic::Status) never panic (per scope §Acceptance intent line 69–71)
4. Instrumentation must preserve determinism: same seed + same log spec ⇒ identical emitted shape (excluding wall-clock stamps) (per scope §Acceptance intent line 73–75)
5. Standalone emit primitive (not yet timeline/journal-wired); match the altitude of siblings (`raw-otlp-message-scaffold` / `error-spans` / `exception-events`) with loopback gRPC stub, real `:4317` never bound (per scope §Boundaries / out of scope line 38–42)
6. Self-observation of logs-record builder/emitter boundary call via `#[tracing::instrument]` span on `LogsEmitter` egress; log level `info` for boundary-call summaries (per obs-plan §4 Must-trace path scenarios, §6 Log Coverage §Boundary-call wrappers)
7. No absolute file paths, no internal struct names in any emitted field; redaction via allowlist + field-name filtering + Display-not-Debug at `anyhow` edge (per obs-plan §1 Instrumentation scope and obs-plan-amendments §2026-06-15-log-error-boundary-redaction)

## Patterns to follow
1. Module boundary span naming: `{module}.{operation}` pattern — e.g., `emit.batch` for batch egress; here: `emit.logs_batch` for log-record egress (per obs-plan §4 Span naming convention)
2. Boundary-call instrumentation: wrap tonic `LogsServiceClient` call with `#[tracing::instrument]` span carrying `batch_index`, `record_count`, `severity_range` (min/max SeverityNumber in this batch) attributes; log method name + latency_ms + error/status on completion (per obs-plan §6 Boundary-call wrappers)
3. Loopback gRPC stub for test: bind to `127.0.0.1:0` (ephemeral port), implement tonic `LogsService`, capture received `ExportLogsServiceRequest` for assertion (mirroring existing trace stub) (per scope §Acceptance intent line 76–77)
4. Structured error handling: refused/malformed transport ⇒ typed `EmitError`, collector `tonic::Status` ⇒ typed input via span attributes (never panic at error-wall) (per scope line 19–20 and obs-plan §7 Error Capture & Reporting)

## Anti-patterns to avoid
1. Do NOT initialize OTel SDK, spawn batch exporters, or emit any OTel self-instrumentation (determinism + recursion guard violations) (per obs-plan §3 and §6 Obs Anti-Patterns: "Emit SDK init" + "OTel auto-instrumentation spawning batch tasks")
2. Do NOT use tokio virtual clock for `LogRecord` timestamps; only `std::time::SystemTime` / `Instant` (per scope §What it builds line 27 and obs-plan §1 Instrumentation scope note on `conductor-timeline`)
3. Do NOT treat collector `tonic::Status` as a fatal panic; surface it as a typed `EmitError` result (error-wall discipline) (per obs-plan §7 Error Capture & Reporting)

## Contract bindings
**obs ↔ tests harness** — tests consume the loopback gRPC stub capture: log records received must carry the correct `SeverityNumber` / `SeverityText` pair (P-007 validation, §3 test-plan §6 critical-path assertion); the Run-report envelope must include scenario-result fields (`verdict`, `state`, `latency_ms`, `slo_tier`) if journal-wired (deferred if standalone primitive) (per obs-plan §3 Observability Harness Contract and scope §Boundaries / out of scope)

## Acceptance criteria contributions
1. (obs) `LogsEmitter` emits `ExportLogsServiceRequest` via loopback gRPC; transport-layer faults surface as typed `EmitError`, not panics (error wall intact).
2. (obs) Instrumentation: `#[tracing::instrument]` span on `LogsEmitter::emit()` call carries `record_count`, `severity_min`, `severity_max` attributes; boundary-call log at `info` level includes method status + latency_ms.
3. (obs) Determinism: identical seed + identical log-spec input ⇒ identical emitted `LogRecord` shape on identity-relevant fields (severity + body text); wall-clock stamps exempt; severity is spec-controlled, not seed-derived.
4. (obs) No absolute file paths or struct names in `LogRecord.body` or span attributes; redaction layer (if implemented) uses allowlist + field-name filter, preserves `target` module correlation field.

## Relevant amendment history
(none) — the amendments in obs-plan-amendments.md (2026-06-15, 2026-06-16, 2026-06-17) concern structured-logging-stack, emission-journal-writer, and raw-otlp-message-scaffold chunks respectively; severity-logs is the next chunk in sequence and does not yet have a prior amendment. The 2026-06-17 amendment (no-SDK invariant + dormant transitive OTel SDK noted) is relevant context for this chunk's instrumentation posture but does not amend severity-logs' own rules.