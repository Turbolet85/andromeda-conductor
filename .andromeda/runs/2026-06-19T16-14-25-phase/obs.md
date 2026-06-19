# obs extract

## Relevance
Partial — port-occupier is a fault-injection helper (chaos-instrumentation trigger in §5) with low observability surface.

## Constraints
- Instrumentation must remain orthogonal to core/timeline/emit (per §1 Table "Instrumentability" for `conductor-faults`): this chunk does NOT depend on conductor-emit or conductor-timeline (§1 "Crate-per-seam law")
- Fault spans must be children of `timeline.execute` and bounded to named-set only (per §4 Fault-injection spans + §11 Anti-Patterns "bounded span-name set")
- Loopback-only binding to `127.0.0.1:4317` is a trust-boundary invariant; this is THE documented exception to "Conductor opens no inbound listener" (scope §54)
- No OTLP / gRPC protocol implementation, no scenario-config wiring, no Pulse management (scope §26-31, §43-44)
- Bind failure must surface as a typed condition (verdict/error wall), never a panic (scope §55)

## Patterns to follow
- Module documentation in `lib.rs` precedent (conductor-emit: `rate.rs`, `latency.rs`, `pii.rs` re-exported with module-doc; §1 §Surfaces/contracts)
- Span attribute naming `fault_type` + `fault_duration_ms` + `fault_start_offset_ms` (per §4 Fault-injection spans: `fault.port_occupier` pattern)
- Structured error/condition logging via `tracing::warn!` / `tracing::error!` for bind failures (§6 boundary-call wrappers; §6 log levels: `warn` for recoverable errors)
- Service-identity fields on every structured log (§3 Logging stack: `service.name`, `deployment.environment` on all JSONL lines)

## Anti-patterns to avoid
- No OTel SDK init, no OTLP self-export (§3 "no OTel SDK for Conductor self-observation" — behavioral invariant per 2026-06-17-raw-otlp-message-scaffold amendment)
- No high-cardinality span names (port number per-span forbidden; only `fault.port_occupier` allowed per §4 bounded set + 2026-06-18-severity-logs amendment)
- No dependency on conductor-emit, conductor-timeline, or conductor-core beyond error types (§1 crate-per-seam)
- No reuse flags (`SO_REUSEADDR`) that would permit co-binding; Windows `SO_EXCLUSIVEADDRUSE` semantics must deny the port reliably (scope §66)

## Contract bindings
- **Tests harness binding** (per obs-focus §Cross-domain bindings): fault-injection spans and structured error logs are consumed by the test harness (tests excerpt §5 "chaos-instrumentation" + critical path verification)
- **Trace correlation via `run_id`** (per §3 Correlation, §4 Both-surface parity): fault-instrumentation spans inherit the parent `timeline.execute` span's `run_id` field (no W3C `traceparent`)
- **Trust-boundary contract** (per scope §54 + CLAUDE.md): this is the sole deliberate inbound bind in Conductor; the implementation must visibly honor loopback-only + cleanup (RAII/Drop) to maintain the invariant

## Acceptance criteria contributions
- (obs) `fault.port_occupier` span emitted with `fault_type="port_occupier"`, `fault_duration_ms`, `fault_start_offset_ms` attributes; span is a child of `timeline.execute` (per §4 Fault-injection spans pattern)
- (obs) Bind-failure condition (port already occupied) logged as structured `tracing::warn!` with `error` and `port` fields; not a panic (per scope §55 verdict/error wall)
- (obs) All structured logs include `service.name`, `run_id` identity fields (per §3 service-identity + §6 required fields on every line)
- (obs) No absolute host paths, no internal `conductor_*` module names in fault logs (per scope §100 + 2026-06-15-log-error-boundary-redaction amendment: value-level scrub of file paths, allowlist + Display-edge for struct names)

## Relevant amendment history
- **2026-06-18-severity-logs** (§Anti-Patterns Spans): bounded span-name set clarified; new low-cardinality spans added (e.g., `emit.logs_batch`); fault spans follow `{module}.{operation}` convention. Rationale: the `fault.port_occupier` span name must be in the bounded set, not dynamic per port.
- **2026-06-17-raw-otlp-message-scaffold** (§3 OTel SDK init): behavioral no-SDK invariant clarified; opentelemetry-proto's transitive SDK deps are dormant. Rationale: conductor-faults must NOT initialize any OTel SDK for self-obs (only structured tracing logs).
- **2026-06-15-log-error-boundary-redaction** (§6 Log conformance, §11 Anti-Patterns): redaction model = absolute file-path value scrub + allowlist + Display-edge, NOT blanket `::` token redaction. Rationale: fault logs with typed conditions (e.g., `io::Error` in bind failure) must preserve stack type names while scrubbing host paths.
