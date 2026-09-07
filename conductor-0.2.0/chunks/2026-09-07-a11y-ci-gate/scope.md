# Scope — A11y CI gate

**Marker:** `2026-09-07-a11y-ci-gate`
**Version:** conductor-0.2.0 · **Epoch:** Epoch 6b — Polish & ship
**Working entry:** `conductor-0.2.0/working-route.md:121` (first markerless head)
**Promoted:** 2026-09-07, taking up a `BLOCKED-ON` head on the operator's explicit take-it-anyway ruling.

---

## What the entry states (the surface scope)

- The a11y specs become a **CI gate** on whatever runner the project gets, **without Pulse**, **failing on
  any violation**.
- The gate emits **redacted, service-tagged JSON aligned to the obs-owned format**.
- **Linux+xvfb was an ASSUMPTION, not a capability verdict** — the Windows WebView2 path is the measured one
  (2026-09-01/02), so the gate is **runner-agnostic** and **only the CI arrangement is open**.

## Boundaries

- This chunk does **not** widen a11y coverage or author new WCAG assertions; it takes the specs that exist and
  makes them a gate. New assertions arrive only where a folded CARRY names one.
- It does **not** require a live Pulse: the gate is the **without-Pulse** arm. The driven / `sr*` suites stay
  operator-local. `[inferred]`
- It does **not** stand up a second browser-automation stack — a11y-plan `:565` forbids it and the existing
  WebdriverIO + tauri-driver stack is the one stack. `[inferred]`
- Scope law is untouched: the driver ports (`4444`/`4445`) live and die with the harness; no shipped binary
  gains a listener.

## Surfaces / contracts touched

- `.github/workflows/ci.yml` — the workflow that must gain the a11y job. Today it declares exactly two jobs,
  `rust` (`:16`, `runs-on` `:18`) and `frontend` (`:188`, `runs-on` `:190`), both `windows-latest`. `[inferred]`
- `crates/conductor-tauri/ui/test/a11y/` — `accessibility.e2e.ts`, `operator-hold.e2e.ts`,
  `screen-reader.e2e.ts` + `screen-reader/`. `[inferred]`
- `crates/conductor-tauri/ui/wdio.conf.ts` — the ONE config all suite families fire; owner of the
  `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` skip-at-exit-0 guards, which interact directly with what a CI
  runner can actually execute. `[inferred]`
- `crates/conductor-tauri/ui/package.json` — the `a11y` / `a11y:driven` / `a11y:sr*` / `knip` scripts. `[inferred]`
- `crates/conductor-tauri/ui/test/README.md` — CARRY 1's subject.
- `.andromeda/a11y-plan.md` — CARRY 4's subject (spec-internal inconsistency; read-only for phase).
- The obs-owned redaction contract: no absolute host paths, no internal struct names, service-tagged JSON.

## Capability in play

- `verification-matrix.json#v2-24` — *A11y CI gate and violation JSON* (`method: a11y`), currently
  `chunk: null` / `planned`; one of the version's four unclaimed capabilities. Claim decision is P5's. `[inferred]`

---

## Folded freight

Every annotation below was folded as a HYPOTHESIS and its **named coordinates re-verified against the
artifact** on 2026-09-07 before it shaped this scope. Mechanism truth is NOT settled here — that is P3's
scope-premise closure, and each preserved marker text sets the re-verification depth.

### BLOCKED-ON (ratified: taken up anyway)

> one green CI run of the a11y job after a push — the push is the operator's act.

**Re-verified 2026-09-07 — the block's supporting clauses are now DEAD, its substantive half STANDS:**

- ✅ *"both `ci.yml` jobs already run on `windows-latest`, so a CI runner EXISTS"* — CONFIRMED.
- ⚠️ The entry cites the frontend gate at `ci.yml:164`; it is actually at **`:188`**. The workflow grew since
  the 2026-09-04 measurement. The substantive claim (two `windows-latest` jobs) is unaffected. `[inferred]`
- ✅ *"the a11y job itself is absent from that workflow"* — CONFIRMED. Jobs are `rust` and `frontend` only.
- ❌ *"`origin` stands at `dd15dc3` (2026-08-09) with the local branch 49 commits ahead and unpushed — CI has
  not run in a month"* — **FALSIFIED.** Measured this session: the operator pushed, `origin` is at `e21361a`,
  the branch is **0 ahead / 0 behind**, and CI ran **green** at 2026-09-07 (run `34128631720`: Rust gate
  success, Frontend gate success) as well as on 2026-09-06 and 2026-09-05. `[inferred]`
- **The block therefore still stands on its live half only:** no a11y job exists, so no green a11y CI run can
  exist. Authoring that job is inside this chunk, which is why take-it-anyway is the arm that clears its own
  block. `[inferred]`
- The annotation's dead clauses are stale route text that nothing sweeps; correcting them is this chunk's to
  own at wrap. `[inferred]`

### CARRY 1 — ui test README states a falsified platform verdict
*(from `2026-09-02-screen-reader-manual-spec`, plan §Implementation notes)*

The README still opens "This leg runs only on Linux + xvfb against a live Pulse" — measured false (the leg
runs headfully on the Windows host; only macOS is driver-less). Reword it at THIS entry, which touches the ui
test tree, to the measured platform set.

**Re-verified — CONFIRMED and WIDER than stated:** `crates/conductor-tauri/ui/test/README.md` carries the
false platform verdict at **three** sites, not one — `:8` ("only on Linux + xvfb against a live Pulse"),
`:12` ("tauri-driver drives the platform WebView (WebKitGTK) under xvfb; there is no WKWebView/Windows path"),
and `:26` ("`npm run a11y` # Linux + xvfb + live Pulse only"). The reword is a file-level pass, not a
one-sentence edit. `[inferred]`

### CARRY 2 — the subject-absent guard is loose
*(from `2026-09-04-sr-findings-remediation`, operator WRAP directive 2026-09-04 item 3 — this entry owns the a11y suite)*

Tighten the operator-checklist spec's subject-absent guard from a bare `[role="status"]` to
`[role="alertdialog"]`. Measured: the bare token stands in for "a hold is raised" and was sound only while the
checklist roll-up was the app's only `role="status"`; two a11y-plan-mandated live regions added at that chunk
satisfied it, the skip stopped firing, and the spec asserted checkboxes it found none of. Worked around there
by taking `aria-live` instead, which leaves the guard still loose for the next `role="status"` anyone adds.

**Re-verified — CONFIRMED, target pinned:** the loose guard is
`crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts:300`
(`if (!(await $('[role="status"]').isExisting())) { this.skip() }`). Its **sibling guard at `:294` already
uses `[role="alertdialog"]`**, so the corrected shape has an in-file precedent. `[inferred]`
The CARRY's "worked around there by taking `aria-live`" does not match what `:294` actually took
(`[role="alertdialog"]`); the discrepancy is recorded, not resolved — P3 owns it. `[inferred]`

### CARRY 3 — three SR-leg findings, all measured, none fixed
*(from `2026-09-04-sidecar-spawn-without-a-console-window`, operator WRAP directive items 1+2, placement ruled at that wrap's P5 halt)*

**Coordinates re-verified:** rows `R0-01` / `R0-02` exist at
`crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts:303` / `:311`; the `tabs_to_start` field exists in
`screen-reader/parse-nvda-log.ts` (`:55` doc, `:64` type, `:459` emit); the originating evidence is
`conductor-0.2.0/chunks/2026-09-04-sr-findings-remediation/evidence/{leg-verdict.md,nvda-pass.json}`.

**(1) The load-time alert is NOT announced.** `[inferred]` — mechanism claim, marker text preserved verbatim:
> R0-01 graded `not-announced` with `heard: []` once the reload was dropped, so the prior chunk's fix (four
> `role="alert"` regions mounting EMPTY at first paint) is necessary but NOT sufficient: NVDA binds a window on
> its FIRST FOCUS EVENT, which necessarily follows a load-time error, so the text arrives in a region nobody is
> listening to yet. The remedy is a RE-ANNOUNCE mechanism (re-assert the message after first focus), not another
> mount tweak — that shape is disproved. Row R0-01 and its `expected` already say silence here is a FINDING,
> never a pass.

**(2) The sequential-focus start point is still wrong.** `[inferred]` — mechanism claim, marker text preserved:
> `tabs_to_start` measured live 3 · empty 6 · error 5 with `initial_focus: "BODY"`, unchanged by that chunk.
> NEW fact from it: `browser.refresh()` had been RESETTING the start point, which is why dropping the reload
> exposed the defect at R0-02 (`the first Tab landed on "Close window", not "Minimize window"`). The holder is
> STILL UNMEASURED — cmdk 1.1.1 has no `autofocus`, both its `.focus()` calls are guarded, no `src/` file
> focuses at mount, yet the leg's own comment attributes it to "the picker input at mount". Measure WHICH
> element holds Chromium's sequential-focus-navigation starting point before designing any fix; the leg's
> `tabsToStart` cycling loop would mask one.

`cmdk ^1.1.1` confirmed in `package.json:24`. `[inferred]`

**(3) The leg has real run-to-run announcement variance.** `[inferred]` — mechanism claim, marker text preserved:
> a harness-reliability finding against test-plan §10's zero-flake bar, recorded here rather than passed: two
> identical runs graded S0-01/S0-02 `not-announced` then `announced-as-expected`, and three `empty`-subject rows
> (E0-06/E0-07/E0-09) moved the same way, correlating with that subject's `nvda_named_window` flipping.
> Consequence for anyone reading this leg: ONE observation cannot separate variance from regression — take a
> second before concluding, as that chunk did to exonerate its own change.

**Direct bearing on this chunk:** a gate that FAILS ON ANY VIOLATION cannot be built over a suite with known
run-to-run variance without deciding which suites the gate actually runs. That tension is this chunk's to
resolve, not to inherit silently. `[inferred]`

### CARRY 4 — a pre-existing internal inconsistency in `a11y-plan.md`
*(from `2026-09-06-operator-gated-live-suite`, wrap P2 fan-out — surfaced as out of the detectors' scope, proposed by nobody)*

`:565` still reads "reduced-motion emulation is the one platform-dependent assertion", a verdict `:218`,
`:424` and `:597` already retired in an earlier chunk. Re-verify the four line numbers before acting — they
are the agent's, unre-derived at that wrap.

**Re-verified — ALL FOUR LINE NUMBERS ARE CORRECT.** The CARRY's caution was warranted but the coordinates
hold: `:565` carries the stale clause verbatim at the END of its line; `:218` states "no assertion in this
harness is platform-dependent (as measured 2026-09-01)"; `:424` labels the platform framing "**The superseded
framing, kept for provenance**"; `:597` records the caveat "**RESOLVED 2026-09-01**, and not as the platform
question it was framed as". The inconsistency is real and one-sided — `:565` is the lone unretired site. `[inferred]`
Note: `:218` additionally cites `ci.yml:18, :178` as the two `windows-latest` jobs; `:178` is stale by the same
drift as the BLOCKED-ON's `:164` (actual `:188`). `[inferred]`
This is a SPEC master; phase is read-only on it. Any correction is a wrap amendment, never a phase edit. `[inferred]`

### CARRY 5 — `knip.json` is OWED
*(from `2026-09-07-dependency-polish`, operator wrap-directive item 2 — this entry named as the owner, the do-it call staying the founder's at promotion)*

`knip` is installed on the web plane but its series is unusable — a `knip.json` entry-point config is OWED.
That chunk added `knip` as a `crates/conductor-tauri/ui` devDependency plus a `knip` npm script (report-only,
no gate, `npm audit --omit=dev` still 0 production vulns, knip added ZERO advisories — 38 dev-tree before and
after). Its first run returned **20 findings, all 20 false positives of ONE class**. `[inferred]` — mechanism
claim, marker text preserved verbatim:
> WebdriverIO discovers specs through `wdio.conf.ts`'s `specs`/`suites` config and loads devDependencies through
> its own plugin resolution, and neither is an import edge, so knip's reachability graph cannot see the package's
> largest consumer — it flagged the three `test/a11y/*.e2e.ts` specs as unused files and five devDeps as unused,
> four of which (`@axe-core/webdriverio`, `axe-core@4.12.0`, `colorjs.io@0.6.1`, `lighthouse@13.0.3`) are
> a11y-plan §3-mandated BY NAME AND PIN.

Nothing was deleted; every finding is dispositioned in that chunk's `evidence/knip-first-report.md`, and
a11y-plan §11 forbids acting on such a report against the harness. This entry is the named owner because it
owns the ui test tree and those specs ARE the false positives; knip ships a `wdio` plugin for exactly this.

**Until the config lands, the next code-audit record's A5 `dead-code-web` column must read "tool present,
series unusable (100 % FP)" — never a usable series, and never `tool-missing`, which is now false.**

**Re-verified — CONFIRMED:** `knip ^6.34.0` at `package.json:42` with the `"knip": "knip"` script at `:16`;
**no `knip.json` / `knip.ts` exists**, so the CARRY stands unmet. The evidence file
`conductor-0.2.0/chunks/2026-09-07-dependency-polish/evidence/knip-first-report.md` is present. All four
a11y-plan-pinned devDeps are installed at the named pins (`@axe-core/webdriverio ^4.12.1`, `axe-core ^4.12.0`,
`colorjs.io ^0.6.1`, `lighthouse ^13.0.3`). `[inferred]`

---

## Open questions for P3/P4

1. **Which suites does the gate run?** The routine `--e2e` / `a11y` arm is the without-Pulse candidate; the
   `driven` and `sr*` suites need a live Pulse and/or `CONDUCTOR_NVDA` and skip at exit 0 when unset. A gate
   that "fails on any violation" must not be satisfiable by a suite that skipped itself green. `[inferred]`
2. **What makes the gate red on a runner with no `CONDUCTOR_MSEDGEDRIVER`?** The current guard SKIPS at exit 0
   — correct for a local dev host, but a CI gate that skips is not a gate. `[inferred]`
3. **Violation JSON shape** — "redacted service-tagged JSON aligned to the obs-owned format" needs its exact
   fields pinned against obs-plan before implement. `[inferred]`
4. **Does CARRY 3's known variance bar the `sr*` suites from the gate?** See CARRY 3(3). `[inferred]`

---

## Premise closure (P3, 2026-09-07)

Every `[inferred]` bullet above was re-checked against the code. Full disposition in `research.md`
§Scope premise closure.

**VERIFIED — tags discharged:** the without-Pulse boundary · the one-stack ban · the `ci.yml` job
coordinates · the spec-file set · `wdio.conf.ts` as the single config · the npm scripts · `v2-24`'s
matrix state · the block's live half · CARRY 1's three README sites · CARRY 2's target and its `:294`
precedent · `cmdk ^1.1.1` · CARRY 4's four line numbers · every CARRY 5 particular.

**PREMISE-CORRECTED (three, none changing the work):**
- `[premise-corrected: the frontend gate is at ci.yml:188, not :164]` — the BLOCKED-ON's coordinate is
  stale by 24 lines, and `a11y-plan.md:218` carries the same drift as `:178`. The substantive claim
  (two `windows-latest` jobs, no a11y job) holds.
- `[premise-corrected: origin is at e21361a, 0 ahead / 0 behind, CI green run 34128631720]` — the
  BLOCKED-ON's "49 commits ahead and unpushed / CI has not run in a month" is dead. The block still
  stands on its only live clause: no a11y job exists.
- `[premise-corrected: the sibling guard at :294 took [role="alertdialog"], not aria-live; no aria-live
  guard exists in the specs]` — CARRY 2's account of the prior workaround is wrong. Its requested fix
  is unaffected and now has a confirmed in-file precedent.

**Scope ADDITIONS from research (these outrank the four open questions above):**
- **The Operable gap is measured, not theoretical.** The routine arm carries **zero** `browser.keys`
  calls; all keyboard/focus/trap coverage lives in `operator-hold.e2e.ts` — the `driven` suite, which
  test-plan §11 and architecture §CI/CD both bar from CI. a11y-plan §11 requires both contrast AND
  keyboard on every must-be-accessible path, so a without-Pulse gate cannot satisfy §11 as written.
- **The gate cannot key on the exit code.** `verification-harness.md:58` measured that `run --e2e`
  returns 0 both on a full pass and on a total skip. The prescribed remedy is asserting the SPEC LIST
  and reconciling the passing/skipped tallies. The *Release build and bundle* entry carries a CARRY
  asking for the same assertion — ownership is P4's to resolve.
- **The violation-JSON shape is settled**: the eleven-field run-report envelope, `a11y-plan.md:228-250`
  verbatim. Open question 3 above is answered.

**Amended at validation-1 (P5) — the fold absorbed two annotations it could not discharge:**
- **CARRY 3 is CARRIED FORWARD, not work this chunk performs.** All three findings are SR-leg and therefore
  CI-ineligible by construction; finding (2) is the stated reason the Operable carve-out was chosen. The
  promotion fold read them as absorbed work; they are not. Wrap must re-CARRY all three.
- **CARRY 4 needs an Expected amendment, not a code change.** Its subject is `a11y-plan.md:565`, a spec
  master — phase and implement are both read-only on it, so the correction rides wrap's amendment flow.
