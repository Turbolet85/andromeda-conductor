# Scope — Determinism-replay harness

**Marker:** 2026-06-17-determinism-replay-harness
**Version:** conductor-0.1.0 · Epoch 2 (Timeline engine) · chunk 4 of 4
**Working entry:** "Determinism-replay harness — same scenario+seed yields identical stream shape via insta golden + proptest, tokio start_paused"

## What it builds
The **determinism-replay test harness** that turns "same scenario + seed ⇒ identical emission-stream
shape" from a stated invariant (architecture §Design Philosophy) into an **enforced, regression-caught
property** over `conductor-timeline`. The seeded scheduler and the `Scenario → PhaseTimeline` bridge
already exist (chunks 1–2 of this epoch); this chunk **proves they are deterministic** and **freezes
their output shape against silent drift** — it is primarily a *test* chunk, not a new production seam.

The "stream shape" under test is the scheduler's observable output: the ordered
`Vec<PhaseTransition>` (`index`, `name`, virtual `elapsed_ms`) returned by `run_timeline`, driven on
tokio's **virtual** clock via `start_paused`. Concretely the harness adds:

- **insta golden snapshot(s)** — the missing *absolute* freeze. Existing determinism tests assert only
  *relative* consistency (`first == second`, within-bound, monotonic); none pins the concrete
  `elapsed_ms` values, so a change to the RNG algorithm, the jitter formula, the gap mapping, or the
  conversion would pass them all. A committed golden of the exact transition sequence — taken through
  the **full pipeline** from the committed `scenarios/error-baseline-spike.toml` fixture
  (`Scenario::from_toml_str` → `PhaseTimeline::from(&Scenario)` → `run_timeline`) at a fixed seed —
  catches that drift, and is the on-disk guardrail for the **ChaCha8 cross-platform/cross-version
  stability** promise (the explicit reason ChaCha8Rng was chosen over StdRng). insta runs in
  **assert/CI mode (fail, never auto-write)** per the testing rule.
- **proptest property test(s)** — generalize reproducibility from the handful of hand-picked seeds to
  the seed space (and, where it adds signal, to arbitrary *valid* timelines/scenarios): the **replay**
  property (`∀ seed: run_timeline(tl, seed)` twice ⇒ identical) plus the universally-true structural
  invariants (each gap ∈ `[base−jitter, base+jitter]` clamped ≥ 0; `elapsed_ms` monotonic
  non-decreasing). `proptest-regressions/` is committed so a discovered counterexample is replayed.
- **`start_paused` virtual-clock discipline** — all of the above run under
  `#[tokio::test(flavor = "current_thread", start_paused = true)]`, asserting *scheduled shape*, never
  real wall-clock duration (zero real-time waits, zero flakiness, no nextest retries).

## Boundaries (what it does NOT build)
- **NOT** new scheduler/conversion production logic — `run_timeline`, `Phase`/`PhaseTimeline`/
  `PhaseTransition`, and `From<&Scenario>` already exist and are the system *under test*. No production
  type changes are expected; the golden snapshots via `PhaseTransition: Debug` (`assert_debug_snapshot!`),
  so no `Serialize` derive is required. (Were a YAML/JSON snapshot judged clearer, a minimal
  `#[derive(Serialize)]` is the *only* production delta — resolve in planning; default is Debug.)
- **NOT** a golden over the **JSONL emission journal** or its `journal_emitted_at` stamps. Those come
  from `std::time` (real clock) and are deliberately non-reproducible; snapshotting them would flake.
  The golden captures **only** the seeded/virtual dimension (`Vec<PhaseTransition>`) — keeping the
  wall-clock-vs-virtual-clock wall intact is itself part of what the harness guards ("a golden test
  catches a paused-clock leak").
- **NOT** OTLP emission to `:4317` (Epoch 3) — there is still nothing to emit; the stream shape is the
  scheduler's transition sequence, not gRPC bytes.
- **NOT** a verdict/`ReportState` or `runs.db`/Markdown report (Epochs 5–6) — this chunk produces a
  passing/failing *test*, not a run-report envelope.
- **NOT** a cross-platform CI matrix that re-proves ChaCha8 stability on other OSes — reproducibility is
  asserted on the dev host; the committed golden is the cross-version tripwire, not a multi-OS job.
- **NOT** new dependencies — `insta` and `proptest` are already `conductor-timeline` dev-deps.

## Surfaces / contracts it touches
- **Crate seam:** entirely within **`conductor-timeline`** (its `tests/` integration target + any
  `#[cfg(test)]` proptest strategies) plus a read-only dependency on `conductor-core`
  (`Scenario`/`from_toml_str`) and the `From<&Scenario> for PhaseTimeline` bridge. No new cross-seam
  edge; nothing in `conductor-report`/`conductor-emit`.
- **Public API under test:** `run_timeline`, `Phase`, `PhaseTimeline`, `PhaseTransition`,
  `TimelineError` (test the public seam + observable output, never private fns).
- **Fixture:** the committed `scenarios/error-baseline-spike.toml` (already loaded by a core test) is the
  canonical scenario input for the end-to-end golden; the seed is pinned in-test.
- **Test infra contract (test-plan §4):** insta snapshots committed under
  `crates/conductor-timeline/tests/snapshots/`; `proptest-regressions/` committed; insta in assert mode
  (fail, don't write); driven by `start_paused` + (if needed) `tokio::time::advance`.
- **Determinism invariant (the quality bar):** same scenario+seed ⇒ same shape; assert scheduled
  ordering/shape, never real duration; seed *all* randomness (timeline seed + proptest strategies);
  zero nextest `retries`. Assert **both** directions where a unit test is the right tool (same seed ⇒
  identical *and* different seeds ⇒ divergent) — though the universal "different seeds diverge" claim is
  probabilistic and stays a fixed-seed test, **not** a proptest `∀` (a short timeline can coincide).
- **Verdict/error wall:** an empty timeline remains `TimelineError::EmptyTimeline` (a harness fault),
  never a verdict — unchanged, and not re-litigated here beyond the existing coverage.

## Intent anchor (validation-1)
A plan satisfies this scope when it adds, **within `conductor-timeline` and without new production
logic or dependencies**, (1) at least one **committed insta golden** that pins the *absolute*
`Vec<PhaseTransition>` stream shape for a fixed scenario+seed taken through the **full
`Scenario`-fixture → `PhaseTimeline` → `run_timeline`** pipeline (assert mode, fail-don't-write), and
(2) **proptest** property coverage generalizing **replay determinism** (same seed ⇒ identical shape)
and the structural invariants (per-gap bound, monotonic `elapsed_ms`) across the seed space with
**committed regressions** — all driven under `start_paused`, capturing **only** the seeded/virtual
dimension (never the `std::time` journal stamps), with zero retries. It explicitly **defers** OTLP
emission, verdict/report artifacts, and any multi-OS re-proof to their later chunks/epochs.
