# Fan-out results — 2026-09-15-scenario-tier-honesty

7 Explore doc-agents, one per spec source, one parallel batch. **Every return: `proposals: []`.**
**Amendments applied: 0 · escalations: 0 · cascade: not entered (no spec source changed).**

No raw twin was saved for any doc: a twin is warranted for a return that CARRIED PROPOSALS (so an applied
amendment can be audited against what was proposed). All seven returned empty; stripping removed only trailing
evaluation commentary the prompt had asked them to omit, never content. This file is the sanctioned audit
artifact for them, and it carries each agent's substantive basis rather than only its verdict.

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions · D-platform-claim | `proposals: []` |
| security-plan | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim | `proposals: []` |
| design-system | D-design-tokens · D-design-derived-count · D-platform-claim | `proposals: []` |
| layout-templates | D-layout-surface · D-layout-derived-count · D-platform-claim | `proposals: []` |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim | `proposals: []` |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim | `proposals: []` |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema · D-platform-claim | `proposals: []` |

## Why each family found nothing

- **Structural detectors** (arch-resources, arch-decisions, security-input/subprocess/deps, design-tokens,
  layout-surface, tests-coverage/framework, obs-instrumentation/stack/redaction, a11y-surface) all key on the
  report's `Symbols / APIs`, `Crates / modules`, `Dependencies`, `Schema / config` and
  `Coverage of new surfaces` bullets, which read `none` for a data-only chunk. No new boundary, spawn, crate,
  dependency, operation or UI element exists for them to bind to.
- **`*-derived-count` detectors** (design, layout, tests) key on `Counts / qualifiers moved`, which reads
  `none — verified`. Three agents independently re-swept their own doc for all 11 touched scenario names —
  design-system 0 matches, layout-templates 0 matches, obs-plan 1 match carrying no tier value — and each
  confirmed the tier tokens their doc carries are the closed-set enumeration or a `<slo_tier>` placeholder,
  which the detector's own rule names as correct set-naming rather than a stale literal.
- **Plan↔plan binds** (tests-obs-harness, a11y-obs-schema) key on a harness / envelope / schema change;
  `Harness / gate surface` reads `none` and the envelope's eleven keys are unchanged.
- **D-platform-claim** found no trigger in any of the seven: the two wordings this chunk retired are
  latency-semantics glosses, not platform / runner / driver verdicts, and they were falsified in the committed
  scenario corpus rather than in a spec. Four agents ran their own wrap-tolerant sweep for the retired
  wordings and each returned 0 for its doc. Several noted that the report's own "Not a headless skip" line
  pre-empts the misreading, and that its runner-portability line CONFIRMS rather than falsifies.

## Orchestrator validation (main)

1. **Playbook** — not applicable: 0 proposals to match against the 46 rules.
2. **Cross-contradiction** — not applicable: 0 proposals.
3. **Intent-consistency** — ALIGNED. The working-route entry asks that every declared tier fit its phase
   duration inside the closed set or state why, with three situations kept distinct; the report's Outcome
   re-asserts all 13 acceptance criteria MET against the diff, and the three situations are enumerated in
   `evidence/tier-ledger.md`. No divergence to classify.
4. **Absence needs evidence** — PASSED, re-derived by the orchestrator rather than accepted from the agents.
   The report's load-bearing absence is "no master pairs any of the 8 re-tiered scenarios with a tier
   literal". Re-measured across all seven masters as a line-set intersection:

   | master | lines naming a touched scenario | lines with a tier literal | intersection |
   |---|---|---|---|
   | architecture | 70, 85, 135 | 4 | **none** |
   | security-plan | — | 0 | none |
   | design-system | — | 3 | none |
   | layout-templates | — | 12 | none |
   | test-plan | 155, 348 | 2 | **none** |
   | obs-plan | 349 | 12 | **none** |
   | a11y-plan | — | 3 | none |

   Line profile checked first, as the check requires: `architecture.md` carries 13 lines over 2 000 chars and
   its longest is **line 70 at 10 724 chars** — one of the hits — so it was read rather than characterised;
   it carries no tier literal at any offset. `test-plan.md` likewise carries 13 lines over 2 000 chars, and
   its two hits were resolved BY OFFSET:
   - `test-plan.md:155` (5 337 chars) @offset 684 — names `halo-hue-encoding` with
     `live_leg_budget_sec 180`, the harness LEG budget anchored to `live_leg_order`. That is a different
     budget from the scenario's `slo_tier`, and this chunk did not change that scenario's tier in any case.
     **No change.**
   - `test-plan.md:348` (505 chars) — a drive-order list of the five severity-family scenarios
     (`SCENARIO=<name> … run`), names only, no tier literal. **No change.**

   Every hit the search returned is read and dispositioned; none is characterised from a clipped view.
5. **Expected-amendments reconciliation** — the chunk's `plan.md` carries no `Expected amendments (wrap)`
   list (its only occurrence of the phrase is a conditional instruction for /implement, which found no
   qualifying master). The coverage floor is therefore empty and nothing can under-run silently.
6. **Disproved-claims disposition** — the report's `Spec claims disproved by measurement` entry ends DISPOSED
   on both halves. The seven masters: verified clean by a wrap-tolerant sweep, 0 hits in each, corroborated
   independently by four agents. The committed scenario corpus: the 7 files carrying a retired premise were
   CORRECTED IN THIS CHUNK — the fix is the disposition. The one remaining routed item is the ledger-precision
   note on `verification-matrix.json#v3-05` ("the nine ceiling-declaring ones" is true but non-exhaustive at
   11 post-change), routed to a dated `notes` line at P7.3 with acceptance untouched.

**Drift = 0.** Nothing staged to escalate; no user resolution owed; no body edited, so no sidecar entry and no
cascade re-derivation is due this wrap.
