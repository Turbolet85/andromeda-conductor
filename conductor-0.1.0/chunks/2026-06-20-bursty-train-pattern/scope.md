# Scope — Bursty-train pattern (P-013)

**Marker:** `2026-06-20-bursty-train-pattern`
**Crate:** `conductor-faults`
**Epoch:** 4 (Fault helpers) — chunk 4/4 (completes the epoch)
**Pulse capability:** P-013 (activity-floor)

## Working-route intent
> Bursty-train pattern — active 5min / quiet 10min repeating (P-013 activity-floor)

## What it builds
A deterministic fault-helper *primitive* in `conductor-faults` that describes the **bursty-train**
activity pattern: a repeating duty cycle of an **active** emission window followed by a **quiet**
window (canonically 5 min active / 10 min quiet), looping indefinitely.

It is the P-013 activity-floor **false-positive guard**: a legitimately *periodic* ("bursty") service
whose long quiet windows must NOT be read as death. It is the healthy-but-intermittent third member of
the activity-floor lever family:

- `AbruptSilence` (P-014) — permanent stop, "the service died" (no resume, no duration).
- `EmissionGap` (P-015) — one finite gap then resume, "a restart" (>20s, exact length).
- **`BurstyTrain` (P-013, this chunk)** — a *repeating* active/quiet cycle, "a healthy bursty floor"
  (the recurrence is the point; the quiet window deliberately exceeds the ~30s naïve activity-floor
  death cue, so only a detector that recognises the *recurring train* avoids a false positive).

Like its siblings, this is a pure descriptor primitive: it OWNS un-jittered duty-cycle values (the
timeline applies seeded jitter when it later drives the pattern — Epoch 7) and exposes the durations +
a within-cycle phase query (active vs quiet at an offset) plus the `*_ms` accessors the
timeline/`PhaseSpec` boundary will consume.

## Surfaces / contracts touched
- **New module** `crates/conductor-faults/src/train.rs` — concept-named (matches `gap.rs` / `silence.rs`
  / `port_occupier.rs`), exporting the `BurstyTrain` type + any duty-cycle bound constants.
- **`lib.rs`** — `mod train;` + `pub use` re-export + the crate-doc "Shipped so far" line extended to
  4/4 (mirroring how `silence.rs` was added).
- **`error.rs` / `FaultError`** — IF the helper validates a bound (active/quiet > 0, sanity ceiling),
  extend the `#[non_exhaustive]` enum with the named variant(s); the constructor returns
  `Result<Self, FaultError>`, matching `EmissionGap::new`. (The exact constructor shape — validated
  `Result` vs. fixed canonical marker — is the one open scope question, resolved at plan time.)

## Invariants this chunk must honour
- **Verdict/error wall:** an out-of-range duty cycle is a typed `FaultError` value (`Result::Err`),
  never a panic.
- **Determinism:** no seed in the helper (the duty cycle is a fixed descriptor); owned values are exact
  and un-jittered, exactly as `EmissionGap` owns its exact gap. Same inputs ⇒ same `BurstyTrain`.
- **No new deps; `Cargo.lock` un-drifted.**
- Gates green: `-p conductor-faults` + workspace nextest, clippy `-D warnings`, doctest, llvm-cov.

## Boundaries — explicitly NOT in this chunk (route-sequenced elsewhere)
- **Timeline/scheduler wiring** — driving the duty cycle as phases under `conductor-timeline` (with
  seeded jitter): Epoch 7.
- **The activity-floor scenario(s)** — `train/lunch/silence` declarative scenarios (P-013..P-016, P-057):
  Epoch 7 ("Activity-floor + restart-suppression scenarios").
- **Obs `fault.*` span** — a `fault.{...}` tracing span for the pattern: Epoch 7/8 (route-sequenced;
  the obs span set is `fault.{silence,ramp,port_occupier}` today and grows with timeline wiring).
- **Scenario-config garde wiring** for the faults helpers (`PortOccupier`/`EmissionGap`/`AbruptSilence`/
  `BurstyTrain`): Epoch 7.
- **Actual OTLP emission** during active windows — that is the `conductor-emit` seam driven by the
  timeline, not this descriptor.

## Definition of done
Epoch 4 (Fault helpers) reaches 4/4. `conductor-faults` exposes `BurstyTrain` alongside `PortOccupier`,
`EmissionGap`, `AbruptSilence`, with unit + doctest coverage and all gates green.
