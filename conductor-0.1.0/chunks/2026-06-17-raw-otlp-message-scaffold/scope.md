# Scope — Raw OTLP message scaffold

**Marker:** `2026-06-17-raw-otlp-message-scaffold`
**Working-route entry:** "Raw OTLP message scaffold — opentelemetry-proto structs over tonic/prost gRPC egress to :4317"
**Epoch:** 3 — Emission primitives (chunk 1 of 8)
**Primary crate:** `conductor-emit`
**P-IDs:** none directly — this is the foundational scaffold the P-005..P-060 emission scenarios build on.

## What this chunk builds
The foundational emission-primitive layer in `conductor-emit`: hand-constructed **raw OTLP protobuf
message structs** (via opentelemetry-proto 0.32.0, NOT the OTel SDK exporter) plus the **tonic/tonic-prost
gRPC client** that ships them to Pulse's loopback OTLP ingest at `127.0.0.1:4317`.

Concretely:
1. **Crate dependencies** — wire `conductor-emit` to opentelemetry-proto 0.32.0 (`gen-tonic` + `trace`, plus
   the `metrics`/`logs` features later Epoch-3 chunks will need), tonic 0.14.6, tonic-prost 0.14.6, prost 0.14,
   plus its `conductor-core` edge. Add shared pins to root `[workspace.dependencies]`; `Cargo.lock` regenerated,
   committed, audit/deny-clean.
2. **Raw OTLP trace construction** — a builder/helper surface that assembles a well-formed
   `ExportTraceServiceRequest` from raw structs: `ResourceSpans` (with a `Resource` carrying `service.name`)
   → `ScopeSpans` → `Span` (trace_id / span_id / name / start+end nanos / `Status`). Byte-level control is
   the point — these are hand-built, exposing the exact fields fault injection will later perturb (severity,
   status code, fingerprint identity, root-vs-child placement).
3. **gRPC egress client** — a `TraceServiceClient` over a tonic channel to `127.0.0.1:4317`, with an export
   entrypoint that sends a request and surfaces the tonic outcome.
4. **Transport-error surface** — a connect/transport refusal is a harness `Result::Err` (verdict/error wall +
   the "OTLP egress liveness equivalent"), NOT a verification verdict; a reachable server's `tonic::Status` is
   a first-class typed input, never a panic. Establishes the seam-typed `EmitError` (thiserror) enum.
5. **A minimal end-to-end emit path** — emit one well-formed trace/span to `:4317`, exercised against a
   loopback gRPC stub in tests (determinism discipline: loopback stubs only in tests).

## Boundaries (explicitly NOT this chunk)
- No ERROR-status spans / root-vs-child placement (next chunk — P-005, P-008).
- No exception span events or fingerprint control (P-006, P-017, P-018).
- No severity-boundary logs (P-007); no `LogsServiceClient` emission logic beyond optional dep/feature enablement.
- No latency shaping (P-011, P-012); no multi-service topology / W3C trace-context propagation (P-008, P-027);
  no PII payload corpus; no traffic-rate ramps (P-026).
- No fault injection (conductor-faults / later); no `:4317` port-occupier bind.
- No MCP read-back / verification (Epoch 5); no full preflight liveness/canary gate (Epoch 5) — only the
  transport-connectable error surface at the emit layer.
- No deep timeline-run wiring required: the scaffold is a callable emission primitive; integrating it into
  `run_timeline`'s per-transition loop may be light or deferred at the plan's discretion (the journal writer
  already records emissions — this chunk adds the wire egress).

## Surfaces / contracts touched
- `crates/conductor-emit/` — `Cargo.toml` (real deps), `src/lib.rs` + modules (raw-type builders, gRPC client, `EmitError`).
- Root `Cargo.toml` `[workspace.dependencies]` — opentelemetry-proto / tonic / tonic-prost / prost shared pins.
- `Cargo.lock` — new transitive tree (gRPC/protobuf/h2/hyper); committed, un-drifted, cargo-audit + cargo-deny green.
- OTLP/gRPC egress to `127.0.0.1:4317` — the sole deliberate egress (`:4318` unused); the pinned outbound interface.
- `conductor-core` dependency edge (every seam → core); no forbidden cross-seam edge.
- Verdict/error wall: `EmitError` for harness/transport faults; `tonic::Status` as typed input.

## Definition of done (intent anchor for validation-1)
A `conductor-emit` that builds a well-formed raw OTLP trace `ExportTraceServiceRequest` from opentelemetry-proto
structs and ships it over a tonic gRPC `TraceServiceClient` to `127.0.0.1:4317`, with a transport refusal
surfacing as a typed harness `Result::Err`; exercised against a loopback gRPC stub in tests; `Cargo.lock`
committed + audit/deny green; `clippy -D warnings` clean. Raw types are consumed from opentelemetry-proto's
`gen-tonic` feature — no local `.proto` codegen / `build.rs` (the `tonic-prost-build` route does not apply;
refined during planning from the arch's general note).
