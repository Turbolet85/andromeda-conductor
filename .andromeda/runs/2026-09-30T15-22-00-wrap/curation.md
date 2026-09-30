# Curation — 2026-09-30-the-sr-pass-regrades-on-the-os-input-path

CLAUDE.md ecosystem curated:
  Corrections (exempt from the cap):
    ~ CLAUDE.md USER:session-learnings, the 2026-08-22 entry's clause (3): "[corrected 2026-09-30: the key path now exists …]"
      Proof: C1 and the regrade — every OS browse key in NVDA's `Input:` log, 6 browse rows heard on the agent arm (`evidence/nvda-pass.json`, `evidence/confound-control.md`).
    ~ .claude/rules/verification-harness.md Session Additions 2026-09-02, clause (5): "[corrected 2026-09-30: … injected keys reach focus handling only in the window's first burst …]"
      Proof: C1, one session under the leg's own driver launch — injected 0/5 and 0/4, OS 5/5 (`evidence/confound-control.md` §Arms).
  Tier 1 (CLAUDE.md USER:session-learnings): none new
  Tier 2 (.claude/rules/*):
    + a11y.md: "Send a modifier through `SendInput` in its OWN call and release it only after NVDA has handled the key it modifies …" (confidence 0.8)
      Proof: live sr #1 went red at S1-04 — after one batched OS Shift+Tab NVDA logged every later OS Tab as `shift+tab`; the 9-key probe over the fixed script matched NVDA's `Input:` sequence exactly, and live sr #2 went green with 20/20 keys identical (report Deviation 2).
    + verification-harness.md: "A slot-gated leg that throws on a HARNESS defect … may be re-fired once … a second harness defect on the same leg stops and reports …" (confidence 0.7)
      Proof: the overseer's rulings at the sr-empty re-fire and the live-defect report, 2026-09-30 (report Decisions & corrections).
  Tier 3 (.claude/docs/session-learnings.md):
    + "A sweep over the masters needs `-oiE … | wc -l`, and an escaped pipe under `-E` is a literal" (confidence 0.8)
      Proof: this wrap's P1 site sweep returned false zeros under `-E` with an escaped pipe, and line counts on the masters' multi-KB lines; the re-count with `-oiE | wc -l` found the sites the report now lists.
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred by the cap · 2 rejected below threshold:
    - "the start-of-document reset stays injected because an OS Tab skips the host-chrome BODY stop" (0.6, exactly — rejects; the fact that the reset stays injected is amended into a11y-plan §3 and test-plan §6, and the reason lives in the spec's comment)
    - "an OS arrow key needs KEYEVENTF_EXTENDEDKEY or NVDA reads numpad 2" (0.2 — preventive, never measured failing)
  Recurrence: recurrence-despite-learning: CLAUDE.md USER:session-learnings 2026-09-17 (on multi-KB single lines `grep -c` counts LINES) → handoff Deferred learnings.
  CLAUDE.md size: 137/200 · T1 47.5 KB, 9 over 600 B
