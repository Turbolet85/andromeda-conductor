# Codebase Research — 2026-09-01-desktop-a11y-sweep

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 11 · **Live leg run:** 1 (`agent-run.sh run --e2e`, exit 1, reproduced)

The chunk's single blocking unknown — the identity of the one genuine axe violation — was not
answerable from any artifact (the predecessor recorded it as a COUNT). It was resolved by **running the
leg and decoding the axe payload out of the driver log**, not by inference.

## The measured red (authoritative — this run, 2026-09-01)

`2 passing · 3 failing · exit 1`, matching the CARRY exactly.

| # | Spec | Result | Cause |
|---|---|---|---|
| 1 | zero axe violations on the Minimal-tier baseline | ✖ | 1 genuine violation (below) |
| 2 | Blocked row never rendered red | ✓ | — |
| 3 | token-pair contrast meets WCAG AA | ✓ | asserts only 3 of the 9 required pairs |
| 4 | operator-pause dialog: alertdialog, trap, Escape, focus restore | ✖ | `[role="alertdialog"]` not displayed — **subject absent** |
| 5 | operator-checklist: `role=status` + Space toggle | ✖ | `[role="status"]` not existing — **subject absent** |

### The violation, named
**`color-contrast`** · impact `serious` · tags `cat.color, wcag2aa, wcag143, ACT, EN-9.1.4.3` ·
"Elements must meet minimum color contrast ratio thresholds" · **18 nodes**.

Every one of the 18 nodes carries the **same foreground**: `#717aa0` = **`--text-tertiary`**.

| Nodes | Element | fg | bg | measured | required |
|---|---|---|---|---|---|
| 4 | `th.type-label` (coverage table header strip) | `#717aa0` | `#24273a` (`--color-raised-2`) | **3.50:1** | 4.5:1 |
| 14 | `.cov__cat` / `.cov__unrun` (coverage rows) | `#717aa0` | `#1f2130` (`--color-raised-1`) | **3.78:1** | 4.5:1 |

So the defect is **one token, not eighteen markup sites** — `--text-tertiary` is too dark for the raised
surfaces it is rendered on. Fixing at the token is also what design-system.md mandates (tokens by
`var(--…)` name; never a per-site hex).

## The 18 nodes are a PARTIAL view — the token failure is wider than the leg measured

axe can only score what was rendered. Computing a11y-plan §6's **nine required pairs** from the shipped
`tokens.css` values (WCAG 2.1 relative-luminance math, both themes) finds **three failures, of which axe
saw only part of one**:

| Pair | Required | Dark | Light |
|---|---|---|---|
| `--text-primary` / `--color-base` | 4.5 | 11.18 ✓ | 15.68 ✓ |
| `--text-secondary` / `--color-raised-1` | 4.5 | 7.54 ✓ | 10.98 ✓ |
| **`--text-tertiary` / `--color-base`** | 4.5 | **4.06 ✗** | 5.68 ✓ |
| **`--text-muted` / `--color-inset`** | 4.5 | **2.91 ✗** | **3.86 ✗** |
| `--color-id-cyan` / `--color-base` | 4.5 | 9.96 ✓ | 4.76 ✓ |
| `--count-nominal` / `--color-base` | 3.0 | 11.12 ✓ | 4.66 ✓ |
| `--count-hold` / `--color-base` | 3.0 | 8.78 ✓ | 4.47 ✓ |
| `--status-fail` / `--color-base` | 3.0 | 5.10 ✓ | 4.91 ✓ |
| `--color-focus` / `--color-base` | 3.0 | 9.96 ✓ | 4.76 ✓ |

Two consequences for the plan:
- **`--text-tertiary` also fails on `--color-base` (4.06:1)** — latent, because the empty-state prose that
  uses it (`App.tsx` ×6: "Loading scenarios…", "No coverage data.", "No run yet") was not in a failing
  state during the scan. Fixing only the two raised backgrounds would leave a violation that appears the
  first time the app renders an empty state.
- **`--text-muted` / `--color-inset` fails in BOTH themes** and axe never saw it (no disabled control or
  placeholder was rendered). It is a required §6 pair regardless.

**Feasibility (constraint, not a prescribed value).** `--text-tertiary` must clear 4.5:1 against its worst
background, `--color-raised-2` `#24273A`. A uniform lightening of ~1.155× reaches it (`#838DB9` →
4.54 / 4.91 / 5.27 across raised-2 / raised-1 / base), so the constraint is satisfiable inside the existing
hue without introducing a new colour. The light theme's `--text-tertiary` already passes and must NOT be
lightened — the two themes move in **opposite directions**, and `--text-muted` needs a lift in dark and a
darkening in light.

**Collision note:** dark `--text-muted` (`#565F89`) is byte-identical to `--count-blocked`. layout-templates
§Primary content block 1 records that shared hex as a hazard ("a plain dim reads as `Blocked`"), so moving
`--text-muted` off it relieves a documented hazard rather than creating one — but it is a design-system
token decision, not a silent edit.

## Why specs 4 and 5 are subject-absent, not defects

Read at source, both subjects exist and are correctly built:
- `OperatorPauseDialog.tsx:33-62` — Radix `@radix-ui/react-alert-dialog`, `AlertDialog.Content` (which
  renders `role="alertdialog"` natively), checklist rendered as a **sibling of `AlertDialog.Description`**
  exactly as layout-templates mandates.
- `OperatorChecklistView.tsx:18` — the unticked-count footer already carries `role="status"`
  `aria-live="polite"`. **The spec's selector is correct; nothing is missing.**

They fail only because the dialog is never open on an idle console.

### What it takes to make the subject present (the gating mechanism, read at source)
`conductor-run/src/lib.rs:533-543` fires the hold **only** when all three hold:
1. preflight is **ready** (a Blocked gate short-circuits to a `RunRecord::blocked` at `:517-525`),
2. `route_read_back` returns `Graded` or `AutoResolved` — not `Blocked`,
3. `scenario.expected.is_empty()` — a **declare-only** scenario.

Checklist ROWS additionally require the scenario to declare `[[checklist]]`. Exactly two committed
scenarios do:

| Scenario | P-ID | expected | checklist | phases |
|---|---|---|---|---|
| `halo-hue-encoding` | P-025 | 0 | 1 | 2 × `gap_ms = 3000` (~6s — the cheaper driving target) |
| `halo-breathing-encoding` | P-026 | 0 | 1 | 2 × `gap_ms = 5000` |

So a real HOLD needs a **live, preflight-ready Pulse** — which is why this leg is operator/local, never CI.

## Graph impact (code-graph, `ts` plane)
- Query recorded at `.andromeda/runs/2026-09-01T20-11-05Z-phase/tree-query-2026-09-01-desktop-a11y-sweep.json`.
- `OperatorPauseDialog` resolves at `crates/conductor-tauri/ui/src/components/OperatorPauseDialog.tsx`
  with its props typeLiteral (`open`/`onOpenChange`/`checklist`/`onProceed`/`onAbort`/`allowNoGo`) —
  the component is prop-driven and presentational, so a driven assertion needs the app to open it, not a
  new component API.
- The chunk changes no exported symbol: the token fix is CSS-value-only and the spec work is test-side, so
  there is **no caller set to thread**.

## Patterns detected
- **Token-only styling** (`CoverageMatrix.css:39,72,88`): every failing node reaches `--text-tertiary`
  through `var(--text-tertiary)`, never a literal — so a single token edit repairs all 18 nodes at once.
- **Lamp label+glyph pairing** (`StatusLamp.tsx:13-18` + `lamp.ts:13-18`): all six states carry
  `label` + `glyph` (`Pass ●` / `Fail ●` / `HOLD ●` / `Manual ☐` / `Residual ◌` / `Blocked ○`), with the
  glyph `aria-hidden` and the label in text — the not-color-alone assertion has a real DOM target.
- **Reduced-motion already enforced globally** (`tokens.css:80-85`): `@media (prefers-reduced-motion: reduce) { * { animation: none !important; transition: none !important; } }`.
- **Skip-with-named-precondition** (`wdio.conf.ts:57-66` `reportSkip()` + `onPrepare` `process.exit(0)`):
  the existing shape the spec-level context-skip should mirror rather than invent a second convention.
- **No `data-testid` anywhere in `ui/src`** (0 hits) — a11y-plan §3's focus-identity pattern uses
  `data-testid`, but none exist, so focus assertions must key on role + accessible name/text (which is
  also what test-plan §6 requires).

## Conventions to follow
- **Selectors**: role / text / `aria-live` only; no xpath, no hashed classes (`accessibility.e2e.ts`
  already complies — `[role="alertdialog"]`, `[role="status"]`, `input[type="checkbox"]`).
- **Wait on a signal, never sleep**: `wdio.conf.ts:104-124` polls a real readiness condition and swallows
  the throwing `Page/Frame is not ready` window — the established pattern for any new wait.
- **Classic WebDriver is enforced** (`wdio.conf.ts:78`) — `wdio:enforceWebDriverClassic: true`.
- Token values live in `tokens.css` mirrored from design-system.md §Tokens; names are the binding
  contract and must not be renamed.

## Files to modify
- `crates/conductor-tauri/ui/src/styles/tokens.css` — the failing token values, both `:root` (dark) and the
  `prefers-color-scheme: light` block. **No token renames, no new colours.**
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — the context-skip shape for the two
  subject-absent specs; extend the contrast spec from 3 pairs to the §6 table of nine; add the
  not-color-alone and reduced-motion assertions; **replace the stale header comment** (it still says
  "never on the Windows dev host", measured false by the predecessor — the same class of stale source
  claim as the CARRY's fifth site, which no detector greps).
- `crates/conductor-tauri/ui/wdio.conf.ts` — only if the driven arm needs a second suite selector.

## New files to create
- (decided at P4) a driven-arm spec under `crates/conductor-tauri/ui/test/a11y/` for the live-Pulse HOLD
  leg, if the plan separates the two arms by file rather than by in-spec guard.

## Scope premise closure
Every `[inferred]` bullet in `scope.md` was re-checked; `scope.md` is amended accordingly.
- **VERIFIED** — v2-23 / v2-24 / v2-30 are separate matrix entries (read from `verification-matrix.json`);
  the harness transport is intact and reusable; v2-22 is an affordance cap and tauri-driver supplies real
  keypresses; the spec header carries a stale platform claim (read at source).
- **FALSIFIED / corrected** — "the specific axe rule was never captured" is now **closed by measurement**:
  it is `color-contrast`, 18 nodes, one token. The scope's framing of the fix as possibly "markup/ARIA or
  a token" is resolved: it is a **token value defect**, and its true extent is **wider than the leg
  measured** (a latent third background plus a second failing token neither theme satisfies).
- **VERIFIED, no action** — `tauri` resolves to **2.11.3** (≥ 2.10.3, security's open question: no bump
  needed); `crates/conductor-tauri/ui/logs/` is git-ignored (`.gitignore:23`, arch's open question);
  `agent-run.ps1:102-108` matches `.sh` for `--e2e` at parity.

## Open questions
- **Does the sweep drive the live-Pulse HOLD arm, or defer it?** → blocks: **plan-decision**. The routine
  arm can reach green without it, but v2-22's keyboard clauses (trap / Escape / focus restore / Space
  toggle) are provable only with the subject present, and a skip is not a pass.
- **Does the contrast assertion cover both themes?** → blocks: **plan-decision**. The colorjs.io
  token-name checker resolves whatever `:root` the active OS theme produces, so one run proves one theme;
  `--text-muted` fails in BOTH, and dark/light need opposite corrections.
- **Does `browser.emulate('prefers-reduced-motion','reduce')` work under `enforceWebDriverClassic` on
  WebView2?** → blocks: **implementation-scope**. a11y-plan §6 labels it *recorded-not-established*; the
  documented fallback is an OS-level preference or a `CONDUCTOR_TEST_REDUCED_MOTION` hook.
