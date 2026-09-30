# Curation — 2026-09-30-the-screen-reader-content-findings-fixed

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + a11y.md: "NVDA's browse caret follows the last DOM focus … walked FORWARD from a focused control just before its node" (confidence 0.9)
                                              + a11y.md: "Browse mode never reads an aria-label on a role-less element … visually-hidden text" (confidence 0.8)
                                              + a11y.md: "An NVDA started while another window holds the foreground can stay bound to it … void, never graded" (confidence 0.9)
  Corrections (cap-exempt):                   verification-harness.md 2026-09-02 entry, clause "an action timeline stamped per row lets a parser assign utterances by time window" [corrected 2026-09-30]
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup (case-insensitive stop tokens satisfied by incidental text — CLAUDE.md T1 2026-08-21 "ask what OTHER content the pattern admits") · 2 task-specific (CONDUCTOR_NVDA unset in the session; the pulse-app launch leaf names) · 0 conflict · 0 deferred
  No-other-home: "Browse mode never reads an aria-label on a role-less element" (no master states it: grep `Scenario count|Scenarios completed|aria-label` over the seven for the count — 0 hits for the count's naming)
  CLAUDE.md size: 137/200 · T1 47.5 KB, 9 over 600 B

## Proofs
- **browse caret / bind:** E0-01 from BODY after the injected reset cycle walked the report table (`row 1 column 1 P-ID …`, sr-empty re-fire 19:18Z); re-sited to Close window it reached "No scenarios found." in 2 keys (19:23Z). S0-11/S0-12 walked before S0-01 heard nothing on seven ArrowDowns (live leg #2) — S0-01 then heard "Conductor document"; walked after S0-02 from the held target they heard "heading level 1 Conductor · idle" / "Scenario count: no run yet" (live leg #3). Record: `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-content-findings-fixed/evidence/operator-legs.md` slot history. Signals: measured +0.4, repeated (three rows, three sessions) +0.3, specific detail +0.2.
- **aria-label in browse mode:** S1-02 from the held target heard "blank / … heading level 1 Conductor · live / 0" with the count's name on an `aria-label` (live leg #2); with visually-hidden text it heard "Scenarios completed: 0" (live leg #3). Signals: measured +0.4, specific detail +0.2, no-other-home +0.2.
- **NVDA bind at start:** sr-empty slot 2 — NVDA's log `Foreground took too long to change. Foreground still … (CASCADIA_HOSTING_WINDOW_CLASS). Should be … (Tauri Window)`; every focus row silent, the terminal prompt heard on every key, while `send-keys.ps1` sent every key (its guard saw Conductor); the operator minimized every window and the re-fire bound. Signals: measured +0.4, operator "call that run void … do not re-fire it" (never-language) +0.3, specific detail +0.2. Re-homed from verification-harness `:59`'s theft clauses to a11y.md by tiebreaker 6 (the leg's files sit under `ui/**`, a11y.md's paths; verification-harness's paths do not cover them).
- **clock correction:** live leg #1 — 35 key pairs, keys 8–29 at ~2 870 ms against a ~390 ms baseline; `content info` headered 19:25:48.392Z already in the file at 19:25:46.713Z; the step-aware calibration moved S0-01…S1-04 onto their own speech in the re-parse control. Record: `evidence/nvda-pass.json` `subjects.*.clock`.
