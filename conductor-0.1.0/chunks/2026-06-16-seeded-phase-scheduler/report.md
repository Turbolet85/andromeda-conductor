# Report — 2026-06-16-seeded-phase-scheduler

**Chunk:** Seeded phase scheduler — deterministic current_thread tokio::time phase sequencing on a seeded RNG (conductor-timeline)
**Date:** 2026-06-16T19:34:21Z
**Commits:** this chunk — none yet (lands in P7 as `feat(2026-06-16-seeded-phase-scheduler)`). Since last_wrap: `df736a0` base-ci-agent-run-harness-skeleton + `45af7d8` chore(toolchain) rust-analyzer (prior chunk + incidental edit, committed earlier this session).

## Changes (structured — detectors read this)
- **Files:**
  - New: `crates/conductor-timeline/src/phase.rs`, `crates/conductor-timeline/src/scheduler.rs`, `crates/conductor-timeline/tests/determinism.rs`
  - Modified: `crates/conductor-timeline/src/lib.rs` (placeholder → module decls + re-exports), `crates/conductor-timeline/Cargo.toml`, `Cargo.toml` (workspace), `Cargo.lock`
- **Symbols / APIs:** new public surface of `conductor-timeline` (was an empty `pub mod` placeholder):
  - `pub struct Phase { name: String, gap: Duration }` + `Phase::new`
  - `pub struct PhaseTimeline { phases: Vec<Phase>, jitter: Duration }` + `PhaseTimeline::new`
  - `pub struct PhaseTransition { index: usize, name: String, elapsed_ms: u128 }`
  - `pub async fn run_timeline(&PhaseTimeline, seed: u64) -> Result<Vec<PhaseTransition>, TimelineError>`
  - `pub enum TimelineError { EmptyTimeline }` (`thiserror::Error`, `#[non_exhaustive]`)
  - New tracing span name: `timeline.execute` (internal span, attribute `phase_count`; records `seed`).
  - No IPC methods, HTTP endpoints, event topics, ports/sockets, or env vars added.
- **Crates / modules:** no new crates (all 8 workspace members pre-exist). `conductor-timeline` gains modules `phase` + `scheduler` and becomes its first real (non-placeholder) surface. It is now the first consumer of the workspace-declared `tokio`.
- **Dependencies:**
  - Workspace `[workspace.dependencies]` += `rand_chacha = "0.9"`, `rand_core = "0.9"` (both already present in `Cargo.lock` transitively via `proptest`; resolved versions unchanged — rand_chacha 0.9.0 / rand_core 0.9.5).
  - `conductor-timeline` deps += `tokio` (feature `time`), `tracing`, `thiserror`, `rand_chacha`, `rand_core` (and pre-existing `conductor-core`). dev-deps += `tokio` (`test-util`,`macros`,`rt`,`time`), `rstest`, `proptest`, `insta`.
  - No version bumps. tokio resolved to 1.52.3 (workspace floor `"1.48"`).
- **Schema / config:** none — no `runs.db` schema, no scenario-config keys, no violation schema this chunk.
- **Coverage of new surfaces:**
  - `conductor-timeline::run_timeline` (the `timeline.execute` op) → validation **n/a** (no external-input boundary — takes a typed `PhaseTimeline` + `u64`; scenario-config deserialization / CONDUCTOR_* paths / MCP stdout are untouched) · instrumentation **span✓** (`timeline.execute`, attr `phase_count`; `emission_count` deferred — see Deviations) · PII **n/a** (span records only `seed`/`phase_count`; no host paths or struct names) · tests **unit✓** (6 tests, `#[tokio::test(flavor="current_thread", start_paused=true)]`) · a11y **n/a** (headless, no UI) · tokens **n/a** (no UI)

## Deviations from intent
- **insta golden snapshot not written** (plan marked it "(optional)"). A fresh `.snap` fails the headless nextest gate as *pending* until a manual `cargo insta accept` — which would break the automated gate. Reproducibility is proven by `same_seed_reproduces_the_sequence` (`assert_eq` on the full transition vec) + bound/clamp/zero-jitter tests. `insta` is wired as a dev-dep for the Epoch-2 replay-harness chunk that owns golden snapshots.
- **No proptest test this chunk.** A proptest harness is sync and fights `start_paused`; the bounded-jitter property is covered by explicit example tests. proptest wired as a dev-dep (convention + pre-staging for replay-harness chunk #4).
- **`TimelineError` has one variant (`EmptyTimeline`).** Plan sanctioned "empty/degenerate guard if warranted." An empty timeline is the only genuine harness-fault (nothing to sequence) → `Err` per the verdict/error wall, never a panic; all other ops are infallible (clamped arithmetic).
- **`conductor-timeline` declares `conductor-core` but does not reference it in code this chunk.** It is the architectural seam edge (dep-tree shows the edge); the scheduler takes `seed: u64` directly to stay decoupled until the next chunk wires `Scenario`→phases. No unused-dep lint on stable rustc/clippy.
- **`emission_count` span attribute deferred.** obs-plan §4 lists `phase_count` + `emission_count` on `timeline.execute`; emission does not exist until the `conductor-emit` chunk (Epoch 3), so only `phase_count` is set now (build-sequencing — the emit chunk adds `emission_count`).

## Decisions & corrections
- **Seeded gap jitter (user decision, phase P4).** The scheduler draws bounded per-gap jitter from the seeded `ChaCha8Rng` so the seed *materially* shapes stream timing (not merely held for later) — chosen over "wire + hold only" / "defer seeding." `scope.md` DoD was reconciled to match (intent-incompleteness, recorded at phase P5).
- **Determinism-test principle (user review, this session).** A seeded/determinism substrate must explicitly test that *different* seeds *diverge* (different seed ⇒ different shape), not only that the same seed reproduces — reproducibility alone would also pass if the seed were computed-but-never-applied. Verified `different_seeds_yield_different_shapes` (`assert_ne!` on the transition vec, 4 jitter draws, fixed seeds 1 vs 2) already covers this direction; no new test needed.
- **RNG = `rand_chacha::ChaCha8Rng` via `seed_from_u64`** (default I locked at P4): algorithm-stable across platforms/versions → reproducible, unlike `StdRng`.
- **`elapsed_ms` is a deterministic cumulative sum of effective gaps** (virtual ms), never a clock read — keeps the path pure and avoids any std::time-vs-virtual-clock confusion (`std::time::Duration` is used only as a value passed to `tokio::time::sleep`, not a clock read).

## Outcome
- **Acceptance criteria:** met. Determinism (same seed ⇒ identical sequence; different seed ⇒ different shape), bounded/clamped seeded jitter, runtime-agnostic `async fn`, `tokio::time`-only scheduling, typed `TimelineError` (no panic/unsafe), `timeline.execute` span, crate-per-seam edge intact.
- **Gates green (commands run):** `cargo nextest run -p conductor-timeline --profile ci` (6/6) · `cargo test -p conductor-timeline --doc` (ok, 0 doctests) · `cargo clippy -p conductor-timeline --all-targets -- -D warnings` (clean) · `cargo audit` (clean, 145 deps) · `cargo deny check` (advisories/bans/licenses/sources ok) · `cargo llvm-cov nextest -p conductor-timeline --fail-under-lines 60` (**100%** lines on phase.rs + scheduler.rs).
- **Smoke:** `bash scripts/agent-run.sh run` (workspace test+lint gate) green — boot-path unchanged but agent-run.sh is in the plan's Test Commands, so P3 fired and passed.
