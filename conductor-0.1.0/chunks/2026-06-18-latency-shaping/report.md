# Report — 2026-06-18-latency-shaping

**Chunk:** Latency shaping — seeded per-operation span-duration sampling realizing target p50/p95/p99 over the ≥50-sample latency floor (conductor-emit, P-011/P-012)
**Date:** 2026-06-18T17:38:32Z
**Commits:** (uncommitted at report time — wrap commits) feat(2026-06-18-latency-shaping): latency shaping — seeded per-operation span durations (conductor-emit, P-011/P-012)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-emit/src/latency.rs` (new) · `crates/conductor-emit/tests/latency_shaping.rs` (new) · `crates/conductor-emit/src/message.rs` (modified) · `crates/conductor-emit/src/lib.rs` (modified)
- **Symbols / APIs:** new public exports from `conductor-emit`: `LatencyProfile` (struct; `new(p50_ms,p95_ms,p99_ms)->Option<Self>` enforcing p50≤p95≤p99; `p50_ms`/`p95_ms`/`p99_ms` accessors) · `LatencyOp<'a>` (struct: `operation:&str`, `profile:LatencyProfile`, `samples:usize`) · `latency_trace_request(service_name:&str, seed:u64, &[LatencyOp]) -> ExportTraceServiceRequest`. Internal: `message::timed_span` (`pub(crate)`; `end = start + duration_nanos`). **No new IPC method / endpoint / socket / port / env var.**
- **Crates / modules:** `conductor-emit` gains module `latency` (+ `mod latency;` / re-exports in `lib.rs`). No crate added/removed. No cross-seam edge added (code-graph: zero edges to/from conductor-emit).
- **Dependencies:** **none added, none bumped.** `rand_chacha`/`rand_core` were already crate deps; `opentelemetry-proto` `trace` feature already enabled (no `Cargo.toml` change). **`Cargo.lock` un-drifted.**
- **Schema / config:** none (no migration, no config key, no violation schema). The p50≤p95≤p99 invariant is **constructor-validated inside conductor-emit** (mirroring `logs::Severity::new`); the scenario-config garde wiring (extending `EmissionSpec`, the `scenario.rs:97` note) is **explicitly deferred** to the scenario epoch — not this chunk (per `phase_spec.rs:6`).
- **Coverage of new surfaces:**
  - `latency_trace_request` (emit library primitive) → validation **n/a** (input is a typed `LatencyProfile`, constructor-validated emit-side; NOT an external-input / deserialization / `CONDUCTOR_*`-path / MCP-stdout boundary) · instrumentation **n/a** (pure builder — emission rides the existing `emit.batch` span on `TraceEmitter::export`; **no new self-obs span by design**, bounded span-name set unchanged) · PII **n/a** (synthetic numeric durations + caller-supplied benign operation names; no host paths / struct names; no logging or artifact write added) · tests **unit(7)+integ(2) ✓** · a11y **n/a** (no UI surface) · tokens **n/a** (no UI surface)

## Deviations from intent
- **Operation descriptor is a named public `LatencyOp<'a>` struct**, not a bare tuple (plan step 4 left the param shape open as `operations: &[…]`). *Justified:* documented fields mirror the typed-wrapper convention (`Severity`); re-exported alongside `LatencyProfile`. In-scope (latency.rs + lib.rs, both planned touchpoints).
- **Sampler = stratified inverse-CDF** (one seeded uniform per stratum through the piecewise-linear quantile), resolving research open-question 1 (implement's call). *Justified:* low-discrepancy ⇒ tight percentile convergence at the ≥50 floor while staying seed-sensitive (both-directions determinism holds).
- **Convergence test: ±12% tolerance at n=2000** (research open-question 2). *Justified:* realized error <1% with stratified sampling + interpolated percentiles; ±12% is robustness headroom that still meaningfully asserts convergence.
- **`LatencyProfile` accessors unused internally** (the 3 "missed functions" → latency.rs 95.57% line coverage). *Justified:* intentional public API for downstream consumers (verify / scenario-wiring), not dead code; `pub` so no dead_code lint.

## Decisions & corrections
- **Profile-type location:** `LatencyProfile` + the p50≤p95≤p99 check live in **conductor-emit**, not core — settled by `phase_spec.rs:6-8` ("the concrete OTLP taxonomy … latency targets … lands in the Epoch-3 emission seam, extending `EmissionSpec` later without reshaping the scenario model") + the `Severity::new` precedent (an emit primitive self-validates its bounded input). Core's scenario-config garde remains the authoritative validator for the *future* EmissionSpec latency field.
- **Determinism on durations, not timestamps:** the seed governs the duration sequence (`end−start`); absolute `start`/`end` are wall-clock `unix_nanos()`. Determinism tests project to durations (reusing the error-spans wall-clock-projection learning) — no whole-`Span` golden.
- **No new self-obs span / no new Cargo feature** — kept the primitive a pure function (avoids touching the bounded span-name set / obs amendment); reuses the already-enabled `trace` proto types.

## Outcome
- **All 9 plan acceptance criteria met.** Gates green (first run, zero fix iterations): `cargo nextest run -p conductor-emit` (34 passed) · `cargo nextest run --workspace --profile ci` (115 passed, was 106) · `cargo clippy --workspace --all-targets -- -D warnings` (clean) · `cargo llvm-cov nextest -p conductor-emit --fail-under-lines 60` (95.96% total, latency.rs 95.57%) · `cargo audit` + `cargo deny check` (clean; advisories/bans/licenses/sources ok) · `Cargo.lock` un-drifted.
- **Real smoke (P3): skipped — no boot-path change** (library primitive in conductor-emit; zero external callers; no binary/daemon/entry-point; plan lists no `agent-run.sh` command).
