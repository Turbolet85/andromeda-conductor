# design extract

## Relevance
Partial — the chunk ships no UI, token or component change, but the property `:384` asserts (every idle-console control reachable by Tab alone) and the accessible names the cycle collects are both design mandates, so design bounds what a "correct counting basis" may assume about the control set.

## Constraints
- Keyboard-first is a standing mandate, not a test artifact: design-system §Anti-Patterns → Per-Surface Bans → desktop-webview bans hover-only interactions without keyboard alternatives, and §Navigation Pattern requires keyboard-first run control (pick → start → holds → report) with no browser-style nav. A corrected `:384` must still go red if any idle-console control loses its keyboard path — a basis that can no longer detect that has stopped asserting.
- The accessible names `tabCycleNames()` collects exist because design-system §Iconography requires every icon-only titlebar/lamp control to carry an `aria-label`. Whether every focusable in the shipped idle console actually yields a non-empty name (vs. an empty string that silently collapses two entries in a set-based basis) is research's question.
- The idle console's control inventory is design-specified, not arbitrary: frameless titlebar min/close Lucide controls at 16px (§Component Patterns 1) plus the scenario/suite picker and start/stop buttons (§Component Patterns 5). Any "focusable set" the corrected expectation computes must equal that mandated set; whether the shipped idle console yields exactly those six is research's question.
- §Component Patterns 5 requires disabled controls to render as `--text-muted` (a real disabled attribute, no color-only signal), and the scope's own `querySelectorAll` predicate excludes `button:not([disabled])` — so the focusable denominator is state-dependent. Whether any idle-console control is disabled at assertion time (start vs. stop) is research's question.
- One surface, one focus cycle: §Navigation Pattern mandates a single-station console with no router and no breakpoints, so wrapping back to `BODY` is the expected terminal state of the *only* cycle. How many times it wraps is therefore an environment property (driver/runtime), never a design one — which is exactly why a visit tally is the wrong basis.
- §Component Patterns 2 requires the operator-pause `AlertDialog` to trap focus. The idle-console cycle is only the console's cycle while no dialog is mounted; a basis measured with a dialog open would measure the trap, not the console.
- If any production UI/CSS file is touched (the scope says none should be), §Surface: desktop-webview → Tokens is the binding contract — bind `var(--…)` by name from the `:root` token block, never a raw hex or ms, and the focus ring stays `0 0 0 2px var(--color-focus)` per §Depth Strategy.

## Patterns to follow
- §Component Patterns 1 (frameless titlebar): `aria-label`-ed 16px min/close controls with a 150ms hover lift — the titlebar contributes named, keyboard-reachable focusables to the cycle; treat their names as the stable identity in any set-based basis.
- §Component Patterns 5 (picker + start/stop): keyboard-first controls with `--color-focus` rings on focus; first-class keybindings mirroring k9s/lazygit. This is the pattern that makes "reached by Tab alone" the right property to assert rather than a pointer-driven one.
- §Surface: desktop-webview → Tokens ships a global `@media (prefers-reduced-motion: reduce) { * { animation: none !important; transition: none !important; } }`, and §Motion requires entrance animations to be *none* at expression `0.3` — so focus order is specified to be time-independent; a correct basis should not need to wait on any transition.
- §Verdict/report-state lamp (§Component Patterns 4) is required to be motionless and always paired with text — nothing in the idle console is specified to enter or leave the focus cycle over time.

## Anti-patterns to avoid
- Never restore green by removing a keyboard path, adding `tabindex="-1"`, or making a control pointer-only — §Anti-Patterns → Per-Surface Bans (desktop-webview) bans hover-only interaction without a keyboard alternative.
- Never add or remove a control (or an icon-only affordance) to make the count line up — §Iconography's "icons clarify, not decorate" and §Component Patterns 1/5 define the mandated control set; the assertion follows the design, not the reverse.
- Never land a new hex, ms literal or palette row on this chunk — §Anti-Patterns → Universal Bans plus the 34-token `:root` contract; the amendment history shows every prior a11y-arm fix reused existing tokens rather than adding one.

## Contract bindings
- design §Iconography (`aria-label` on icon-only controls) ↔ a11y SC 4.1.2 / SC 2.4.3 — the names feeding both `:384` and the wrap-insensitive `:397`.
- design §Motion hard limits (reduce-motion override mandatory) ↔ a11y SC 2.3.3 — if the runner runs under reduce-motion, the design contract says the reached set must be identical either way.
- design §Color Palette → Border Progression (Focus = `--color-focus` `#7DCFFF` / `#0969DA`) ↔ a11y §Contrast (SC 1.4.11 / 2.4.7) — relevant only if the corrected assertion grows a focus-visibility leg.
- design ↔ tests/CI harness: the arm's printed verdict and `$A11yExpectedSkips = 2` in `Assert-A11yVerdict` are the tests domain's contract; design contributes only the expected control-set composition, not the pass/skip tallies.

## Acceptance criteria contributions
- The corrected assertion still fails if any idle-console control is reachable only by pointer (per design-system §Anti-Patterns → Per-Surface Bans → desktop-webview).
- Every entry in the reached set carries a non-empty accessible name, so no two icon-only titlebar controls collapse into one under a set-based basis (per design-system §Iconography).
- No token, hex, ms literal or palette row is added or changed by this chunk; the `:root` token contract is untouched (per design-system §Surface: desktop-webview → Tokens).
- The reached set is invariant under `prefers-reduced-motion: reduce` — focus order does not depend on any transition or entrance animation completing (per design-system §Motion → Hard limits).

## Relevant amendment history
- **2026-09-01-desktop-a11y-sweep** (§Color Palette → Text Hierarchy; §Tokens both theme blocks) — the one prior amendment in this arm's area: an axe `color-contrast` failure on the desktop a11y arm moved `--text-tertiary` and `--text-muted`. Directly relevant as method, not content: the amendment records that the 18-node violation list was "a floor, not the scope" — the fix was solved against a11y-plan §6's nine pairs, in opposite directions per theme. Same discipline this chunk needs: the failing message is a floor, and the bracket-list identity is not the basis.
- **2026-09-02-screen-reader-manual-spec** and **2026-09-04-sr-findings-remediation** (§Component Patterns 3/5/6/7) — prior corrections where a11y/SR-arm expectations were corrected to what the code soundly produces (shipped empty-state strings, the checklist's single real mount), with one correction escalated to the operator because its basis lay outside the report's Changes. Precedent for correcting an expectation's basis, and for escalating when the basis rests on a read outside the chunk.
- **2026-06-15-design-token-typography-bundle**, **2026-06-26-component-primitives-library** — the "spec-illustration → sound-impl reconciliation" routine (playbook `:28`): where a spec's illustrative value is unrealizable as written, the value moves and the invariant holds. This is the governing precedent if the corrected counting basis changes what the assertion literally compares while preserving the SC 2.1.1 property.
