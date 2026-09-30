# Codebase Research — 2026-09-30-the-sr-cause-isolated-on-this-host

## Scope
- **Depth:** moderate. **Reads:** 9. **Globs/Greps:** 11.
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, 75.0 KB, over the read cap.
  - Read as a structural extraction: an index of every non-blank line with its length, then offset-bounded reads of
    `:58-68`. Those lines hold every SR/NVDA/webview entry: census, foreground, `sr*` no-rebuild, the cumulative
    record, printed-verdict atoms and NVDA log headers.
  - `:15-24` (the 5-command surface) was read from the index.
  - Applied additions: 8 — `:58` (census + stop form, the `Spec Files:` TAB atom), `:59` (foreground; injected keys
    never reach browse mode; the parallel session's focus theft; deciding theft from NVDA's own log), `:62`, `:63`,
    `:64`, `:66`, `:68`, `:49`.
  - Also read: `.claude/rules/a11y.md` (auto-loaded) and `host-win32.md` (always loaded).
- **Platform issues consulted:**
  - NVDA `source/inputCore.py` on `master`, fetched raw from `raw.githubusercontent.com/nvaccess/nvda`. The fetched
    source states that `InputManager.executeGesture` logs `log.io("Input: %s" % gesture.identifiers[0])`, guarded by
    `if log.isEnabledFor(log.IO) and not gesture.isModifier:`. It does no injected-key filtering in that file.
  - This is the fact the operator-keyboard arm's stimulus timeline rests on (below).
  - No hosted-runner bullet exists. The only CI reading is Setup 5a's, re-read here.

## Files inspected
- `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/report.md` (`:135-165`) — the
  PREREQ's origin. `run --unit` and clippy were both `not run — defer (key): zero Rust delta`.
- `…/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/plan.md` (`:1-290`) — the prior fence. The unit
  entry is `:140-143`, with `defer = 'zero Rust delta: …'`. The clippy entry is `:145-148`, with `defer = 'zero Rust
  delta, as the unit entry'`. The coherence probe, strict `--e2e`, `sr*`, hygiene, delta-guard and census entries
  are the reusable forms.
- `…/evidence/cause-control.md` (full) — the configuration table, the silence shape, the candidates still standing
  and the product lever named. Cited for the carried state; nothing in it is re-graded here.
- `scripts/agent-run.sh` (`:10-11`, `:295-346`, `:394`):
  - `run --unit` = `ensure_frontend; "$CARGO" nextest run --workspace --profile ci` (`:295`), the nextest leg alone.
  - Clippy is a separate command, run only by the full bundled `run` (`:321`). So the PREREQ is two entries, not one.
- `crates/conductor-tauri/ui/wdio.conf.ts` (`:150-460`):
  - `SR_SUITES` (`:164-168`) holds three subjects only (`live`/`empty`/`error`).
  - The capability is fixed at `browserName: 'wry'` + `tauri:options.application` (`:345-349`).
  - The one tauri-driver spawn (`:417`) and the NVDA spawn (`:213`) take a fixed argv:
    `-m --no-sr-flag -c <leg cfg dir> -l 12 -f <log>`. The stop form is `-q` (`:245`).
  - No config branch reaches a non-Conductor host. Pointing this stack at Edge would be a new capability and a new
    program under the driver.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` (`:1-120`, plus a grep):
  - `Subject` comes from `rows.ts`, and `main` maps anything that is not `empty`/`error` to `live` (`:483`).
  - Rows come from the spec (`ROWS`, `rowsFor`), and `HEARD_MAX = 400` (`:198`). The scrub to `<host-path>`
    (`:266`) adds a `security_finding` (`:424`).
  - A non-Conductor page has NO spec rows, so this parser cannot grade a control arm over Edge. Such an arm is
    recorded outside the pass format.
- `.andromeda/playbook.md` (`:118-126`) — `:124` IS the *Boundary widening* pattern, with its verdict `escalate` at
  `:125` and "always a human's call" at `:126`. The coordinate the entry cites resolves.
- `runs/sr-leg/nvda-speech.{empty,error,live}.log` (a grep):
  - `grep -c 'Input: '` returns 0 in all three, and `grep -c 'executeGesture'` returns 0 in all three.
  - WebDriver-injected keys therefore leave NO input entry in NVDA's log. This confirms verification-harness.md
    `:59` (5): they bypass the OS keyboard path.
  - A physical key would log one (the inputCore source above). This is unmeasured on this host and is the first
    slot's check.

## Graph impact
- **not queried.** The modify-set holds no code symbol: the chunk writes chunk evidence only under the recommended
  arm (below), with no `crates/*/src` change. The TS harness symbols that come closest (`startNvda`, `writeNvdaPass`)
  are read, never changed. If P4's fork picks a harness-changing arm, the ts plane is queried then, before the list
  closes.

## Patterns detected
- **The prior chunk's control discipline** (`cause-control.md` §Configuration table, §Witness): one variable per arm.
  - Every run carries runtime × driver × NVDA × OS build × `allowInChromium` × bundle × desktop.
  - A witness makes an arm a measurement rather than void.
  - Utterances are quoted by hand, with host paths described in words, and the host-path hygiene entry proves it.
  - This is the precedent for committing control-arm speech that has no spec rows.
- **Theft decided from NVDA's own log** (verification-harness.md `:59`, as extended 2026-09-30). A focused console
  logs a steady stream of terminal notifications. Their absence across the graded window, with no foreground entry for
  another window, makes a silent arm valid.
- **Census by parentage** (host-win32.md 2026-09-17; `cause-control.md` §Census), taken before and after every leg.

## Conventions to follow
- **Registry keys in the colon-free `reg.exe` form; host paths described in words** (host-win32.md 2026-09-11 as
  extended). The prior coherence entry (`plan.md:155`) reads the runtime's `pv` in that form.
- **Diff-shaped probes name the chunk base `4460307`** (W182).
- **`jq`/hygiene entries read the committed evidence copies**, never the working record (`plan.md:133-137`).

## Measured host facts (read-only probe, 2026-09-30T12:58Z; scratch script, run by path)
- **Edge browser** `pv` 154.0.4258.37 (the `EdgeUpdate` client key for the browser). This is the SAME build string as
  the WebView2 runtime `pv` 154.0.4258.37. So an Edge arm varies the HOST (browser vs WebView2 embedding) at an
  identical Chromium build.
- **OS** `CurrentBuild.UBR` 26200.9457, unchanged from the prior record.
- **WebView2 hosts running:** one family, SearchHost ×6, all on runtime **153.0.4234.48**. The version was read from
  each process's command line and is rendered here as the version alone. They were created 2026-09-26, before 154
  landed on 2026-09-29, so they still hold the superseded runtime.
  - The only other WebView2 app on the host is therefore on 153 today. An arm over it varies app AND runtime at once,
    and licenses nothing about 154 by itself.
  - If SearchHost restarts before the slot, it loads 154 and becomes a genuine other-app-on-154 arm. So the version is
    re-read at the slot, never assumed.
  - No `msedge.exe` and no `nvda.exe` are running.
- **Release bundle** `target/release/conductor-tauri.exe` was last written 2026-09-30T12:42:11Z (after the prior
  wrap). It is rebuilt by this chunk's strict `--e2e` before any Conductor arm (verification-harness.md `:62`).

## The PREREQ, measured
- **Entry forms:** `bash scripts/agent-run.sh run --unit` (role unit) and `cargo clippy --workspace --all-targets -- -D
  warnings` (role lint), as the prior fence carried them. Closing the deferral = both entries carry NO `defer` key in
  this fence, so /implement and the wrap run them whatever the delta.
- **Precedent:** `2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed` closed the same deferral class as
  "PREREQ closes — MET: `run --unit` (1136/1136) + clippy green" (its `report.md`). Re-derived by grep over that
  report. The tally is that date's sample, not a bound.
- **Delta at take-up:** `git diff --stat 4460307 -- crates scripts contracts scenarios .github Cargo.lock Cargo.toml`
  prints nothing, so the gates measure HEAD = the chunk base.

## The no-boundary control, researched
The entry's control (1) reads silent-there ⇒ below Conductor, heard-there ⇒ Conductor's posture. Research found an
axis the entry does not name: the **input path**.
- Every silent run so far is WebDriver-injected: CDP key events that never touch the OS keyboard path and leave no
  `Input:` entry.
- The one heard run (2026-09-07) was injected too, so injection alone did not silence 152. But nothing has measured a
  PHYSICAL keyboard on 154 in either host (`cause-control.md` §USER consequence: "a physical keyboard was not
  measured").
- The user consequence the prior chunk recorded (a real NVDA user hears no focus changes) rests on the injected path
  standing in for a physical one. That is an unmeasured equivalence.

Three ways to drive control (1), each measured against the boundary registries:
- **(K) Keyboard, no automation.** In one quiet-desktop slot:
  - NVDA starts at the leg's registered argv, pointed at a copy of the committed leg profile, with `-l 12`.
  - The operator presses Tab at the physical keyboard in (a) a plain focus page in Edge 154 and (b) Conductor's
    release binary. Conductor is started by the agent with the repo root as its working directory, a product
    launch, so the catalog resolves as it does for the `sr-empty`/`sr-error` subjects.
  - Optionally (c): the Windows search panel (SearchHost, the other WebView2 app, runtime re-read at the slot).
  - The log timestamps each physical key itself (`Input: kb(desktop):tab`, per the inputCore source), so the stimulus
    timeline is machine-read, never operator recollection.
  - The page can be a `data:text/html,…` URL typed into Edge: no file, no listener and no network (the trust boundary).
  - No committed spawn form changes. The NVDA and Conductor launches are interactive-session acts outside rule (b)'s
    three loci (security.md: a spawn outside the driver stack, `agent-run` and `ci.yml` adds no form; the
    `secret_scan_gate` precedent 2026-09-24).
  - Its one open question is whether the operator reads an agent-launched NVDA/Conductor as within "no-boundary".
  - It answers two things in one slot. Edge vs Conductor under the same physical input tests the host. Conductor
    physical vs Conductor injected (the prior legs) tests the input path.
- **(W) WebDriver re-pointed at Edge** (msedgedriver + an Edge capability, injected Tab like the Conductor legs).
  - It keeps the input path equal to the prior legs, so the host is the one variable.
  - It adds a new capability/program under the driver. If committed, that is a new rule (b) form, the eighth, which
    escalates (security.md) and needs the founder's live word (security.md 2026-09-29). If uncommitted, it is still a
    second use of `CONDUCTOR_MSEDGEDRIVER` that arch §Occupied Resources does not describe (arch-history item 6).
  - Edge under automation also runs with automation flags, which differ from a user's Edge.
- **(O) Operator-only.** The operator starts NVDA themselves and walks Edge. No agent launch at all.
  - The record depends on the operator passing the leg's exact argv, and that leg-profile equality is the part most
    likely to drift.

### P4 addendum — the no-human variant S (OS-level synthetic keys), researched on the overseer's word
The overseer ruled K the primary control. K is a DESK task, runnable only with the founder at the machine. The
overseer also asked for one no-human variant to be weighed.
- **S = OS-level synthetic keys** (`SendInput` from a script) at Edge and at Conductor, with no WebDriver.
- **Does NVDA treat an OS-injected key like a physical one? Yes, by default.** The fetched sources:
  - NVDA `source/keyboardHandler.py` on `master`: `internal_keyDownEvent(vkCode, scanCode, extended, injected)`
    returns early (passes the key through unprocessed) only
    `if injected and (ignoreInjected or not config.conf["keyboard"]["handleInjectedKeys"])`.
    `ignoreInjected` is NVDA's own flag while it injects keys itself.
  - NVDA `source/config/configSpec.py`: `handleInjectedKeys= boolean(default=true)`.
  - The leg's committed `nvda.ini` carries no `[keyboard]` section (a grep for `handleInjected` returns nothing;
    sections are `[general]`, `[speech]` and `[virtualBuffers]` only), so the default applies.
  - An S key therefore reaches `inputCore.executeGesture`, which logs `Input: kb(desktop):tab` at IO level, the same
    as a physical key.
  - It reaches NVDA's keyboard hook path. That is exactly the path WebDriver/CDP injection bypasses (0 `Input:` entries
    over all three prior logs), so S also reaches browse mode, the class E0-09 waits on. That is a side observation;
    this chunk regrades nothing.
  - To the app, S and a physical key are the same window message; only a low-level hook can see the injected flag.
- **Does S trip *Boundary widening* (playbook `:124`)?** No, as planned.
  - The pattern is "a chunk WIDENS what crosses an already-hardened boundary (… a subprocess/IPC boundary gains a new
    crossing)". S commits no spawn form: the walk script lives under the gitignored `runs/sr-control/`, with its text
    reproduced in evidence for reproducibility.
  - It is invoked from the implement session, outside rule (b)'s three loci (driver stack · `agent-run` · `ci.yml`;
    security.md, the 2026-09-24 `secret_scan_gate` precedent).
  - It binds no listener: Conductor idle opens no port, and the `:4317` occupier arms only in a fault phase. Edge on a
    `data:` URL fetches nothing on the leg's behalf. It adds no handle beyond `CONDUCTOR_NVDA`, which it guards the
    wdio way.
  - The harness already registers OS-level key synthesis (`activate-window.ps1`'s `keybd_event` ALT, a rule (b) form
    in the driver stack), so S introduces no new mechanism class.
  - Residual for review: Edge's own background traffic, such as update and SmartScreen, is the browser's, not the
    leg's. The leg itself requests nothing.
- **Comparability.** S differs from K only in who presses the key, a distinction NVDA itself ignores by default. So an
  S reading is a no-human proxy for K, and K remains the human confirmation. S differs from every prior silent run in
  the input path (OS hook vs CDP), which is the unmeasured axis.
- **Foreground.** SendInput goes to the foreground window. Activate a target only when it is NOT already foreground,
  because a synthetic ALT on an already-foreground app opens its System menu (verification-harness.md `:59`,
  corrected 2026-09-04).

### P4 addendum 2 — W and the 153 arm, researched on the founder's ratification («Да», scope.md)
- **W (WebDriver at Edge 154).** Keep it OUT of `wdio.conf.ts`. Research found no way to reach Edge from the config
  without a new capability and program (`wdio.conf.ts:345-349` is fixed to `browserName: 'wry'` + `tauri:options`).
  A committed suite would also be a fourth suite family, restated at five registry sites against arch's ~0 B headroom
  (arch-history items 4, 20). So W is a gitignored session script (`runs/sr-control/edge-webdriver-walk.ps1`), like S's:
  - It starts msedgedriver from `CONDUCTOR_MSEDGEDRIVER` (154.0.4258.37, which drives the Edge 154 browser natively) on
    `--port=4445`, the registered dev-only, harness-lifetime native-driver port (arch §Occupied Resources, Ports), so
    no new listener.
  - It opens a W3C session with `browserName: 'MicrosoftEdge'` and `ms:edgeOptions.args ['--inprivate']`, navigates
    to the same `data:` page, and sends 5 Tabs through `POST /session/{id}/actions`, 4 s apart and stamped.
  - It deletes the session, stops the driver pid and quits NVDA.
  - The keys travel CDP, the same injected path as every prior silent Conductor run. So W's `Input:` count must read
    **0**, the witness that W is on the injected path and S is not.
  - The ratified crossing (scope.md) is exercised from the implement session, outside rule (b)'s three loci. No
    committed spawn form moves, so the wrap records the ratification rather than a new form.
- **The 153 arm.**
  - **The variable.** `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` REPLACES the app's `browserExecutableFolder` when the
    environment is created. Source: the WebView2 Win32 reference, `CreateCoreWebView2EnvironmentWithOptions`, fetched:
    "If you find an override environment variable, use the `browserExecutableFolder` and `userDataFolder` values as
    replacements…".
  - **The folder is admissible.** The reference refuses only a path containing `\Edge\Application\`, and the Evergreen
    versioned folder is `…\EdgeWebView\Application\153.0.4234.48\`, which holds `msedgewebview2.exe` (re-listed at P4:
    `153.0.4234.48` and `154.0.4258.37` present).
  - **The variable reaches the app.** `wdio.conf.ts:380` spreads `process.env` into the tauri-driver spawn env
    (`appEnv`, `:417-421`), and the variable flows from there to the native driver and the app it launches. No harness
    edit is needed.
  - **Firing form:** the existing `npm run a11y:sr-empty` leg, with `CONDUCTOR_MSEDGEDRIVER` set per command to the
    OPERATOR-SUPPLIED 153 driver and the loader variable set to the 153 folder.
  - **Witness that 153 loaded:** the pass record's `webview2_runtime`, the version the driver session attached to
    (`parse-nvda-log.ts:90-91`), must read `153.0.4234.48`. Anything else makes the arm VOID.
  - **Driver admission:** a pre-execution Authenticode check (`Valid` + an `O=Microsoft Corporation` signer), the one
    admitting control security-plan applies to a driver that arrives from outside a lockfile (security-history item
    17), plus version coherence with the folder.
  - **A registration owed at the wrap:** a dev-host use of a `WEBVIEW2_*` loader variable is unregistered
    (arch-history items 9, 10). It is now ratified, and the wrap records it.
  - **Evergreen may prune `153.0.4234.48` once SearchHost releases it,** so re-list the folder immediately before the
    arm. Gone → the arm is `not-run — folder pruned`, never fetched.

## New files to create
- `conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/evidence/no-boundary-control.md`
- `conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/evidence/nvda-pass.153.json`

## Files to modify
- none

## Open questions
- Which driving method for control (1): K, W or O → blocks: plan-decision (P4 fork).
