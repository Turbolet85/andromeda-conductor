# Codebase Research — 2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration

## Scope
- **Depth:** deep · **Reads:** 11 · **Globs/Greps:** 8 · **Graph queries:** 2 (ts plane, `db_state` warm)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read as a STRUCTURAL extraction
  (69.8 KB, past the 25 000-token read cap): `grep -n` index over `^#{1,4} ` + `^- 20\d\d-` giving 5 headers
  and 27 dated entries, then offset-bounded reads of the three bearing on this leg (`:62` `--e2e` rebuild
  ownership · `:64` expect-atom discipline · `:66` red-baseline / capture-encoding hazards). It does **not**
  auto-load for this chunk (its `paths:` are `scripts/agent-run.*` · `crates/conductor-cli/**` ·
  `crates/conductor-verify/**` · `crates/**/tests/**`; the modify-set is `ui/test/**`) — read deliberately
  because the plan names a live leg, which is the measured hazard that rule's own reference warns about.
  Also consulted, both auto-loading on `ui/**`: `.claude/rules/a11y.md` (8.6 KB, full) and
  `.claude/rules/frontend.md` (18.2 KB, full). `.claude/rules/testing.md` does NOT match `ui/test/**` and
  that is DELIBERATE, not a gap — its own header scopes it to "Rust test code".

## Files inspected
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (`:150-230`, `:300-428`) — the modify target.
  `activeName()` `:165-177`, `tabCycleNames()` `:185-198`, the SC 2.1.1 spec `:373-391`, SC 2.4.3 `:393-408`.
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` (`:40-66`) — the two `aria-label`ed window buttons;
  the live count is a `<span aria-live="polite">` with no `tabindex`, so it is NOT focusable.
- `crates/conductor-tauri/ui/src/components/RunControls.tsx` (`:15-42`) — **the decisive read.** `Start` uses
  `aria-disabled` (stays focusable, deliberately, so the hold dialog's restore has a target); `Stop` uses the
  **native `disabled={!running}`**, so on an idle console it is neither focusable nor matched by the query.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (`:53-64`) and `RunReport.tsx` (`:44-50`) —
  each renders `<div className="…__scroll" tabIndex={0}>` with **no accessible name of its own**; both carry
  a comment recording that the name sits on the inner `<table>` on purpose (a focusable `role="group"` made
  NVDA read the entire table in one utterance — S0-09 / E0-05).
- `crates/conductor-tauri/ui/src/App.tsx` (`:340-372`) — both scroll regions mount whenever their data is
  non-empty, which the arm's own fixture seed guarantees.
- `.github/workflows/ci.yml` (`:238-300`, `:386-450`, plus a step-name index over the whole file) — the
  `a11y` job's shipped arrangement.
- `scripts/agent-run.ps1` (`:55-105`) and `scripts/agent-run.sh` (`:180-230`) — `Assert-A11yVerdict` /
  `assert_a11y_verdict`, the acceptance's own mechanism.
- `crates/conductor-tauri/ui/wdio.conf.ts` (`:320` specs) — the routine arm loads `accessibility.e2e.ts` alone.

## Graph impact (ts plane, `crate = conductor-ui`)
- **`tabCycleNames`** (`fn`, def `accessibility.e2e.ts` graph line 184 → **editor `:185`**) — **2 callers, both
  in the same file**: graph lines 373 and 393 → editor `:374` (SC 2.1.1) and `:394` (SC 2.4.3). The caller set
  is complete and file-local, so a change to this helper's RETURN SHAPE threads to exactly two sites and
  `:397`'s `names[0] / names[1]` reads are boundary members of this chunk whether or not they are edited.
- **`activeName`** — defined THREE times, once per spec file (`accessibility.e2e.ts:165`,
  `operator-hold.e2e.ts:25`, `screen-reader.e2e.ts:185`; editor lines). They are independent copies, not one
  shared helper: editing one does not move the others, and only the first is in this chunk's arm.
- `probe_hits` non-zero on every name queried, `db_state` warm — the 2-row caller result is a genuine
  measurement, not an index gap.

## Patterns detected
- **Fold the observed context INTO the asserted value** (`accessibility.e2e.ts:384-390`, and `frontend.md`
  2026-09-02): `@wdio/globals`' `expect` has no two-arg message form, so every assertion in this file embeds
  its evidence in the compared string. Any corrected assertion must keep doing this.
- **Short identifying projection, never raw node text** (`activeName` `:160-163` doc comment): the helper
  deliberately returns a bounded projection because a `textContent` fallback on `BODY` once dumped ~8 KB.
- **Both-directions exact-set equality with no literal-count predicate** — the shape test-plan §6 mandates for
  the coverage/scenario-backing gates, and the shape the tests and a11y extracts both independently name for
  this correction.
- **`this.skip()` with a stated reason for a subject-absent spec, never a pass** (`:311-318`, `:412-425`) —
  and `:348-352` records a skip being deliberately REMOVED once the fixture could seed its subject.

## Conventions to follow
- **No baked tally**: test-plan §6 de-literalized `10 passing / 2 skipped` to the SET it evidences; the same
  rule governs any count this chunk writes into code, a comment, or a record.
- **Identity over name** for anything set-keyed here — forced by the `DIV`/`DIV` collision below.
- **A RED baseline validates only the failing direction** (`verification-harness.md:66`, measured
  2026-09-11): a corrected assertion turning green does NOT establish that it counts correctly. The green
  run's own hit list must be hand-checked. Same entry: a PowerShell-redirected capture is UTF-16 and `grep`
  returns a silent zero on it — decode before concluding anything from a captured leg log.
- **App-side changes need an `--e2e` run to reach the bundle** (`verification-harness.md:62`); a harness-side
  change (a spec edit — which is all this chunk touches) takes effect immediately.

## The load-bearing mechanism, stated as the equality the fix needs
**The idle console's focusable set has SIX members and only FIVE distinct accessible names.** Enumerated
against the spec's own selector
(`a[href],button:not([disabled]),input:not([disabled]),select,textarea,[tabindex]:not([tabindex="-1"])`):

| # | element | `activeName()` yields |
|---|---|---|
| 1 | `Titlebar.tsx:51` button | `Minimize window` |
| 2 | `Titlebar.tsx:59` button | `Close window` |
| 3 | picker cmdk `<input>` | its placeholder |
| 4 | `RunControls.tsx:21` `Start` (`aria-disabled`, focusable) | `Start` |
| 5 | `CoverageMatrix.tsx:60` `cov__scroll` | **`DIV`** |
| 6 | `RunReport.tsx:47` `report__scroll` | **`DIV`** |

`Stop` (`RunControls.tsx:34`, native `disabled`) is correctly excluded on both sides. Six matches the
runner's measured `focusableCount = 6` exactly.

**The equality the correction needs, and the one it must NOT use.** `activeName()` (`:165-177`) returns
`el.tagName` for any element that is neither `aria-label`ed nor one of `BUTTON/A/SELECT/TEXTAREA`, so members
5 and 6 both project to the string `DIV`. Therefore:
- `new Set(names).size === focusableCount` yields **5 === 6 → still RED**. The obvious fix is wrong.
- Reconciling those two numbers by NAME would mask a collision rather than measure reachability.
- The sound basis is **element identity**: index each visited element into the live focusable set
  (`Array.from(document.querySelectorAll(SEL)).indexOf(document.activeElement)`) and assert the reached index
  set equals `{0 … n-1}` in both directions. That is invariant under wrap count, survives duplicate names,
  and still fails loudly when a control drops out of the Tab order. Names continue to ride into the message.

**Why the dev host's "5 over 5" baseline is not a contradiction**: it predates the run-report subject whose
`report__scroll` is member 6. The denominator GREW — which is the argument against baking it, not evidence of
a regression.

**ESTABLISHED by a measured local run (2026-09-17, dev host runtime 153, capture `runs/a11y-e2e.log`,
15.2 MB, UTF-8 — encoding probed per `verification-harness.md:66b` before reading).** The red baseline
printed the walk verbatim, and it settles the mechanism:

```
Expected: "6 reached [DIV | Minimize window | Close window | Filter scenarios by name or P-ID… | Start | DIV | DIV | …]"
Received: "20 reached [ …same list… ]"
```

- **`focusableCount` is 6 here too**, and the walk's distinct names are exactly FIVE
  (`DIV`, `Minimize window`, `Close window`, `Filter scenarios by name or P-ID…`, `Start`) — the 6-elements /
  5-names collision is now MEASURED, not inferred.
- **The cycle is six long and contains no `BODY`.** Reading the sequence, positions 1-6 are
  `DIV_report, Minimize, Close, Filter, Start, DIV_cov` and position 7 returns to `DIV_report`. Rotating it
  to the document start gives `Minimize, Close, Filter, Start, cov__scroll, report__scroll` — exactly DOM
  order, which confirms both the enumeration and the render order.
- **`20` is the LOOP CAP (`for i < 20`), not a wrap count.** `tabCycleNames()` phase 1 Tabs up to 20 times
  waiting for a `BODY` stop that never comes, exhausts the cap, and leaves focus at an ARBITRARY point;
  phase 2 then walks 20 more from wherever that was and truncates. The runner's `12` is the same defect
  landing on a different cut.
- **Therefore `:397` (SC 2.4.3) shares the root cause and is NOT wrap-insensitive.** It failed on this run as
  `Start then DIV`. Its pass on runtime 131 was the walk happening to stop one position before
  `Minimize window` — configuration luck, not a property. A fix confined to `:384` would ship a latent flake
  into the very configuration whose green this chunk claims.

**The repair is in the walk, not in either assertion's arithmetic:** stop depending on a `BODY` sentinel.
Detect the cycle by the first REPEATED element identity, then compare against the live focusable set — set
equality for SC 2.1.1, and rotation-aligned order equality for SC 2.4.3. Both become environment-independent
by construction, which is what a11y-plan §11 requires.

## Files to modify
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — the SC 2.1.1 assertion at `:384`, and
  whatever `tabCycleNames` (`:185`) must return to supply element identity. **Caller threading (from the
  graph, not memory):** both call sites — `:374` (SC 2.1.1) and `:394` (SC 2.4.3) — are boundary members; if
  the helper's return shape changes, `:397`'s `names[0] / names[1]` reads move with it.
- *(conditional on operator rulings 1 and 3 — see Open questions)* `.github/workflows/ci.yml` — the `a11y`
  job's `runs-on` label (`:250`), the Evergreen floor gate (`:282`), and the launcher in the asserting step
  (`:431`).
- **No change required** to `scripts/agent-run.{sh,ps1}` — see Open question 4's answer.
- **Companion sweep** — `grep -rn 'tabCycleNames' crates/conductor-tauri/ui --include=*.ts --include=*.tsx`:
  **3 hits · 3 in the modify-set · 0 elsewhere** (the definition `:185` + the two call sites). A second sweep
  on the selector literal `grep -rn 'tabindex="-1"' …`: **2 hits · 1 in the modify-set · 1 no-change** —
  `screen-reader.e2e.ts:207` uses a deliberately LOOSER selector (no `:not([disabled])`) for the
  operator-local SR leg, a different arm with a different basis; out of scope, no change.
- Spec count re-derived: `grep -cE "^  it\(" accessibility.e2e.ts` → **14**, consistent with the run's
  11 passing + 1 failing + 2 skipped.

## Open questions
1. **Do operator rulings 1 and 3 land, and which way?** → blocks: **plan-decision**. HEAD's `a11y` job is
   `runs-on: windows-2025` (`ci.yml:250`), gates on the Evergreen `152+` floor (`:282`), and launches the
   asserting step through `a11y-limited-token-launch.ps1` at Medium integrity (`:431`). The configuration
   that produced 11 passing / 1 failing / 2 skipped was `windows-2022` + a coherent 131/131 driver+runtime +
   HIGH integrity + **no launcher**. All three deltas are recorded at HEAD as measured-but-not-acted-on
   (`:243-249`, `:423-430`) and each explicitly says it needs an operator ruling. **So the `:384` correction
   alone cannot produce a green CI verdict** — on HEAD's configuration the arm does not reach its assertions.
   P4 must put this fork to the operator rather than plan around it.
2. **Why does the cycle wrap twice on the runner?** → blocks: nothing (recorded unknown). Wrap-invariant
   under the chosen basis; worth capturing from the next green run's message rather than chasing now.

### Answered here, recorded so P4 need not re-derive
- **`Assert-A11yVerdict` tolerates 11/1/2 → 12/0/2 with NO change to either shell.** It grades exactly three
  predicates and none is a pass tally: the `^[webview2 …windows` driven-session banner; `Spec Files:` failed
  `> 0` → red; and skips `-gt $A11yExpectedSkips` where `$A11yExpectedSkips = 2` (i.e. "at most 2").
  `agent-run.ps1:62,68-100` · `agent-run.sh:188,193-226`, semantics identical across the pair. The only stale
  artifact is a doc COMMENT at `agent-run.ps1:59` / `agent-run.sh:185` ("Measured 2026-09-07: 10 passing,
  2 pending of 12") that no predicate reads — a de-literalization candidate, not a contract change.
- **The landmark/`contentinfo` second cause of red cannot fire in this arm** — carried forward from `v3-02`'s
  ledger note (measured at the predecessor), re-confirmed structurally here: the routine arm loads
  `accessibility.e2e.ts` alone (`wdio.conf.ts:320`) and that file carries no landmark assertion across its
  14 `it()` blocks.
