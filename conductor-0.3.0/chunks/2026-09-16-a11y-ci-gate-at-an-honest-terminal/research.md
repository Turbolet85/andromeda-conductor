# Codebase Research — 2026-09-16-a11y-ci-gate-at-an-honest-terminal

## Scope
- **Depth:** deep · **Reads:** 11 · **Globs/Greps:** 9 · **Graph queries:** 2 (`ts` plane)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — in full as a structural
  extraction (67 lines / 69.2 KB; header index + every `## Session Additions` span, lines 40–67, read
  offset-bounded in three passes) · `.claude/rules/frontend.md` and `.claude/rules/a11y.md` — header +
  long-entry index. `host-win32.md` and `security.md` load unconditionally.
  **Retrieval note:** the chunk's PRIMARY file, `.github/workflows/ci.yml`, is covered by NO rule's
  `paths:` — the harness rule's own retrieval caveat (`:58`) predicts exactly this, so the read above is
  the only channel by which these facts reach the chunk.

## Files inspected
- `.github/workflows/ci.yml` (`:94-104` audit gate · `:238-420` a11y job · `:556-580` conformance gate) —
  the whole subject surface; step shapes, guards, and `continue-on-error` placement.
- `crates/conductor-tauri/ui/wdio.conf.ts` (`:325-420`, plus a symbol index) — the ONE driver spawn site
  and the capability block; where any launch-context change would land if not at the step level.
- `scripts/agent-run.ps1` (`:62-100` `Assert-A11yVerdict` · `:252-300` the `--e2e` leg) — the printed-verdict
  mechanism that defines "honest terminal" in shipped code.
- `scripts/agent-run.sh` (`:197-199`, sweep) — the `.sh` twin of the strict arm.
- `.config/nextest.toml` (`:7-17`) — `retries = 0` in both profiles.
- `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/evidence/reading.md`
  (`:92,137-147,156-172,200-206`) — the CARRY's cited evidence.
- `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/plan.md` (`:368-380`) — the
  rejected-approach record that scoped de-elevation OUT of that chunk and INTO this one.
- `scripts/code-graph-cookbook.md` (`:1-60`) — read before composing SQL, per the P3 contract.
- CI run `35079315258` job log (8 998 lines, `gh run view --log`) and its `a11y-session-diag` artifact
  (6 files; `msedgedriver.log` 3 511 B read in full, `app.stderr.txt` 0 B, `Crashpad/` metadata only).

## Graph impact (from the code-graph query; `ts` plane, `tree-query-2026-09-16-a11y-ci-gate-at-an-honest-terminal.json`)
- **`nativeDriver`** — 1 call site (`wdio.conf.ts:353`); definition `:55`. Local to the config.
- **`exitUnresolvedHandle`** — 2 call sites (`wdio.conf.ts:356`, `:384`); calls `strictMode` (`:75`).
- **`reportSkip`** — 1 call site (`wdio.conf.ts:355`).
- **`census`** — defined in `test/a11y/screen-reader/parse-nvda-log.ts:323`, 4 call sites
  (`wdio.conf.ts:18,239,395,449`, `parse-nvda-log.ts:493`).
- **Meaning for the change:** the handle-guard surface has a blast radius of ONE file. A launch-context
  change placed at the spawn site touches no other module; placed at the CI step it touches no TS at all.

## Patterns detected
- **The honest-terminal mechanism is already shipped and correct** (`scripts/agent-run.ps1:68-100`).
  `Assert-A11yVerdict` asserts three things in order: the `^\[webview2 [^\]]*windows` driven-session
  banner (absent + strict ⇒ exit 1), the `Spec Files:` failed count, and a skip tally against
  `$A11yExpectedSkips = 2` (`:62`). This answers the a11y and tests extracts' shared research question
  — the strict handle IS declared (`ci.yml:383`, `CONDUCTOR_A11Y_STRICT: '1'`) and the printed verdict
  IS asserted.
- **The asserting step is correctly shaped** (`ci.yml:380-401`): no `continue-on-error`, no `if:`. All
  four `continue-on-error: true` keys in the a11y job sit on diagnostics/uploads (`:361` versions,
  `:412` session isolation, `:548` cause probes, `:554` upload). The arch/tests/security constraint is
  already satisfied — so outcome (B) cannot be reached by relaxing it, and nothing needs repairing here.
- **The step already reads the runner variable in the step SHELL** (`ci.yml:386-394`), not through
  `${{ env.* }}`, with a deliberate comment forbidding `$PSNativeCommandUseErrorActionPreference`
  because it would destroy the diagnostic the verdict is read from.
- **One spawn site, cwd-anchored** (`wdio.conf.ts:405`):
  `spawn(process.execPath, [driverCli, '--native-driver', native], { cwd: repoRoot, env: appEnv })` —
  array-form, no shell. The app is NOT launched here: msedgedriver launches it
  (`msedgedriver.log`: `Launching Microsoft Edge: "…\target\release\conductor-tauri.exe"`), so the
  process chain is `pwsh (elevated) → node/wdio → node/tauri-driver → msedgedriver → conductor-tauri.exe`.
- **`browserName` is translated, not passed through**: the capability block declares `'wry'`
  (`wdio.conf.ts:339`) while the driver's `InitSession` records `"browserName": "webview2"` with
  `ms:edgeOptions.binary` — tauri-driver performs the translation. The real path therefore DOES use the
  host-app launch mode whose omission invalidated an earlier isolation probe (2026-09-07 learning).

## Conventions to follow
- **Assert the token the runner PRINTS, measured** (`verification-harness.md:64`): the `Spec Files:` line
  OMITS the `failed` term when nothing failed; the skip tally is the per-spec `-` markers, never `pending`.
  `Assert-A11yVerdict` already encodes both.
- **A captured `runs/a11y-e2e.log` is UTF-16** (`verification-harness.md:66`, and `agent-run.ps1:270`
  writes it with `*>`): grep returns ZERO on it without decoding. Any new assertion over that capture
  inherits this.
- **GitHub Actions echoes each step's own source into the job log** (`verification-harness.md:66`): a
  literal inside a `Write-Output "…"` template appears TWICE; the tell is the unexpanded `$var`, the fix
  a `| grep -av 'Write-Output'` stage. Verified live this pass — the `[precondition]` sweep over run
  35079315258 returned both the echoes (`ci.yml` source, `:7914-7922`) and the one real output line
  (`:7938`); the real line is the one read.
- **A leg that boots an external process ends with a process census, taken twice, pattern derived from
  what the leg can START** (`verification-harness.md:58`) — and it must include `msedgewebview2.exe`,
  omitted once before, which made attribution impossible.
- **No smoke entry when the chunk has no producing leg** (`verification-harness.md:49`, extended
  2026-09-11 on THIS chunk's direct predecessor): a chunk whose whole surface is a CI workflow step plus
  a script has no boot path, so a listed bare verb satisfies the authoring check while asserting nothing.
  Prefer the stated-absence arm.

## New files to create
- *(provisional — decided by the P4 approach fork)* `scripts/a11y-limited-token-launch.ps1` — if outcome
  (A) is attempted via a de-elevating wrapper. Arch registers CI-only probe scripts under `scripts/`
  (`webview2-cause-probe.ps1` precedent: invoked solely by `ci.yml`, wired into neither harness shell),
  and the prior chunk rejected "an inline probe body in `ci.yml`, or a second script" for its own probe —
  so whether this is a new file or an extension of the existing one is a P4 decision, not settled here.

## Files to modify
- `.github/workflows/ci.yml` — the `a11y` job: the launch context of the asserting step (`:380-401`),
  and, if outcome (B), the exclusion's recorded form. The asserting step's own guard shape is already
  correct and must stay byte-equivalent in that respect.
- `crates/conductor-tauri/ui/wdio.conf.ts` — *only if* the token drop lands at the spawn site (`:405`)
  rather than at the step. Graph says the blast radius is this file alone.
- **Companion sweep** (`grep -rn` over `*.ts,*.rs,*.ps1,*.sh,*.yml,*.toml`, node_modules and `target/`
  excluded): `'A11y routine arm'`: **1 hit** · 1 changed (`ci.yml:380`, the step itself) · 0 no-change.
  `CONDUCTOR_A11Y_STRICT`: **8 hits** · **0 changed** · 8 no-change — `ci.yml:344,383` (comment + the
  set site) and the three READERS `wdio.conf.ts:69,77`, `agent-run.ps1:72,73`, `agent-run.sh:197,199`,
  all of which every extract requires to stay byte-unchanged (security-plan's "ADDED arm, never a
  relaxation"). The reader set is recorded here precisely so the plan cannot drift into it.

## Premise closure + mechanism re-derivation
- **`[inferred]` the CARRY's causal claim → VERIFIED.** Re-derived at HEAD, not recalled. The cited file
  exists and carries a controlled variation: `reading.md:92` grades elevation **ESTABLISHED** "by direct
  variation with a control on both sides"; `:137-147` tables the pair differing on `IsElevatedAdmin`
  alone (same machine, driver, runtime, tree, session) with the non-elevated known-good at 12 passing /
  2 skipped; `:162-163` scopes it to the dev host with the runner's elevation measured independently.
  Run 35079315258 is a **third, independent** confirmation: `IsElevatedAdmin: True` on the runner with
  every rival cause measured away in the same run.
- **The MECHANISM stays unestablished, and the plan must not assert it.** `reading.md:158` marks it
  `hypothesis:` in the source — WHY WebView2 declines the endpoint under an elevated token is not
  derivable from this repo, and nothing this pass measured establishes it. The CARRY's own marker text
  ("recorded, not established") is therefore accurate at HEAD and is preserved.
- **`[inferred]` the next arm is a limited-token launch → VERIFIED as the sanctioned next step, and its
  cost is now earned.** The prior chunk's `plan.md:371-375` REJECTED de-elevating on the runner for
  itself with an explicit condition — "It earns its cost only if arm B leaves elevation standing" — and
  arm B did leave it standing. The same line names the risk this chunk inherits: a scheduled-task or
  `runas` token dance has "failure modes [that] would then have to be separated from the endpoint's".
- **`[inferred]` no new third-party dependency → VERIFIED.** `schtasks` / `runas` / `Start-Process
  -Credential` are OS built-ins; none enters security-plan's third dependency class (a CI-time-fetched
  binary), so no Authenticode gate is required for this arm.
- **Correction to a supplied count (re-derived, not copied).** The take-up directive states the arm
  failed "after three session attempts over ~63 s". The ~63 s holds (`Spec Files: 0 passed, 1 failed,
  1 total (100% completed) in 00:01:03`). The attempt count does not: the routine-arm step's log carries
  ONE session-creation failure event rendered on four lines at a single instant (09:33:43.564, .565,
  .615 and the reporter echo at 09:33:44.03), with wdio's summary reporting `(1 retries)`. Derivation:
  `grep 'A11y routine arm' <log> | grep -n 'DevToolsActivePort'` → 4 lines, all at one timestamp.
- **The `(1 retries)` does not breach the zero-flakiness bar.** `.config/nextest.toml:7,17` sets
  `retries = 0` in both profiles; `wdio.conf.ts` sets no `specFileRetries` (grep: no match), so the
  count is wdio's own default spec-file behaviour, not a configured retry policy — and the run still
  FAILED, so nothing was masked. This settles the tests extract's research question.

## Open questions
1. Is a non-elevated principal actually reachable inside a hosted `windows-2025` step, and does the app
   survive the token drop — working directory, `RUNNER_TEMP`/`TEMP` on **different volumes** (measured
   `[diag] TEMP and RUNNER_TEMP share a volume: False`), and profile access for `EBWebView`?
   → blocks: **plan-decision** (it decides whether (A) is attemptable at all, and at which level of the
   process chain the drop is placed).
2. Does `--remote-debugging-pipe` offer a route that needs no token drop? The driver's own log recommends
   it over the port file (`msedgedriver.log`: "Use the --remote-debugging-pipe Chrome switch instead of
   the default --remote-debugging-port"), and the driver currently passes `--remote-debugging-port=0` via
   `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`. Whether tauri-driver/WebView2 honour the pipe transport is
   unverified here. → blocks: **plan-decision** (a second candidate approach for the same outcome).
3. If (B): does the exclusion take design-system's `Blocked` shape (never measured) or `KnownResidual`
   (measured but pre-accepted)? The measured cause points at `KnownResidual`, but the choice changes
   what the a11y capability's matrix acceptance may claim. → blocks: **plan-decision**.
