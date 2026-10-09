# Merge decisions — conductor-0.4.0 route draft, second pass

_Phase 3 of the second pass (after the operator's edit of the intent at the first Phase 4). One line per
suggestion: `{validator} {kind} · {applied|adjusted|rejected|deferred} · {reason / adjustment / tradeoff}`.
13 suggestions from four validators; the a11y validator returned none. 9 applied · 4 adjusted · 0 rejected ·
0 deferred. No suggestion installed, extended or sequenced work for a retired surface, so no rejection names a
retiring id. The first pass's record stands in `first/merge-decisions.md`; its accepted rewrites and splits were
carried into this pass's draft before validation, each re-checked against the revised intent, and the draft as it
stood before this merge is `.draft-before-merge.md`._

## security

security Rewrite `Local-model grading retired` (+ "model-text capture exception") · applied · The corpus-content ban's one ratified exception is scoped to the real-model capture; v4-19 closes it "with v4-02", and no line named the closure. "and" dropped twice to hold 25 words.
security Rewrite `Agent's answer taken in` ("handed out credential-free; answer returned by command") · applied · The handout is a new outbound artifact that the hygiene entry's absence list does not name; the agent connects to the door with its own admission (v4-23), so Conductor's door credential and engine token must not ride along.

## design

design Rewrite `Run read from the command line` (+ "non-verdict readings apart") · adjusted · Applied. The closed label set needs a place for outcomes that are not verdicts — the clock-sensitive verdict, the slow families' not-reported reading, the agent's answer — once the manual and degraded treatments leave. To fit both design rewrites in 25 words, "per-event verdict lines" became "verdict lines" and "report" left the line (the report is `Run record for the one form`'s).
design Rewrite `Run read from the command line` ("restated without window or hold") · adjusted · Applied with the fit above. The two `Surface: cli` sections lean on the window's token names and on a signature written around the window's titlebar count, so the restatement absorbs two losses. Per the operator's word this is the design master's own rule re-homed, not a requirement.

## tests

tests Insert `Door reads checked without an engine` (Epoch 1, after `Engine read through the door`) · applied · The CI tier of every read-back path today is the sidecar's stub, which Epoch 5 retires; under the revised v4-12 CI runs only what claims Conductor alone, so the door read path needs its engine-less home before Epoch 3 builds on it. Not a re-install: it is the stub tier's successor for a new read path.
tests Insert `Capability backing gate on the one form` (Epoch 4, before `Regression set driven against a live engine`) · applied · The standing backing gate reads the single-list catalog, which leaves in Epoch 5; nothing re-subjected it. It keeps "classified automatic" from standing in for "has a run", and makes the live drive's "every kept detection" a checked set.
tests Rewrite `Linux-only base CI` (+ "test, coverage-floor, zero-retry") · adjusted · Applied together with the obs rewrite of the same line: one kept-list naming test, coverage-floor, zero-retry, supply-chain, secret-scan, static, own-log conformance and unlogged-panic gates; "and the" trimmed to hold 25 words.
tests Rewrite `Short regression runs: quiet and lifecycle` (+ "with one bypass") · applied · A suppression-only run reads "nothing reported", the same as its event-less control; the bypass arm is what lets that proof discriminate. Surfaced at Phase 4: the same tension holds for every kept subject whose correct reaction is silence.
tests Rewrite `Living background` (+ "same seed same stream") · applied · The background is the one new stream source that carries noise; the determinism bar is stated for the schedule and now for it.
tests Rewrite `Disk bound over ten hours` ("checked without waiting them") · applied · A bound provable only by a real ten-hour run has no default-suite proof and would duplicate `Ten-hour run completed`.

## obs

obs Insert `Own-log gates on the one form` (end of Epoch 3) · applied · Checked in the workflow: CI's own-log conformance and unlogged-panic gates grade a log whose only producer is a single-list run forced to block through the shared-machine precondition. Epoch 5 removes that file, run path and precondition; the one form supplies the producer first. `(v4-12)` added to the line.
obs Reorder `Own log over a run of hours` before `Living background` · applied · The first hour-long runs are the background's graded healthy hour and the four families; the tick and the intact-log clause depend on neither. It now follows `Load bound re-measured`.
obs Rewrite `Linux-only base CI` (+ "own-log conformance and unlogged-panic gates") · adjusted · Applied through the merged kept-list recorded under tests. Checked: both gates sit in the `rust` job the move replaces.

## a11y

a11y — · no suggestions · Unchanged from the first pass: the plan's only assertable surface leaves in Epoch 1. One note for a wrap, not the route: code that outlives the window cites the plan's state-naming crosswalk until Epochs 3 and 5.
