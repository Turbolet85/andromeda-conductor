# Codebase Research — 2026-06-18-latency-shaping

## Scope
- **Depth:** moderate · **Reads:** 7 (`lib.rs`, `Cargo.toml`, `message.rs`, `logs.rs`, `span_tree.rs`, `scenario.rs`, `phase_spec.rs`) · **Globs/Greps:** 3 (emit src glob, ChaCha8 grep, core latency grep) · **code-graph:** 2 queries

## Files inspected
- `crates/conductor-emit/src/lib.rs` (full) — module list (`client`/`error`/`exception`/`logs`/`message`/`span_tree`) + the public re-export surface a new `latency` module joins.
- `crates/conductor-emit/Cargo.toml` (full) — deps already include `rand_chacha` + `rand_core` (seeded RNG ready) and `opentelemetry-proto` with `features = ["gen-tonic", "trace", "logs"]`. **The `trace` feature is already on** — latency shaping reuses trace types, so it needs **no new Cargo feature** (unlike the logs chunk, which added `"logs"` for the log proto types).
- `crates/conductor-emit/src/message.rs` (full) — shared raw-struct primitives: `pub(crate) span()` / `span_with_events()` set `start_time_unix_nano == end_time_unix_nano == unix_nanos()` (**zero-duration spans today**). `pub(crate) unix_nanos()` = `SystemTime::now()` epoch nanos; `pub(crate) service_resource()` / `string_kv()`. The duration-shaping hook is here: end must become `start + sampled_duration`.
- `crates/conductor-emit/src/logs.rs` (full) — **the module template**: a typed constructor-validated wrapper (`Severity::new(1..=24) -> Option<Self>`), a `*_request` builder reusing `message::{service_resource, unix_nanos}`, and a `shape()` projection test that excludes wall-clock fields. **Seedless** (severity is caller-given) — latency differs in being seeded.
- `crates/conductor-emit/src/span_tree.rs` (full) — **the seeded-builder template**: `error_trace_request(service_name, seed: u64, …)` → `let mut rng = ChaCha8Rng::seed_from_u64(seed)` → threads `&mut rng` into `pub(crate) gen_id::<N>(rng)`. Determinism tests: `shape()` (identity+linkage+status, **no timestamps**), `same_seed_reproduces_identical_shape`, `different_seeds_diverge` (≥2 seeds, multiple draws).
- `crates/conductor-core/src/scenario.rs` (70–139) — `scenario.rs:97`: *"The latency-target ordering invariants (p50≤p95≤p99) join it when the Epoch-3 latency spec lands"* — refers to the eventual **scenario-config** garde wiring, not the primitive's own type.
- `crates/conductor-core/src/phase_spec.rs` (full) — **decisive on the type-location fork** (line 6–8): the concrete OTLP taxonomy incl. *"latency targets"* lands in the **Epoch-3 emission seam**, extending `EmissionSpec` (`#[non_exhaustive]`) later *without reshaping `PhaseSpec`/the scenario model*. No latency type exists in core today.

## Graph impact (from the code-graph query)
- **`crate_edges` FROM `conductor-emit`** → `[]`; **TO `conductor-emit`** → `[]`. **Zero workspace caller/callee edges** — confirms the handoff's "zero external callers of conductor-emit". This is a **standalone Epoch-3 primitive**: adding `latency.rs` + lib re-exports has no downstream blast radius; nothing to migrate.
- No core symbol is referenced by emit today (emit imports neither `conductor_core` types in `message.rs`/`logs.rs`) — consistent with keeping the `LatencyProfile` type local to emit.

## Patterns detected
- **Seeded builder** (`span_tree.rs:43`, `exception.rs:133`): top-level `…_request(service_name, seed, …)` seeds `ChaCha8Rng::seed_from_u64(seed)` internally and threads `&mut rng`; identity/sampled output is a deterministic function of `seed`, no clock read for the seeded part.
- **Wall-clock vs seeded split** (`message.rs:65-89`, `logs.rs:4-5`): timestamps are `std::time` (`unix_nanos()`); the determinism contract governs only the non-clock shape. For latency, the **duration** (`end − start`) is the seeded output; the absolute `start`/`end` are wall-clock.
- **Determinism shape-projection test** (`span_tree.rs:98-112`, `logs.rs:86-99`): tests assert on a projection that excludes wall-clock — for latency, project to the per-span **duration** (`end − start`) sequence.
- **Constructor-validated typed wrapper** (`logs.rs:30-45` `Severity::new`): an emit primitive validates its own bounded input via a constructor returning `Option`, not via core's garde — the model for `LatencyProfile::new(p50, p95, p99)` asserting `p50 ≤ p95 ≤ p99`.
- **`#[non_exhaustive] EmissionSpec`** (`phase_spec.rs:43-49`): the forward-compat hook for the *later* scenario-config wiring of latency targets — out of scope here, noted so the plan doesn't touch it.

## Conventions to follow
- Both-directions determinism test (testing.md 2026-06-16): same seed ⇒ identical duration sequence AND ≥2 seeds ⇒ divergent (multiple draws), per `span_tree.rs:155-170`.
- Seeded output carrying wall-clock fields ⇒ assert on a shape-projection excluding the clock (testing.md 2026-06-17 error-spans) — project to durations, not absolute nanos.
- Integration/egress tests drive a loopback `TraceService` tonic stub on `127.0.0.1:0` (testing.md 2026-06-17; `tests/egress.rs`); **never bind real `:4317`** (reserved for the Epoch-4 port-occupier).
- `std::time` for stamps, never tokio's virtual clock (CLAUDE.md invariant; `message.rs:109`).
- No new self-obs span name unless registered in the bounded set (observability.md §Spans) — keep the primitive a **pure function** (no I/O, no span) to avoid an obs amendment.

## New files to create
- `crates/conductor-emit/src/latency.rs` — `LatencyProfile` (p50/p95/p99 ms, `new()` asserting ordering, mirroring `Severity`); a seeded duration sampler (profile + n + `&mut ChaCha8Rng` → durations realizing the target percentiles over ≥50 samples); a `latency_trace_request(service_name, seed, &[op…])` builder producing per-operation spans with shaped `end − start` durations. + `#[cfg(test)]` unit tests.
- `crates/conductor-emit/tests/latency_shaping.rs` — integration test over the loopback `TraceService` stub: shaped durations appear per operation; per-operation independence; refused transport ⇒ `EmitError` (mirroring `tests/egress.rs` / `tests/severity_logs.rs`).

## Files to modify
- `crates/conductor-emit/src/message.rs` — add a duration-aware span constructor (e.g. `pub(crate) fn timed_span(name, ids…, status, duration_nanos)` setting `end = start + duration_nanos`), reusing `unix_nanos()`; the existing zero-duration `span()` stays for the OK/error builders.
- `crates/conductor-emit/src/lib.rs` — `mod latency;` + `pub use latency::{LatencyProfile, latency_trace_request, …}`.
- (No `Cargo.toml` change — `trace` proto feature already enabled; `rand_chacha`/`rand_core` already deps.)

## Open questions
- **Sampling algorithm** (implement-time, non-blocking): recommend a piecewise inverse-CDF anchored at 0.5→p50 / 0.95→p95 / 0.99→p99 (+ a `0` floor and a bounded tail past p99) through a seeded uniform — hits the anchor percentiles by construction. Final method is `/andromeda-implement`'s call.
- **Realized-percentile test tolerance** (implement-time): the unit assertion that realized p50/p95/p99 ≈ targets needs a small sampling-convergence band (the EMIT-side test tolerance; distinct from verify's ±15% SLO, which is Epoch-5). Pick when writing the test.
