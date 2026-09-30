# Scope — 2026-09-30-the-screen-reader-pass-grades-again-on-this-host

**Working entry (`working-route.md:65`):** The screen-reader pass grades again on this host — the cause of NVDA hearing
no webview focus event here removed, then S0-09, E0-05, E0-09 and every other silent focus row regraded.

**Matrix target:** none. The pool reads `unclaimed 0 of 11` at take-up (`matrix.py show --unclaimed`); `v3-03`, whose
chunk minted this entry, is already `verified`. P4 re-reads the pool; this chunk is expected to link nothing.

**Chunk base (W182):** `2c9b37d` (`2c9b37d48599ea28d9902deda2e807d76fa4fccf`), HEAD at take-up. Every diff-shaped gate
probe names it explicitly (`git diff --numstat 2c9b37d -- <f>`), because the operator pre-CI commit moves HEAD before
the wrap (founder directive W182, as applied at `2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed/scope.md:9-11`).

## Operator directives and founder rulings at take-up (2026-09-30)
- **Every NVDA run is a quiet-desktop OPERATOR SLOT, and `sr-empty` counts too, though it needs no port.**
  pulse-builder works in parallel on this desktop. So STOP and ask before firing ANY `sr*` suite (or any other NVDA
  launch), in research and in implement alike. A live `sr` run also needs the `:4317` slot.
- **The WebView2 154 cause is a HYPOTHESIS, never a premise.** It is tested with a control before any remedy is built
  on it.
- **Diff-shaped probes name the chunk base (W182):** `2c9b37d`, above.
- **Founder ruling: `/andromeda-evolve-diagnose` is not this chunk.** It waits until the epoch's carried work is done.
  The new-session nudge for Epoch 4 stands unacted.
- **Founder ruling: the U35 upgrade door is not this chunk.** `upgrade.py detect` reads U35 `behind` (masters' logs +
  keyed contracts), awaiting a 0-pending wrap door. This chunk neither opens nor closes it.

## P4 rulings (overseer, founder-delegated, 2026-09-30)
- **Control: object-model first, one variable, no boundary crossing.** An `sr-error` reproduction at NVDA defaults,
  then `sr-empty` with ONLY `[UIA] allowInChromium = 2` in the leg's own `nvda.ini`. It was first ruled `= 3`, then
  re-ruled to `= 2` once P4 showed that Conductor's webview is on IA2 on both dates. The default-resolves-to-IA2 line
  is quoted with its coordinate in research.md. Ask for each quiet-desktop slot; pulse-builder may hold the first.
- **Remedy if `= 2` restores focus:** it becomes the leg's stated AT posture, and all three subjects are regraded under
  it, each verdict naming its configuration. The default-setting silence is recorded as a finding that also states the
  USER consequence: an NVDA user at defaults on WebView2 154 hears no focus changes in Conductor. A product-side lever
  (for example WebView2 browser arguments) is measured for existence and NAMED, never built here. Any such lever
  crosses the app boundary and halts for the founder's live word.

## CI verdict read at Setup (5a)
- `2c9b37d` (the last wrap's flip, = HEAD): **verdict not yet available** — CI#36709475243 `in progress`, checks 3/3
  open, the oldest (Rust gate) at 186 s when read at 2026-09-30T11:38Z. Recorded as read and UNRESOLVED, never as
  green. Its commit carries the prior chunk's wrap: docs, the matrix and evidence, no code delta over `4bab3c6`, which
  ran green 3/3 as CI#36705777679 (handoff). **Re-read at P3 (2026-09-30T~12:0xZ): green — CI#36709475243
  completed/success, checks 3/3, wall 678 s.** Nothing to fold.

## The CARRY, folded (from `2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed`)
- **Origin:** minted on the operator relay `conductor-wrap-63-2026-09-30.md` §2 (the prior plan's SR condition: an owner
  entry in this version, never residuals). The relay is not a repo file. The prior `report.md:288` cites it as the
  operator directive between implement and report; that is the one in-repo witness, re-verified at take-up.
- **The measurement (2026-09-30):** every SR FOCUS row of both subjects graded `not-announced`, at WebView2
  `154.0.4258.37`, msedgedriver `154.0.4258.37` and NVDA `2026.2`. The live-region rows were heard. RE-VERIFIED at
  take-up against that chunk's `evidence/nvda-pass.json` (`recorded_at` 2026-09-30T10:49:56Z, `webview2_runtime`
  154.0.4258.37, `nvda_version` 2026.2): 17 rows `not-announced`, 16 `not-run-here`, 10 `announced-as-expected`,
  6 `announced-differently`, 2 `subject-absent`.
- **`attach_observed.empty` false:** RE-VERIFIED. `subjects.empty.attach_observed` is `false` in that record, while
  `subjects.live.attach_observed` is `true`.
- **The two-sided control:** the BASE `7ee2fea` bundle is silent the same way under `sr-empty`, per that chunk's
  `evidence/driven-leg.md` Findings 2 (`:60-67`, re-verified). The DOM half passed: focus lands on the `aria-current`
  coverage row (`expectActiveCoverageRow()`).
- [premise-corrected: the last run that HEARD focus is 2026-09-07T22:09:30Z, not 2026-09-04 — seven committed
  records under `conductor-0.2.0/chunks/2026-09-07-sr-findings-fixed/evidence/nvda-pass.{empty,error,live}.run*.json`,
  missed by a `*nvda-pass.json` census] **The last runs that heard focus are the 2026-09-07 `sr-findings-fixed`
  records**: every subject at WebView2 **152.0.4191.66** (each session's `@session` `browserVersion`, and each record's
  `webview2_runtime`), NVDA 2026.2, bundle `00181df` plus that chunk's own `ui/src` fix (it landed at `06db2f9`). Focus
  rows graded `announced-as-expected`: empty 5/5 (×2 runs), error 3/3 (×2), live 16/16 (×2). The light-gate live run at
  22:09:30Z heard 15/16; S3-04 (Escape NoGo, focus restore) was `not-announced`. The CARRY's 2026-09-04 record
  (152.0.4191.62) is the last announced run for S0-09 / E0-02 / E0-05 **under that record's name** only. Neither
  record carries a driver version.
- **The 2026-09-30 record's `error` subject is dated 2026-09-07T22:06:07Z** (VERIFIED at P3). It is the 09-07 light-gate
  error run at 152.0.4191.66, carried forward because `nvda-pass.json` is cumulative per subject
  (verification-harness.md 2026-09-07). Its focus rows R0-02 / R0-03 / R0-04 (Minimize / Close / Start) were heard.
  Those are the same three controls E0-02..04 walk and went silent on 09-30. `sr-error` was not re-run on 09-30.
- [closed at P3 as an OPEN hypothesis: no measurement has varied it; the plan's steps 3–5 are its test] **hypothesis (the CARRY's own marker, verbatim):** "the WebView2 runtime moving to 154.0.4258.37 (the
  driver bumped to match it on 2026-09-30) is the variable that changed — untested." Per the operator directive above,
  it is tested with a control before it shapes a remedy. **P3 narrowed the candidate set without firing NVDA
  (research.md §Variable ledger). Still open:** the runtime/driver pair, 152.0.4191.66 → 154.0.4258.37 (through
  153.0.4234.x). The Windows cumulative updates KB5124008 (09-10, build 26200.9445) and KB5129195 (09-15, 26200.9457).
  The desktop: pulse-builder's terminal, Edge browser 154 installed 09-26, and SearchHost's WebView2 tree since 09-26.
  NVDA's object-model choice for Conductor's WebView2 document [premise-corrected at P4: IA2 on BOTH dates. The
  in-process `vbufBase` backend teardown in `msedgewebview2.exe` appears at `nvda-speech.empty.log:309-311` (09-30,
  silent) and at `nvda-speech.error.log:351-355` (09-07, heard). The 09-30 UIA freeze stack belongs to the pulse-app
  window. So the open variant is forcing UIA, `[UIA] allowInChromium = 2`]. **Ruled out by diff over the window 06db2f9..2c9b37d:** the Rust app (formatting and a version string
  only), `tauri`/`wry` (the lockfile moves only workspace-crate versions and test-only edges), `wdio.conf.ts` (a
  driven-only branch), `activate-window.ps1` and `nvda.ini` (unchanged), and the `ui/src` bundle (no commit after
  `06db2f9`). The SR spec changed only the coverage-row lines, none of the E0-02..04 path. The hypothesis stays
  UNTESTED: no measurement has varied any open candidate.
- **Desktop conditions measured those runs:** NVDA speaks "Landscape" ×3 around window activation, and a parallel agent
  session on the same desktop takes the foreground. So the legs need the desktop quiet (the operator-slot directive).
  [premise-corrected at P3: "Landscape" is NOT a 09-30 novelty. The 09-07 error log that heard focus speaks it ×2 at the
  same point (`runs/sr-leg/nvda-speech.error.log`, 2026-09-07T22:05:58–22:06:00Z), so it does not discriminate.]
- **The silence has a measured SHAPE (P3, from the on-disk NVDA logs; nothing fired).** In both 09-30 subjects NVDA
  DID speak Conductor's focus during `bringToForeground()`'s own Tab cycle: "Conductor document", "Minimize window
  button", "Start button unavailable Control+Enter", "Run report … table". After the "Landscape" events it then said
  nothing to any row-driven Tab, until the app closed 39 s later. The 09-07 log shows the same opening followed by
  per-Tab focus speech. So the fault is focus events STOPPING after the first burst, not NVDA failing to attach. A
  third silent `sr-empty` run exists: 2026-09-30T11:33Z, rebuilt at `4bab3c6`, now the working copy's `empty`
  subject. In the live run NVDA's core froze 20.9 s inside `UIAHandler.browseMode._get_isAlive` while it held the
  andromeda-pulse WebView2 window (`nvda-speech.live.log:69-210`).
- **This entry owns the prior chunk's red plan entry 12** (the S0-09 / E0-05 regrade: the `jq` gate over the committed
  `nvda-pass.json`, expecting both `announced-as-expected`; prior `plan.md:372-377`, `report.md:272-274`).

## What this chunk builds
1. **The cause is found.** A discriminating experiment separates the WebView2 runtime hypothesis from its alternatives,
   and a control is run where the real path is known to pass. The cause is named with the measurement that isolates
   it; a cause that fits the story is not enough.
2. **The cause is removed, or routed around by a stated posture.** The fix goes in whichever layer the cause lives in:
   the harness's launch or foreground path, NVDA's configuration, the webview's accessibility-tree exposure, or a
   runtime/driver pin. If the cause turns out to be external and not removable from this repo (a runtime regression
   Conductor cannot pin), the chunk records it with its evidence. It then routes the regrade honestly and never fakes
   a pass. RULED at P4: this is Branch B, where the `= 2` arm is silent too; the chunk records it and stops, and a
   runtime control would need its own ratification.
3. **The regrade.** S0-09, E0-05 and E0-09 are regraded, plus every other focus row the 2026-09-30 run graded silent.
   The regrade makes a fresh committed `nvda-pass.json` under this chunk's `evidence/`. "Every other silent focus row"
   is VERIFIED at P3 as the 17 `not-announced` rows of the 2026-09-30 record, all `class: focus`: E0-02..06, S0-01,
   S0-02, S0-04..07, S0-09, S1-03, S1-04, S2-03, S2-04 and S3-02 (`jq` group over that record). E0-09 is `class: browse`,
   graded `not-run-here`. Browse rows are fed only by the OS keyboard hook (the 2026-08-22 learning), so its regrade
   stays a recorded finding, never a pass.

## Boundaries
- No Pulse UI automation. NVDA drives only Conductor's own webview, through the existing `sr*` WebdriverIO +
  tauri-driver stack (scope law, CLAUDE.md). No new listener beyond the registered `4444`/`4445` driver ports.
- No new harness-spawn form without escalation (security.md: the governed forms stay seven; an eighth crossing
  escalates). The NVDA handle stays `CONDUCTOR_NVDA`, guarded at the wdio edge.
- The speech log stays untrusted third-party text: bounded, host paths scrubbed to `<host-path>` before commit.
- Status stays never color-alone. The a11y claims `v3-03` verified are not reopened.
- Stop everything started: every NVDA / driver / app / webview process a leg or a probe starts is censused by parentage
  and stopped before the chunk's work ends (memory: stop-everything-you-start).
- Out of scope: the Epoch 4 evolve diagnosis, the U35 door, the markerless siblings at `:67` and `:69`.
