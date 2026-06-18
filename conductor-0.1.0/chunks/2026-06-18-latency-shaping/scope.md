# Scope — Latency shaping

**Marker:** `2026-06-18-latency-shaping`
**Working entry:** `Latency shaping — target p50/p95/p99 per operation (P-011, P-012)`
**Version / epoch:** conductor-0.1.0 · Epoch 3 (Emission primitives) chunk 5/8
**Crate:** conductor-emit
**P-IDs:** P-011 (Per-Operation Latency Baseline) · P-012 (Latency Regression Detection)

## What it builds
A latency-shaping emission primitive in `conductor-emit` that, given a per-operation **target
latency profile** (p50/p95/p99 in milliseconds), produces concrete span **durations** — realized
as `start_time_unix_nano` / `end_time_unix_nano` on the existing raw-OTLP `Span` — so that an
emitted multi-span stream for a given operation realizes the targeted percentile distribution.
Keyed by **operation** (span name): distinct operations carry independent profiles. Deterministic
under a seed — same profile + seed ⇒ same duration sequence (reproducibility is the determinism bar).

It supplies the controllable latency input Pulse's **P-011** (per-operation latency baseline —
"no direct user signal; baseline feeds P-012") and **P-012** (latency regression detection)
consume: a steady known p50/p95/p99 (the baseline) and a shifted profile (e.g. 3× p99, the
regression) are both just target profiles fed to the same primitive. The primitive must be able to
emit enough samples per operation to clear Pulse's **latency exclusion floor (spec ≥50 samples/window)**
— distinct from the error-rate floor of 10, and a known Pulse spec/code divergence
(`MIN_EWMA_SAMPLES = 10` in Pulse vs the spec's 50); Conductor targets the spec value.

## Boundaries (what it is NOT)
- **Not scenario / timeline-wired** — a standalone primitive like the other Epoch 3 chunks (zero
  external callers of `conductor-emit` today). The latency-regression *orchestration* (known profile
  90s → 3× p99 90s → candidate after 60s persistence) is the Epoch 7 `latency-regression` scenario;
  per-phase traffic ramps are the later Epoch 3 "Traffic-rate ramps" chunk. This chunk maps a
  profile → durations and nothing more.
- **Not verification** — no MCP read-back, no SLO / ±15% tolerance band, no calibration-region
  routing, no sample-count-floor *judgement*. Those are `conductor-verify` (Epoch 5) concerns; the
  ≥50-sample floor appears here only as the count this primitive must be able to *produce*.
- **Not config validation** — the p50 ≤ p95 ≤ p99 ordering invariant + severity-mix sums already
  live in `conductor-core`'s garde layer; this primitive *consumes* an ordered profile (it may
  assert the ordering as a local precondition, but core remains the authoritative scenario-config
  validator).
- **Not the error-rate / span-count primitives** (P-009 / P-010 baseline-spike) — latency only.

## Surfaces / contracts touched
- **conductor-emit:** a new module (e.g. `latency.rs`) + `lib.rs` re-exports; possibly a
  `Cargo.toml` feature consistent with the existing `trace` / `logs` features. Builds directly on
  the raw-OTLP trace scaffold (`Span` timing fields, `TraceEmitter`) and the seeded-RNG discipline
  (`ChaCha8Rng` via rand_chacha — the determinism RNG already owned by conductor-timeline) for
  reproducible sampling of a percentile distribution.
- **OTLP semantic conventions:** only `start_time_unix_nano` / `end_time_unix_nano` are shaped; any
  wall-clock base stamp comes from `std::time` (never tokio's virtual clock), per the journal-relative
  SLO invariant.
- **Determinism contract:** same target profile + seed ⇒ identical duration sequence, reproducible
  across platforms (ChaCha8 stability) — distinguishing this **seeded** primitive from the seedless,
  caller-controlled severity primitive.

## Acceptance intent (val-1 anchor)
- A per-operation target profile (p50/p95/p99 ms) maps to span durations whose realized percentiles
  match the targets within a sampling-convergence tolerance, over a sample set ≥ the 50-sample floor.
- Deterministic: identical seed + profile reproduces the exact duration sequence (unit-tested).
- Per-operation independence: distinct operations carry distinct profiles within one batch.
- Integrates with the existing raw-OTLP `Span` / `TraceEmitter` path; release gates stay green
  (nextest, clippy `-D warnings`, cargo-audit + cargo-deny, `Cargo.lock` un-drifted).
