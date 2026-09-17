# Scope — Medium-integrity launch for the a11y routine arm

**Marker:** `2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm`
**Version:** conductor-0.3.0 · Epoch 3 — The a11y capability's terminal
**Promoted:** 2026-09-16

## Intent (from the working-route entry, verbatim)

> Medium-integrity launch for the a11y routine arm — the a11y job's leg runs at Medium integrity and
> the routine arm's asserted verdict is read from a CI run that reached its assertions, or the CI half
> is closed as a ratified exclusion resting on THIS measured cause

This is the successor to `2026-09-16-a11y-ci-gate-at-an-honest-terminal`, which left the terminal
**UNDECIDED** and shipped a cause instead. The two allowed outcomes are unchanged from the predecessor
(green arm, or ratified exclusion); what is new is that one specific remedy is now named by measurement
rather than by hypothesis, and the two mechanisms already tried are retired **by mechanism**.

## The premise this chunk inherits

**PREMISE (measured, not assumed):** the mandatory INTEGRITY LABEL, not the administrator role, decides
whether WebView2 grants a debugging endpoint. Established 2026-09-16 by variation with a control on both
sides across three dev-host legs at runtime 153 — A admin+High → no session (`DevToolsActivePort`, 2:00) ·
B non-admin+High → no session (2:00) · C non-admin+Medium → session created
(`[webview2 153.0.4234.32 windows]` ×35, 0:06); A→B varies the role with integrity held (no change, role
exonerated), B→C varies integrity with the role held (the session appears).

Evidence re-verified at promotion:
`conductor-0.3.0/chunks/2026-09-16-a11y-ci-gate-at-an-honest-terminal/evidence/integrity-level-is-the-discriminator.md`
(present, 3,660 B).

The marker text `(measured, not assumed)` is preserved verbatim because it sets the re-verification depth
P3 owes this claim. Statedness is not measurement at HEAD: the legs were measured on the **dev host** at
runtime 153, and the subject of this chunk is the **runner**, whose session context differs. P3's scope
premise closure decides whether that gap needs closing before the remedy is built.

## What this chunk builds

**SUBJECT:** an explicit medium-integrity launch — `CreateRestrictedToken` + `SetTokenInformation` setting
the mandatory label to the Medium SID, then `CreateProcessAsUser`.

Both mechanisms the predecessor shipped are retired BY MECHANISM, not by exhaustion:

- `RunLevel Limited` cannot lower the label — the runner's account is the built-in Administrator (RID 500)
  with `FilterAdministratorToken` absent, so no filtered token exists to drop to.
- `runas /trustlevel:0x20000` cannot either — it strips the Administrators group and leaves the label
  (measured on the runner: role False, integrity `S-1-16-12288`).

A **seventh** governed harness-spawn form is already registered for the launcher this entry replaces or
extends; an **eighth crossing escalates again** (security-plan §Security Anti-Patterns → Code Patterns
rule (b)). Whether this chunk's launcher is a replacement of the seventh form or an eighth crossing is a
scope question P2/P4 must settle explicitly — it is an escalation trigger either way, not a detail.

## Boundaries

- **In scope:** the launch mechanism for the a11y job's routine leg, its CI wiring, and reading the
  routine arm's asserted verdict from a run that reached its assertions — or, failing that, composing the
  ratified exclusion on the measured cause with a named owner.
- **Not in scope:** the `:384`/`:397` runtime-153 regression (separate defect, own owner — see CARRY 1);
  the screen-reader census surface (CARRY 3, surface 2); building the console footer `contentinfo` strip
  (see the matrix-carried obligation below).
- **Trust boundary:** unchanged. No new inbound listener; the driver ports `4444`/`4445` live and die with
  the harness, as the scope law already admits for the dev-only webview legs.

## Folded annotations (from the working entry, all 7 at annotation positions)

### CARRY 1 — integrity buys the SESSION, not a green arm

Keep the two questions apart. A green CI arm ADDITIONALLY needs `accessibility.e2e.ts:384` (SC 2.1.1
reachability) and `:397` (SC 2.4.3 focus order) to hold at the runner's WebView2 152, where the 2026-09-10
record says they passed. Both were added by `06db2f9` in 82 lines with their CI proof recorded as OWED, and
both now FAIL on a dev host floated to 153 (10 passing / 2 failing / 2 skipped) — a **dated advance
warning**, red in CI the day the runner image moves, and a separate defect with its own owner.

Re-verified at promotion: `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts:384` is the
`expect(… reached …)` assertion inside `it('every idle-console control is keyboard-reachable by Tab alone
(SC 2.1.1)')` (:373); `:397` is the `expect(names[0] then names[1] …)` inside `it('idle focus order follows
the run-console-idle layout: window controls, then the picker (SC 2.4.3)')` (:393). Commit `06db2f9` is
`feat(2026-09-07-sr-findings-fixed)`. All three coordinates hold exactly as written.

### CARRY 2 — WINDOW STATION is a NAMED, UNVARIED risk

No longer the last candidate standing now that integrity is measured, but **not excluded** as a SECOND
contributor on the runner, whose session context differs from the dev host's. The witness already reports
`WindowStation` / `UserInteractive` / `SessionName` from inside the leg, so the axis is observable at zero
extra cost on the next probe.

### CARRY 3 — ONE hygiene defect with THREE surfaces, measured not noticed

Orphaned `msedgewebview2` processes survive every leg because stopping a driver does not kill the browser
hosts it launched.

1. They hold inherited redirect handles and pin the parent — the ~15-minute CI tail after a dead leg.
2. `CENSUS_NAMES` at `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts:130-137` omits
   the webview host, so the leg census cannot see them — SR-leg territory (all three call sites are
   screen-reader-gated), so that surface is this CARRY's, **not the launcher's**.
3. They accumulate across runs.

The predecessor's reap improved survivors 7 → 17 reaped and is explicitly NOT done — 6 in-window survivors
at the last measurement, and its own `OrphansLeft` under-reports because it counts while the tree is still
settling.

Re-verified at promotion: `CENSUS_NAMES` is exactly at `parse-nvda-log.ts:130-137`, a six-name `Set`
(`nvda.exe`, `conductor-tauri.exe`, `msedgedriver.exe`, `node.exe`, `andromeda-pulse-mcp.exe`,
`pulse-app.exe`) with no webview host — the omission is confirmed. **Coordinate correction:** the entry
cites the three `census()` call sites as `wdio.conf.ts:240,396,450`; the file is at
`crates/conductor-tauri/ui/wdio.conf.ts`, **not** under `test/a11y/`. The line numbers are exact and all
three sit in screen-reader-gated paths, so the claim holds — only the unqualified path needed correcting.

### CARRY 4 — the Evergreen float charged a cost BEFORE the gate bounding it ever lit

An unprompted overnight runtime bump broke the arm's two newest assertions with no repository input
changing. `security-plan.md` §Dependency Security bounds the float with "pin once the a11y job actually
GATES"; that disposition returns to the pool with `v3-02` and should be taken knowing the float has now
demonstrated this failure mode unprompted.

Re-verified at promotion: security-plan.md:185 states the exit condition verbatim — "once the `a11y` job
actually **GATES** (the remote-debugging endpoint opens and the routine arm runs to completion), the
runtime is pinned to a version, at which point the versioned Standalone Installer replaces the
bootstrapper." The current posture is floor-conditional (fetch only below 152; the `≥ 152` floor assertion
runs every job, outside the conditional).

### CONTEXT — `v3-02` returns to the pool for this entry

`v3-02` was un-claimed at the predecessor's wrap — the requirement is right and the world is wrong — and
returns to the pool for this entry. Operator-directed at the 2026-09-16 wrap.

Re-verified at promotion: `v3-02` reads `status: planned`, `chunk: None`, `ref: None`, and appears in the
unclaimed pool of 6. Its acceptance was PREMISE-CHECKED and carried forward unchanged at the predecessor's
P5 — deliberately NOT concretized, because concretizing it into a limited-token launch would have placed
every clause downstream of the very premise that chunk was measuring. That reasoning is now spent: the
premise IS measured, which is what makes concretization a live question for this chunk's P5.

## Matrix-carried obligations on `v3-02` (NOT stated by the working entry) `[inferred]`

`v3-02`'s `notes` carry two planning obligations folded at the 2026-09-11 route Phase 4 on operator
direction. The working entry names neither, so both enter scope tagged `[inferred]` for P3 to close:

1. **WebView2 runtime pin disposition.** This capability's terminal fork IS the float's stated exit
   condition, so whichever way the terminal lands, the float is dispositioned in the same chunk — pinned
   if the job gates, retired if the CI half is excluded. This is the same obligation CARRY 4 restates from
   the entry side; the two agree.
2. **Landmark coupling to an unbuilt surface.** The console footer `contentinfo` strip is designed but not
   shipped, and a11y-plan bans a landmark-less window. If the routine arm starts gating, that ban could
   turn the unshipped footer into a RED **inside the very gate this capability is trying to make green**.
   This is an INDEPENDENT second cause of red, living in a surface 0.3.0 deliberately does not build, so it
   cannot be reasoned about from the endpoint question alone. The note is explicit that planning **must
   MEASURE** whether the routine specs pass against the console as it stands before promising a green
   terminal; a promise made without that measurement is the failure the note predicts.

Obligation 2 is the sharpest planning input this chunk has and it is invisible from the working entry
alone. P3 owes a measurement, not an argument.

**P3 CLOSURE — obligation 2 measured and dissolved.** The measurement was taken (four independent ways, see
premise 2 below and `research.md`): the routine arm cannot raise a landmark violation, because axe runs only
WCAG A/AA-tagged rules and every landmark-family rule in axe-core 4.12.0 is `best-practice`. The unshipped
footer is therefore NOT an independent second cause of red inside this gate, and the green terminal this
chunk may promise is not exposed to it. Obligation 1 (the Evergreen pin disposition) stands unchanged and
is still owed whichever way the terminal lands.

## Premises closed at P3 (evidence in `research.md`)

1. **Dev-host integrity finding transfers to the runner** — **STILL OPEN, instrument now named.**
   Unmeasurable from this host by construction. The operator's phase directive (2026-09-16) authorizes the
   ci-probe for this chunk, which is the only instrument that takes this measurement before the chunk
   ships; CARRY 2's window-station axis rides the SAME probe at zero extra cost, since
   `scripts/a11y-token-witness.ps1:88-100` already prints `WindowStation` / `UserInteractive` /
   `SessionName` from inside the leg. Carried to the plan as the decisive implementation-scope question.

2. `[premise-corrected: the routine arm cannot see the landmark gap — axe-core 4.12.0 tags every
   landmark-family rule best-practice, and WCAG_TAGS selects none of them]` **The unbuilt `contentinfo`
   footer is NOT a second cause of red in the routine arm.** Matrix obligation 2 demanded a measurement
   rather than an argument; four independent measurements agree — the arm runs `accessibility.e2e.ts`
   alone (`wdio.conf.ts:320`); that file makes no landmark assertion across its 14 `it()` blocks; axe runs
   `withTags(['wcag2a','wcag2aa','wcag21aa'])` and none of the seven landmark-family rules is selected
   (68 of 105 rules run); and the only three `contentinfo` mentions in the test tree are SR-arm records of
   its ABSENCE as a finding. The DOM gap is real (`contentinfo` and `<footer` both 0 hits in `ui/src/`) —
   a11y-plan §11's landmark-less-window ban simply has no automated enforcement in this arm, which is the
   honest way to state it. The `:384`/`:397` half of this premise remains open (bullet 5).

3. `[premise-corrected: replacement, not an eighth crossing — the registry counts spawn FORMS and this
   changes one launcher's internal mechanism at one call site]` **The launcher REPLACES the registered
   seventh governed spawn form; the count stays SEVEN.** The predecessor's own amendment ruled that the
   mechanism registered must be the one that SHIPS. Escalation is owed **either way** under security-plan
   §Security Anti-Patterns → Code Patterns rule (b), and the admitting argument does genuinely change (the
   seventh is admitted `-File`-only and never `-Command`; the successor is admitted as a fixed
   `lpApplicationName` with a guarded composed `lpCommandLine`), so the difference is registry TEXT, not
   whether to escalate. Carried to the P5 review as a confirm-class item.

4. **The API path is reachable from PowerShell with no new dependency** — VERIFIED for two of the three
   APIs by a scratchpad probe at HEAD: `CreateRestrictedToken` and
   `SetTokenInformation(TokenIntegrityLevel)` load through `Add-Type` P/Invoke and succeed. Stated limits,
   which the plan must not overclaim: the probing shell was already Medium, so the High→Medium LOWERING
   direction is unproven; `CreateProcessAsUser` was not exercised (it requires a launch); and nothing
   about the runner. This closes "does the remedy need a different host" — it does not — and no more.

5. **`:384`/`:397` at the runner's WebView2 152** — **STILL OPEN.** Both fail at 153 on the dev host; their
   152 status rests on the 2026-09-10 record, not a fresh run. Split out of the old bullet 2 because the
   landmark half is now settled and this half is not. Rides the same ci-probe if a session appears.
