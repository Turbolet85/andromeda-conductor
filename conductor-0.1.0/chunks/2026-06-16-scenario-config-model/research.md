# Codebase Research — 2026-06-16-scenario-config-model

## Scope
- **Depth:** moderate · **Reads:** 9 (core: scenario/error/lib; timeline: phase/scheduler/lib; 3× Cargo.toml) · **Globs:** 3
- The two target crates already exist with real surfaces; this chunk extends them, so research is moderate (not cold-start). `scenarios/` is empty — the first fixture is net-new.

## Files inspected
- `crates/conductor-core/src/scenario.rs` (full) — `Scenario { name, p_ids: Vec<PId>, seed, slo_tier }`, derives `Debug,Clone,PartialEq,Eq,Serialize,Deserialize,Validate`. **No phase field yet.** Module doc (l.5-6) + l.66-67 explicitly reserve this chunk: *"The declarative per-phase emission spec is a later chunk. garde validation attaches here"* and *"the worked `custom` validator the Epoch-2 emission-spec invariants … will sit beside."* garde idioms in use: `#[garde(length(min=1))]`, `dive`, `custom(fn)`; `#[garde(skip)]` on `seed`/`slo_tier`; field-level `custom` (no container custom in 0.22.1).
- `crates/conductor-core/src/error.rs` (full) — `CoreError { Config(String), Validation(#[from] garde::Report) }`, `#[non_exhaustive]`; `pub type Result<T>`. Config-parse → `Config(String)`; garde fail → `Validation` via `#[from]`. The verdict/error-wall pieces this chunk needs already exist. Its test (l.51-57) constructs a `Scenario` literal — **will need new fields**.
- `crates/conductor-core/src/lib.rs` (full) — module list + re-exports; `pub use scenario::{PId, Scenario, SloTier}`. A new `phase_spec` type must be added to the module list + re-exported.
- `crates/conductor-timeline/src/phase.rs` (full) — `Phase { name: String, gap: Duration }` (`::new(name, gap)`); `PhaseTimeline { phases: Vec<Phase>, jitter: Duration }` (`::new(phases, jitter)`); `PhaseTransition { index, name, elapsed_ms }`. Timeline types derive `Debug,Clone,PartialEq,Eq` — **no serde** (they are runtime types, not config). Doc (l.5-6): these "carry timing only, never what a phase emits" — the emission *spec* is core's, timing is timeline's.
- `crates/conductor-timeline/src/scheduler.rs` (full) — `run_timeline(&PhaseTimeline, seed) -> Result<Vec<PhaseTransition>, TimelineError>`; `TimelineError::EmptyTimeline` on empty phases. `jitter` is a single symmetric ±bound applied per gap via `ChaCha8Rng`. The conversion must produce a `PhaseTimeline` this consumes unchanged.
- `crates/conductor-timeline/src/lib.rs` (full) — `pub use phase::{Phase, PhaseTimeline, PhaseTransition}`, `pub use scheduler::{run_timeline, TimelineError}`.
- `Cargo.toml` (workspace, full) — `[workspace.dependencies]` has serde, serde_json, garde 0.22.1, thiserror, tokio, rand_chacha/core. **No `toml`, no `serde_yaml`.** Adding a config-format crate is a new workspace dep (→ audit/deny re-green + Cargo.lock commit).
- `crates/conductor-core/Cargo.toml` (full) — deps: serde, serde_json, thiserror, garde(derive), tracing(+subscriber); dev: rstest, proptest, insta.
- `crates/conductor-timeline/Cargo.toml` (full) — deps: conductor-core, tokio(time), tracing, thiserror, rand_chacha, rand_core; dev: tokio(test-util,macros,rt,time), rstest, proptest, insta.

## Patterns detected
- **garde field-level validators** (`scenario.rs:15,52,56,68`): `#[garde(custom(fn))]` + `dive` + `length`; cross-cutting rules live on the field they concern (no container `custom` in 0.22.1). The new phase-spec validation follows the same shape.
- **serde-transparent newtype + renamed enum** (`scenario.rs:13-15,33-44`): `PId` is `#[serde(transparent)]`; `SloTier` variants are `#[serde(rename="<5s">)]`. Wire forms are deliberate.
- **Conversion ownership by orphan rule** (`dependency-tree`: `conductor-timeline → conductor-core`): timeline owns `PhaseTimeline`, so `impl From<&Scenario> for PhaseTimeline` is legal there; core must NOT depend on timeline (would reverse the seam).
- **Determinism test discipline** (`testing.md` Session Addition): seeded components assert BOTH directions (same-seed-reproduces AND different-seeds-diverge) with ≥2 known-divergent seeds; `#[tokio::test(flavor="current_thread", start_paused=true)]`.
- **Error wall** (`error.rs`, `scheduler.rs:21-27`): parse/validation faults are `Err(CoreError)`; never panic, never a verdict.

## Conventions to follow
- **Extend `Scenario`, don't duplicate** (`scenario.rs:5-6,66-67`): add the phase sequence + emission spec as new field(s) on the existing `Scenario`, with garde co-located; reuse `CoreError::Validation`/`Config`.
- **Conversion is total + infallible post-validation**: garde guarantees `phases` non-empty before conversion, so `From<&Scenario> for PhaseTimeline` is infallible; an empty timeline is caught later by `run_timeline → EmptyTimeline`.
- **Timeline types stay serde-free**; the serde/garde model is core's, the runtime `Phase`/`PhaseTimeline` is timeline's. Map `gap_ms`/`jitter_ms` (`u64`) → `Duration::from_millis(..)` at the conversion boundary.
- **Phase label legibility** (design AC, `design.md`): garde `length(max=40)` on the phase name (rejects un-renderable titlebar labels); identity is a short machine-readable label.
- **No new dep unless the format decision needs it** (`security.md` supply-chain): if TOML is chosen, add `toml` to `[workspace.dependencies]` and re-green audit/deny + commit `Cargo.lock`.

## New files to create
- `crates/conductor-core/src/phase_spec.rs` — `PhaseSpec` (serde + garde): `name` (`length(1..=40)`), `gap_ms: u64` (`range(max=..)`), and a minimal, `#[non_exhaustive]`/extensible declarative `emission` descriptor (the Epoch-3 emission taxonomy fills it; NOT built here). Re-exported via `lib.rs`. (May instead live inside `scenario.rs` beside the reserved comment — P4 picks placement.)
- `scenarios/error-baseline-spike.{toml|json}` — first representative fixture (P-009/P-010, a phase sequence, seed, slo_tier, jitter); format per the P4 user decision. Used by an end-to-end load→validate→convert test.

## Files to modify
- `crates/conductor-core/src/scenario.rs` — add `phases: Vec<PhaseSpec>` (`length(min=1)` + `dive`) and `jitter_ms: u64` (`range`) to `Scenario`; update the 7 in-module test struct-literals + `scenario_with` helper to set the new fields.
- `crates/conductor-core/src/error.rs` — update the `garde_report_becomes_a_validation_fault` test's `Scenario` literal (l.51-57) for the new fields.
- `crates/conductor-core/src/lib.rs` — add `mod phase_spec;` + re-export `PhaseSpec` (and any sub-types).
- `crates/conductor-timeline/src/` (new `convert.rs` or in `phase.rs`) — `impl From<&Scenario> for PhaseTimeline`; re-export nothing new (trait impl) or add a helper fn; add conversion + end-to-end determinism tests.
- `Cargo.toml` + `crates/conductor-core/Cargo.toml` — add `toml` dep **only if** TOML format chosen (else unchanged).

## Open questions
- **Config file format (P4 user decision):** `toml` (new dep, ergonomic, arch-leaning) vs `serde_json` (zero dep, matches JSONL/runs.db). Decides the fixture extension + whether a workspace dep is added.
- **`PhaseSpec` placement:** new `phase_spec.rs` module vs extending `scenario.rs` in place (beside the reserved comment) — P4 picks; both satisfy arch.
- Emission-descriptor richness is intentionally **minimal/forward-compatible** this chunk (Epoch 3 = "Emission primitives" owns the taxonomy: error spans, severity, fingerprints, latency p50≤p95≤p99, ramps). Not a user question — resolved by the route's epoch boundary.
