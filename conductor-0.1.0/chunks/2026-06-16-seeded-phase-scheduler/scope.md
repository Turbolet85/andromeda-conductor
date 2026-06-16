# Scope — Seeded phase scheduler

**Marker:** `2026-06-16-seeded-phase-scheduler`
**Version:** conductor-0.1.0
**Epoch:** 2 — Timeline engine (1st chunk; opens Epoch 2)
**Working entry:** Seeded phase scheduler — current_thread tokio::time deterministic phase sequencing

## What it builds
The deterministic core of the timeline engine in `conductor-timeline`: a seeded phase scheduler that
sequences an ordered set of timeline phases on a `current_thread` tokio runtime using `tokio::time`, so the
same scenario + seed always yields the same emission-stream *shape* (ordering + relative timing). This is the
determinism substrate every later emission/fault chunk drives.

1. **Seeded determinism** — a seedable PRNG (seed sourced from the scenario / `CONDUCTOR_SEED`) owns every
   non-deterministic choice the scheduler makes. In THIS chunk that is **bounded jitter on inter-phase gaps**
   (user decision, phase P4): the seed materially shapes stream timing, so a fixed seed reproduces a fixed
   shape and a different seed yields a different (still-reproducible) shape. No ambient entropy, no
   system-clock-derived scheduling decisions.
2. **Phase sequencing on `tokio::time`** — phases run in declared order; inter-phase gaps / silence / hold
   durations are realized with `tokio::time::sleep` (the virtual clock), giving exact gap/ramp/silence timing.
   The scheduler advances phase-to-phase and surfaces phase-boundary transitions to its caller (a hook/event
   the emission seam will later consume) — it *sequences*, it does not *emit*.
3. **Runtime-agnostic library** — the scheduler is a `conductor-timeline` library API callable identically by
   the headless CLI path and the Tauri shell. The `current_thread` flavor is the contract (zero work-stealing
   ⇒ deterministic ordering): the CLI owns `#[tokio::main(flavor="current_thread")]`; the core can also drive a
   hand-built `Builder::new_current_thread()` runtime under Tauri.
4. **A minimal phase-timing representation** — just enough structure (an ordered phase list with per-phase
   timing) for the scheduler to operate on; the *rich declarative per-phase emission spec* is the next chunk.

## Boundaries (what this chunk does NOT do)
- **NOT the scenario-config model** — the declarative per-phase emission spec (serde + garde, *what* each phase
  emits) is the **next Epoch-2 chunk**. This chunk needs only the timing/sequencing skeleton; it may reuse
  conductor-core's existing scenario/seed scaffolding but does not build the full per-phase emission spec.
- **NOT emission** — no OTLP, no opentelemetry-proto, no tonic, no `:4317`. Phase boundaries are surfaced to a
  caller; actual span/metric/log emission is **Epoch 3** (`conductor-emit`).
- **NOT the emission journal** — per-run JSONL + `std::time` wall-clock stamps are the **3rd Epoch-2 chunk**.
  This chunk writes no journal and must NOT take wall-clock from `std::time` for *sequencing* (scheduling is
  tokio-virtual-clock only); the `std::time`-for-journal vs tokio-virtual-clock-for-scheduling split is honored
  by staying on `tokio::time`.
- **NOT the determinism-replay/golden harness** — the insta-golden + proptest replay proof is the **4th
  Epoch-2 chunk**. Unit tests here use `tokio start_paused` to assert deterministic timing, but the cross-seed
  golden harness is later.
- **NOT fault timing patterns** — ramps / abrupt silence / bursty-train / port-occupier are **Epoch 4**
  (`conductor-faults`); this chunk provides the generic sequencing those will compose, not the fault patterns.

## Surfaces / contracts touched
- **Built:** `conductor-timeline` — the seeded phase scheduler API + its seedable-PRNG wiring + the minimal
  phase-timing type(s) it sequences (the crate is a scaffold skeleton today).
- **Consumed (read-only / dependency edge):** `conductor-core` (shared scenario/seed types per the
  crate-per-seam edges); tokio (`current_thread` + `time` features); a seedable RNG dependency.
- **Invariants honored:** determinism under a seed (same scenario+seed ⇒ same stream shape); `current_thread`
  runtime, zero work-stealing; `tokio::time` (virtual clock) for ALL scheduling, never `std::time`; seeded RNG
  owns all non-determinism; `start_paused` only in tests; crate-per-seam edges (no forbidden cross-seam dep).

## Definition of done (intent anchor for validation-1)
- `conductor-timeline` exposes a deterministic, seeded phase scheduler that sequences an ordered phase list on
  a `current_thread` tokio runtime via `tokio::time`, surfacing phase-boundary transitions to its caller
  without emitting anything.
- A seedable PRNG is wired such that a fixed seed reproduces a fixed phase sequence/shape (ordering + relative
  timing).
- Unit tests under `tokio start_paused` prove deterministic sequencing/timing: a fixed seed reproduces an
  identical advance pattern, each gap lands at its declared base ± the seed-derived **bounded jitter** (never
  negative, within the declared bound), and a different seed yields a different but still-reproducible shape.
- The scheduler is runtime-agnostic (no CLI/Tauri coupling) and adds no OTLP/journal/scenario-spec surface
  (those are later chunks); crate-per-seam edges stay clean (clippy `-D warnings`, nextest `ci` green).
