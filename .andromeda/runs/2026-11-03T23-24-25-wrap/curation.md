CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A chunk's own evidence file fails `rec.py check` until it is landed"
    Proof: implement's first `rec.py check --root . --no-heal` printed `records: docs · unlanded 1` and `check: 1 failure(s)` with `retired paths present 0` in every family; after `gate.py hygiene` (clean) and `rec.py land` (tx 101) it printed `check: clean` (`.andromeda/runs/2026-11-03T23-17-49-implement/outcome-2026-11-03-architecture-registry-check-retired.md`, Deviations). The overseer ruled the land the tool's own step (inputs#I2). Confidence 0.8: verified by a real gate reading +0.4 · specific technical detail +0.2 · no other durable home +0.2 (no route annotation, master, rule row, matrix note, contract document or hook carries it).
  Filters: 0 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 below threshold
  No-other-home: "A chunk's own evidence file fails `rec.py check` until it is landed"
  CLAUDE.md size: 139/200 · T1 6.1 KB, 0 over 600 B

Decisions & corrections, each weighed:
- The overseer's halt set for implement (stop on another dry-run reading, a store defect or a refused write): not a learning. A one-run directive with no "from now on"; it held, none fired.
- The overseer's ruling that the step-5 land is the tool's own step: applied, the Tier-3 entry above.
- The reconcile-then-`git rm` order (CARRY `cry-b7jcaggf`, F412): rejected, Filter 4. It is already carried on the route at an annotation position (the entry's CARRY) and is the F412 history; no new facet.
- The bash guard's refusal of `rec.py find --help` piped into `head`: not a learning. The hook that refused it is its home (Filter 4's hook clause), and it cost one re-issue.
- The run-dir named with the host's local date instead of `date -u`'s: rejected, Filter 4. One-off, one rename (0.2 specific − 0.3 one-off).
- This wrap's bash-guard refusal of a bare `cd` into the run dir: not a learning. The guard refused the act at the cost of one re-issue (a subshell), so `host-linux.md` §Paths did not recur. The guard is the home.
- The cites batch's five rows routed to P3 (`citation-dispositions.md`): left verbatim. They are dated entries in `verification-harness.md`'s `## Session Additions` (four citations into the old `test-plan.md` / `obs-plan.md` line numbers) and in `docs/session-learnings.md` (one into `test-plan.md`), each citing a rendered master file that no longer stands in the tree. They are history, preserve-verbatim. No measurement made their claims false, only their coordinates unresolvable, so no correction.
- The apply writer's step-2 patterns carrying only the retired clause's verbs (main added the name-token pattern): rejected, Filter 2 as a project learning. It is a pipeline fact, recorded in the evolve family (`reconcile` step record).
- The old `.andromeda/{doc}.md` reach (leaves and rules still pointing at removed render paths): not a learning. It is a standing state carried to the operator's card and the handoff.
