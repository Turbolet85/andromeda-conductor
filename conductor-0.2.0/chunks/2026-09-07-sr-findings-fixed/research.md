# Codebase Research — 2026-09-07-sr-findings-fixed

## Scope
- **Depth:** deep (mature codebase, two bodies, one of them a live-leg lane) · **Reads:** 14 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read structurally (60 lines / 60 390 B; the `## Session Additions` entries run multi-KB per line, so an index of headers + per-entry introducers preceded offset-bounded reads of entries `:49`, `:51`, `:58`, `:59`). 4 additions applied: the SR leg's five-handle firing form, the re-activate-only-when-not-foreground correction, the process-census + stop-form obligation, and the "read THIS file, not the route entry" rule. `.claude/rules/host-win32.md` also applied (structural extraction of the long-line file; exit-code isolation).

## Files inspected
- `.github/workflows/ci.yml` (219-262) — the `a11y` job. `:258` sets `CONDUCTOR_MSEDGEDRIVER: ${{ env.EDGEWEBDRIVER }}\msedgedriver.exe`; `:261` prepends `$PSNativeCommandUseErrorActionPreference = $true`; `:252-253` carries the no-install-step comment.
- `scripts/agent-run.ps1` (68-99 `Assert-A11yVerdict`, 258-277 the `--e2e` arm) — capture to `$a11yLog`, `$LASTEXITCODE` from the bare command, `Get-Content` at `:273`, assertion at `:275`.
- `scripts/agent-run.sh` (193-226 `assert_a11y_verdict`, 236-251 the `--e2e` arm) — semantically identical assertion; capture at `:247` uses `… || e2e_rc=$?`, print at `:248`.
- `crates/conductor-tauri/ui/wdio.conf.ts` (40-92 guards, 150-170 SR subject map, 318-395 suites + the one spawn site) — `nativeDriver()`, `strictMode()`, `exitUnresolvedHandle()`, `SR_SUITES`, per-suite `CONDUCTOR_RUNS_DIR`.
- `crates/conductor-tauri/ui/src/App.tsx` (80-330, probed) — four `role="alert"` regions at `:245`, `:272`, `:285`, `:309`, all mounted unconditionally; the rationale comment at `:241-244`.
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` (full, 70 lines) — the decisive DOM-order read for finding (2).
- `crates/conductor-tauri/ui/src/components/ScenarioPicker.tsx` (probed) — zero `focus` / `ref` / `useEffect` occurrences.
- `crates/conductor-tauri/ui/src/main.tsx` (full) — no mount-time focus; `StrictMode` + `createRoot` only.
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` (55-120) — `bringToForeground()`, `initialFocus`, the `tabsToStart` cycling loop.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/activate-window.ps1` (full) — the synthetic ALT `keybd_event` pair + `SetForegroundWindow`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` (rows S0-01, S0-02, R0-01, R0-02).
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (the four `this.skip()` sites).
- `crates/conductor-tauri/ui/package.json` (the five `a11y*` scripts; `cmdk` pinned `^1.1.1`).
- `conductor-0.2.0/route-archive.md:224` — the frozen *A11y CI gate* entry's archived `BLOCKED-ON`.

## Graph impact (code-graph, `ts` plane, `db_state: fresh`)
Query: `SELECT symbol, kind, file, def_line FROM symbol WHERE name IN (…7 names…) AND kind <> 'meta'` → **`rows: 6`** (trace `tree-query-2026-09-07-sr-findings-fixed.json`; the terminal view was `tail`-clipped to 4 — the trace's `rows` field is the basis, per the cookbook).
- **`nativeDriver`** — `wdio.conf.ts:56` · **`strictMode`** — `:68` · **`exitUnresolvedHandle`** — `:75` · **`nvdaExe`** — `:182` · **`startNvda`** — `:199` · **`writeA11yEnvelope`** — `:268` (editor lines; graph `def_line` is 0-indexed).
- **`reactivateWindow` returned no row, and that is a real finding, not an index gap**: `grep -rn 'reactivateWindow' crates/conductor-tauri/ui/` returns nothing. It was retired by `2026-09-04-sidecar-spawn-without-a-console-window`, exactly as harness rule `:59`'s 2026-09-04 correction records.
- All six live in ONE file, so this chunk's harness-side blast radius is `wdio.conf.ts` plus the two shells — no cross-module threading.

## Patterns detected
- **Capture-then-print-then-assert** (`agent-run.sh:247-250` · `agent-run.ps1:270-275`): the exit is read from the BARE command, the log is printed, and the PRINTED verdict decides. This is the shape test-plan §3 mandates and the shape CI defeats.
- **Guard / cost separation** (`wdio.conf.ts:56-81`): `nativeDriver()` decides whether a handle RESOLVES; `strictMode()` decides only what an unresolved handle COSTS. Both skip sites route through one `exitUnresolvedHandle()`.
- **Unconditionally-mounted announce regions** (`App.tsx:241-245`): the region mounts empty at first paint so the message arrives as a CHANGE — the prior chunk's fix for four sibling error sites.
- **Per-suite subject env at ONE spawn site** (`wdio.conf.ts:366-395`): `CONDUCTOR_RUNS_DIR` (and `scenarios` where the subject needs a trimmed catalog) chosen from `SR_SUITES` by the invoked suite.
- **Context-skip is never a pass** (`accessibility.e2e.ts`, four sites): each `this.skip()` carries a comment naming the absent subject.

## Conventions to follow
- **Identify CI jobs by NAME, not `ci.yml` line coordinate** (test-plan §9, established by `2026-09-06-coverage-completeness-gate`). This research cites lines for precision at HEAD; the plan's durable references are the job name `a11y` and the step name.
- **Both shells move together** (test-plan §3 5-command-discipline-wire; architecture §Design Philosophy) — `agent-run.sh` is the release gate, `.ps1` is at identical semantics.
- **Never relax the guard to obtain a colour** (security-plan §Input Validation → `CONDUCTOR_A11Y_STRICT`; a11y-plan §11 CI) — `nativeDriver()` / `nvdaExe()` / `UNSAFE_PATH` stay byte-unchanged.
- **Handle names, never handle values**, in any printed precondition (security-plan §Input Validation; architecture §Occupied Resources — Environment variables).
- **No `tabindex > 0`** (a11y-plan §11 Keyboard) — a focus-order fix comes from natural DOM/Radix order.
- **No in-test `sleep(N)`** (test-plan §11 E2E; the carve-out was proposed and REJECTED at `2026-09-06-operator-gated-live-suite`) — finding (3)'s variance may not be settled by widening a settle window.
- **Every leg that boots an external process ends with a process census, and names its stop form** (harness rule `:58`; extended `:58` — the census names WHO STOPS each survivor).

## New files to create
- (none) — every change lands in files that exist.

## Files to modify
- `.github/workflows/ci.yml` — resolve the driver path in the step's shell (runner-process env is visible there, the `${{ env.* }}` expression context is not), print a handle-named `Test-Path` precondition, and stop the step's error preference from preempting the harness's own print.
- `scripts/agent-run.ps1` — whatever the fix shape requires to keep `Get-Content $a11yLog` reachable under a CI error preference; parity twin of any change to `agent-run.sh`.
- `scripts/agent-run.sh` — parity twin (test-plan §3 identical semantics). May end byte-unchanged if the fix is CI-side only; the parity CHECK is owed either way.
- `crates/conductor-tauri/ui/src/App.tsx` — finding (1)'s re-announce mechanism (re-assert the load-error text after the first focus event), sited as a sibling per layout-templates' region-siting precedent.
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` — finding (2)'s measurement instrument (record WHICH element holds the sequential-focus starting point before the `tabsToStart` loop masks it) and finding (3)'s per-row variance recording; the leg's `:80-83` comment is corrected by whatever the measurement shows.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` — a row for the re-announce mechanism (a11y-plan §3 requires a new announcement to enter the must-announce inventory).
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` — **only if** the measurement implicates it; DOM order is already correct, so this is contingent, not planned.

## Open questions
- **Which element holds Chromium's sequential-focus-navigation starting point on the `error` subject?** → blocks: **implementation-scope**. DOM order is excluded (`Titlebar.tsx:51-58` Minimize precedes `:59-66` Close) and the leg's own attribution to "the picker input at mount" is unsupported (`ScenarioPicker.tsx` has no focus site; cmdk's `CommandInput` sets no autofocus and its single `.focus()` path is guarded by `document.activeElement.hasAttribute("cmdk-input") || …("cmdk-root")`). Two unexamined candidates: `activate-window.ps1`'s synthetic ALT, and the `tabsToStart` loop's inability to distinguish document `<body>` from host chrome. The fix design depends on the answer, so the plan's measurement step must precede its remedy step.
- **Does the CI fix remove `$PSNativeCommandUseErrorActionPreference = $true`, or work around it?** → blocks: **plan-decision**. Removing it restores the harness's own capture-then-print (the `.sh` arm already survives by `|| e2e_rc=$?`) and GitHub's `pwsh` shell still propagates the script's final `$LASTEXITCODE`; keeping it requires restructuring the `.ps1` arm. Resolved at P4 as a stated lean.
- **Can findings (1)-(3) be graded this session?** → blocks: **plan-decision**. Measured at P3: `CONDUCTOR_NVDA` unset and Pulse down, so every `sr*` suite skips at exit 0 today. Findings (1)/(2) (`sr-error`) and finding (3)'s E-rows (`sr-empty`) need NVDA only; finding (3)'s S-rows need a live Pulse as well. Put to the operator at P4.

<!-- Coordinate correction for the plan's benefit: harness rule `:58` names the expected-skip pair as
`accessibility.e2e.ts:295`/`:301` and the fixture-seeded pair as `:239`/`:251`. At HEAD the four
`this.skip()` sites are `:276` and `:288` (fixture-seeded) and `:332` and `:342` (live-hold). The SET
semantics — two live-hold subjects expected, a third skip means seeding failed — is unchanged; only the
coordinates moved, which is precisely why test-plan §9 requires identifying by name rather than line. -->
