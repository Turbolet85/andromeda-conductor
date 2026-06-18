# Report — 2026-06-18-traffic-rate-ramps

**Chunk:** Traffic-rate ramps — seeded time-varying emission-rate profile (ramp + breathing curve) realizing P-026 halo-breathing throughput (conductor-emit, P-026)
**Date:** 2026-06-18
**Commits:** (uncommitted at report time — this wrap commits) chunk work since `pii-payload-corpus`; Epoch 3 chunk 8/8 (closes Epoch 3)

## Changes (structured — detectors read this)
- **Files:** NEW `crates/conductor-emit/src/rate.rs` · NEW `crates/conductor-emit/tests/traffic_rate_ramps.rs` · MOD `crates/conductor-emit/src/lib.rs` (module decl + re-export + module-doc list entry).
- **Symbols / APIs:** new public (library-internal to the `conductor-emit` seam) — `RateCurve` (enum: `Ramp{from_rate,to_rate,windows}` / `Breathing{center_rate,amplitude,period_windows,windows}`), `RateCurve::ramp`, `RateCurve::breathing`, `RateCurve::windows`, `RateCurve::window_counts`, `rate_trace_request`. Re-exported from `lib.rs` as `pub use rate::{rate_trace_request, RateCurve}`. **No IPC method · no endpoint · no event · no socket · no port · no env var.**
- **Crates / modules:** added module `rate` inside the existing, already-registered `conductor-emit` crate. **No new/removed crate.**
- **Dependencies:** **none added · none bumped.** `Cargo.toml` unchanged — reuses `rand_chacha` + `rand_core` (ChaCha8Rng/seed_from_u64) + `opentelemetry-proto`/`tonic` already present; dev-deps `tokio`/`tokio-stream` already present.
- **Schema / config:** **none** — no scenario-config/garde wiring, no emission-journal line-schema change, no `runs.db` change, no violation schema. Window counts are returned in-memory; nothing is journaled.
- **Coverage of new surfaces:**
  - `RateCurve` / `window_counts` (rate-shape math) → external-input **n/a** (args are programmatic `u32`/`usize`; **no** scenario-config / `CONDUCTOR_*` path / MCP child stdout / deserialized-struct boundary) · instrumentation **n/a** (pure function, no hot-path op) · PII **n/a** · tests **unit ✓** (7) · a11y **n/a** · tokens **n/a**.
  - `rate_trace_request` → OTLP **PRODUCT-stream** trace builder (synthetic root OK spans, seeded ids) emitted via the existing `TraceEmitter` (loopback stub in tests) → validation **n/a** · instrumentation **n/a** (pure builder; the egress span lives on `TraceEmitter::export`, unchanged — no new `#[tracing::instrument]`, no new span name) · PII **n/a** (synthetic span name + random ids, no payload) · tests **unit ✓ + integ ✓** (loopback) · a11y **n/a** · tokens **n/a**.
  - Self-observation: **unchanged** — no new `tracing` logging, no OTel SDK init/use (only the `opentelemetry-proto` PRODUCT types), no artifact/log/`runs.db` writes ⇒ no host-path / struct-name emission.

## Deviations from intent
- **None material** — implemented exactly per `plan.md` (jitter ≈±15% for seed-materiality; `window_counts` public as the asserted breathing-shape contract; breathing constructor rejects `amplitude ≥ center_rate`; plain `#[test]` — `conductor-emit` has no rstest dev-dep, matching latency/pii/topology precedent).
- **Smoke skipped (no boot-path change)** — pure library primitive; no binary/entry-point/harness-script touched. `agent-run.sh status` is a per-`run_id` disk reader (needs a run; none exists — producing one needs the operator/Pulse-gated `run` leg). Not a failure.
- **Impl detail:** `window_counts(seed)` and `rate_trace_request(...)` each seed their own `ChaCha8Rng` from the same `seed` (two independent deterministic streams: counts vs span-ids) rather than threading one RNG as `latency.rs` does — the natural consequence of the plan's *public, seed-based* `window_counts`. Determinism holds (both are pure functions of `seed`; verified same-seed/diff-seed).
- **Coverage:** 1 uncovered line in `rate.rs` (99.34%) is a defensive guard path (`windows ≤ 1` ramp branch or the `.max(0.0)` clamp that `target ≥ 0` makes unreachable); no acceptance criterion depends on it.

## Decisions & corrections
- No user corrections this session; the user reviewed the plan at phase-P5 and chose **Apply** (no scope change requested).
- Design decisions carried from the plan (all spec-aligned, no new convention introduced): seeded per-window jitter is a *correctness* requirement (an un-jittered curve makes the seed inert → the diverge test can't pass); `window_counts` is public (deliberate divergence from latency's private sampler — the count sequence IS the P-026 "breathing shape" and the asserted contract); two-RNG determinism (above).
- Candidate Tier-3 note for curation: "a seeded *count/rate* primitive needs seeded jitter for the seed to be material — exact-target rounding makes the seed inert" (generalizes the 2026-06-16 both-directions learning to count-shaping). Possibly a dedup of the existing learning.

## Outcome
- **All acceptance criteria met.** Gates green (first-run, 0 fix-loop iterations):
  - `cargo nextest run -p conductor-emit` → **65/65** (was 57; +7 unit +1 integration)
  - `cargo clippy --workspace --all-targets -- -D warnings` → **clean**
  - `cargo nextest run --workspace --profile ci` → **146/146** (was 138)
  - `cargo test -p conductor-emit --doc` → **0** (no executable examples)
  - `cargo llvm-cov nextest -p conductor-emit --fail-under-lines 60` → **rate.rs 99.34% lines** / total 97.65%
- **Smoke:** skipped — no boot-path change (pure `conductor-emit` library primitive).
