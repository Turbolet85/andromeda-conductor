# Probe verdict — hosted-runner WebView2 session

**Probe run:** 34234558853 · ref `ci-probe/2026-09-08-hosted-runner-webview2-session` (deleted after reading)
**Image:** `windows-2025` · **A11y job:** 2026-09-08T13:51:26Z → 14:03:24Z (11 m 58 s) · **conclusion:** failure
**Versions on the runner:** msedgedriver `151.0.4129.101` · WebView2 Evergreen Runtime `151.0.4129.101` ·
Edge `151.0.4129.101` — all three identical, no skew.

Job step outcomes: routine arm **failure** · session-isolation diagnostic **success** · session-diag upload
**success** · conformance gate **skipped** (its predecessor failed). Rust gate ✓ and Frontend gate ✓ on the
same commit.

## The finding

**The WebView2 runtime on the hosted runner never opens its remote-debugging endpoint.** This is upstream of
`msedgedriver` and upstream of `@crabnebula/tauri-driver`: it reproduces with **no driver in the picture at
all**.

Probe (a) is what establishes it. A bare launch of the release binary, with `WEBVIEW2_USER_DATA_FOLDER` set
and `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=0`:

- the loader **did** honour the handle — `EBWebView` was created under the given folder (`present: True`);
- the app **stayed alive** for the whole window (`HasExited=False`);
- and `DevToolsActivePort` **never appeared within 90 seconds**.

The identical form on the dev host produces the port file in **1 second**. So the app starts, the profile is
created where it was told, and the debugging endpoint simply never comes up.

Everything previously observed downstream follows from this: msedgedriver waits its 60 s, finds no port file,
and returns `session not created: DevToolsActivePort file doesn't exist`; tauri-driver relays it. That is why
the failure was byte-identical through the intermediary and direct — **the driver was never the
discriminator**, and no driver swap (H1) or user-data-folder handling (H2) can reach it.

## Measurements

| Probe | Hosted runner | Dev host (same logic, known-good) |
|---|---|---|
| `TEMP` / `RUNNER_TEMP` share a volume | **False** | True (artifact of the local substitution) |
| **(a)** bare-app `DevToolsActivePort` first seen | **never within 90 s** | **1 s** |
| **(a)** bare-app `EBWebView` present | **True** | True |
| **(a)** app-alone `HasExited` | False (stayed up) | False (stayed up) |
| **(b)** driver-alone `POST /session` | **500 — `session not created: DevToolsActivePort file doesn't exist`** | **200** (0.65 s) |
| **(b)** fresh `scoped_dir` first seen / count | **never / 0** | 1 s / 2 |
| **(b)** any fresh `scoped_dir` `EBWebView` | **False** | True |
| **(b)** any fresh `scoped_dir` `DevToolsActivePort` | **False** | True |
| **(c)** `LOCALAPPDATA` app `EBWebView` present | **False** | True (long-lived host) |
| **(d)** staged diagnostic files | 5 | 5 |

Reading notes:

- **(b) `count: 0`** — the poll saw no fresh `scoped_dir` under `$env:TEMP`. **This is NOT evidence that the
  driver created none, and the earlier reading of it as "consistent with (a)" is RETRACTED.** The SR chunk's
  runner verbose log shows msedgedriver populating `Preferences`, so a profile directory demonstrably existed
  on that runner. `hypothesis:` the probe watched the wrong directory (`TMP` vs `TEMP` differ for the step's
  shell), or the driver removed the dir inside the 1-second poll interval. Unmeasured either way — this line
  is a finding to chase, never a conclusion to build on. On the dev host the same driver creates **two**
  `scoped_dir` siblings per session, of which only one holds the profile.
- **(c) False** — the app did **not** fall back to its default profile. It used the folder it was given, which
  is further confirmation that folder handling is not the fault.
- **TEMP / RUNNER_TEMP on different volumes** is recorded as a runner fact. It is *not* implicated: probe (a)
  used `RUNNER_TEMP` throughout and still created `EBWebView` there successfully.

## What this retires

- **H1 (official `tauri-driver` crate)** and **H5 (`tauri-plugin-automation`)** — both are driver-layer
  changes, and the fault is measured upstream of every driver. No longer worth their operator ratification.
- **H2 (`WEBVIEW2_USER_DATA_FOLDER`)** — already retired at the P5 review as not-a-lever; probe (a) now shows
  the handle is honoured on the runner *and* the port file still never appears, so it is doubly out.
- **H3 as DRIVER-skew** — the runner's driver, runtime and Edge are all `151.0.4129.101`, self-consistent, and
  probe (a) has no driver in it at all. The driver is not the discriminator.

## H3 re-opened on the RUNTIME axis (operator correction, 2026-09-08)

The dev-host driver has been `152.0.4191.53` since **2026-09-02 14:45** (`msedgedriver-151.0.4129.101.exe` is
kept beside it, dated 2026-08-20). So every dev-host run since — the SR chunk, `a11y-ci-gate`, today's probes,
and 46 `[webview2 152.0.4191.66 windows]` banners in `runs/a11y-e2e.log` — drove **runtime 152 with driver
152**. "Driver 151 drove runtime 152 green" was the **2026-09-02 morning** measurement (the test-plan sidecar's
own "the WebView2 update mid-day"), true then, and NOT a description of the SR chunk's runs.

Stated as the set this actually establishes:

| | Runtime | Driver | Outcome |
|---|---|---|---|
| Dev host, 2026-09-02 morning | 152.0.4191.x | 151.0.4129.101 | pass |
| Dev host, since 2026-09-02 14:45 | 152.0.4191.x | 152.0.4191.53 | pass |
| **Hosted runner** | **151.0.4129.101** | 151.0.4129.101 | **fail** |

Every passing case runs **runtime 152**; the single failing case runs **runtime 151**. So **H3 stays rejected
as driver-skew and is OPEN as a runtime-version question** — the runtime major is the strongest remaining
software variable, and it is the one this probe did not vary.

## What remains unmeasured

**Why** the runtime does not open the endpoint. Candidates not discriminated by this probe — none measured,
none to be inherited as a cause: the runtime-152-vs-151 difference above (the leading one); no interactive
desktop session on a hosted runner; a Session-0 / service-account restriction on the WebView2 browser process;
an image policy on remote debugging. The next attempt varies the RUNTIME, not any driver-layer option.

**The next probe is the runtime one:** install WebView2 runtime 152+ in-job (signed Evergreen bootstrapper,
signature checked before it runs) and re-run probe (a). That varies the single variable every passing case
shares and the failing case lacks, and it needs no driver change at all. **H4 (self-hosted Windows runner)**
remains route-forward only — it would also change the session/desktop context, but no such runner exists for
this project (operator-confirmed 2026-09-08).

## Provenance

All figures read from run 34234558853's job log and, for the dev-host column, from a validation script
**regenerated from `.github/workflows/ci.yml`** with two substitutions (`RUNNER_TEMP`→`TEMP`,
`EDGEWEBDRIVER`→`CONDUCTOR_MSEDGEDRIVER`), so the shipped logic is what produced the known-good numbers. No
absolute host path appears in this record.
