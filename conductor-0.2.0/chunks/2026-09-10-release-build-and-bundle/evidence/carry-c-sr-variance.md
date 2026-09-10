# CARRY C — the SR leg's run-to-run announcement variance

**Disposition: a cause-level FINDING, routed. Not fixed here, and no measurement arm ran.**
Per the chunk plan step 10 the investigation is bounded and "the chunk does not commit to a fix";
per a11y-plan §3/§11 the SR leg is supplemental and never sole, so no release claim rests on `S3-04`.

## What the CARRY asserted, and what survived

| claim | verdict at HEAD |
|---|---|
| The variance is real and not closed | **HOLDS** — `run3-lightgate` moved `S3-04` to `not-announced`, `heard: []` |
| The tally 33 / 15 / 2 / **1** | **EXACT** — re-counted from the committed capture |
| The mover is `S3-04` ("Escape resolves NoGo; focus restored") | **HOLDS** — `state: aborted`, `arm: agent` |
| `hypothesis:` the `nvda_named_window: false` correlation is the thread to pull first | **DOES NOT SURVIVE** (below) |

### The stated hypothesis is falsified by its own cited evidence

`nvda_named_window` does not discriminate across the three live runs. It also is **not** a top-level
field — it lives at `subjects.{subject}.foreground.nvda_named_window`, and a top-level probe reads
`None` for every run.

| capture | `subjects.live.recorded_at` | `nvda_named_window` | `S3-04` |
|---|---|---|---|
| `nvda-pass.live.run1.json` | 2026-09-07T19:40:33.165Z | `true` | announced-as-expected |
| `nvda-pass.live.run2.json` | 2026-09-07T19:43:17.621Z | **`false`** | **announced-as-expected** |
| `nvda-pass.live.run3-lightgate.json` | 2026-09-07T22:09:30.914Z | `false` | **not-announced** |

`run2` is the counter-example: the same `false` value with the row announcing normally, so the field
is 1-of-2 and cannot be the discriminator. All three `subjects.live.recorded_at` values are distinct,
so this is a genuine within-subject comparison and the cumulative-per-subject trap
(`verification-harness.md :63`) does not apply to it.

## What the code says now — the first cause was already fixed, and a second one remains

**The assertive-preemption cause is CLOSED at HEAD.** `Titlebar.tsx:23-27` carries the fix and cites
this very row:

> an assertive region preempts speech on EVERY flip — which cancelled the dialog's focus-restore
> announcement when a hold resolved (NVDA pass 2026-09-02, S2-07/**S3-04**)

with `phaseLiveness = runState === 'hold' ? 'assertive' : 'polite'` (`:27`). So the row moved on
run3 **despite** that fix, which is what makes the remaining cause a distinct one.

**The remaining mechanism — an extra, semantically spurious live-region mutation inside the exact
window where focus is restored.** On Escape → NoGo:

1. `App.tsx:190-194` `resolveHold()` sets `setHoldPrompt(null)` and then **`setRunState('live')`
   unconditionally — for `NoGo` as well as `Go`.**
2. `OperatorPauseDialog.tsx:43-48` `onCloseAutoFocus` calls `target.focus()` on the captured
   invoker, in the same close.
3. The backend's channel later delivers the `aborted` stage → `STATE_FOR_STAGE.aborted`
   (`App.tsx:83-88, :208`) → `setRunState('aborted')`.

So the titlebar label span mutates **`HOLD` → `live` → `aborted`**, and its `aria-live` attribute
mutates **`assertive` → `polite`** on the same node in the same commit as its text — while focus is
being moved to the Start button. The intermediate `live` is not a state the run is ever in on a NoGo:
the run is ending. It exists because `resolveHold` is shared with the `Go` path, where resuming to
`live` is correct; nobody scoped it by decision.

The spec itself already records the collision: `S3-05` is annotated *"(same instant as S3-04 — the
Aborted stage fires as the Escape resolution returns; shared window)"*
(`nvda-pass-spec.md:152`). A 1-in-3 intermittency fits a same-commit ordering race between a
mutating live region and a focus event far better than a field that reads `false` on a run that
announced normally.

## Why no fix, and why no measurement

- **No measurement arm was available.** The `sr` leg is gated on `CONDUCTOR_NVDA`, which is **unset**
  in this run's environment (the operator supplied `CONDUCTOR_MSEDGEDRIVER` but not `CONDUCTOR_NVDA`),
  and the chunk's `[[gate]]` fence carries **no `a11y:sr` entry** — so there was nothing to run and
  nothing to run it with. Recorded as a plan gap, not worked around.
- **A fix here would be unverifiable and disproportionate.** Scoping the `live` flip by decision
  changes shipped run-state behaviour that other SR rows (`S3-05`) and e2e specs assert; an app-side
  change is additionally invisible to the `sr*` suites until an `--e2e` run intervenes
  (`verification-harness.md :62`). Landing it with no NVDA arm to grade it is exactly the shape that
  produced the earlier stale-bundle false pass.
- **The banned remedies stay banned.** No retry, no runner pin, no in-test `sleep(N)`
  (a11y-plan §11 SLO; test-plan §10, §11 → E2E — the carve-out refused at
  `2026-09-06-operator-gated-live-suite`).

## Routed

To `.andromeda/residuals.md` via wrap's route-resolve (the only writer of that file), carrying:
the falsified `nvda_named_window` hypothesis with its three-run table, the surviving mechanism above
with its coordinates, and the note that the measurement needs `CONDUCTOR_NVDA` plus a gate entry that
does not exist yet.
