# Leg verdict — the SR findings re-run, agent-driven (2026-09-04)

**Record:** `nvda-pass.json` beside this file (51 rows), produced by the `sr*` leg from NVDA's own speech log
and the leg's action timeline — never by hand. `operator_review` is EMPTY: this record carries the agent arm's
grading only, and the review seam is open.

Run under the operator's post-implement directive (2026-09-04): a live Pulse launched by the overseer at
06:06Z (deterministic L4 + MCP), `CONDUCTOR_NVDA` supplied, and the plan's deferral of findings 3 and 8
withdrawn.

## Identity

| Field | Value |
|---|---|
| NVDA | 2026.2 (portable copy named by `CONDUCTOR_NVDA`; the leg's own profile) |
| WebView2 Runtime | 152.0.4191.53 (session and EdgeUpdate registry agree) |
| Build | commit `ff08206a5417` **plus this chunk's uncommitted working tree** — the record's `build_commit` names the commit, not the tree under test |
| Bundle | `cargo build --release -p conductor-tauri --features tauri/custom-protocol`, rebuilt after the last source change |
| Sessions | `empty` 06:43:58Z · `error` 06:44:13Z · `live` 06:58:05Z (UTC) |
| Attach | `live: true · empty: true · error: true` |
| Live stimulus run | `2026-09-04T06-56-42-321` (persisted under the leg's own runs dir) |
| Catalog | the trimmed 3-scenario catalog via `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios` |

## Outcomes

| Subject | announced-as-expected | announced-differently | not-run-here (browse — finding) | subject-absent | not-run-here (by design) |
|---|---|---|---|---|---|
| live (37) | 24 | 0 | 11 | 1 (S1-05) | 1 (T-01) |
| empty (10) | 6 | 0 | 3 | 1 (E0-10) | 0 |
| error (4) | 4 | 0 | 0 | 0 | 0 |
| **all (51)** | **34** | **0** | **14** | **2** | **1** |

Against the 2026-09-02 baseline (33 / 1 / 14 / 2 / 1): **`announced-differently` is now zero** and the
`findings` array fell 16 → 15. The row that left is **S0-16** — finding 6.

## The eight findings

| # | Finding | Verdict | Evidence |
|---|---|---|---|
| 1 | initial focus skips the titlebar | **NOT FIXED — defect confirmed by measurement** | `tabs_to_start`: live **3**, empty **6**, error **5** |
| 2 | scroll region reads its whole content | **FIXED** | S0-09 / E0-05 / E0-06 announced-as-expected |
| 3 | assertive flip cancels focus-restore | **FIXED** | S2-07 and S3-04 both speak the restore |
| 4 | load-time alert silent until reload | fix landed, **NOT discriminated** | the leg still reloads |
| 5 | `aria-current` speaks only "current" | **FIXED** | S0-06 / S1-03 speak " · selected" |
| 6 | filter-miss prose never announced | **FIXED** | S0-16 announced-as-expected; left `findings` |
| 7 | titlebar count unlabeled/unannounced | fix landed, **rows not gradeable**; audible elsewhere | "Scenarios completed: 0/1/2" in four windows |
| 8 | Stop announces abort then settles to idle | **FIXED (client half measured)** | S3-05 ends "Conductor · aborted" |

### 1 — initial focus: confirmed, still unfixed

`initial_focus` reads `BODY` on all three subjects, as it did on 2026-09-02 — it samples `activeElement`, and
the defect lives in Chromium's separate sequential-focus starting point. `tabs_to_start`, surfaced through
`writeNvdaPass` by this chunk precisely to discriminate it, is the measurement: the **live** subject reaches the
host-chrome stop in **3** Tabs, while the two picker-less subjects need **6** and **5**. A walk beginning at the
document start costs a full cycle; three does not. So with a populated catalog the walk begins partway through
the order, and a user's first Tab skips the titlebar controls — the route entry's claim, now measured rather
than inferred.

The mechanism remains unidentified and **nothing was changed on a guess**: cmdk 1.1.1 contains no `autofocus`
of any spelling, both its `.focus()` calls are guarded on focus already being inside cmdk, and no `src/` file
passes `autoFocus` or focuses at mount. The leg records the COUNT, not the element that held the starting
point; naming it needs one more measurement.

### 2 — scroll regions

Heard at S0-09: `Capability coverage matrix region` → `Coverage rows table with 83 rows and 4 columns` →
`Coverage rows row 1 column 1 P-ID column 2 Capability …`. That is the standard table summary plus the header
row, not the 83 rows in one utterance the operator heard on 2026-09-02. E0-06 the same shape at
`Run report rows table with 4 rows and 6 columns`. The token grade alone does not prove this — the heard text
does, which is why it is quoted here.

### 3 — focus restore after a hold

Both paths now speak the restore, and the phase line no longer preempts it:

- **S2-07 (Proceed):** `Proceed button` → `Conductor · live` → … → `Run controls grouping Start button unavailable`
- **S3-04 (Escape → NoGo):** `Conductor · live` → … → `Run controls grouping Start button` → `Conductor · aborted`

On 2026-09-02 the Escape path spoke no restore at all in the 10:36 session. The hold fired from the in-run
preflight canary, so these are live-Pulse rows the plan had deferred.

### 4 — load-time alert: landed, not discriminated

The alert region is now mounted empty at first paint, so the error arrives as a change. **This run does not
prove it:** the leg's R0-01 action still reloads the document before asserting, so a post-reload announcement
and a first-load announcement are indistinguishable here. The spec row's stimulus and the row's `expected` were
corrected to say so — an earlier edit of mine had claimed "no reload is needed", which the leg does not
demonstrate. Proving it needs the leg's R0-01 action to drop the reload; that is a follow-on, not a fix.

### 5 — selection in the accessible name

S0-06 heard `Suite — all scenarios · 3 scenarios · selected`; S1-03 heard `… · selected current`. The option's
`aria-label` carries the selected state, so it is announced as the option becomes current instead of waiting
for a re-read. `aria-current` still speaks as "current", which is correct and unchanged.

### 6 — filter-miss prose

S0-16 heard `No scenarios match.` — announced-differently → announced-as-expected, and the row left the
`findings` array. The persistently-mounted `aria-live` region replaced cmdk's `Command.Empty`, whose
`role="presentation"` is set after the prop spread and cannot be overridden.

### 7 — titlebar count

Its own two rows (S0-12, S1-02) are browse-class and stay `not-run-here` findings — per test-plan §1 that class
is never a pass and never a manual arm, whatever the fix. But the fix is **audible in other rows' windows**:
`Scenarios completed: 0` (S1-01), `Scenarios completed: 1` (S2-07), `Scenarios completed: 2` (S3-04/S3-05). The
count now announces as it advances, under a name. Recorded as corroboration, not as those rows' grade.

### 8 — Stop announces an abort the backend contradicted

S3-05's window ends `… Run controls grouping Start button` → `Scenarios completed: 2` → **`Conductor · aborted`**,
with no settle to `idle` after it. Two honest bounds: the leg's S3-01 stops during scenario **2 of 3**, so it
exercises `drive_run`'s pre-existing loop-head poll, not the post-loop poll this chunk added; and **T-01** (the
un-stopped last-scenario case the post-loop poll exists for) is `not-run-here` by design. That specific case is
covered at the unit tier by `drive_run_reports_an_abort_raised_during_the_last_scenario`, not by this leg.

An intermediate run measured the opposite failure and is why the shipped shape is what it is: with the client's
immediate `setRunState('aborted')` removed, the phase line still read `live` at the S3-01 bound, because the
backend polls only between scenarios. The fix is both halves — the client announces at once, and the backend's
post-loop poll makes that announcement true rather than overwriting it.

## S1-01 — a host path was heard, and it is the sidecar console

The record flags `S1-01` with its `security_finding` prompt. The window reads `<host-path>` →
`<host-path> terminal blank` → `pane`. **The verdict is: a FOREIGN WINDOW TITLE, not text Conductor rendered** —
the sidecar spawn's console pane taking the OS foreground ~200 ms after Start, as measured 2026-09-02. It is
therefore **not** a finding against the `sanitize_error` edge, and it is owned by the next route entry
(*Sidecar spawn without a console window*), not by this chunk. The leg's ingest scrub replaced the real path
with `<host-path>` before the record was written; the committed record is host-path clean on all six anchors
(word-anchored drive letter, `%APPDATA%`, `/Users/`, `/home/`, `.cargo`, `.rustup`).

## Honest limits

1. **`operator_review` is empty.** Every grade here is the agent arm's. The review seam is open.
2. **Finding 1 is measured, not fixed**, and its mechanism is unidentified.
3. **Finding 4's fix is unproven** while the leg reloads.
4. **Finding 7's own rows cannot be graded** on the agent arm; the corroboration above is from adjacent windows.
5. **Finding 8's last-scenario case** is unit-tier evidence, not leg evidence (T-01 not run by design).
6. **The 14 browse-class rows remain findings** — never passes, never a manual arm — pending OS-level key
   injection (the CARRY pinned to this entry and, by the operator's 2026-09-04 ruling, out of this chunk).

## Firing form (for the harness rules)

The live subject needs **five** handles, not the four the directive listed. `wdio.conf.ts:126` gives the `sr`
suite no `scenarios` field — unlike `sr-empty`/`sr-error` — so `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios`
must come from the shell (repo-relative; `resolve_under` rejects absolute handles). Without it the app loads all
35 scenarios and the walk fails mid-way at S0-04 on option order, naming nothing about the catalog.
