# Cause control — NVDA hears Conductor's webview focus only in its first burst

Chunk `2026-09-30-the-screen-reader-pass-grades-again-on-this-host` · chunk base `2c9b37d` · measured 2026-09-30.
Host paths and registry keys are described in words; utterances are quoted with any host path elided.

## Verdict — Branch B (the `= 2` arm was silent too)

**Under both of NVDA's object models, NVDA 2026.2 hears Conductor's webview focus during the window's first
burst and then no later focus change, on WebView2 runtime/driver 154.0.4258.37 / 154.0.4258.37 and Windows
26200.9457.** The cause is therefore NOT NVDA's Chromium object-model choice. The single-variable control
(`[UIA] allowInChromium = 2`, witnessed applied) moved NVDA from its in-process IA2 buffer to UIA, changed the
burst's wording, and changed nothing about the silence that follows it.

The step-5 predicate, read: under `= 2`, are E0-02, E0-03 and E0-04 all `announced-as-expected`? **No** — all three
(and E0-05, E0-06) are `not-announced`. So the leg's `nvda.ini` was restored byte-identical to `2c9b37d`
(`git diff --quiet 2c9b37d -- …/nvda-config/nvda.ini` exit 0), no regrade leg fired, and no row is graded a pass it
did not earn.

**Candidates still standing** (none isolated; this chunk varied none of them):
- the WebView2 runtime/driver pair, 152.0.4191.66 (last heard, 2026-09-07) → 154.0.4258.37, with 153.0.4234.48
  between — its runtime directory is still on disk (re-listed at step 1, created 2026-09-20);
- the two Windows cumulative updates, KB5124008 (2026-09-10, build 26200.9445) and KB5129195 (2026-09-15, build
  26200.9457) — not reversible as a control, so they stay a recorded, unisolated candidate;
- the desktop: the parallel pulse-builder session, the Edge browser 154 install (2026-09-26) and SearchHost's
  WebView2 tree (six processes, created 2026-09-26, present in every census below).

**What the control adds.** In both arms NVDA's log holds NO entry of any kind between the end of the burst and the
app's close — no focus event, no foreground event, no warning. Nothing about the stamped Tabs reached NVDA through
either API. This is consistent with focus events not being delivered from the webview/host after the first burst,
rather than with NVDA mis-handling events it receives; it is recorded as a reading, not an established mechanism.

## The USER consequence

An NVDA 2026.2 user on WebView2 runtime 154.0.4258.37 with Windows build 26200.9457 — at NVDA's defaults, and equally
with UIA forced for Chromium — hears Conductor's window, its titlebar, the coverage matrix and the current row once,
when the window first takes focus, and then hears **no focus change at all**: Tab to Minimize, Close, Start, the
coverage matrix or the report grid is silent. Live regions still speak (R0-01's load-error alert was heard, as were
the 2026-09-30 live subject's "Scenarios completed" and HOLD announcements). Measured through WebDriver-injected Tab
(the agent arm); a physical keyboard was not measured, and the first burst is itself driven by injected Tab, so the
injection path is not what separates heard from silent.

## The product-side lever — named, NOT built

Conductor sets no WebView2 browser arguments (`additionalBrowserArgs` / `additional_browser_args`: 0 hits over
`crates/conductor-tauri`, excluding `ui/node_modules` — research.md §Controls). So wry's default applies:
`--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection` (`wry-0.55.1/src/webview2/mod.rs:294-297`, handed
to `set_additional_browser_arguments` at `:327`). The product surfaces that could carry a lever are:
- Tauri's per-window `additionalBrowserArgs` in `tauri.conf.json` — it REPLACES wry's default, so any lever must
  re-include those three `--disable-features` entries;
- the WebView2 loader's additional-browser-arguments environment variable.

No Chromium argument that would restore focus events is nameable from upstream source (research.md: Chromium's
`accessibility_features.cc` on `main` defines only `kUiaDisconnectRootProviders`, `kUiaEventOptimization`,
`kUiaMathMlSupport`). Any such lever crosses the app boundary, so it waits for the founder's live word; this chunk
measured its existence by source read only and injected nothing.

## Configuration table

`allowInChromium`: `default` = no `[UIA]` section, which NVDA resolves to its in-process IA2 buffer for Conductor's
WebView2 document (the IA2 basis below); `2` = the `[UIA]` section present (UIA forced).

| run (UTC) | subject | runtime | driver | NVDA | OS build | `allowInChromium` | bundle | desktop | focus rows |
|---|---|---|---|---|---|---|---|---|---|
| 2026-09-07 22:06Z (heard) | error | 152.0.4191.66 | not recorded | 2026.2 | before KB5124008 (< 26200.9445; build not recorded) | default | `00181df` + the 09-07 `ui/src` fix (landed `06db2f9`) | a Conductor-session terminal repainting | R0-02..04 `announced-as-expected` |
| 2026-09-30 10:46Z | empty | 154.0.4258.37 | 154.0.4258.37 | 2026.2 | 26200.9457 | default | the prior chunk's candidate `ui/src` (committed as `4bab3c6`) | pulse-builder stopped | E0-02..06 `not-announced` (committed record, prior chunk) |
| 2026-09-30 10:52Z (base control) | empty | 154.0.4258.37 | 154.0.4258.37 | 2026.2 | 26200.9457 | default | base `7ee2fea` `ui/src` | pulse-builder stopped | E0-02..04 `not-announced`; fails at E0-05 by design (prior `driven-leg.md` Findings 2) |
| 2026-09-30 11:33Z | empty | 154.0.4258.37 | 154.0.4258.37 | 2026.2 | 26200.9457 | default | rebuilt at `4bab3c6` | no other WebView2 app open (census-before: two `node.exe`) | E0-02..06 `not-announced` (`nvda-pass.defaults.json` `subjects.empty`, recorded 11:34:05Z) |
| **2026-09-30 12:17Z — K-reproduce (slot 1)** | error | 154.0.4258.37 | 154.0.4258.37 | 2026.2 | 26200.9457 | default | rebuilt at `2c9b37d` (step 2) | pulse-builder in phase research, its terminal repainting; foreground only before activation and after close | R0-02, R0-03, R0-04 `not-announced`; R0-01 (live) `announced-as-expected` |
| **2026-09-30 12:19Z — K-object-model (slot 2)** | empty | 154.0.4258.37 | 154.0.4258.37 | 2026.2 | 26200.9457 | **2** | rebuilt at `2c9b37d` (step 2) | as slot 1 | E0-02..06 `not-announced`; E0-01, E0-07..09 browse `not-run-here`; E0-10 `subject-absent` |

Step-1 reads (2026-09-30T12:14Z): driver/runtime coherence entry printed `driver 154.0.4258.37 runtime
154.0.4258.37` (runtime from the EdgeUpdate client key `pv` value); machine-wide Evergreen runtime directories
`153.0.4234.48` (created 2026-09-20) and `154.0.4258.37` (created 2026-09-29); OS `CurrentBuild`.`UBR` =
26200.9457 (25H2); NVDA product version 2026.2, and both legs' logs open with `Starting NVDA version 2026.2` and
`Windows version: Windows 11 25H2 (10.0.26200.9457)`. Bundle: step 2's strict `--e2e` run relinked the release
binary at the chunk base (`Compiling conductor-tauri v0.2.0` … `Finished release`), and each record's
`build_commit` reads `2c9b37d…`.

## The silence shape

Times are UTC; NVDA's log clock is local (UTC+2) and was converted before pairing with `actions.*.jsonl`.

**2026-09-07 error, heard** (research.md §The silence's measured shape): activation, the load-error alert, the burst
("Conductor document" / "banner landmark Minimize window button" / "Start button unavailable"), "Landscape" ×2
(22:05:58–22:06:00Z), then one focus utterance per stamped Tab — "banner landmark Minimize window button"
22:06:02Z, "Close window button" 22:06:03Z, "Start … unavailable" 22:06:04Z.

**2026-09-30 11:33Z empty, default, silent** (research.md): the burst at 11:33:22Z, "Landscape" ×3
(11:33:23–11:33:24Z), then nothing until the app closed at 11:34:03Z; the stamped Tabs at 11:33:31/35/39/43/47Z fall
inside the gap.

**Slot 1 — K-reproduce, 12:17Z error, default, silent** (`runs/sr-leg/nvda-speech.error.log`, `actions.error.jsonl`):
- 12:17:10.1–12:17:19.8Z — NVDA reads the pulse-builder terminal, the foreground when NVDA started; at 12:17:19.1Z it
  logs "Foreground took too long to change … Should be … (Tauri Window)".
- 12:17:20.675Z — the load-error alert (R0-01, heard).
- 12:17:20.775–20.952Z — the burst: "Conductor document", "banner landmark", "clickable banner landmark Minimize
  window button", "button Minimize window", "Close window button", "main landmark", "Capability coverage matrix
  region", "Coverage rows table", "row current".
- 12:17:21.9–23.4Z — "Landscape" ×3.
- 12:17:24.1Z → 12:17:38.77Z — **no log entry of any kind**. The stamped Tabs R0-02 12:17:26.459Z, R0-03
  12:17:30.560Z, R0-04 12:17:34.636Z fall inside this gap. The terminal-output notifications that NVDA logs while
  the terminal holds focus stop at 12:17:20.2Z and resume only at 12:17:38.9Z, so no non-Conductor window took
  focus during the graded rows — the run is valid, not void.
- 12:17:38.80Z — the app closes (the IA2 teardown below); 12:17:38.854Z focus returns to the terminal.

**Slot 2 — K-object-model, 12:19Z empty, `= 2`, silent** (`runs/sr-leg/nvda-speech.empty.log`, `actions.empty.jsonl`):
- 12:19:44.889Z — NVDA reads a WebView2 host window whose name is the runtime executable's absolute path, then
  "terminal blank" (the pre-existing host console window of the prior chunk's `driven-leg.md` Findings 1 and 3; it
  falls outside every row window, so the committed record carries no host path and no `security_finding`).
- 12:19:45.912–46.657Z — the burst, in UIA's wording: "Conductor - Web content region", "clickable banner landmark
  – button" (the titlebar buttons named by their glyph, "–" and "✕", not by their `aria-label`), "Conductor
  document" plus its URL (scheme elided here), "main landmark Run controls grouping Start button unavailable",
  "Capability coverage matrix region", "Coverage rows table", "row", then the run-report table read as a table.
- No "Landscape" this run (it is ×2 on the heard 09-07 run and ×3 on the silent default runs, so it does not
  discriminate either way).
- 12:19:46.7Z → 12:20:27.6Z — **no log entry of any kind**. The stamped Tabs E0-02 12:19:54.928Z, E0-03
  12:19:58.995Z, E0-04 12:20:03.065Z, E0-05 12:20:07.163Z, E0-06 12:20:11.233Z fall inside this gap; no terminal
  notification either, so no focus theft — the run is valid.
- 12:20:27.6Z — the app closes; no `vbufBase` teardown is logged (UIA builds no in-process buffer).

**Side observation (not graded, not a finding of this chunk's intent):** if focus speech is restored under UIA by a
later remedy, the titlebar controls would be spoken by glyph ("–", "✕") rather than by name — a regrade under UIA
would read those rows `announced-differently`, not `announced-as-expected`.

## Witness that NVDA took the key (slot 2)

- (a) `runs/sr-leg/nvda-speech.empty.log` carries no configuration-error or validation line for `[UIA]`: a grep for
  `UIA|allowInChromium|validat|reset|config … error|invalid`, excluding NVDA's own console-UIA and UIA-handler
  module names, returns only the configobj version banner (`:16`) and the UIA MTA thread start (`:51-52`).
- (b) NVDA's exit ("Base configuration saved", `nvda-speech.empty.log:470`) rewrote the leg's working profile in its
  own serialisation (tab-indented) and that saved `[UIA]` section still holds `allowInChromium = 2`.
- (c) Behavioural, beside the two the plan asked for: the burst's wording changed to UIA naming, and the close logged
  no in-process `vbufBase` teardown, where the default arm logs one.

The arm is therefore a measurement, not void.

## The IA2 basis (default arm)

NVDA's in-process virtual-buffer backend runs inside `msedgewebview2.exe` only on the IA2 path; UIA browse mode
builds no in-process buffer.
- This chunk, slot 1 (default): `runs/sr-leg/nvda-speech.error.log:471-472` — `ERROR - RPC process … (msedgewebview2.exe)`
  / `build\x86_64\vbufBase\backend.cpp, VBufBackend_t::terminate, 239`, at the app's close (12:17:38.811Z).
- This chunk, slot 2 (`= 2`): no such line anywhere in `runs/sr-leg/nvda-speech.empty.log`.
- As research read them (both files since overwritten by this chunk's legs — `startNvda` removes the previous
  session's log, `wdio.conf.ts:207`): the 2026-09-30 11:33Z silent default run at `nvda-speech.empty.log:309-311`
  and the 2026-09-07 heard run at `nvda-speech.error.log:351-355` (research.md §Controls, K-object-model).
- NVDA's selection rule (research.md, `UIAHandler/__init__.py` `_isUIAWindowHelper`, master): for
  `Chrome_RenderWidgetHostHWND` at the default setting NVDA stays off UIA whenever the in-process approach and IA2
  access are available. `configSpec.py` (master): `allowInChromium = integer(0, 3, default=0)`, `0:default, 1:Only
  when necessary, 2:yes, 3:no`.

## The committed records

- `nvda-pass.defaults.json` — the working record snapshotted right after slot 1 (sha256 prefix `8dfd77f3818c27a1`).
  Its `error` subject is the K-reproduce arm (recorded 2026-09-30T12:17:40.227Z); its `empty` subject is the
  11:33Z default run and its `live` subject the prior chunk's 10:49Z run, both carried (the record is cumulative per
  subject, verification-harness.md 2026-09-07).
- `nvda-pass.json` — the working record as the `= 2` arm left it (sha256 prefix `41c37f2227726832`). Its subjects
  span THREE configurations and must not be read as one: `empty` = the `= 2` arm (2026-09-30T12:20:29.136Z),
  `error` = the default K-reproduce arm (12:17:40.227Z), `live` = the prior chunk's default run (10:49:56.719Z).

## Operator-entry results (fired by hand, once each, on the operator's word for the slot)

| plan entry | slot | `nvda.ini` state | fired (UTC) | exit | `Spec Files:` + TAB + ` 1 passed, 1 total` |
|---|---|---|---|---|---|
| 6 `npm run a11y:sr-error` | 1 — K-reproduce | byte-identical to `2c9b37d` (`git diff --quiet` exit 0 before firing) | 12:17:05Z → 12:17:40Z | 0 | held (1 line) — the DOM half only; the speech is graded by the rows above |
| 7 `npm run a11y:sr-empty` | 2 — K-object-model | `2c9b37d` plus the `[UIA] allowInChromium = 2` section alone | 12:19:31Z → 12:20:29Z | 0 | held (1 line) — the DOM half only |

Each ran with `CONDUCTOR_NVDA` set for the command alone to the one portable NVDA 2026.2 copy found on the host (a
bounded search of the user profile and tool roots); the handle is unset in this session's environment. Neither leg
was re-fired. Entries 8 and 9 (the live probe and the live `sr` leg) are Branch A only and did not fire; no `:4317`
slot was requested.

## Gate readings over the committed record (Branch B)

The plan's three regrade entries read red BY CONSTRUCTION — no regrade leg fires under Branch B:
- focus-row count: **24** non-`announced-as-expected` focus rows, where the plan forecast 21 for the 2026-09-30 record.
  The 24 are that record's 21 (17 `not-announced` + 4 `announced-differently`: S0-03, S2-02, S2-07, S3-04) PLUS
  R0-02, R0-03, R0-04 — heard on 2026-09-07 and carried in the old record, now `not-announced` because slot 1 refreshed
  the error subject at defaults. Nothing was widened to fit.
- S0-09 / E0-05 / E0-09: `S0-09 not-announced`, `E0-05 not-announced`, `E0-09 not-run-here` (browse-class, a
  recorded finding, never a pass).
- one-configuration `recorded_at`: `false` — the `live` subject is the prior chunk's 10:49:56Z run.

The closed-set / scrub-pairing entry and the S1-01 / `security_finding` entry both read `0`; the leg profile's diff
over `2c9b37d` is empty; the delta guard over `crates` / `scripts` / `contracts` / `scenarios` / `.github` / the Cargo
manifests prints nothing.

## Census

Taken with `Win32_Process` and parentage over the families `nvda`, `msedgedriver`, `conductor-tauri`,
`tauri-driver`, `node`, `msedgewebview2`, `pulse-app`, `andromeda-pulse-mcp`, before the work and after every leg.

| census (UTC) | rows | contents |
|---|---|---|
| before step 1, 12:14:55Z | 6 | six `msedgewebview2.exe`, parent chain SearchHost (created 2026-09-26) — not this session's |
| after step 2 (`--e2e`), 12:16:27Z | 6 | identical to the baseline |
| after slot 1 (`sr-error`), 12:18:53Z | 6 | identical to the baseline |
| after slot 2 (`sr-empty`), 12:21:18Z | 6 | identical to the baseline |
| final, after the full gate block (its `--e2e` re-run included), 12:25:32Z | 6 | identical to the baseline; the `tasklist` census entry reads 0 harness images |

| process | started by | final state |
|---|---|---|
| `tauri-driver`, `msedgedriver.exe`, `conductor-tauri.exe` and its `msedgewebview2.exe` children, wdio `node.exe` — step 2 | this run (the `--e2e` leg) | terminated (wdio `onComplete`; census equal to baseline) |
| `nvda.exe` — slot 1 | this run (the `sr-error` leg) | terminated (the harness's `nvda -q`; NVDA's own `NVDA exit` 12:17:39.5Z) |
| `tauri-driver`, `msedgedriver.exe`, `conductor-tauri.exe` + webview children, wdio `node.exe` — slot 1 | this run (the `sr-error` leg) | terminated |
| `nvda.exe` — slot 2 | this run (the `sr-empty` leg) | terminated (`nvda -q`; `NVDA exit` 12:20:28.5Z) |
| `tauri-driver`, `msedgedriver.exe`, `conductor-tauri.exe` + webview children, wdio `node.exe` — slot 2 | this run (the `sr-empty` leg) | terminated |
| six SearchHost `msedgewebview2.exe` | the OS (SearchHost, 2026-09-26) | left running — not this session's |
| `pulse-app`, `andromeda-pulse-mcp` | — | not started (Branch B fires no live leg); absent from every census |
