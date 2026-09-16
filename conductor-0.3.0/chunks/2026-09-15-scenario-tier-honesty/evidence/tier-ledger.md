# Tier ledger — all 36 committed scenarios

Chunk `2026-09-15-scenario-tier-honesty`. State **after** the change. This is the enumeration that keeps the
three situations separately identified, so a ratified posture is never recorded as a defect and a defect is
never excused as posture (`verification-matrix.json#v3-05` limb (d)).

## How to read it

`slo_tier` bounds `latency_ms = read_back_observed_at − journal_emitted_at`, which is measured
**journal-relative across the whole run**: `conductor-run/src/execute.rs:68-69` stamps `journal_emitted_at`
before `run_timeline_observed` (line 75) executes the entire timeline, and lines 120-121 stamp
`read_back_observed_at` after the timeline completes and read-back returns. So a scenario's declared tier must
hold its own summed `gap_ms` — a delegated budget (what Pulse does after a stimulus) is a different quantity
and is graded at the harvest tier, never through this field.

- **Situation 1 — ceiling posture (9).** Declares `<90s`, the ceiling of the closed set, and exceeds it. No
  tier in the set can hold these; the ceiling plus a stated reason is the honest declaration. **Not a defect.**
  Precedent: `2026-08-18-error-baseline-spike-live-proof`, which re-tiered that scenario `<5s` → `<90s` on the
  "whole-run latency exceeds every tier by construction" rationale.
- **Situation 2 — was over every tier while declaring a smaller one (2).** Re-declared to the ceiling here.
  Post-change these read like Situation 1; the column records what they WERE.
- **Situation 3 — was mis-declared where a larger existing tier holds (6).** The genuine defects, corrected by
  re-declaring within the closed set. All six now fit.
- **fits (19).** Held their declared tier all along. The acceptance owes these no stated reason.

## The 36

| scenario | tier | summed `gap_ms` | fits | situation | reason owed | reason |
|---|---|---|---|---|---|---|
| `ack-cooldown` | `<90s` | 370000 | NO | 2 (was over every tier) | yes | stated |
| `activity-floor` | `<90s` | 3900000 | NO | 1 (ceiling posture) | yes | stated |
| `auto-resolve-idle-window` | `<90s` | 200000 | NO | 1 (ceiling posture) | yes | stated |
| `cadence-config` | `<20s` | 10000 | yes | fits | no | stated |
| `constellation-severity-live-wiring` | `<90s` | 30000 | yes | 3 (was mis-declared) | no | stated |
| `cross-incident-recurrence` | `<20s` | 6000 | yes | fits | no | stated |
| `degraded-mode-report` | `<20s` | 6000 | yes | fits | no | stated |
| `error-baseline-spike` | `<90s` | 150000 | NO | 1 (ceiling posture) | yes | stated |
| `exception-event-capture` | `<5s` | 2000 | yes | fits | no | none |
| `findings-counter-refresh` | `<20s` | 6000 | yes | 3 (was mis-declared) | no | stated |
| `fingerprint-distinct` | `<90s` | 30000 | yes | fits | no | none |
| `fingerprint-storm` | `<90s` | 24000 | yes | fits | no | none |
| `halo-breathing-encoding` | `<20s` | 10000 | yes | fits | no | stated |
| `halo-hue-encoding` | `<90s` | 180000 | NO | 1 (ceiling posture) | yes | stated |
| `high-severity-log-capture` | `<5s` | 4000 | yes | fits | no | none |
| `incident-auto-resolution` | `<90s` | 364000 | NO | 1 (ceiling posture) | yes | stated |
| `investigate-actions-functional` | `<90s` | 35000 | yes | fits | no | stated |
| `last-span-ago-tracking` | `<20s` | 6000 | yes | fits | no | stated |
| `latency-regression` | `<90s` | 370000 | NO | 1 (ceiling posture) | yes | stated |
| `live-only-service-truth` | `<20s` | 18000 | yes | fits | no | stated |
| `orthogonal-health-domains` | `<90s` | 18000 | yes | fits | no | stated |
| `pii-scrub` | `<5s` | 4000 | yes | fits | no | stated |
| `project-context-grounding` | `<20s` | 6000 | yes | fits | no | stated |
| `pulse-run-contract` | `<90s` | 120000 | NO | 1 (ceiling posture) | yes | stated |
| `receiver-failed-port-conflict` | `<90s` | 75000 | yes | fits | no | stated |
| `receiver-lifecycle-state` | `<90s` | 69000 | yes | fits | no | stated |
| `report-render-surface` | `<20s` | 6000 | yes | 3 (was mis-declared) | no | stated |
| `restart-suppression` | `<90s` | 167000 | NO | 1 (ceiling posture) | yes | stated |
| `root-span-error-scope` | `<5s` | 4000 | yes | fits | no | none |
| `service-constellation-discovery` | `<20s` | 6000 | yes | 3 (was mis-declared) | no | stated |
| `service-went-silent` | `<90s` | 335000 | NO | 1 (ceiling posture) | yes | stated |
| `severity-tier-autonomous` | `<90s` | 120000 | NO | 2 (was over every tier) | yes | stated |
| `severity-tier-curious` | `<90s` | 26000 | yes | fits | no | stated |
| `severity-tier-suggested` | `<90s` | 82000 | yes | 3 (was mis-declared) | no | stated |
| `span-status-error-detection` | `<5s` | 2000 | yes | fits | no | stated |
| `threshold-hot-reload` | `<20s` | 6000 | yes | 3 (was mis-declared) | no | stated |

## Roll-up

- **9 / 2 / 6 / 19 = 36** — the partition reproduces `verification-matrix.json#v3-05`'s `observed_gap` exactly.
- **11 scenarios still exceed their declared tier, and every one of them declares the ceiling `<90s`** — the 9
  Situation-1 files plus the 2 Situation-2 files re-declared here. No non-ceiling tier is exceeded anywhere.
- **0 over-tier scenarios lack a stated reason.** Every one of the 11 carries its reason on the `slo_tier` line.
- **5 scenarios carry no trailing reason at all** (`exception-event-capture`, `fingerprint-distinct`,
  `fingerprint-storm`, `high-severity-log-capture`, `root-span-error-scope`) — all five FIT, so the acceptance
  owes them nothing. This is not a fourth situation.

## Stated reasons: truth, not just presence

Seven files stated a reason resting on a premise that is false at HEAD; all seven were corrected in this
chunk. The reasons' truth is not greppable, which is why it is enumerated here rather than asserted by a gate
— but the two retired WORDINGS are swept mechanically by the plan's retired-premise probe.

| retired wording | files (before) | now |
|---|---|---|
| "…is phase timing, not the SLO budget" | `ack-cooldown` · `auto-resolve-idle-window` · `incident-auto-resolution` | 0 |
| "Conductor's own MCP round-trip" (gloss of the latency formula, retired 2026-09-10) | `findings-counter-refresh` · `halo-hue-encoding` · `report-render-surface` · `service-constellation-discovery` | 0 |

The round-trip row is why the sweep must strip comment markers and join lines before matching:
`service-constellation-discovery` wrapped the phrase across two comment lines, so a plain single-line grep
returned 3 of 4. The delegated-timing claim those four files also carry — that the real budget grades at the
harvest tier and never through `budget_ms` — is TRUE and was preserved; only the gloss of what `latency_ms`
measures was corrected.

## Measured this run

| probe | before | after |
|---|---|---|
| non-ceiling tier exceeded by own duration | 8 | **0** |
| files carrying either retired gloss (wrap-tolerant sweep) | 7 | **0** |
| tiers outside the closed set | 0 | **0** |

All 36 files parse under `tomllib`. No `gap_ms`, `seed` or `jitter_ms` value was changed.
