# Hosted-runner WebView2 endpoint — probe reading

**Capability:** `v3-01` · **Chunk:** `2026-09-11-hosted-runner-endpoint-cause-probed`

## Run identity

| field | value |
|---|---|
| run id | `34586959536` |
| event | `push` |
| branch | `build/conductor-0.3.0` |
| commit | `b7ef1bd7` |
| run conclusion | **failure** |
| job | `A11y gate (routine arm · axe · contrast · violation JSON)` — **failure** |
| job | `Rust gate (build · test · lint · supply-chain · coverage)` — success |
| job | `Frontend gate (npm audit · build)` — success |
| probe step | `WebView2 cause probes (diagnostic)` — ran, `[probe] complete: 3/3 sections` present exactly once |
| image | hosted `windows-2025` |

The run's red is the **standing WebView2 session-creation failure**, not this chunk. The `A11y routine arm`
step failed with `session not created: DevToolsActivePort file doesn't exist`. The probe step is
`continue-on-error` and cannot move the job's verdict; the two jobs this chunk did not touch both succeeded.

Control values are from `scripts/webview2-cause-probe.ps1` run on the dev host — the same script, so the
comparison is measured on both sides rather than approximated on one. On that host the endpoint appears in
about a second.

### Incidental: the driver-bind claim does not reproduce on this run

`.andromeda/residuals.md` records that the routine arm's red had *moved* to tauri-driver never listening on
`4444` — "four refused POSTs inside ~1.5 s against a 6.9 s driver bind". **That is not what this run shows.**
Measured from the job log: the driver was listening and accepted the connection, and the failure is the
endpoint, twice over.

| stamp | event |
|---|---|
| `10:09:50.296Z` | `@wdio/utils: Connecting to existing driver at 4444` — connected, no refusal |
| `10:09:50.314Z` | `POST /session` |
| `10:10:50.734Z` | `WARN webdriver: session not created: DevToolsActivePort file doesn't exist` (60 s later) |
| `10:10:50.734Z` | retry `POST /session` |
| `10:11:50.302Z` | `ERROR webdriver: operation aborted due to timeout` (a further 60 s) |
| `10:14:27.377Z` | driver-alone probe, bypassing tauri-driver: same `session not created: DevToolsActivePort file doesn't exist` |

There is **no `ECONNREFUSED` anywhere in this run's log**. A bind race is intermittent by nature, so this does
not prove the earlier observation wrong on the runs it was taken from — but it does mean the routine arm's red
is currently the endpoint, not the bind, and `v3-02` should re-measure rather than inherit the residual's
framing. Recorded here because it bears on attributing this run's red.

---

## (1) Policy state — **reading matches the control; candidate RETIRED**

| key | hosted runner | dev host (control) |
|---|---|---|
| `HKLM\SOFTWARE\Policies\Microsoft\Edge` | ABSENT | ABSENT |
| `HKLM\SOFTWARE\Policies\Microsoft\EdgeUpdate` | ABSENT | ABSENT |
| `HKLM\SOFTWARE\Policies\Microsoft\EdgeWebView` | ABSENT | ABSENT |
| `HKCU\SOFTWARE\Policies\Microsoft\Edge` | ABSENT | ABSENT |
| `HKCU\SOFTWARE\Policies\Microsoft\EdgeUpdate` | ABSENT | ABSENT |

**What this establishes.** No Edge / EdgeUpdate / EdgeWebView policy is present on either side, so no policy
difference exists that could explain a behavioural difference. This **retires the leading candidate** —
`architecture.md` §Established Decisions [CI/CD] named "a hosted-image policy or session property" as the
leading unmeasured pair, and the policy half is now measured and falsified.

## (2) Loaded module versions — **NO READING; the candidate stays UNMEASURED**

| process | hosted runner | dev host (control) |
|---|---|---|
| `msedgewebview2` | no live process | 6 live; `msedgewebview2.exe` and `msedge_elf.dll` at FileVersion `152.0.4191.66` |
| `conductor-tauri` | no live process | no live process |
| `msedge` | no live process | no live process |

**Recorded as having returned nothing, not omitted.** The probe found nothing to inspect, and the cause is
this probe's **placement**, measured from the job log rather than inferred:

- The `WebView2 session isolation (diagnostic)` step launches the app, observes it, and then stops the
  processes it started. Its `tasklist` at `10:13:21.76Z` shows `conductor-tauri.exe` (pid 8984) and three
  `msedgewebview2.exe` processes alive, all in session 2, with `app-alone HasExited=False`.
- Its final line is stamped `10:14:58.9421153Z`; the cause-probe's completion marker is `10:14:59.5867824Z` —
  **0.645 s later**, after that step's teardown. (Its first line is `10:14:59.5824028Z`, 0.640 s after.)

So the webview host was alive during the preceding step and gone by the time this probe ran. The reading is
honest and the candidate is simply **not yet measured**: what module the host processes actually load on the
hosted image remains unknown. The fix is placement — probe while the app is alive — not a different probe.

## (3) Session identity — **reading DIFFERS from the control on two axes**

| property | hosted runner | dev host (control) | same? |
|---|---|---|---|
| `SessionId` | **2** | 1 | differs (both non-zero) |
| `Win32_Process SessionId` | **2** | 1 | differs (both non-zero) |
| `UserInteractive` | **True** | True | same |
| `AuthenticationType` | **NTLM** | CloudAP | **differs** |
| `IsSystem` | False | False | same |
| `IsAuthenticated` | True | True | same |
| `IsElevatedAdmin` | **True** | False | **differs** |

**What this establishes, and what it does not.**

- **The service-context hypothesis is FALSIFIED.** The runner is **not** session 0 and **is**
  `UserInteractive`, with a window station; `tasklist` independently shows the app and its webview children
  on "Console" in session 2. A non-interactive service session was the other half of the leading candidate
  pair, and it does not hold.
- **A new candidate is named that no prior document mentions: elevation.** The hosted runner runs
  **elevated** (`IsElevatedAdmin: True`) where the dev host does not. This is the one substantive,
  controllable difference the three probes surfaced. It is a **lead, not a finding** — nothing here tests
  whether de-elevating opens the endpoint.
- `AuthenticationType` differs (NTLM vs CloudAP). Recorded because the acceptance requires every reading
  recorded; no mechanism is claimed for it.

---

## Summary against the three candidates

| candidate | status after this reading |
|---|---|
| WebView2 / Edge policy state on the image | **RETIRED** — measured absent on both sides |
| Session or service-account property | **PARTLY RETIRED** — session 0 and non-interactive both falsified; **elevation** newly named and untested |
| Module version the host processes loaded | **UNMEASURED** — probe returned nothing; cause is probe placement, measured |

## Remediable or permanent — **not decided by this reading**

Stated plainly rather than rounded up. The three readings retire two documented candidates and name a third
that is controllable, which points toward *remediable* — but the question is not settled:

1. the module-version candidate returned nothing, so it is neither retired nor confirmed; and
2. elevation is a difference, not a demonstrated cause — no probe here varied it.

A follow-up that (a) moves the probe to a point where the webview host is alive and (b) drives the app
de-elevated would close both gaps. Both are cheap and neither needs new machinery.

## Reading discipline

No green, conformance, or platform-capability claim is drawn from any probe. No reading is generalised beyond
the image and run it was taken on. Jobs and steps are identified by name throughout; no `ci.yml` line
coordinate is cited, since adding this chunk's step moved every coordinate below it.
