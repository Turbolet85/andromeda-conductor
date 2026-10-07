# Session Handoff

**Last Updated:** 2026-10-07T13:34Z
**Branch:** `build/conductor-0.3.0` · 0 ahead of `origin/build/conductor-0.3.0` as read at this wrap's Setup (HEAD
`15fab57`, the operator pre-CI commit, pushed); this wrap's commit lands on top and is pushed.
**Status:** clean
**Last Commit:** `2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive` — wrap (session 184).

## Position
- **Done:** `2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive` — a capture run, no
  `v3-09` verdict. Three drives in one sitting at Pulse `f70be92`, prompt `v2.6`, the operator's recording
  pass-through as the model binary; d1, d2, d3 read `Identified` as observations. No drive missed, so the d3
  position stands at 2 misses of 3 live runs, a rate. CI#37625765074 green on `15fab57`.
- **Next, `### Epoch 5b — Version close`, in order:**
  1. `working-route.md:98` a sixth `v3-09` series — BLOCKED-ON Pulse "The L4 probe reproduces the canary-history
     miss"; re-read at this wrap: Pulse's record still `pending` at `48714f0`. Standing.
  2. `:100` "Version close on measured evidence".

## Work done
- The contract gained `## The 2026-10-07 capture run` (add-only, digest fixed before d1); the harvest gained the
  run's module, three pins and six tests, no verdict test; the ledger carries each drive's bracket and stamps.
- Operator pass on the overseer's word: hygiene clean, pre-CI commit `15fab57`, guarded push, CI green.

## Drift resolved
- 14 proposals from four detectors (architecture 2, security-plan 5, test-plan 5, obs-plan 2); 13 applied as 11
  body edits, 1 rejected, 0 escalations. Three leaves re-derived (`commands.md`, `tests-summary.md`,
  `security-summary.md`). Record: `.andromeda/runs/2026-10-07T13-18-48-wrap/fanout-results.md`.
- The "driven only as the pre-stated series" rule (architecture `:70`, test-plan §2, §6, §9) was KEPT, with the
  2026-10-07 capture run named as its one dated, verdict-less record. No class of capture runs was minted.

## Notes
- **Last failed command:** none open.
- **Open question for the founder, before the sixth series is pre-registered** (a `CARRY:` on `:98`): d2's first
  hypothesis quotes `scope_id=conductor` and still places the storm in `conductor-canary`; the rank-1 rule grades
  it `Identified`. Should the rule reject that case? No rule changed at this wrap (the overseer's directive).
- **The overseer's own reading of the operator's capture folder is in no committed file** — the boundary of this
  chunk. Input I10 holds the operator-pass directive as an excerpt for that reason.
- **Architecture registries after this wrap:** §Occupied Resources 38114 B, §Established Decisions 38068 B, of
  38115 B (`scripts/arch-registry-check.py measure --file .andromeda/architecture.md`). The posture-contract
  entry now names record kinds; the series dates live in the sidecar.
- **Advisory-db:** three pre-id-assignment leftovers were removed from the cargo home's copy at implement; the
  copy read clean and at the remote's head afterwards.
- **No raw fan-out twins** for the three empty detector returns: each verbatim return carried a home-rooted path
  (the prompts named documents by absolute path). Their stripped substance is in `fanout-results.md`.
- **The data dir `rm-recorded-run`** stays under the cache dir's `pulse-legs`, beside the two earlier ones.
- **Founder ruling 2026-10-07 11:49** (carried): reproduce, then fix, then a sixth series.
- **Surfaced, no owner yet (carried):** a11y-plan's reproduced envelope schema types `verdict` with no null arm,
  while every real-model envelope reads `verdict` null.
- **Left as directed (carried):** the 2026-08-22 Tier-1 bullet (over the cap) and `.claude/rules/host-win32.md` —
  the pipeline overseer's door (PC35). **Curation conflict (carried):** that entry opens "This host is
  Windows-only"; the dev host is Linux. The operator's to re-word.
- **Host:** Linux dev host (Omarchy); `grep` is ugrep. Run the harness as `bash scripts/agent-run.sh`.
- **Curation this wrap:** T2 1 (`verification-harness.md`: a drive's bracket ends before the capture's own
  read-back) · extended T2/`security.md` (a green audit reading needs the porcelain check too).
- **Deferred learnings:**
  - recurrence-despite-learning (carried): CLAUDE.md 2026-08-21 (verify the artifact, not the exit code).
  - recurrence-despite-learning: `host-win32.md` §Transports — a `cat` heredoc append to a committed document
    was attempted at implement; the PreToolUse guard refused it.
