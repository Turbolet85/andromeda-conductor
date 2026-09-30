# Curation — 2026-09-30-the-screen-reader-pass-grades-again-on-this-host

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "Read NVDA's `-l 12` log by its entry HEADERS, and snapshot it before the next leg of the same subject"
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup (the Bash guard's doubled-backslash block — host-win32.md 2026-09-23; the configuration-bound SR verdict — amended into a11y-plan §3 by this wrap's P2) · 1 task-specific (CONDUCTOR_NVDA supplied per command from the one portable copy) · 0 conflict · 0 deferred
  No-other-home: "Read NVDA's `-l 12` log by its entry HEADERS, and snapshot it before the next leg of the same subject"
  Extended: T2/verification-harness.md: "A screen reader is driven by the OS FOREGROUND …" (2026-09-02 entry, its 2026-09-30 parallel-session clause) + "decide a theft from NVDA's own log before grading"
  CLAUDE.md size: 137/200 · T1 47.3 KB, 9 over 600 B

## Proofs

- **Extended — theft decided from NVDA's own log** (score 0.9: "never grade a theft" +0.3 · verified by measurement +0.4 ·
  specific technical detail +0.2). Proof: both of this chunk's legs — `runs/sr-leg/nvda-speech.error.log` terminal-output
  notification entries (`NVDAObjects.UIA.winConsoleUIA`, "blocked to avoid double-report") run to 12:17:20.2Z, stop, and
  resume at 12:17:38.9Z after the app closed, with no entry of any kind in 12:17:24.1Z–12:17:38.77Z where the stamped
  R0-02..R0-04 Tabs fall; the same shape in the `= 2` leg (12:19:46.7Z–12:20:27.6Z). It turned the operator's slot
  condition ("a focus theft voids the run") into a check both legs passed (report §Decisions & corrections).
- **New — NVDA log headers + snapshot before the next leg** (score 0.8: verified by measurement +0.4 · specific technical
  detail +0.2 · reached no other durable home this wrap +0.2 — not on the route, not in a master, not a playbook rule, not
  a matrix note). Proof: (a) a `grep -n … | awk -F'[()]'` time extraction over the error log printed body text
  (`'activityId': 'TerminalTextOutput'`) instead of header times, re-done by a header-parsing script; (b) research's
  teardown coordinates `nvda-speech.empty.log:309-311` and `nvda-speech.error.log:351-355` no longer exist on disk after
  this chunk's two legs (`wdio.conf.ts:207`, `rmSync(logPath)` in `startNvda`) — `cause-control.md` §The IA2 basis cites
  them "as research read them".
- Rejected: `CONDUCTOR_NVDA` unset in the agent's shell (task-specific host state, one-off: 0.2 − 0.3);
  the doubled-backslash guard (duplicate of host-win32.md's 2026-09-23 transport correction, the same mechanism);
  the configuration-bound SR focus verdict (its home is a11y-plan §3, amended this wrap).
