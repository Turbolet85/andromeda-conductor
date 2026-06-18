# Scope — Traffic-rate ramps (P-026)

**Marker:** `2026-06-18-traffic-rate-ramps`
**Working entry:** Traffic-rate ramps — halo-breathing emission ramps (P-026)
**Epoch:** 3 — Emission primitives (chunk 8/8 — **closes Epoch 3**)
**Crate:** `conductor-emit`
**P-IDs:** P-026

## What it builds
A `conductor-emit` primitive that generates a **deterministic, seeded, time-varying emission-rate
profile** — the spans-per-window counts that realize a target traffic-rate curve (linear ramp segments
and/or periodic "breathing" oscillation) over a span of time. Driven over the timeline, the resulting
OTLP span stream's *throughput* rises and falls on a reproducible curve.

This drives Pulse's **P-026 "halo breathing"**: in the constellation view, a service-dot's halo pulses
at a rate that tracks the service's live traffic rate. Exercising P-026 means feeding Pulse a span
stream whose *rate* visibly ramps and breathes so the operator can observe the halo's breathing
speed up and slow down.

It is the **rate analogue of the already-built latency-shaping primitive**: latency-shaping seeded
per-operation span *durations* to hit target p50/p95/p99; traffic-rate ramps seeds per-window span
*counts* to hit a target rate curve. Same crate, same `ChaCha8Rng` determinism discipline, same
loopback-capture test style.

## Boundaries (what it does NOT do)
- **No halo verification.** P-026 is a visual / operator-checklist claim ("halo breathing?") with no
  programmatic MCP read-back; this chunk *produces* the emission shape only. Verification routes to a
  later epoch (ManualCheck / operator checklist), not here.
- **Not durations, not error-fraction.** Span durations are latency-shaping (done); error-fraction
  ramps are the Epoch-4 fault helpers (later). This chunk shapes *throughput / span count over time*.
- **Does not own the wall-clock driving.** The seeded phase scheduler in `conductor-timeline`
  (Epoch 2, built) sequences phases on `tokio::time`. This primitive provides the rate-curve→counts
  math + batch building that a driver consumes; it stays in `conductor-emit` and does **not** take a
  dependency on `conductor-timeline` (crate-per-seam law). The exact emit/timeline division of labour
  (does emit return per-tick counts, an iterator of emission instants, or pre-sized batches?) is a
  plan-time question to resolve in P3/P4.
- **No scenario-config / garde wiring.** Consistent with its Epoch-3 siblings, this is a standalone
  seeded primitive exercised in isolation; scenario-config wiring is deferred to the scenario epoch.
- **No new dependency** expected (reuses `rand_chacha` + the existing OTLP message builders).

## Surfaces / contracts it touches
- `conductor-emit`: new module (e.g. `rate.rs`) — a seeded rate-profile type + a counts/batch builder,
  re-exported from `lib.rs` with a module-doc (matches `pii.rs` / latency-shaping precedent).
- **Determinism contract:** `ChaCha8Rng` via `seed_from_u64` — same scenario+seed ⇒ same rate curve
  (the universal determinism bar; cross-platform/version-stable).
- Reuses the existing trace/span message builders (it produces *more or fewer* spans, not a new wire
  shape).
- Possible touch: the emission-journal line schema *iff* per-window rate/count needs journaling —
  a research question (tests/obs-owned schema; do not assume).

## Acceptance intent (anchor for validation-1)
- A seeded rate-profile primitive that, given a target rate curve (ramp and/or breathing parameters)
  over a duration, deterministically yields the per-window span counts realizing it.
- Reproducible: identical seed ⇒ identical count sequence; verified by unit tests.
- Realizes the curve shape: ramp segments move monotonically between endpoint rates; breathing
  oscillates around a center rate — asserted within seeded tolerance.
- Integrates with the existing OTLP trace builders to emit the shaped stream (loopback-capture test).
- All gates green: `cargo nextest` (emit + workspace), clippy `-D warnings`, llvm-cov, doctest;
  no new dependency / no Cargo.toml drift unless justified.
