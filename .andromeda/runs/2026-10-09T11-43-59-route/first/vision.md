# Conductor 0.4.0 — vision

_Derived from `conductor-0.4.0/intent.md` (the operator's file, second assembly, 2026-10-09; md5
`3e5f1a6c7d0fdee44badc0ddb5ae6f32` when this was derived). That file is the authoritative version intent; this
is its framing (its §1, §2, §4 and §7), not a second source._

## The problem this version advances

Conductor 0.3.0 closed on 2026-10-08 at `97dea7f`, 11 of 11 verified. What it proves is narrow: that one kind
of event raises one kind of reaction, on one service, read through a sidecar it spawns on the same machine —
with half its claims left to a human looking at a window. It cannot show whether a watcher on another host
stays quiet for hours on a living system and speaks, correctly and in time, when something planted in that
system goes wrong (intent §2).

Pulse changes form in its own 0.4.0: a console engine on its own Linux node, with no window, no local model in
the engine, no workspace detection and no operating-system credential store. Telemetry reaches it over a
network behind a token; an investigating agent reaches it through a door inside the engine's process; an
incident is the engine's own record. Conductor has to follow, and it gets the job the founder named on
2026-10-07 and widened on 2026-10-08: stop emitting small series of single events; simulate a real service for
hours, and record how the watcher reacts (intent §1, his words quoted there).

## Who this is for, and why now

The Pulse developer and the founder, who judge Pulse by what Conductor measures. The two versions advance in
pairs, one builder at a time: Pulse's route is committed (`b3ac58a`), and each of its checks expects a
Conductor that can already do a named thing (intent §6). A Conductor that still spawns a sidecar reads nothing
from the first commit in which Pulse's door replaces it.

## The rule of this version

**A removal is complete only when what existed solely to serve the removed thing is gone too, or has a new
subject named in the intent** (intent §4). Conductor loses four things at once — its own window, Pulse's
window, Pulse's local model, and the machine it shared with Pulse — and each loss is a capability of this
version with an outcome of its own, never a non-goal. The founder's word on how the turn is done: nothing that
does not fit the new form is smoothed over.

## What is in

- **What leaves (intent Theme 0, F6b):** Conductor's window and everything shaped for a panel; everything that
  grades Pulse's local model, real or canned; everything that grades Pulse's window, the operator hold
  included; everything that assumes a shared machine — the fixed local address, the spawned sidecar, the
  shared-directory gate, the port fault, the harvest of Pulse's log file; the short-run tiers; the capability
  set pinned to Pulse 0.3.0; the other operating systems; the records that describe the old product; and the
  single-list scenario form with all 37 files.
- **Console, network, door (Theme 1):** a console program that sends its stream to an engine on another host,
  with a token over an encrypted channel, holds credentials given at run time, and reads the engine's reaction
  through the door an investigating agent uses.
- **A small world (Theme 2):** three to five named services that call each other, a living background, and
  events on one timeline that may overlap; a run is a world and a schedule, and that is the only form.
- **Declared truth and grading by time (Theme 3):** every scheduled event declares what is true; a proof
  discriminates; reactions are logged against the timeline; grading lays the two timelines together.
- **The long run (Theme 4):** ten hours or more on a processor at any hour; a recorded run replayed faster than
  it was recorded; the bad-version scenario end to end across two hosts.

## What is out

- Grading slow findings as caught: Pulse's ladder of timeframes is its 0.5.0. In 0.4.0 the slow families are
  emitted and the reading "not reported" is the measured starting point (intent §6).
- The real stream from one of the founder's own projects: it is Pulse's finding and gets an engine run of its
  own. The real stream and Conductor's world never share a store (intent §6).
- Any model credential of Conductor's own, and any model call by the engine (intent F12; Pulse's 0.5.0).

## What "0.4.0 done" means (intent §7)

1. The bad-version scenario (F12) passes on a real two-host run.
2. A ten-hour run completes with its graded timeline.
3. Every detection the engine still claims has a short run that discriminates, read through the door.
4. **No survivor.** The question the intent was built from is asked again of the tree at the close — what
   stands that exists only for a window (Conductor's or Pulse's), for Pulse's local model, for a shared
   machine, for another operating system or for a human in the loop — and the answer is nothing.
