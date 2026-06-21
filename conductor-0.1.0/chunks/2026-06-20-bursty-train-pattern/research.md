# Codebase Research — 2026-06-20-bursty-train-pattern

## Scope
- **Depth:** moderate · **Reads:** 6 (`gap.rs`, `silence.rs`, `error.rs`, `port_occupier.rs`, `lib.rs`, `Cargo.toml`) · **Graph queries:** 5 (`tree-query-2026-06-20-bursty-train-pattern.json`)

## Files inspected
- `crates/conductor-faults/src/gap.rs` (full) — the **closest analog**: a validated `Result<Self, FaultError>` constructor over a `Duration`, `MIN_GAP`/`MAX_GAP` const bounds at module top, `gap()` + `gap_ms()` accessors, `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`, a doctest on `new`, and a `#[cfg(test)]` module with boundary-exclusive/inclusive + distinct-inputs tests. This is the template to mirror.
- `crates/conductor-faults/src/silence.rs` (full) — the infallible-marker branch (`AbruptSilence`, `Default`, `resumes()->false`); shows the contrast a parameterized `BurstyTrain` is NOT.
- `crates/conductor-faults/src/error.rs` (full) — `FaultError` is `#[derive(Debug, thiserror::Error)] #[non_exhaustive]` with per-variant `#[error("…")]` Display; current variants `Bind` / `GapTooShort` / `GapTooLong`. Each has a Display-string unit test.
- `crates/conductor-faults/src/port_occupier.rs` (full) — third helper; note it also ships a crate-local integration test at `tests/port_occupier.rs` (graph shows it), the RAII/`occupy_default()` convenience-constructor pattern.
- `crates/conductor-faults/src/lib.rs` (full) — crate doc "Shipped so far" list (currently 3 helpers) + `mod`/`pub use` re-export block; the two edit sites for registration.
- `crates/conductor-faults/Cargo.toml` (full) — deps are exactly `conductor-core` + `thiserror` (both workspace). `std::time::Duration` covers durations; **no new dep needed**.

## Graph impact (from the code-graph query)
- **`FaultError`** — every reference is inside `crates/conductor-faults/src/error.rs` (variant defs + the in-file Display tests). **No other crate consumes `FaultError`** → adding variant(s) is a zero-blast-radius change, and `#[non_exhaustive]` already shields any future external `match`.
- **`conductor-faults` dependents** — `crate_edges WHERE to_crate='conductor-faults'` returns **[]**: nothing imports the crate yet (timeline/scenario wiring is Epoch 7). Adding `BurstyTrain` cannot break any caller.
- **`EmissionGap` / `AbruptSilence`** — all refs are within their own modules + in-crate tests; `gap_ms()` has **no external consumer yet** (it exists in anticipation of the Epoch-7 `PhaseSpec` boundary). The new `*_ms` accessors will be the same: defined now, consumed Epoch 7.
- **Collision check** — no symbol matching `bursty`/`BurstyTrain`/`train` exists. `train.rs` + `BurstyTrain` are free at the symbol level (mirrors the master-route marker collision-check result).

## Patterns detected
- **Three-branch fault-constructor idiom** (`gap.rs:44`, `silence.rs:29`, `port_occupier.rs:29`): `Result<Self, FaultError>` for a validated bound · infallible `Self` for none · (the `Option` branch is documented in the handoff but unused so far). `BurstyTrain` has two positive durations to validate ⇒ the `Result` branch, exactly like `EmissionGap`.
- **Const bounds at module top** (`gap.rs:18,22`): `MIN_GAP`/`MAX_GAP` as `pub const Duration`, re-exported from `lib.rs`. A bursty-train would follow with its own bound consts (e.g. a max active/quiet ceiling).
- **`*_ms` accessor with documented-safe narrowing** (`gap.rs:61-64`): `as_millis() as u64` is annotated safe because the value is validated `<= MAX_GAP`. Reuse the same justification comment.
- **Doctest-on-constructor** (`gap.rs:35-43`): every public constructor carries a runnable ```` ``` ```` example asserting the happy path + one `is_err()` rejection.
- **In-module `#[cfg(test)]` + optional crate-local `tests/`** : `gap`/`silence` test in-module; `port_occupier` additionally has `tests/port_occupier.rs`. A pure-logic descriptor only needs the in-module tests (no I/O to integration-test).

## Conventions to follow
- Module is **concept-named** (`train.rs`), matching `gap.rs`/`silence.rs`/`port_occupier.rs` (arch §Conventions — Naming).
- Derive `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` on the value type (`gap.rs:26`).
- New `FaultError` variant(s) carry a `#[error("…")]` Display string + a Display unit test (`error.rs:24-28, 47-66`).
- `lib.rs`: add `mod train;`, extend the `pub use` block, and extend the crate-doc "Shipped so far" to 4/4 (mirrors how `silence` was registered).
- Verdict/error wall: out-of-range ⇒ typed `FaultError`, never panic (every extract; arch + security + obs + tests agree).

## New files to create
- `crates/conductor-faults/src/train.rs` — `BurstyTrain` (active/quiet duty-cycle descriptor) + bound consts + accessors + phase query + `#[cfg(test)]` tests + constructor doctest.

## Files to modify
- `crates/conductor-faults/src/lib.rs` — `mod train;` + `pub use train::{BurstyTrain, …}` + crate-doc "Shipped so far" 4/4 line.
- `crates/conductor-faults/src/error.rs` — add the duty-cycle bound variant(s) to `FaultError` (+ Display test) **iff** the `Result` (validated) shape is chosen at P4.

## Open questions
1. **Constructor shape** — validated `BurstyTrain::new(active, quiet) -> Result<Self, FaultError>` (mirrors `EmissionGap`, adds `FaultError` variant(s)) vs. a fixed-canonical 5min/10min marker (infallible, mirrors `AbruptSilence`). The route's "active 5min / quiet 10min" reads as canonical *defaults*, not the only values. → **P4 AskUserQuestion** (the "Helper shape" decision, exactly as the silence chunk did).
2. **Phase-query + accessor API** — confirm the surface: `active()`/`quiet()`/`period()` Durations, `*_ms()` accessors, and the within-cycle query `is_active_at(offset) -> bool` (`offset % period < active`). Resolve at P4 from the obs/tests extract signals.
3. (none further)
