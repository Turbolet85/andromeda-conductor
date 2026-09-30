# Report — 2026-09-30-the-sr-cause-isolated-on-this-host

**Chunk:** The SR cause isolated on this host — NVDA's focus silence after the first burst bound to the injected, driver-launched path; heard on OS-level keys; runtime pair ruled out
**Date:** 2026-09-30T14:00Z
**Commits:** none since `last_wrap` 2026-09-30T12:41:00Z (HEAD `4460307` = the chunk base; no operator pre-CI commit this chunk, by the operator's word: "No operator CI pass this chunk (evidence-only; the operator legs were the slots)")

## Changes (structured — detectors read this)
- **Files:** `conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/evidence/no-boundary-control.md` (new) ·
  `…/evidence/nvda-pass.153.json` (new; the 153 arm's scrubbed pass record, copied from the gitignored
  `runs/sr-leg/nvda-pass.json`). Nothing under `crates/`, `scripts/`, `contracts/`, `scenarios/`, `.github/`, `Cargo.*`
  (delta guard `git diff --name-only 4460307 -- crates scripts contracts scenarios .github Cargo.lock Cargo.toml` +
  untracked listing: no output). Two gitignored SESSION scripts under `runs/sr-control/` (`key-walk.ps1`,
  `edge-webdriver-walk.ps1`), never committed, their text quoted whole in the evidence.
- **Symbols / APIs:** none (no code change).
- **Crates / modules:** none.
- **Dependencies:** none (lockfiles untouched).
- **Schema / config:** none. `nvda-pass.json`'s shape unchanged; the committed profile `nvda.ini` unchanged (copied, not
  edited, by both session scripts).
- **Spec-master edits:** none before P2.
- **Counts / qualifiers moved:** none — verified (no count a master bakes moved: the focus-row count 24 over the
  prior chunk's committed `nvda-pass.json` is unchanged, the record untouched; `grep -c` of the three reds' subjects
  in that file unchanged since `4460307`).
- **Dev-tool versions:** none changed. Re-read on the dev host 2026-09-30T13:2xZ: the DRIVER (`CONDUCTOR_MSEDGEDRIVER`,
  msedgedriver) 154.0.4258.37 and the WebView2 RUNTIME 154.0.4258.37 (coherence entry `driver 154.0.4258.37 runtime
  154.0.4258.37`); the Edge BROWSER 154.0.4258.37 (EdgeUpdate client key `{56EB18F8-…}` `pv`); NVDA 2026.2 (the one
  portable copy, set per command on `CONDUCTOR_NVDA`, unset in the agent shell); OS 26200.9457. One ADDITIONAL host
  driver used once, never installed or registered: an operator-supplied msedgedriver 153.0.4234.48 (the overseer
  downloaded it; the session verified Authenticode `Valid`, `O=Microsoft Corporation`, SHA-256 `4AE19C1D…0402E` equal
  to the overseer's, ProductVersion 153.0.4234.48, BEFORE it executed), passed only as the session variable
  `DRIVER_153`. No lockfile-resolved package is this line's subject.
- **Harness / gate surface:** none committed. The chunk ran existing forms plus ratified session-level crossings:
  - Arm W: a WebDriver session driving the Edge 154 BROWSER (`browserName: 'MicrosoftEdge'`, `--inprivate`) through
    msedgedriver 154 on the registered dev-only native-driver port `:4445`, from a gitignored session script — outside
    rule (b)'s three loci, no committed form. Founder, live, relayed by the overseer, to «W и 153 — да?»: «Да».
  - Arm 153: the existing `npm run a11y:sr-empty` leg with `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` set PER COMMAND to the
    on-disk Evergreen folder for 153.0.4234.48 and `CONDUCTOR_MSEDGEDRIVER` set per command to the operator-supplied
    153 driver. The loader variable REPLACES the app's browser-executable folder (WebView2 Win32 reference,
    `CreateCoreWebView2EnvironmentWithOptions`, research.md addendum 2); `wdio.conf.ts:380` carries it into the app env.
    No committed reader of either value. Same «Да», as a CONTROL only.
  - Arm S: OS-level synthetic keys (`SendInput`) from a gitignored session script at Edge 154, at the release bundle
    launched BY PATH from the repo root (no driver) under the `sr-empty` env, and at the Windows search panel — no
    boundary crossed (research.md §P4 addendum; the harness already registers OS key synthesis in
    `activate-window.ps1`).
- **Cross-project / external claims:** NVDA upstream (`keyboardHandler.py` `handleInjectedKeys` default true;
  `inputCore.executeGesture` logs `Input: %s`) — research.md, fetched; MEASURED here: every S key logged as
  `Input: kb(desktop):tab`, so NVDA handled the injected OS keys like physical ones. No CI run read by this chunk's
  gates (evidence-only chunk; CI#36716763078 over `4460307` was read green at phase P3, scope.md).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. a11y-plan §3 *Screen reader test pattern* (`a11y-plan.md:268`, the clause at offset ~1040): "so an NVDA user on
     that configuration hears no focus change in Conductor after the first burst". Measured false for the OS keyboard
     path: arm S-conductor, same configuration (runtime 154.0.4258.37, Windows 26200.9457, NVDA 2026.2, default
     `allowInChromium`), 5 `SendInput` Tabs, each logged as `Input: kb(desktop):tab` and each followed within
     ~50-210 ms by its focus utterance (Minimize window · Close window · Start … Control+Enter · coverage rows table,
     row current · Run report table) — `evidence/no-boundary-control.md` §Arms. The sentence's own premise (the agent
     arm) stays true: the silence reproduces on the WebDriver-injected, driver-launched path (R153-empty, and the prior
     chunk's legs). The leaf `.claude/rules/a11y.md` restates the verdict ("the agent arm hears focus only in the
     window's first burst"; grep `first burst` → 1 hit) without the user sentence.
  2. Same clause: "the runtime/driver pair, the Windows cumulative updates and the desktop stay candidates". The
     runtime/driver PAIR is retired as a candidate: arm R153-empty (runtime 153.0.4234.48 via the loader folder,
     operator-supplied driver 153.0.4234.48, witness `webview2_runtime` = `153.0.4234.48` and banner
     `[webview2 153.0.4234.48 windows]`) left E0-02..E0-06 `not-announced` with empty `heard`, as on 154. The
     cumulative updates and the desktop still stand against the 2026-09-07 heard run (runtime 152).
  3. The prior chunk's `cause-control.md` §The USER consequence carries the same user sentence — a chunk evidence
     record, NOT a master; left unedited by plan and relay; this chunk's evidence re-states it (§USER consequence).
- **Expected amendments (from plan):**
  1. a11y-plan §3 *Screen reader test pattern* — extend the configuration-bound focus verdict with the S / W / 153
     readings and the input-path axis → **carried** (Spec claims disproved 1-2; Harness bullet). Sites:
     `grep -cE 'first burst' a11y-plan.md` = 1 (`:268`); `grep -c '154\.0\.4258\.37' a11y-plan.md` = 2 (both `:268`
     — `grep -c` counts lines, both hits are on the one line).
  2. a11y-plan §3 — bound "agent arm hears focus only in the first burst" to the CDP-injected path with the OS-path
     reading beside it (conditional: S-conductor heard) → **carried**, condition MET (S-conductor heard). Same site.
  3. architecture §Occupied Resources (env handles) — a dev-host, per-command use of
     `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` for the 153 CONTROL, no committed reader (conditional: 153 ran) →
     **carried**, condition MET. Sites: `grep -c 'WEBVIEW2_BROWSER_EXECUTABLE_FOLDER'` = 0 in all seven; the nearest
     row `architecture.md:202` (`WEBVIEW2_USER_DATA_FOLDER` · `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`, grep `WEBVIEW2_`
     → arch 1, test-plan 2).
  4. test-plan §6 desktop-webview row — the measured driver × runtime PAIR set gains the dev-host 153.0.4234.48 pair
     for the `sr-empty` leg (conditional: 153 ran) → **carried**, condition MET. Site `test-plan.md:307`
     (`grep -n 'desktop-webview (Tauri 2 bundled webview)'` → 1); `grep -c '153\.0\.4234\.48'` = 0 in all seven.
  5. security-plan §Security Anti-Patterns → Code Patterns rule (b) — record the ratified W crossing (session-script
     WebDriver session to Edge on `:4445`, outside the three loci, no committed form, count stays seven) and the 153
     driver's pre-execution Authenticode admission on the dev host → **carried** (Harness bullet; Dev-tool bullet).
     Site `security-plan.md:367` (`grep -cE 'count stays seven|the count stays'` → 1).
  Relay (operator, `conductor-wrap-srcause-2026-09-30.md` §1): keep the founder's «Да» quoted wherever these crossings
  are recorded (security rule (b), arch §Occupied Resources, test-plan §6).
- **Coverage of new surfaces:** none (no product, UI or harness surface added; the two session scripts are
  uncommitted instruments quoted in evidence).

## Deviations from intent
- **Arm S took three starts; no key was sent in the first two.** (1) The portable NVDA resolved the relative `-c`/`-f`
  paths against its own install folder: it came up on a fresh default profile there and never logged `NVDA
  initialized` into the repo log path; the script quit it, the config tree it created (all dated this start) was
  removed, and both scripts moved to absolute paths as `wdio.conf.ts` passes them. (2) PowerShell 5 has no `[ushort]`
  accelerator: the script died after Edge opened, before the first Tab; NVDA and that Edge tree (both parented by the
  dead script) were stopped, the type became `[uint16]`, and the key path was dry-tested with an empty key list. The
  third start is the reading. Justification: a start failure before any stimulus is not a reading, and the slot was
  still open (overseer: pulse-builder window-free).
- **Entry 11 (driver admission) was split to honour its note.** Its `run` executes the driver's `--version` in the same
  command as the Authenticode check, whatever the signature prints — contradicting its own note ("Authenticode-verified
  before any execution"). The signature + SHA-256 half ran alone first (Valid, Microsoft signer, hash equal), then the
  entry in full. Relay §1: a real plan defect, carried to curation.
- **Edge relaunched itself in the background** (`--no-startup-window`) when S closed its InPrivate window, outliving
  the script; attributed by creation time inside the leg's close and a pre-S census of 0 msedge; stopped by its root.
- **The W script's base URL is shown with `http[:]//` in evidence** (the host-path anchor reads `p:/` as a drive);
  the elision is stated in the evidence.
- **K (physical-keyboard desk task) not run:** the founder was at work; three rows `not-run`, clearing event named.
- scope record: none — `gate.py scope` clean (changed 0 · listed 0 · recorded 0 · excluded 45).

## Decisions & corrections
- Operator (every arm): every NVDA launch is a quiet-desktop slot — /implement STOPS and asks with its run length and
  answers a hold with the hold option; the overseer then sent "go S", "Go — desktop is quiet" (W), "go 153" with the
  driver path. Honoured for all three; none started unasked.
- Operator: the 153 driver is the operator's to supply, never fetched; the agent verifies it before execution.
- Relay (§1): name the confound word for word — injected keys vs launch under tauri-driver + msedgedriver; S-conductor
  changed both at once.
- Relay (§1): a binary is never executed before its signature verdict is read (entry 11 defect → harness curation).
- Relay (§1): redact the drive path in the three phase-run logs in place, never delete them.
- Sweep hazards found: (a) `grep -c` over an NVDA log with the injected axe library absent is fine, but the
  `--e2e` log embeds axe-core's source, so `grep -E 'failing|passing'` matched library text (`failingChecks`) and
  returned 2.5 MB — anchored `grep -oE '[0-9]+ passing'` answered; (b) a JSON `\uE004` written through the Write tool
  arrives as the raw U+E004 character; (c) `attach_observed:false` reads like "NVDA never bound the window" but is
  derived from "a graded focus row was heard" (`parse-nvda-log.ts:400-402`).
- Finding for route-resolve (relay §2): OS-level `SendInput` keys reach NVDA's keyboard hook (13/13 logged), the path
  browse mode listens on — the browse-class rows (E0-09 class) may be agent-reachable; folded into the minted entry.

## Outcome
Acceptance criteria, against the diff (two new evidence files, nothing else committed):
- **PREREQ closed — MET.** `run --unit` exit 0 (`1136 tests run: 1136 passed, 0 skipped`), clippy exit 0; no `defer`
  key; recorded in evidence §PREREQ.
- **S is a measurement — MET.** Witness 13; every S row carries configuration, `Input:` count, utterances, theft check.
- **W on the injected path — MET.** `session created`; `Input:` 0 at exit 1.
- **153 follows its rule — MET.** Rule RUN (S-conductor heard); folder re-listed; driver admitted before execution;
  `Spec Files: 1 passed, 1 total`; `nvda-pass.153.json` `webview2_runtime` = `153.0.4234.48`.
- **Each reading licenses only its configuration — MET** (evidence §What each reading licenses; Search on 153 licenses
  nothing about 154).
- **Focus vs live-region graded separately — MET** (no live region changed in these walks; stated).
- **USER consequence re-stated — MET**; prior `cause-control.md` unedited (`git diff 4460307 --stat` over it: empty).
- **Three reds routed, none graded — MET** (gate: 3 rows).
- **K run or recorded — MET** (recorded `not-run`, clearing event).
- **Only ratified widenings ran, nothing committed moved — MET** (delta guard: no output; W on `:4445` only; no
  listener survived).
- **Evidence host-path-free — MET** (hygiene entry 0; no raw log committed).
- **Routine arm unaffected — MET** (strict `--e2e`: `[a11y] verdict asserted — 0 failed · 2 skipped (expected 2)`).
- **Teardown exact — MET** (census 0 harness images; no 4444/4445 listener).

The confound, word for word (relay §1): **injected keys vs launch under tauri-driver + msedgedriver** — S-conductor
changed both at once, so this chunk does not separate them; the Windows updates and the desktop are not ruled out
either, against the 2026-09-07 heard run.

Gates, by `run`, implement's run (`.andromeda/runs/2026-09-30T13-21-48-implement`):
- `bash scripts/agent-run.sh run --unit` — green · exit 0.
- `cargo clippy --workspace --all-targets -- -D warnings` — green · exit 0.
- the 154 coherence probe (`d=$("$CONDUCTOR_MSEDGEDRIVER" --version …`) — green · `driver 154.0.4258.37 runtime 154.0.4258.37`.
- the Edge-build probe (`b=$(reg query …56EB18F8…`) — recorded · `edge-browser 154.0.4258.37`.
- `CONDUCTOR_A11Y_STRICT=1 bash scripts/agent-run.sh run --e2e` — green · all three atoms held.
- `… key-walk.ps1 -Mode synthetic` — leg operator: fired by hand on the overseer's "go S"; start 3 exit 0 (`walk complete`); results in evidence.
- `grep -c 'Input: kb(desktop):tab' …synthetic.log` — green · last line 13.
- `… edge-webdriver-walk.ps1` — leg operator: fired by hand on the overseer's word; exit 0; results in evidence.
- `grep -c 'Input: ' …webdriver.log` — green · exit 1 · last line 0.
- `ls "…/EdgeWebView/Application/"` — recorded · `153.0.4234.48`, `154.0.4258.37`, `SetupMetrics`.
- the driver admission (`Get-AuthenticodeSignature … $DRIVER_153`) — leg operator: by hand; every atom held (split, see Deviations).
- `cd crates/conductor-tauri/ui && … npm run a11y:sr-empty` (153) — leg operator: by hand on "go 153"; exit 0; `Spec Files:` atom held.
- the 153 witness (`jq -r '.webview2_runtime == "153.0.4234.48"'`) — green · `true`.
- `… key-walk.ps1 -Mode physical` (K) — leg operator: NOT run (founder not at the desk); rows recorded `not-run`.
- the K witness — green · `3` (the three not-run rows).
- the arm-row count — green · `8`. The routed-reds count — green · `3`.
- the evidence host-path hygiene — green · exit 1 · `0`.
- the delta guard over `4460307` — green · no output.
- `netstat -ano | grep -cE ":(4444|4445) …LISTENING"` — green · `0`.
- `tasklist | grep -ciE "^(msedgedriver|conductor-tauri|nvda|tauri-driver)\.exe"` — green · `0`.
Smoke: the `role = 'self-verify'` entry ran as a P2 gate (strict `--e2e`); not re-driven.
Watches: none folded.
Outcome basis: implement's P4 report as given in this conversation, then the operator relay (no directive between
implement and this report changed any result).
Process hygiene (implement P4 census, re-measured 13:49:43Z): NVDA ×5 sessions, Edge trees (S start 2, S start 3, the
background relaunch), Edge under WebDriver, msedgedriver 154, conductor-tauri (S), and the 153 leg's msedgedriver /
tauri-driver / conductor-tauri / node — all terminated; SearchHost and its six WebView2 processes — the OS's, left
running; pulse-builder's terminal — the operator's parallel session.
