# Codebase Research — 2026-09-30-the-screen-reader-pass-grades-again-on-this-host

## Scope
- **Depth:** deep on the SR harness and the recorded evidence, minimal on product code (no product delta is in play)
  · **Reads:** 19 · **Globs/Greps:** 24
- **NVDA runs fired in research:** 0. Every NVDA run is an operator quiet-desktop slot (take-up directive). Everything
  below comes from committed records, the on-disk NVDA logs the 2026-09-30 runs left under the gitignored
  `runs/sr-leg/`, git history, and read-only host probes (registry, `Get-HotFix`, the update-history COM API, a
  directory listing).
- **Harness rules consulted:** `.claude/rules/verification-harness.md` was read in full as a structural extraction:
  the header index plus every entry, offset-bounded, and line 47 read in 4 KB windows. The SR-bearing entries are
  `:57` (attended GUI launch), `:58` (census and stop form; the `Spec Files:` atom, whose TAB was corrected
  2026-09-30), `:59` (the foreground and NVDA binding recipe, extended 2026-09-30: a parallel session takes the
  foreground), `:61` (live `sr` needs the sidecar on `PATH`), `:62` (`sr*` suites never rebuild; only `--e2e` does),
  `:63` (`nvda-pass.json` is cumulative per subject) and `:64` (`expect` atoms from printed output). The live-leg
  entries `:47-:56` and `:60` bind the live `sr` subject. `.claude/rules/a11y.md` and `.claude/rules/frontend.md` were
  read in full (they auto-load on `ui/**`). `host-win32.md` and `security.md` are always loaded.
- **Platform issues consulted:** three web searches over the NVDA and WebView2Feedback trackers, for NVDA focus events
  not announced after the first on WebView2 154, a Chromium auto-disable of accessibility, and WebView2 screen-reader
  focus changes not reported in 2026. None returned an entry matching this signature, so none was fetched as a
  source; a search summary is never a source. One source WAS fetched: NVDA's own
  `source/config/configSpec.py` (master branch on GitHub, not the 2026.2 tag). It states
  `[UIA] allowInChromium = integer(0, 3, default=0)`, commented `0:default, 1:Only when necessary, 2:yes, 3:no`.

## Files inspected
- `crates/conductor-tauri/ui/wdio.conf.ts` (150-466) — `startNvda` (`:206-242`: detached array spawn, fixed argv
  `-m --no-sr-flag -c <leg cfg> -l 12 -f <log>`, readiness on `NVDA initialized` plus a ≤8 s quiet settle),
  `stopNvda` (`:244-250`), the SR subject table (`:163-168`) and the one spawn site (`:393-421`).
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` (55-189, 584-670) — `bringToForeground()` (`:68-129`:
  `activate-window.ps1`, then a Tab cycle to BODY, then a ≤4 s wait for NVDA to name the window, then `quiet(6000)`),
  `stamp`/`act`/`settle` (`:132-182`), the empty subject walk (`:592-626`) and the error subject walk (`:639-669`).
- `crates/conductor-tauri/ui/test/a11y/screen-reader/activate-window.ps1` (full) — `FindWindow`, synthetic ALT,
  `ShowWindow(SW_RESTORE)`, `SetForegroundWindow`, verify.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-config/nvda.ini` (full) — `[general]`, `[speech]`
  (`synth = silence`, `autoLanguageSwitching = False`) and `[virtualBuffers] autoSayAllOnPageLoad = False`. There is NO
  `[UIA]` section, so NVDA runs its default object-model choice for Chromium.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` (attach sites) — `attach_observed` is true iff
  some non-browse, stamped, run row heard ANY utterance (`:372-402`, `:432`). `empty: false` on 2026-09-30 therefore
  means that no empty row heard anything at all.
- `conductor-0.3.0/chunks/2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed/evidence/{nvda-pass.json,driven-leg.md,driver-runtime.md}`
  — the silent record, the two-sided control (Findings 2) and the coherent 154/154 pair.
- `conductor-0.2.0/chunks/2026-09-07-sr-findings-fixed/evidence/nvda-pass.{empty,error,live}.run*.json` and
  `actions.*.jsonl` — the last runs that heard focus (see the Variable ledger).
- `runs/sr-leg/nvda-speech.{live,empty,error}.log` (gitignored; the live log from the 2026-09-30 10:47Z run, the empty
  log from the 11:33Z run, the error log from the 2026-09-07 22:06Z run). They were read through a scratchpad script
  that scrubs host paths and quoted nowhere raw.
- `.claude/rules/verification-harness.md`, `a11y.md`, `frontend.md` — as above.
- `conductor-0.3.0/chunks/2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed/plan.md` (291-445) — the
  measured firing forms and `expect` atoms of every webview and SR leg (the `[[gate]]` fence).

## Graph impact
- **ts plane**, one query over `calls` for `callee_name IN ('bringToForeground','startNvda','stopNvda','nvdaExe',
  'writeNvdaPass','census')`. The trace `tree-query-2026-09-30-the-screen-reader-pass-grades-again-on-this-host.json`
  records `rows: 14`. Every edge is harness-local: `wdio.conf.ts` (the start/stop/census/`writeNvdaPass` callers,
  `:411`, `:447`, `:453`), `screen-reader.e2e.ts` (`bringToForeground` at `:380`, `:593`, `:640`) and
  `parse-nvda-log.ts` (`:485`). No product code calls into the SR harness, and a harness-side remedy reaches no Rust
  symbol.
- **rust plane**: not queried. No Rust symbol is in play, because the Rust app delta over the silent window is
  formatting and a version string (the Variable ledger).

## Variable ledger — what changed between the last HEARD run and the silent runs
The window runs from 2026-09-07 (heard) to 2026-09-30 (silent). Each row is derived beside it.

| candidate | 09-07 (heard) | 09-30 (silent) | status | derivation |
|---|---|---|---|---|
| WebView2 runtime | 152.0.4191.66 | 154.0.4258.37 (153.0.4234.x between) | **OPEN** | each record's `webview2_runtime`; the `@session` `browserVersion` lines in `actions.*.jsonl`; host dir listing |
| msedgedriver | not recorded | 154.0.4258.37 | OPEN (paired with the runtime) | prior `evidence/driver-runtime.md` |
| Windows build | before KB5124008 | 26200.9457 (KB5124008 on 09-10 → .9445; KB5129195 on 09-15 → .9457; .NET KB5126052 on 09-10) | **OPEN** | `Get-HotFix`; the update-history COM API |
| desktop | a Conductor-session terminal repainting | the pulse-builder terminal repainting; Edge browser 154 installed 09-26; SearchHost's six `msedgewebview2` since 09-26 | OPEN | NVDA logs' first utterances; host dir `CreationTime`; prior census |
| NVDA | 2026.2, leg `nvda.ini` | 2026.2, same `nvda.ini` | ruled out as a version/config change | both logs' `Starting NVDA version 2026.2`; `git diff 06db2f9 2c9b37d -- …/nvda-config` empty |
| Rust app / `tauri` / `wry` | `06db2f9` | `7ee2fea`/`4bab3c6` | ruled out | `git diff 06db2f9 2c9b37d -- crates/conductor-tauri/src Cargo.lock`: `main.rs` rustfmt-only; `tauri.conf.json` version string; the lock moves only `0.1.0→0.2.0` workspace versions plus test-only edges (`proptest`, `rstest`, `sha2`, `regex`) |
| `ui/src` bundle | `00181df` + the 09-07 fix | `7ee2fea` `ui/src` (the control) | ruled out | `git log 06db2f9..7ee2fea -- crates/conductor-tauri/ui/src` is empty; the whole `00181df..7ee2fea` delta is the 09-07 chunk's own `App.tsx` fix, which the 09-07 light-gate ran (its log carries the fix's focusin re-assertion behaviour) |
| harness path for E0-02..04 | — | — | ruled out | `git diff 06db2f9 2c9b37d -- screen-reader.e2e.ts`: only the coverage-row lines; `wdio.conf.ts`: a driven-only branch; `activate-window.ps1` unchanged |

### The silence's measured shape (the one datum a control must explain)
- **09-07, error subject, heard** (`nvda-speech.error.log`, local clock = UTC+2). Activation, then the load-error alert,
  then "Conductor document" / "banner landmark Minimize window button" / "Start button unavailable" during
  `bringToForeground`'s Tab cycle. Then "Landscape" ×2 (22:05:58–22:06:00Z), then one focus utterance per stamped Tab:
  "banner landmark Minimize window button" 22:06:02Z, "Close window button" 22:06:03Z, "Start … unavailable"
  22:06:04Z.
- **09-30, empty subject, silent** (`nvda-speech.empty.log`, the 11:33Z run). The same opening: "Conductor document",
  "Minimize window button", "Start button unavailable Control+Enter" and "Run report … table" at 11:33:22Z, inside the
  activation cycle. Then "Landscape" ×3 (11:33:23–11:33:24Z), then NOTHING until the app closed at 11:34:03Z. The
  stamped Tabs at 11:33:31/35/39/43/47Z (`actions.empty.jsonl`) fall inside that gap.
- **09-30, live subject, silent** (`nvda-speech.live.log`, the 10:47Z run). NVDA opened on the andromeda-pulse WebView2
  window. Its core then FROZE 20.9 s (`:69-210`); the frozen stack sits in
  `UIAHandler\browseMode._get_isAlive` via `api.setFocusObject → treeInterceptorHandler.cleanup` (`:120-136`), so NVDA
  was handling a WebView2 154 document through its **UIA** browse-mode path. That was the pulse-app window's document;
  Conductor's own is on IA2 (K-object-model below). After recovery it heard "Conductor
  document … Coverage rows table … row current", and afterwards only live regions ("Conductor · live",
  "Scenarios completed: N", "HOLD — operator pause", "aborted"). No row-driven focus was heard.
- **So the equality a cause must satisfy:** on 09-30, NVDA hears Conductor's focus events during the activation burst
  and hears none for the WebDriver-driven Tabs that follow. On 09-07 it heard both. The mechanism exists; what stops
  after the first burst is the open question. A cause that explains "never attaches" is refuted by the burst.
  "Landscape" is present on both sides, so it does not discriminate (it is ×2 vs ×3).

### Controls reachable without a network fetch (each is an NVDA run, so an OPERATOR SLOT)
- **K-reproduce:** `sr-error` at today's configuration. Its three focus rows R0-02..04 were heard on 09-07 against
  what is effectively the same app. It needs no Pulse and no port, and it also refreshes the error subject the
  final record must carry on one configuration.
- **K-object-model (single variable), CORRECTED at P4.** Conductor's own webview is on NVDA's in-process **IA2**
  path on BOTH dates, so the UIA freeze stack above belongs to the pulse-app window, not to Conductor. The basis is the
  teardown lines, inside a `msedgewebview2.exe` process as the app closed:
  - 09-30 silent empty run: `runs/sr-leg/nvda-speech.empty.log:309-311` reads
    `ERROR - RPC process 16264 (msedgewebview2.exe) (13:34:03.885)` / `Thread 39208, build\x86_64\vbufBase\backend.cpp,
    VBufBackend_t::terminate, 239:` / `Could not execute renderThread_terminate in UI thread`. That is NVDA's
    in-process virtual-buffer backend, the IA2 path. UIA browse mode builds no in-process buffer.
  - 09-07 heard error run: the same pair at `runs/sr-leg/nvda-speech.error.log:351-355`.
  - The empty run had no other WebView2 app open: `census-before.empty.txt` holds two `node.exe` rows, no
    `pulse-app`, and the teardown lands in the second the app closed.
  - NVDA's own selection rule, from `source/UIAHandler/__init__.py` `_isUIAWindowHelper` on the master branch
    (fetched): for `Chrome_RenderWidgetHostHWND` at the default setting, NVDA stays off UIA whenever
    `canUseOlderInProcessApproach and hasAccessToIA2` holds, and is forced off by `AllowUiaInChromium.NO`.
  So the default resolves to IA2 here, and forcing IA2 (`= 3`) would change nothing. The discriminating arm is
  **`[UIA] allowInChromium = 2` ("yes")**. The control changes only that key in the leg's own `nvda.ini`: NVDA's argv
  is unchanged, the `CONDUCTOR_NVDA` registration and spawn form stay, and the file is committed harness data.
  - HEARD under `= 2`: the IA2 focus-event stream is what stops after the first burst on this runtime/OS pair.
  - SILENT under `= 2`: neither object model hears it, which points past NVDA's API choice to event delivery on the
    host.
  The value set comes from NVDA's master-branch `configSpec.py`; the leg's own behaviour confirms that 2026.2 reads it
  the same way, never an assumption.
- **K-runtime:** the Evergreen runtime **153.0.4234.48 is still on disk** beside 154 (machine-wide
  `EdgeWebView\Application`, dir created 2026-09-20). The WebView2 loader can be pointed at it by the loader's
  executable-folder env variable, set in the invoking shell for one control run. The loader reads it; the harness
  code does not change. But the host holds only a **154** msedgedriver (a bounded search under the user profile and the
  tool roots found one copy), and an incoherent pair is REFUSED (a11y.md 2026-09-17). A 153 control therefore needs a
  153 driver the OPERATOR supplies, which is the operator's host task (test-plan §6), never an agent fetch. The known-good
  152 runtime is absent from the host, and a fixed-version 152 is an operator-supplied artifact under the same rule.
  **Time-sensitive:** Evergreen prunes superseded runtime dirs, so the 153 dir may vanish before implement. Re-list it
  before planning on it.
- **Product-side lever — named, not built (operator condition at P4).** This was measured by source read only; no run
  injected anything into the app.
  - The app sets no WebView2 browser arguments: `grep -rn additionalBrowserArgs|additional_browser_args` over
    `crates/conductor-tauri` (excluding `ui/node_modules`) returns 0 hits.
  - So wry's default applies: `--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection`, at
    `wry-0.55.1/src/webview2/mod.rs:294-297`, handed to `set_additional_browser_arguments` at `:327`.
  - The product surfaces that could carry a lever are Tauri's per-window `additionalBrowserArgs` (tauri.conf.json,
    which REPLACES wry's default, so a lever must re-include those three features) and the WebView2 loader's
    additional-browser-arguments env variable.
  - No specific argument can be named from source. Upstream Chromium's `ui/accessibility/accessibility_features.cc`
    (fetched, `main`) defines only `kUiaDisconnectRootProviders` (disabled), `kUiaEventOptimization` (enabled) and
    `kUiaMathMlSupport` (enabled): no native-UIA-provider switch, and no auto-disable-accessibility feature.
  - Any such lever crosses the app boundary, so it is never built or measured in this chunk. It halts for the founder's
    live word (P4 ruling).
- **Out of reach:** the two Windows cumulative updates. They cannot be rolled back as a control, so if K-object-model
  and K-runtime both fail to discriminate, the OS stays a recorded, unisolated candidate.

## Patterns detected
- **Configuration-bound verdicts** (`a11y.md` 2026-09-17; test-plan §9, per tests-history `2026-09-11-*`): every
  compared run records its runtime × driver pair, and every cause claim is scoped to the configurations measured.
- **Single-variable variation against a control where the real path passes** (tests-history
  `2026-09-11-hosted-runner-endpoint-cause-closed`): the model for this chunk's cause claim. A known precedent also
  applies: a runtime-major story fit a webview failure once and was falsified (arch-history `2026-09-09-*`,
  `2026-09-11-*`).
- **`nvda-pass.json` is cumulative per subject** (`verification-harness.md:63`): the 2026-09-30 record's `error`
  subject is 09-07's. A same-configuration regrade re-runs all three subjects.
- **`sr*` never rebuild** (`:62`): an app-side change needs an `--e2e` run first. A harness-side or `nvda.ini` change
  takes effect immediately.

## Conventions to follow
- **SR firing forms** (prior `plan.md:336-369`, measured green 2026-09-30):
  - `cd crates/conductor-tauri/ui && npm run a11y:sr-empty` (env `CONDUCTOR_NVDA`, `CONDUCTOR_MSEDGEDRIVER`);
  - `npm run a11y:sr-error`, the same env;
  - live: `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios PATH="/d/dev/projects/andromeda-pulse/target/release:$PATH"
    npm run a11y:sr` plus the three `ANDROMEDA_PULSE_*`, after the non-priming `conductor preconditions` probe and a
    ≥150 s quiet window.
  - The green atom is `Spec Files:` + TAB + ` 1 passed, 1 total` (`verification-harness.md:58` as corrected
    2026-09-30).
  - The stop form is the harness's own `nvda -q`, then a census.
  - A driver/runtime coherence read precedes any webview leg (prior `plan.md:318-324`).
- **Census by parentage** before and after every leg (host-win32.md 2026-09-17; memory: stop-everything-you-start).
- **Evidence hygiene:** committed SR records pass `writeNvdaPass`'s scrub. Control write-ups describe host paths and
  registry keys in words or the colon-free `reg.exe` form, never quoted (host-win32.md 2026-09-11).

## New files to create
- `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/evidence/nvda-pass.json` — the regrade record, all three subjects on one configuration
- `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/evidence/cause-control.md` — the discriminating controls, each run's configuration (runtime × driver × NVDA × OS build × `allowInChromium`), census tables and the cause verdict

## Files to modify
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-config/nvda.ini` — provisional: only if the object-model control decides the cause and the plan adopts the lever as the leg's stated posture (Open questions)

## Open questions
- RESOLVED at P4 (overseer, founder-delegated). The control is object-model first: an `sr-error` reproduction at
  defaults, then `sr-empty` under `[UIA] allowInChromium = 2`. The first ruling named `= 3`; it was re-ruled to `= 2`
  on the corrected IA2 basis above. If the `= 2` arm does not decide the cause, the chunk stops and reports; a runtime
  control would need its own ratification.
- RESOLVED at P4. If `= 2` restores focus, it becomes the leg's stated AT posture, and the regrade is graded under it
  with every verdict naming its configuration. The default-setting silence is recorded as a finding that states the
  USER consequence, and the product-side lever is named, never built.
- The modify list stays provisional until the `= 2` arm runs: `nvda.ini` is kept on a heard result and restored
  byte-identical to the base on a silent one. → blocks: implementation-scope.
