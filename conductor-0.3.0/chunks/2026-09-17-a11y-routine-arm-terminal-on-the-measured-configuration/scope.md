# Scope — The a11y routine arm's terminal, on the measured configuration

**Marker:** `2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration`
**Version:** conductor-0.3.0 · **Epoch 3** — The a11y capability's terminal
**Working entry:** `conductor-0.3.0/working-route.md:37` (minted at the 2026-09-17 wrap's P5)
**Capability owned:** `v3-02` — returned to the pool by the predecessor for this entry.

---

## The chunk in one line

`:384`'s counting basis corrected so SC 2.1.1 measures the reachability property rather than one
environment's focus-cycle wrap, and the arm's CI verdict read green.

## What this chunk builds

1. **A counting-basis correction at the SC 2.1.1 assertion** — `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts:384`.
   The assertion must compare the property it claims to assert (every focusable control is reached by Tab
   alone) rather than a visit tally that varies with how many times the focus cycle wraps before returning
   to `BODY`.
2. **A green CI verdict for the routine a11y arm** — the arm's own printed verdict, with passing and
   skipped counts matching the expected set, read from a CI run of the `a11y` job on the measured
   configuration.

## Boundaries — what this chunk is NOT

- **Not an accessibility fix.** The reachability property SC 2.1.1 asserts is satisfied on the runner;
  the proof is inside the failure message. Nothing in the app's focus behaviour is in scope.
- **Not a re-opening of the exclusion arm.** See the CONTEXT fold below — it is dead on measurement.
- **Not a resolution of the four carried operator escalations.** They are folded below because they ride
  this entry, but none can be actioned without the operator; this chunk surfaces them, it does not decide them.
- ~~**Not a change to the two wrap-insensitive assertions**~~ — **`[premise-corrected: FALSIFIED by a
  measured local run, 2026-09-17. `:397` is NOT wrap-insensitive and shares `:384`'s root cause.]`** The
  dev-host baseline leg (runtime 153, harness capture `runs/a11y-e2e.log`) failed BOTH specs, and `:397`
  failed as `Start then DIV` — because `tabCycleNames()`'s phase 1 Tabs up to 20 times looking for a `BODY`
  stop that **does not exist in this webview's Tab cycle**, exhausts its cap, and leaves focus at an
  ARBITRARY point; phase 2 then starts there. `:397` passing on runtime 131 was configuration LUCK (the walk
  happened to stop one before `Minimize window`), not insensitivity. `:387`'s `missing` check does use set
  membership and is genuinely unaffected.

  **Consequence:** fixing `:384` alone would leave `:397` latently non-deterministic on a configuration whose
  green this chunk is about to claim — which test-plan §10's zero-flakiness budget and a11y-plan §11's
  "NO assertion in this harness is platform-dependent" both forbid. Repairing the shared root cause in
  `tabCycleNames()` is therefore REQUIRED for an honest green, not a scope widening of convenience. Flagged
  for the P5 review as a measured expansion of the touched surface (one helper, both call sites).
- **No Rust surface, no dependency delta expected** — the predecessor's own work touched none.

---

## Measured this phase (coordinate re-verification, per promotion.md's fold-as-hypotheses rule)

Each coordinate the working entry names was re-verified against the artifact before it shaped this scope:

- **`accessibility.e2e.ts:373-384` resolves as stated.** `:373` opens `it('every idle-console control is
  keyboard-reachable by Tab alone (SC 2.1.1)')`; `:384` is the `expect(...)` whose ACTUAL side interpolates
  `names.length` and whose EXPECTED side interpolates `focusableCount` (a `document.querySelectorAll` count
  over `a[href],button:not([disabled]),input:not([disabled]),select,textarea,[tabindex]:not([tabindex="-1"])`).
  So the entry's reading is correct: the expectation counts DISTINCT DOM-focusable controls, the label counts
  Tab VISITS.
- **`:397` resolves as stated** — the SC 2.4.3 `expect` inside the `it(...)` opened at `:393`.
- **`v3-02`'s acceptance is unchanged and not refined downward**, confirmed by reading the ledger entry.

**One correction to the entry's own reasoning, measured at HEAD rather than assumed:** the entry cites
"expected and received bracket lists are identical to the character" as evidence that the same six controls
were reached. That identity is **true by construction** — both sides of the `.toBe()` interpolate the *same*
`names.join(' | ')` expression, so the bracket lists could not differ whatever the run did. It makes the
failure message legible; it is **not** independent evidence that the reached set equals the focusable set.
The claim that the reachability property holds therefore still needs a basis, and supplying one is part of
this chunk's work rather than something the predecessor already established. *(This does not dispute the
conclusion — it names what has and has not been proven.)*

---

## Folded annotations (verbatim from the working entry, per promotion.md §Atomic order step 2)

### PREMISE

- `[premise-corrected: the measured-green configuration is NOT what HEAD ships — `ci.yml:250` is
  `runs-on: windows-2025`, `:282` gates on the Evergreen `152+` floor, and `:431` still launches the
  asserting step through `a11y-limited-token-launch.ps1` at MEDIUM integrity. All three differ from the
  configuration that produced 11/1/2.]* **PREMISE (measured, not assumed):** the arm RUNS and is one
  assertion from green. Run 35192876641 — hosted `windows-2022`, native WebView2 runtime 131.0.2903.86,
  msedgedriver pinned to 131.0.2903.86, HIGH integrity, no launcher — created the first hosted-runner
  WebView2 session in this project's history (`DevToolsActivePort` in 1 s, banner
  `[webview2 131.0.2903.86 windows]`) and ran the routine arm at 11 passing / 1 failing / 2 skipped, the two
  skips being the expected live-hold set. SC 2.4.3 (`:397`) PASSES on 131.

  **What survives and what does not.** The run happened and its facts stand — they are recorded at HEAD in
  `ci.yml:243-249` and in `v3-02`'s ledger note, and nothing here disputes them. What is FALSE as written is
  the implied reading that this chunk is one assertion away from a green CI verdict: that holds only on the
  probe configuration, which the predecessor deliberately did NOT ship. **On HEAD's configuration the arm
  does not reach its assertions at all** — so `:384` is one assertion from green on a configuration that
  exists only in three deleted diffs. Closing the CI half therefore REQUIRES operator rulings 1 and 3
  (label + floor, launcher); it is not reachable by any edit this chunk may make unilaterally.

### SUBJECT

- **VERIFIED (tag dropped) — SUBJECT:** the single red `:384` (SC 2.1.1) is a COUNTING-BASIS defect in the
  assertion, not an a11y defect — expected and received bracket lists are identical to the character (same six
  controls, same order, same one wrap) and only the prefix differs: expected `6 reached`, received
  `12 reached`. The expectation counts DISTINCT controls, the label counts VISITS. Record it as
  "12 visits / 6 distinct", never as a bare count; the reachability property SC 2.1.1 asserts is satisfied and
  the proof is inside the failure message. `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts:373-384`.
  *(Coordinates verified. The "identical to the character" clause stays qualified by the by-construction
  correction above. The six focusables are now ENUMERATED — see the next bullet.)*
- **`[premise-corrected: there are six distinct focusable ELEMENTS but only FIVE distinct accessible NAMES —
  two of the six are unnamed `<div tabIndex={0}>` scroll regions that both resolve to the string `DIV`.]`**
  The entry's "6 distinct" is right about elements and wrong about anything name-keyed. Research enumerated
  the seeded idle console's focusable set against
  `a[href],button:not([disabled]),input:not([disabled]),select,textarea,[tabindex]:not([tabindex="-1"])`:
  (1) `Minimize window` · (2) `Close window` (`Titlebar.tsx:51,59`, both `aria-label`ed) · (3) the picker's
  cmdk filter `<input>` (named by placeholder) · (4) `Start` (`RunControls.tsx:21` — `aria-disabled`, so it
  stays focusable) · (5) `cov__scroll` (`CoverageMatrix.tsx:60`) · (6) `report__scroll`
  (`RunReport.tsx:47`). **`Stop` is excluded** — `RunControls.tsx:34` uses the NATIVE `disabled={!running}`,
  so in an idle console it is neither focusable nor matched by the query. That is exactly six, matching the
  runner's measured `focusableCount = 6`.

  **Consequence for the fix, and it is the load-bearing one.** `activeName()`
  (`accessibility.e2e.ts:165-177`) returns `el.tagName` for any element that is neither `aria-label`ed nor in
  `['BUTTON','A','SELECT','TEXTAREA']` — both scroll regions are bare `<div>`s carrying the name on their
  inner `<table>` instead (deliberate, per the NVDA findings cited in both components). So they BOTH project
  to `DIV`. **The obvious correction — `new Set(names).size` against `focusableCount` — yields 5 vs 6 and
  leaves the assertion RED**, and any repair that reconciles those numbers by name would be masking a
  collision rather than measuring reachability. The basis must key on element IDENTITY (position within the
  live focusable set), never on the projected name; names may still ride into the failure message.
- **`[premise-corrected: still unestablished, but no longer load-bearing.]` Not established (carried from the
  v3-02 ledger note):** why the focus cycle wraps on the runner and did not on the dev host at runtime 152.
  P3 could not settle it without a run, and it is recorded as open. **It stops gating the fix**: an
  identity-keyed basis is invariant under how many times the walk wraps, so the correction is sound whether
  the cycle yields one pass or two. The dev host's older "5 over 5 focusables" baseline is likewise NOT a
  contradiction — it predates the run-report subject whose `report__scroll` is the sixth focusable; the
  denominator GREW, which is itself the argument against ever baking it.

### CONTEXT — the capability

- **CONTEXT:** `v3-02` returns to the pool for this entry — its acceptance is CORRECT as written and was NOT
  refined downward; the world is one small fix from meeting it. The exclusion arm is DEAD on measurement: the
  endpoint demonstrably opens on a hosted runner, so any permanent exclusion would have been ratified on a
  false basis.

### CARRY — four unresolved operator rulings

- **CARRY:** FOUR operator rulings reached no decision at the 2026-09-17 wrap and the probe-scoped code was
  REMOVED rather than shipped unratified —
  1. the `>= 152` Evergreen floor is falsified as a posture (not necessary: coherent 131/131 works; not
     sufficient: coherent 152/152 fails, run 34654076633; and it DESTROYED the working configuration at run
     35185153012 by fetching 153 over a native 131) and the measured subject is driver/runtime major
     COHERENCE;
  2. security-plan's stated float exit condition names a mechanism that does not exist — the Standalone
     Installer is Evergreen and takes no version, only Fixed Version is versioned (>250 MB,
     `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`, latest/second-latest majors only, so 131 is unobtainable from
     Microsoft);
  3. the seventh governed spawn form's disposition if the launcher leaves the asserting step;
  4. a second non-loopback egress (`msedgedriver.microsoft.com`) if the driver pin ships.

  **None can be actioned without the operator.** Each is a `security-plan` / arrangement-row question, and
  three of the four (1, 3, 4) gate what the CI job may contain — so whether this chunk can reach a green CI
  verdict at all depends on which of them the operator rules on. P4 must state this dependency explicitly
  rather than plan around it.

### CARRY — the configuration bound

- **VERIFIED (tag dropped) — CARRY:** integrity's SIGN is configuration-bound — Medium helped at runtime 153
  on the dev host, High is REQUIRED at 131 on windows-2022. The 2026-09-16 legs A/B/C are BOUNDED by this,
  never retired. *(Evidence pointer spot-checked still-true at HEAD: `ci.yml:413-430` records both halves in
  the asserting step's own comment — the Medium-integrity rationale as shipped, then the inversion measured
  at runs 35191569653 / 35192876641, explicitly marked "MEASURED, NOT YET ACTED ON".)*

### CONTEXT — external corroboration

- `[inferred]` **CONTEXT:** external corroboration at actions/runner-images#14738 — the same failure on a
  byte-identical image (`windows-2025-vs2026` 20260828.587) and runtime (152.0.4191.66) for a plain Tauri/wry
  app with no token work, reported passing on windows-2022 at 131.0.2903.86.
  *(An external issue summarised at second hand. Per the 2026-09-17 session learning — a search summary is a
  claim ABOUT a source, never the source — this is NOT to be relied on without fetching the issue, and it is
  corroboration only; nothing in the plan should rest on it.)*

---

## Surfaces and contracts touched

| Surface | Expected involvement |
|---|---|
| `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` | the `:384` assertion; `tabCycleNames()` at `:185` is the value's producer |
| `.github/workflows/ci.yml` | the `a11y` job — only if an operator ruling changes what the job may contain |
| `scripts/agent-run.{sh,ps1}` | `Assert-A11yVerdict` reads the arm's printed verdict + expected skip count (`$A11yExpectedSkips = 2`) — the acceptance's own mechanism, expected unchanged |
| `conductor-0.3.0/verification-matrix.json` | `v3-02` claim at P5 |
| `.andromeda/security-plan.md` | escalations 1–4 are questions ABOUT it; this chunk does not amend it (phase is read-only on specs; wrap owns amendments) |

## Open questions — closed by P3

1. **What is the correct counting basis?** ANSWERED: element IDENTITY within the live focusable set, both
   directions, never a name-keyed set and never a literal. Forced by the `DIV`/`DIV` name collision above.
2. **Why does the cycle wrap on the runner?** STILL OPEN, and deliberately not on this chunk's critical path —
   an identity-keyed basis is wrap-invariant. Carried as a recorded unknown, not a blocker.
3. **Which escalations gate a green CI verdict?** ANSWERED: rulings **1** (the `≥152` floor / the
   `windows-2025` label) and **3** (the launcher in the asserting step). Ruling 4 (driver pin) is optional —
   the measured-green run pinned the driver, but `EDGEWEBDRIVER` from a `windows-2022` image may already be
   coherent. Ruling 2 (the float's exit condition) is a spec-text defect that blocks nothing here.
4. **Does `Assert-A11yVerdict` tolerate 11/1/2 → 12/0/2?** ANSWERED — YES, with no change to either shell.
   It grades three predicates and NONE is a pass tally: the driven-session banner, `Spec Files:` failed `> 0`,
   and skips `-gt $A11yExpectedSkips` (2). `agent-run.ps1:62,68-100` · `agent-run.sh:188,193-226`. The only
   stale thing is a doc COMMENT ("Measured 2026-09-07: 10 passing, 2 pending of 12") which no predicate reads.
