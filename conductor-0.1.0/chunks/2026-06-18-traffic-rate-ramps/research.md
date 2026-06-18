# Codebase Research — 2026-06-18-traffic-rate-ramps

## Scope
- **Depth:** moderate · **Reads:** 5 (`lib.rs`, `Cargo.toml`, `latency.rs`, `message.rs`, `tests/latency_shaping.rs`) · **Globs/Greps:** 1 glob + 3 code-graph queries

## Files inspected
- `crates/conductor-emit/src/lib.rs` (full) — module list + `pub use` re-export surface; module-doc enumerates the trace-request builders. New module slots in as `mod rate;` + a `pub use rate::{...}` line + an addition to the module-doc trace-request list.
- `crates/conductor-emit/Cargo.toml` (full) — deps already present: `conductor-core`, `opentelemetry-proto` (gen-tonic/trace/logs), `tonic`, `thiserror`, `tracing`, `rand_chacha`, `rand_core`; dev-deps `tokio` (macros/rt/net) + `tokio-stream` (net). **No new dependency needed** for rate ramps.
- `crates/conductor-emit/src/latency.rs` (full) — **the closest precedent** (seeded per-span sampling). Mirror its shape exactly for the rate analogue (see Patterns).
- `crates/conductor-emit/src/message.rs` (full) — the shared `pub(crate)` raw-struct primitives the new builder reuses: `service_resource`, `string_kv`, `span`, `span_with_events`, `span_with_attributes`, `timed_span`, `ok_status`, `error_status`, `unix_nanos`, `DEFAULT_SERVICE_NAME`.
- `crates/conductor-emit/tests/latency_shaping.rs` (full) — the loopback-capture integration-test template (CapturingService + ephemeral-port stub + `TraceEmitter`).

## Graph impact (from the code-graph query)
- **`latency_trace_request` / `pii_trace_request` / `service_topology_request`** — all callers are inside `conductor-emit` itself (own `#[cfg(test)]` modules + the `lib.rs` re-export line). **No external caller** — the Epoch-3 emit builders are not yet wired into `conductor-timeline` (that wiring is Epoch 7 — scenario catalog). The new `rate` module is therefore a **leaf**: blast radius = none beyond its own tests. Adding it cannot break any existing caller.

## Patterns detected
- **Seeded-sampling primitive = (profile type + op struct + `*_trace_request` builder + private sampler)** (`latency.rs:24-117`): `LatencyProfile::new(...) -> Option<Self>` self-validates ordering in the constructor; `LatencyOp<'a>{operation, profile, samples}` describes one unit; `latency_trace_request(service_name, seed, &[op]) -> ExportTraceServiceRequest` seeds one `ChaCha8Rng::seed_from_u64(seed)` and loops; the private `sample_durations_nanos(profile, n, &mut rng)` does a stratified inverse-CDF draw (low-discrepancy yet seed-sensitive) via `next_unit(rng)` (uniform `[0,1)` from 53 mantissa bits), `quantile`, `lerp`.
- **ID generation** (`latency.rs:81-82`): `gen_id::<16>(&mut rng)` / `gen_id::<8>(&mut rng)` from `crate::span_tree` produce seeded trace/span ids off the same RNG.
- **Wall-clock isolation** (`message.rs:119-139` `timed_span`): `start = unix_nanos()` (wall-clock `std::time`), `end = start + duration` — the **seed governs the per-span shape value (duration/count), never the absolute stamps**. The rate builder follows suit: counts are seeded, timestamps are wall-clock.
- **Loopback integration test** (`tests/latency_shaping.rs:20-103`): `CapturingService` impl `TraceService` stashes the last request in `Arc<Mutex<Option<…>>>`; `start_stub()` binds `127.0.0.1:0`, spawns `Server::serve_with_incoming(TcpListenerStream)`; test is `#[tokio::test(flavor="current_thread")]`, connects `TraceEmitter::connect("http://{addr}")`, `.export(req)`, asserts on the captured request; a refused transport (`http://127.0.0.1:1`) ⇒ `EmitError::Transport`.

## Conventions to follow
- **Determinism, both directions** (testing rule + session-learning 2026-06-16): assert same-seed ⇒ identical count sequence AND ≥2 different seeds ⇒ divergent counts. **This forces a design constraint** (see Open questions OQ-jitter): the counts must carry *seeded* jitter, else the seed is computed-but-not-material and the diverge direction can't pass.
- **No whole-message byte golden when wall-clock fields exist** (session-learning 2026-06-17, error-spans): assert determinism on a shape-projection (the count-sequence / ids), not the full payload (timestamps would flake). Use exact-string `assert_eq!` on the projection, NOT insta (test-plan §4 amendment 2026-06-16; insta is E2E-only).
- **Pure builder, no per-builder `#[tracing::instrument]`** (`latency.rs` / `pii.rs` carry none): egress instrumentation lives on the `TraceEmitter::export` path (`emit.batch`, obs rule bounded span set), NOT on the message builder. The rate builder stays a pure function — this *agrees with* the obs anti-pattern "no per-emission spans / bounded span-name set" (the obs extract's `#[tracing::instrument]` suggestion is superseded by crate precedent; do not add new spans).
- **`ChaCha8Rng::seed_from_u64`** (arch §Determinism RNG): cross-platform/version-stable; never `StdRng`/`DefaultHasher`.
- **Test convention**: conductor-emit has **no rstest dev-dep** — prior chunks (incl. pii) use plain `#[test]` + helper loops. Match that (the tests extract's rstest suggestion does not fit this crate; adding a dev-dep is unjustified for this chunk).

## New files to create
- `crates/conductor-emit/src/rate.rs` — the seeded rate-curve primitive: a `RateCurve` type (ramp + breathing) with a validating constructor, a private seeded `window_counts(curve, &mut rng) -> Vec<u32>` sampler (target curve + bounded seeded jitter), and `rate_trace_request(service_name, seed, curve, span_name) -> ExportTraceServiceRequest` that emits the per-window spans; module-doc states the determinism contract + the rate-analogue-of-latency framing. In-crate `#[cfg(test)]` unit tests.
- `crates/conductor-emit/tests/traffic_rate_ramps.rs` — loopback-capture integration test (mirror `latency_shaping.rs`): export a ramp + a breathing curve to the stub, assert total span count == Σ window counts and reproducibility; a refused-transport `EmitError` case is already covered by the existing latency test (optional to repeat).

## Files to modify
- `crates/conductor-emit/src/lib.rs` — add `mod rate;`, add `pub use rate::{rate_trace_request, RateCurve};` (exact surface set in P4), and extend the module-doc trace-request enumeration to include `rate_trace_request`.

## Open questions
- **OQ-curve-api:** `RateCurve` exact shape — a single enum `{ Ramp{from,to,windows}, Breathing{center,amplitude,period,windows} }`, vs a focused struct, vs a composable segment list. (P4 — recommend the enum: two named variants cover the scope's "ramp + breathing" with a `LatencyProfile`-style validating constructor; segment composition is over-scope for an Epoch-3 primitive.)
- **OQ-window-visibility:** how the integration test observes *per-window* counts — emit one `ScopeSpans` per window (window = its own scope), or a flat span list asserted only on total + cover per-window shape in the in-crate unit test on `window_counts`. (P4 — recommend per-window `ScopeSpans` for faithful windowed emission + direct testability; falls back to flat+unit-test if it complicates the wire shape.)
- **OQ-jitter (design constraint, not optional):** counts must be `target ± seeded bounded jitter` (analogue of latency's stratified draw), because the determinism-both-directions convention requires the seed to materially drive output. Exact jitter bound (e.g. ±10–20% of target, clamped ≥0) is a P4 number. Confirm in P4 that the realized mean still tracks the curve (ramp monotonic between endpoints within tolerance; breathing oscillates above & below center).
