# Curation — 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin wrap

Scope: this window's conversation (a resume at P2; the P1 window's conversation is gone, so a correction only that
conversation held is not curated), plus the report's *Decisions & corrections*.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + security.md: "A boundary widening is ratified only by the founder's live word, never by a delegate: while the founder is away, the owning master records the widened state as a BREACH with a route-owned remedy, never as ratified."
      Proof: E1 at this wrap. The overseer declined the recommended ratification of the test-source capture pins
      ("a boundary widening halts for the founder live word (his ruling 2026-09-27); he is away"), and the breach was
      recorded in security-plan §Data Protection. Confidence 0.7: explicit correction +0.4, halting imperative +0.3.
      `rules/security.md` has no `paths:`, so the entry is held to the Tier-1 bar (one sentence, 224 B).
    ~ verification-harness.md (correction, cap-exempt): the 2026-06-27 entry's "four diagnostic-quality capabilities … sit in `conductor_core::UNBACKED_AUTO`" → backed, out of the pin, `[corrected 2026-09-29: …]`.
      Proof: report Counts bullet — `UNBACKED_AUTO` 8 → 4 (`drift.rs:61`); `v3-10` MET; cascade sweep rows diag-quality
      @c2330 and unbacked @c2412 on that line (`cascade-dispositions.md`).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict · 0 deferred (cap) · 3 below threshold
    - dup: an agent-launched `pulse-app` needs `ANDROMEDA_PULSE_MODEL_PATH` + `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` —
      `docs/session-learnings.md:285` already carries the run recipe. The a1 `model_not_configured` fault reproduced
      a documented recipe → handoff Deferred learnings as recurrence-despite-learning.
    - below threshold, 0.6 (verified +0.4, detail +0.2; its fact rides this wrap's P5 route annotation on the new
      series entry): a data dir's NAME reaches the model and Pulse's scrubber may rewrite it.
    - below threshold, 0.6 (its fact was amended into security-plan's ingest row this wrap, so no conditional
      signal): an MCP id sweep anchored on the active set misses incidents that auto-resolved before the poll.
    - below threshold, 0.2 (one-off, no gate failure): bash expands PowerShell's `$_` inside a double-quoted
      `-Command`.
  Recurrences → handoff:
    - host-win32 "Encoding & heredocs" [corrected 2026-09-23]: this window, the bash-guard hook blocked an inline
      python edit carrying a doubled backslash. The P1 window's lone `chr(92)` class failure is in the report.
    - the documented Pulse run recipe (model env), as above.
  CLAUDE.md size: see P7's health row.
