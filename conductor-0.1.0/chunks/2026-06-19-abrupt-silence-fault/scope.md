# Scope — Abrupt-silence fault (P-014 activity-floor permanent stop)

**Marker:** `2026-06-19-abrupt-silence-fault`
**Version:** conductor-0.1.0 · **Epoch 4 (Fault helpers), 3/4**
**Crate:** `conductor-faults` · **Pulse P-ID:** P-014 (activity-floor / abrupt permanent silence)
**Working entry:** _Abrupt-silence fault — permanent emission stop (P-014)_

## What it builds
A fault helper in `conductor-faults` that produces an **abrupt, permanent emission stop** — emission ceases
sharply and **never resumes**. This is the P-014 activity-floor "the service died" lever: a fresh service
that has been actively emitting (establishing an activity floor) goes abruptly silent, and Pulse raises an
activity-floor cue ~30s after the last emission (input.md:71 "abrupt permanent silence (P-014)"; :106
activity-floor row — _"fresh service: 5min active → abrupt stop → cue at 30s"_).

The defining property is **permanence / no resume** — this is the terminal counterpart to [`EmissionGap`]
(P-015). Where `EmissionGap` owns a finite, exact gap `Duration` bounded by the 20s restart floor and
*resumes* (Pulse reads a restart), abrupt-silence has **no resume boundary** and **no finite silence length
to own** — the silence is unbounded/terminal, so its invariant story differs from the gap's floor/ceiling.
The helper encapsulates:

- A **permanent-stop marker** — the structural signal that, from its onset, emission ceases and does not
  resume. The *absence* of a resume boundary is the contract that distinguishes it from `EmissionGap`.
- **Determinism by construction** — abrupt-silence carries no seeded randomness; it is
  deterministic / seed-independent (like the exact gap and the fingerprint primitive). Same scenario ⇒ same
  stop.
- **Verdict/error wall** — if construction can fail at all, an invalid configuration surfaces as a typed
  `FaultError` VALUE, never a panic. *Whether construction is fallible at all* is a genuine HOW question for
  the plan: unlike the gap, permanent silence has no 20s floor / ceiling to violate (it is unconditional), so
  the helper may be **infallible** — the first faults helper with no validation error variant — or carry a
  minimal guard. The plan decides; scope only requires that ANY failure path stays a typed value, never a
  panic.

## Boundaries (what it is NOT)
- **Not the live emit/timeline wiring.** Like `PortOccupier` and `EmissionGap` before it, this chunk ships the
  abrupt-silence *structure/helper* as a standalone primitive. It does NOT itself emit OTLP (that is
  `conductor-emit`) and is NOT yet driven under a running scenario timeline — that wiring, including the
  preceding "active" lead-in phase and the obs `fault.silence` span, is Epochs 7/8. Whether it stays
  self-contained or takes a dep is a HOW decision (both siblings stayed self-contained, no new deps).
- **Not the 30s cue / verification.** Asserting Pulse's activity-floor cue at ~30s is the **activity-floor
  scenario** (P-013/P-014) in Epoch 7 via MCP read-back. The 30s is *Pulse's* reaction latency, not a
  Conductor input bound. This chunk only produces the fault input; it makes no MCP call and renders no verdict.
- **Not the active lead-in.** The "5 min active" that establishes the floor before the stop is timeline-phase
  concern (a preceding emission phase), not this helper — consistent with the gap chunk keeping the timeline
  out of scope.
- **Not P-015 gap/resume** (finite silence that resumes — Epoch 4 chunk 2, shipped) and **not P-013 bursty
  train** (repeating active/quiet cycles — Epoch 4 chunk 4, next). This chunk is the single permanent-stop
  pattern only.

## Surfaces / contracts touched
- **`conductor-faults` public API** — a new abrupt-silence helper type + `pub use` re-export in `lib.rs`
  (alongside `PortOccupier`/`EmissionGap`); crate-doc line updated (the doc already names "silence" as an
  Epoch-4 helper, and `gap.rs` already forward-references "the permanent-silence fault (P-014)" — this chunk
  fulfills that reference).
- **`FaultError`** (`conductor-faults/src/error.rs`) — extended with a validation variant **only if**
  construction is fallible (HOW for the plan; `#[non_exhaustive]`, additive). If the helper is infallible,
  `FaultError` is untouched — itself a notable, captured contrast with the gap.
- **Determinism contract** — abrupt-silence is deterministic / seed-independent by construction (no seed
  param); tested as such — NOT a seed `different-seeds-diverge` test (there is no seed to drive divergence;
  same rationale as the gap, per testing-rule 2026-06-16/2026-06-18).
- **Verdict/error wall** — any construction failure ⇒ `FaultError` value, never panic.
- **Downstream consumer (later epochs, not built here):** the activity-floor scenario family (P-013/P-014)
  and the timeline/emit stop execution; the obs `fault.silence` span (obs rule names the bounded set
  `fault.{silence,ramp,port_occupier}`) is deferred to the driven-under-timeline epoch.

## Acceptance intent (anchor for validation-1)
- A constructable abrupt-silence fault helper that models a **permanent emission stop with no resume** — the
  terminal contrast to `EmissionGap`.
- Deterministic by construction (no seed; reproducible; seed-independent).
- The permanence / no-resume property is expressed in the type and exercised by a test that distinguishes it
  from a resuming gap.
- Any construction failure ⇒ typed `FaultError`, no panic — or, if the plan chooses an infallible constructor,
  no error path at all, with the contrast documented.
- Re-exported from `conductor-faults` `lib.rs`; gates green (faults seam tests, workspace nextest, clippy
  `-D warnings`, llvm-cov threshold, doctest).
