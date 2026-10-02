# Curation — 2026-10-02-p-075-assert-round-against-pulse

**Sources:** this window's conversation (the P2 resume plus the founder ruling relayed mid-turn) and the report's
*Decisions & corrections*. The implement window's conversation is gone, so only what the report recorded from it
could be curated.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings): none
  Tier 2 (.claude/rules/*): + verification-harness.md: "Before planning a live leg's precondition from the SUT's UI
    source, read each gating predicate's DEFINITION — a name like `isOpen` is a claim about the code, not about what
    is on screen …"
    Proof: the chunk's P3 premise ("P-037 needs Pulse's Report window OPEN") came from reading
    `useReport(isOpen ? id : null)` as visibility. P5 sourcing showed `isOpen` is `effectiveId !== null`
    (Pulse `ReportWindow.tsx:23,32` at S `03ec944`), so the hidden window self-selects `findings.rows[0]` and a row
    click PINS the selection. The overseer ratified hands-off, and leg R's render sample then fired with no desktop
    input (`evidence/pulse-r.jsonl`; report Spec claims disproved 2; plan Constraints "REJECTED at P5").
    Score: verified by measurement +0.4 (falsified the plan's premise and changed the round's design) · specific
    technical detail with context +0.2 · no-other-home +0.2 = 0.8. The fact sits only in the chunk's own
    scope / research / plan, with no master, route annotation or matrix note.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 1 task-specific · 0 conflict · 0 deferred · 1 recurrence (→ handoff)
  - dup: "`grep -cF span-a` over obs-plan matches `span-attribute`". This is a token-proxy false positive already
    covered by the CLAUDE.md 2026-08-21 entry as extended 2026-09-06 ("ask what OTHER content the pattern admits").
  - dup: "the operator pass's commit and push were made by the agent on the overseer's word". This is the CLAUDE.md
    2026-10-01 Tier-1 entry verbatim in substance.
  - task-specific / below threshold: "deterministic incident formation took 46 ms at S against ~110 s at `83d4060`,
    so a poll budget sized for one HEAD can be generous for another". It is a one-off dated measurement:
    +0.4 +0.2 −0.3 (one-off) = 0.3. Its general form is the CLAUDE.md TIME-axis entry (re-check the SUT's HEAD).
  - recurrence-despite-learning: this wrap's first cascade sweep keyed `perm-degraded` on `permanently` and missed
    "permanent `degraded_mode`" in a leaf. The CLAUDE.md 2026-08-21 entry as extended 2026-09-06 already states
    that a claim's wording escapes a tight pattern ("use a pattern WIDER than the one that reads naturally"). It was
    caught by a hand probe of the leaves and the sweep was re-run (`cascade-dispositions.md`).
  - not curated: the founder ruling of 2026-10-02 ("do everything planned for 0.3.0 properly, defer nothing"). It is
    version-scoped direction, so its home is the route, minted at P5, not a curation tier.
  CLAUDE.md size: 138/200 · T1 47.8 KB, 9 of 17 bullets over 600 B
