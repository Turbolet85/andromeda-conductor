# obs extract

## Relevance
Partial — chunk builds an emission-rate primitive (P-026) within conductor-emit; observability impacts are instrumentation of the rate-curve generation + batch building, plus no new log schema changes.

## Constraints
- Per §1 Obs Scope Summary, conductor-emit is instrumentable: "Raw OTLP message construction and gRPC egress to 127.0.0.1:4317; instrumentation of outbound gRPC call only" — spans must wrap the rate-curve logic and the batch builder, never initialize an OTel SDK (§3 OTel SDK init: behavioral ban, no SDK for self-obs).
- Per §2 Telemetry Strategy, no metrics backend applies (Minimal tier); rate-carving is pure-math, not a perf-budget violation — no `latency_ms` / `slo_tier` fields added by this chunk.
- Per §4 Span Coverage, span names follow `{module}.{operation}` bounded set (§11 Anti-Patterns / Spans: "bounded span name set" — `emit.batch` already exists; no new high-cardinality names).
- Per §4 Fault-injection spans: "Bounded 'typical/high' profiles ONLY (P-060 SLO checks); explicitly NOT saturation (50k+ spans/sec)" — rate ramps must not emit per-span instrumentation; wrap the batch generation only.
- Per §3 Logging stack: no new schema fields required (the Run-report envelope fields remain constant §6; scenario-specific fields are caller-owned, not shaped by this primitive).

## Patterns to follow
- Reuse `ChaCha8Rng` via `seed_from_u64` (determinism contract: same scenario+seed ⇒ same rate curve, cross-platform stable); matches latency-shaping precedent in same crate.
- Instrument rate-curve generation + batch builder via `#[tracing::instrument]` with attributes: span name `emit.batch` (or a distinct child), attributes `emission_count`, `phase` (if distinct phases); no per-emission spans (cardinality guard).
- Use wall-clock `std::time::SystemTime` / `Instant` (never tokio virtual clock) for any timing in span attributes (rate curves are deterministic math, not wall-clock dependent).

## Anti-patterns to avoid
- NEVER add OTel SDK / meter provider for self-observation — rate carving must stay pure math + `tracing` instrumentation; no background batch tasks (§11: "no OTel SDK for self-obs, breaks current_thread determinism").
- NEVER introduce high-cardinality span names or span-per-emission instrumentation — rate ramps produce *multiple* spans per window; batch-wrap only (§11: "bounded span name set").
- NEVER skip the `run_id` field on any self-obs log line — if the rate primitive logs boundary events, include `run_id` for correlation (§11 Logs).

## Contract bindings
- **obs ↔ emit span hierarchy:** per obs §4 Critical Path 1, `emit.batch` spans are children of `timeline.execute` and siblings to `verify.readback` + `report.generate`; rate-curve logic must preserve the span hierarchy so root `scenario.run` correlation is intact. No NEW binding to tests harness (log schema/envelope remain §3's contract; rate-curve is internal to emit, not a seam) — concurs with tests extract.

## Acceptance criteria contributions
- (obs) Rate-curve primitive emits `emit.batch` span(s) per window with bounded attributes (`emission_count`, `phase` if applicable); cardinality bounded (no per-span instrumentation).
- (obs) Determinism verified: identical seed ⇒ identical rate curve and batch sequence; no use of tokio virtual clock in instrumentation attributes.
- (obs) `run_id` present on any self-obs boundary log (rate batch generation boundary event, if logged).
- (obs) Zero new fields added to Run-report envelope schema (§3 §6); scenario-config-owned `slo_tier` / `fingerprints` fields unchanged.

## Relevant amendment history
- **2026-06-18-severity-logs:** `emit.logs_batch` added to the bounded span-name set (logs egress span on `LogsEmitter::export`); if this chunk's batch builder emits logs, distinguish logs egress (`emit.logs_batch`) from trace egress (`emit.batch`) in span naming. (Does not force a change; note for consistency if a new logs-emitting path is added.)
