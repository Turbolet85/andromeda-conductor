# Hosted-runner WebView2 endpoint — the cause readings

Chunk `2026-09-11-hosted-runner-endpoint-cause-closed`. Extends the probing pass recorded at
`conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-probed/evidence/reading.md`, which retired two
of three candidates and left the third unread.

Every reading below is stated with the run that produced it and bounded to what that run measured. Nothing
here claims a green run, WCAG conformance, or a platform capability.

## Runs read

| run | head sha | branch | event | conclusion | a11y job |
|---|---|---|---|---|---|
| `34645345201` | `c93a379` | `build/conductor-0.3.0` | push | failure | `A11y gate (routine arm · axe · contrast · violation JSON)` — failure |
| `34654076633` | `eecc7f4` | `build/conductor-0.3.0` | push | failure | same job — failure |

Both runs' overall `failure` is attributed to two standing defects, neither of them this chunk's: the
`Rust gate (build · test · lint · supply-chain · coverage)` job is red on a pre-existing ledger-gate defect
(below), and the `A11y gate` job is red at the WebView2 session-creation failure this chunk is measuring.
The `Frontend gate (npm audit · build)` job succeeded in both.

## Arm A — the module-version reading, taken where its subject is alive

The probing pass could not read this: its probe ran in the `WebView2 cause probes (diagnostic)` step, which
executes after the `WebView2 session isolation (diagnostic)` step has already stopped the app it started. The
reading was recorded then as *returned nothing, for a measured reason* — 0.645 s of separation.

Moving the module section into the isolation step's bare-app window resolved it. **One job log carries both
placements**, 97 s apart, which is what makes this a placement result rather than an assertion:

| step | reading (run `34645345201`) |
|---|---|
| `WebView2 session isolation (diagnostic)` @ 20:52:44 | `msedgewebview2 : 6 live process(es)` · `msedgewebview2.exe fileversion: 153.0.4234.32` · `msedge_elf.dll fileversion: 153.0.4234.32` · `conductor-tauri : 1 live process(es)` · `EmbeddedBrowserWebView.dll fileversion: 153.0.4234.32` |
| `WebView2 cause probes (diagnostic)` @ 20:54:21 | `msedgewebview2 : no live process` · `conductor-tauri : no live process` · `msedge : no live process` |

The later step's `no live process` lines are **expected and retained**, not a defect: that invocation runs
outside the live window by design, and the pair is the evidence. Proven by the committed gate
`grep -c "\[diag\] (2) msedgewebview2 : [1-9]"` over the captured job log reading `1`, not asserted.

Dev-host control, same script, `-Section modules`: `msedgewebview2 : 6 live process(es)` at
`152.0.4191.66`.

## Arm C — driver/runtime major skew, varied

Arm A's reading exposed a difference nothing had named. At run `34645345201` the hosted image carried a
**coherent 152 triple** — `msedgedriver: 152.0.4191.66`, `Edge browser: 152.0.4191.66`, and the WebView2
runtime at `152.0.4191.66` before the install step — and the `Install WebView2 Evergreen runtime 152+ (gate)`
step then raised the **runtime alone** to `153.0.4234.32`. The skew was created by the job itself, and arm A
proves it reached the processes: every module they loaded was `153.0.4234.32` under a driver still at 152.

The earlier runtime-major test (run `34280136892`) moved the runtime while the driver stayed put, so a skew was
present in every run this job has ever had; no observation existed with the two on one major.

**The variation:** the install step was made to honour the gate its own name states — install only when the
measured `before` is below the floor. At run `34654076633` it no-opped:

- `WebView2 Evergreen Runtime install: SKIPPED (present 152.0.4191.66 already satisfies the 152+ floor)`
- `WebView2 Evergreen Runtime in effect: 152.0.4191.66` · `msedgedriver: 152.0.4191.66` · `Edge browser: 152.0.4191.66`
- the first run in this job's history with driver, browser and runtime on one major

**The variation reached the processes, not just the registry** — the reading that makes this a controlled
variation rather than a configuration change: arm A's probe in the same run read
`msedgewebview2.exe fileversion: 152.0.4191.66` and `msedge_elf.dll fileversion: 152.0.4191.66`, against
`153.0.4234.32` in run `34645345201`.

**The outcome did not move.** Every isolation reading is byte-identical across the two runs:

```
(a) bare-app DevToolsActivePort first seen: never within 90s
(a) bare-app EBWebView present: True
(a) app-alone HasExited=False ExitCode=n/a
(b) fresh scoped_dir first seen: never  count: 0
(b) any fresh scoped_dir EBWebView present: False
(b) any fresh scoped_dir DevToolsActivePort present: False
(c) LOCALAPPDATA app EBWebView present: False
driver-alone status=500
```

with wdio reporting `session not created: DevToolsActivePort file doesn't exist` and `0 passed, 1 failed`.

**Driver/runtime major skew is RETIRED as a cause** — by direct variation with a control on both sides, not by
argument. This is a negative result, and it is worth more than the positive it did not produce: the candidate
was measured rather than reasoned away.

## The candidate ledger

| candidate | status | basis |
|---|---|---|
| WebView2 / Edge **policy state** | RETIRED | all five probed keys ABSENT on the hosted runner AND on the dev host — no difference exists to explain one |
| **session 0 / non-interactive** service context | RETIRED | runner is `SessionId 2`, `UserInteractive: True`, corroborated by the isolation step's own `tasklist` on Console session 2 |
| **loaded module version** | MEASURED, and its **skew RETIRED** | unreadable until arm A repositioned the probe; then varied directly by arm C — driver, browser and runtime brought to one major, modules confirmed at `152.0.4191.66`, and every failure reading unchanged |
| **elevation** | **ESTABLISHED** | varied directly on the known-good host (arm B): elevated and non-elevated differ on `IsElevatedAdmin` alone — same machine, driver, runtime, tree and session — and the elevated run reproduces `session not created: DevToolsActivePort file doesn't exist` where the non-elevated case is a recorded green |

**The decision: REMEDIABLE IN PRINCIPLE.** The endpoint failure has a named cause — the app running under an
elevated token — and a stated remedy: it must not run elevated. This is a cause reached by variation with a
control, not by exhausting the list; the three retirements above narrowed the field, but it is arm B that
decides it. The MECHANISM behind the cause is `hypothesis:` and unmeasured (below).

## Arm B — elevation varied on the known-good host: the failure reproduces

Run elevated on the dev host 2026-09-12 11:43:25–11:58:52 local, one session, `CONDUCTOR_A11Y_STRICT=1` set
in-session, `E2E_EXIT=1`, sentinel `ARMB2_DONE` reached (so the capture completed rather than dying silently
as the first attempt did).

**The driver starts and accepts; the endpoint never appears.** From the leg's own log:

```
Starting msedgedriver 152.0.4191.53 (…) on port 4445
msedgedriver was started successfully on port 4445.
crabnebula_tauri_driver: webdriver started on port 4445
[0-0] RUNNING in wry - …/test/a11y/accessibility.e2e.ts
[0-0] INFO webdriver: [POST] 127.0.0.1:4444/session
```

and, 60 seconds later, from the error capture the first attempt could not produce:

```
WebDriverError: session not created: DevToolsActivePort file doesn't exist
  when running "127.0.0.1:4444/session" with method "POST"
```

*(The URL scheme is elided from those two quoted lines — scheme, colon, double slash — and nothing else is
changed. This record is gated by a host-path grep whose drive-letter anchor `[A-Za-z]:[\/]` is satisfied by
any such scheme, exactly as it is by the PowerShell registry provider prefix the probe script already renders
around. The standing rule is to fix the output, never the pattern: loosening the anchor would blind the gate
to the leaks it exists for.)*

The release build completed first (`Finished release profile in 22.82s`), so the leg reached session creation
with a freshly built bundle. **The same signature as runs `34645345201` and `34654076633`.**

The leg carries **no** `[webview2 … windows]` banner, **no** `verdict asserted` line, **no** `leg skipped`
line and **no** `Spec Files:` summary — it never reached the printed-verdict assertion, failing earlier at
session creation. Worth stating precisely: the non-zero exit came from wdio's own failure, not from the
`CONDUCTOR_A11Y_STRICT` arm. That handle was set as insurance against a vacuous green (an unresolved handle
would otherwise skip at exit 0); it was not needed, because the failure was louder than a skip.

**The control pair is what makes this a variation rather than an observation.** Same machine, same driver,
same runtime, same tree, same session, one dimension moved:

| field | non-elevated control | elevated |
|---|---|---|
| `IsElevatedAdmin` | `False` | **`True`** |
| `SessionId` | `1` | `1` |
| `UserInteractive` | `True` | `True` |
| `Win32_Process SessionId` | `1` | `1` |

The non-elevated known-good on this host is the release chunk's routine arm at **12 passing / 2 skipped** over
the shipped bundle, banner `[webview2 152.0.4191.66 windows]`, recorded 2026-09-10.

**Process census after the leg: zero orphans** — `msedgedriver` 0, `conductor-tauri` 0, `tauri-driver` 0,
`node` 0. The six `msedgewebview2` are the long-lived set the harness rule documents, not this leg's.

### What this establishes, and how far it reaches

**Elevation is the cause of the missing remote-debugging endpoint** — established by direct variation with a
control on both sides, on a host where the non-elevated case is a recorded green. Not by exhausting a list.

`hypothesis:` **WHY** WebView2 declines to bring up the remote-debugging endpoint under an elevated token is
**not measured here.** What is measured is the correlation under single-variable variation. The mechanism is
unprobed and is marked as a hypothesis deliberately, so no later reader mistakes it for a finding.

Scope: measured on this dev host. The hosted runner's elevation was measured independently
(`IsElevatedAdmin: True`, run `34586959536`), and the failure signature is identical across both — but the
variation itself was performed here, and that is the basis this record rests on.

## Arm B, first half — the elevated control capture

Elevation cannot be self-granted, so this arm is the operator's to run.

**First half, complete.** From an elevated dev-host PowerShell, the same probe script recorded
`IsElevatedAdmin: True`, `SessionId: 1`, `UserInteractive: True`, `IsSystem: False`,
`AuthenticationType: CloudAP`, against the non-elevated control's `False` / `1` / `True`. Elevation is
therefore **machine-proven as the only varied dimension** of that pair, rather than operator-attested.

**A first attempt produced no reading**, and is recorded rather than omitted: it ended during the release
build, after the frontend build completed, with its error invisible — `scripts/agent-run.ps1` reports through
`[Console]::Error.WriteLine`, which writes the raw console handle that a PowerShell `*>` redirect does not
capture, and the script's `exit` terminated the wrapper before any sentinel. Two host preconditions were
measured before the retry so it would not be spent on a preventable failure: `CONDUCTOR_MSEDGEDRIVER` is
persisted at User scope and resolves to a file (an elevated shell of the same user inherits it), while
`CONDUCTOR_A11Y_STRICT` is unset at every scope and therefore had to be set in-session. One hypothesis for the
first attempt's death — an unresolvable toolchain on the inherited PATH — was tested and **refuted**: both the
cargo and npm entry points are on the persisted Machine+User PATH, and the retry's own capture shows cargo
resolving and the release build completing in 22.82s. The proximate cause of the first attempt's death remains
undetermined; it is superseded rather than explained.

## What this does not establish

**The mechanism.** Why an elevated token suppresses the remote-debugging endpoint is unprobed. The record
carries that as `hypothesis:` and nothing downstream may promote it to a finding without measuring it.

**That the remedy works.** "Do not run the app elevated" follows from the cause, but no run has yet
demonstrated a limited-token launch producing the endpoint on a hosted runner. That is a measurement, not an
inference, and it belongs to the next arm.

**Anything about other hosts.** The variation was performed on this dev host. The hosted runner's elevation is
measured and its failure signature matches, which is what makes the result actionable there — but a
runner-side confirmation is still a separate measurement.

## The next arm — recorded, not designed here

`v3-02`'s terminal is no longer a candidate permanent exclusion. Hosted Windows runners run as an
administrator by construction, so the remedy to test is launching the app — or the whole wdio leg — under a
**limited token** (a scheduled task with a non-elevated principal, or a `runas`-class trust-level launch) and
re-measuring whether `DevToolsActivePort` appears. Recorded here as the entry's next arm; its design belongs
to whoever takes that entry up, not to this chunk.

## Not this chunk's

The `Rust gate` job is red in both runs on a pre-existing defect: the ledger gate's requirement-id filter is
keyed to a `v2-` prefix while this version declares `v3-` ids, so its own anti-vacuity assertion fires. The
gate's directory scan was deliberately generalised to survive the version transition; the id prefix was not.
906 tests run, 905 passed, 1 failed. It landed with the version directory, in the commit that also deferred the
gate that would have caught it, and it is owned outside this chunk.
