# Scope — Constellation + context-grounding scenarios

**Chunk:** `2026-06-22-constellation-context-grounding-scenarios`
**Epoch:** 7 (Scenario catalog) · chunk 7/8
**Working-route intent:** _"Constellation + context-grounding scenarios — service dot/hue/stability, git commits/recurrence, P-032 known-residual (P-025..P-027, P-036)"_
**P-IDs:** P-025, P-026, P-027 (constellation) · P-032, P-036 (context-grounding)

## What it builds
Declarative `scenarios/*.toml` catalog entries (the established Epoch-7 pattern) for the constellation +
context-grounding capability family, plus the `conductor-core` scenario test wiring that loads and guards them.
No new emission primitives, no runtime engine code.

Per-P-ID:
- **P-025 Halo Hue Encoding** (`DriveObserve` → operator-checklist/ManualCheck) — drive an error-state-shaped
  stream so Pulse shifts a service's halo hue; hue is a visual claim verified by operator checklist (the ≤2s
  pipeline-latency-to-hue-shift is matrix-delegated to Conductor's emission; the halo render is Pulse's).
- **P-026 Halo Breathing Encoding** (`DriveObserve` → ManualCheck) — drive the rate-ramp/breathing emission
  curve (the `conductor-emit` `rate.rs` primitive already exists) so Pulse's halo "breathes" with throughput;
  visual/operator-checklist.
- **P-027 Service Constellation Auto-Discovery** (`DriveObserve` → ManualCheck) — drive a multi-service
  `service.name` topology (the `conductor-emit` `topology.rs` primitive already exists) so a new service
  appears as a constellation dot (≤5s, matrix-delegated), with stable positions across a Pulse restart and a
  >20-services cap; visual/operator-checklist.
- **P-032 Project Context Grounding** (`Auto` → **KnownResidual**) — emit an incident inside a real git
  workspace with known recent commits; the Report's context section should surface the last-5 commits. This is
  the catalog's FIRST known-residual: Pulse's production `recent_commits` producer is a stub until v0.3.0 and
  Conductor is its designated detector — the deviation is pre-accepted, so it reports KnownResidual, never a
  surprise Fail.
- **P-036 Cross-Incident Pattern Reference** (`Auto`) — recur a fingerprint across two sessions so the second
  incident's read-back shows "Previously seen" populated; backed by the cross-run `runs.db` fingerprint index.

## Boundaries
- **Catalog + test wiring only** — new TOMLs under `scenarios/` + `conductor-core` scenario-model test
  additions; no new `ComparisonKind` / emission primitive / engine path **unless** research proves the
  report-state expression requires a minimal model addition.
- **First operator-checklist (ManualCheck) family + first KnownResidual scenario** — this chunk is the
  catalog's inflection for the two remaining report states: ManualCheck via the **visual/operator-checklist
  path** (distinct from severity-lifecycle's `CalibrationRegion → ManualCheck` *auto* path) and KnownResidual
  via P-032's pre-accepted deviation. Whether existing scenario-model fields can express "operator-checklist
  claim" and "known-residual expected state", or a minimal model addition is needed, is the chunk's central
  design question — resolve in the plan, mirroring severity-lifecycle's "zero model change unless proven
  necessary" posture.
- **Emission primitives reused, not built** — P-026 (`rate.rs` breathing), P-027 (`topology.rs` multi-service
  constellation), P-025 (error-state hue shaping) all already exist from Epoch 3.
- **P-032/P-036 environment** — declared to run inside a real git workspace with known commits (an
  operator/Epoch-10 live-proof precondition; the TOML declares the intent, the live wiring is downstream).
- **Scope law** — every scenario carries its P-ID; Conductor opens no new listener.
- **Determinism** — any seeds chosen must not collide with or perturb existing golden-fed seeds
  (severity-lifecycle used 4317019..023); goldens are expected UNCHANGED.

## Surfaces / contracts touched
- `scenarios/*.toml` — new catalog entries (one or more per P-ID family).
- `conductor-core` scenario model (`scenario.rs` / `expected.rs`) — at minimum test wiring (loader rstest +
  per-scenario class/state guards); possibly a minimal expected-state expression for ManualCheck/KnownResidual
  (TBD in plan).
- ReportState `ManualCheck` + `KnownResidual` (`lamp.rs`, already present) — first catalog exercise.
- `runs.db` cross-run fingerprint index (P-036 "Previously seen") — declared intent; the storage seam exists.
- MCP read-back contract — P-032 `retrieve_report` context section; P-036 `query_incident_list` / fingerprint
  recurrence (declared; live read-back is Epoch-10).
- Coverage-matrix (`coverage.rs`) already classifies all five P-IDs (DriveObserve×3, Auto×2) — no change
  expected.

## Definition of done (acceptance intent)
- A scenario TOML exists for each of P-025, P-026, P-027, P-032, P-036 — each carrying its P-ID, expected
  outcome, and SLO tier; loadable + garde-valid.
- The constellation trio (P-025/026/027) expresses the operator-checklist/ManualCheck (DriveObserve) path
  via empty `expected`; P-032 **declares + documents** its KnownResidual (empty `expected`; the
  Fail→KnownResidual routing is Epoch-8 evaluator-owned — refined from "expresses" per P4 research: the
  scenario model has no `state` field, state is producer-assigned); P-036 expresses the Auto recurrence check.
- scenario test wiring loads + guards the new family (per-file class/state purity + the suite-level guard
  updated for the new report states).
- Gates green (conductor-core + workspace nextest, clippy `-D`, doctest); goldens unchanged; `agent-run.sh run`
  exit 0.
