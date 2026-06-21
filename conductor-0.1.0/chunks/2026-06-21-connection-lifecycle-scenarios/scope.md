# Scope — Connection-lifecycle scenarios (P-001..P-004)

**Marker:** `2026-06-21-connection-lifecycle-scenarios`
**Version:** conductor-0.1.0 · **Epoch 7 (Scenario catalog) — chunk 1 of 8**
**Working-route intent:** _Connection-lifecycle scenarios — Listening/Receiving/Idle/Stalled states with orthogonal port-occupier (P-001..P-004)_

## What it builds

The **first real entries of the `scenarios/` catalog** — declarative TOML scenario config keyed to Pulse's
connection-lifecycle capabilities **P-001..P-004**, driving the full ingest-health state walk:

- **Listening** — no-emit baseline (Pulse up, no telemetry arriving).
- **Receiving** — telemetry arriving, detection ≤1s.
- **Idle → Stalled** — emission stops; Idle window 10–60s degrading to Stalled >60s.
- **ReceiverFailed** — the **orthogonal port-occupier** holds loopback `:4317` so Pulse's receiver cannot
  bind, ReceiverFailed ≤2s (reuses the Epoch-4 `conductor-faults` port-occupier — the sole deliberate inbound bind).
- **Orthogonality** — connection-state is independent of error-state: an error-burst does not change the
  connection verdict, and a clean stop is not an error (the P-004 guard, per input.md §coverage line 102).

Because P-001..P-004 are classified **drive+observe** (handoff + coverage matrix), the **auto-measurable** legs
(the timing windows: Receiving ≤1s, ReceiverFailed ≤2s, Idle/Stalled boundaries) are expressed as declarative
`expected` blocks; the **visual/UX** half (the lifecycle badge / hue) routes to the operator checklist. No
live MCP run happens in this chunk — that is the Epoch-8 CLI driver + Epoch-10 E2E.

**This is also where carried follow-up (d) lands:** the `Scenario`/`PhaseSpec` model today
(`conductor-core/src/scenario.rs:68`) carries only `name · p_ids · seed · slo_tier · phases · jitter_ms`.
`ExpectedOutcome` (`expected.rs`) and the operator-pause holds (`pause.rs`) exist as core types but are **not
TOML-wired**. Authoring these scenarios requires wiring an `expected` block (and, if a lifecycle scenario needs
a go/no-go, `holds`) into the serde+garde TOML surface so a scenario can declare its expected read-back outcome
+ SLO. The model-extension depth is the central planning question (below).

## Requirement source of truth

- **`.andromeda/refs/pulse-capability-spec.md`** — THE normative requirement source: every P-001..P-060 carries
  a "Conductor verification" clause with concrete scenario parameters (gap lengths, SLO windows, expected
  read-back). The exact P-ID ↔ state ↔ SLO-tier mapping + the precise Idle/Stalled/ReceiverFailed thresholds
  are pinned there — read it in codebase-research (P3) and planning (P4).
- **`.andromeda/input.md` §Coverage line 102** — the connection-lifecycle behavior summary
  (`no-emit (Listening) → emit (Receiving ≤1s) → stop (Idle 10–60s → Stalled >60s); port-occupier on :4317 →
  ReceiverFailed ≤2s; error-burst + clean stop (orthogonality)`).
- **Existing precedent:** `scenarios/error-baseline-spike.toml` (P-009/P-010) — the current TOML shape
  (`name · p_ids · seed · slo_tier · jitter_ms · [[phases]] {name, gap_ms}`); the new scenarios extend it.

## Boundaries

- **In:** the connection-lifecycle scenario TOML file(s) under `scenarios/`; the `Scenario`/`PhaseSpec`
  serde+garde extension to carry the `expected` outcome block (+ SLO) — and `holds` iff a lifecycle scenario
  needs an operator go/no-go; garde validation for the new fields; reuse of the `conductor-faults` port-occupier
  for the ReceiverFailed leg; the orthogonality expression (connection-state ⟂ error-state); per-scenario SLO
  tiers chosen to fit the windows (<5s for Receiving/ReceiverFailed; the Idle/Stalled long windows mapped in planning).
- **Out:** the live MCP read-back verification *run* (Epoch-8 CLI driver / Epoch-10 E2E); the CLI/desktop
  surfaces (Epoch 8/9); the OTHER Epoch-7 scenario families (hard-signals P-005..P-008, error-baseline,
  activity-floor, fingerprint, severity-lifecycle, constellation, scrub/pipeline — each its own later chunk);
  any new fault primitive (the port-occupier already exists — this chunk only *drives* it from config).
- **Scope law:** every scenario carries its P-ID(s); no new inbound listener beyond the existing `:4317`
  port-occupier (released on cleanup).

## Surfaces / contracts touched

- `conductor-core::Scenario` / `PhaseSpec` (`scenario.rs`, `phase_spec.rs`) — add the `expected` (+ optional
  `holds`) TOML field(s) + garde rules; the `from_toml_str` deserialize path.
- `conductor-core::expected::ExpectedOutcome` (`expected.rs`) — wired into the scenario TOML surface.
- `conductor-core::pause` holds (`pause.rs`) — wired iff a lifecycle scenario needs a go/no-go (likely not for
  drive+observe timing claims; resolve in planning).
- `conductor-faults` port-occupier (Epoch 4, P-003 ReceiverFailed) — referenced/driven by the ReceiverFailed scenario.
- `conductor-core::SloTier` — per-scenario tier selection for the lifecycle windows.
- **Artifact:** new `scenarios/*.toml` connection-lifecycle file(s) (the first catalog entries beyond the fixture).
- Requirement contract: `refs/pulse-capability-spec.md` per-P-ID "Conductor verification" clauses.

## Open questions — resolve in planning (P4)

1. **Exact P-ID ↔ state ↔ SLO mapping** — read `pulse-capability-spec.md` for the precise P-001/P-002/P-003/P-004
   definitions, thresholds (Idle 10–60s, Stalled >60s, Receiving ≤1s, ReceiverFailed ≤2s) and SLO tiers.
2. **File granularity** — one TOML per P-ID (4 files) vs one connection-lifecycle scenario whose `phases` walk
   the whole state machine carrying `p_ids = [P-001..P-004]` (input.md frames it as one continuous walk).
   "No scenario without a P-ID" is satisfied either way; pick the shape that the scheduler + expected-block
   model express cleanly. **Likely AskUserQuestion.**
3. **Model-extension depth (follow-up d)** — does this chunk wire `expected` into the TOML (almost certainly,
   to express the timing windows), and does it also wire `holds`? Keep the extension minimal to what these 4
   capabilities need; defer broader expected-kinds to the family that needs them.
4. **Orthogonality expression** — how the error-burst-with-clean-stop guard (P-004) is encoded as phases +
   expected so connection-verdict stays independent of error-verdict.
5. **drive+observe split** — which assertions are auto (`expected` timing) vs operator-checklist (the visual
   lifecycle badge), and how the scenario marks the operator-checklist half (ManualCheck routing).

## Definition of done (chunk-level)

- The connection-lifecycle scenario TOML(s) for P-001..P-004 exist under `scenarios/`, deserialize +
  garde-validate via `Scenario::from_toml_str`, and produce a valid `PhaseTimeline` through the existing
  scheduler — proven by a fixture round-trip test (mirrors the `error-baseline-spike` fixture test).
- Any `Scenario`/`PhaseSpec` model extension (the `expected`/`holds` TOML wiring) is serde+garde-validated
  with valid/invalid `#[case]` rows; cross-field invariants hold.
- The ReceiverFailed leg references the existing port-occupier fault — no new inbound bind introduced.
- Determinism preserved: same scenario+seed ⇒ same stream shape (driven under `start_paused`).
- Gates: `cargo nextest` workspace green · clippy `-D warnings` clean · doctest 0 · no new dependency unless justified.
