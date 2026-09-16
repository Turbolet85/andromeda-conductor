# The dev-host control no longer reproduces, and I asserted that it did

**Measured:** 2026-09-16 · **Two local legs, one variable** · **Cost:** zero CI runs

## What was claimed

This chunk discharged `v3-02`'s folded obligation (2) — "planning must MEASURE whether the routine specs
pass against the console as it stands before promising a green terminal" — by citing the routine arm's
**12 passing / 2 skipped** from 2026-09-10 and showing the console was unchanged since
(`git log --since=2026-09-10 -- ui/src ui/test` → 0 commits; `grep contentinfo` → 0 hits).

**That was not a measurement.** It established that one INPUT had not moved and treated the six-day-old
verdict as still current. The obligation's own wording anticipated exactly this failure.

## What is true today

Two legs run on this host within minutes of each other, differing only in the launch:

| Leg | Session created | Verdict |
|---|---|---|
| Under `runas /trustlevel:0x20000` | yes — `[webview2 153.0.4234.32 windows]` | `Spec Files: 0 passed, 1 failed` in 00:00:08 |
| Control, no `runas` | yes — same banner | `Spec Files: 0 passed, 1 failed` in 00:00:08 |

Both fail identically, on the same assertion at `accessibility.e2e.ts:397` — SC 2.4.3 focus order,
expected `"Minimize window then Close window"`, received `"Start then DIV"`.

**Two conclusions, and they are separate.**

1. **The failure is not caused by the restricted token.** Control and treatment are byte-comparable on
   the assertion, so `runas` neither creates nor masks it. It is a red this chunk did not introduce.
2. **The recorded green no longer reproduces on this host.** The control side of the elevation finding —
   the non-elevated known-good the whole cause rests on — cannot currently be re-run to green.

## The control repair was executed, and the skew is NOT the cause

Operator-ruled 2026-09-16: align the dev host's driver to its 153 runtime and re-run. Executed host-only
— no repository pin exists to move (`ci.yml` takes the driver from the runner image via `EDGEWEBDRIVER`;
`package.json` pins no driver package), and the handle was overridden **process-scoped for the re-run**,
leaving the operator's User-scope value untouched. The 153 driver was fetched and admitted by the same
control the project applies to a CI-fetched third-party binary: `Get-AuthenticodeSignature` → `Valid`,
signer `O=Microsoft Corporation`, checked before use.

**Result: no change whatsoever.** Three legs, one variable each, all identical:

| Leg | Driver | Runtime | Failing sites | Tally |
|---|---|---|---|---|
| under `runas` | 152.0.4191.53 | 153.0.4234.32 | `:384`, `:397` | 10 passing / 2 failing / 2 skipped |
| control, no `runas` | 152.0.4191.53 | 153.0.4234.32 | `:384`, `:397` | 10 passing / 2 failing / 2 skipped |
| **driver aligned** | **153.0.4234.32** | 153.0.4234.32 | `:384`, `:397` | 10 passing / 2 failing / 2 skipped |

So the driver/runtime major SKEW is eliminated as the cause. Note precisely what that does and does not
exclude: the runtime is **153 in all three legs**, and aligning the driver does not revert it. "Runtime
153 behaves differently from 152" therefore remains an open candidate — untested, because testing it
means downgrading an Evergreen runtime, which is a materially more invasive host change and is not taken
here.

## The regression is two tests, not the arm

Against the recorded green of **12 passing / 2 skipped**, today's arm is **10 passing / 2 failing /
2 skipped** — the same two skips, ten still green, and exactly two regressed:

- `accessibility.e2e.ts:384` — SC 2.1.1 reachability
- `accessibility.e2e.ts:397` — SC 2.4.3 focus order

Both are tab-order assertions, and both are precisely the **hold-free Operable pair** that
`2026-09-07-sr-findings-fixed` moved ONTO the routine arm, recording at the time that their **CI proof
was still OWED**. The pair that regressed is the newest content on the arm and the content whose
verification was already outstanding.

## Operator ruling: do not downgrade — the finding is dated advance warning

Ruled 2026-09-16. The runtime is the live suspect and the control is NOT on this chunk's critical path.

**The sequence, measured.** The WebView2 runtime directory for `153.0.4234.32` was created
**2026-09-15 21:38:03** — five days AFTER the 2026-09-10 recorded green. It is the only version
directory present: Evergreen removes its predecessors, so the prior version is not recoverable from this
host, but the bump's date is not in doubt.

**Why no downgrade.** CI measured `152.0.4191.66` in run 35079315258's own diagnostics, so the runner is
on the same major the green was recorded on. The dev-host regression at 153 therefore does not predict a
CI failure of these two assertions, and the endpoint question does not need the restored control. A
downgrade would also fight a mechanism built to move forward, and would not hold.

**What it is instead.** Dated advance warning: `:384` (SC 2.1.1 reachability) and `:397` (SC 2.4.3 focus
order) break on WebView2 153 and will go red in CI the day the runner image moves to it. The pair was
added by `06db2f9` in **82 lines** to `accessibility.e2e.ts` — the commit landed 2026-09-08 under chunk
marker `2026-09-07-sr-findings-fixed`, which recorded their CI proof as still OWED. This is a separate
defect with its own owner; it belongs on the route, not in this chunk's scope or its terminal.

## It is also evidence about the Evergreen float

`security-plan.md:185`'s third dependency class bounds the floating Evergreen bootstrapper with one
stated exit condition: pin the runtime once the a11y job actually GATES. This is the float **charging a
cost before the gate ever lit** — a silent overnight bump broke the arm's two newest assertions, on a
host where nothing in the repository changed.

`v3-02` already carries the pin-or-retire disposition as a folded obligation, to be taken whichever way
the terminal lands. That decision should now be taken knowing the float has demonstrated this failure
mode unprompted, rather than only as a theoretical exposure.

## Recurrence note — clipped views, three instances in one session

The `:397`-only misreading above was the **third** instance of one class today, across both the
implementer and the reviewer: a count or site list read off a `tail`/`head` view, then found different
when re-derived over the whole artifact. The other two were a per-path grep result quoted from a clipped
combined search, and a site enumeration taken from a narrower re-read than the graph query that had
already answered it. Recorded as a recurrence rather than three separate fixes, because the pattern —
not any one slip — is what the discipline has to catch: the project's own rule already says a count is
never read off a clipped output, and it was breached three times in a day by people who know it.

## Probable mechanism, correlated not established

This host now carries **msedgedriver 152.0.4191.53 against WebView2 runtime 153.0.4234.32** — a
driver/runtime MAJOR skew. That is the same condition the project already recorded as a measured
candidate for this failure class, in `ci.yml`'s own note on run `34645345201`: an unconditional install
"raised the runtime alone to 153.0.4234.32, and the live processes then loaded the 153 modules while the
driver stayed 152".

Correlation only. Nothing here varied the runtime, so the skew is not established as the cause of the
focus-order failure; it is the obvious candidate and it is cheap to test by aligning the pair.

Note the direction: CI installs a 152+ runtime and measured `152.0.4191.66` with a matching driver — the
runner is currently COHERENT and this dev host is the skewed one. The usual polarity is reversed.

## What this does and does not do to the elevation finding

**Does not void it.** The elevation cause was established 2026-09-12 by variation with a control on both
sides, on this host, when the pair reproduced. A measurement that really held is not retroactively
falsified because the host later drifted — the change invalidates FUTURE legs, not past ones.

**Does block re-confirmation.** Any new attempt to vary elevation on this host now runs against a tree
whose arm fails for an unrelated reason, so the control side is unavailable until the skew is resolved.

**And it removes a claim this chunk was leaning on.** Any wording that says "the routine arm passes
locally" must now say "passed on 2026-09-10; does not reproduce on 2026-09-16 at driver 152 / runtime
153". That correction belongs in the chunk's own record, not in a spec.

## The discharge was a citation, not a measurement — and the reviewer's check inherited the same gap

Recorded at the reviewer's instruction, because it is the same class on both sides.

The obligation asked for a **measurement of the arm "as it stands"**. What was supplied was a six-day-old
verdict plus an "unchanged since" argument — and that argument's input set was **repository** inputs
only: `git log` over `ui/src` and `ui/test`, and a `grep` for `contentinfo`. Both were true and both
reproduced when re-run at review.

The gap is what they cannot see. The arm depends on **host tooling no repository grep can reach** — the
WebView2 runtime, the Edge browser and the driver all live outside the tree, all move independently of
it, and two of the three moved. So "unchanged" was sound over the inputs it covered and silent over the
inputs that actually changed. A reviewer verifying the two greps therefore confirms the citation
faithfully and inherits its blind spot; that is not a failure of the check but of its scope.

The correction applies whatever the aligned re-run returns: this chunk's landmark discharge is a
**citation**, and the only measurement of the arm "as it stands" is the one taken on 2026-09-16.

## Bound on what the earlier probes established

The blind window described below was open for **CI probe runs 35079315258, 35095825604 and the probe
series** as well. Those runs showed a session-creation failure and would equally have hidden any earlier
failure — a frontend or cargo build fault would have produced no visible output at all. So their
negative readings bound to "no session was created", and must not be read as "nothing else failed
first". The token witness's readings are unaffected: it runs before the leg and writes to a file.

## Method note

Both legs also closed a blind window that had nothing to do with their verdict: everything the harness
does BEFORE wdio — frontend build, cargo build — printed only to a console that a detached launch
discards, so a failure in that window produced no visible output at all, in CI as much as locally. The
leg's console is now captured to a gitignored file and printed by the launcher, which is how the
focus-order assertion became readable here.
