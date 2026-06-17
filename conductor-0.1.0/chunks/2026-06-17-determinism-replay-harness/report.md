# Report — 2026-06-17-determinism-replay-harness

**Chunk:** Determinism-replay harness — insta golden freeze of the absolute PhaseTransition stream shape (full Scenario-fixture→PhaseTimeline→run_timeline pipeline) + proptest replay/bounds/monotonic invariants across the seed space, all under tokio start_paused (conductor-timeline)
**Date:** 2026-06-17
**Commits:** none yet (uncommitted; this wrap creates the chunk commit). Epoch 2 (Timeline engine), chunk 4 of 4 — the epoch closer.

## Changes (structured — detectors read this)
- **Files:** NEW `crates/conductor-timeline/tests/replay.rs`; NEW `crates/conductor-timeline/tests/snapshots/replay__fixture_seed_424242.snap` + `…/replay__fixture_seed_7.snap`. **No production files modified** (`determinism.rs` untouched).
- **Symbols / APIs:** **NONE** — no public API change; no new exports / IPC methods / endpoints / ports / sockets / env vars. Test-internal only: `fixture_timeline()` + `arb_timeline()` helpers and 4 test fns (2 `#[tokio::test(start_paused)]` goldens + 2 `proptest!` properties), all in an integration-test target.
- **Crates / modules:** none added/removed/changed — work is confined to the existing `conductor-timeline` crate's `tests/` target.
- **Dependencies:** **NONE added or bumped.** `insta = "1"` and `proptest = "1"` are pre-existing workspace dev-deps already listed in `conductor-timeline/Cargo.toml`; `tokio` dev carries `test-util` already (enables `start_paused`).
- **Schema / config:** none (no migrations, config keys, or violation schemas).
- **Coverage of new surfaces:** this chunk adds **no** external-input surface, hot-path operation, or UI element — it adds *tests over* the existing `run_timeline` seam. The one relevant line:
  - `conductor-timeline determinism (run_timeline · Scenario→PhaseTimeline)` → validation: fixture loaded via `Scenario::from_toml_str` (garde✓, no silent parse) · instrumentation n/a (test code; `run_timeline` already `#[tracing::instrument]`) · PII n/a · tests: golden (insta, **assert-mode**) + property (proptest, seed-space) atop the existing unit/integ · a11y n/a · tokens n/a.

## Deviations from intent
1. **proptest asserts synchronously outside the async block** — the implementation retrieves the transition vec(s) from `rt.block_on(async { … })` then runs `prop_assert*` *after* it, rather than `?`-propagating `prop_assert` through the async block (the plan's step-6 sketch). Functionally identical; cleaner; sidesteps the async-Result-propagation pitfall the plan's own implementation notes flagged. In-scope.
2. **Snapshots authored via `INSTA_UPDATE=always cargo test`** — the plan's explicitly-listed fallback (cargo-insta is not installed). The committed `.snap`s assert green under the real gate (`cargo nextest run`, no env override).
3. **No `proptest-regressions/` file materialized** — every property held across all generated cases, so proptest persisted no counterexample (the plan anticipated the file only "if/when" one appears). The directory will appear on a first real failure.
- (Non-deviations, recorded to forestall detector misfires: **no coverage `--fail-under` gate** was added — CI coverage is measure-only and the threshold is Epoch 10, `ci.yml:56`; the golden captures **only** `Vec<PhaseTransition>`, never a `std::time` journal stamp.)

## Decisions & corrections
- **proptest search RNG intentionally not pinned** (default entropy + committed regressions). Sound because the properties are universally true (∀ seed/timeline) — the search cannot produce a flake; any counterexample is a genuine determinism break, persisted and replayed deterministically. The SUT's own seed is fully controlled.
- **Async-in-proptest pattern for this project:** the `#[tokio::test]` macro cannot wrap a `proptest!` block, so each property builds a `tokio::runtime::Builder::new_current_thread().enable_time().start_paused(true)` runtime inside the case and asserts synchronously after `block_on`.
- **Snapshot authoring without cargo-insta:** `INSTA_UPDATE=always cargo test -p conductor-timeline --test replay` to write the accepted `.snap`, then assert via `cargo nextest run` (CI/assert mode).
- Two goldens chosen over one: the fixture seed (424242) **and** a contrasting seed (7) — visibly distinct shapes (2028/3075 vs 2049/3060) freeze seed-sensitivity, not just one seed.

## Outcome
- **All acceptance criteria met:** committed insta golden pins the exact `Vec<PhaseTransition>` for the fixture scenario+seed through the full `from_toml_str → From → run_timeline` pipeline; a second golden freezes a contrasting seed; proptest asserts replay determinism + structural invariants (bounded gap, monotonic `elapsed_ms`, len/order) across the seed space; all under `start_paused`; zero nextest retries; captures only the seeded dimension; no OTel/`:4317`; no new dependency; no production change; no coverage gate added.
- **Gates green:** `cargo nextest run -p conductor-timeline` → 13/13 (assert mode). Dogfood `scripts/agent-run.ps1 run` → **EXIT 0**: `cargo nextest run --workspace --profile ci` 81/81 · `cargo test --workspace --doc` ok · `cargo clippy --workspace --all-targets -- -D warnings` clean.
- **Smoke:** ✓ — the plan's Test Commands list `agent-run`, so the smoke fired; the full-workspace dogfood gate ran clean. (No boot-path change — test-only — but the harness path itself was exercised end-to-end.)
- **Fix-loop:** green in 1 iteration (first compile passed). Nothing surfaced, nothing stuck; scope boundary held (`conductor-timeline/tests/` only).
