# Scope — 2026-09-30-the-sr-pass-regrades-on-the-os-input-path

**Working entry (`working-route.md:69`):** The SR pass regrades on the OS input path. The confound (injected keys vs
launch under tauri-driver + msedgedriver) is separated first. Then S0-09, E0-05, E0-09 and every silent focus row are
regraded on the path that step validates, with the configuration named in every verdict.

**Matrix target:** none expected. At take-up the pool read `verified 10/11 · deferred 1 · planned 0 · unclaimed 0`
(`matrix.py coverage`, new-session 2026-09-30T14:13Z). P4 re-reads the pool with `show --unclaimed`.

**Chunk base (W182):** `ff4f571` (`ff4f571ef04b3755f6552dee129fe918fbfe6305`), HEAD at take-up. Every diff-shaped gate
probe names it explicitly (`git diff --numstat ff4f571 -- <f>`), because an operator pre-CI commit moves HEAD before the
wrap (founder directive W182, restated by the operator at this take-up).

## Operator directives and founder rulings at take-up (2026-09-30)
- **Founder ruling: `/andromeda-evolve-diagnose` waits — not now.** The new-session nudge for Epoch 4 stands unacted.
- **Founder ruling: the U35 upgrade door is not this chunk.** `upgrade.py detect` reads U35 `behind`, awaiting a
  0-pending wrap door. This chunk neither opens nor closes it.
- **The founder is away from the desk today, so arm K stays a desk task.** It is not run in this chunk unless the
  founder is at the machine. It is never simulated by synthetic keys.
- **Every NVDA run is the operator's quiet-desktop slot.** Research and implement alike STOP and ask before any NVDA
  launch, stating the run's expected LENGTH. A hold is answered with the hold option; nothing starts unasked.
- **Boundary escalation:** if committing `SendInput` into the leg makes it a boundary crossing, that is a P4 escalation
  that halts for the founder's live word.
- **Diff-shaped probes name the chunk base `ff4f571` (W182).**
- **P4 escalation ruling (playbook `:124`, 2026-09-30)** on committing the OS key path. The overseer answered «Decide
  after step 1»: "this is the founder live word only, so I am asking him now. Plan both branches; implement halts at
  the branch point UNLESS his quoted word has been relayed by then. If he ratifies before implement reaches it, I will
  send the quote and the chunk proceeds without the halt." So step 2's regrade is conditional on step 1's branch AND
  the founder's quoted word. Without the word, the reds route forward (plan.md step 6).
- **P5 approval (overseer, 2026-09-30):** "yes — approved by the overseer. The founder has been asked for the
  send-keys.ps1 word; if his quote arrives before the branch point I will relay it verbatim, otherwise halt there as
  planned. Live sr on H-R: launch pulse-app yourself on my slot and stop it afterwards; ports 4317/4318 must be my
  grant too. Every slot: stop and ask with its length." So:
  - `pulse-app` is this session's to launch and to stop, inside the live-`sr` slot;
  - binding `4317` / `4318` (pulse-app's receivers) is part of that same granted slot, never taken on its own.
- **Founder word at the branch point (2026-09-30, live, relayed verbatim by the overseer)**, in answer to the
  `send-keys.ps1` question with the C1 reading in hand (injected 0/5 + 0/4, OS 5/5): «Да делай» ("Yes, do it"). It
  ratifies committing `send-keys.ps1` as proposed: a closed key set, a foreground guard, a `-File` fixed argv, and rule
  (b)'s governed forms moving from seven to eight. The chunk took branch H-R on it. Written here by the wrap, on the
  operator's relay `conductor-wrap-osinput-2026-09-30` §2, because /implement never writes this file. The
  2026-09-30 «Да» for arms W and 153 stays scoped to those controls.
- **Founder ruling on arm K (2026-09-30, live, relayed verbatim by the overseer at the wrap):** «ну раз уже
  разобрались не вижу особого смысла париться» ("well, since we've already sorted it out, I don't see much point in
  bothering"). Arm K is RETIRED, not pending: recorded "not run — founder ruling, cause already isolated by C1", and
  carried into no successor entry. This supersedes the desk-task framing of the take-up directive and the K fold below.

## The CARRY, folded — the ordered steps (from `2026-09-30-the-sr-cause-isolated-on-this-host`; relay `conductor-wrap-srcause-2026-09-30` §2)
- **Step 1 — separate the confound first.** Send OS-level `SendInput` keys into Conductor launched UNDER tauri-driver +
  msedgedriver, the way the `sr*` legs launch it.
  - Heard ⇒ the injection is the variable, and the leg's key path moves to OS-level input.
  - Silent ⇒ the driver-launch posture is the variable, and that is what gets fixed or bound.
- **Step 2 — then regrade** on the path step 1 validates, never before. The rows: S0-09, E0-05, E0-09, and every row
  that went silent (R0-02..R0-04, E0-02..E0-06). Every verdict names runtime × driver × NVDA × OS build × input path.
- **Measured basis, RE-VERIFIED at take-up** against `…/2026-09-30-the-sr-cause-isolated-on-this-host/evidence/
  no-boundary-control.md` §Arms and §Candidates:
  - S-conductor heard every focus change on 154.0.4258.37 (5 Tabs, 5 utterances), with OS-level keys into the bundle
    launched by path and no driver.
  - W-edge was heard with injected keys (4 of 5 Tabs; the fifth moved inside Edge's own tab strip).
  - R153-empty was silent at 153, as at 154 (E0-02..E0-06 `not-announced`, empty `heard`).
  - §Candidates names the standing confound word for word: "injected keys delivered to Conductor's WebView2 host
    launched under tauri-driver + msedgedriver. S-conductor varied TWO things at once."
- **In scope (the entry's claim):** "OS-level keys reach NVDA's keyboard hook (13 of 13 logged), the path browse mode
  listens on, so the browse-class rows (the E0-09 class) may be agent-reachable." VERIFIED at P3 as far as it goes:
  - The 13/13 is measured (the S log's `Input: kb(desktop):tab` count, evidence §Arms witnesses).
  - The parser already token-grades a browse row WITH heard text on the agent arm (`parse-nvda-log.ts:275-294`). Only
    the empty-row note `BROWSE_NOT_DELIVERABLE` (`:271-273`, "driver-injected keys bypass NVDA's keyboard hook") is
    bound to the input path.

  Whether any browse row is actually HEARD on the OS path stays unmeasured, a hypothesis for step 2 to measure.
- **Hypothesis, kept verbatim from the entry:** "`SendInput` from an agent-launched script is no boundary crossing
  (that chunk's no-boundary research); if research finds it becomes one once COMMITTED into the leg, that is a P4
  escalation that halts for the founder's live word." [premise-corrected: TRUE for an UNCOMMITTED session script
  outside rule (b)'s three loci. COMMITTED into the `sr*` leg it becomes a crossing, for these reasons:
  - Rule (b) (`security-plan.md:367`) registers the window-activation form for ONE step, whose only synthetic input
    is the ALT that lifts the foreground lock (`activate-window.ps1:33-34`).
  - A committed OS-key script is a new spawn site in the driver-stack locus, with a new capability: keystrokes into
    whatever window is foreground.
  - Precedent: security-history `2026-09-01-webview-self-verify-windows-host` ("a chunk that genuinely ADDS a spawn …
    escalates") and `2026-09-16` ("an eighth crossing escalates again").
  - Playbook `:124` *Boundary widening* holds on "a subprocess/IPC boundary gains a new crossing".

  So the commit is a P4 escalation for the founder's live word (security.md 2026-09-29: never a delegate's). Step
  1's control stays uncommitted.]
- **Registration rule (operator ruling at that wrap, E1):** a reader or binder this entry COMMITS
  (`WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`, `:4445`) registers in arch §Occupied Resources THEN. Nothing is registered
  for an uncommitted, per-command use.
- **Mechanism, recorded not established, kept verbatim** (evidence §Candidates): "after the first burst, injected
  focus moves inside Conductor's driven WebView2 document stop producing the focus events NVDA consumes, while OS-path
  focus moves in the same bundle do not. No arm measured the events themselves." VERIFIED at P3 as a READING: the
  record states exactly this, and no code has moved since (chunk base = HEAD). The plan never states it as fact, and
  step 1 measures NVDA's speech, not the events.

## The CARRY, folded — the three owned reds (the light gate's owner)
- **This chunk owns the three reds routed to it** over the committed
  `chunks/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/evidence/nvda-pass.json`. RE-VERIFIED at take-up
  by reading that record:
  - **Focus-row count: 24, bar 0.** 24 = 20 `not-announced` + 4 `announced-differently`, all `class focus`,
    `arm agent`.
  - **S0-09 / E0-05 `announced-as-expected`:** both read `not-announced`, empty `heard`. E0-09 is `class browse`,
    `arm operator`, `not-run-here`; it is one of 15 such operator browse rows.
  - **All three subjects on one configuration: `false`.** `subjects` carries `live` / `empty` / `error` from separate
    runs (`empty` 12:20:29Z, `error` 12:17:40Z; `attach_observed` `{live: true, empty: false, error: true}`).
- **Regraded only on the validated path, never by a rerun-until-announced.** The committed record is the prior
  chunk's evidence. P4 decides whether a regrade writes a new record beside it, and never edits it in place.

## The CARRY, folded — arm K [RETIRED at the wrap by the founder's ruling above; the fold is kept as the take-up record]
- **Arm K:** the physical keyboard over Edge 154, Conductor and the search panel. It was not run because the founder
  was at work. It is runnable only with the founder at the desk and is never simulated by synthetic keys. Today's
  ruling keeps it a desk task, so this chunk records it `not-run` with its clearing event unless the founder arrives.

## Boundaries
- **In scope:**
  - step 1's confound control;
  - step 2's regrade, on the validated path only, with the full configuration named in every verdict;
  - whatever harness change step 1's result requires (the leg's key path moving to OS-level input, or the
    driver-launch posture fixed or bound), subject to the boundary escalation above;
  - the three owned reds, closed on the validated path or routed forward with the measured cause;
  - arm K recorded.
- **Out of scope:** evolve-diagnose; U35; a Windows rollback; fetching any driver (an operator-supplied one only,
  admitted by signature before it executes); Pulse's UI (scope law); a rerun-until-announced.
- **Surfaces touched:** VERIFIED at P3 (research.md §Files to modify):
  - step 1 → the chunk's own `evidence/` plus a gitignored session script under `runs/sr-control/`;
  - the ratified branch only → `screen-reader.e2e.ts` (`tab` / `shiftTab` / `browseKey`, 21 / 6 / 7 call sites),
    `parse-nvda-log.ts` (the input-path note and field), `nvda-pass-spec.md` `:21-24`, and a new sibling
    `send-keys.ps1` beside `activate-window.ps1`.

  [premise-corrected: `wdio.conf.ts` needs no change on any branch; it holds no key-path code.]

## CI verdict read at Setup (5a)
- `ff4f571` (the last wrap's flip, = HEAD): **verdict not yet available.** CI#36727246031 `in progress`, checks 3/3
  open, the oldest (Rust gate) at 188 s when read at 2026-09-30T14:15Z. Recorded as read and UNRESOLVED, never as
  green. Its commit carries the prior chunk's wrap: evidence, docs and masters, with no code delta. **Re-read at P3
  (2026-09-30T~14:3xZ): green — CI#36727246031 completed/success, checks 3/3, wall 554 s.** Nothing to fold.
