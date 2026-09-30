# Session Handoff

**Last Updated:** 2026-09-30T14:08Z
**Branch:** `build/conductor-0.3.0` · 0 ahead of `origin/build/conductor-0.3.0` as read at this wrap's Setup
(HEAD `4460307` = the chunk base; no operator pre-CI commit this chunk — evidence-only, the legs were the slots)
**Status:** clean
**Last Commit:** 2026-09-30-the-sr-cause-isolated-on-this-host — the wrap commit

## Position
- Done: `2026-09-30-the-sr-cause-isolated-on-this-host`. The focus silence is bound to the WebDriver-injected,
  driver-launched path, and OS-level keys are heard.
  - S-conductor was heard on every Tab (`SendInput`, bundle launched by path with no driver).
  - S-edge and W-edge were heard.
  - R153-empty was silent at 153/153, as at 154, so the runtime pair is ruled out.
  - The confound still standing: **injected keys vs launch under tauri-driver + msedgedriver**.
  - Record: `chunks/…/evidence/no-boundary-control.md` (+ `nvda-pass.153.json`).
- Next: "The SR pass regrades on the OS input path" (minted this wrap on relay `conductor-wrap-srcause-2026-09-30`
  §2, at the head of the tail). Step 1 separates the confound (OS keys into Conductor launched UNDER the driver);
  step 2 regrades on the validated path. It owns the three routed reds and arm K.

## Work done
- PREREQ closed: `run --unit` 1136/1136 and clippy green; strict `--e2e` green.
- Arms S, W and 153 fired once each on the overseer's slots.
- K was not run: the founder was at work.

## Drift resolved
- 6 amendments over 4 masters; 2 escalations resolved; 3 proposals rejected by the operator.
  - a11y-plan §3: the focus verdict is now bound to the input path; the user sentence and the pair candidate retired.
  - test-plan §6: the 153 pair joins the pair set.
  - security rule (b): W and 153 recorded as ratified CONTROLS; the count stays seven.
  - arch: CONDUCTOR_MSEDGEDRIVER / CONDUCTOR_NVDA narrowed to "only COMMITTED reader".
- Operator (E1): arch registers STANDING committed readers/binders only. No row for
  `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` or the one-off `:4445` binder; a successor that commits one registers it then.
- Leaves re-derived: `rules/a11y.md`, `docs/a11y-summary.md`, `rules/security.md`.

## Notes
- Last failed command: none.
- Curation: +2 `verification-harness.md` (admission probe gates execution on the signature verdict; portable NVDA
  needs absolute `-c`/`-f`), +1 `host-win32.md` (PS5 has no `[ushort]`).
- Deferred learnings (max-3 cap, confidence 0.8 each):
  - Edge relaunches itself in the background (`--no-startup-window`) when a leg closes its window; census and stop
    it by root.
  - The Write tool decodes a JSON backslash-u escape into the raw character; build it from `[char]92` / `chr(92)`.
  - `recurrence-despite-learning: host-win32.md 2026-09-23 backslash-pair transport correction` (two more doubled-
    backslash commands, both blocked by the guard).
- Proposed playbook rule, not appended: "a configuration-bound verdict's CONSEQUENCE or candidate list narrowed by a
  control this chunk ran, stated with its epistemic status, is routine" (A1/A2 matched no rule; the relay settled them).
- Epoch 5 has grown to 9 chunks through insertions. A boundary here would restore the diagnose/audit cadence; the
  split is the operator's call.
- Host: census back to SearchHost's own 7; no 4444/4445 listener. `CONDUCTOR_NVDA` is unset in the agent shell; the
  legs set it per command to the portable copy. The session scripts stay in the gitignored `runs/sr-control/`.
- Health: see the wrap card.
