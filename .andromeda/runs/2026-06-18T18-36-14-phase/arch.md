# arch extract

## Relevance
Relevant — multi-service topology is a cross-cutting emission-seam concern within the Determinism and Standard Contracts scope.

## Constraints

1. **Crate placement** — per §Inherited Defaults (Module boundaries), code lives in `conductor-emit` workspace crate; forbidden cross-crate deps won't compile.
2. **Determinism mandate** — per §Design Philosophy (Determinism under a seed), same scenario+seed must produce identical stream shape; any service partitioning or ID assignment must be a deterministic function of the seeded RNG (ChaCha8Rng per §Established Decisions [Determinism RNG]).
3. **Async runtime constraint** — per §Established Decisions [Async Runtime Flavor], the deterministic `current_thread` tokio runtime owns the timeline engine; span/trace emission models must integrate cleanly with this single-threaded scheduler.
4. **OTLP raw types** — per §Stack and Technologies (OTLP emission) and §Established Decisions [OTLP Emission Strategy], use raw `opentelemetry-proto 0.32.0` message structs (`ExportTraceServiceRequest` / `ResourceSpans` / `Span`), not the SDK exporter (which offers no control over fingerprint identity or root placement).
5. **Standard Contracts — OTel Semantic Conventions** — per §Conventions (Interface surfaces), W3C Trace Context semantics and OpenTelemetry Semantic Conventions (service.name, trace_id, span_id, parent_span_id, Status.Code) are the shared vocabulary and MUST be realized in raw OTLP fields.
6. **No new dependency expected** — the chunk reuses `trace` proto types already enabled; no new Cargo feature or external crate addition unless drift-detected.
7. **gRPC transport binding** — per §Stack and Technologies and §Occupied Resources (Ports), multi-service traces ship over tonic 0.14.6 gRPC to `127.0.0.1:4317` (loopback egress to Pulse); refused transport surfaces as typed `EmitError` per the verdict/error wall.

## Patterns to follow

1. **Span tree extension pattern** — per the error-spans precedent (Epoch 2), build on the existing `span_tree.rs` intra-trace tree builder; cross-service parent linkage (a span in service A parents a span in service B) extends the tree builder rather than duplicating single-service logic.
2. **`ExportTraceServiceRequest` partitioning** — per §Conventions (Outbound emission), one request carries ≥2 `ResourceSpans`, each with a distinct `service.name` Resource attribute; the emit primitive constructs multi-service topology within one transport message (reused from existing single-service message.rs) rather than spawning parallel requests.
3. **Deterministic ID seeding** — per the timeline engine (seeded-phase-scheduler precedent, Epoch 1), IDs are seeded from the scenario seed via ChaCha8Rng (`seed_from_u64`); cross-platform-stable ID assignment ensures reproducibility.

## Anti-patterns to avoid

1. **Clock-driven timing** — do not read wall-clock for trace/span boundaries; wall-clock stamps come from `std::time::SystemTime` / `Instant`, but the determinism invariant (same seed ⇒ same shape) must not depend on them. Timeline offsets derive from seeded schedule, not elapsed real time.
2. **SDK exporter convenience** — do not use `opentelemetry-otlp` exporter for ease; raw OTLP types are non-negotiable for fingerprint and error-placement control (per §Established Decisions [OTLP Emission Strategy]).
3. **Service/span randomization without seed** — do not assign service names, trace IDs, or span IDs via non-deterministic sources; all must be seeded functions of the scenario seed.

## Contract bindings

- **obs ↔ emit** — traces/spans are observed via `tracing` + `tracing-subscriber` (self-observation; obs-plan §3), not OTLP; OTLP is the product emission (per §Established Decisions [OTLP Emission Strategy]).
- **verify ↔ emit** — cross-service error placement (P-008) extends the verdict/error wall contract; `EmitError` surfaces refused transport; MCP read-back will verify the distinct `service.name` set (P-027, service constellation).
- **faults ↔ emit** — the fingerprint-storm fault (Epoch 7) will depend on this chunk's fingerprint primitive from `conductor-emit` (per amendment 2026-06-18-exception-events-fingerprint-control); no cross-service injection in Epoch 6.

## Acceptance criteria contributions

1. (arch) Multi-service trace builder emits ≥2 distinct `service.name` ResourceSpans in a single `ExportTraceServiceRequest` per §Standard Contracts (Outbound emission).
2. (arch) Shared `trace_id` flows across all services with valid cross-service `parent_span_id` linkage (W3C Trace Context propagation per §Conventions).
3. (arch) Code lives in `conductor-emit` per §Inherited Defaults (Module boundaries); workspace crate-per-seam discipline enforced at compile time.
4. (arch) Deterministic under scenario seed: service assignment + ID generation produce identical topology shape given same seed + ChaCha8Rng per §Cross-cutting Patterns (Determinism discipline).
5. (arch) Refused gRPC transport to `127.0.0.1:4317` surfaces as typed `EmitError` per §Established Decisions (Error Handling) — verdict/error wall respected.

## Relevant amendment history

- **2026-06-18-exception-events-fingerprint-control** — fingerprint primitive (`fingerprint()` + `exception_trace_request()` builder) placed in `conductor-emit`, co-located with exception content it derives from; conductor-faults narrowed to fingerprint-STORM fault (Epoch 7), which will depend on emit. Routine spec→impl alignment. Cascaded to CLAUDE.md §Modules (Orbit only; stack.md unchanged).
