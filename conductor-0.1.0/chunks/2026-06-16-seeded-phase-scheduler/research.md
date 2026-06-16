# Codebase Research — 2026-06-16-seeded-phase-scheduler

## Scope
- **Depth:** moderate · **Reads:** 7 · **Globs/Greps:** 4
- The target crate (`conductor-timeline`) is an empty placeholder; `conductor-core` is mature. Research
  focused on the dependency-wiring convention, the seed source, and the (absent) span pattern.

## Files inspected
- `crates/conductor-timeline/src/lib.rs` (full, 1 line) — only a doc comment
  `//! Deterministic seeded phase scheduler (tokio current_thread).`; empty placeholder this chunk fills.
- `crates/conductor-timeline/Cargo.toml` (full) — sole dep `conductor-core.workspace = true`; **no tokio, no
  RNG, no tracing**. This chunk adds them.
- `Cargo.toml` (workspace, full) — `[workspace.dependencies]` already pins `tokio = "1.48"` (line 30) but
  **no crate consumes it yet**; **no `rand`/`rand_chacha`** present; `thiserror = "2.0.18"` (48); resolver 3,
  edition 2024, `rust-version = "1.94.1"` (17).
- `crates/conductor-cli/src/main.rs` (full) — `fn main()` is **non-async** (not `#[tokio::main]`), just calls
  `conductor_core::init_observability("conductor", None)`. CLI runtime wiring is deferred → not touched here.
- `crates/conductor-core/Cargo.toml` (full) — feature-select convention `garde = { workspace = true,
  features = ["derive"] }` (12); dev-deps `rstest`/`proptest`/`insta` via `.workspace = true`.
- `crates/conductor-core/src/obs.rs` (grep) — tracing used via macros only (`tracing::info!` :65,
  `tracing::error!(panic=…)` :92); the subscriber + `std::panic` hook live here.
- Living docs `.andromeda/context/{dependency-tree,api-surface}.md` — current module/API + dep graph.

## Patterns detected
- **Crate-per-seam edge** (`crates/conductor-timeline/Cargo.toml:9`): `conductor-core.workspace = true` is the
  only dependency; the compiler enforces the seam. New deps join as `*.workspace = true` + a `features` list.
- **Workspace-pin + per-crate feature-select** (`crates/conductor-core/Cargo.toml:12`):
  `garde = { workspace = true, features = ["derive"] }` is the house pattern; `tokio`/RNG follow it.
- **tokio pre-declared, unconsumed** (`Cargo.toml:30`): `tokio = "1.48"` already in `[workspace.dependencies]`;
  this chunk is its **first consumer** — features are chosen at `conductor-timeline`, version stays workspace-pinned.
- **tracing via macros, zero spans** (`crates/conductor-core/src/obs.rs:65,92`): no `#[tracing::instrument]`
  exists anywhere → `timeline.execute` is the **first span**; follows obs-plan §4 + the bounded span-name set
  in `.claude/rules/observability.md` (no prior span to copy).
- **Seed already modeled** (api-surface: `Scenario { name, p_ids, seed: u64, slo_tier }`): the `u64` seed
  source exists in `conductor-core`, but `Scenario` has **no phases field** → the scheduler takes a `u64` seed +
  its own minimal phase type; it need not depend on `Scenario` (the per-phase emission spec is the next chunk).
- **No RNG in-tree** (grep): `rand`/`rand_chacha` appear only transitively under `proptest`
  (`dependency-tree.md:108-117`, rand 0.9.4 / rand_chacha 0.9.0) — a **direct** seedable-RNG dep is new.

## Conventions to follow
- **All deps `*.workspace = true`** — add the RNG to `[workspace.dependencies]` first (single version pin),
  then consume it (+ tokio) from `conductor-timeline` with explicit `features` (`Cargo.toml:20-65` precedent).
- **Typed `thiserror` enum per seam** — define a `TimelineError` (thiserror 2.0.18) for harness faults only;
  outcomes are values / `Result::Err` is harness-only (verdict-error wall), mirroring `conductor-core::CoreError`.
  No `panic!`, no `unsafe` (security extract; `.claude/rules/security.md`).
- **Determinism test idiom** — `#[tokio::test(flavor="current_thread", start_paused=true)]` + `tokio::time::advance`;
  assert ordering/shape + virtual-clock gap placement, never real wall-clock (`.claude/rules/testing.md`
  §Determinism; test-plan §4). Needs tokio dev-features `test-util` + `macros` (+ `rt`,`time`).
- **Bounded span name** — `#[tracing::instrument(name = "timeline.execute", …)]`; `timeline.execute*` is in the
  allowed set; no high-cardinality names (`.claude/rules/observability.md` §Spans; obs extract).
- **Coverage floor** — new-code line coverage ≥ 60 (`cargo llvm-cov nextest --fail-under-lines 60`,
  `.claude/rules/testing.md` §Running).

## New files to create
- `crates/conductor-timeline/src/lib.rs` — populate the placeholder: module decls + public re-exports.
- A scheduler module (e.g. `src/scheduler.rs`) — the seeded phase scheduler: an `async fn` that sequences an
  ordered phase list on `tokio::time::sleep`, holds a seeded `ChaCha8Rng`, surfaces `PhaseTransition` events;
  `TimelineError`.
- A phase-model module (e.g. `src/phase.rs`, or inline in lib) — the minimal `Phase`/`PhaseTimeline` (ordered
  phases + per-phase duration) + the `PhaseTransition` event type.
- Tests — in-crate `#[cfg(test)]` and/or `crates/conductor-timeline/tests/` for determinism + virtual-clock timing.
- (Exact module split is P4's call; arch §Directory structure names the crate "deterministic seeded phase
  scheduler (tokio::time)".)

## Files to modify
- `crates/conductor-timeline/Cargo.toml` — add `tokio = { workspace = true, features = ["time"] }` to
  `[dependencies]`, `tracing.workspace = true`, and the RNG dep; add `[dev-dependencies]`
  `tokio = { workspace = true, features = ["test-util","macros","rt","time"] }` + `rstest`/`proptest`/`insta`.
- `Cargo.toml` (workspace) — add the seedable-RNG dep to `[workspace.dependencies]` (see open question #2);
  `Cargo.lock` regenerates and stays committed + un-drifted (supply-chain invariant).
- **NOT modified:** `crates/conductor-cli/src/main.rs` (the `#[tokio::main(flavor="current_thread")]` wiring is
  a later CLI-epoch concern); `conductor-core` (seed already present, no change needed).

## Open questions
1. **Does the scheduler exercise a seeded decision this chunk, or only construct + hold the PRNG?** A purely
   declared ordered schedule is already deterministic without the RNG. Recommendation: wire `seed → ChaCha8Rng`
   and prove reproducible draws now (e.g. a deterministic per-phase value or optional bounded gap-jitter),
   leaving stochastic *emission* decisions to later chunks — so "same seed ⇒ same shape" is non-trivially
   tested. Resolve at P4.
2. **RNG crate:** `rand_chacha::ChaCha8Rng` (algorithm-stable across platforms/versions → reproducible; the
   right call for a determinism substrate) + `rand_core` for `SeedableRng::seed_from_u64`, vs `rand::rngs::StdRng`
   (no stability guarantee). Recommend `rand_chacha`; must clear `cargo audit` + `cargo deny` (both MIT/Apache).
3. **`emission_count` span attribute:** obs-plan §4 lists `phase_count` + `emission_count` on `timeline.execute`;
   emission doesn't exist yet → set `phase_count` now, defer/zero `emission_count` until the emit chunk. Minor.
