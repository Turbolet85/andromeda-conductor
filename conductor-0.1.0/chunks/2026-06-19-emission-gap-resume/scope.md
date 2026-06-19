# Scope — Emission gap/resume (P-015 restart detection)

**Marker:** `2026-06-19-emission-gap-resume`
**Version:** conductor-0.1.0 · **Epoch 4 (Fault helpers), 2/4**
**Crate:** `conductor-faults` · **Pulse P-ID:** P-015 (restart detection)
**Working entry:** _Emission gap/resume — exact gap lengths >20s with resume (P-015 restart detection)_

> _Refined during P4 planning (val-1 intent-incomplete): the determinism framing below was corrected from
> "under seed" to "by construction (exact)" — an exact gap is seed-independent by nature._

## What it builds
A fault helper in `conductor-faults` that produces a **deterministic, exact-length emission gap** (a silence
window) followed by a **resume** — the P-015 restart-detection lever. Pulse treats a service whose emission
stops for longer than its restart threshold (>20s) and then resumes as a *restart*, emitting a `RestartEvent`
within ≤2s (input.md:70, :107 — restart-suppression family uses a 25s gap). The helper encapsulates:

- An **exact gap duration** with a validated **>20s floor** (the restart-detection threshold). "Exact" is the
  defining property: the gap must deterministically clear 20s, so it is NOT subject to the timeline's
  seeded breathing/ramp jitter that could perturb a gap below threshold (phase.rs `jitter` bound).
- A **resume boundary** — the marker/structure that signals emission restart after the gap, distinguishing
  this from a permanent stop.
- **Determinism by construction (exact)** — the gap is a fixed value, reproducible across platforms/versions;
  the configured duration (not a seed) materially drives the output, so it is **seed-independent** like the
  exception-fingerprint primitive. The project determinism bar is satisfied trivially by exactness.
- **Validation + typed errors** — an invalid configuration (gap ≤ the 20s floor, or above a sanity ceiling)
  surfaces as a typed `FaultError` VALUE (`Result::Err` is harness-fault only), never a panic — the
  verdict/error wall. (Whether bounds also express as garde on a core descriptor is a HOW decision for the
  plan; planning chose a self-contained constructor-validated type, not a garde config struct.)

## Boundaries (what it is NOT)
- **Not the live emit/timeline wiring.** Like the port-occupier primitive (standalone `std::net`, not yet
  driven under the timeline — that wiring is Epochs 7/8), this chunk ships the gap/resume *structure/helper*.
  It does NOT itself emit OTLP (that is `conductor-emit`) and is NOT yet plugged into a running scenario
  timeline. Whether it adds a `conductor-timeline`/`conductor-core` dependency or stays self-contained is a
  HOW decision for the plan (planning chose self-contained, no new deps), but cross-seam scope-law still
  holds (faults depends only inward).
- **Not P-014 permanent silence** (abrupt stop, no resume — Epoch 4 chunk 3) and **not P-013 bursty train**
  (repeating active/quiet cycles — Epoch 4 chunk 4). This chunk is the single gap-then-resume pattern only.
- **Not verification.** Asserting Pulse's `RestartEvent ≤2s` read-back is the restart-suppression scenario
  in Epoch 5/7 (MCP read-back). This chunk only produces the fault input; it makes no MCP call and renders
  no verdict.
- **Not the suppression / bypass logic** (P-016/P-057 burst-within-window, bypass triple) — those ride the
  same restart-suppression scenario family later but are out of scope here.

## Surfaces / contracts touched
- **`conductor-faults` public API** — a new gap/resume helper type + `pub use` re-export in `lib.rs`
  (alongside `PortOccupier`); crate-doc line updated (the doc already names "silence" as an Epoch-4 helper).
- **`FaultError`** (`conductor-faults/src/error.rs`) — extended with the validation variant(s) for the
  gap/resume helper (typed seam error, `thiserror`; `#[non_exhaustive]`, additive).
- **Determinism contract** — exact, reproducible gap length; tested as exact-value preservation (the input
  materially drives the output) per the testing rule — NOT a seed `different-seeds-diverge` test.
- **Verdict/error wall** — invalid config ⇒ `FaultError` value, never panic.
- **Downstream consumer (later epochs, not built here):** the restart-suppression scenario family
  (P-015/P-016/P-057) and the timeline/emit gap execution; obs `fault.silence` span (obs rule names the
  bounded span set `fault.{silence,ramp,port_occupier}`) is deferred to the driven-under-timeline epoch.

## Acceptance intent (anchor for validation-1)
- A constructable gap/resume fault helper with an exact, deterministic gap duration and a >20s-floor guard.
- Invalid gap (≤ floor) / over-ceiling ⇒ typed `FaultError`, no panic.
- Deterministic by construction (reproducible exact gap; the configured duration materially drives output;
  seed-independent).
- Re-exported from `conductor-faults` `lib.rs`; gates green (faults seam tests, workspace nextest, clippy
  `-D warnings`, llvm-cov threshold, doctest).
