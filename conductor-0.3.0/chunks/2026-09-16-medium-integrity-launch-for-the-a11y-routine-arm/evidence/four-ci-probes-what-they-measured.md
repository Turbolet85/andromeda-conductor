# Four CI probes — what they measured, and where the evidence stops

**Chunk:** `2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm`
**Date:** 2026-09-16 · driven inside the fix-loop under the operator's ci-probe directive
**HEAD throughout:** `139bbb1` — no operator commit, every probe a detached ref, deleted after each.

| probe | run | probe sha | a11y job | what it settled |
|---|---|---|---|---|
| 1 | 35141676310 | `88979ff` | failure, 60 s | High→Medium lowering measured on the runner; leg never reached wdio |
| 2 | 35143892765 | `ce513bb` | failure, 13.5 m | leg builds and reaches the driver handshake at Medium; race lost |
| 3 | 35145664132 | `e53acac` | failure | race WON on the 4th POST; endpoint asked at Medium, `from chrome not reachable` after 61.8 s |
| 4 | 35147042449 | `60f42b2` | failure | second Medium sample, same signature (n = 2) |
| 5 | 35150449243 | `4d46c10` | failure | **the discriminator** — same instrument at both labels in one run; TEMP writable at Medium |

## SETTLED — the premise the plan could not measure on this host

**The High→Medium lowering works on the runner.** Measured on every probe from 2 onward, and from two
independent vantage points:

```
[diag] A11Y_LIMITED_TOKEN_PARENT_INTEGRITY: S-1-16-12288      (the step: High)
[diag] A11Y_LIMITED_TOKEN_CHILD_INTEGRITY:  S-1-16-8192       (the token it built: Medium)
[diag] (leg) IntegrityLevelSid: S-1-16-8192                   (read from INSIDE the leg)
[diag] (leg) IntegrityLevel: Medium
[diag] (leg) IsElevatedAdmin: True
[diag] (leg) WindowStation: WinSta0
```

Scope premise 1 is closed, and it closes in TWO halves that must not be collapsed:

- **The MECHANISM transfers.** The lowering direction the dev host could not exercise (its shell was
  already Medium) works on the runner, measured repeatedly and confirmed from inside the launched process
  rather than only from the launcher that set it.
- **The FINDING does NOT transfer.** On the dev host, non-admin+Medium CREATED a WebView2 session in six
  seconds. On the runner, Medium does not — it fails, differently but just as completely. So the
  predecessor's leg-C result is a dev-host property, not a runner property, and this chunk's remedy does
  not make the arm green. That is a premise disproof at the chunk's own level, and it is the honest
  headline: the integrity label was the right variable to test and the test came back negative HERE.

**CARRY 2's window-station axis is retired as a contributor.** `WindowStation: WinSta0` — the leg runs on
the interactive station, so the named-but-unvaried alternative is now varied and is not the blocker.

**A cell the predecessor's table never had.** Its three legs were admin+High, non-admin+High,
non-admin+Medium. `DuplicateTokenEx` keeps the group and moves only the label, so the leg now runs
**admin+Medium** — integrity varied with the role HELD, which is a cleaner variation than leg C's.

## SETTLED — the endpoint does NOT open at Medium, and it fails with a DIFFERENT error

Probes 3 and 4 both reached WebView2 at Medium and got an answer. The answer is not the High-run failure.

Probe 3's arm won the driver race on its 4th POST (after `Retrying 3/3`), at `20:25:03.944`. That POST
CONNECTED and returned at `20:26:05.740` — **61.80 s**, the WebView2 timeout signature, matching run
35102123985's 61.3 s. Its message is wrapped across two log lines, which is why it reads bare:

```
session not created
from chrome not reachable
```

**The signature dissociates cleanly by integrity, with three controls on the High side:**

| run | integrity | `chrome not reachable` | `DevToolsActivePort` |
|---|---|---|---|
| 35102123985 | High | **0** | 5 |
| 35103823579 | High | **0** | 5 |
| 35105588216 | High | **0** | 5 |
| 35145664132 (probe 3) | **Medium** `S-1-16-8192` | **8** | 0 from the leg |
| 35147042449 (probe 4) | **Medium** `S-1-16-8192` | **2** | 0 from the leg |

Every Medium hit is owned by the `A11y routine arm` step — 8 and 2 respectively, none from a diagnostic
step. So the Medium side is **n = 2**, not a single observation.

**The integrity change DID alter WebView2's behaviour on the runner — it moved the failure EARLIER, not
away.** At High the browser host starts and never publishes its debugging port; at Medium the driver
cannot reach the browser at all.

**Why the earlier reading missed this.** The attribution check searched for `DevToolsActivePort`, the High
signature, found 0 from the leg, and concluded the question had never been asked. The count was right and
the conclusion was wrong: at Medium the leg's error is a DIFFERENT STRING, so a sweep keyed on the High
wording cannot see it. A token that names one arm of a fork is not a probe for the fork.

**The exclusion arm now HAS a measured basis** — this signature change — where before it had none.

## THE DISCRIMINATOR — probe 5 (run 35150449243), same instrument, same run, one variable

Probes 3 and 4 established the signature but differed from the High controls in INSTRUMENT as well as
integrity: the leg goes through wdio and tauri-driver, the High `driver-alone` (b) sequence does not.
Probe 5 closes that gap by running (b)'s own sequence INSIDE the medium-integrity launch, so the
instrument is held and only the mandatory label varies — and both readings come from the SAME RUN, on the
same image, minutes apart.

| step (run 35150449243) | integrity | result |
|---|---|---|
| `WebView2 session isolation` (b) | High `S-1-16-12288` | status 500 · `session not created: DevToolsActivePort file doesn't exist` |
| `WebView2 driver-alone at medium integrity` | Medium `S-1-16-8192` | status 500 · `session not created` / `from chrome not reachable` · 64 s |

**`chrome not reachable` reproduces with wdio AND tauri-driver out of the chain.** The effect is isolated
to the driver-to-browser step under Medium. That is a qualitatively stronger basis than two leg runs
sharing one instrument: single-variable variation with the control in the same run.

### The profile-directory hypothesis is EXCLUDED

Mandatory integrity denies write-up, so a Medium child unable to create msedgedriver's
`scoped_dir…\EBWebView` profile would make the browser die instantly and surface as exactly this error.
Measured from inside the Medium leg on the runner, twice in this run:

```
[diag] (leg) TempTail: ...\Local\Temp
[diag] (leg) TempWritable: True (created, wrote and read back a byte under a scoped_dir-style child)
```

The child resolves a TEMP and can create, write and read back under it. So the launcher is NOT denying the
driver its profile directory, and the exclusion does not collapse into a launcher bug on that account.

### What the driver's own log adds

Both logs are the same binary (`152.0.4191.66`) with `--verbose`, from the same run:

| | High | Medium |
|---|---|---|
| `Launching Microsoft Edge` | 1 | 1 |
| `DevTools HTTP Request failed` | **0** | **51** |
| terminal line | `Failed to connect to msedge. Attempting to kill it.` → `DevToolsActivePort file doesn't exist` | timeout → `from chrome not reachable` |

At both labels the driver LAUNCHES the app; neither reaches a working DevTools connection. The failure
modes differ in where they die: High gives up on the absent port FILE without one HTTP attempt, Medium
polls an HTTP endpoint 51 times and times out.

**Not to be over-read:** no port was ever resolved in the Medium log either — no `127.0.0.1:<port>` or
websocket endpoint line appears. The 51 attempts are the driver polling and failing, NOT evidence that the
port file appeared or that the endpoint partially opened. Medium does not get measurably closer to a
working endpoint; it fails differently.

## THE TRANSPORT VARIATION — probe 6 (run 35184180963)

Every failure up to here belonged to the PORT transport and to its two failure points: the
DevToolsActivePort FILE (absent at High) and the loopback TCP connect (refused at Medium).
msedgedriver's own verbose log recommends the other transport, in BOTH the High and the Medium run —
"Use the --remote-debugging-pipe Chrome switch instead of the default --remote-debugging-port". A pipe
bypasses both failure points, and it had never been exercised AS A TRANSPORT: the workflow's pipe-route
step launches the app with the switch set and watches for a profile directory and liveness, but never
speaks over the pipe, and a profile appears in the bare-app control too.

The switch is passed through `ms:edgeOptions.args`, NOT through `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`:
msedgedriver OVERWRITES that variable for the browser it launches (its own log prints the value it set),
so a pre-set value cannot survive. Independently corroborated — the reporter on actions/runner-images
#14738 hit the same override from the wry/Tauri side.

| variant | integrity | transport requested | httpAttempts | pipeMentions | transport ACTUALLY used | result |
|---|---|---|---|---|---|---|
| driver-alone | Medium | port | 51 | 1 | port (honoured) | `from chrome not reachable` |
| driver-alone | Medium | pipe | 49 | 3 | **port — NOT honoured** | **UNMEASURED for pipe** |
| driver-alone | High | pipe | **0** | 3 | **pipe (genuine)** | fails at 60 s |

**The instrument was checked, not assumed, and that check changed a cell.** Each run reads back its own
driver log, because a requested transport the driver ignored would otherwise be reported as a transport
result. At High the switch demonstrably took effect — HTTP attempts 51 → 0, pipe mentions 1 → 3. **At
Medium it did not**: the switch was passed (3 mentions) yet 49 HTTP attempts still occurred, so that run
re-measured the PORT path under a pipe request. It is recorded as **UNMEASURED for the pipe transport at
Medium**, not as "pipe fails at Medium". Without the counters it would have read as the latter.

**The pipe is not a remedy where it was genuinely used.** The High pipe run establishes that the failure
survives a REAL transport change.

**One string in that row must not be over-read.** The High pipe run returned
`DevToolsActivePort file doesn't exist` while running in pipe mode — where no such file exists at all.
That text is msedgedriver's GENERIC "could not reach the browser" message, not evidence about a file and
not evidence of a file cause. Read as a file fact it would point the diagnosis in a direction the run
does not support.

### The limits, stated with the finding

- What is established is the **SIGNATURE and its LOCUS**, not the full **MECHANISM**. Probe 5 isolates the
  effect to the driver-to-browser step under Medium and excludes the profile-directory route. It does not
  say WHY that step fails — "the lowered token denies the driver-to-browser channel some other resource"
  and "WebView2 itself behaves differently at Medium" remain unseparated, and nothing here chooses between
  them.
- The varied factor is clean, though: probes 3 and 4 ran a PLAIN primary duplicate with only the mandatory
  label lowered (`IsElevatedAdmin: True` throughout), so integrity moved and the role was held. What is
  unresolved is the causal pathway, not which attribute changed.
- Both readings give the same terminal answer, which is the answer this chunk owes: **the WebView2
  debugging endpoint does not open at Medium integrity on this runner.**

Probe 1's `DevToolsActivePort` hits remain a separate lesson: 11 total, **0** outside the GitHub command
echo. Read without the echo filter that run would have read as "the endpoint stayed closed at Medium" —
true by luck, unfounded in fact, since that run's leg never reached wdio at all.

## A cost, not a blocker: the startup race that PREDATES this chunk

wdio connects to tauri-driver on 4444; tauri-driver binds 4444 only after msedgedriver answers on 4445.
wdio's budget is a small number of attempts over ~1.5–2.5 s, and a cold msedgedriver on the hosted image
sits on that edge.

Probe 4, the tightest reading:

```
20:38:32.834  crabnebula_tauri_driver: waiting for webdriver to start on port 4445
20:38:33.836  wdio attempt 1 -> ECONNREFUSED
20:38:33.845  crabnebula_tauri_driver: webdriver started          (9 ms after attempt 1)
20:38:34.339  wdio attempt 2 -> ECONNREFUSED                      (494 ms AFTER "started")
              Spec Files: 0 passed, 1 failed, 1 total in 00:02:01 · error: wdio exited 1
```

Probe 3 lost by 33 ms on attempt 2 and still saw attempt 3 refused ~1 s after "webdriver started". So the
4444 listener lags that log line — the race is not purely luck-of-the-draw.

**This red is NOT this chunk's.** Basis: run **35111618735**, headSha `7f5b399c`, on the tree BEFORE this
chunk's edits — 5 real ECONNREFUSED, identical shape. The control that proves the race is winnable is run
**35102123985** (headSha `7e07b783`, also predecessor tree): 2 ECONNREFUSED, race won on the third attempt,
and the arm then reached WebView2 and produced `session not created: DevToolsActivePort` at High integrity.
Owner: the wrap's pin.

The race costs attempts but did not block the measurement: probe 3 lost its first three POSTs and WON the
fourth, and probe 4 reached WebView2 as well. The race remains a real, pre-existing defect worth its pin —
it is simply not what stood between this chunk and its answer.

## Two defects this chunk introduced and fixed

**1. The token moved both axes.** The first implementation used `CreateRestrictedToken(LUA_TOKEN)`, which
lowers the integrity label AND adds restricting SIDs AND disables the administrator group. This chunk's
whole finding is that the label decides the endpoint and the role does not — so moving both was a
conflation of exactly the two questions the predecessor established must be kept apart. The
over-restriction broke child-process creation inside the leg: probe 1 shows the label dropping to Medium
correctly, then `whoami` failing with `ApplicationFailedException` and the leg never reaching wdio.
`DuplicateTokenEx` + `SetTokenInformation` relabels a plain duplicate and moves the label alone. Probe 2
confirmed the fix from inside: the witness's integrity reader, which threw under the restricted token, reads
`Medium` cleanly.

**2. A leg that never ran reported exit 0.** Probe 1's asserting step read `success` because it exits with
the leg's own code and the leg exited 0 without reaching wdio; only the downstream conformance gate caught
it, with "the a11y leg wrote no violation record". That is the false green a11y-plan §11 bans, and it is
also `v3-02`'s acceptance clause doing precisely its job — the clause exists so a wholly skipped run cannot
read as a pass. The witness now refuses both shapes at its own exit: **96** if the leg process never starts,
**97** if it returns 0 without producing its wdio log.

A third, smaller defect was mine too: the leg-console print I added to `ci.yml` used a blind `-Tail 120`,
which on a stderr dominated by cargo progress shows only `Compiling` lines and lets the driver window scroll
past. It now elides compile progress and keeps the rest.
