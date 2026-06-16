# Scope — Scenario-config model

**Chunk:** `2026-06-16-scenario-config-model`
**Epoch:** 2 — Timeline engine (chunk 2 of 4)
**Working entry:** "Scenario-config model — declarative per-phase emission spec, serde + garde validated"

## What this builds
The declarative, config-layer representation of a scenario's **phase sequence** — the serde-deserialized + garde-validated data structure that describes, per phase, what the timeline should do (phase identity/kind, its timing contribution, and a declarative *emission spec* of what that phase emits) — plus the **conversion that wires a validated `Scenario` into the timeline scheduler's `PhaseTimeline`** (the `Phase`/`PhaseTimeline` types built in the previous chunk, `2026-06-16-seeded-phase-scheduler`).

This is the bridge between two things that already exist:
- the **conductor-core scenario model** + its garde `Validate` (built in `2026-06-15-conductor-core-shared-types` and `2026-06-15-config-validation-surface`), and
- the **conductor-timeline `PhaseTimeline`** the seeded scheduler consumes.

After this chunk, a scenario config under `scenarios/` deserializes into a validated model whose phase sequence can be handed to `run_timeline(...)`.

## In scope
- A declarative **per-phase emission spec** schema (serde structs): each phase carries its identity/kind, its timing contribution (the duration/gap the scheduler uses), and a *description* of the emission for that phase (data only — NOT the emitter).
- **garde validation** for the new per-phase fields, co-located with the serde structs: non-negative durations, sane ordering/bounds, and any cross-field invariants the phase list requires; extends the existing scenario-model validation and reuses the garde `Report`→`CoreError` bridge.
- The **`Scenario` → `PhaseTimeline` conversion** ("wires Scenario→phases into the scheduler"): a total, deterministic, order-preserving mapping from the validated config phases to the scheduler's ordered `PhaseTimeline`. The seed stays the scenario's; this chunk supplies the phase sequence the seed shapes.
- Reconciliation with the existing conductor-core scenario model (extend it; do not duplicate).
- At least one representative scenario fixture exercising the schema + conversion end-to-end (config → validate → `PhaseTimeline`).

## Out of scope (deferred, by epoch)
- Actual OTLP emission / wire building — Epoch 3 (`conductor-emit`); the emission spec here is declarative data, not bytes on the wire.
- The per-run JSONL emission-journal writer — Epoch 2 chunk 3.
- The determinism-replay harness (insta golden + proptest sweep) — Epoch 2 chunk 4 (dev-deps already wired; basic determinism asserts here are fine, the full golden/proptest sweep is not).
- Expected-outcome blocks + SLO tier model, verdict logic, MCP read-back — Epoch 5.
- Fault-helper semantics (ramps / silence / port-occupier) — Epoch 4; this chunk only models phases declaratively.

## Surfaces / contracts touched
- **conductor-core** — the scenario model + its garde `Validate` impls (likely owning crate of the scenario config schema; the per-phase spec extends it).
- **conductor-timeline** — `Phase` / `PhaseTimeline` (the conversion target; depends on core, never the reverse).
- **`scenarios/`** — declarative config files (serde + garde), one per Pulse P-ID; this chunk defines/extends their phase schema (a representative fixture).
- **Verdict/error wall** — config-parse + garde-validation failures surface as `ConfigError`/`CoreError` (harness `Result::Err`), never a verdict; the deterministic conversion never panics.
- **Determinism invariant** — same scenario + seed ⇒ same `PhaseTimeline` shape; the config→timeline mapping is order-preserving and total.

## Open questions for the plan (resolve in P3 research → P4 plan)
- Does the existing conductor-core scenario model already carry a phase/timeline notion, or is the per-phase spec net-new? Read the current model first — **extend, don't duplicate**.
- Crate ownership of the `Scenario → PhaseTimeline` conversion: conductor-timeline (which sees core) vs a `From`/`TryFrom` on core types. Respect the crate-per-seam dependency direction (timeline → core).
- How to shape the declarative "emission spec" so Epoch-3 `conductor-emit` can consume it without rework (forward-compatible, not built here).

## Definition of done (acceptance anchor)
A scenario config (serde) with a phase sequence loads, garde-validates (bounds + cross-field rules), and converts deterministically into a `PhaseTimeline` the existing seeded scheduler accepts — with invalid configs rejected as a typed `Err` (harness fault, not a verdict, never a panic) — all behind the existing gates green (`nextest --profile ci` / clippy `-D warnings` / `cargo audit` / `cargo deny`).
