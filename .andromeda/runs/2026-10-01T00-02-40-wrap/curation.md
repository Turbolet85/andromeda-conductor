# Curation — 2026-09-30-full-gate-regression-over-the-moved-surfaces

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    corrected verification-harness.md (2026-09-02 SR entry): "paired events land milliseconds apart (a shared window,
      not two)" → not on this host; a row declared to share the previous instant is stamped AT that instant
    corrected frontend.md (2026-09-07 knip entry): "a11y-plan §11 forbids acting on such a report against the harness"
      → no such §11 rule; the residual (15) dispositioned, `npm run knip` exits 0
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup (bash-guard doubled-backslash / heredoc-to-file blocks → host-win32.md §Transports and its 2026-09-23
    clause; pulse-app forced stop → the agent memory) · 0 task-specific · 0 conflict · 0 deferred
  Extended: T2/verification-harness.md: "A slot-gated leg that throws on a HARNESS defect … re-fired once" + a
    grading-side fix is proven by replaying it over the session's captures, unaffected subjects byte-identical
  Corrections (cap-exempt): 2 — both in-place, `[corrected 2026-10-01: …]`

## Proofs
- verification-harness.md correction — Proof: `runs/sr-leg/actions.live.jsonl` of the first live fire, S2-01→S2-02 at
  53 ms and S2-07→S2-08 at 62 ms against `SHARED_WINDOW_MS = 50`; both first rows graded `not-announced` with
  `heard: []`; the re-fire with partner-at-instant stamps graded all 51 rows (`evidence/nvda-pass.json`,
  `evidence/regression.md` §SR harness defect). Operator directive: curate it where the harness parser rules live.
- frontend.md correction — Proof: research.md P3 (a11y-plan §11 `:511-587` grep: no unused-export / knip / harness
  rule; `npx knip` at HEAD: 15 findings); this chunk's `npm run knip` entry exit 0.
- verification-harness.md extension — Proof: the overseer's pre-re-fire condition; `replay-subject.ts` over the
  session's sr-empty (10 rows) and sr-error (4 rows) captures, 0 stamps moved, graded rows byte-identical
  (`evidence/regression.md` §SR replay condition). Score: operator directive +0.4 · verified by measurement +0.4 = 0.8.

CLAUDE.md size: read at P7 (health check 1).
