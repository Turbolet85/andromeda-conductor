# Report — 2026-09-30-the-sr-pass-regrades-on-the-os-input-path

**Chunk:** The SR pass regrades on the OS input path. The confound is separated (OS keys into the driver-launched
bundle), then the silent focus rows, S0-09, E0-05 and E0-09 are regraded on the validated path, with the
configuration named.
**Date:** 2026-09-30
**Commits:** none since `last_wrap` (HEAD `ff4f571` = the chunk base; no operator pre-CI commit this chunk)

## Changes (structured — detectors read this)
- **Files:**
  - new `crates/conductor-tauri/ui/test/a11y/screen-reader/send-keys.ps1`;
  - modified `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts`, `…/screen-reader/parse-nvda-log.ts`,
    `…/screen-reader/nvda-pass-spec.md` (`:20-31`, the row-class paragraph split into row classes plus input paths);
  - chunk evidence: new `evidence/confound-control.md`, `evidence/nvda-pass.json` (the regrade record, with the
    operator's review transcribed by this wrap);
  - `scope.md`: two directive bullets added by this wrap (the founder word, the arm-K ruling), plus the K-fold heading
    annotated. Basis: `git status --short` + `gate.py scope` (clean: changed 4 · listed 4).
- **Symbols / APIs:**
  - `send-keys.ps1`:
    - one mandatory `-Key` in a closed `ValidateSet` (`Tab` · `ShiftTab` · `h` · `d` · `ArrowDown`);
    - resolves the foreground window's process and exits **4** (nothing sent) unless it is `conductor-tauri`;
    - sends through `SendInput` one event per call — Shift down, key down, key up, then Shift up 300 ms later;
    - ArrowDown carries `KEYEVENTF_EXTENDEDKEY`;
    - exits **5** if fewer events were inserted than requested, or if `GetAsyncKeyState(VK_SHIFT)` still reads down
      afterwards.
  - `screen-reader.e2e.ts`: new `keyRecord` · `injectKeys` · `osKey` · `type OsKey`.
    - `osKey` spawns `spawnSync('powershell.exe', ['-NoProfile','-ExecutionPolicy','Bypass','-File', <send-keys.ps1>,
      '-Key', <constant>])` with `windowsHide` and a 20 s timeout. A non-zero status THROWS, naming the exit (4 =
      foreground refusal), and never falls back to an injected key.
    - `tab()` (21 call sites), `shiftTab()` (6) and `browseKey()` (7; typed to `'h'|'d'|'ArrowDown'`, combobox guard
      kept) now call `osKey`.
    - `bringToForeground`'s start-of-document reset cycle calls `injectKeys('Tab')` directly and no longer calls
      `tab()`.
    - Every remaining `browser.keys` call goes through `injectKeys`: picker text, Backspace, the picker's arrows,
      Enter, Space, Escape.
    - Every key appends `{ts, id:'@key', key, input}` to the actions timeline.
    - Sole callers are file-local; call-site counts are research.md §Graph impact, ts plane.
  - `parse-nvda-log.ts`:
    - new `export type InputPath = 'os'|'webdriver'|'mixed'|'none'` and `PassRow.input`;
    - `Timeline.keys`, and `readStamps` ingests `@key`;
    - new `inputPathOf`; `grade(row, heard, input)`;
    - `BROWSE_NOT_DELIVERABLE` is replaced by `BROWSE_NOT_DRIVEN`. A silent browse row with an OS key in its window is
      now `not-announced` on the agent arm. Without one it stays `not-run-here` on the operator arm, with the new
      note.
    - `writeNvdaPass`'s signature is unchanged; its callers stay `wdio.conf.ts:454` and the CLI `:486`.
  - Ports: none new. The legs use the registered `4444`/`4445` for their lifetime; pulse-app bound `4317`/`4318` on
    the overseer's grant and released them.
  - Env vars: none new.
  - Processes: the `sr*` leg's spawned children gain one per-key `powershell.exe -File send-keys.ps1`, beside NVDA and
    `activate-window.ps1`, with no listener.
- **Crates / modules:** none.
- **Dependencies:** none (no manifest or lockfile touched; `git diff --name-only ff4f571 -- Cargo.lock Cargo.toml
  crates/conductor-tauri/ui/package*.json` prints nothing).
- **Schema / config:**
  - `nvda-pass.json` `PassRow` gains `input` (closed set `os|webdriver|mixed|none`; the jq closed-set gate over
    `.input` reads 0);
  - the actions timeline gains `@key` records;
  - the empty-browse-row note text changed (`BROWSE_NOT_DRIVEN`);
  - `operator_review` transcribed (2026-09-30, founder-delegated reviewer), and every row carries `review_grade`.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - **Harness-spawn rule (b) governed forms: seven → eight** (`send-keys.ps1`, founder-ratified). Sweep
    `grep -oiE 'seven|SEVEN|seventh'` over the seven masters: 14 hits.
    - security-plan `:367` — 5 hits on rule (b): "seven governed forms" and "the count stays seven" ×2 → change (3);
      "The SEVENTH is …" and "the seventh form the one exception…" are ordinals naming the seventh form, and stay true
      because the eighth is array-form → no change (2). Corrected at P2 on the security doc-agent's reading;
      authoring had marked all five → change;
    - architecture `:55` (seven library crates) and `:178` (seven run-contract terms) → no change, different
      subjects;
    - design-system `:322`, layout-templates `:134/:145/:283`, a11y-plan `:156` (never "a seventh lamp") → no change;
    - test-plan `:614/:616` (seven sibling files) → no change.
    - Leaf: `.claude/rules/security.md` ("governs **SEVEN** forms", "The count stays seven").
  - **SR focus-row count 24 → 2**, on the agent arm, over the regrade record (jq
    `select(.class=="focus" and .outcome!="announced-as-expected")`).
  - **Browse rows:** the prior record's 15 operator `not-run-here` rows become 7 `not-run-here` + 6 heard on the
    agent arm (`announced-differently`) + 2 `not-announced` on the agent arm, over the regrade record's `.class ==
    "browse"` rows.
- **Dev-tool versions:** none — msedgedriver re-read at 154.0.4258.37 and the WebView2 runtime at 154.0.4258.37
  (coherence entry, ×3 this chunk), NVDA re-read at 2026.2, Edge browser at 154.0.4258.37; all unchanged.
- **Harness / gate surface:**
  - the `sr*` legs' key path: Tab, Shift+Tab and the browse keys are now OS-level (`send-keys.ps1`, foreground-
    guarded); the reset cycle and every other key stay WebDriver-injected;
  - each row records its input path;
  - no agent-run, CI, wdio.conf.ts or package.json change (the delta guard over `ff4f571` prints nothing).
- **Cross-project / external claims:** Pulse (`andromeda-pulse` repo), the live leg's SUT:
  - `target/release/pulse-app.exe`, sha256 `e76be39b17bc8eed3d10666088b90443ea2f39e6de27db5dc2f84025a1c44a32`,
    written 2026-09-30T14:23:24Z, Pulse HEAD `c6eb395` (its perf chunk's operator pre-CI commit, 14:28:09Z; no product
    change outside it, per the overseer);
  - `andromeda-pulse-mcp.exe` sha256 `2179caab9f7247a52cba867d52e0c7de14472dbc642433ca3d27bf56c34cc634`, written
    05:17:07Z;
  - posture read from Pulse's log: `inference_mode` deterministic, `workspace_root_basename` `srliveretry`, OTLP gRPC
    bound on 127.0.0.1:4317;
  - no CI run was read by this chunk's gates (CI for `ff4f571` was read green at P3: CI#36727246031).
- **Reverted / negative API facts:**
  - The start-of-document reset cycle routed through `tab()` onto the OS path, as plan step 7b's letter read. Under
    OS Tab the cycle never reads the host-chrome BODY stop (10 OS Tabs, all heard), so sr-empty #1 threw at E0-02.
    The reset reverted to injected Tab.
  - A single batched `SendInput` for Shift+Tab. It left NVDA holding Shift for every later OS key (live sr #1). It was
    replaced by per-event calls with a delayed Shift release.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **The confound is resolved.**
     - Claim: "injected keys vs the driver launch still confounded" — a11y-plan §3 `:268`; rule `a11y.md` Testing
       ("injected keys vs the driver launch still confounded").
     - Evidence: C1 (`evidence/confound-control.md` §Arms), one session under the leg's own driver launch —
       injected Tabs 0/5 and 0/4 after the first burst, OS `SendInput` Tabs 5/5.
     - The input path is the variable; the mechanism stays recorded, not established.
  2. **Browse mode is reachable on the OS path.**
     - Claim: browse-mode rows are "findings pending OS-level key injection" / "injected keys never reach browse mode,
       a missing key path" — a11y-plan `:268`; test-plan `:47` and `:307`; `a11y.md` Testing.
     - Evidence: the regrade record — every OS browse key appears in NVDA's `Input:` log, 6 browse rows are heard and
       token-graded on the agent arm, and 2 (S0-13, S0-14) were driven and not announced.
     - The key path now exists for `h` / `d` / ArrowDown. Rows with no OS key in their window stay operator findings.
  3. **The plan's forecast "24 → 0"** (plan step 9, a forecast, not a spec). Measured 24 → 2; both remaining rows are
     content findings.
- **Expected amendments (from plan):** sites located by the `-oiE` sweeps above plus
  `grep -noiE 'pending OS-level key injection|OS-level key|input path|injected keys|driver-injected|keyboard hook|activate-window|window-activation|NVDA'`.
  - **a11y-plan §3 *Screen reader test pattern* — carried** (Spec claims disproved 1–2; Harness surface).
    - Sites: a11y-plan `:268` (1 line; 'pending OS-level key injection' 1, 'input path' 1, injected-class 3).
    - The confound verdict is bound to C1's configuration: branch H-R, the agent arm's key path OS for Tab /
      Shift+Tab / `h` / `d` / ArrowDown.
    - Retire "pending OS-level key injection" ONLY for the browse rows driven on the OS path: E0-07, E0-08, E0-09,
      S0-13, S0-14, S0-15, S3-06, S3-07.
  - **test-plan §6 desktop-webview row + §1 Untestable zones — carried.** Sites: test-plan `:47` (keyboard hook /
    OS-level key / driver-injected) and `:307` (pending OS-level key injection, `activate-window`); the firing form
    gains the OS key path, and the browse-mode zone's remedy is discharged for the rows driven.
  - **security-plan §Security Anti-Patterns → Code Patterns, rule (b) — carried** (Counts: seven → eight).
    - Site: security-plan `:367` (5 count hits, 1 'window-activation').
    - The EIGHTH governed form: `send-keys.ps1`, the window-activation form's shape (a fixed OS program, `-File`, a
      repo script, a closed `-Key` constant, no operator value), foreground-guarded, in the dev-only driver-stack
      locus.
    - Escalated under playbook `:124` and ratified by the founder's live word «Да делай» (2026-09-30, relayed
      verbatim by the overseer).
    - The W/153 «Да» stays scoped to those controls.
  - **architecture §Cross-cutting Patterns → Trust boundary — carried.** Site: architecture `:261` (the screen-reader
    suites "additionally spawn the host NVDA … and a fixed-argv PowerShell window-activation script"). It gains the
    per-key send-keys script, with no listener. Lockstep sites: `CLAUDE.md` Critical Warnings (a leaf, via the
    cascade).
  - **No amendment for C1 under rule (b) — not carried**, as the plan stated: C1 is an uncommitted session script
    outside the three loci, on the registered stack and ports, and its count effect is nil.
- **Also for the fan-out (founder ruling at this wrap, not in the plan's list): arm K is RETIRED.**
  - The ruling, verbatim: «ну раз уже разобрались не вижу особого смысла париться».
  - Site: a11y-plan `:268` ("a physical keyboard is unmeasured"). The clause stays true but gains its status: not run
    — founder ruling, cause already isolated by C1; retired, not pending.
  - The a11y-plan sidecar entry at `-amendments.md:231` is history, left unedited.
- **Coverage of new surfaces:**
  - `send-keys.ps1` → validation closed `ValidateSet` + foreground guard✓ · instrumentation n/a (dev-only harness;
    each key is an `@key` timeline record) · PII n/a · tests e2e (the three `sr*` legs green + a 9-key validation
    probe) · a11y n/a · tokens n/a.
  - `PassRow.input` / `@key` → validation closed set, jq gate✓ · instrumentation n/a · PII n/a (host-path scrub
    unchanged, `security_finding` 0) · tests e2e (the three legs) · a11y n/a · tokens n/a.

## Deviations from intent
1. **The reset cycle stays injected** (plan step 7b and the Implementation notes listed `bringToForeground`'s cycle
   among `tab()`'s 21 sites).
   - Measured: under OS Tab the cycle skips the host-chrome BODY stop and runs to its 10-Tab cap, so the first row's Tab
     lands on BODY (sr-empty #1).
   - Fix: the cycle alone is injected. Every row's Tab, Shift+Tab and browse key stays on the OS path, so the OS key
     set narrowed inside the ratified shape.
   - The overseer accepted it as a fix-loop re-fire ("a harness throw, not a grade; the narrowing stays inside the
     ratified key set").
2. **`send-keys.ps1` releases Shift in its own `SendInput` 300 ms after the key and checks Shift reads up** (step 7a
   said "down/up through SendInput, exit 0 when every event inserted, else 5").
   - Measured: after one batched OS Shift+Tab, NVDA logged every later OS Tab as `shift+tab` (15:05:45.691,
     15:05:47.128) and its browse-mode Shift+Tab moved focus backwards (live #1, S1-04).
   - Validated before the re-fire by a 9-key probe over the C1 launch: NVDA's `Input:` sequence and the landings were
     identical to the keys sent, and all nine were spoken.
   - Inferred cause (not established): the batched Shift-up can land while NVDA re-sends the key with injected input
     ignored.
3. **Two subjects were fired twice** (the plan criterion reads "fired once per subject"): sr-empty (#1 red, #2
   green) and live sr (#1 red, #2 green).
   - Each first run was a harness throw in this chunk's key path, snapshotted and recorded before the green run.
   - Each re-fire had its own granted slot, the overseer's words being "accepted as a fix-loop re-fire" and "probe
     first … this is the last live fire".
   - No run was re-fired to chase an announcement.
4. **An unplanned validation probe** (`runs/sr-control/shift-probe.ps1`, gitignored, derived from C1's script), on
   the overseer's grant.
5. **C1's start-of-document burst is back-to-back, as the leg's is.** The graded keys are 4 s apart (the plan said
   "Keys are 4 s apart"). The burst mirrors the leg's cycle, and it is not decision-bearing.
6. **`scope.md` was written by this wrap** (the founder word and the K ruling), on relay
   `conductor-wrap-osinput-2026-09-30` §2 and the founder's ruling. /implement's contract forbids that write, so it
   was flagged at implement's report.

Scope record: none — `gate.py scope` clean (changed 4 · listed 4 · recorded 0), at /implement P4 and at this wrap's
P1.

## Decisions & corrections
- **Founder word, branch point**, relayed verbatim: «Да делай». It ratifies committing `send-keys.ps1`, and rule (b)
  moves seven → eight.
- **Founder ruling at the wrap**, relayed verbatim: «ну раз уже разобрались не вижу особого смысла париться». Arm K is
  RETIRED, recorded "not run — founder ruling, cause already isolated by C1", and carried into no successor.
- **Overseer rules this chunk:**
  - every NVDA or window run is a quiet-desktop slot, stop and ask with its length;
  - live `sr` needs the 4317 grant; pulse-app is this session's to launch and to stop;
  - a harness throw may be re-fired once as a fix-loop re-fire, but a second harness defect on the same leg stops and
    reports;
  - a fix is validated by a probe before any live re-fire; the second live fire was the last, and whatever it graded
    is the reading.
- **The operator review is transcribed from relay §1.** One correction is carried in `operator_review.notes`:
  - T-01 is a live-class row whose `not-run-here` reason is the record's note (the live subject is stopped by
    design), not a missing OS browse key;
  - the relay's basis line 4 is kept verbatim;
  - the relay's list of 31 accepted ids matches the record exactly (jq over `.outcome=="announced-as-expected"`).
- **Measured mechanics:**
  - an OS Tab from the report region may skip the host-chrome BODY stop, while an injected Tab from the same place
    lands on it;
  - NVDA keeps a stale Shift after a batched injected Shift+Tab;
  - an arrow key sent without `KEYEVENTF_EXTENDEDKEY` is the numpad key, which NVDA binds to a review command.
- **Sweep hazards met in this wrap:**
  - `grep -E` with `\|` counts a literal pipe, so the first site sweep returned false zeros;
  - `grep -c` over the masters' multi-KB single lines counts lines, not occurrences — re-counted with `-oiE … | wc -l`;
  - the bash guard blocked 3 doubled-backslash commands at implement.

## Outcome
Acceptance criteria, re-asserted against the diff:
- **C1 is a measurement** — MET.
  - The witness `grep -c 'Input: kb(desktop):tab'` reads 5.
  - Every key is stamped with phase, input, landing and foreground.
  - Theft is decided from NVDA's log (none inside a graded window).
  - The `C1-(I1|O|I2)` entry reads 3.
- **The branch follows its rule** — MET. One `**Branch:** H-R` line. The configuration is bound runtime × driver ×
  NVDA × OS build × input path × launch posture, and the mechanism is recorded, not established.
- **Commit only on the word** — MET.
  - The entry exits 0, printing `H-R with the founder word quoted`.
  - The delta guard prints nothing (only the four ratified files moved), and there is no `-Command` spawn.
- **send-keys.ps1 matches the ratified shape** — MET, with deviation 2 (the Shift-release timing and an up-check
  inside exit 5). The key set is closed, exit 4 means nothing sent, and it runs `-File` with a fixed argv and no
  operator value. `tab` / `shiftTab` / `browseKey` alone call it, and a refusal throws. The diff introduces no other
  OS key site: the reset cycle is injected (deviation 1).
- **Each changed member loaded by an executed leg** — MET.
  - `typecheck:e2e` is green.
  - sr-empty, sr-error and sr each printed `Spec Files:` TAB `1 passed, 1 total` on a granted slot.
  - "Fired once per subject" holds for sr-error only. sr-empty and sr each had one prior red harness run
    (deviation 3), on the overseer's grant.
- **The three reds GRADED** — MET as a grading. Two bars are not met, and they are measured reds, routed:
  - focus-row count 2 (bar 0; forecast 24 → 0);
  - S0-09 and E0-05 `announced-differently` (missing P-001);
  - E0-09 graded on the agent arm via the OS path;
  - one-configuration `true`.
  - Each is in §Reds with its configuration. `verification-matrix.json` is untouched (no claim).
- **Every other branch** — n/a (H-R). The prior chunks' records are unedited (`git diff --stat ff4f571` over both
  prints nothing).
- **The closed sets hold** — MET; the entry reads 0.
- **Arm K recorded** — MET: three rows. Since the wrap it reads "not run — founder ruling, cause already isolated by
  C1; retired, not pending".
- **Evidence host-path-free, teardown exact** — MET: hygiene 0, no 4444/4445 listener, census 0.
- **The routine arm is unaffected** — MET. Strict `--e2e` printed `[a11y] verdict asserted — 0 failed` with `(expected
  2)`, twice.
- **Every launched arm's backend log is clean** — MET; the panic entry reads 0.

Gates (implement's full block, run `2026-09-30T14-45-00-implement`):

| entry (`run`) | verdict |
|---|---|
| `bash scripts/agent-run.sh run --unit` | not run — defer (zero Rust delta). Name check of every uncommitted file over `crates/`: 0 hits. The one Rust test that reads untracked files, `secret_scan_gate`, was run alone: 5/5 passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | not run — defer (as the unit entry) |
| driver/runtime coherence | green · `driver 154.0.4258.37 runtime 154.0.4258.37` |
| `npm --prefix crates/conductor-tauri/ui run typecheck:e2e` | green |
| `CONDUCTOR_A11Y_STRICT=1 bash scripts/agent-run.sh run --e2e` | green · `verdict asserted — 0 failed` · `(expected 2)` |
| `powershell … runs/sr-control/driver-os-walk.ps1` | leg operator — fired by hand once on the granted slot, exit 0 (evidence §Arms) |
| `grep -c 'Input: kb(desktop):tab' …driver-os.log` | green · 5 |
| C1 phase rows | green · 3 |
| Branch line | green · 1 |
| commit only on the word | green · exit 0 |
| delta guard | green · exit 1, no output |
| prior chunks' evidence diff | green · no output |
| no `-Command` | green · 0 |
| `rm -f runs/sr-leg/nvda-pass.json` | leg operator — fired by hand before sr-empty #1 and #2 |
| `npm run a11y:sr-empty` | leg operator — #1 red (harness), #2 `1 passed, 1 total` |
| `npm run a11y:sr-error` | leg operator — `1 passed, 1 total` |
| non-priming probe | leg operator — `sidecar on PATH`, `True` (both live runs) |
| `npm run a11y:sr` (live) | leg operator — #1 red (harness), #2 `1 passed, 1 total` |
| closed sets over the record | green · 0 |
| RED 1 focus count | recorded · `focus-nonexpected 2` — a measured red, routed (§Reds) |
| RED 2 S0-09 / E0-05 / E0-09 | recorded · all `announced-differently agent os` — S0-09/E0-05 routed |
| RED 3 one-configuration | recorded · `one-configuration true` |
| reds table | green · 3 |
| K rows | green · 3 |
| hygiene | green · 0 |
| panic sinks | green · 0 |
| 4444/4445 listener | green · 0 |
| post-leg census | green · 0 |

- Wrap light gate (run `2026-09-30T15-22-00-wrap`):
  - the full block re-ran green 17 · recorded 3 · red 0;
  - the operator legs were re-verified from the recorded evidence, not re-run;
  - the clippy `defer` stands (clippy reads Rust source only; none changed).
  - The unit `defer` was VOIDED at the wrap: `secret_scan_gate` reads every tracked and untracked file (`git
    ls-files`), and this wrap changed masters, leaves and CLAUDE.md. So `bash scripts/agent-run.sh run --unit` ran in
    full beside the tool — exit 0, `1136 tests run: 1136 passed, 0 skipped`. The name check found every other `.rs`
    mention of a changed doc to be a comment, not a read.
- Watches: none folded.
- Smoke: the self-verify entry ran as a P2 gate (twice); no boot-path change.
- Outcome basis: implement's P4 report and this session's conversation (the same window). Post-implement artifacts:
  the operator review transcription and the K ruling (this wrap).
- Process hygiene (implement P4's census, re-measured at 15:19:49Z):

  | process | started by | final state |
  |---|---|---|
  | NVDA (C1, empty ×2, error, live ×2, probe) | this chunk's legs | terminated |
  | tauri-driver, msedgedriver, conductor-tauri and their WebView2 children | this chunk's legs and session scripts | terminated |
  | pulse-app pid 31960 and pid 20564, each with its sidecar | this session, on the overseer's grant | terminated (`Stop-Process` after 15 s); 4317/4318 released |
  | SearchHost + its WebView2 tree | the OS | left running: not this session's |
