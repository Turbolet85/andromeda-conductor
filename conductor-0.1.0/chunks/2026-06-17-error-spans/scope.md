# Scope — Error spans

**Marker:** `2026-06-17-error-spans`
**Working-route entry:** "Error spans — Status.Code=ERROR with root-vs-child placement (P-005, P-008)"
**Epoch:** 3 — Emission primitives (chunk 2 of 8)
**Primary crate:** `conductor-emit`
**P-IDs:** P-005 (hard ERROR-status signal), P-008 (root-vs-deep error placement — emission side only)

## What this chunk builds
Extends the raw OTLP trace scaffold (`conductor-emit`) with the first *fault-meaningful* span
content: the ability to mark a span errored via the OTel `Status.Code=ERROR` signal and to place
that error at a chosen depth in an intra-trace span tree (root span vs a deep child) — the emission
primitive that the P-005 and P-008 scenarios will later drive.

Concretely:
1. **ERROR status construction** — a helper surface that stamps a `Span.status` with
   `Status { code: STATUS_CODE_ERROR, message }` (plus the OK / UNSET counterparts), the canonical
   OTel hard error signal (P-005). Status code is a first-class, byte-controllable field that fault
   injection will later perturb (e.g. a span that *should* be ERROR emitted as UNSET).
2. **Intra-trace span tree** — construct a multi-span trace with correct `trace_id` sharing +
   `parent_span_id` linkage: one root span (empty `parent_span_id`) and N descendant spans, so an
   error can be attached at the root or at a deep child at a chosen depth (P-008 root-vs-deep,
   emission side). Span / trace IDs are produced under the determinism discipline (seeded,
   reproducible — never a raw RNG / clock read) so the same scenario+seed yields the same trace shape.
3. **Root-vs-child placement selector** — a builder/parameter that chooses WHERE in the tree the
   errored span sits (root | deep child at depth *d*), assembling a well-formed
   `ExportTraceServiceRequest` whose `ResourceSpans → ScopeSpans → Span[]` carry the parent/child
   topology with the single ERROR span at the requested position.
4. **Egress reuse** — the assembled multi-span error trace ships over the *existing* tonic
   `TraceServiceClient` to `127.0.0.1:4317`; no new transport surface. `EmitError` / `tonic::Status`
   handling is unchanged (verdict/error wall).
5. **End-to-end test coverage** — emit a root-error trace and a deep-child-error trace to a loopback
   gRPC stub; assert the received protobuf carries `Status.Code=ERROR` at the expected span and the
   parent/child linkage is well-formed (determinism: loopback stubs only in tests).

## Boundaries (explicitly NOT this chunk)
- No exception span events (`exception.type` / `message` / `stacktrace`) and no fingerprint
  identity/variants — next chunk (P-006, P-017, P-018). The error signal here is the `Status` field
  ONLY.
- No severity-boundary logs / `SeverityNumber` (P-007); no `LogsServiceClient` emission logic.
- No latency shaping (P-011, P-012).
- No CROSS-service virtual topology and no W3C trace-context propagation across service edges — that
  is the later "Multi-service topology" chunk (P-008's topology half, P-027). This chunk's tree is a
  single-service intra-trace parent/child hierarchy.
- No PII payload corpus; no traffic-rate ramps (P-026).
- No fault injection (conductor-faults / later); no `:4317` port-occupier bind.
- No MCP read-back / verdict logic — P-005's hard pass/fail and P-008's calibration-region weighting
  are Epoch 5/7; this chunk is emission *construction* only.
- No required `run_timeline` per-transition wiring — the error-span builders are callable emission
  primitives (consistent with the scaffold chunk); deeper timeline integration is at the plan's
  discretion / deferred.

## Surfaces / contracts touched
- `crates/conductor-emit/` — extend the raw-type builder surface (e.g. `message.rs` + a new
  span-tree / status module) with ERROR-status construction, span-tree assembly
  (trace_id / span_id / parent_span_id), and the root-vs-child placement selector; `lib.rs` re-exports.
- Deterministic ID generation — span/trace IDs derived under the seeded discipline (no entropy /
  clock read), preserving "same scenario+seed ⇒ same stream shape". The builder takes its
  randomness/IDs as input (seed or caller-supplied) rather than owning global entropy.
- OTLP/gRPC egress to `127.0.0.1:4317` — reused, unchanged (the sole deliberate egress; `:4318` unused).
- `conductor-core` edge only; no forbidden cross-seam dependency. No new workspace dependency is
  expected (opentelemetry-proto's `Status` / `Span` parent-link fields are already in the tree from
  the scaffold) — if any dep/feature *is* added, `Cargo.lock` stays committed + cargo-audit/deny green.
- Verdict/error wall unchanged: `EmitError` for harness/transport faults; `tonic::Status` typed input.

## Definition of done (intent anchor for validation-1)
`conductor-emit` can construct a well-formed multi-span OTLP trace in which a chosen span carries
`Status.Code=ERROR`, placed either at the root or at a deep child via a parent/child span tree with a
shared `trace_id` and correct `parent_span_id` linkage, and ship it over the existing tonic
`TraceServiceClient` to `127.0.0.1:4317`. Span / trace IDs are deterministic under seed. Exercised
against a loopback gRPC stub asserting the ERROR status sits at the expected span and the tree is
well-formed; `Cargo.lock` committed + cargo-audit/deny green; `cargo clippy -D warnings` and
`cargo nextest` clean. No exception events, no severity logs, no cross-service topology (later chunks).
