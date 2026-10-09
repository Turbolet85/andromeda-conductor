# Session Handoff

**Last Updated:** 2026-10-09T13:26Z
**Branch:** `build/conductor-0.4.0` · 0 ahead of `origin/build/conductor-0.4.0` as read at this wrap's Setup
**Status:** clean
**Last Commit:** no-marker — operator-requested adaptation, 0-pending wrap; beneath it, the 0.4.0 capability ids put
into the project's declaring form

## Position
- **Done:** 0.3.0 is closed on record (11 of 11 verified). The 0.4.0 route stands: 24 requirements `v4-01`…`v4-24`,
  61 entries in 8 epochs, the matrix at 24 `planned`. No chunk wrapped, no phase started.
- **Next:** the route's first entry, `Conductor's window retired` (`conductor-0.4.0/working-route.md:11`). The first
  chunk waits for the founder's word.
- **Not done at the 0.3.0 close, each still waiting for his word:** no merge into `main`, no tag, no release, no
  publishing version bump, no branch deletion.

## Work done
- `conductor-0.4.0/requirements.md`: the 24 capability lines read `- **v4-NN** · …`, the form this project's ledger
  gate reads. Id markup only; a commit of its own, outside the wrap.
- `conductor-0.4.0/working-route.md:15`: a `CARRY:` on `Linux-only base CI`, the ledger gate's own repair, no
  requirement id.
- The record: `.andromeda/runs/2026-10-09T13-23-15-wrap/adaptation-record.md`, with the relay beside it.

## Drift resolved
None. This path runs no report and no fan-out; no master changed.

## Open residuals
None in `.andromeda/residuals.md`.

## Notes
- **Last failed command:** none open.
- **The red on `d227e59`, and its owners.** Push run `37932450560` failed in the Rust gate on one test,
  `conductor-report::matrix_ledger_gate every_requirement_capability_has_exactly_one_matrix_entry`: the route wrote
  the capability ids bare, the form its contract allows, and this project's gate reads the bolded form only. Repaired
  by the commit `fix(route): conductor 0.4.0 capability ids take the project's declaring form — id markup only`,
  directly beneath this wrap's commit; the gate reads 4 of 4 green on the dev host. The CI verdict of the push that
  carries it was not measured when this was written. The first `/andromeda-phase` reads every commit from `355a678`
  through `HEAD` and will meet `d227e59` red: it is this one, repaired, and the gate's narrowness is owned by the
  `CARRY:` above.
- **An exception to the 2026-08-10 rule, on a delegate word.** `requirements.md` is immutable; this one edit was made
  on the word of the pc overseer as operator, founder-delegated, not the founder's own. His later word supersedes it.
  Reason and byte proof are in the adaptation record.
- **The next route run writes bare ids again** (the route letter's template). Until the `CARRY:` is built, a new
  version's `requirements.md` turns this gate red the same way.
- **Two lines owed on Pulse's side, the operator's to carry:** the door offering the engine's memory and database
  size, and the notification record. The entries `Engine memory and database growth read through the door` and
  `Notification record read through the door` are not built before Pulse's line exists.
- **`Accepted capability set re-based` waits for Pulse's `Capability record re-based`.** It is built beside the
  0.3.0 pin, which leaves with the single-list form.
- **Provisional in the intent until the founder's own word:** F2b (`v4-11`), and who the large model is in F12
  (`v4-23`).
- **Phase 5 of the route, one check carried on the operator's acceptance:** the two-host path's reachability entry
  sits in Epoch 6, not Foundation. The reason is in `.andromeda/runs/2026-10-09T11-43-59-route/final-validation.md`.
- **Outside the tree:** the GitHub default branch was `build/conductor-0.3.0` as the 2026-10-08 relay reported. Not
  re-measured here. No pull request was opened for 0.4.0.
- **Data dirs:** `~/.cache/pulse-legs/` was not touched. No Pulse was launched.
- **Host:** Linux dev host (Omarchy); `grep` is ugrep; `pwsh` 7.6.6 is installed. Run the harness as
  `bash scripts/agent-run.sh`.
- **Deferred learnings (carried, none new):**
  - recurrence-despite-learning: CLAUDE.md 2026-08-09 (a claim about a system is a hypothesis until measured) —
    a host claim, "no `pwsh`", was copied from the plan into committed evidence before it was measured; corrected
    before the commit.
  - recurrence-despite-learning: CLAUDE.md 2026-08-09 (an "every" over a family) — a site list for one claim was
    taken over three masters instead of seven and under-ran by two; the detectors' sweeps caught it.
  - recurrence-despite-learning: `host-linux.md` §Transports — a document append through a shell heredoc, refused
    by the PreToolUse guard at the 2026-10-08 sixth-series wrap.
  - recurrence-despite-learning: `host-linux.md` §Paths — a `cd` into the sibling Pulse repo, refused by the guard
    at the 2026-10-07 wrap.
