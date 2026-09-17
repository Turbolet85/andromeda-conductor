# Curation log — 2026-09-17-keyboard-and-focus-order-coverage-ownership

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  — none
  Tier 2 (.claude/rules/*):                   + host-win32.md: "a Windows process is attributed by
                                                PARENTAGE, never by image name or StartTime"
                                              ~ frontend.md: knip entry extended (exit-1 on the residual
                                                12; a script registration makes an entry pattern redundant)
  Tier 3 (.claude/docs/session-learnings.md): — none
  Filters: 1 duplicate · 0 task-specific · 0 conflict · 2 confidence-threshold · 0 deferred (cap not reached)
  Extended: T2/frontend.md: "knip's `webdriver-io` plugin resolves DEPENDENCIES, never spec ENTRIES"
  CLAUDE.md size: unchanged — no Tier-1 write this wrap
```

## Applied

### T2 → `.claude/rules/host-win32.md` — process attribution by parentage
**Proof:** `Win32_Process` census taken before and after this chunk's `--e2e` leg. Twelve `msedgewebview2`
processes present; resolving each `ParentProcessId` to its image name gave six rooted at `SearchHost.exe`
(created 2026-09-16 18:33:31) and six at `WhatsApp.Root.exe` (2026-09-17 18:04:15), **none** descending from
`tauri-driver`, `msedgedriver` or `conductor-tauri`. Recorded at
`conductor-0.3.0/chunks/2026-09-17-keyboard-and-focus-order-coverage-ownership/evidence/process-census.md`.
Confidence 0.8 — verified by measurement (+0.4: it falsified a standing CARRY premise), specific technical
detail with context (+0.2), reached no other durable home (+0.2: the route disposition records THIS
measurement, not the reusable technique).
**Home:** `host-win32.md` by tiebreaker 6 (host/shell mechanics). That file carries no `paths:`, so it loads
every turn and the entry was judged at Tier 1's bar — written as one sentence plus its mechanism, ~600 B.

### T2 → `.claude/rules/frontend.md` — knip entry extended (Filter 1 additive facet)
**Proof:** `npm run knip` exits **1** on the 12 residual findings the matched entry already counts — measured
in a detached worktree at the parent commit `3ddd405` with `node_modules` junctioned, giving byte-identical
findings to the working tree; committed as `evidence/knip-at-HEAD-3ddd405.log`. Separately, adding
`test/a11y/check-claim-ownership.ts` to `knip.json`'s `entry` drew knip's own "Remove redundant entry pattern"
hint, because the `package.json` script already made it reachable; the registration was reverted and
`knip.json` ended byte-identical to HEAD.
Confidence 0.8 — verified by measurement (+0.4), specific technical detail (+0.2), no other durable home
(+0.2). The facet is one the matched entry lacks: it counts the findings but never says the command exits
non-zero on them, which is the fact that decides whether it can be a gate.
**Form:** amended in place, same tier and location, tagged `Extended 2026-09-17`; the evidence stays here, not
in the entry.

## Rejected

- **"Named in a prior plan is not evidence a gate was ever green"** — confidence 0.6 (measurement +0.4,
  technical detail +0.2), which REJECTS at the exact-0.6 mass point, and neither conditional signal applies:
  its subject is the phase P5 novelty predicate, a PIPELINE mechanism, not this project's code or practice.
  Routed where it belongs instead — the evolve friction record `contract.structural-blind-spot` at
  2026-09-17T16:08:53Z-b, which reaches the founder through diagnosis rather than through this project's
  always-loaded text.
- **"`tsx` is a TypeScript executor, not a test runner"** — confidence 0.6, and the no-other-home signal is
  disqualified by construction: this wrap amended the fact into `test-plan.md` §4 and re-derived it into
  `docs/tests-summary.md`. It has a durable home already.

## Recurrences (→ handoff, never silently dropped)

Filter 1's recurrence clause: a dedup hit whose matched entry records a DEFECT means the corpus did not
prevent the recurrence.

- `recurrence-despite-learning:` CLAUDE.md Tier-1 2026-09-06 ("the same rule has a FALSE-POSITIVE face … ask
  what OTHER content the pattern admits"). It recurred **three times in this one session**: the fan-out
  anchor probe flagged three correctly-anchored items at phase P2; `matrix.py audit` reported a phantom
  ledger-note because the plan's prose DENYING that token spelled it; and the report's own Symbols bullet
  stated a grep count of 1 where running it returned 2, the second hit being a comment this chunk had just
  written. Each was caught by reading the hits, which is what the entry prescribes — the entry is correct and
  did not prevent recurrence.
- `recurrence-despite-learning:` `amendment-flow.md` §Cascade ("every amended line is re-read for an intra-line
  DUPLICATE of the retired mechanism"). This wrap's own `test-plan.md:247` amendment left the retired claim
  standing in the sentence's head while the addition contradicted it 400 characters later; the step-2 sweep
  caught it one step after the apply. Cause worth the pipeline's attention: the edit's anchor began AFTER the
  head clause, so the stale mechanism was never inside the replaced span.
