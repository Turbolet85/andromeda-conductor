# Scope — Multi-service topology

**Marker:** `2026-06-18-multi-service-topology`
**Version:** conductor-0.1.0 · **Epoch:** 3 (Emission primitives) · chunk 6/8
**Working-route intent:** _Multi-service topology — service.name virtual topology + W3C trace propagation (P-008, P-027)_
**Pulse capabilities:** P-008 (root-vs-deep error placement, viewed across services) · P-027 (service constellation — the distinct `service.name` set Pulse renders as service dots)

## What it builds
An emission primitive in **`conductor-emit`** that constructs OTLP traces spanning **multiple emulated services**:

1. **`service.name` virtual topology** — a single `ExportTraceServiceRequest` carrying ≥2 `ResourceSpans`, each with a distinct `service.name` Resource attribute, so one emission realizes a multi-service topology (the set of names Pulse visualizes as a constellation of service dots — P-027).
2. **W3C Trace Context propagation across emulated service edges** — one shared `trace_id` flows across all participating services, with cross-service parent→child linkage (a downstream-service span's `parent_span_id` references a `span_id` owned by the upstream service) so a single logical trace spans the topology. W3C `traceparent` semantics (version · trace-id · parent-id · trace-flags) realized in the raw OTLP span fields.
3. **Cross-service error placement (P-008)** — extends the existing intra-trace root-vs-child placement (`span_tree.rs`, error-spans chunk) to the cross-service case: a deep error in a downstream service vs an error at the topology root, so Pulse's root-vs-deep weighting can be exercised across the service boundary.

## Boundaries (what it is NOT)
- **`conductor-emit` only** — an emission-seam primitive. No scenario-config (garde) wiring — that extension of `EmissionSpec` is **deferred to the scenario epoch** (mirrors latency/severity deferral). No `conductor-verify` / MCP read-back, no `conductor-faults`, no CLI/Tauri surface.
- **No new dependency / no new Cargo feature** expected — reuses the already-enabled `trace` proto types (consistent with error-spans / latency / severity).
- **Not a fault** — this is well-formed multi-service emission, not an injected fault (port-occupier / silence / storm live in Epoch 4).
- Does not re-derive single-service span/trace construction — builds on the raw OTLP scaffold + `span_tree.rs`.

## Surfaces / contracts touched
- **`crates/conductor-emit/src/span_tree.rs`** — extend the intra-trace tree builder with cross-service parent linkage (a span in service A parents a span in service B under one `trace_id`), or a sibling builder if the seam is cleaner separate.
- **`crates/conductor-emit/src/message.rs`** — per-service `Resource` (`service.name`) + `ResourceSpans` partitioning within one `ExportTraceServiceRequest`.
- **`crates/conductor-emit/src/lib.rs`** — re-export the new public primitive(s).
- **`crates/conductor-emit/tests/`** — a new integration test (loopback `TraceService` stub) asserting: distinct `service.name`s present; shared `trace_id` across services; cross-service `parent_span_id` linkage; refused-transport ⇒ `EmitError`.
- **OTel Semantic Conventions / W3C Trace Context** — the shared vocabulary (`service.name`, `trace_id`, `span_id`, `parent_span_id`, trace flags).

## Determinism (the bar)
Same scenario+seed ⇒ same stream shape. Any randomized assignment (service partitioning, seeded id generation) must be a deterministic function of the seed; cross-platform-stable (`ChaCha8Rng`). Determinism asserted on the seed-governed projection; wall-clock `start`/`end` stay outside the determinism assertion (per the latency/error-spans precedent).

## Acceptance intent (full criteria in plan.md)
- Multi-service trace builder emits ≥2 distinct `service.name` ResourceSpans under one request.
- One `trace_id` shared across services with valid cross-service `parent_span_id` linkage (W3C propagation).
- Cross-service root-vs-deep error placement constructible (P-008).
- Deterministic under seed; refused transport ⇒ typed `EmitError` (verdict/error wall).
- Gates green: `conductor-emit` + workspace nextest · clippy `-D warnings` · llvm-cov ≥ threshold · audit + deny · `Cargo.lock` un-drifted.
