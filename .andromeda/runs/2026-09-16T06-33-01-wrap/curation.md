# Curation — 2026-09-15-scenario-tier-honesty

Mode: default (auto-apply). Candidates analysed: 5 · applied: 3 · rejected: 1 · recurrence: 1.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (none)
  Tier 2 (.claude/rules/*):                   + host-win32.md: "a phrase wrapped across two comment lines is invisible to a line-granular grep"
  Tier 3 (.claude/docs/session-learnings.md): + "The amendment cascade sweeps the spec tier, not the committed data"
                                              + "A review edit re-opens the checks that READ the edited artifact"
  Filters: 0 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  Recurrence: recurrence-despite-learning → handoff
  CLAUDE.md size: 136/200 · T1 45.6 KB, 8 over 600 B
```

## Applied

### T2 → `.claude/rules/host-win32.md` — the wrapped-phrase sweep hazard
Confidence **0.8** = explicit user correction (+0.4) + verified by measurement (+0.4).
**Proof:** the operator raised it as the P5 review's polish item; measured this session —
`grep -l 'MCP round-trip' scenarios/*.toml` returns **3**, the comment-marker-stripped wrap-tolerant sweep
returns **4**; the fourth is `scenarios/service-constellation-discovery.toml`, which ends line 18 with
"Conductor's own MCP" and resumes line 19 with "round-trip, not a Pulse-internal duration". A first
"wrap-tolerant" attempt that collapsed whitespace WITHOUT stripping the comment marker also returned 3 — the
`#` sat mid-phrase — which is why the entry names the order of the two operations.
**Routing:** Tier-2 home registry sends host/shell probe mechanics to `host-win32.md`; that file carries no
`paths:` frontmatter, so tiebreaker 2's exception applied and the candidate was judged at Tier 1's ~600 B
one-sentence bar, which it meets. Placed beside the 2026-09-10 entry it inverts (a long line hiding many
clauses vs one phrase spanning many lines) so a reader hitting either meets both.

### T3 → `.claude/docs/session-learnings.md` — the cascade's reach
Confidence **0.8** = verified by measurement (+0.4) + specific technical detail with context (+0.2)
+ reached no other durable home this wrap (+0.2).
**Proof:** obs-plan retired the "MCP round-trip" gloss on 2026-09-10 (its own amendments sidecar records it);
at HEAD on 2026-09-16 the wrap-tolerant sweep found it still stated in 4 committed scenario TOMLs, with 3 more
carrying the sibling wording — while the same sweep over all seven masters returned **0 hits in each**.
**No-other-home check (the +0.2's precondition):** this wrap applied 0 amendments, minted no playbook or
drift-base rule, dispositioned it to no matrix ledger note, and the route carries it at no annotation
position — so no other channel took it. Recurrence is n=1, which is below the bar for proposing a
`drift-base.md` detector; if it recurs, that is the channel.

### T3 → `.claude/docs/session-learnings.md` — re-validation after a review edit
Confidence **0.8** = explicit user correction (+0.4) + verified by measurement (+0.4).
**Proof:** the operator's second P5 message raised it as a REQUIRED-RESOLUTION against validation check 4 (6).
The gap is measurable in this session's own trace: after applying the first polish I re-ran `gate.py --dry-run`,
the size check and the `new`/`baseline` pairing, and did not re-run the predicate that reads
`## Acceptance Criteria` — the one the edit had just invalidated by giving the criterion a named method that
no `[[gate]]` entry performed.

## Not applied

### Rejected — Filter 2 (task-specific)
The multi-line trailing TOML comment form (a continuation `#` line indented under `slo_tier`). A formatting
choice local to this chunk's files; one-off mention (−0.3) with no general pattern behind it.

### Recurrence — Filter 1, logged to the handoff, never dropped
The `duration_ms` → `gap_ms` probe miss at phase P1 (the first corpus measurement keyed phase duration on a
field the scenario files do not use and returned 0 ms for all 36; the real key is `gap_ms`, 99 occurrences vs
0). Filter 1 matches CLAUDE.md Tier 1's 2026-08-22 entry, whose own clause already says: "before grepping for
a token as a proxy for a practice, confirm the project prescribes THAT token." The matched entry is a DEFECT
record, so this is a recurrence rather than a duplicate — `recurrence-despite-learning`, logged to the
handoff's Deferred learnings. No third entry is minted: per the corpus rule a third restatement is not a
remedy, and the remedy is a CHECK in the owning step's reference.
