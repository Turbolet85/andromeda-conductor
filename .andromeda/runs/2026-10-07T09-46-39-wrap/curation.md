# Curation — 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "a real-model series whose drives share one growing data dir confounds each drive's ordinal with everything that accumulates there; keep a capture line that counts each input the model reads" (confidence 1.0)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 2 task-specific · 1 below threshold · 0 new conflict (1 carried → handoff) · 1 recurrence (→ handoff)
```

## Applied

- **Tier 2 · `.claude/rules/verification-harness.md` `## Session Additions`** — the shared-dir confound and the
  input-counting capture line, one entry (the overseer's two candidates, worded as one rule).
  Proof: the six committed captures of the 2026-10-06 and 2026-10-07 series, recounted at this wrap. Each prints
  `creating digest corpus retrieval rows:` and `pulse-log digest.corpus.retrieve:`; the pairs read 1 / 10, 3 / 13
  and 6 / 16 for d1, d2, d3 in BOTH series, with grades `Identified`, `Identified`, `NotIdentified` in both. The
  miss is the six-row drive 2 of 2; the one- and three-row drives pass 4 of 4; the row count is fixed by the
  drive's position on the one shared dir, so six drives do not separate it from the ordinal, the earlier
  incidents, the uptime or the canary's history (the chunk's `evidence/attempt-ledger.md`, the covariate section;
  `report.md`, the same table). The table exists only because the capture already printed the row count.
  Signals: verified by measurement +0.4 · specific technical detail with context +0.2 · explicit curation request
  +0.5 (the overseer's wrap relay §4 offers both as curation candidates, the agent's to word or drop) → 1.0.
  Class: harness leg design → `verification-harness.md` (tiebreaker 6); 647 B, under the Tier-2 cap.

## Filtered

- **dup** — `grep -c -F 'Previously Seen'` over a capture counts the rule text's mentions as well as the list
  lines. Covered by CLAUDE.md's 2026-08-21 entry as extended (ask what other content a pattern admits; read the
  hits).
- **task-specific** — an estimated timestamp written into a run-dir script; a hook refused it and the clock was
  read. One event, and the guard exists.
- **task-specific** — `inputs.py snap` takes no wrap step. Pipeline telemetry, recorded in the friction ledger;
  not a project learning.
- **below threshold (exactly 0.6)** — a relinked binary whose inputs did not change is byte-identical, so it is
  proven by build provenance and never by a digest that must differ. Measurement +0.4, specific detail +0.2. The
  no-other-home signal does not apply: the contract section this chunk wrote carries it.

## To the handoff

- **Conflict carried, not new:** CLAUDE.md's 2026-08-22 Tier-1 entry opens "This host is Windows-only"; this series
  again ran on the Linux dev host. The entry is the operator's to re-word.
- **recurrence-despite-learning:** CLAUDE.md 2026-08-21 (verify the artifact, not the exit code) — an anchored
  Edit that inserted a section into the attempt ledger dropped the next section's heading, the edit reported
  success, and the pushed pre-CI commit carried the ledger without it until this wrap restored it.
