# Fan-out results — 2026-09-10-live-pulse-in-lane-scenario-round

Seven Explore doc-agents, one per spec source, run in one parallel batch against
`conductor-0.2.0/chunks/2026-09-10-live-pulse-in-lane-scenario-round/report.md`.

## Verdicts — 7/7 `proposals: []`

| doc | detectors evaluated | verdict | basis given |
|---|---|---|---|
| arch | D-arch-resources · D-arch-decisions · D-platform-claim | `proposals: []` | zero source delta; no symbol/crate/dependency; the three exercised surfaces pre-existing and already registered; the disproved claim is scenario-level, not a platform verdict |
| security-plan | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim | `proposals: []` | no new external-input surface; sidecar/preflight rules CONFIRMED not changed (fixed-name-on-`PATH` probe green, readiness gate intact); no dependency delta; platform sentences confirmed rather than falsified |
| design-system | D-design-tokens · D-design-derived-count · D-platform-claim | `proposals: []` | no new UI element; `Counts moved` is "none — verified"; `slo_tier` carried only as the closed SET (`:322`, `:86`), which the detector contract defines as correct and NOT a hit |
| layout-templates | D-layout-surface · D-layout-derived-count · D-platform-claim | `proposals: []` | (bare return) |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim | `proposals: []` | swept for the moved literals (28/29/32/562/902) and `constellation-severity`/`<20s` — none present; the `120s` occurrences match the CONFIRMED Pulse constant; the live legs are the pre-existing §2/§9/§11 operator-gate exception, not new non-determinism |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim | `proposals: []` **+ one adjacent observation** | 0/0/1 `run_check` matches `obs-plan.md:229`; the only OTLP named is Pulse's own receiver binds, not Conductor self-obs; evidence files measured clean of host paths |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema · D-platform-claim | `proposals: []` | no new interactive element; the eleven-key reproduction (§1 :96-108, §3 :233-245) still matches obs §6; a11y's platform verdicts already state the measured SET and nothing retires them |

## The one non-empty signal — and how it was disposed

The obs-plan agent returned `proposals: []` but appended an **adjacent observation, explicitly outside all
four of its invariants**: the report's measurement that the latency delta "spans the whole emission window"
sits beside `obs-plan.md:349`'s gloss of that same delta as "(Conductor's MCP round-trip)", noting "no
detector scoped to obs-plan covers latency semantics".

**Orchestrator disposition — RAISED and APPLIED as routine (Validate check 5).** Verified before acting:
`obs-plan.md:349` does carry the gloss; the report substantiates the falsification with three measurements
(`latency_ms` 18169 / 35120 / 30212 against sidecar `duration_ms` 0–22). Playbook `:28` governs it
(reconciling a spec's illustrative wording to the sound shipped implementation where the invariant still
holds) → routine, not an escalation; the passage's conclusion is unchanged and strengthened.

**Sweep for duplicate occurrences** (the cascade's verbatim-citation rule): `grep -n` for both dash forms of
the delta across `.andromeda/*.md` → 9 hits, each READ; **only `:349` glosses it as a round-trip**
(`architecture.md:7`/`:60`, `obs-plan.md:399`/`:408`/`:560` state the formula plainly, several with the
correct "(wall-clock)"). A `round-trip|roundtrip` sweep across all seven masters returned only the CANARY
round-trip and a serde round-trip — different, correct usages, read and dismissed.
`grep -rn "MCP round-trip" .claude/ CLAUDE.md` → **0**, so no distillation leaf carried the claim and none
needed re-derivation for it.

## Validate checks

1. **Playbook** — one proposal-equivalent raised; `:28` matched with every qualifying clause holding. No two-rule collision.
2. **Cross-contradiction** — none (no two proposals; the single raise touches one section).
3. **Intent-consistency** — the report matches the chunk's working-route entry and its plan acceptance criteria; no divergence.
4. **Absence needs evidence** — every "none" in the report's Changes carries its basis (`git status --short`, the `cargo audit` package-count line, the grep sweeps above).
5. **Expected-amendments reconciliation** — the plan lists "none to any of the seven masters"; re-verified at `architecture-amendments.md:403` and `architecture.md:69`, both still true. The floor did NOT under-run: the one master amendment applied this wrap came from check 5's own raise, not from the plan's list.
6. **Disproved-claims disposition** — two entries, both DISPOSED: the scenario `slo_tier` claim → routed to `.andromeda/residuals.md` as an owned `open` residual (operator-directed, P5); the `obs-plan.md:349` gloss → amendment applied above. The report's bullet was updated to name the second, which it had under-run at authoring time.

## Outcome

**1 amendment applied · 0 escalations · drift = 0.**
