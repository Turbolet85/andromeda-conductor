# Codebase Research — 2026-06-19-abrupt-silence-fault

## Scope
- **Depth:** moderate · **Reads:** 5 (gap.rs, lib.rs, error.rs, port_occupier.rs, Cargo.toml) · **Globs/Greps:** 1 grep + 2 code-graph queries
- Small additive primitive in a mature, self-contained crate; the two shipped siblings fully fix the pattern.

## Files inspected
- `crates/conductor-faults/src/gap.rs` (full) — `EmissionGap`: the direct contrast. Fallible `new(Duration) -> Result<Self, FaultError>` validating `(MIN_GAP=20s, MAX_GAP=3600s]`; `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`; accessors `gap()` / `gap_ms()` ("the representation the timeline / `PhaseSpec` boundary consumes"); 6 unit tests + 1 doctest; module-doc explicitly says the resume "is the contrast with the permanent-silence fault (P-014)" — **this chunk fulfills that forward-reference.**
- `crates/conductor-faults/src/port_occupier.rs` (full) — `PortOccupier`: fallible `occupy(port) -> Result<Self, FaultError>` because it acquires an OS resource (`TcpListener::bind`); RAII `Drop` → `release()` (idempotent); `OTLP_INGEST_PORT` const. Fallibility here is driven by *resource acquisition*, not value validation.
- `crates/conductor-faults/src/error.rs` (full) — `#[non_exhaustive] enum FaultError` (`thiserror`): `Bind{addr,source}` (port), `GapTooShort{gap,min}` / `GapTooLong{gap,max}` (gap). Each variant has a Display test. `#[non_exhaustive]` is the documented extension point "so later Epoch-4 fault helpers extend the surface."
- `crates/conductor-faults/src/lib.rs` (full) — crate-doc lists the seam as "ramps · silence · port-occupier · fingerprint-storm"; `mod {error,gap,port_occupier}` + `pub use` of each public type. The "silence" concept slot is where this chunk lands.
- `crates/conductor-faults/Cargo.toml` (full) — deps are only `conductor-core` + `thiserror` (both workspace). `std::time::Duration` is used by gap.rs with **no dependency** — confirms abrupt-silence needs **no new deps**.

## Graph impact (from the code-graph query → `tree-query-*.json` adoption trace)
- **`conductor-faults` inbound crate edges = `[]`** — no other workspace crate uses it yet (timeline/cli wiring is Epoch 7/8). Adding a new `pub` type + `pub use` is **purely additive, zero external blast radius**.
- Current public surface (symbol query): `FaultError` (+ variants/fields), `EmissionGap` (+ `MIN_GAP`/`MAX_GAP`), `PortOccupier` (+ `OTLP_INGEST_PORT`) — all under `crates/conductor-faults/src/{error,gap,port_occupier}.rs`. Nothing references them cross-crate.

## Patterns detected
- **Two distinct fallibility drivers** (`gap.rs:44`, `port_occupier.rs:29`): `Result<Self, FaultError>` is used when there is *either* a value bound to validate (gap's 20s floor) *or* a resource to acquire (port bind). **Abrupt-silence has neither** → it is the first helper with nothing to fail on.
- **Module-per-helper, concept-named file, descriptive type** (`gap.rs`→`EmissionGap`, `port_occupier.rs`→`PortOccupier`): files are named by the fault concept, the type is descriptive.
- **Value helpers are `Copy` + full derives** (`gap.rs:26`): `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`; resource helpers are `Debug`-only (own a socket). Abrupt-silence is a pure value → follows the `EmissionGap` derive set.
- **Inline tests + a constructor doctest** (`gap.rs:35-43`, `gap.rs:67-116`): every helper ships `#[cfg(test)] mod tests` + a `///` doctest on the constructor.

## Conventions to follow
- New module declared `mod <name>;` + `pub use <name>::<Type>;` in `lib.rs`, alphabetical-ish next to siblings (`crates/conductor-faults/src/lib.rs:11-17`).
- Crate-doc (`lib.rs:1-9`) gets a new "Shipped so far" line for the helper (mirrors the `EmissionGap` line at `lib.rs:8-9`).
- `FaultError` extension (only if fallible) is additive under `#[non_exhaustive]` with a matching Display test (`error.rs:14`, `:35-66`).
- Determinism is tested as **reproducibility only** — NOT `different-seeds-diverge` (no seed exists; per testing-rule 2026-06-16/2026-06-18 and the `EmissionGap` precedent).

## New files to create
- `crates/conductor-faults/src/silence.rs` — the `AbruptSilence` helper (concept-named like `gap.rs`; houses the type + inline tests + constructor doctest). *(Module-name nit — `silence.rs` vs `abrupt_silence.rs` — settled in the plan; `silence.rs` mirrors `gap.rs`/`EmissionGap` and the obs `fault.silence` span name.)*

## Files to modify
- `crates/conductor-faults/src/lib.rs` — add `mod silence;` + `pub use silence::AbruptSilence;` + a crate-doc "Shipped so far" line.
- `crates/conductor-faults/src/error.rs` — **only if** the constructor is chosen fallible (research strongly favors infallible → `error.rs` likely **untouched**, itself the notable contrast).

## Open questions
1. **Constructor fallibility / shape — the one real design fork (→ P4).** Evidence (no bound, no resource, no parameter) strongly favors an **infallible, parameterless `AbruptSilence` marker** (`new() -> Self`, full value derives, `FaultError` untouched). The alternative — forcing a parameter/validation — would be the "computed-but-never-applied" anti-pattern the gap scope explicitly rejected for the seed. Worth a one-line user confirmation since it sets the crate's first infallible-helper precedent and the prior chunk deliberately chose `Result` over `Option`.
2. **Permanence must be positively testable, not just "absent."** Give the type a semantic the test can assert (e.g. a `resumes()->bool`/`is_permanent()` accessor, or a documented `Copy`/`Eq` marker) so "no resume boundary" is exercised, distinguishing it from `EmissionGap` — decided in the plan.
3. **(carried, deferred) obs `fault.silence` span lists `fault_duration_ms`** — a permanent stop has no finite duration; the span's permanent case (null/sentinel duration) is an Epoch-7/8 wiring concern, out of this chunk. Flagged so it is not lost.
