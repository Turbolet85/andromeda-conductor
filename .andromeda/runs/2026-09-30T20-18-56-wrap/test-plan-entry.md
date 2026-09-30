
## 2026-09-30-the-screen-reader-content-findings-fixed — the browse zone empties; T-01's quiet window and the SR clock calibration
**Section:** §1 Untestable (browse-mode zone) · §2 Deterministic bullet (the `sr*` real-wall-clock exceptions) · §6 E2E desktop-webview row (the screen-reader leg) · §11 E2E no-sleep ban
**Change:**
- §1: the zone's definition kept; it holds NO row as of the 2026-09-30 regrade (`not-run-here 0` over 51 rows).
- §2: the live `sr` suite's real-wall-clock list gains a 170 s quiet window inside the walk before T-01's second, un-stopped run (a Pulse-side precondition, not synchronisation); the grading timeline calibrates NVDA's log clock per session from the stimulus pairs.
- §6: every browse row now drives an OS key (a bounded walk keyed on speech, a miss recorded, never thrown); the record is graded with the per-session clock calibration — only a stretch more than 300 ms over the session baseline is shifted back, a non-step stretch voids the session, the calibration rides each subject's `clock` with its pair table; known limitation: one slow key send can cross 300 ms alone (357 ms on one key, no grade changed). Was "one preflight canary" for the live subject; now TWO — the stopped run, then T-01's second run after the quiet window; spec wall time `00:05:58` a dated sample, not a bound.
- §11: the carve-out also names the live `sr` leg's in-session 170 s quiet window (the same Pulse dedupe class, a guard throwing if the run settles before its hold); the ban on sleeping to synchronise unchanged.
**Why:** the regrade drove every browse row and T-01; the live leg measured NVDA's log clock stepping ~2.5 s ahead mid-session, moving rows onto their successors' speech; the operator ruled the in-session quiet window the same class as the `--live` suite's and recorded the 357 ms limitation.
**Ref:** .andromeda/runs/2026-09-30T20-18-56-wrap/
