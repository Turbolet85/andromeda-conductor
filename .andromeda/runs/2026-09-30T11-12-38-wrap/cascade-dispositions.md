# Cascade dispositions — 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed

**The search.** `cascade.py sweep` over `cascade-patterns.toml` (ten patterns, every control fired on the pre-pass
masters at `7ee2fead`, the pre-CI parent), covering the retired claims by WORDING and by MECHANISM:
- `nosuite` ("No suite asserts") — the unasserted-claim statement.
- `virtscroll` ([Vv]irtual[- ]scroll) — the virtualization claim.
- `ringfade` / `boxshadow` — the ring's drawing mechanism (fade-in; a `box-shadow` / `0 0 0 2px` ring).
- `k9skeys` (`mirroring k9s`, `k9s / lazygit`, `hint frame`) and `radixkeys` (shortcuts "exposed via Radix
  primitives") — the key register and its claimed mechanism.
- `ariasel` (`aria-selected`), `selectedrow` ("selected row") — the row-marking claim.
- `drivensample` (`~54s`) and `onlysr` ("ONLY `sr-empty` and `sr-error`") — the driven arm's duration and the
  scenarios-dir spawn claim.

Plus, by hand, a TOPIC grep over the leaves (virtual-scroll · ring fade · box-shadow · k9s · lazygit · Radix primitives ·
shortcut · row navigation · "asserted by NO suite" · driven arm · `CONDUCTOR_SCENARIOS_DIR` · `aria-selected` ·
`claim-ownership` · keyboard-first · `~54s`) over CLAUDE.md, `.claude/docs/{a11y,design,tests}-summary.md`,
`commands.md`, `stack.md`, `gotchas.md`, `conventions.md`, `.claude/rules/{a11y,frontend,testing}.md` — because
`a11y.md:24` states the retired claim as "asserted by NO suite", which no swept pattern spells.

Not looked for: the `Spec Files: 1 passed` literal (0 master hits — no control could fire; its homes are the plan
(corrected), `verification-harness.md:58` Session Additions (P3) and `verification-matrix.json#v3-03` (P7.3)).

## Rows
| row | disposition |
|---|---|
| `a11y-plan.md:356` ariasel (new) | no change — this pass's own negation ("so not `aria-selected`"), a true statement |
| `test-plan.md:307` onlysr (standing, edited) | no change — the amended clause now scopes "ONLY" to the SR suites and names the driven suite beside it |
| `a11y-plan.md:424` ringfade (standing, @c1117) | no change — `cascade.py window` shows a "Formerly this read …" provenance sentence (history) |
| `.claude/rules/a11y.md:19` virtscroll (leaf) | re-derived — every row renders, no virtualization |
| `.claude/rules/a11y.md:32` ringfade (leaf) | re-derived — the ring is an untransitioned outline |
| `.claude/docs/design-summary.md:32` virtscroll (leaf) | re-derived — every row rendered behind one roving stop |

Every other pattern: 0 rows with its control fired — a statement about that pattern (the pass's own edits removed each
standing site), not an absence proof.

## Leaves re-derived (step 3)
- `.claude/rules/a11y.md` (body; `## Session Additions` untouched): `:19` landmarks bullet (virtualization) · `:22` the
  keyboard bullet (the modifier map; + the coverage matrix's one roving stop, a new bullet) · `:24` ownership (driven /
  routine owners, `inPlace` enforced, the 11 · 9 (5) · 2 · 0 enumeration — was "Three claims … asserted by NO suite") ·
  `:32` reduced motion (no ring fade) · `:38` Testing (routine + driven arm contents).
- `.claude/docs/a11y-summary.md` `:14` (routine arm contents, driven ownership, `inPlace` checker) · `:19` (table
  semantics, roving stop, `aria-current`, no virtualization).
- `.claude/docs/design-summary.md` `:32` (coverage matrix).
- No re-derivation needed (topic grep, 0 retired claims): CLAUDE.md `GENERATED:setup:*` (no env-var / key-map /
  coverage-matrix claim; arch's amendment is one §Occupied Resources env-var line no CLAUDE.md block restates) ·
  `commands.md:23` (lists `CONDUCTOR_SCENARIOS_DIR` by name only — still true) · `tests-summary.md` · `testing.md` ·
  `frontend.md:40` (generic "keyboard-first; visible focus ring" — still true) · `stack.md` / `gotchas.md` /
  `conventions.md`.
- Curation homes / judgment bases: 0 rows for every pattern. `verification-harness.md:58` (Session Additions) carries
  the `Spec Files: 1 passed, 1 total` literal → P3 curation (in-place correction), never a cascade edit.
