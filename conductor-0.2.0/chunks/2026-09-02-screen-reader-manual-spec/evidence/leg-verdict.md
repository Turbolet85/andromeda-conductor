# Leg verdict — the screen-reader pass, agent-driven (2026-09-02)

**Record:** `nvda-pass.json` beside this file (51 rows, one object per spec row of
`crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md`), produced by the `sr*` leg from NVDA's own
speech log and the leg's action timeline — never by hand. `operator_review` is **transcribed** (2026-09-02, see
§Operator review below): the review seam is closed, and `v2-23` flips to `verified` at wrap.

## Identity

| Field | Value |
|---|---|
| NVDA | 2026.2 (portable copy named by `CONDUCTOR_NVDA`; the leg's own profile: `silence` synth, auto say-all off, language switching off) |
| WebView2 Runtime | 152.0.4191.53 — the version the driver session attached to; the EdgeUpdate registry key read the same after a background update on this day (it read 151.0.4129.107 in the morning) |
| Build | commit `76550a4a0ade30b9a814ec2f684135d719d15e53`, release bundle `--features tauri/custom-protocol` |
| Live stimulus run | `2026-09-02T10-40-35-495` (persisted under the leg's own runs dir, two `ManualCheck` records: `halo-breathing-encoding` P-026 then `halo-hue-encoding` P-025, the run stopped before its third scenario) |
| Fixture stimulus | `lamps-fixture` (the committed `lamps-journal.jsonl`, re-seeded clean per session) |
| Sessions | `sr-empty` 10:42:59Z · `sr` 10:41:57Z · `sr-error` 10:44:50Z (UTC, the record's `subjects[].recorded_at`) — one leg version for all three |
| Attach | `live: true · empty: true · error: true` — NVDA bound to the driven WebView2 window in every final session |

## Outcomes

| Subject | announced-as-expected | announced-differently | not-run-here (browse class — finding) | subject-absent | not-run-here (by design) |
|---|---|---|---|---|---|
| live (37 rows) | 23 | 1 (S0-16) | 11 | 1 (S1-05) | 1 (T-01) |
| empty (10 rows) | 6 | 0 | 3 | 1 (E0-10) | 0 |
| error (4 rows) | 4 | 0 | 0 | 0 | 0 |
| **all (51)** | **33** | **1** | **14** | **2** | **1** |

Every `focus` and `live` row the agent arm drove was announced with the content the spec expects — the assertive
phase-line flips (`Conductor · live` / `HOLD — operator pause` / `aborted`), both operator-pause dialogs with title,
description and the focused `Abort`, the checklist row, the polite roll-up (`All observations confirmed`), the
`role="alert"` load error, every titlebar and run control with its state. The 14 `browse`-class rows are static
text NVDA reaches only through its browse-mode commands, which WebDriver-injected keys never deliver (measured on
every session: an injected `h` / `d` / arrow produced no speech while the same session's focus events did). They
are recorded `not-run-here` as FINDINGS — never passes, never hard failures — and, by the operator's review, there
is no manual arm for them in this chunk: OS-level key injection (`SendInput` / `SendKeys`, which NVDA's keyboard
hook sees) makes them agent-driven, owned by the route CARRY the operator's review names (B.2).

## Operator review (transcribed 2026-09-02)

The operator reviewed `nvda-pass.json` on 2026-09-02; the grades below are the operator's judgment and are
transcribed verbatim into `operator_review` (with `review_grade` on every row):

- The **33 `announced-as-expected`** agent rows — ACCEPTED as heard.
- **S0-16** (heard the focused option where the spec expected no announcement on appearance) — a FINDING, not a
  pass: the cause is initial-focus placement (focus lands in the picker input at load, so NVDA correctly announces
  the focused element); the spec's presentation-role expectation was written for an unfocused element. The row's
  expected text and the a11y-plan §5 initial-focus question ride wrap (finding #2 below).
- The **14 browse-class rows** — `not-run-here`, reason: browse-mode commands are unreachable by WebDriver-injected
  keys (measured every session); OS-level key injection makes them agent-driven — owned by the CARRY in B.2.
  Findings, never passes, never hard failures; no evening manual arm this chunk.
- The **2 `subject-absent`** rows and the **1 not-driven-by-design** row — ACCEPTED with their reasons.

## What the leg measured about the leg itself (and fixed before the final run)

1. **NVDA follows the OS foreground, not WebDriver's document focus.** A window launched by tauri-driver (a
   background process) never takes the foreground; the first sessions logged Pulse's window instead. The leg
   activates the app window (`activate-window.ps1`, a synthetic ALT to lift the foreground lock + `SetForegroundWindow`)
   before its first row — and `FindWindow` needs an `IntPtr` class argument, because a PowerShell `$null` bound to
   a `string` P/Invoke parameter arrives as `""` (exit 3 against a window that existed).
2. **NVDA binds a window on its first focus event, not on activation**, and only if it has fully started:
   readiness is the log's `NVDA initialized` line plus a quiet settle — a window created in the half-second after
   `Starting NVDA` was never bound (one whole subject went silent).
3. **The app's own focus start.** With a populated catalog the sequential-focus starting point sits at the picker
   input at mount, so the first Tab landed on `Start`; neither `blur()` nor a programmatic selection collapse moved
   it. Cycling Tab to the host-chrome stop (reads as `BODY`) before the first row does.
4. **Starting a run hands the foreground to a console window.** The sidecar spawn (`conductor-verify`, no creation
   flags) opens a Windows Terminal pane titled with the sidecar's exe path 200 ms after `Start`; NVDA announced it
   (`Terminal`, the path, `pane`) and every later focus row went silent until the leg re-activated the app window.
   The host path NVDA spoke is that pane's title — a **foreign window**, not text Conductor rendered — which is why
   `S1-01` carries a `security_finding` flag whose text says so; the alert prose itself carried no path (`R0-01`).
5. **NVDA's defaults flood the windows**: auto say-all on document load read the whole 83-row matrix into one
   row, and the `silence` synth spoke `English (not supported)` on every language change until the leg's profile
   turned both off.
6. **Paired events land a few milliseconds apart** (a state flip and the dialog it opens; the Escape resolution and
   the `Aborted` stage that follows it), so rows stamped together share one window.
7. **`drive_run` polls the abort flag only before a scenario starts.** A `Stop` during the LAST scenario ends in
   `Done` → `Conductor · idle`, never `Aborted` (heard on the 10:31 session); the trimmed catalog carries a third,
   never-executed scenario so the backend `Aborted` stage fires after the stopped one.

## Findings for wrap (product and plan)

- **The sidecar console steals the foreground on `Start`** (item 4). This is not a leg artefact: any keyboard or
  screen-reader user who presses `Start` loses focus to a terminal pane. Fix is in `conductor-verify`'s spawn
  (`CREATE_NO_WINDOW` / no console for the sidecar) — Rust, outside this chunk; a route entry's.
- **Initial focus sits in the picker input** when the catalog has entries, so a keyboard user's first Tab skips the
  titlebar controls (item 3).
- **Focusing a scroll region reads its whole content**: `Coverage rows` (83 rows) and `Run report rows` are read in
  one utterance on focus — the `tabindex=0` group carries the entire table as its accessible content.
- **Assertive phase-line flips cancel the focus-restore announcement**: after a hold resolves, `Conductor · live`
  (and, on the stopped run, `aborted`) is spoken and the restored `Start` sometimes only as
  `Run controls grouping Start button unavailable` (S2-07) — and on the Escape path the restore was not spoken at all
  in one session (S3-04 in the 10:36 run) while the DOM restore held.
- **A load-time `role="alert"` is silent until the window is bound** — the original load's error was never spoken;
  only a reload with NVDA tracking the window announced it (R0-01). For a user whose NVDA is already running the
  same first-focus binding applies.
- **`aria-current` is spoken as `current`**; the appended ` · selected` text reads only on re-navigation (S0-06,
  S1-03).
- **The picker's filter-miss prose (`role="presentation"`) is never announced** (S0-16, announced-differently:
  only the option list's re-announcement was heard).
- **The titlebar count is unlabeled and unannounced** (S0-12, S1-02 — operator arm).
- **Plan-vs-shipped**, unchanged from research: the shipped `RunState` set includes `aborted` and has no
  `report-terminal`; the report-site checklist does not ship; no footer / `contentinfo`, no `h1`; the dialog title
  is `{p_id} — operator-checklist`; the prose strings differ. The landmark and heading rows are `browse` and so on
  the operator arm.
- **`Stop` during the last scenario reads `aborted` then settles to `idle`** (item 7) — the label announces an
  abort the backend never records; a UX/semantic inconsistency for a route entry.
- **Hold order in a trimmed catalog is sorted-filename order** (P-026 before P-025) — the spec says so; the plan's
  narrative had hue first.

## Process census

Taken by the leg inside `onPrepare` (before NVDA starts) and `onComplete` (after NVDA quit and the driver kill),
recorded per subject in the JSON. Baseline before the whole leg (this session's shell, 10:00Z): `pulse-app.exe 50164`.

| Process | Before | After | Final state |
|---|---|---|---|
| `nvda.exe` | absent | absent | terminated by the leg (`-q`), every session |
| `conductor-tauri.exe` | absent | absent | terminated (`onComplete` → `tauriDriver.kill()`), every session |
| `msedgedriver.exe` | absent | absent | terminated with the driver tree |
| `andromeda-pulse-mcp.exe` | absent | absent | terminated with the app that spawned it |
| `node.exe` ×2 | present | present | the leg's own launcher and npm processes (same PIDs before and after) — not survivors |
| `pulse-app.exe` (50164) | present | present | **left running — operator stops it** |

Ports `4444` / `4445`: no LISTENING socket after the leg (checked before the first session; the driver releases them).

## Honest limits

One screen reader (NVDA 2026.2), one host (Windows 11, WebView2 152.0.4191.53), one leg version, one operator
review (transcribed). NVDA's wording is version-bound; the grade is on required tokens, the review is the judgment.
The `browse` class is not reachable through WebDriver-injected keys on this stack — 14 rows are findings owned by
the OS-level-key-injection CARRY, not passes and not a manual arm of this chunk. The `T-01`
(un-stopped run settles to `idle`) row was not driven, but the flip WAS heard once on the 10:31 session that
stopped during its last scenario, which is how finding 7 was measured. WCAG conformance is not claimed by this
pass; the automated baseline (`v2-22`) remains the conformance evidence.
