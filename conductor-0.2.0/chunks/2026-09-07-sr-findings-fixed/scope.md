# Scope — SR findings fixed

**Marker:** `2026-09-07-sr-findings-fixed`
**Working entry:** `conductor-0.2.0/working-route.md:123` (Epoch 6b — Polish & ship)
**Promoted:** 2026-09-07

---

## What this chunk is

Two bodies of work the working entry binds together, both owed to the a11y/screen-reader surface this
entry owns:

1. **The three measured SR-leg findings, fixed at their defects** — re-announce after first focus (1),
   the sequential-focus start point measured then fixed (2), and the run-to-run announcement variance
   recorded (3). All three were measured at `2026-09-04-sidecar-spawn-without-a-console-window`, carried
   through `2026-09-07-a11y-ci-gate` without being discharged, and minted onto this entry by operator
   wrap-directive item 3a.

2. **The a11y CI gate's first run is RED — own the fix.** The clearing event the second CARRY names has
   REALIZED, and it realized red. That CARRY's `red = own the fix` arm is therefore live in this chunk,
   not a contingency.

The two are not independent: a11y-plan §11 `:516` names finding (2) as the stated reason the Operable
carve-out exists, so what this chunk does to (2) decides whether routine keyboard specs ever become
gateable.

---

## Body 1 — the CI red

### Measured state (operator directive, re-verified at HEAD)

CI run `34148079506` on `00181df` (push 2026-09-07T17:33Z): Frontend ✓, Rust ✓, **A11y gate ✗** at the
step *A11y routine arm (tauri-driver + axe + contrast)* — exit 1 two seconds after the 6 m 10 s release
build; the conformance step skipped; the upload found no `runs/a11y/*.jsonl`.

- **First cause, measured at source.** `.github/workflows/ci.yml:258` sets
  `CONDUCTOR_MSEDGEDRIVER: ${{ env.EDGEWEBDRIVER }}\msedgedriver.exe`. GitHub's `env` context "contains
  variables that have been set in a workflow, job, or step. It does not contain variables inherited by
  the runner process" (contexts reference, *env context*); the image's `EDGEWEBDRIVER` is a
  runner-process variable. The run's own env dump reads the literal `\msedgedriver.exe` — verified
  against the job log. **[VERIFIED at P3: `ci.yml:258` reads exactly that expression; the job's env
  dump prints `CONDUCTOR_MSEDGEDRIVER: \msedgedriver.exe`.]**
- **Consequence.** The handle resolved to a non-file, the wdio-edge guard found no driver, and the
  STRICT arm exited 1 as designed. **[VERIFIED at P3: `nativeDriver()` `wdio.conf.ts:56-61` returns
  `undefined` unless `existsSync && statSync().isFile()`; `strictMode()` :68-71; `exitUnresolvedHandle()`
  :75-81 — the directive's `:69-80` re-verified to that span.]** The gate behaved correctly; the
  wiring of the path did not.
- **The WebView2-runtime hypothesis is SECOND and untested.** It is the one `hypothesis:` the sibling
  chunk shipped unmeasured, and it stays unmeasured until the driver resolves. Do not treat a green run
  as evidence about it, and do not treat this chunk's fix as having tested it. `[inferred —
  hypothesis-marked in the directive, preserved]`

### A second defect, measured here, that the directive did not name

**CI swallowed the diagnostic output.** `scripts/agent-run.ps1:270-275` deliberately captures
`& npm run a11y *> $a11yLog`, reads `$LASTEXITCODE` from the bare command, then `Get-Content $a11yLog`
**prints** it, then asserts the printed verdict. In CI the step prepends
`$PSNativeCommandUseErrorActionPreference = $true` (`ci.yml:261`), which turns the native non-zero into
a terminating `NativeCommandExitException` at the `& npm run a11y` line — so `Get-Content` never ran and
the job log carries one line (`node.exe ended with non-zero exit code: 1`) instead of wdio's output.
The strict arm's own `console.log('error: CONDUCTOR_A11Y_STRICT is set, so an unresolved handle is a
GATE FAILURE, not a skip.')` went to the log file and never reached the operator. **[VERIFIED at P3,
and it is a PARITY defect, not merely a CI one: `agent-run.sh:247` writes
`( cd "$UI_DIR" && npm run a11y ) > "$a11y_log" 2>&1 || e2e_rc=$?`, whose `|| …=$?` survives a non-zero
under `set -euo pipefail`, so the `.sh` arm always reaches its `cat "$a11y_log"` at :248. The `.ps1`
arm's `$LASTEXITCODE` read is correct in isolation and is preempted only by the workflow's own
preference — so the two shells' "identical semantics" contract (test-plan §3) holds on this host and
BREAKS under CI. The evidence is the absence from the job log of the strict arm's own
`error: CONDUCTOR_A11Y_STRICT is set …` line, which `exitUnresolvedHandle()` provably printed to the
captured file.]**

This is in scope because it cost real diagnostic capability on the gate's very first run: the harness's
capture-then-print design is defeated by the workflow's own error preference, and the next red run would
be just as opaque.

### Fix shape (the directive names the form; the chunk chooses and justifies)

Resolve the path in the step's **shell**, where runner-process variables are visible:
`$env:CONDUCTOR_MSEDGEDRIVER = Join-Path $env:EDGEWEBDRIVER 'msedgedriver.exe'` plus a **printed
`Test-Path` precondition line**, or write it to `>> $env:GITHUB_ENV` in a prior step. The `ci.yml:252-253`
comment (the image ships the driver, so no install step and no operator handle in CI) stays true under
either form and is not re-litigated. `[inferred — directive-supplied form]`

### The still-owed clearing event

`route-archive.md:224` holds the frozen *A11y CI gate* entry's `BLOCKED-ON: one green CI run of the a11y
job after a push — the push is the operator's act`. That entry is `complete` in master and its freight is
spent; the **verification** rides this chunk. The clearing event is a **GREEN** run — this chunk's fix
landing is necessary and not sufficient, and the build branch's push stays the operator's act.

---

## Body 2 — the three SR findings

Folded verbatim from the CARRY (marker text preserved; all three are measured, none fixed):

1. **The load-time alert is NOT announced.** R0-01 graded `not-announced` with `heard: []` once the
   reload was dropped, so the prior chunk's fix (four `role="alert"` regions mounting EMPTY at first
   paint) is necessary but NOT sufficient: NVDA binds a window on its FIRST FOCUS EVENT, which
   necessarily follows a load-time error, so the text arrives in a region nobody is listening to yet.
   **The remedy is a RE-ANNOUNCE mechanism (re-assert the message after first focus), not another mount
   tweak — that shape is disproved.** `[inferred — measured-marked, mechanism preserved]`

2. **The sequential-focus start point is still wrong.** `tabs_to_start` measured live 3 · empty 6 ·
   error 5 with `initial_focus: "BODY"`; `browser.refresh()` had been RESETTING it, which is why
   dropping the reload exposed the defect at R0-02 (the first Tab landed on "Close window", not
   "Minimize window"). **The holder is STILL UNMEASURED** — cmdk `^1.1.1` (verified in
   `crates/conductor-tauri/ui/package.json:24`) has no `autofocus`, both its `.focus()` calls are
   guarded, no `src/` file focuses at mount, yet the leg's own comment attributes it to "the picker
   input at mount". **Measure WHICH element holds Chromium's sequential-focus-navigation starting point
   BEFORE designing any fix; the leg's `tabsToStart` cycling loop would mask one.**
   **[premise-corrected at P3 — the CARRY's "still unmeasured" holds, but two things are now positively
   established and one candidate is EXCLUDED. (a) DOM order is NOT the cause: `Titlebar.tsx:51-58`
   renders Minimize BEFORE `:59-66` Close, matching the visible order layout-templates §Component —
   Header requires, so the first Tab landing on "Close window" cannot be a source-order defect.
   (b) The leg's own attribution is UNSUPPORTED by the code it names — `ScenarioPicker.tsx` contains
   zero `focus`/`ref`/`useEffect` occurrences, cmdk's `CommandInput` sets no autofocus, and cmdk's only
   `.focus()` path is guarded by `document.activeElement.hasAttribute("cmdk-input") ||
   …("cmdk-root")`, which cannot fire when nothing is focused. So "the picker input at mount"
   (`screen-reader.e2e.ts:80-83`) is a comment to re-measure, not a lead to build on. Two candidates
   the CARRY never named survive: `activate-window.ps1`'s synthetic ALT `keybd_event` pair (still
   present), and the `tabsToStart` loop's own BODY reading, which cannot distinguish document `<body>`
   from host chrome outside the webview. The measurement step stands and must not assume any of
   these.]**

3. **The leg has real run-to-run announcement variance** — a harness-reliability finding against
   test-plan §10's zero-flake bar (`test-plan.md:500`: "A runner-dependent result is in this class — a
   test green under one runner and red under the other is a shared-state defect to remove at the cause,
   never a runner to pin to"). Two identical runs graded S0-01/S0-02 `not-announced` then
   `announced-as-expected`, and three `empty`-subject rows (E0-06/E0-07/E0-09) moved the same way,
   correlating with that subject's `nvda_named_window` flipping. **ONE observation cannot separate
   variance from regression — take a second before concluding.** The entry asks for this to be
   *recorded*, not necessarily eliminated. `[inferred — measured-marked]`

### The carve-out this touches

a11y-plan §11 `:516` (CARVE-OUT, recorded 2026-09-07) states the Operable half stays operator-local
"until a hold-free keyboard path exists on the routine arm", and gives the reason explicitly: "**the
sequential-focus start-point defect is open and its holder unmeasured, so routine keyboard specs would
gate on a known-red subject**". Finding (2) is that defect. Closing it removes the stated blocker — it
does not by itself add routine keyboard specs. Whether this chunk goes on to do that is a **scope fork
for the operator at P4/P5**, because it is what `v2-24` ("keyboard PASS/FAIL … gated in CI") would need
and that capability is currently pooled with a failed-concretization note naming exactly this reason.

---

## Boundaries

- **No live Pulse is required for Body 1.** The routine `--e2e` arm seeds its own committed fixture; the
  `a11y:driven` and `sr*` suites do need one and stay operator-local (architecture §Established
  Decisions [CI/CD]).
- **The SR leg needs the host NVDA** (`CONDUCTOR_NVDA`) and is operator-local. Grading rows 1-3 is an
  agent-driven leg on this Windows host, not a CI addition. Do not attempt to move any `sr*` suite into
  CI. **[P3 refinement — only ONE of the three findings needs a live Pulse.** `wdio.conf.ts:157-161`
  maps the subjects: `sr` = `live` (the shell must also supply
  `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios`), `sr-empty` = `empty` (catalog `runs/sr-leg/empty`),
  `sr-error` = `error` (catalog `runs/sr-leg/bad`). Findings (1) and (2) are rows R0-01/R0-02 on the
  **`sr-error`** subject and finding (3)'s E0-06/07/09 are on **`sr-empty`** — neither needs Pulse.
  Only finding (3)'s S0-01/S0-02 half rides `sr`, the live subject. **Measured at P3 (2026-09-07T18:2xZ):
  `127.0.0.1:4317` not connectable · `andromeda-pulse-mcp` not on `PATH` · all three `ANDROMEDA_PULSE_*`
  unset · `CONDUCTOR_NVDA` unset · `CONDUCTOR_MSEDGEDRIVER` declared** — so every `sr*` suite would skip
  at exit 0 today, and the environment is an operator precondition the plan must name rather than
  assume.]
- **The strict-mode guard is not relaxed to obtain redness or greenness.** security-plan §Anti-Patterns
  → Input; the wdio-edge guard (existence + `isFile` + shell-metacharacter rejection) stays byte-intact,
  and only what an unresolved handle COSTS is at issue — and that is already shipped.
- **The build-branch push stays the operator's act.** A CI-probe branch is the in-chunk verification
  form; it never substitutes for the operator's push of `build/conductor-0.2.0`.
- **No spec master is amended here** (phase is read-only on specs; wrap owns amendments). Where this
  chunk's measurement contradicts a11y-plan §11 `:516`'s premise, that is a wrap-time reconcile.
- Out of scope: the WebView2-runtime hypothesis (second, untested until the driver resolves); the
  `knip.json` config and the a11y-plan `:565` clause (both DISCHARGED by `2026-09-07-a11y-ci-gate`).

---

## Surfaces and contracts touched

| Surface | File | Nature |
|---|---|---|
| CI a11y job | `.github/workflows/ci.yml` (:254-262 step; :248-253 comments) | the driver-path resolution + the diagnostic-print defect |
| Harness verdict arm | `scripts/agent-run.ps1` (:68-99 `Assert-A11yVerdict`, :258-277 `--e2e`) | printed-verdict assertion under a CI error preference |
| Harness parity | `scripts/agent-run.sh` (`assert_a11y_verdict`) | sh/ps1 parity is a standing invariant |
| wdio edge | `crates/conductor-tauri/ui/wdio.conf.ts` (:68-81 strict arm) | READ-ONLY for Body 1 — the guard is correct |
| SR leg + spec | `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` + `test/a11y/screen-reader/` | findings 1-3 |
| App under test | `crates/conductor-tauri/ui/src/**` | the re-announce mechanism (1) and the focus-start-point fix (2) |
| Violation record | `runs/a11y/<run_id>.jsonl` via `writeA11yEnvelope` | must be produced by a green run (the conformance step consumes it) |

## Verification-matrix bearing

- `v2-24` is pooled with a failed-concretization note whose stated reason is finding (2). Its acceptance
  also carries a stale `Linux+xvfb` platform clause the frozen `:121` entry already calls an ASSUMPTION.
- **[scope amended at P5 val-1 — intent-incomplete, resolved by operator ruling at P4.]** The chunk now
  ALSO carries a third body the working entry did not state: the routine arm gains the **hold-free
  Operable set only** — SC 2.1.1 keyboard reachability of the idle console's controls, and SC 2.4.3
  focus order per a11y-plan §5 run-console-idle (titlebar window controls → picker input). Focus-trap
  and focus-restoration stay in the driven suite, operator-local. `v2-24` is CLAIMED by this chunk,
  concretized to the routine arm's keyboard PASS/FAIL gated in CI — which its requirement text already
  names, so this is a concretization and not a weakening — with the `Linux+xvfb` clause rewritten to the
  measured Windows WebView2 runner. The CI proof is the in-chunk probe run, with the operator's push
  after the wrap re-proving it. The a11y-plan §11 carve-out retirement and the §5 record ride
  **Expected amendments (wrap)**, never a phase edit.
- **Environment ruling (operator, P4):** the FULL SR leg runs — the operator boots `pulse-app` and
  exports `CONDUCTOR_NVDA`, so all three findings are graded including the live `sr` subject. The P3
  measurement that only finding (3)'s S-rows need Pulse stands as a fact about the subjects, not as a
  reduction of the leg.
- No other pooled capability (`v2-04`, `v2-21`, `v2-27`) is in this chunk's lane.
