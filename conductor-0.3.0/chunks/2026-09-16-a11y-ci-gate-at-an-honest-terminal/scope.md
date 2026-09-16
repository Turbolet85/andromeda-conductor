# Scope — A11y CI gate at an honest terminal

**Marker:** `2026-09-16-a11y-ci-gate-at-an-honest-terminal`
**Version:** conductor-0.3.0 · Epoch 3 — The a11y capability's terminal
**Promoted:** 2026-09-16

## Intent (from the working-route entry, verbatim)

> A11y CI gate at an honest terminal — routine arm's asserted verdict green in CI, or a ratified
> exclusion naming its measured cause and owner

Two outcomes are allowed by construction, and the entry does not pre-select one. The chunk's job is to
reach whichever the evidence supports, and to leave the a11y capability's CI terminal **honest** — i.e.
the asserted verdict means what it says, rather than a job that is green because it asserts nothing or
red because nobody has ruled on it.

## What this chunk decides

The a11y routine arm has a registered CI job that **fails**, and the failure is not a defect in the
assertion — it is the runner refusing to give the app a debuggable WebView2 session. This chunk must
close that terminal one of two ways:

- **(A) Green arm** — change the app's launch context in CI so the WebView2 remote-debugging endpoint
  appears, the driver creates a session, and the routine arm's axe + contrast assertions actually run
  and pass. The CARRY names the arm to try (below).
- **(B) Ratified exclusion** — if (A) does not produce the endpoint, record the exclusion with its
  **measured** cause and a named owner, so the a11y capability's coverage claim rests on a stated,
  operator-ratified boundary rather than on a red job nobody has adjudicated.

Which one ships is the operator's call at the P5 review. This scope commits to attempting (A) first
because the CARRY names a live, untried arm; it does not commit to (A) succeeding.

## Measured state at take-up — CI run 35079315258

Read at promotion from the run itself (head `468f4d34`, pushed 2026-09-16T09:25:20Z, 11m38s,
conclusion `failure`). These are measurements from this run, not inherited claims.

**Job outcomes:** Rust gate `success` · Frontend gate `success` · A11y gate `failure`.

### The PREREQ is discharged and green

The Scenario-assertion audit gate registered by the previous chunk executed for the **first time** in
this run — Rust job step 13, `Scenario-assertion audit gate`, conclusion `success`. Its workflow
definition (`.github/workflows/ci.yml:99-103`) carries the ledger presence guard
(`test -f contracts/scenario-audit-ledger.toml` → `::error::` + exit 1) and then
`cargo nextest run -p conductor-core --test scenario_audit_gate --profile ci`. It carries **no**
`continue-on-error` key, so the GitHub default (`false`) applies and a red there would fail the build.
**No debt from that gate reaches this chunk.** The PREREQ is closed by this reading.

### The only red in the run is this chunk's own subject

A11y job step 9, `A11y routine arm (tauri-driver + axe + contrast)`, exit 1 after ~63 s
(`Spec Files: 0 passed, 1 failed, 1 total (100% completed) in 00:01:03`). **[premise-corrected at P3:
the take-up directive said "three session attempts"; the step's log carries ONE session-creation
failure event rendered on four lines at a single instant (09:33:43.564/.565/.615 + the reporter echo
at 09:33:44.03), wdio reporting `(1 retries)` — re-derived, see research.md]** The asserting step
carries no `continue-on-error` — the four
`continue-on-error: true` keys in the a11y job all sit on diagnostic/upload steps
(`WebView2 driver + runtime versions`, `WebView2 session isolation`, `WebView2 cause probes`,
`Upload session diagnostics`). Verbatim failure, the endpoint's URL scheme elided per the host-path
rule (the quote is otherwise byte-faithful):

```
session not created: DevToolsActivePort file doesn't exist when running
"[scheme]//127.0.0.1:4444/session" with method "POST"
```

then `FAILED in wry — ui/test/a11y/accessibility.e2e.ts (1 retries)` · `Spec Files: 0 passed, 1 failed`
· `error: wdio exited 1`. Downstream, step 13 (`A11y violation-JSON conformance gate`) was **skipped** —
so the conformance gate currently asserts nothing on a failing run.

### Causes measured AWAY in this same run

The run's own diagnostic steps (10–12, all `success`) eliminate every alternative explanation that had
standing. This is new information — it was not in the take-up directive — and it is what makes the
remaining hypothesis load-bearing:

| Candidate cause | Measurement in run 35079315258 | Verdict |
|---|---|---|
| Driver not resolving (the first run's defect) | `[precondition] CONDUCTOR_MSEDGEDRIVER: resolved` | eliminated |
| Driver/runtime/browser skew | msedgedriver `152.0.4191.66` · WebView2 Evergreen Runtime `152.0.4191.66` · Edge `152.0.4191.66` | eliminated — all three identical |
| Runtime below the floor | `[precondition] WEBVIEW2_RUNTIME_MAJOR: 152` | eliminated — floor met |
| Edge policy suppressing debugging | all five policy keys `ABSENT` (Edge · EdgeUpdate · EdgeWebView, machine and user hives) | eliminated |
| App crashing before it listens | `(a) app-alone HasExited=False ExitCode=n/a` | eliminated — app alive |
| WebView2 never starting | 6 live `msedgewebview2` processes; `EmbeddedBrowserWebView.dll` `152.0.4191.66`; `(a) bare-app EBWebView present: True` | eliminated — WebView2 runs |
| Port file written somewhere unscanned | `(b) fresh scoped_dir DevToolsActivePort present: False` · `(b) scoped_dir holding EBWebView: none` · `(c) LOCALAPPDATA app EBWebView present: False` | eliminated — absent everywhere checked |

### The failure is upstream of the driver — three arms, one signature

The run reproduces the same failure at three independent depths, which localizes it:

1. **In-path** (the real wdio arm): `session not created: DevToolsActivePort file doesn't exist`.
2. **Driver-alone** (`[diag] driver-alone status=500`): body is the *same* `session not created:
   DevToolsActivePort file doesn't exist` — so the failure does not need wdio.
3. **Bare-app, no driver at all**: `[diag] (a) bare-app DevToolsActivePort first seen: never within 90s`
   — while the app is alive and WebView2 is running.

Arm 3 is the decisive one: **the app never publishes the remote-debugging port in this environment even
when no driver is involved.** Any fix therefore has to change the app's *launch context*, not the
driver's configuration or the wdio harness. This also satisfies the bypass-probe discipline — the
driver-alone bypass is informative precisely because its in-path twin fails identically, rather than
standing alone.

### What remains standing

`[diag] (3) session identity` reports `SessionId: 2` · `UserInteractive: True` ·
`AuthenticationType: NTLM` · `IsSystem: False` · `IsAuthenticated: True` · **`IsElevatedAdmin: True`** ·
`Win32_Process SessionId: 2`. Elevation is confirmed present on the runner and is the only named
candidate the run does not eliminate. Also recorded: `[diag] TEMP and RUNNER_TEMP share a volume: False`.

## Folded annotations

The taken-up working entry carried two annotations at annotation positions (`PREREQ:`, `CARRY:`); no
`BLOCKED-ON:`. Both fold here. Their named coordinates were re-verified against the artifacts
themselves at promotion; their *mechanism* claims are hypotheses for P3 to close.

### PREREQ (folded — DISCHARGED at promotion)

> read the first CI run of the Scenario-assertion audit gate (registered 2026-09-16 at
> `2026-09-16-scenario-assertion-audit-gate`, never yet executed …); a red there is THAT gate's own
> debt, not this chunk's, and it is read before this entry's own CI work begins.

Discharged by the reading above: step 13 `success`. The annotation's own framing anticipated a possible
red; the measured outcome is green, so nothing carries forward. The annotation described itself as
TIME-triggered rather than delta-triggered — that trigger has now fired and is spent.

The annotation cites, as the precedent it guards against, the a11y gate's own first run going red on
`${{ env.EDGEWEBDRIVER }}` being empty in the workflow env context — "a defect observable ONLY on the
runner, which cost a blind iteration and a chunk." **That precedent is directly relevant to this
chunk's method** and is kept as a constraint below, not merely as history.

### CARRY (folded — live, and its premise survives today's run)

> this entry's terminal is NO LONGER a candidate permanent exclusion. The cause is established —
> elevation, by direct variation with a control on both sides: an elevated leg on the known-good dev
> host reproduces `session not created: DevToolsActivePort file doesn't exist` where the non-elevated
> case is a recorded green, the pair differing on `IsElevatedAdmin` alone (2026-09-12,
> `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/evidence/reading.md`).
> Hosted Windows runners run as an administrator by construction, so the NEXT ARM is a limited-token
> launch of the app — or of the whole wdio leg — via a scheduled task with a non-elevated principal or
> a `runas`-class trust level, then a re-measurement of whether the endpoint appears. Not designed here
> (operator directive, 2026-09-12 wrap). **The MECHANISM behind the cause is recorded, not established**

Coordinate re-verified: the cited evidence file exists
(`conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/evidence/reading.md`, 13 904 B).

- **VERIFIED at P3 — the causal claim, marker text preserved verbatim: "The cause is established —
  elevation, by direct variation with a control on both sides … The MECHANISM behind the cause is
  recorded, not established."** Re-derived at HEAD against the cited evidence: `reading.md:92` grades
  elevation ESTABLISHED by variation; `:137-147` tables the control pair differing on `IsElevatedAdmin`
  alone with the non-elevated known-good at 12 passing / 2 skipped. Run 35079315258 is a third
  independent confirmation (elevation present; every rival cause eliminated) and narrows where the cause
  acts — at the app's own WebView2 debugging initialization, per bare-app arm 3 above, not at the driver
  handshake. **The MECHANISM stays unestablished** (`reading.md:158` marks it `hypothesis:`), so the
  plan states the cause and never the mechanism.
- **VERIFIED at P3 — the next arm is a limited-token launch**, a scheduled task with a non-elevated
  principal or a `runas`-class trust level, applied to the app or to the whole wdio leg, followed by
  re-measurement of whether the endpoint appears. Explicitly **not designed** by the route entry; the
  design is this chunk's work. Its cost is now *earned*: the prior chunk rejected de-elevation for
  itself on the stated condition that "it earns its cost only if arm B leaves elevation standing"
  (`2026-09-11-hosted-runner-endpoint-cause-closed/plan.md:371-375`), and arm B left it standing. The
  same line names the inherited risk — a token dance has failure modes that must be separated from the
  endpoint's.

## Two obligations folded into `v3-02` — added at P5 validation-1

Discovered at P4 when the capability pool was read; the working-route entry does not carry them and the P1
scope therefore missed them. Both were folded into `v3-02`'s `notes` at the 2026-09-11 route Phase 4 on
operator direction, and both are this chunk's to discharge.

1. **WebView2 runtime pin disposition.** The CI-fetched Evergreen bootstrapper carries ONE stated exit
   condition in security-plan's third dependency class — the a11y job actually gating — and **this chunk's
   terminal fork IS that exit condition**. So whichever way the terminal lands, the float is dispositioned in
   the same chunk: pinned if the job gates, retired if the CI half is excluded. (The posture became
   floor-conditional on 2026-09-12; the pin-once-it-GATES disposition stands and is still unmet.)
2. **Landmark coupling to an unbuilt surface — MEASURED at P4, as the note demanded.** The console footer
   `contentinfo` strip is designed but not shipped, and a11y-plan bans a landmark-less window, so the note
   required measuring whether the routine specs pass against the console *as it stands* before promising a
   green terminal. They do: **12 passing / 2 skipped** on the non-elevated dev host (2026-09-10), and the
   console is unchanged since — `git log --oneline --since=2026-09-10 -- crates/conductor-tauri/ui/src
   crates/conductor-tauri/ui/test` returns 0 commits and `grep -rn 'contentinfo' crates/conductor-tauri/ui/src`
   returns 0 hits. The unshipped footer is therefore **not** a second cause of red today; the arm passed
   without that landmark.

## Boundaries / non-goals

- **Not re-chasing the first run's defect.** `CONDUCTOR_MSEDGEDRIVER` resolves; the empty
  `${{ env.EDGEWEBDRIVER }}` is fixed and is not the current cause.
- **Not widening the trust boundary.** The dev-only driver ports (`4444`/`4445`) live and die with the
  harness; nothing here opens an inbound listener in a shipped binary. A limited-token launch mechanism
  is a CI-job and harness-edge concern, not a change to shipped binaries.
- **Not touching PULSE's UI.** Driving Conductor's own webview remains in-scope self-verification.
- **Not re-opening `2026-09-11-hosted-runner-endpoint-cause-closed`.** Its reading stands; this chunk
  acts on the arm that chunk named.
- **Not a scenario/verification-corpus change.** Epoch 2's subject is closed; this is the a11y terminal.
- **VERIFIED at P3 — no new third-party dependency.** `schtasks` / `runas` / `Start-Process -Credential`
  are OS built-ins, so this arm does not enter security-plan's third dependency class (a CI-time-fetched
  binary) and needs no Authenticode gate. Should a candidate approach require one after all, it crosses
  that class — surface rather than absorb.

## Surfaces and contracts this touches

- `.github/workflows/ci.yml` — the `a11y` job: step 9 (the asserting routine arm) and, if (B), the
  ratified-exclusion form. The diagnostic steps 10–12 and their `continue-on-error: true` posture are
  existing and load-bearing for evidence.
- `crates/conductor-tauri/ui/test/a11y/` — `accessibility.e2e.ts` and the wdio config that spawns
  tauri-driver; the launch-context change may land at this edge.
- `scripts/webview2-cause-probe.ps1` and the session-isolation diagnostic script — the instruments that
  produced the evidence above; they already carry the registry-provider-form host-path discipline.
- `.andromeda/a11y-plan.md` §3 (the a11y harness) and `.andromeda/test-plan.md` §9/§10 (CI integration /
  quality gates) — the specs that say what the terminal must assert.
- `conductor-0.3.0/verification-matrix.json` — the a11y capability whose terminal this is.
- `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` env-handle discipline (security-plan §Input Validation):
  discharged at the wdio harness edge, not in Rust. A new launch handle would inherit that duty.

## Method constraints carried from the PREREQ's precedent

The a11y gate's first run went red on a defect **observable only on the runner**, costing a blind
iteration and a chunk. Both candidate outcomes here depend on runner-only behaviour, so the plan must
minimise blind iterations: prefer a change whose effect the existing diagnostic steps (10–12) can
already report, and treat each CI run as a measurement to read in full rather than a pass/fail to
retry. A chunk that SHIPS a CI gate cannot prove that gate at its own wrap — the job's first run
necessarily follows the commit and the push — so the acceptance wording must anticipate a
met-in-mechanism-locally form with the CI run named as the operator's act.

## Open premises for P3 — closed, see `research.md` §Premise closure

1. **ANSWERED — no, and it need not.** The mechanism stays `hypothesis:`-marked at its source and is not
   derivable from this repo; P3 established the CAUSE (elevation, now on three independent measurements)
   and ruled that the plan states the cause and never the mechanism. Arm 3 narrows where it acts.
2. Is a non-elevated principal reachable on a hosted `windows-2025` runner, and does the app survive
   the token drop (working directory, `RUNNER_TEMP`/`TEMP` on different volumes per the diagnostic,
   profile access for `EBWebView`)?
3. Does any supported WebView2 additional-browser-argument path (`--remote-debugging-port`) offer a
   route that does not require dropping the token at all?
4. If (B): what is the exclusion's exact shape — which assertions stay asserted, what the a11y
   capability's matrix acceptance then claims, and who owns it.
