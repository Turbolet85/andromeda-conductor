# Scope — Error-baseline-spike + latency-regression scenarios (P-009..P-012)

**Marker:** `2026-06-22-error-baseline-spike-latency-regression-scenarios`
**Version:** conductor-0.1.0 · **Epoch 7 (Scenario catalog) — chunk 3 of 8**
**Working-route intent:** _Error-baseline-spike + latency-regression scenarios — baseline convergence, ramp, candidate persistence (P-009..P-012)_

## What it builds

The **statistical-anomaly family of the `scenarios/` catalog** — declarative TOML scenario config keyed to
Pulse's first four statistical-baseline capabilities **P-009..P-012** (`pulse-capability-spec.md` §3
"Statistical Anomaly Detection"). Unlike the hard signals (P-005..P-008, detectable from the first span),
these require **baseline accumulation**: the detector must converge a per-service / per-operation baseline
before a deviation can be flagged, so each scenario is a **baseline → deviation timeline** (converge, then
ramp the signal past the multiplier threshold and hold it past the persistence window):

- **P-009 — Per-Service Error Rate Baseline** — EWMA error-rate baseline per `service.name` (~5-min window,
  converges within 60s of bootstrap, persisted across restarts). **Baseline math → `Hard`.** Conductor recipe:
  inject sustained error rate at a known level; after the convergence window the reported baseline matches
  injection **within ±10%**. (Boundary: <10 spans/min excludes a service from spike detection — the "10-span
  error-rate floor".)
- **P-010 — Error Rate Spike Detection** — short-term (30s window) error rate exceeding baseline by **≥3.0×**
  with **≥30s persistence**; detection latency **<2s p99**. **Threshold detection → `Hard`.** Conductor recipe:
  establish baseline over 90s, ramp to **3.5×** for 60s, verify candidate emission within 2s of crossing the
  30s-persistence threshold.
- **P-011 — Per-Operation Latency Baseline** — streaming p50/p95/p99 per (`service.name` + operation) via
  t-digest, 5-min window. **Baseline math → `Hard`.** Conductor recipe: inject a known latency distribution;
  after convergence baseline p99 matches injection **within ±15%**. (Boundary: <50 spans/window excludes an
  operation from regression detection — the "50-sample latency floor".)
- **P-012 — Latency Regression Detection** — current p99 exceeding baseline p99 by **≥2.5×** with **≥60s
  persistence**; errors weighted independently from latency. **Threshold detection → `Hard`.** Conductor
  recipe: establish latency baseline over 90s, ramp to **3× p99** for 90s, verify candidate emission after the
  persistence threshold.

So **all four checks carry `class = "Hard"`** — these are deterministic baseline-math + threshold-detection
claims (the architecture's Probabilistic-Assertion Policy lists "baseline math" and "suppression/bypass logic"
as hard pass/fail; the **model-interpretive** severity that *consumes* these attention cues is P-020, a later
severity-lifecycle scenario, not asserted here). This is the clean all-`Hard` counterpart to ch2's single
P-008 `CalibrationRegion` exception.

The P-IDs **pair into two scenarios, not four** — detection (P-010 / P-012) is sequentially dependent on its
baseline (P-009 / P-011) on one shared timeline, so a single scenario per pair carries both P-IDs + a
baseline-assertion and a detection-assertion `[[expected]]` check. This naturally diverges from ch1/ch2's
one-file-per-P-ID granularity (an explicit P4 confirm) and matches the architecture's canonical
`error-baseline-spike` example (`p_ids = ["P-009","P-010"]`).

**`scenarios/error-baseline-spike.toml` already exists** (created in the scenario-config-model chunk as the
first schema-exercise fixture: `p_ids=["P-009","P-010"]`, baseline + spike phases) but carries **no
`[[expected]]` block** — so the P-009/P-010 leg **formalizes the existing fixture into a verifying catalog
entry** (add the baseline-match + spike-candidate expected checks; tune phases to the 90s-converge /
60s-at-3.5× recipe), and the P-011/P-012 leg adds a **new** `latency-regression.toml`.

This is **catalog wiring over primitives that already exist** (Epoch 3): latency-shaping (P-011/P-012 — seeded
p50/p95/p99 sampling over the ≥50-sample floor), error spans / error-rate (P-009/P-010), traffic-rate ramps
(the baseline → spike / regression ramp). **No new emit primitive.** The `Scenario.expected:
Vec<ExpectedCheck>` TOML surface was wired in ch1, so this chunk *consumes* it (target: zero further
`Scenario`/`ExpectedCheck` model change). **No live MCP run** happens here — that is the Epoch-8 CLI driver +
Epoch-10 E2E.

## Requirement source of truth

- **`.andromeda/refs/pulse-capability-spec.md` §3 (P-009..P-012, lines 166–204)** — THE normative source; each
  P-ID carries a "Conductor verification" clause with the concrete inject → converge → ramp → verify recipe +
  a boundary clause (the <10-span / <50-span floors).
- **`.andromeda/architecture.md` §Probabilistic-Assertion Policy + §Timing-Tolerance Model** — baseline math is
  `Hard`; the sample-count floors (50-sample latency / 10-span error-rate) route their *floor-edge* behavior to
  the calibration-region bucket, but the primary baseline-match (±10% / ±15%) and candidate-emission checks are
  the deterministic `Hard` claims. The run-report-envelope example names `error-baseline-spike` /
  `p_ids=["P-009","P-010"]`.
- **Existing precedent:** ch1/ch2 `scenarios/*.toml` — the catalog TOML shape (`name · p_ids · seed · slo_tier ·
  jitter_ms · [[phases]]{name,gap_ms}` + `[[expected]]{kind,class,expected}`) + the existing
  `error-baseline-spike.toml` fixture (the P-009/P-010 starting point).

## Boundaries

- **In:** the two statistical-anomaly scenario TOMLs under `scenarios/` (`error-baseline-spike.toml` formalized
  for P-009/P-010; new `latency-regression.toml` for P-011/P-012); their `[[phases]]` (baseline-converge →
  ramp / hold-past-persistence) + `[[expected]]` checks (all `class="Hard"`: baseline-match-within-tolerance +
  detection-candidate-present); per-scenario SLO tier; fixture round-trip tests proving each deserializes +
  garde-validates + builds a valid `PhaseTimeline` through the existing scheduler.
- **Out:** the live MCP read-back verification *run* (Epoch-8 CLI driver / Epoch-10 E2E); the **±10%/±15%
  tolerance math + persistence-window timing** as *evaluated* logic (the Epoch-8 `slo`/evaluator + driver own
  the runtime comparison; the TOML only declares intent); any **new emit primitive** (latency-shaping /
  error-spans / ramps all exist from Epoch 3); the restart-suppression interaction (P-016/P-057 — that is ch4's
  activity-floor + restart family); the model-interpretive severity that consumes these cues (P-020 —
  severity-lifecycle, a later chunk); the OTHER Epoch-7 families; the CLI/desktop surfaces (Epoch 8/9).
- **Scope law:** every scenario carries its P-ID pair; **no new inbound listener** — pure-egress scenarios (no
  port-occupier).

## Surfaces / contracts touched

- **Artifact:** `scenarios/error-baseline-spike.toml` (formalize: add `[[expected]]`, tune phases to the recipe)
  + new `scenarios/latency-regression.toml` — the primary deliverable.
- `conductor-core::Scenario` / `ExpectedCheck` (`scenario.rs`) — *consumed* via `from_toml_str`; extended only
  if a comparison kind / claim class / phase field is genuinely missing for the baseline-match-within-tolerance
  check.
- `conductor-core::{ClaimClass, ComparisonKind, SloTier}` — class selection (all `Hard`), per-check comparison
  kind (does an existing kind express "value within ±N% of expected"? — P3/P4 question), per-scenario tier.
- Emit primitives **referenced, not modified:** `conductor-emit` latency-shaping (P-011/P-012), error-spans /
  error-rate (P-009/P-010), traffic-rate ramps (the baseline → spike / regression ramp).
- Requirement contract: `pulse-capability-spec.md` §3 per-P-ID "Conductor verification" clauses.

## Open questions — resolve in planning (P4)

1. **File granularity** — 2 files (one per P-ID-pair, since detection is baseline-dependent on a shared
   timeline — the natural shape + the architecture's `error-baseline-spike` precedent) vs 4 files (one per
   P-ID, mirroring ch1/ch2). Recommend **2**; confirm via AskUserQuestion (this is the deviation from the
   ch1/ch2 convention).
2. **Tolerance-band comparison kind** — P-009/P-011 assert "baseline matches injection within ±10%/±15%". Does
   the existing `ComparisonKind` set express a *within-tolerance / approximate* match, or does the TOML only
   declare the expected baseline marker (a `Contains`/equality value) and defer the ±N% math to the Epoch-8
   evaluator? Prefer **declare-only** (no model change), consistent with ch2 deferring the SLO-timing math.
3. **Persistence-window phases** — P-010 needs ≥30s persistence, P-012 ≥60s. Does the `Phase` model express
   phase *duration* (to hold the ramp past persistence), or is the persistence realization an Epoch-8 runtime
   concern with the TOML only naming the baseline / ramp phases? (The existing `error-baseline-spike.toml`
   phases carry only `gap_ms` — confirm whether a duration / repeat field exists or is out of scope here.)
4. **SLO tiers** — P-010 detection <2s p99 → `<5s`; P-012 (≥60s persistence, candidate after the window) →
   `<5s` for the post-threshold emission latency or `<20s`? Confirm per the tier-scaling model.
5. **Zero-model-change confirmation** — confirm the ch1 `expected` / phase surface covers both recipes with
   **no** further `Scenario`/`ExpectedCheck`/`Phase` change (the goal), or whether a *minimal* addition (a
   tolerance kind or a phase-duration field) is unavoidable.

## Definition of done (chunk-level)

- `scenarios/error-baseline-spike.toml` (formalized, P-009/P-010) + `scenarios/latency-regression.toml` (new,
  P-011/P-012) exist under `scenarios/`, deserialize + garde-validate via `Scenario::from_toml_str`, and
  produce a valid `PhaseTimeline` through the existing scheduler — proven by fixture round-trip tests (mirrors
  ch1/ch2 + the existing `error-baseline-spike` fixture test).
- All P-009..P-012 expected checks are `class = "Hard"` (no `CalibrationRegion` here — the clean counterpart to
  ch2's P-008), with valid/invalid garde `#[case]` rows wherever the model is touched.
- **No new emit primitive and no new inbound bind**; all referenced primitives are the existing Epoch-3 ones.
- Determinism preserved: same scenario+seed ⇒ same stream shape (driven under `start_paused`).
- Gates: `cargo nextest` workspace green · clippy `-D warnings` clean · doctest 0 · no new dependency unless
  justified.
