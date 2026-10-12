# Curation — 2026-10-12-panel-shaped-types-retired (wrap P3)

Sources: this window's conversation + the report's *Decisions & corrections* (P1 wrote what curation needs there — the
operator's word at this resume). Default mode (auto-apply).

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "Setup's Bash guard does not refuse a `cd` chained behind `2>/dev/null || true;` …" (confidence 0.8)
                                              + testing.md: "`tokio::signal::ctrl_c()` installs its OS handler on its first POLL, not on the call …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "`cascade.py sweep --pattern` takes its ids after ONE flag" (confidence 0.8)
  Filters: 0 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "the guard passes a `cd` behind `2>/dev/null || true;`" · "`ctrl_c` registers on first poll" · "`--pattern` keeps only the last repeated flag"
  CLAUDE.md size: 137/200 · T1 6.1 KB, 0 over 600 B
```

## Applied
- **T2 host-linux.md** — the guard gap. Score: measured +0.4 · specific technical detail +0.2 · no other durable home +0.2
  (only the report and the friction log carry it) = 0.8.
  Proof: report.md *Decisions & corrections* → "Sweep hazards found this chunk", third bullet — at the 2026-10-12 P1 window a
  `cd DIR 2>/dev/null || true; …` compound ran past the PreToolUse guard and moved the session cwd into the chunk's
  `inputs/`; restored with one `cd` (friction record `2026-10-12T07:43:03Z-d`).
- **T2 testing.md** — `ctrl_c` first-poll registration. Score 0.4 + 0.2 + 0.2 = 0.8 (the fact is in a code comment and the
  report, neither a listed home; no master carries the mechanism — test-plan §1 records only the test's limits).
  Proof: report.md *Decisions & corrections* → "Testing mechanism (in-process SIGINT on a current-thread runtime)" —
  `install_stop_handler` spawns the listener then `yield_now().await`s once (`crates/conductor-cli/src/commands/suite.rs`
  79-99 @84); the unix test `the_first_interrupt_sets_the_stop_flag_and_the_process_lives` green on the Linux host.
- **T3 session-learnings.md** — the `--pattern` sweep hazard (+ the 16-char id bound). Score 0.4 + 0.2 + 0.2 = 0.8; a sweep
  hazard with no rule file covering the pipeline tools → Tier 3 (Filter 2's sweep-hazard keep).
  Proof: report.md *Decisions & corrections* → "Sweep hazards" bullets 1–2 — seven repeated `--pattern` flags ran one pattern
  and printed `total (1 pattern)`; a 17-char id exited 2 naming it.

## Filtered
- task-specific: the operator's roll-up correction (`1 scenario` / `N scenarios`) — product wording, now amended into
  layout-templates and design-system (P2), so the master owns it; the width-wording mismatch — amended into the masters
  on the operator's word (P2).

## Recurrences (→ handoff Deferred learnings)
- recurrence-despite-learning: `host-linux.md` §Paths ("`cd` only inside a subshell") — the P1 window's bare
  `cd DIR 2>/dev/null || true; …`.
- Not a recurrence: this window's append carrying a stray `cat >> /dev/null <<'X'` was refused by the guard and re-issued
  once as an Edit — the guard held `host-linux.md` §Transports (`guard-refused:1` on the reconcile step record).
