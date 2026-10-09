# Merge decisions — conductor-0.4.0 route draft

_Phase 3. One line per suggestion: `{validator} {kind} · {applied|adjusted|rejected|deferred} · {reason / adjustment / tradeoff}`.
19 suggestions from four validators; the a11y validator returned none. 9 applied · 9 adjusted · 0 rejected ·
1 deferred, and one clause of an adjusted insert deferred as a requirement the intent does not state. No suggestion
installed, extended or sequenced work for a retired surface, so no rejection names a retiring id. Every adjustment
below is a fit to the 25-word bound or a narrowing; none adds a capability._

## security

security Insert `Agent start boundary` (Epoch 8, before `Large-model reading recorded`) · adjusted · Inserted with "started as a fixed program" re-worded "started as a governed spawn": intent F12 has the DEVELOPER name the agent, while security-plan §Security Anti-Patterns → Code Patterns forbids an operator-chosen program for the sidecar and says a ninth spawn form escalates — which form admits this spawn is the escalation the plan demands, not the route's to fix. Surfaced at Phase 4.
security Reorder `Credential hygiene` before `Two-host path reachable` · applied · The two-host path is the first entry to hold a token and it writes a recorded verdict; the read source and the absence-from-artifacts boundary precede it (security-plan §Bootstrap phases, §Security Anti-Patterns → Secrets). Same direction as the obs reorder — applied once.
security Rewrite `Conductor's window retired` (+ "Tauri-tree audit exceptions") · adjusted · Applied; "webview harness arms" and "fetched-driver dependency class" shortened to hold the line at 25 words.
security Rewrite `Linux-only base CI and harness` (+ "keeping supply-chain and secret-scan gates") · adjusted · Applied together with the tests rewrite of the same line; both terms do not fit one 25-word line, so the entry is split in two inside Foundation: `Linux-only base CI` (the gates kept, what leaves) and `Fast feedback on Linux`. Checked: the locked build, the secret-scan gate and both supply-chain steps sit in the `rust` job the move replaces.
security Rewrite `Security posture restated…` (+ "the loaded security rule") · applied · The always-loaded rule says "no network service, no secrets"; two epochs of credential work would otherwise run under a rule forbidding it. `Records restated` (Epoch 8) keeps the other first-read records.
security Rewrite `Two-host path reachable` ("encrypted, engine-verified channel") · applied · v4-10 refuses an unverifiable engine; a token must not cross to another host once before that holds, the reachability run included.
security Rewrite `Credential hygiene` (absent from "arguments … errors") · applied · Merged with the design rewrite of the same line; "read from a stated place" shortened to "source stated" to fit.
security Rewrite `Credential hygiene` ("secret-scan gate names their shapes" replaces the blocked arm) · applied · The gate is a self-kept rule set and catches a credential class only once its shape is named (security-plan §Secret Management → Secret-scan gate shape).
security Rewrite `Channel refusals` (takes the blocked arm) · adjusted · Applied; the line then exceeded 25 words, so "fixed local address leaves; same path on one machine" became its own removal entry, `Fixed local address retired` (v4-04, v4-10), directly after.

## design

design Insert `Console design restated for the command line alone` (Epoch 3) · adjusted · Not inserted as an entry of its own: folded into `Run read from the command line`, which now carries "closed text-label set; console design restated without the hold". The surface is the one the version keeps, so nothing here serves a retired surface. Tradeoff surfaced at Phase 4: the validator wanted the restatement as a separate entry ahead of its consumer, and what the console's signature is once the hold leaves is a design decision no requirement states.
design Rewrite `Old verdict vocabulary retired` (+ "recessive non-lamp tier stays") · adjusted · Applied; the line then exceeded 25 words, so the entry is split by requirement: `Model and manual vocabulary retired` (v4-02, v4-03) carries the stays-clause, `Short-run tiers retired` (v4-05) carries the tiers.
design Rewrite `Credential hygiene` (+ "console output") · applied · Merged with the security rewrite of the same line. Checked: design-system's universal ban on a styled credential surface exists and is worded for a Conductor that owns none.

## tests

tests Insert `Harness verbs on the one form` (end of Epoch 3) · applied · Re-subjects the five-command harness to the door's gate and the one form's record before Epoch 4's runs must be harness-runnable. Not a re-install: master-route holds `2026-06-23-5-command-agent-run-harness` complete.
tests Reorder `Load bound re-measured` before `Living background` · deferred · Unclear dependency, left in place. For: a run past the standing ten-minute bound is no evidence about the engine, so the first graded healthy hour wants the new bound. Against: v4-05 measures the load "with its store on disk", and Pulse's route puts its store on disk in its Epoch 6, after the long-run memory entry of its Epoch 5 — so the bound may not be measurable before the first long run. The operator's call at Phase 4.
tests Reorder `Spawned sidecar and shared-machine gate retired` before `Old verdict vocabulary retired` · applied · Checked: the old run path's gate reaches the sidecar client, and the model and window scenarios run through it; the gate now leaves after its last users, directly after `Single-list scenario form retired`. The sidecar is still replaced, never adapted.
tests Rewrite `Linux-only base CI and harness` (+ "every leg run locally") · adjusted · Applied through the split recorded under security.
tests Rewrite `Engine-backed check pipe reachable` ("CI-or-local leg stated") · adjusted · Applied as "CI-or-local home stated", keeping the draft's clause that engine-less checks claim Conductor alone. Where the engine-backed gate runs is not stated by any requirement — surfaced at Phase 4.

## obs

obs Insert `Own log over a run of hours` (Epoch 7, after `Run written as it goes`) · adjusted · Inserted WITHOUT "size bounded". The tick and the second-command clause rest on v4-09 (a run is followed from the command line) and v4-21; checked — the log sink opens truncating on every command. A size bound on Conductor's own log (and on its journal's disk growth) is a control no requirement states: v4-21 bounds memory only. Raised at Phase 4 as a requirement the intent is missing, not carried on a route line.
obs Reorder `Credential hygiene` before `Two-host path reachable` · applied · Same direction as the security reorder — applied once, both reasons logged.

## a11y

a11y — · no suggestions · The plan's only assertable surface leaves in Epoch 1; none of its seven bootstrap items is owed. One note for a wrap, not the route: a state-naming crosswalk in the plan is cited by code that outlives the window until Epochs 3 and 5.

## orchestrator — one correction made after the merge, no validator behind it

orchestrator Rewrite `Accepted capability set re-based` and `Single-list scenario form retired` · applied · The re-base sits in Epoch 4, while the 37 single-list files stand until Epoch 5 and may not load without an id from the accepted set. Re-basing the one pinned set in place would have forced edits to scenario files that are about to leave — the adapting the directive names as the defect. The re-based set is now built beside the 0.3.0 pin ("old pin stands meanwhile"), and the pin leaves with the form (`Single-list scenario form retired` gains "the 0.3.0 capability pin" and `v4-06`). Shown at Phase 4.
