# Design System — Amendments

_Append-only changelog of amendments to `design-system.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-design-token-typography-bundle — §Tokens illustrative CSS `@theme` → `:root`
**Section:** §Surface: desktop-webview / Tokens
**Change:** the token block is now declared on plain `:root` (not `@theme`), with `@import "tailwindcss"` retained for the engine; the light-mode override is `@media (prefers-color-scheme: light) { :root { … } }` (not nested `@theme`). Token NAMES + VALUES are unchanged (the 34-token binding contract). Added a Tailwind-v4 NOTE explaining the reason.
**Why:** Tailwind v4 `@theme`/`@theme static` tree-shakes non-namespace tokens (the chunk observed only 23/34 emit — dropping all `--space-*`, three `--radius-*`, `--motion-micro`, `--ease-quiet`) and forbids `@theme` nested inside `@media`. The shipped `tokens.css` declares all 34 on `:root` (verified: 34/34 emit; Vite's minifier preserves author custom props). Report Deviation #2 / D-design-tokens; user chose the `:root` replacement at the wrap escalation. Cascaded to `.claude/rules/frontend.md`. A generalized `playbook.md` rule was added (spec-illustration → sound-impl reconciliation, when the report proves the invariant = routine).

## 2026-06-24-sanitized-stderr-agent-mode-logging — §cli "Error output" documents the error:/hint: token reuse
**Section:** §Surface: cli / Output structure (5. Error output)
**Change:** noted the `error:` label reuses Fail red (ANSI 203) and `hint:` the Residual mute (ANSI 246), tty-gated on a stderr-specific `IsTerminal` gate (distinct from the stdout gate), the ASCII labels always present (never color-alone).
**Why:** D-design-tokens fired on the new `error:`/`hint:` cli surface. The detector proposed adding a "Hint grey" palette ROW — CORRECTED at validation: ANSI 246 is the EXISTING "Residual mute" token, not a new color, so NO palette entry was added; the error edge REUSES two shipped tokens (203/246) and the invariant holds (token-based, never color-alone — report Coverage tokens design-token✓). Applied as a use-site note on the existing "Error output" entry (the spec-illustration → sound-impl reconciliation routine rule), not a duplicate color. Cascade: design-summary.md / rules/frontend.md carry no per-cli-token detail → no-op.

## 2026-06-24-paused-count-hold-point-signature — §Motion gains a Motion-tokens table (registers --motion-heartbeat)
**Section:** §Motion (calibrated to expression level 0.3)
**Change:** Added a Motion-tokens table (mirroring the §Spacing / §Border-Radius token tables) registering `--motion-micro` (150ms), `--motion-heartbeat` (1600ms), `--ease-quiet`; the heartbeat-period row ties the live count breath to the signature (value-ticking on the count arrives with the Epoch-9 live-counter `Channel`). Token NAMES/VALUES otherwise unchanged.
**Why:** The chunk added `--motion-heartbeat: 1600ms` to `tokens.css` (the heartbeat period — the existing `--motion-micro` 150ms is a flicker, and tokens-by-name forbids a raw ms); user-approved at /andromeda-phase P5 (scope val-1 intent-incomplete). §Motion previously documented motion as prose + a duration-scale expression table with no named-token inventory, so the new token was unregistered (D-design-tokens drift). Cascade: design-summary.md §Spacing&motion motion line gains `--motion-heartbeat`; rules/frontend.md carries no motion-token inventory → no-op.

## 2026-06-26-component-primitives-library — operator-pause dialog fade reconciled 200ms → `--motion-micro` (150ms)
**Section:** §Motion (This project's values) + §Component Patterns §2 (Operator-pause go/no-go dialog)
**Change:** The operator-pause dialog fade is `--motion-micro` (150ms), not a literal 200ms — both the §Motion "This project's values" transitions line and the §Component-Patterns §2 dialog entry now read "150ms fade (`--motion-micro`)". The generic expression-scale ceiling is untouched (§Motion "200ms fades at most" + the `0.3-0.4 → 200ms fade` reference row remain — 150ms still satisfies "at most 200ms").
**Why:** D-design-tokens (warning). The component-primitives chunk shipped `OperatorPauseDialog` (Radix AlertDialog) with `animation: … var(--motion-micro)` because no 200ms token exists in `tokens.css` (only `--motion-micro: 150ms`); tokens-by-name forbids a raw 200ms literal, so the spec's aspirational 200ms is unrealizable as written. Routine spec-illustration → sound-impl reconciliation (the `@theme`→`:root` / `--motion-heartbeat` precedents) — the never-color-alone + token-bound invariants hold (report Coverage tokens ✓). Cascade: design-summary.md / rules/{frontend,a11y}.md carry no fade-duration literal → no-op.

## 2026-08-08-sut-capability-manifest — De-hardcoded P-ID range in token-usage examples
**Section:** §Color Palette (Primary) · §Typography (Data row) · §Surface: cli ANSI map · §Brand Identity
**Change:** Mono status-tier / ANSI-117 usage examples name "P-IDs" instead of the fixed `P-001..P-060` range.
**Why:** Illustrative examples carried the superseded range; no token, hex or type-role changed.

## 2026-08-09-current-sut-coverage-classification — De-hardcoded the last two literal-60 prose counts
**Section:** §Brand Identity (Domain anchors) · §Surface: desktop-webview → Component Patterns 3 (Coverage matrix)
**Change:** "the dense single-row-per-P-ID wall of all 60 capabilities" now reads "over the manifest's accepted capability set"; "Virtual-scroll for the full 60-row wall" now reads "for the full wall (one row per manifest capability)". Wording only — no token, hex, or type-role changed.
**Why:** The coverage classification widened from 60 to 82 rows this chunk, so both counts were stale. These are the two prose sites the 2026-08-08 de-hardcoding sweep missed while correcting the token-usage examples — same rationale, same de-hardcode-don't-substitute treatment (name the set, never the new literal). The edits landed at /implement (see this chunk's report §Deviations 1); this entry is the history half that flow owed. Cascade: `.claude/docs/design-summary.md` + `.claude/rules/frontend.md`.

## 2026-08-09-out-of-scope-classification-treatment — ANSI 246 / Residual-mute records its second non-lamp reuse; results-vs-coverage table disambiguated
**Section:** §Surface: cli / Tokens (the ANSI 256 map, Residual-mute entry) · §Surface: cli / Component Patterns #3
**Change:** the Residual-mute → ANSI 246 entry now records the tier's two NON-lamp uses (the `hint:` stderr label and the coverage-matrix out-of-scope Mode cell), names `var(--status-residual)` as the webview half of the same by-name pair, and states that the always-rendered label carries the signal while Markdown uses emphasis as its color-free counterpart. Component Pattern #3 is retitled **Results / SLO table** and now states explicitly that the coverage matrix is a separate 4-column table with no verdict/state column.
**Why:** the chunk tinted the out-of-scope Mode cell with the existing pair (zero new tokens, zero new ANSI entries — report §Coverage of new surfaces), so the map was no longer an accurate account of where the tier is used. The retitle fixes a pre-existing conflation the chunk's research surfaced: the block headed "coverage-matrix / SLO table" enumerated the results table's 6 columns while `conductor coverage` renders 4. Detector D-design-tokens (proposal) + operator-resolved escalation (the column conflation).
