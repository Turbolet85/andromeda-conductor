# Curation — 2026-09-29-dual-license-mit-or-apache-2-0

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-win32.md: "TaskStop can leave a Monitor's `tail -F` child running with a dead parent — census by CommandLine, stop only your own watch" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): corrected in place — 2026-06-15 "cargo-deny over an unpublished workspace" (cap-exempt correction)
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold ("read a CI job's runner from its own `runs-on`, never a sibling job's" — 0.2: specific detail only, an in-pass slip caught by the author's own re-read)
  No-other-home: "read the local advisory-db copy's HEAD against FETCH_HEAD, never @{u}"; "TaskStop can orphan a Monitor's tail -F"
  Extended: T2/security.md: "2026-08-09: External supply-chain state DECAYS under a static dependency tree" + "HEAD vs FETCH_HEAD, never @{u}" (confidence 0.8)
  CLAUDE.md size: 137/200 · T1 47.3 KB, 9 over 600 B

## Proofs
- Correction (session-learnings 2026-06-15): with `deny.toml`'s `private` exemption removed and no `license` key,
  `cargo deny check licenses` exited 4 with nine `error[unlicensed]` (`chunks/2026-09-29-dual-license-mit-or-apache-2-0/evidence/deny-red-before.txt`);
  with `license.workspace = true` it prints `licenses ok` (implement gate log, this chunk). The entry's "private.ignore
  skips their license check" is therefore false of the current tree; its wildcard half stands.
  Proof: the red-before-green control above.
- Extended security.md (advisory-db currency): in the implement run, `git -C $CARGO_HOME/advisory-db rev-parse @{u}`
  = `20377f44` (committed 2026-05-01) while `HEAD` = `FETCH_HEAD` = `f23b7682`, `FETCH_HEAD` written 18:47Z during
  the gate run's `cargo audit`; `@{u}` is an ancestor of `HEAD`. A HEAD-vs-upstream reading would have called a
  current copy five months stale. Signals: measured +0.4 (it falsified the literal of the implement skill's
  currency clause) · specific detail +0.2 · no-other-home +0.2 (the fact is on no route annotation, master or
  playbook rule; the friction record is telemetry, not a home) = 0.8.
  Proof: the three git readings above, implement run `.andromeda/runs/2026-09-29T18-46-00-implement/`.
- host-win32.md (TaskStop orphan): after `TaskStop` on the gate-fence Monitor, `Get-Process tail` listed PID 33456
  (created 20:48 local, CommandLine = the Monitor's `tail -n +1 -F` on this session's task output) with a
  ParentProcessId whose process no longer existed; a second `tail -F` (PID 36352, 19:14 local) was another
  project's session watching its own scratchpad. 33456 was stopped by PID and re-probed gone; 36352 was left.
  Signals: measured +0.4 · specific detail +0.2 · no-other-home +0.2 = 0.8.
  Proof: the census in this conversation, recorded in `report.md` → Process hygiene.
