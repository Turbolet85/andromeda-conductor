# security extract

## Relevance
Relevant — the chunk extends OTLP emission primitives (multi-service topology, trace propagation) within conductor-emit (Epoch 3), touching input validation (deterministic seeding) and outbound gRPC transport (existing loopback boundary).

## Constraints
1. All seeded span/service randomization MUST be deterministic under the scenario seed via ChaCha8Rng (security plan §Input Validation § Determinism); per §Threat Model Summary attack surface loopback-only egress discipline.
2. New `ExportTraceServiceRequest` emission from multi-service builder MUST carry valid prost-encoded `ResourceSpans` — no unbounded protobuf recursion on the construction path (security plan §Input Validation, protobuf-decode DoS lineage RUSTSEC-2020-0002 / RUSTSEC-2024-0437).
3. Cross-service trace construction with shared `trace_id` and parent-span linkage MUST NOT leak absolute paths, service-naming secrets, or internal struct metadata into the `service.name` attribute or span fields exported to Pulse (security plan §Error Handling § Run-report artifact sanitization, applied to OTLP emission).
4. Refused OTLP transport (loopback to `127.0.0.1:4317` down) MUST surface as typed `EmitError` routed through the verdict/error wall, never panic (security plan §Error Handling, §Threat Model Summary § Attack surface — tonic result handling).

## Patterns to follow
- Deterministic seeding via ChaCha8Rng per the latency/severity/error-spans precedent (security plan §Input Validation § Determinism; Threat Model Summary § Attack surface entry "OTLP/gRPC egress").
- Module-internal typed errors (`EmitError` from the existing seam) collapsed to `anyhow` only at binary/IPC edges (security plan §Error Handling).
- Loopback-only outbound transport discipline: `127.0.0.1:4317` address hard-coded, `tonic::Status` results treated as typed verification inputs, never silent ignore (security plan §Threat Model Summary).

## Anti-patterns to avoid
- NEVER spawn or re-spawn the gRPC client per-emission — reuse the pooled/configured tonic client instance to keep transport surfaces bounded (rmcp STDIO command/argument-injection scope; security plan §Input Validation § Anti-patterns "spawn").
- NEVER expose service names, trace-id generation logic, or span partition strategy to operator-supplied config or scenario input in this chunk — those are deferred to the scenario epoch (security plan scope; chunk boundaries state "deferred to scenario epoch").
- NEVER let unbounded protobuf decoding on the read-back path (in tests) corrupt verdict classification — the integration test's loopback `TraceService` stub MUST bound prost decoding and treat decode errors as typed failures (security plan §Input Validation, protobuf DoS).

## Contract bindings
Outbound emission ↔ loopback transport layer (tonic client reuse discipline, error handling per Threat Model Summary § OTLP/gRPC egress). Tests harness ↔ mock `TraceService` loopback stub (bounded protobuf decoding, refuse-transport verification per Acceptance intent).

## Acceptance criteria contributions
- (security) Determinism assertion passes: same seed + scenario ⇒ same span tree shape, service names, trace_id allocation (multiplatform ChaCha8Rng).
- (security) Refused OTLP transport produces typed `EmitError` (verdict/error wall), no panic.
- (security) `cargo audit`/`cargo deny` green (no new dependencies in this chunk per scope).
- (security) Integration test loopback stub bounds prost decoding and rejects malformed/oversized traces as typed errors.

## Relevant amendment history
- **2026-06-15-dependency-audit-gate:** toolchain ≥ 1.94.1 confirmed done (1.95.0); no new Cargo dependencies expected this chunk. Cargo-audit/deny floors remain green.
- **2026-06-15-structured-logging-stack:** if multi-service emission adds new env-label reads (e.g. service naming), they follow the non-path string label pattern (JSON-escaped in log values, no validation); distinct from path-handle canonicalization (§Input Validation note).
