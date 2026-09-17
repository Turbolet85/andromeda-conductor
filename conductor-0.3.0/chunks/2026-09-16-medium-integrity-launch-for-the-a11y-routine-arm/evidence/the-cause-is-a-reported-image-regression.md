# The cause belongs to the runner image, not to Conductor and not to the integrity label

**Chunk:** `2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm` · **Date:** 2026-09-17

## The external record

`actions/runner-images` issue **#14738** — "WebView2 WebDriver automation fails on windows-2025
(DevToolsActivePort file doesn't exist)". **Verified by fetching the issue itself**, not from a search
summary (see the warning at the end, which is the reason that distinction is recorded).

| | issue #14738 | this job, measured |
|---|---|---|
| image | `windows-2025-vs2026` version `20260828.587` | identical — read from probe 5's `Set up job` |
| WebView2 runtime | `152.0.4191.66` fails | identical |
| error | `DevToolsActivePort file doesn't exist` | identical to our High signature |
| stack | plain Tauri/wry app | same |
| token/integrity work | **none** | ours had it; the third party's failure has none |
| works on | `windows-2022` with runtime `131.0.2903.86` | untried until now |

The reporter ruled out driver/runtime version mismatch, GPU absence, and a missing or broken runtime
install. Issue open, no maintainer response, reporter's own reading is an image regression.

**The issue mentions no elevation, no integrity level and no restricted token anywhere.**

## What this costs this chunk, stated plainly

A third party reproduces our exact failure on our exact image and runtime with a plain Tauri/wry app and
**no token manipulation at all**. The cause therefore belongs to the IMAGE. Both integrity labels fail
here, and the failure survives both transports at both labels (probe 6).

So **the Medium line was varying a variable that was not the cause.** That is the honest shape of this
chunk's central work. What was measured remains true and remains recorded — the High→Medium lowering
works on the runner, the driver-alone isolation holds with the wrapper out of the chain, the
profile-directory route is excluded — but those are second-order differences INSIDE a failure that
belongs to the image. The integrity hypothesis was a reasonable thing to test, it was tested correctly,
and it is not the answer.

This also means a PERMANENT exclusion would have been wrong to write, and was not written. The
acceptance requires the basis to be the measured cause; the measured cause is now an image regression
with a documented working configuration, which is a remediable condition rather than a permanent one.

## MEASURED: the endpoint opens on windows-2022 with its NATIVE runtime — and the cause follows the RUNTIME

Two windows-2022 probes, and the first one was confounded by this repo's own gate.

| run | image | runtime IN EFFECT | driver / browser | bare-app `DevToolsActivePort` |
|---|---|---|---|---|
| 35150449243 (probe 5) | windows-2025 | 152.0.4191.66 | 152 / 152 | never within 90 s |
| 35184180963 (probe 6) | windows-2025 | 152.0.4191.66 | 152 / 152 | never within 90 s |
| 35185153012 | windows-2022 | **153.0.4234.32 — installed by OUR floor gate** | 152.0.4191.77 / 152 | never within 90 s |
| 35186499514 | windows-2022 | **131.0.2903.86 — native, floor bypassed** | 151.0.4129.107 / 151 | **1 s** |

**Row 3 is the strongest line in this record.** The `≥ 152` floor saw windows-2022's native 131, fetched
Evergreen and installed 153 — and the failure came WITH IT onto windows-2022. That is a WITHIN-IMAGE
variation: same image label, runtime varied, failure follows the runtime. It is stronger evidence than any
cross-image comparison, and it was produced by accident when the gate defeated its own experiment.

**Row 4 is the first time in six days that the debugging endpoint has appeared at all.** A bare app on
windows-2022 at its native runtime publishes `DevToolsActivePort` in ONE SECOND, where every
windows-2025 reading is "never within 90 s". The capability is not gone; it is gone on the newer runtime.

**So "image regression" is the right family but the wrong grain.** The discriminator that tracks the
failure is the WebView2 RUNTIME VERSION, not the image label. windows-2022 is valuable because it ships an
older runtime, not because of anything else about it — and forcing the newer runtime onto it reproduces
the failure exactly.

### What blocks green on windows-2022 now is a DIFFERENT and tractable problem

```
session not created: This version of Microsoft Edge WebDriver only supports Microsoft Edge version 151
Current browser version is 131.0.2903.86
```

A driver/runtime version mismatch — the image ships msedgedriver 151 against runtime 131. That is an
ordinary matching problem with known remedies, not a capability that has been withdrawn:

- install a VERSIONED WebView2 runtime matching the shipped driver (151) via the Standalone Installer —
  which is precisely the "pin the runtime to a version, Standalone Installer replaces the bootstrapper"
  disposition `security-plan.md` §Dependency Security already names as this float's exit condition; or
- pin msedgedriver to the runtime the image ships (131).

Either way the terminal is no longer plausibly a permanent exclusion. **A permanent exclusion written at
any point before this run would now be false**, and this is the second time in this chunk that holding it
open has been vindicated by the next measurement.

## THE ENDPOINT OPENED — run 35191569653, windows-2022, coherent 131/131

First WebView2 session created on a hosted runner in this investigation. The driver was pinned down to
the image's native runtime (both 131.0.2903.86, Authenticode-verified) and the driver-alone POST returned
**status 200** with a real session body:

```
"browserName": "webview2",  "browserVersion": "131.0.2903.86",
"ms:edgeOptions": { "debuggerAddress": "localhost:60826" },
"msedge": { "msedgedriverVersion": "131.0.2903.86 (cf5bf30...)" }
```

`debuggerAddress` IS the endpoint opening — the thing that had not happened once in six days.

| cell | integrity | transport | result |
|---|---|---|---|
| bare app | High | — | `DevToolsActivePort` in **1 s** |
| session-isolation (b) | **High** | port | **200 — session created** |
| driver-alone | **High** | pipe | **200 — session created** |
| driver-alone | Medium | port | 500 · `chrome not reachable` |
| driver-alone | Medium | pipe | 500 · `chrome not reachable` |

Three 200s, all High. Four 500s across this run and its predecessors, all Medium.

### The premise is INVERTED — and the predecessor's finding was configuration-BOUND, not wrong

On the dev host at runtime 153, non-admin+Medium created a session and High did not. Here, at 131 on
windows-2022, **High creates the session and Medium is what breaks it.**

**Integrity's SIGN is not invariant — it is an INTERACTION with the runtime/image configuration.** The
predecessor's legs A/B/C were correctly measured on what they measured, and this record does not retire
them: it bounds them. Writing "the earlier finding was false" would be the wrong correction and would
discard a sound measurement; writing "integrity's effect depends on the configuration it is measured in"
is what the evidence supports.

For this chunk the consequence is still sharp: on the working configuration, the medium-integrity launch
this chunk exists to build is the thing standing between the arm and green.

### What the 200 does NOT establish

The predecessor's own sentence applies again, verbatim: **integrity buys the SESSION, not a green arm.**

- A 200 proves the endpoint opens. It does not prove the routine arm passes.
- `accessibility.e2e.ts:384` / `:397` have **NEVER been run on runtime 131**. They are measured broken on
  153 (dev host) and recorded green on 152 at 2026-09-10. Runtime 131 is older than both.
- The green cell is **n = 1**.

So the remaining question is not whether a session can be created — it can — but whether the ARM passes
on this configuration. That is what run 35192876641 measures.

## THE ARM RAN — run 35192876641, and it is ONE SPEC from green

Configuration: windows-2022 · native runtime 131.0.2903.86 · driver pinned to 131.0.2903.86
(Authenticode-verified) · **High integrity, launcher REMOVED** · the arm itself, not driver-alone.

The driven-session banner appeared for the first time in this investigation:

```
[webview2 131.0.2903.86 windows #0-0] Running: webview2 (v131.0.2903.86) on windows
[webview2 131.0.2903.86 windows #0-0] Session ID: 94e745eb57383f2a1515c63eb694887f
```

**Tally: 11 passing · 1 failing · 2 skipped** (the two skips are the expected live-hold set).

### The one failure is `:384`, and it is NOT a session or endpoint problem

`✖ every idle-console control is keyboard-reachable by Tab alone (SC 2.1.1)` —
`accessibility.e2e.ts:373`, assertion at `:384`:

```
Expected: "6 reached [Minimize window | Close window | Filter scenarios… | Start | DIV | DIV | …]"
Received: "12 reached [Minimize window | Close window | Filter scenarios… | Start | DIV | DIV | …]"
```

`focusableCount` (the DOM query) is 6; `names.length` (the Tab cycle) is 12, and the name list REPEATS —
the cycle wrapped and revisited every control. The spec's cycle-termination does not detect the wrap on
runtime 131. **`:397` (SC 2.4.3 focus order) PASSES on 131.**

Attribution note: the `no such element` lines for `[role="alertdialog"]` in this log belong to the
operator-pause SKIP GUARD, not to this failure. Reading them as the cause would misattribute it.

### What `:384` now has, for the first time: a reading on three runtimes

| runtime | `:384` (SC 2.1.1) | `:397` (SC 2.4.3) | basis |
|---|---|---|---|
| 152 | green | green | 2026-09-10 record |
| 153 | FAIL | FAIL | dev host, 10 passing / 2 failing / 2 skipped |
| **131** | **FAIL — cycle wraps, 12 vs 6** | **green** | run 35192876641 |

This is CARRY 1's territory — a separate defect with its own owner, dated-advance-warning class — and
`accessibility.e2e.ts` is explicitly OUT OF SCOPE for this chunk under its plan's
`## Constraints & rejected approaches`. It is recorded here, not fixed here.

### The floor gate is now in direct conflict with the fix

The `≥ 152` floor was minted when OLD runtimes were the suspected cause on windows-2025. The measurement
inverts that premise: the old runtime is the one that works, and the floor forces every job onto the
failing one. On windows-2022 the floor is actively hostile to the remedy. `security-plan.md` §Dependency
Security states the floor assertion sits OUTSIDE the conditional and runs every job, so this is a posture
question for the operator, not a detail — and it is unresolved either way the terminal lands.

## Three documented routes, cheapest first

1. **Pin the job to `windows-2022`** — being probed. The pin's own stated rationale is
   explicit-label-rather-than-`windows-latest` and names no year, so moving to another explicit label
   leaves the decision behind it intact. Contradicts the literal label in this chunk's acceptance
   criterion and in architecture.md §Established Decisions [CI/CD], so it is an Expected amendment, not a
   silent edit.
2. **Attach instead of launch** — launch the app with a FIXED `--remote-debugging-port=<port>` and set
   `DebuggerAddress` so the driver attaches. Removes both measured failure points at once: a fixed port
   needs no DevToolsActivePort file, and the driver's launch path is bypassed. Untried.
3. **The registry channel** — `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` as a REGISTRY value, a different
   channel from the environment variable that msedgedriver overwrites (measured here) and that wry/Tauri
   also overrides (measured by the #14738 reporter). Untried.

## A methodological note worth keeping

The search engine's summary of this issue confidently asserted a mechanism — "WebView2 150+ hardening for
elevated hosts, `WEBVIEW2_*` ignored" — which fit this runner's built-in-Administrator account and this
chunk's token work so precisely that it would have explained everything. Fetching the issue returned an
explicit statement that those topics are not in it. The summary had synthesised the cause.

A search summary is a claim ABOUT a source, never the source. The failure mode is sharpest exactly when
the summary matches the hypothesis you are already holding.
