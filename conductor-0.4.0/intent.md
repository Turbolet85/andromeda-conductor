# Conductor 0.4.0 — intent

**Purpose & how to use this file.** This is the INTENT for version 0.4.0 — what `/andromeda-route --version 0.4.0`
derives `vision.md`, `requirements.md` and `working-route.md` from. Every finding states what we OBSERVED and what
we EXPECT. It says WHAT, never HOW.

_Second assembly, 2026-10-09, by the pc overseer; the first was never handed to a route. An audit of the first
against Conductor's tree at `97dea7f` — "what of the old form would still stand, with no finding that removes it,
replaces it or gives it a new subject?" — found twelve such clusters. The founder's word on how this turn is done, the
same day: «без полумер … если что то не вписывается в нашу новую парадигму исправляем а не стараемся сгладить углы
просто». So this assembly names every survivor and gives it an owner. Sources: his rulings of 2026-10-08/09 (trail:
`~/dev/projects/additional/pc-overseer/shaping-0.4.0.md`), that audit, and Pulse's own intent for its 0.4.0
(`andromeda-pulse/andromeda-pulse-0.4.0/intent.md`), whose twin this is: the two versions advance in pairs. He reviews
the derived requirements and route at the route's Phase 4._

_Revised the same day, after Pulse's route was committed (`b3ac58a`): §6 now carries what the engine offers at each of
its checks, read from that route, because Conductor's order follows it; F12 says who the large model is in this
version; six controls of the new surfaces stand inside their findings (F2, F2b, F3, F9, F10) and one correction in §6
(the real stream never shares an engine with Conductor's world); the type counts in R2, R3 and R5 are restated on one
basis — Rust source files at `97dea7f`, counted by me. Every other OBSERVED figure is the audit's, of which I re-read
four rows: Phase A re-reads each at HEAD and records the measured form. The EXPECT lines are the authority._

_Revised once more at its route's first Phase 4 (run `2026-10-09T11-43-59-route`): the eight requirements that review
found missing are decided here, inside F2, F2b, F3, F10 and F12, and §6's table is corrected — the long run that
measures the engine belongs with the engine's store on disk (Pulse's Epoch 6), not with its Epoch 5. The 23 findings,
their order and their ids (`v4-01`…`v4-24`) are unchanged. At its second Phase 4 one more: F7b as first written
would have refused every proof whose correct reaction is silence; it now says what such a proof's control is._

---

## 1. Context

Conductor is the tool built to test Pulse. Its 0.3.0 closed on 2026-10-08 at `97dea7f` (11 of 11 verified): it emits
synthetic OTLP telemetry from 37 scenario files, reads Pulse's reaction through an MCP sidecar it spawns beside
itself, and grades it; some claims are graded by an operator looking at Pulse's window, one scenario grades Pulse's
local model. It is a Tauri desktop application beside a command line.

Pulse changes form in its 0.4.0 (the founder's rulings, 2026-10-09): a console engine on its own Linux node — no
window, no local model in the engine, no workspace detection, no operating-system credential store; telemetry reaches
it over a network behind a token; an investigating agent reaches it through a door inside the engine's process; an
incident is the engine's own record and its report is written from the engine's facts. Conductor has to follow, and
it gets the job the founder named on 2026-10-07 and widened on 2026-10-08:

> «чтоб оценить по номальному надо чтоб кондуктор умел спавнить продолжительную серию разных событий и логировать
> как на них реагирует модель, а потом логи анализировать»

> «сейчас … кондуктор выдает маленькие серии конкретных событий но это никак не похоже на работу какой то настоящей
> системы за которой надо следить, основное что я думаю что надо кондуктор выдавать длинные серии предположим на 10
> часов или даже больше, чтоб типо симулировать работу настоящего сервиса»

In short: stop emitting small series of single events; simulate a real service for hours, and record how the
watcher reacts.

## 2. The core problem (one sentence)

> Conductor proves that one kind of event raises one kind of reaction, on one service, through a sidecar on the same
> machine, and leaves half its claims to a human looking at a window — it cannot show whether a watcher on another
> host stays quiet for hours on a living system and speaks, correctly and in time, when something planted in that
> system goes wrong.

## 3. What WORKS today — preserve, build on

- The emission shapes (plain, error, exception with its variants, severity, latency, pii, ramp, breathing,
  topology) and the seeded scheduler with jitter: the vocabulary a world's events are made of.
- The run record, the journal and the report as artifacts; the verdict a check gives; the run class for "the harness,
  not the engine, was the cause".
- A gate before a run that proves the engine is there and answers (re-subjected by F3).
- The fourteen scenarios whose subject is a detection the engine keeps (error-rate spike, latency regression, retry
  storm, distinct fingerprints, silence, the activity floor, restart suppression, auto-resolve, exception and
  high-severity-log capture, error status and root-span scope, PII scrubbing) — as SUBJECTS; their form changes (F6b).

## 4. The rule of this version

**A removal is complete only when what existed solely to serve the removed thing is gone too, or has a new subject
named here.** Conductor loses four things at once — its own window, Pulse's window, Pulse's local model, and the
machine it shared with Pulse. Each is a finding of its own below, so that it gets a capability id and a matrix line
whose acceptance states an outcome.

## 5. Findings — OBSERVED vs EXPECT

Coordinates in OBSERVED were read from source at `97dea7f`; sizes are line counts of the files named.

### Theme 0 — What leaves

**R1 — Conductor's own window is gone.** (ruled by the founder, 2026-10-09: both projects go to a console form)
OBSERVED: the crate `conductor-tauri` (59 files, about 7000 lines, 3200 of them accessibility tests); the CI jobs
`frontend` and `a11y`; three window scripts and the e2e and a11y arms of the run script; thirteen master and leaf
files about design, layout and accessibility. Outside that crate, types shaped for the panel: the run-event channel
payload, an envelope read "for the desktop report", a catalogue summary "for the picker", a text limit sized for one
dialog row, a Tauri log sink, two webview fixtures with three tests.
EXPECT: no crate depends on the window toolkit; no type, limit, fixture or sink exists for a panel; no gate boots a
window or audits one; every function Conductor has is reachable from the command line; the masters say in plain words
that Conductor has no interface.

**R2 — everything that grades Pulse's local model is gone.** (ruled, 2026-10-09: «Да, убираем»)
OBSERVED: the real-model scenario, its posture contract of 936 lines with six pre-registered series, eighteen test
files (about 4900 lines), a live leg in the run script. And the other half, which the first assembly missed: the
deterministic stand-in for the model is a launch term (`l4-deterministic`), a posture enum (`L4Posture`, in 13
source files), an environment handle; a verdict exists for "the model's weighting" (`CalibrationRegion`, in 22 source
files) and a report state for Pulse's degraded mode (`KnownResidual`, in 12 source files); eight scenarios have the
model as their subject (the three severity tiers, the degraded-mode report, project-context grounding, the cadence
configuration, the run contract's canned mode, the real-model interpretation).
EXPECT: no scenario, contract, launch term, posture, verdict, report state, test or script names a model of Pulse's,
real or canned; none of it is run or extended; the records of what was measured stay as history, marked closed with
the model they graded.

**R3 — everything that grades Pulse's window is gone.** (new; follows from Pulse's R1, ruled)
OBSERVED: seven scenarios have Pulse's window as their subject (the halo's hue and breathing, the findings counter,
the Investigate buttons, the report's render surface, the constellation's live-only truth and its discovery); a
measurement contract of 302 lines and a delegated-timing harvest of five files (about 1240 lines) grade how fast the
window paints; an operator hold stops a run for a human to look (`pause.rs` in the core and the command line, about
500 lines), fed by a `[[checklist]]` schema; a state and a lamp exist for a claim "with no programmatic verdict"
(`ManualCheck` in 24 source files, `[MANUAL]`); sixteen capabilities are classified "drive and observe".
EXPECT: Conductor grades nothing a human must look at: no hold, no checklist, no manual state or lamp, no
drive-and-observe class, no contract or harvest about a window's timing, no scenario whose subject is a window. A run
needs no person from its first line to its last.

**R4 — everything that assumes Conductor and Pulse share a machine is gone.** (new; follows from Pulse's F2, ruled)
OBSERVED: emission goes to the fixed address `http://127.0.0.1:4317`, plain, with no credential and no setting for
another (`conductor-emit/src/client.rs:23`). Conductor spawns Pulse's sidecar from its own `PATH` and passes it a data
directory; the gate before a run requires that directory to equal the live Pulse's, a workspace key, three
environment handles read in "the shell that also launches pulse-app", and a walk of `PATH`. A fault binds Pulse's port
on Conductor's own host. Five scenarios have that shared machine as their subject (the receiver's lifecycle and its
port conflict, the connection tracker, the health domains, the workspace-key reconciliation). The run contract has
terms for a shared data directory, a built sidecar and an enabled MCP. **Twelve test files (about 3800 lines) hold the
real proofs of the detections that survive — and they read Pulse's log file from that shared disk.** The live suite
launches three legs against an operator-started application, two of them about the window and the model.
EXPECT: nothing in Conductor reads a file of the engine's, starts a process of the engine's, binds a port of the
engine's or requires a directory, a key or an environment in common with it; the address and the credentials of the
engine are given to a run; no scenario, fault, contract term, precondition or suite has a shared machine as its
subject. In development both programs run on one machine through the same network path as on two.

**R5 — the short-run time model is gone.** (new)
OBSERVED: a closed set of tiers (under 5, 20 and 90 seconds) is a field of every scenario and a column of every run
(`SloTier`, in 24 source files); latency is one number per scenario, read-back minus emission; twelve scenarios
already exceed every tier and are pinned as exceptions in an audit ledger; the load envelope bounds an emitting phase
and a scenario at ten minutes on the premise of a stall in Pulse's in-memory store.
EXPECT: time is judged per event on the run's one timeline — how long after an event's onset the engine reported it
(F9) — and no tier, no per-scenario latency and no exception list for one remains. The load a run may put on the
engine is re-measured against the engine as it is now, with its store on disk, and bounds a run of hours.

**R6 — the capability set follows Pulse's current record.** (new; the standing rule "every Conductor intent syncs
against the REAL Pulse artifacts", made a finding)
OBSERVED: the accepted set is pinned at Pulse `v0.3.0`, P-001…P-082 (`contracts/pulse-capabilities.toml`), with a test
that P-083 is rejected; 82 hard-coded classification rows, a list of unbacked ids, drift gates and `coverage-matrix.md`
stand on it; a scenario may not load without a `P-` id from it; four pinned copies of Pulse's 0.2–0.3 records sit in
`.andromeda/refs/` (its capability spec of 852 lines, its sidecar contract, an audit, a matrix). Pulse's 0.4.0 retires
part of the 82 with its window, its model and its desktop, re-bases the record id by id (its R11), and adds P-083
onward.
EXPECT: Conductor's accepted set, its classification and its coverage record are derived from Pulse's re-based
capability record and name no capability Pulse has retired; every run and every event in a schedule names the
capability of the engine it exercises; the pinned references are Pulse's current ones. Where Pulse's record is not
yet written, Conductor's stays unbuilt rather than guessed.

**R7 — the other operating systems are gone.** (new)
OBSERVED: the `rust` CI job runs on a Windows runner with a frontend-bundle step; `agent-run.ps1` (456 lines) twins
the run script.
EXPECT: Conductor builds, tests and runs on Linux alone and holds no script or CI leg for another system.

**R8 — the records say what Conductor is.** (new)
OBSERVED: `CLAUDE.md` opens "a desktop control-panel app (Tauri 2) over a headless-drivable Rust core … local-only, no
network service of its own"; `.andromeda/project.yaml` says a harness for Pulse's "60 claimed capabilities";
`.andromeda/input.md` says it verifies "via an operator checklist where the reaction is visual" and "Runs next to a
real Pulse instance on the dev host"; architecture places it beside Pulse in three places; the security rule every
agent loads says "local-only loopback tool, no network service, no secrets", which F2b contradicts; three records
describe Pulse's credential store.
EXPECT: every record a newcomer or an agent reads first describes a console harness that drives an engine on another
host over a network with credentials it holds; no rule instructs what this version removes or forbids what it adds.

### Theme 1 — Console, network, door

**F1 — a program without a window.**
OBSERVED: a Tauri crate with its own interface beside `conductor-cli`.
EXPECT: Conductor is a console program; a run is started, followed and read from the command line.

**F2 — the stream goes over a network.**
OBSERVED: R4's fixed local address.
EXPECT: Conductor sends its stream to an engine on another host, with a token over an encrypted channel; the same
path is used when both run on one machine. It never sends a token to another host over a plain channel, and it
refuses an engine whose identity it cannot verify; what it verifies the engine against is given to a run together
with the address, and where that comes from is stated.

**F2b — Conductor holds credentials now.** (the overseer's, PROVISIONAL until the founder's own word)
OBSERVED: Conductor has never held a secret; its security plan rates it a minimal-tier local utility that persists
only its own synthetic data.
EXPECT: the engine's token and the door's credential are given to Conductor at run time and are never written to a
run description, a run record, a journal, a report or a log; a run against an engine it cannot authenticate to is
reported as blocked, not as a failed check; where a run reads its credentials from is stated, and it is never the
repository; the engine's address is not a secret, but it is kept out of every committed artifact as a host path
is today — a run's record names its engine by the label the operator gave it; Conductor holds no credential of a model's (F12); the security master rates Conductor for what it now is.

**F3 — the reaction is read through the door an agent uses.** (ruled)
OBSERVED: R4's spawned sidecar; a pinned contract for its stdio protocol and five tools; one of the tools writes (it
marks an incident resolved) and Conductor's lifecycle tests use it.
EXPECT: Conductor reads the engine's reaction through the door the investigating agent uses — so every run also
proves the door and the network receiver — and reads the engine's one-line status as the panel module does. The gate
before a run proves three things through that path: the engine is reachable, it admits Conductor, and a canary event
sent now comes back as a finding. The one write the door keeps is used where a run needs it and named as a write.
A reaction is attributed to the run that caused it: a run states the engine state it starts from, and what the
engine already held when the run began — incidents, learned state — is told apart from what the run raised. Every
claim about the engine is made against the engine; a check that runs without one claims only Conductor's own
behaviour and says so. Checks that need an engine run where an engine is — the dev host and the two-host run —
and leave run records; Conductor's CI runs only what claims Conductor alone and holds no credential of an engine's.
Admission is the engine owner's consent: Conductor sends nothing to an engine that has not admitted it, so an engine
that watches a real product is never a target by accident. Besides findings and the status line, Conductor reads
through the door the record of each notification the engine raised.

### Theme 2 — A small world

**F4 — several services that call each other.**
OBSERVED: every shape but `topology` emits as the one service `conductor`; a phase has no service of its own; the
`topology` shape draws one linear chain with at most one failing span.
EXPECT: a world of three to five named services with call links between them, each carrying a version; a fault can
be placed in one service and be felt in another.

**F5 — a living background.**
OBSERVED: between a scenario's events nothing happens.
EXPECT: a traffic curve with a compressed "day", ordinary noise, harmless errors at a low rate — so that silence
from the watcher on a healthy hour means something.

**F6 — events on a schedule, several at once.**
OBSERVED: a scenario is one ordered list of phases; two events on different services cannot overlap; a slow ramp
leaves in one export; the suite runs scenarios back to back with no shared timeline.
EXPECT: one timeline for the whole run; events are scheduled on it and may overlap. The first long run holds four
families (ruled): a silent death, a slow creep, periodic spikes, a break after a change — the last one as a new
version of a service that starts failing.

**F6b — one way to describe a run.** (new)
OBSERVED: 37 scenario files in the single-list form; by subject 7 are about Pulse's window, 8 about its model, 5
about the shared machine, 14 about a detection the engine keeps, 3 unclear (an acknowledge click in Pulse's window, a
"previously seen" line in its report, a hot reload by an operator's hand). 35 of the 37 assert nothing at read-back;
the two that do check the word `error`, and the audit ledger records both as not discriminating.
EXPECT: a run is a world and a schedule of declared events, and that is the only form. Each detection the engine
still claims has a SHORT run in that form as its regression proof — its stimulus one scheduled event, its truth
declared, its reaction read through the door. The three unclear subjects follow Pulse's re-based record (R6): what the
engine still claims and Conductor can exercise from another host gets a run; what needs a hand on the engine's host
leaves. The single-list scenario form and every file in it are gone.

### Theme 3 — Declared truth and grading by time

**F7 — the schedule says what is true.**
OBSERVED: a scenario has no field for the expected cause or the expected service; a known cause lives in a comment;
"nothing should surface" cannot be declared.
EXPECT: every scheduled event declares what it is, in which service, from which minute, and what a correct
reaction names; a healthy stretch declares that nothing should be reported.

**F7b — a proof discriminates.** (new)
OBSERVED: the two live assertions pass for any stimulus; the audit ledger exists to record that.
EXPECT: every regression run has a paired control that differs from it in exactly one declared thing and is graded
differently. For a detection, the control is the same world without the event, graded "nothing reported". For a
subject whose correct reaction is silence — a suppression, a floor — the control keeps the event and removes what
should silence it, and is graded "reported". A run whose assertion would also hold in its control is refused when
the run is described, not recorded afterwards.

**F8 — the reaction is logged against the timeline.**
OBSERVED: a run record keeps a verdict, instants and fingerprints, and no text of the reaction; the security plan
forbids persisting Pulse's corpus content, with one scrubbed exception written for the model's text.
EXPECT: a run keeps, per reaction, when the engine spoke and the structured facts it named (service, version, kind
of finding, error identity, evidence identifiers). The world is Conductor's own synthetic one, so those facts are
Conductor's own names coming back; the content boundary is restated for that case, and the exception written for the
model's text is closed with R2.

**F9 — grading lays the two timelines together.**
OBSERVED: grading is a set of per-scenario checks against one read-back taken after the scenario ends.
EXPECT: for a whole run: which planted events were caught, how many minutes after their onset, how many reports
came during healthy stretches per hour, and whether each report named the declared service and cause. The two
timelines are laid together on one stated time base; the difference between the two hosts' clocks is measured, and
a verdict it could change is reported as such. The verdicts that remain are the ones a machine can give.

### Theme 4 — The long run

**F10 — hours, on a processor, at any hour.**
OBSERVED: R5's ten-minute bound; the longest tier is 90 s; the real-model leg needed a GPU and quiet windows.
EXPECT: a run of ten hours or more against the console engine, with no GPU; it also reads the engine's memory
against its 1 GB target and the growth of its database file — through the door, as part of the engine's own state,
never from a file or a process of the engine's (R4). The run's own record, journal and log are written as
the run goes, stay bounded in memory, and their size on disk over ten hours is bounded and stated; a run interrupted at any hour leaves a readable record of what happened until
then.

**F11 — a recorded run can be played again.**
OBSERVED: a reading cannot be repeated without driving the whole thing live.
EXPECT: because the engine counts by event time, a run's emitted stream can be replayed to it faster than it was
recorded and give the same reactions; a change to the engine is then measured in minutes.

**F12 — the end-to-end scenario of the pair.** (ruled as the definition of done for both projects; who the large model
is in this version is the overseer's, PROVISIONAL until the founder's own word)
EXPECT: Conductor rolls out a "bad version" of one service in its world, to an engine on another host; it reads
that the status line and the report name the service, the version and the new error; a large model, given the
report and the door, names the planted cause — Conductor records that answer against the declared truth. In this
version the large model is an agent the developer already runs, connected to the door the way a developer connects
it. Conductor does not start it and holds no model credential: for a run it hands out the report and the door to
ask, and it takes the agent's answer back through a command — bounded in size and scrubbed before it is recorded.
The engine calls no model (that is Pulse's 0.5.0). The answer is a reading recorded over planted faults, never a
gate that can fail a build.

## 6. Scope, priorities, non-goals

- **Order (ruled):** in pairs with Pulse, one builder at a time. Pulse's door replaces its stdio sidecar as soon as
  its console engine exists, ahead of its removals (Pulse's intent, third assembly, F6), so the door is Conductor's
  read path from its first check of the engine; the spawned sidecar (R4) is replaced, never adapted.
- **What the engine offers at each of its checks** — read from Pulse's `andromeda-pulse-0.4.0/working-route.md` at
  `b3ac58a`; each line is what Conductor must be able to do by then, and Conductor's order follows it:
  - From Pulse's entry `Door inside the engine's process` (its Epoch 1) no sidecar exists: the engine is read through
    the door on its own host, while telemetry still goes to its loopback receiver with no credential. Reading
    through the door is therefore Conductor's first step.
  - `Capability record re-based` (Pulse's Epoch 1) is the record R6 waits for.
  - `Theme 0 checked by the external harness` (end of Pulse's Epoch 3): detection after the removals, read through
    the door; from that epoch an incident is the engine's own record, not a model's answer.
  - Pulse's Epoch 4 opens the network receiver behind the token and the door's reach from another host. `Theme 1
    checked` (end of its Epoch 5): another host sends and reads through the door. (Pulse measures its own memory
    in that epoch by its own means.)
  - `Theme 2 checked` (Pulse's Epoch 6): restart survival and the door's answers, input to output; the restart
    between the two readings is done on the engine's host by whoever runs the check, never by Conductor (R4). With
    the store on disk comes the long Conductor run that reads the engine's memory and its database's growth (ruled:
    the 1 GB target is measured by a long Conductor run at the step of memory on disk; R5, F10) — hypothesis: the
    door offers those two figures as the engine's own state (Pulse's P-093); if Pulse's route has no entry that
    says so, name it at Phase 4 as a line owed on Pulse's side.
  - `Theme 3 checked` (Pulse's Epoch 7): what was sent is what the door returns — service, version, attributes.
  - `Theme 4 checked` (Pulse's Epoch 8): a worse release and a quiet service each told correctly — this needs the
    world's versions (F4) and the schedule's declared truth (F6, F7).
  - `Theme 5 checked` (Pulse's Epoch 9): a planted fault reaches the status line, the notification and the report —
    Conductor reads all three through the door (F3).
  - `Bad-version scenario end to end` (Pulse's Epoch 10) is F12; Pulse's `Version close on Linux` needs R6's
    accepted set agreeing with Pulse's capability record.
- **Out of this version:** grading of slow findings as caught (Pulse's ladder of timeframes is its 0.5.0; in 0.4.0
  the slow families are emitted and the reading "not reported" is the measured starting point).
- **Beside Conductor, not inside it:** a real stream from one of the founder's own local projects is watched too
  (ruled 2026-10-08; which project is not yet named). It is Pulse's finding and it gets an engine run of its own:
  one engine watches one product, so the real stream and Conductor's world never share a store.

## 7. Definition of done

1. F12 passes on a real two-host run.
2. A ten-hour run completes with its graded timeline.
3. Every detection the engine still claims has a short run that discriminates, read through the door.
4. **No survivor.** At the close, the question this assembly was built from is asked again of the tree — what stands
   that exists only for a window (Conductor's or Pulse's), for Pulse's local model, for a shared machine, for another
   operating system or for a human in the loop — and the answer is nothing.

## 8. What we expect Andromeda to derive from this intent

- `vision.md` from §1, §2, §4 and §7; `requirements.md` with one numbered capability per EXPECT, every removal
  included. Ids continue this project's own scheme: `v4-01` onward.
- A route in which no removal is a title only: each chunk that retires a thing names what leaves with it, and what
  replaces a thing is reachable before the chunk that removes it — the short runs through the door (F6b) exist before
  the log harvest (R4) and the single-list scenarios leave.
- The three `open` lines of `.andromeda/residuals.md` dispositioned: the founder's 2026-10-07 direction is absorbed
  by Themes 2–4; "which webview legs the Linux host can run" is retired with R1; the run-report envelope's `verdict`
  null arm is absorbed where R3 and F9 change what a verdict is.
