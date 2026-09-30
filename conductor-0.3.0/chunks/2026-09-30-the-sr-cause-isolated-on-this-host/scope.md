# Scope — 2026-09-30-the-sr-cause-isolated-on-this-host

**Working entry (`working-route.md:67`):** The SR cause isolated on this host — why NVDA hears Conductor's webview focus
only in the window's first burst on WebView2 154.0.4258.37 / Windows 26200.9457, isolated by controls run
cheapest-first, each with its boundary named.

**Matrix target:** none expected. At the prior wrap the pool read `verified 10/11 · deferred 1 · unclaimed 0`
(`matrix.py coverage`, new-session 2026-09-30T12:46Z). P4 re-reads the pool with `show --unclaimed`.

**Chunk base (W182):** `4460307` (`44603079262c2d8ccd5f5b685bc8f92fce83a2c2`), HEAD at take-up. Every diff-shaped gate
probe names it explicitly (`git diff --numstat 4460307 -- <f>`), because an operator pre-CI commit moves HEAD before the
wrap (founder directive W182, restated by the operator at this take-up).

## Operator directives and founder rulings at take-up (2026-09-30)
- **Start with the PREREQ:** close the Rust gate deferral before any control runs (below).
- **Founder ruling: `/andromeda-evolve-diagnose` waits — not now.** The new-session nudge for Epoch 4 stands unacted.
- **Founder ruling: the U35 upgrade door is not this chunk.** `upgrade.py detect` reads U35 `behind`, waiting on a
  0-pending wrap door. This chunk neither opens nor closes it.
- **The 153-runtime control is still waiting on the founder's live word.** Plan the no-boundary control first. Do NOT
  plan the 153 arm as runnable unless the founder's word is QUOTED. With no quote, the arm is planned as a named,
  un-runnable step whose clearing event is that word. **SUPERSEDED at P4 by the founder's live word, quoted below.**
- **Founder, 2026-09-30, live (relayed by the overseer during P4), on the desk slot:**
  «Я еще пока на работе, не могу, пусть сам проверит как может» — "I'm still at work, I can't; let it check for itself
  however it can". The overseer's ruling: the physical-keyboard control K is not available today. The no-human
  OS-synthetic-key variant S is PRIMARY if research finds it clean of *Boundary widening*, and K is the fallback desk
  task. This quote ratifies no specific widening.
- **FOUNDER LIVE WORD — RATIFICATION, 2026-09-30 (relayed verbatim by the overseer during P4), in answer to the
  question «W и 153 — да?» ("W and 153 — yes?"): «Да» ("Yes").** Both widenings are ratified:
  - **W** — a WebDriver session driving Edge 154;
  - **the 153-runtime arm** — through the WebView2 loader's executable-folder variable.
  Order stays cheapest-first: S if clean, then W, then the 153 arm ONLY if the cause is still undecided. **The 153
  msedgedriver is the operator's to supply: ask for it when the plan needs it, never fetch it.**
- **Every NVDA run is a quiet-desktop operator slot:** STOP and ask before any NVDA launch, in research and implement
  alike. pulse-builder works in parallel and opens windows only on the operator's slot, so every slot request states
  its expected RUN LENGTH.
- **Diff-shaped probes name the chunk base `4460307` (W182).**

## The PREREQ, folded — close the Rust gate deferral
- **Origin:** `2026-09-30-the-screen-reader-pass-grades-again-on-this-host` (deferred since that chunk). Its report
  (`report.md:146-149`) records `bash scripts/agent-run.sh run --unit` and `cargo clippy --workspace --all-targets -- -D
  warnings` as `not run — defer (key): zero Rust delta`. The wrap re-pinned both as a PREREQ on this successor
  (`tooling.gate-deferral`, deferred 2).
- **Closing it means both entries RUN and read green in this chunk, and the result is recorded.** They are not
  deferred again by their key, even if this chunk's own Rust delta is also zero. VERIFIED at P3: the prior fence
  carried them at `plan.md:140-148` with `defer` keys, `run --unit` is the nextest leg alone (`agent-run.sh:295`) and
  clippy is a separate entry (research.md §The PREREQ, measured).

## The CARRY, folded (from `2026-09-30-the-screen-reader-pass-grades-again-on-this-host`; operator relay `conductor-wrap-sr-2026-09-30` §2)
- **Branch B was measured.** The object-model control (`[UIA] allowInChromium = 2`, witnessed applied) left E0-02..E0-06
  silent, as the default arm left R0-02..R0-04. So NVDA's object-model choice is ruled out. RE-VERIFIED at take-up
  against `…/evidence/cause-control.md` §Verdict and its configuration table (rows 12:17Z and 12:19Z).
- **Candidates still standing** (per that record):
  - the runtime/driver pair: every focus row heard at 152.0.4191.66 on 2026-09-07, silent at 154.0.4258.37, with
    153.0.4234.48 in between;
  - the cumulative updates KB5124008 / KB5129195;
  - the desktop: the parallel pulse-builder session, the Edge 154 install (2026-09-26) and SearchHost's WebView2 tree.
- **Mechanism reading carried from that record, kept VERBATIM as a hypothesis:** "This is consistent with focus events
  not being delivered from the webview/host after the first burst, rather than with NVDA mis-handling events it
  receives; it is recorded as a reading, not an established mechanism." VERIFIED at P3 as a READING: the record
  states exactly this, and the plan never states the mechanism as fact. [premise-corrected: the reading's
  equivalence between WebDriver-injected keys and a user's keyboard is unmeasured. Every silent run is CDP-injected
  (0 `Input:` entries in all three NVDA logs, research.md), and no physical key has been measured on 154 in any
  host. So "focus events are not delivered" is bound to the injected path until an arm varies it.]
- **Control (1), the NO-BOUNDARY control, runs first.** Hypothesis: the silence is host- or runtime-wide, not
  Conductor's. The control runs NVDA at defaults over a plain focus page in Edge 154 (the same Chromium), and/or over
  another WebView2 app on the host.
  - Silent there ⇒ the cause sits below Conductor.
  - Heard there ⇒ the cause is Conductor's own webview posture.
  - That reading decides whether (2) is needed at all.
- **What "no-boundary" admits is a P4 fork.** VERIFIED at P3 that three driving methods exist (research.md §The
  no-boundary control, researched): K (the physical keyboard, no automation, with NVDA at the leg's argv and the
  stimulus timed by NVDA's own `Input:` entries), W (WebDriver re-pointed at Edge, which is a new capability under the
  driver and an eighth crossing if committed) and O (operator-only). The control page needs no fixture: a typed
  `data:` URL is enough, with no file, no listener and no network.
- **Edge-heard and another-WebView2-app-heard are not the same reading.** VERIFIED: Edge's `pv` is 154.0.4258.37, the
  same build as the runtime, so an Edge arm varies the HOST at an identical Chromium. [premise-corrected: "another
  WebView2 app on the host" is NOT on 154 today. The only one running, SearchHost ×6, still holds runtime
  153.0.4234.48 (created 2026-09-26, before 154 landed), so an arm over it varies app AND runtime and is re-read at
  the slot.]
- **Control (2), the RUNTIME control:** the leg pointed at the 153.0.4234.48 runtime still on disk (re-list it first;
  Evergreen prunes superseded dirs) through the loader's executable-folder variable, plus a matching 153 msedgedriver
  the operator supplies. This is a Boundary-widening crossing (playbook `:124`, VERIFIED at P3: the *Boundary widening*
  pattern, `verdict: escalate` at `:125`), waiting on the founder's word (asked by the overseer 2026-09-30, unanswered
  at the prior wrap and at take-up). **RATIFIED at P4 by the founder's «Да» (above): runnable, last in order, only if
  the cause is still undecided after S and W.** The 153 runtime dir and its `msedgewebview2.exe` are still on disk
  (re-listed at P4, 2026-09-30T13:1xZ). SearchHost holds it in use, so it is re-listed again before the arm.
- **A Windows-update rollback is NOT a control this chunk may plan.** It is irreversible host surgery, and the founder's
  call alone.
- **The product lever stays named and unbuilt:** Tauri `additionalBrowserArgs` / the loader's browser-arguments
  variable. Any lever crosses the app boundary.

## The CARRY, folded (from the same chunk, the light gate's owner)
- **This chunk owns the three regrade reds that chunk's plan carried by construction under Branch B:**
  - the focus-row count over its committed `evidence/nvda-pass.json` (24, bar 0);
  - S0-09 / E0-05 `announced-as-expected` (both `not-announced`; E0-09 stays `not-run-here`, a browse finding);
  - all three subjects recorded on one configuration (`false`).
  RE-VERIFIED at take-up against `cause-control.md` §Gate readings over the committed record.
- **They are re-graded only once the cause is isolated and removed or routed — never by a rerun-until-announced.**
  VERIFIED at P3: the reds are AGENT-arm rows over `evidence/nvda-pass.json`, graded only by `parse-nvda-log.ts`
  against the spec, and a control arm over Edge has no spec rows (research.md). Every removal lever (the product lever,
  pinning the app to an older runtime, a rollback) is founder-gated and unratified. The 2026-09-30 «Да» ratifies the
  153 arm as a CONTROL, not as a remedy. So this chunk ROUTES the reds forward with a named owner and the
  measured cause, and never grades them. P4 states the branches by what control (1) licenses.

## Boundaries
- In scope: the PREREQ gates; control (1) as S (primary) with K as the fallback desk task; the ratified W and 153 arms
  in cheapest-first order, each with its configuration recorded (runtime, driver, NVDA, OS build, desktop,
  `allowInChromium`, input path); an evidence record naming what each arm licenses; the routing or regrade of the three
  owned reds.
- Out of scope: building any product lever; fetching a 153 driver (the operator supplies it); a Windows rollback;
  evolve-diagnose; U35; Pulse's UI (scope law).
- Surfaces touched: VERIFIED for arms K and O — the chunk's own `evidence/` only, with no harness, `ui/src` or
  `crates/*/src` change. Arm W would add a wdio capability/config branch (research.md).

## CI verdict read at Setup (5a)
- `4460307` (the last wrap's flip, = HEAD): **verdict not yet available.** CI#36716763078 was `in progress` with checks
  3/3 open; the oldest open check (A11y gate, routine arm) was at 167 s when read at 2026-09-30T12:47Z. Recorded as read
  and UNRESOLVED, never as green. Its commit carries the prior chunk's wrap: docs, the matrix and evidence, with no code
  delta. **Re-read at P3 (2026-09-30T~12:58Z): green — CI#36716763078 completed/success, checks 3/3, wall 699 s.**
  Nothing to fold.
