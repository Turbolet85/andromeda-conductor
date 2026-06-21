# Scope — Verdict + assertion-policy split

**Marker:** 2026-06-21-verdict-assertion-policy-split
**Epoch:** 5 (Verification & read-back) — chunk 4 of 6
**Primary crate:** `conductor-verify` (verdict logic seam)
**Working-route intent:** _"Verdict + assertion-policy split — hard Pass/Fail vs CalibrationRegion classification"_

## What it builds

The classification logic that turns a verification check's outcome into a typed
`Verdict` (`Pass` / `Fail` / `CalibrationRegion` — already defined in `conductor-core`),
implementing the architecture's **two-state probabilistic-assertion policy**:

- **Hard (deterministic) assertions** → `Verdict::Pass` / `Verdict::Fail` on an exact /
  deterministic comparison. Covers the deterministic claim classes: hard signals,
  baseline math, suppression/bypass logic, lifecycle timing.
- **Calibration-region (model-interpretive) assertions** → `Verdict::CalibrationRegion` —
  recorded + reported-for-human, and **never hard-failed on exact values**. Covers the
  model-interpretive claim classes: severity choice, hypothesis quality, the P-008
  root-vs-deep weighting.

Concretely, this chunk delivers:
1. A way to **declare an assertion's policy class** (hard vs calibration-region) — the
   split is a property of the claim, decided up front, not guessed at runtime.
2. The **hard-path evaluator** — deterministic compare of observed vs expected → `Pass`/`Fail`.
3. The **calibration-region path** — capture the observed-vs-expected delta (the
   human-facing signal) and classify as `CalibrationRegion`, never emitting `Fail` on an
   exact-value mismatch of a model-interpretive claim.
4. The verdict-classification module(s) landing in `conductor-verify`, consuming
   `conductor-core::Verdict` (no new cross-seam edge beyond the existing `conductor-core` dep).

## Boundaries (explicitly NOT in this chunk)

- **NOT** the SLO timing / tolerance model — tier-scaled bands `<5s`/`<20s`/`<90s`,
  journal-relative latency math, the 50-sample / 10-span sample-count floors → that is the
  NEXT chunk ("Expected-outcome + SLO timing model").
- **NOT** the per-scenario `expected`-outcome config blocks themselves (next chunk). This
  chunk owns the *classification mechanism*; the next wires in concrete per-scenario values.
- **NOT** the `ReportState` mapping (`Pass`/`Fail`/`ManualCheck`/`KnownResidual`/`Blocked`)
  or run-report envelope serialization → Epoch 6. `Blocked` is already produced by the
  preflight gate; `ManualCheck`/`KnownResidual` are report-state concerns downstream.
- **NOT** MCP read-back transport (done: `mcp-read-back-client`) or preflight (done:
  `preflight-readiness-gate`).
- **NOT** any CLI/desktop rendering of verdicts (Epoch 8/9).

## Surfaces / contracts touched

- **`conductor-verify`** — new verdict-classification module(s); the verdict-logic seam
  named in architecture's module map.
- **Consumes** `conductor-core::Verdict { Pass, Fail, CalibrationRegion }` (existing type).
- **Verdict/error wall:** classification returns `Ok(Verdict)`; `Result::Err` is reserved
  for harness faults only. A model-interpretive assertion never hard-fails — it routes to
  `CalibrationRegion` (report-for-human), never `Fail`.
- **Determinism:** the hard-path comparison is deterministic (same inputs ⇒ same `Verdict`);
  no wall-clock read, no RNG inside the classification itself.
- No new external/network surface, no new occupied resource/port, no new env var.

## Why now

Epoch 5 already has the read-back client + preflight gate + egress liveness in place. The
next thing the suite needs is the *decision* layer: given a read-back observation, what
verdict does the check earn — and is this even a claim that may be hard-failed? This chunk
is the policy split that both the SLO-timing chunk and the run-report envelope build on.
