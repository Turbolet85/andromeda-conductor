# Session Handoff

**Last Updated:** 2026-10-07T15:32Z
**Branch:** `build/conductor-0.3.0` · 0 ahead of `origin/build/conductor-0.3.0` as read at this wrap's Setup (HEAD
`717bbf3`, the setup upgrade commit, pushed); this wrap's commit lands on top and is pushed.
**Status:** clean
**Last Commit:** 0-pending correction wrap (session 185) — no chunk wrapped.

## Position
- **Done:** `2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive` (wrapped at session 184;
  unchanged). 162 master records, all `complete`.
- **Next, `### Epoch 5b — Version close`, in order — the route's order and entries are unchanged:**
  1. `working-route.md:98` a sixth `v3-09` series — BLOCKED-ON Pulse "The L4 probe reproduces the canary-history
     miss"; re-read at this wrap: Pulse's record still `pending`, Pulse HEAD `48714f0`. Standing.
  2. `:100` "Version close on measured evidence" — now carries one `CARRY:` (below).

## Work done
- A correction pass after setup commit `717bbf3` (the host leaf is `host-linux.md`), on the directive in this
  wrap's arguments. No chunk, no report, no fan-out.
- **The 2026-08-22 Tier-1 bullet** was re-tiered as the other eight were: 5078 B to a 482 B lead, its full text
  moved by bytes to `.claude/docs/session-learnings.md` ("Tier-1 entry of 2026-08-22, moved whole"). Its opening
  claim is corrected in both places: the dev host is Linux; CI runs on Windows images. `health.py` check 1 now
  reads 0 of 17 bullets over 600 B, T1 6.1 KB.
- **`testing.md:91` and `verification-harness.md:66`** no longer cite the retired leaf: each citation now names
  where its lesson stands, with a `[corrected 2026-10-07 …]` tag saying what it used to cite.
- **`working-route.md:100`** gained a `CARRY:` for the two product-script comments that still name the retired
  leaf (`scripts/agent-run.ps1:230`, `scripts/a11y-token-witness.ps1:160`). Both files are untouched.
- Record: `.andromeda/runs/2026-10-07T15-28-23-wrap/adaptation-record.md`.

## Drift resolved
- None run: this path has no report and no detectors.
- **Noticed, not touched:** the masters and their leaves still describe the webview legs' home as "the Windows
  dev host" (architecture 4 hits, test-plan 4, the registries 4; leaves `a11y.md:39`, `frontend.md:50`,
  `a11y-summary.md`, `commands.md:30`, `stack.md:39`, `tests-summary.md:46`). These are master claims about
  where a leg was measured. A 0-pending wrap does not amend a master on drift it noticed, and the directive
  did not name them. They wait for a chunk wrap's reconcile, or the operator's word.

## Notes
- **Last failed command:** none open.
- **Open question for the founder, before the sixth series is pre-registered** (a `CARRY:` on `:98`, carried):
  d2's first hypothesis quotes `scope_id=conductor` and still places the storm in `conductor-canary`; the rank-1
  rule grades it `Identified`. Should the rule reject that case?
- **Founder ruling 2026-10-07 11:49** (carried): reproduce, then fix, then a sixth series.
- **Surfaced, no owner yet (carried):** a11y-plan's reproduced envelope schema types `verdict` with no null arm,
  while every real-model envelope reads `verdict` null.
- **Unmeasured:** which legs the Linux host can run. The corrected 2026-08-22 entry says so and points at its own
  clause (2): check for a driver before inheriting a platform verdict.
- **One clause of the retired leaf has no home in `host-linux.md`:** "a PowerShell redirect writes UTF-16/BOM".
  It is stated in `verification-harness.md`'s 2026-09-16 surviving-grandchild entry; the retired body is in git
  history (`717bbf3^:.claude/rules/host-win32.md`).
- **Architecture registries (carried, unchanged):** §Occupied Resources 38114 B of 38115 B — the `CARRY:` on `:98`.
- **The data dir `rm-recorded-run`** (carried) stays under the cache dir's `pulse-legs`.
- **Host:** Linux dev host (Omarchy); `grep` is ugrep. Run the harness as `bash scripts/agent-run.sh`.
- **Curation this wrap:** three corrections, no new learning — T1 1 (the 2026-08-22 bullet, re-tiered and
  corrected) · T3 1 (its full text) · T2 2 (the two citations). Log: the run dir's `curation.md`.
- **Deferred learnings:**
  - recurrence-despite-learning (carried): CLAUDE.md 2026-08-21 (verify the artifact, not the exit code).
  - recurrence-despite-learning (carried, re-named): `host-linux.md` §Transports — a `cat` heredoc append to a
    committed document was attempted at the prior chunk's implement; the PreToolUse guard refused it.
  - recurrence-despite-learning: `host-linux.md` §Paths — a `cd` into the sibling Pulse repo was attempted at
    this wrap's premise re-read; the PreToolUse guard refused it, and the read ran on absolute paths.
