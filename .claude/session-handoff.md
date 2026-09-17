# Session Handoff

**Last Updated:** 2026-09-17T17:46Z
**Branch:** `build/conductor-0.3.0` · **1 ahead** of `origin/build/conductor-0.3.0` as read at this wrap's
Setup; this wrap's own commit and push land after this line is written, so a next session measuring 0 ahead
with HEAD == upstream reads the push as landed.
**Status:** clean — drift 0, 6 amendments applied, 1 escalation raised and resolved with the operator.
**Last Commit:** `feat(2026-09-17-keyboard-and-focus-order-coverage-ownership)` — see below.

## Position

- **Done: `2026-09-17-keyboard-and-focus-order-coverage-ownership`.** Every `a11y-plan` §5 keyboard/focus
  claim now has exactly one stated owner or a recorded gap, and the statement is **mechanically enforced**
  rather than prose-only: a committed claim→owner enumeration plus a checker that fails any owner whose spec
  asserts nothing. **This completes Epoch 3** — its markerless tail is now empty.
- **Next:** `Real-model leg posture and grading rule` — `working-route.md:42`, head of the markerless tail,
  **Epoch 4**. It carries one PREREQ (below).
- **Coverage 6/11 verified · 5 unclaimed** — unchanged. `v3-03` deliberately NOT claimed; see below.

## Work done

Two new TS files under `crates/conductor-tauri/ui/test/a11y/` — `claim-ownership.ts` (10 rows over §5's nine
bullets) and `check-claim-ownership.ts` (run by `tsx`, no browser, no driver, no Pulse). One npm script,
`a11y:ownership`. One spec retitled: `accessibility.e2e.ts:450` named four claims in its title and asserted
none, so it now says what it is — a subject-absence marker whose claims the driven arm owns. Zero `.rs`
delta, zero dependency delta.

The checker's standing verdict: `10 claims · 5 owned (3 operator-local, carve-out) · 2 n/a-by-construction ·
3 recorded gaps`. Its anti-vacuity arm was proven able to FAIL by a one-shot known-positive control.

## Why `v3-03` is still pooled

Its acceptance quantifies over **every** §5 claim — "and the claim is actually asserted in that suite".
**Three claims are asserted by no suite at all**: `:354` and `:356` coverage-matrix row navigation, and
`:370` the first-class shortcuts. Closing them is new keyboard coverage, so the cap stays pooled with a
failed-concretization `notes` line rather than being claimed over a text the evidence contradicts. The new
route entry (below) owns exactly those three plus the SC 2.4.7 gap.

## Drift resolved — 6 amendments, 0 open

`a11y-plan.md` 5 · `test-plan.md` 1. The other five masters returned clean, all seven having evaluated
`D-platform-claim` and declined it on a stating-sentence test.

- **§5** gained owner sentences on six bullets — the driven arm named for trap, containment and restoration;
  three claims recorded as gaps with a route owner.
- **§11 Strategy** gained the substitute-gate sentence `v3-03` asks for: what CI gates in place of a suite it
  cannot run, each item stated as *not* a substitute for the assertions themselves.
- **§5 `:366`** no longer attributes restoration to a "Radix AlertDialog default" — the last survivor of a
  reading `:362`, `:131`, `:253` and `rules/a11y.md` had all already retired.
- **§5 `:355`/`:362`** now include the operator-checklist rows in the HOLD cycle, matching `:131`/`:253`.
- **§1 `:115` CARRY 1 DISCHARGED** — the verbatim dittography repaired by offset (3691 → 3619 chars, the
  clause now once). Coordinates re-measured at this wrap before the edit.
- **`test-plan` §4** — the ui/ proving mechanism named as the SET of executing npm legs, not the wdio legs
  alone. The sweep caught this amendment leaving the retired claim in its own sentence's head; corrected.

**One escalation, resolved with the operator:** how §5 should record three claims no suite asserts, and
whether to mint the playbook rule the dittography class lacked. Both approved — gaps recorded with a named
route owner; rule minted (48 now).

## Curation

T1 0 · **T2 2** (`host-win32.md` — a Windows process is attributed by PARENTAGE, never by image name or
StartTime; `frontend.md` — knip's residual 12 make `npm run knip` exit 1, so it cannot be a gate expecting
exit 0) · T3 0. Filters: 1 duplicate · 2 confidence-threshold · 0 deferred.

**Recurrences (corpus correct, failure reproduced anyway):**
- `recurrence-despite-learning:` CLAUDE.md T1 2026-09-06 (the false-positive face). It recurred **three
  times this session** — the fan-out anchor probe, `matrix.py audit`'s phantom ledger-note (prose *denying*
  the token matched it), and the report's own grep count of 1 where running it gave 2.
- `recurrence-despite-learning:` `amendment-flow.md` §Cascade (re-read every amended line for an intra-line
  duplicate). This wrap's own `test-plan:247` amendment did it; the sweep caught it one step later.

## Notes

- **CARRY 3's premise is DISPROVED, not carried.** All twelve `msedgewebview2` processes are children of
  ordinary desktop apps — six rooted at `SearchHost.exe`, six at `WhatsApp.Root.exe`, **none** descending
  from `tauri-driver`, `msedgedriver` or `conductor-tauri`. This chunk's `--e2e` leg left zero survivors of
  its own. There is no orphan defect in the routine arm to fix. Evidence: `evidence/process-census.md`.
- **A restored pin, not a continued one:** `PREREQ: close rust gate deferral` was ABSENT from the tail —
  the predecessor deferred `cargo clippy` and never pinned it. Origin traced through the chunk reports to
  `2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration`; this is the **2nd** consecutive
  deferral, so the **3rd trips the age trigger**.
- **A red this chunk did not introduce:** `npm run knip` exits 1 on 12 pre-existing unused exports. Basis is
  a detached worktree at `3ddd405` with identical findings (`evidence/knip-at-HEAD-3ddd405.log`); owner is
  this wrap's P5 pin. Worth noting for plan authoring: the gate was listed with `expect = ['exit 0']`, which
  the tree could never satisfy — P5's novelty check asks whether a command was *named* before, and **named
  is not green**.
- **Still open from prior sessions:** the n=1 deferred escalation class · the audit-debt chunk's discarded
  wrap `gates` evolve record · the `quantile` 14-vs-11 correction for `code-metrics.ndjson` · `v3-08` BLOCKED.
- **Last failed command:** none.
