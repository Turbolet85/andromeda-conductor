# Codebase Research — 2026-06-19-emission-gap-resume

## Scope
- **Depth:** moderate · **Reads:** 6 (port_occupier.rs, error.rs, lib.rs, Cargo.toml, rate.rs, phase_spec.rs; phase.rs from setup) · **Globs/Greps:** 3 + 3 code-graph queries

## Files inspected
- `crates/conductor-faults/src/port_occupier.rs` (full) — the self-contained fault-primitive precedent: a validating ctor `occupy(port) -> Result<Self, FaultError>`, a hard-coded `const OTLP_INGEST_PORT`, idempotent `release()`, RAII `Drop`, **no garde, no seed**. The shape this chunk mirrors.
- `crates/conductor-faults/src/error.rs` (full) — `FaultError` is `#[non_exhaustive]`, currently only `Bind { addr, #[source] io::Error }`; crate-doc says *"later Epoch-4 fault helpers extend the surface."* Display+source unit test at :28-38 is the error-test pattern.
- `crates/conductor-faults/src/lib.rs` (full) — `mod x; pub use x::{...}` re-export pattern (:9-13); crate-doc enumerates helpers (`ramps · silence · port-occupier · fingerprint-storm`).
- `crates/conductor-faults/Cargo.toml` — deps are **only** `conductor-core` + `thiserror`. No tokio, no rand.
- `crates/conductor-emit/src/rate.rs` (full) — the seeded-generator precedent (`RateCurve`): **`Option<Self>` validating constructors** (`ramp`/`breathing` → `None` on invalid bounds), `window_counts(seed)` driving output via `ChaCha8Rng::seed_from_u64` + bounded `JITTER`, and the canonical determinism tests `same_seed_reproduces_identical_counts` + `different_seeds_diverge` (:178-188).
- `crates/conductor-core/src/phase_spec.rs` (full) — `PhaseSpec { name, gap_ms: u64, emission }` with garde `#[garde(range(max = MAX_GAP_MS))]`; **`MAX_GAP_MS = 3_600_000`** is documented as sized *"for the Epoch-4 bursty-train ~10-minute quiet window."* The config/TOML validation idiom (distinct from a runtime helper type). `MAX_GAP_MS`/`MAX_JITTER_MS` are `pub(crate)` (not exported).
- `crates/conductor-timeline/src/phase.rs` (setup read) — `Phase { name, gap: Duration }`, `PhaseTimeline { jitter }`: the scheduler **perturbs each phase gap by ± seeded jitter** (`Duration::ZERO` = exact). This is precisely the jitter a P-015 exact gap must avoid.

## Graph impact (code-graph query → tree-query-2026-06-19-emission-gap-resume.json)
- **`conductor-faults` crate edges = `[]`** — a leaf crate: nothing depends on it yet, and it calls nothing outside itself. **Extending `FaultError` / adding a public type has zero external blast radius.** (Timeline/emit/CLI consumers arrive in Epochs 7/8.)
- **`FaultError`** (error.rs:13, variant `Bind` :16) — referenced only intra-crate (constructed at port_occupier.rs:32,35). A new variant adds an arm only here; `#[non_exhaustive]` already forces wildcard arms on any future external `match`.

## Patterns detected
- **Self-contained fault primitive** (port_occupier.rs:25-56): a small struct + a `Result<Self, FaultError>` validating constructor + hard-coded domain constants; no seed, no garde, no async.
- **Seeded value generator** (rate.rs:47-116): `Option<Self>` for pure-value bound checks; `ChaCha8Rng` only where the output is genuinely random. The seed drives output ⇒ the `different_seeds_diverge` test applies — **N/A for an exact, un-jittered gap.**
- **Both-directions determinism test** (rate.rs:178-188): same-input⇒identical AND inputs-materially-drive-output. For an exact gap the driver is the *configured duration*, not a seed (cf. the fingerprint primitive's seed-independent derived value).
- **garde config validation** (phase_spec.rs:22-37): reserved for TOML-deserialized scenario config — not for a constructor-validated runtime helper.
- **Duration units:** core/config uses `u64` ms (`gap_ms`); the timeline `Phase` uses `std::time::Duration`.

## Conventions to follow
- New helper module in `conductor-faults` (sibling to `port_occupier.rs`); register via `mod` + `pub use` in `lib.rs` (lib.rs:9-13).
- Extend `FaultError` with a typed validation variant in `error.rs`, mirroring `Bind`'s `#[error("…")]` + structured fields; add a Display unit test (error.rs:28-38 pattern).
- In-module `#[cfg(test)]` unit tests, rate.rs-style: constructor validation (valid + boundary + reject) and exact-value preservation; plus the FaultError display test.
- No new Cargo deps — `std::time::Duration` + existing `thiserror`/`conductor-core` suffice.

## New files to create
- `crates/conductor-faults/src/gap.rs` — the exact emission-gap+resume helper type (`EmissionGap`): a validating ctor enforcing the >20s floor (+ sanity ceiling), an exact-`Duration` getter, the "resumes after the gap" semantic (the contrast with P-014 permanent silence), and `#[cfg(test)]` tests. A constructor doctest is safe here (no port bind, unlike port_occupier).

## Files to modify
- `crates/conductor-faults/src/error.rs` — add the sub-floor (and optional over-ceiling) `FaultError` variant + its Display unit test.
- `crates/conductor-faults/src/lib.rs` — `mod gap;` + `pub use gap::EmissionGap;` + extend the crate-doc "Shipped so far" line.

## Open questions (surface at P5 review — decided from precedent, user may veto)
1. **Validation idiom** — chosen `Result<Self, FaultError>` (extends the `#[non_exhaustive]` FaultError that is *designed* to extend; matches port_occupier + all substantive extracts; a named sub-floor error beats an anonymous `None` for the P-015-critical 20s threshold). Considered alternative: `Option<Self>` (rate.rs idiom for pure-value bounds).
2. **Seed involvement** — chosen exact / seed-independent (the gap is a fixed >20s value by definition; mirrors the fingerprint primitive's seed-independent derived value; avoids the "seed computed-but-never-applied" anti-pattern). The determinism check becomes exact-value-preservation, not `different_seeds_diverge`. This refines scope.md's "deterministic under seed" → "deterministic by construction (exact)" — same intent, no divergence.
