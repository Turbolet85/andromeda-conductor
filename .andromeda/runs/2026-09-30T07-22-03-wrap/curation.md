# Curation — 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + security.md: "A synthetic leak control a gate needs as its known positive lives in the gitignored `.andromeda/cache/`, never in a committed run dir …" (confidence 0.7)
  Tier 3 (.claude/docs/session-learnings.md): none
  Extended: T2/a11y.md: "2026-09-17: Integrity's SIGN is configuration-bound …" + "on the dev host the Evergreen runtime moves between sessions under an unchanged driver — read both versions before an `--e2e` leg" (confidence 0.8)
  No-other-home: "on the dev host the Evergreen runtime moves between sessions under an unchanged driver"
  Filters: 1 dup (dedup-reject vs this wrap's cascade: a11y.md Testing body now carries the probe-bound stall) · 0 task-specific · 0 conflict · 0 deferred · 3 below threshold

## Applied
- **T2 security.md — synthetic leak controls live in the gitignored cache.** Signals: operator ruling "the synthetic
  leak control must not be committed, and it must stay traceable" (+0.4), "must not" language (+0.3) = 0.7.
  Proof: `gate.py hygiene` refused `.andromeda/runs/2026-09-30T04-14-05-phase/p5-controls/hp-leak.txt` (P1 drive,msys ×3)
  at the operator pass; after the move to `.andromeda/cache/p5-controls/2026-09-30T04-14-05-phase/` (sha256
  `27f41469…`, note `hp-leak.MOVED.md`) it read `hygiene: clean` (42 files).
- **Extended T2 a11y.md 2026-09-17 — the dev-host runtime moves under an unchanged driver.** Signals: verified by
  measurement, a real gate failure (+0.4); specific technical detail (+0.2); reached no other durable home (+0.2 —
  no route annotation, no master amendment, no playbook rule, no matrix note carries it) = 0.8.
  Proof: implement run 1 (`evidence/race-witness.md`, Attempt 1) — `session not created: This version of Microsoft
  Edge WebDriver only supports Microsoft Edge version 152` / `Current browser version is 154.0.4258.37`, after a full
  release build; the operator then installed msedgedriver 154.0.4258.37.

## Rejected (below threshold / other home)
- A stall meant to meet a later WebDriver call binds to that call's own evaluation — 0.6 (measured +0.4, detail
  +0.2), exactly the bar; this wrap amended it into a11y-plan §9/§11 and the a11y.md body (a master home).
- Pulse's workspace key is a PATH — 0.6; amended into security-plan §Input Validation (a master home).
- Rebuild the SUT's MCP sidecar too when its scrubber changes — 0.6 (measured +0.4, detail +0.2), exactly the bar; the
  contract's §The 2026-09-30 series carries it.

## Recurrence (→ handoff Deferred learnings)
- `recurrence-despite-learning`: CLAUDE.md Tier-1 2026-08-21 as extended 2026-09-06 (the false-positive face: "a search
  for a test run's spec list matched the axe-core library source the harness INJECTS"). Recurred twice this session: an
  `--e2e` log grep for spec results matched injected axe source, and a `sha2` site count matched `sha256` at
  security-plan `:335`. Both caught by reading the hits.
