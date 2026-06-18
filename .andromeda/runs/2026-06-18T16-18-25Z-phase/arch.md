# arch extract

## Relevance
relevant — the chunk lands a new emit primitive in conductor-emit (a workspace crate), exposes new gRPC surfaces over existing loopback egress, and follows established emit patterns.

## Constraints
- (arch) Code lives in `conductor-emit` crate per workspace boundary rules (§Inherited Defaults, crate-per-seam Cargo workspace; §Occupied Resources lists `conductor-emit` as a workspace member)
- (arch) OTLP logs service uses raw `opentelemetry-proto` types (ExportLogsServiceRequest/ResourceLogs/ScopeLogs/LogRecord), never the opentelemetry-otlp exporter (§Established Decisions §[OTLP Emission Strategy]: "Raw message structs give byte-level control, which fault injection requires")
- (arch) LogsEmitter ships over loopback gRPC to `127.0.0.1:4317` (§Occupied Resources §Ports: OTLP egress is Conductor's only outbound OTLP surface; no new listener opened)
- (arch) Verdict/error wall: transport refused/malformed surfaces as typed `EmitError` Result::Err; tonic::Status is a typed input never a panic (§Established Decisions §[Error Handling] + §Cross-cutting Patterns §Verdict/error wall)
- (arch) Severity is spec-controlled (caller-provided), not seed-derived; wall-clock stamps come from std::time, never tokio's virtual clock (§Cross-cutting Patterns §Determinism discipline; §Established Decisions §[Timing-Tolerance Model])
- (arch) No new inbound listener; trust boundary remains loopback client-only (§Cross-cutting Patterns §Trust boundary: "Conductor is a gRPC/MCP client, not a server"; port-occupier exception does not apply here)
- (arch) Rust toolchain pinned 1.94.1 MSRV per 2026-06-14 amendment (security-plan §Dependency Security required bump; §Stack and Technologies rust-version row)

## Patterns to follow
- Mirror TraceEmitter + trace_request builders: same `connect(addr) → Result<Self>` / `from_channel(ch) → Self` shape, same loopback stub-test pattern (not binding `:4317` in tests; use ephemeral `127.0.0.1:0`)
- Reuse `message.rs` helpers: `service_resource()`, `string_kv()`, `unix_nanos()` where logs message shares them with trace path (scope/resource/timestamp conventions identical)
- Treat `SeverityNumber` + matching `SeverityText` as controlled OTel semantic convention (§Conventions §Interface surfaces: "OpenTelemetry Semantic Conventions are the shared vocabulary"); both directions testable (same spec → identical projection excluding wall-clock)

## Anti-patterns to avoid
- Do NOT use opentelemetry-otlp exporter; it spawns its own batch tasks and offers no fingerprint-identity control (§Established Decisions §[OTLP Emission Strategy] — single most consequential fork)
- Do NOT bind `:4317` in tests or production (reserved for loopback egress to Pulse; port-occupier scenario is the only deliberate port bind and is a different fault injection, not this chunk)

## Contract bindings
**Emit → Test:** loopback gRPC logs stub (LogsService on ephemeral `127.0.0.1:0`) must be implemented for unit tests; mirrors existing trace stub pattern (per testing.md). Gates: nextest run, clippy, audit/deny, Cargo.lock un-drifted.

**Emit → Report:** emitted log records are intended for runtime feed into the emission journal (`RunRecord`, ground truth of what was sent) in a later timeline-integration chunk; accept-criteria currently test determinism projection (shape/severity/body stable across same seed) but defer journal wiring altitude decision (per scope.md boundary §1 resolve).

## Acceptance criteria contributions
- (arch) LogsEmitter + logs_request builder ship over loopback gRPC; transport refused/malformed surfaces as typed EmitError; tonic::Status is a typed input, never a panic (verdict/error wall per §Established Decisions + §Cross-cutting Patterns).
- (arch) Same seed + same log spec ⇒ identical emitted log shape on a seed-agnostic projection (severity + body + linkage stable); severity spec-controlled, not seed-derived; wall-clock excluded (determinism per §Cross-cutting Patterns §Determinism discipline).
- (arch) No new listener opened; loopback egress to `:4317` only (trust boundary per §Cross-cutting Patterns).
- (arch) opentelemetry-proto `logs` feature enabled; Cargo.lock un-drifted; nextest green, clippy -D warnings green, audit/deny green (§Infrastructure Patterns §Build system).

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold — MSRV raised 1.88.0 → 1.94.1:** All new code in conductor-emit inherits the raised MSRV floor (security-plan §Dependency Security, tar-rs CVE-2026-33056); workspace pins Rust 1.95.0, MSRV 1.94.1.
- **2026-06-18-exception-events-fingerprint-control — fingerprint primitive placed in conductor-emit:** The exception-event fingerprint primitive (`fingerprint()` + exception-trace-request builder, co-located in conductor-emit) precedes this chunk; severity-logs reuses emit infrastructure but does not depend on fingerprints (fingerprints are exception-event derivatives, not log-record concerns at this altitude).