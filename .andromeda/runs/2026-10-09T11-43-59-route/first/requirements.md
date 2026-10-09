# Conductor 0.4.0 — requirements

_Capabilities this version adds or retires. Ids continue this project's own version-scoped scheme (`v2-01…v2-32`,
`v3-01…v3-11`): `v4-01` onward. The `P-NNN` ids these files cite are Pulse's and are never minted into. Derived
from `conductor-0.4.0/intent.md`, one numbered capability per EXPECT, every removal included; each line carries
that finding's EXPECT, and the matching OBSERVED clause travels into `verification-matrix.json` as
`observed_gap`. The intent's EXPECT lines are the authority; a count in an OBSERVED line is not (§Notes)._

## Theme 0 — What leaves

- v4-01 · Conductor's own window is gone — no crate depends on the window toolkit; no type, limit, fixture or sink exists for a panel; no gate boots a window or audits one; every function Conductor has is reachable from the command line; the masters say in plain words that Conductor has no interface (per intent §5 R1)
- v4-02 · Everything that grades Pulse's local model is gone — no scenario, contract, launch term, posture, verdict, report state, test or script names a model of Pulse's, real or canned; none of it is run or extended; the records of what was measured stay as history, marked closed with the model they graded (per intent §5 R2)
- v4-03 · Everything that grades Pulse's window is gone — Conductor grades nothing a human must look at: no hold, no checklist, no manual state or lamp, no drive-and-observe class, no contract or harvest about a window's timing, no scenario whose subject is a window; a run needs no person from its first line to its last (per intent §5 R3)
- v4-04 · Everything that assumes a shared machine is gone — nothing in Conductor reads a file of the engine's, starts a process of the engine's, binds a port of the engine's or requires a directory, a key or an environment in common with it; the address and the credentials of the engine are given to a run; no scenario, fault, contract term, precondition or suite has a shared machine as its subject; in development both programs run on one machine through the same network path as on two (per intent §5 R4)
- v4-05 · The short-run time model is gone — time is judged per event on the run's one timeline, how long after an event's onset the engine reported it; no tier, no per-scenario latency and no exception list for one remains; the load a run may put on the engine is re-measured against the engine as it is now, with its store on disk, and bounds a run of hours (per intent §5 R5; the per-event judgement is v4-20's)
- v4-06 · The capability set follows Pulse's current record — the accepted set, its classification and its coverage record are derived from Pulse's re-based capability record and name no capability Pulse has retired; every run and every event in a schedule names the capability of the engine it exercises; the pinned references are Pulse's current ones; where Pulse's record is not yet written, Conductor's stays unbuilt rather than guessed (per intent §5 R6)
- v4-07 · The other operating systems are gone — Conductor builds, tests and runs on Linux alone and holds no script or CI leg for another system (per intent §5 R7)
- v4-08 · The records say what Conductor is — every record a newcomer or an agent reads first describes a console harness that drives an engine on another host over a network with credentials it holds; no rule instructs what this version removes or forbids what it adds (per intent §5 R8)

## Theme 1 — Console, network, door

- v4-09 · A program without a window — Conductor is a console program; a run is started, followed and read from the command line (per intent §5 F1)
- v4-10 · The stream goes over a network — Conductor sends its stream to an engine on another host, with a token over an encrypted channel; the same path is used when both run on one machine; it never sends a token to another host over a plain channel, and it refuses an engine whose identity it cannot verify (per intent §5 F2)
- v4-11 · Conductor holds credentials now — the engine's token and the door's credential are given at run time and are never written to a run description, a run record, a journal, a report or a log; a run against an engine it cannot authenticate to is reported as blocked, not as a failed check; where a run reads its credentials from is stated, and it is never the repository; Conductor holds no credential of a model's; the security master rates Conductor for what it now is (per intent §5 F2b — the overseer's, PROVISIONAL until the founder's own word)
- v4-12 · The reaction is read through the door an agent uses — Conductor reads the engine's reaction through the door the investigating agent uses, and the engine's one-line status as the panel module does, so every run also proves the door and the network receiver; the gate before a run proves through that path that the engine is reachable, that it admits Conductor, and that a canary event sent now comes back as a finding; the one write the door keeps is used where a run needs it and named as a write; a run states the engine state it starts from, and what the engine already held is told apart from what the run raised; every claim about the engine is made against the engine, and a check that runs without one claims only Conductor's own behaviour and says so (per intent §5 F3)

## Theme 2 — A small world

- v4-13 · Several services that call each other — a world of three to five named services with call links between them, each carrying a version; a fault can be placed in one service and be felt in another (per intent §5 F4)
- v4-14 · A living background — a traffic curve with a compressed "day", ordinary noise, harmless errors at a low rate, so that silence from the watcher on a healthy hour means something (per intent §5 F5)
- v4-15 · Events on a schedule, several at once — one timeline for the whole run; events are scheduled on it and may overlap; the first long run holds four families: a silent death, a slow creep, periodic spikes, and a break after a change, the last as a new version of a service that starts failing (per intent §5 F6)
- v4-16 · One way to describe a run — a run is a world and a schedule of declared events, and that is the only form; each detection the engine still claims has a SHORT run in that form as its regression proof, its stimulus one scheduled event, its truth declared, its reaction read through the door; the three unclear subjects follow Pulse's re-based record (v4-06): what the engine still claims and Conductor can exercise from another host gets a run, what needs a hand on the engine's host leaves; the single-list scenario form and every file in it are gone (per intent §5 F6b)

## Theme 3 — Declared truth and grading by time

- v4-17 · The schedule says what is true — every scheduled event declares what it is, in which service, from which minute, and what a correct reaction names; a healthy stretch declares that nothing should be reported (per intent §5 F7)
- v4-18 · A proof discriminates — for every regression run, the same world without its event is graded "nothing reported"; an assertion that would pass without the stimulus is refused when the run is described, not recorded afterwards (per intent §5 F7b)
- v4-19 · The reaction is logged against the timeline — a run keeps, per reaction, when the engine spoke and the structured facts it named (service, version, kind of finding, error identity, evidence identifiers); the content boundary is restated for a world whose facts are Conductor's own names coming back, and the exception written for the model's text is closed with v4-02 (per intent §5 F8)
- v4-20 · Grading lays the two timelines together — for a whole run: which planted events were caught, how many minutes after their onset, how many reports came during healthy stretches per hour, and whether each report named the declared service and cause; the two timelines are laid together on one stated time base; the difference between the two hosts' clocks is measured, and a verdict it could change is reported as such; the verdicts that remain are the ones a machine can give (per intent §5 F9)

## Theme 4 — The long run

- v4-21 · Hours, on a processor, at any hour — a run of ten hours or more against the console engine, with no GPU; it also measures the engine's memory against its 1 GB target and the growth of its database file; the run's own record and journal are written as the run goes and stay bounded in memory; a run interrupted at any hour leaves a readable record of what happened until then (per intent §5 F10; §7 point 2)
- v4-22 · A recorded run can be played again — a run's emitted stream can be replayed to the engine faster than it was recorded and give the same reactions, so a change to the engine is measured in minutes (per intent §5 F11)
- v4-23 · The end-to-end scenario of the pair — Conductor rolls out a "bad version" of one service in its world, to an engine on another host; it reads that the status line and the report name the service, the version and the new error; a large model, given the report and the door, names the planted cause, and Conductor records that answer against the declared truth; the large model is an agent the developer already runs, started by Conductor with the report, with no model credential of Conductor's own and no model call by the engine; its answer is a reading recorded over planted faults, never a gate that can fail a build (per intent §5 F12; §7 point 1 — who the large model is: the overseer's, PROVISIONAL until the founder's own word)

## The close

- v4-24 · No survivor — at the close the tree is asked again what stands that exists only for a window (Conductor's or Pulse's), for Pulse's local model, for a shared machine, for another operating system or for a human in the loop, and the answer is nothing (per intent §7 point 4; §4)

## Notes — the definition of done, and where each point is owned

- Intent §7 point 1 → v4-23. Point 2 → v4-21 (the graded timeline is v4-20's). Point 3 → v4-16 with v4-18. Point 4 → v4-24.
- Intent §6 «Out of this version» is carried by v4-15 and v4-20 as a stated reading, never a capability: the slow families are emitted, and "not reported" is the measured starting point.

## Notes — OBSERVED figures re-read at HEAD `97dea7f` (2026-10-09, this route's Phase A)

The EXPECT lines are unaffected by a count. Readings that MATCH the intent, each counted here: 59 tracked files in
`conductor-tauri`; three CI jobs (`rust`, `frontend`, `a11y`); three window scripts under `scripts/`; the posture
contract at 936 lines and the measurement contract at 302; `L4Posture` in 13 Rust source files, `CalibrationRegion`
in 22, `KnownResidual` in 12, `ManualCheck` in 24, `SloTier` in 24; the delegated-timing harvest at five files and
1241 lines; `pause.rs` in the core and the command line at 259 + 247 lines; 37 scenario files, two of which carry a
read-back assertion; twelve `over_tier` rows and two `live_assertion` rows in the audit ledger; the load envelope's
two ten-minute bounds; 82 `P-` ids in the capability manifest at `sut_version = "v0.3.0"` and the `P-083` rejection
test; four files under `.andromeda/refs/`, the capability spec at 852 lines; `agent-run.ps1` at 456 lines; the fixed
address at `conductor-emit/src/client.rs:23`; each record quoted under R8.

Readings that DIFFER, or that this run could not reproduce as stated:

1. R1, «about 7000 lines, 3200 of them accessibility tests»: the crate's tracked text files (images, fonts and lock
   files left out) hold 7650 lines; the eleven tracked files under an `a11y` path hold 3612.
2. R1, «thirteen master and leaf files about design, layout and accessibility»: not reproduced as a count. Found:
   three masters (`design-system.md`, `layout-templates.md`, `a11y-plan.md`), the a11y registry with nine key files,
   two summaries under `.claude/docs/` and two rules under `.claude/rules/` (`a11y.md`, `frontend.md`). Which
   thirteen the audit counted is not stated.
3. R2, «eighteen test files (about 4900 lines)»: seventeen files named for the real model under
   `conductor-run/tests/` hold 4747 lines; with the shared `capture_paths` helper, eighteen files and 4792.
4. R3, the operator hold: a third `pause.rs` stands in `conductor-tauri` (274 lines, it leaves with v4-01), and two
   test files serve the hold beside the two counted sources (`operator_pause.rs`, 74 lines;
   `operator_pause_harvest.rs`, 365).
5. R3, «sixteen capabilities are classified drive and observe»: not re-measured to a row count — a text match over
   the classification source could not tell rows from other uses of the name.
6. R4, «twelve test files (about 3800 lines)»: the harvest and live test files left after those the model (R2) and
   the window (R3) own are twelve, and they hold 4141 lines. A wider match — every test file that names the shared
   data directory — gives seventeen files and 6085 lines.
7. R4, «the live suite launches three legs … two of them about the window and the model»: the run script fires four
   legs over three scenarios (`halo-hue-encoding`; `degraded-mode-report` twice; `auto-resolve-idle-window`), then
   a driven accessibility arm; the real-model leg is a separate selector.
8. R7, «the `rust` CI job runs on a Windows runner»: all three jobs do (`rust` and `frontend` on `windows-latest`,
   `a11y` on `windows-2022`). PowerShell also stands outside `agent-run.ps1`: the three window scripts, and the
   key-send and window-activation scripts inside `conductor-tauri`.
9. F6b, the subject split 7 / 8 / 5 / 14 / 3: reproduced by name, with one assignment inferred by elimination —
   the fifth shared-machine scenario (the workspace-key reconciliation) is read as
   `constellation-severity-live-wiring`. The three unclear are `ack-cooldown`, `cross-incident-recurrence` and
   `threshold-hot-reload`.
10. `[[checklist]]` is declared by two scenario files; the `[MANUAL]` lamp is named in five Rust source files.
    The intent gives no count for either.

## Notes — residuals intake (three sources; dispositions written to `.andromeda/residuals.md` at Phase 6 only)

Source 1, `.andromeda/residuals.md`: three `open` lines (17, 19, 21), each premise re-read. Source 2, the 0.3.0
matrix: no `deferred` entry (11 of 11 `verified`) — a noted no-op. Source 3, the 0.3.0 requirements' carried
section: three items.

| # | Item | Premise re-read | Disposition | Standing |
|---|---|---|---|---|
| 1 | `residuals.md:17` — the founder's direction for the next version | holds; the evidence file exists | absorbed:v4-13+v4-14+v4-15+v4-16+v4-17+v4-18+v4-19+v4-20+v4-21+v4-22+v4-23 (intent §8: Themes 2–4) | decided by the intent |
| 2 | `residuals.md:19` — the envelope types `verdict` with no null arm | holds: the field is an optional verdict in the run record; the sample at `obs-plan.md:72` still lists three values | absorbed:v4-03+v4-20 (intent §8); v4-02 moves the same enumerations when the model's verdict leaves | decided by the intent |
| 3 | `residuals.md:21` — which webview legs the Linux host can run; the PowerShell harness's red path | holds in part: the phrase «Windows dev host» stands three times in `test-plan.md` and three in `architecture.md`, and was not found in `design-system.md` or `a11y-plan.md` under that wording | provisional absorbed:v4-01+v4-07 (intent §8: «retired with R1»; the harness half is v4-07's) | **ASKED — the operator's word: absorbed as written, or dropped** |
| 4 | carried — `incident_events` through MCP | **falsified**: the lifecycle-events tool is a pinned required tool (`contracts/mcp-contract.toml`), delivered inside 0.3.0 | provisional re-carried (the letter's default); the reading of lifecycle events through the door is v4-12's | **ASKED — dropped (delivered in 0.3.0), or absorbed:v4-12** |
| 5 | carried — the unbuilt footer status strip and report-site checklist render | holds; both are surfaces of the window | provisional re-carried; subject retired by v4-01 (the checklist half by v4-03) | **ASKED — dropped (subject retired)?** |
| 6 | carried — control-panel-launched parity, Critical Path 7's owed half | holds; a window-launched run | provisional re-carried; subject retired by v4-01 | **ASKED — dropped (subject retired)?** |

`residuals.md:17` also lists four proposals that are not the founder's. Three have a finding here (expected answers
per event → v4-17; structured facts of a reaction → v4-19; a sanctioned capture path → v4-19's restated content
boundary). The fourth — the unmeasured live print of the corrected classifier — is about the real-model capture,
which v4-02 retires.

## Carried residuals (NOT 0.4.0 capabilities — deliberately unnumbered, no matrix entry)

Each of the three below is PROVISIONAL and waits for the operator's word at the Phase 4 review: its subject is
either delivered or retired by a numbered capability above, and the letter drops a residual only on an explicit
say-so. They stay visible here so that none is lost in silence.

- **`incident_events` through MCP.** Carried from 0.3.0 as an external repo's gap. Its premise is no longer true:
  the tool that reads those events is a pinned required tool since 2026-10-03. What remains of its subject —
  reading an incident's lifecycle from the engine — moves to the door with v4-12. Stays here until the operator
  says dropped or absorbed.
- **The unbuilt footer status strip and the report-site operator-checklist render.** Both are surfaces of
  Conductor's window, which v4-01 retires; the checklist itself leaves with v4-03. Stays here until the operator
  says dropped.
- **Control-panel-launched parity (Critical Path 7's owed half).** A run launched from the window, which v4-01
  retires. Stays here until the operator says dropped.
