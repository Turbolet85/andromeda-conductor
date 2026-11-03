# Curation — 2026-11-03-linux-only-base-ci (wrap 2026-11-03T07-20-45-wrap, P3)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none new · 1 correction in place: security.md (2026-09-06 entry)
  Tier 3 (.claude/docs/session-learnings.md): + "Cite another repository's file by the text it holds, never by a line number"
                                              + "The SessionEnd stamp dirties the tree an operator-pass push guard reads"
  Filters: 3 dup · 1 task-specific · 0 conflict · 0 deferred (cap) · 1 confidence · 2 → route (P5) · 8 not a learning
  Load-bearing: "The SessionEnd stamp dirties the tree an operator-pass push guard reads" → Fast feedback on Linux
  No-other-home: "Cite another repository's file by the text it holds, never by a line number"
  CLAUDE.md size: 139/200 · T1 6.1 KB, 0 over 600 B

## Applied
- T3 "Cite another repository's file by the text it holds, never by a line number" — confidence 0.8 (verified by
  measurement +0.4 · specific technical detail +0.2 · reached no other durable home +0.2).
  Proof: architecture fct-4rt3oej6 cited `andromeda-pulse Cargo.toml:154`; the pin stands at line 153 (inputs#I15);
  the row was `held` since the 2026-10-12 wrap; a citation reader failed `[DISPOSITION]` checking 153 against this
  tree's 84-line `Cargo.toml`; the apply writer's `rec put` refused the re-point as `digit moved`
  (`apply-architecture.md` architecture 16); settled by `edits-main.md` (tx 64), naming the pin by its text.
- T3 "The SessionEnd stamp dirties the tree an operator-pass push guard reads" — confidence 0.8 (verified by
  measurement +0.4 · specific technical detail +0.2 · load-bearing for the next entry +0.2, «Fast feedback on Linux»,
  whose operator pass pushes through the same guard).
  Proof: the report's Decisions & corrections — the push entry's clean-tree guard would have refused on the
  hook-written stamp; the stamp went in its own commit `5eb0109` (the second time, after `401d9b6`); the guarded push
  then ran green twice (report Outcome, `--operator 18`).
- Correction in place — `.claude/rules/security.md` 2026-09-06 entry: "`scripts/agent-run.{sh,ps1}` expand" →
  "`scripts/agent-run.sh` expands … [corrected 2026-11-03: the PowerShell twin `agent-run.ps1` retired — `agent-run.sh`
  is the one harness]" (cap-exempt). Proof: this chunk's `git rm scripts/agent-run.ps1` (report Files, Retired).

## Decisions & corrections — each entry's disposition
1. F412, the spine `--reconcile` heal its dry-run did not show — → route (P5): carried on the minted entry
   «Architecture-registry check retired», whose masters reconcile drops the architecture pair (the owned channel); its
   verify-the-artifact half is a duplicate of CLAUDE.md 2026-08-21 (Filter 1).
2. Step 9's command form from the overseer's word — rejected, Filter 2 (task-specific: one step of one chunk).
3. `git rm` refused a retired product file carrying an uncommitted render; `-f` on the store record compared equal —
   → route (P5): rides the same carry as 1 (the minted entry's `git rm` of the architecture pair).
4. `cargo audit` green ≠ `cargo deny` green, measured again — rejected, Filter 1 (duplicate of security.md 2026-06-23).
5. External decay on an unchanged tree reached the first CI read — rejected, Filter 1 (duplicate of the playbook's
   External decay rule and security.md 2026-08-09).
6. The push entry's clean-tree guard and the hook-written stamp — applied, Tier 3 (above).
7. The Bash guard refused `rec.py … | cut | head`, a loop-body form passed — not a learning: tool telemetry, recorded
   as evolve friction, never curated (curation-guide §Analysis scope).
8. Sweep hazard: four unfired controls — rejected, Filter 1 (duplicate of CLAUDE.md 2026-09-17 and the cascade
   contract's unfired-control rule).
9. Sweep hazard: `\.ps1\b` and `Platform…Windows` catch true claims — rejected, Filter 4 (0.2: specific technical
   detail alone; it changed no disposition).
10. The P5 card's three Beyond-the-plan rows — not a learning: route items, P5's.
11. The `d227e59` red discharged — not a learning: route, P5's.
12. The masters' `--product none` belongs to the arch-registry entry — not a learning: route, P5's.
13. No capability claimed — not a learning: P7's coverage no-op.
14. No failed command — not a learning: P6.
15. `health.py` check 13 (F411) — not a learning: an owner outside this repository.

## This window's own candidates
- The fan-out render's `{marker}` false positive; the leaf re-spawn gap between `cascade.py` and `fanout.py`; the
  sidecar read-back past the output cap — not learnings: tool telemetry, recorded as evolve friction (reconcile).
- The leading bare `cd` in the resume's first calls — Filter 1 recurrence of host-linux.md §Paths (`cd` only in a
  subshell): logged to the handoff's Deferred learnings as `recurrence-despite-learning`, never a new entry.

## The curation homes the cascade routed here (cascade-dispositions.md, curation rows) and the citation rows
- `security.md:54` — corrected in place (above).
- `frontend.md:16` (a retired-surface rule file's 2026-06-24 entry) · `testing.md:83` (2026-09-06, "in BOTH shells
  carried", past tense) · `verification-harness.md:49` (a proof's "identical `.sh`/`.ps1` semantics" of its date) ·
  `verification-harness.md:58` (2026-09-07 extension, the retired `--e2e` leg) — no change: dated records of their
  date's state, their directives not resting on the twin.
- `session-learnings.md:1100`, `:1102` (citation rows, the cited chunk reports gone from the tree) — no change: dated
  records of where a reading stood; those reports live in the store (`rec.py render doc:{marker}/report`).
