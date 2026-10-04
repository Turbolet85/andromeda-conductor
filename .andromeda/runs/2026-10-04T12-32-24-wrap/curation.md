# Curation — 2026-10-04-real-model-test-surface-corrective

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "To split an oversized tests/*.rs target whose root holds private or byte-pinned items, move the families into CHILD modules … never sibling modules or new targets" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 3 below threshold · 0 conflict · 0 deferred (cap) · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "split a byte-pinned test target into child modules, never siblings or new targets"
  CLAUDE.md size: 138/200 · T1 47.8 KB, 9 over 600 B
```

## Applied
- T2 `testing.md` — child-module split of a byte-pinned test target (0.8 = verified by measurement 0.4 + technical
  detail 0.2 + no-other-home 0.2; load-bearing could not fire — the second corrective entry, a test-file split, is
  minted only at this wrap's P5).
  Proof: this chunk's P4 fork (research.md Open questions: a sibling module cannot see the rule's private items; a
  `pub` breaks the byte pins) and its gates — `real_model_harvest` 104/104 under nextest and `cargo test`, the rule
  section byte-identical to `1208ca5` (the diff probe printing nothing), 10 child modules compiled through
  `use crate::*;` with no `pub` added inside the rule (implement gate run, `.andromeda/runs/2026-10-04T12-10-49-implement/`).

## Filtered
- dup — "stop this repo's rust-analyzer flycheck before heavy cargo steps" (overseer directive): host-win32.md
  2026-10-02 carries it.
- dup — "a test whose fixtures are shaped by the change needs an inverse control" (the stray-module control on the
  listing arm): testing.md 2026-09-09/09-12 carries it.
- below threshold — "the feature-gated targets are now linted by `agent-run run`" (0.6 exact; the testing.md
  2026-06-21 entry stays true, and test-plan §9 was amended with the fact this wrap — a master home).
- below threshold — the `.git`-less copy recipe reproducing cargo-mutants' copy mode without a mutation run (0.6
  exact; carried in test-plan §4 by this wrap's R2 amendment and in the plan's gate entry).
- below threshold — sweep hazard `\b102\b` matching `anyhow 1.0.102` (0.2; one-off).

## Recurrence-despite-learning (→ handoff Deferred learnings)
- host-win32.md 2026-09-08 (the Bash cwd persists) — a `cd` at the head of one implement probe moved the session cwd
  (fourth consecutive session).
- host-win32.md §Transports ("Documents: the Write tool") — a `cat >> file <<EOF` append to a run-dir record was
  attempted at this wrap's P2 and blocked by the PreToolUse guard; re-done with the Edit tool.
