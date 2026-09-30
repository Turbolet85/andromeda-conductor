# Curation — 2026-09-30-mutation-gate-grades-every-tally-it-rests-on

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): extended in place (1 write)
  Filters: 1 dup (generated-body match) · 0 task-specific · 0 conflict · 0 deferred · 2 recurrence-despite-learning (→ handoff)
  Extended: T3/session-learnings.md: "Ownerless work discovered mid-chunk takes the armed-orphan form, not a plan note (2026-09-16)" + resolved tag
  CLAUDE.md size: 137/200 · T1 47.3 KB, 9 over 600 B

## Applied
- T3 in-place extension — the 2026-09-16 entry's worked example (the gate "never reads `timeout.txt`") gains a
  `[resolved 2026-09-30 …]` tag; the lesson (armed-orphan form) stands untouched.
  Proof: this wrap's cascade sweep row `.claude/docs/session-learnings.md:669` (`no-timeout-read`, `silent-regress`,
  curation) against the shipped gate — `scripts/mutation-gate.py` `grade()` reads all four tallies, and the selftest's
  `timeout-unexpected` arm detects a caught→timeout move (`evidence/selftest-verdict.md`).

## Filtered
- "Mutation testing runs at the epoch-boundary code audit only, never per chunk" — duplicate: this wrap's cascade wrote it
  into `.claude/rules/testing.md`'s generated body (and test-plan §9 owns it).
- recurrence-despite-learning (→ handoff Deferred learnings): the deferral-void basename grep matched `security.md` in a
  doc comment while the uncommitted file was a run-dir extract — Tier-1 2026-08-21 as extended 2026-09-06 (false-positive
  face: "ask what OTHER content a pattern admits"). Caught by reading the hit.
- recurrence-despite-learning (→ handoff): this wrap's cascade pattern for "never reads `timeout.txt`" required "reads" and
  missed the leaf wording "never `timeout.txt`" — Tier-1 2026-08-21 as extended 2026-08-22 ("a count built by
  pattern-matching is bounded by the author's imagination"). Caught by reading the flagged leaf line whole.
- The bash-guard hook refusing any doubled backslash (even in a grep pattern) — one-off, below threshold; the host rule's
  2026-09-23 backslash clause already covers the mechanism.
