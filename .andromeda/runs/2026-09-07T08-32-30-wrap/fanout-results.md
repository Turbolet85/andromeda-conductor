# Fan-out results — 2026-09-06-halo-hue-budget-re-driven

7 Explore doc-agents, one per spec source, one parallel batch. Report was the sole input.
**5 proposals returned · 3 routine · 1 escalation group (2 proposals, applied on operator approval).**
No detector was re-spawned; no return needed stripping or carried an entity escape, so no `.raw-fanout-*`
twin is warranted — this file is the sanctioned audit artifact for the clean returns.

| doc | proposals | verdict |
|---|---|---|
| architecture | 2 (`D-arch-resources` primary + dependent) | **ESCALATED** → operator approved apply-both + a bounding playbook rule |
| test-plan | 2 (`D-tests-derived-count`, `D-platform-claim`) | routine (playbook `:127`; `:28` + Validate check 5) |
| obs-plan | 1 (`D-obs-instrumentation`) | routine (playbook `:28`) |
| security-plan | `proposals: []` | clean |
| design-system | `proposals: []` | clean |
| layout-templates | `proposals: []` | clean |
| a11y-plan | `proposals: []` | clean |

## Validation (the 6 checks)

1. **Playbook** — obs-plan §4 → `:28` (mechanism reconciled to measured reality; leaf name, field, budget,
   harvest tier all preserved). test-plan §3 → `:127` (derived-count on a set the code owns; the proposal
   already chose SET-NAMING, anchoring to `live_leg_order`, rather than a fresh literal). test-plan §9 →
   `:28` + check 5; rule `:46`'s dismissal explicitly does NOT cover it, because `§9:464` states the verdict
   in the doc's own voice. architecture ×2 → **no rule matched**, unease was precedent-shaped (arch's three
   existing `ANDROMEDA_PULSE_*` entries are all handles Conductor READS; this one it neither sets nor reads).
2. **Cross-contradiction** — none. arch's dependent and test-plan §9 qualify the SAME claim in two masters,
   consistently; that is the cross-master edge working, not a conflict.
3. **Intent-consistency** — aligned with the working-route entry + plan acceptance criteria.
4. **Absence needs evidence** — every proposal cited its sweep (obs-plan: `hue_update_ms` → 1 site;
   test-plan: `quiet window` / `not run-stable` single-site; arch: `BOOTSTRAP` → 3 sites).
5. **Expected-amendments floor** — all 6 accounted for: EA1/EA2/EA5 proposed; EA3 dispositioned
   no-such-claim (`grep -rc 'declares no emission of its own'` → 0 in all three named masters); EA4 is the
   ledger note, owner P7.3; EA6's TARGET string is carried inside the test-plan §9 proposal.
6. **Disproved-claims** — both report entries DISPOSED (#1 already corrected in plan/scope pre-implement,
   no master states it; #2 matched by the obs-plan proposal).

## Escalation

**arch `D-arch-resources` (primary + dependent, atomic).** Presented with three marked options; operator
chose *apply both + mint a bounding rule*. Rule appended to `playbook.md`: an external (non-`CONDUCTOR_*`)
handle Conductor neither sets nor reads enters arch's env registry ONLY when a SHIPPED artifact names it —
never one mentioned solely in a report or plan.

## Orchestrator-raised (no detector proposed these)

- `architecture:175` — a SECOND unqualified fixed-`3_600` statement inside the amended master itself; the
  cascade's same-master duplicate rule caught it.
- `.claude/rules/verification-harness.md:19` — the retired 4-leg composition standing in a rule file's
  GENERATED body; re-derived.
- Two further hits sit in preserve-verbatim curation homes (`CLAUDE.md USER:session-learnings`,
  `verification-harness.md ## Session Additions`) and were routed to P3 as in-place extensions, never edited
  by the cascade.

The sweep ran AFTER all five amendments were authored, with a positive control (`window IN FORCE AT BOOT`
fired on 2 files) before any zero-hit probe was trusted.

## Post-apply narrowing (P7, before commit)

The light gate's literal live re-run graded leg A `ManualCheck` where run 1 gave `KnownResidual`, on the
same tree, posture and `pulse-app` process. Four amendments authored above had claimed the stretched
bootstrap posture makes the `AutoResolved` arm reachable *independent of uptime*; measurement showed the
posture removes cause (a) only (`silence_cues_emitted: 0`, `services_in_bootstrap: 2` in BOTH runs) while
cause (b) still decides the leg (+45 s vs +137 s incident timing within the window). `architecture` §69,
`test-plan` §9, `verification-harness.md` item (e) and the scenario header were narrowed to
REACHABLE-not-RELIABLE, and the two affected sidecar entries corrected, before the commit.
