# design extract

## Relevance
partial. The chunk is a diagnosis-and-regrade of NVDA focus announcements, with no new visual surface. Design binds only if the remedy lands in the webview's accessibility-tree exposure or focus path (scope "What this chunk builds" item 2). It also binds through the shipped strings and states the SR rows grade against.

## Constraints
- design-system §Depth Strategy (Values, "Drop shadows: none") requires the keyboard focus ring to be a `2px solid var(--color-focus)` `outline` under `:focus-visible`, with no `box-shadow`. Any webview-side remedy that touches focus handling (for example re-dispatching or re-targeting focus so NVDA hears a focus event) must leave that ring form intact. Whether the current bundle's ring already matches is research's question. The prior chunk's routine arm asserts it.
- design-system §Motion ("This project's values", Micro-interactions) requires the focus ring to appear with NO transition. A remedy must not add a focus fade, a delayed or animated focus reveal, or any timing-based motion to "wake" the screen reader. All motion stays bound by `var(--motion-micro)` / `var(--ease-quiet)` and is dropped under `prefers-reduced-motion: reduce` (§Surface: desktop-webview → Tokens).
- design-system §Surface: desktop-webview → Component Patterns #3 (Coverage matrix) requires the keyboard-current row to carry `aria-current` plus a 2px `--border-emphasis` left edge on its P-ID cell, distinct from the `--color-focus` ring. It also requires ONE tab stop with roving focus (ArrowUp/ArrowDown, Home/End) and a scroll-into-view for an off-screen row. That is the focus target the silent rows exercise. A fix to focus-event exposure must keep this roving model and must not add tab stops. Scope Boundaries: the `v3-03` claims are not reopened.
- design-system §Color Palette → Semantic Colors (the Verdict/ReportState note) and Component Patterns #4 require every status to be paired with a text label, and announced via `aria-live`, never color alone. Any accessible-name or live-region change made for the SR regrade must keep the text label as the carrier.
- design-system §Surface: desktop-webview → Component Patterns #3, #5 and #6 fix the shipped empty and miss strings as three distinct states: `No coverage data.` (matrix), `No scenarios found.` (empty catalog, SR row E0-01), `No scenarios match.` (picker filter miss, from a persistently-mounted announced region, SR row S0-16), plus `No run yet` (report). Regraded E0/S0 rows compare announced text against these strings. A remedy must not reword them. Whether the regraded rows' expected utterances cite these exact strings is research's question.
- design-system §Surface: desktop-webview → Component Patterns #5 and §Navigation Pattern require the modifier keymap (Ctrl+Enter / Ctrl+. / Escape). Single letters stay free for type-ahead and for the screen reader's browse mode. Any harness-side or webview-side key handling added for the regrade must not claim single-letter keys. That matters because E0-09 is browse-class (scope item 3).
- design-system §Surface: desktop-webview → Component Patterns #1 requires the HOLD flip to be announced via `aria-live="assertive"`. The live-region rows were heard on 2026-09-30 (scope CARRY). A focus-path fix must not regress that announcement channel.

## Patterns to follow
- The focus model: roving single-tab-stop row focus with `aria-current` + `--border-emphasis` edge, separate from the `--color-focus` outline ring (design-system §Surface: desktop-webview → Component Patterns #3; §Depth Strategy Values).
- Announced state changes go through a persistently-mounted region. A region that mounts together with its text announces nothing (design-system §Surface: desktop-webview → Component Patterns #5, the S0-16 measurement). Reuse this if the remedy moves or re-mounts any announcing element.
- Status carries text + glyph + `aria-live` (design-system §Surface: desktop-webview → Component Patterns #4; §Iconography "Rule"). Icon-only controls carry an `aria-label`.
- Tokens bound by `var(--…)` name on plain `:root` (design-system §Surface: desktop-webview → Tokens). Any CSS a remedy touches cites tokens, never a raw hex, px or ms value.

## Anti-patterns to avoid
- No motion-based workaround (a focus fade, pulse, blink or delayed transition) to provoke a screen-reader event. Pulse/blink/glow and animation libraries are banned at expression 0.3 (design-system §Motion "Hard limits").
- No `box-shadow` or `backdrop-filter` focus indicator substituted for the outline ring (design-system §Anti-Patterns → Per-Surface Bans, desktop-webview; §Depth Strategy).
- No `alert()`/`confirm()`/`prompt()` or native OS toast as an "announcement" shortcut (design-system §Anti-Patterns → Per-Surface Bans, desktop-webview; §Surface: desktop-webview → Platform-Specific Notes).

## Contract bindings
- design ↔ a11y: `--color-focus` ring (§Border Progression "Focus"; §Depth Strategy) binds to a11y SC 2.4.7 focus-visible, which the routine arm asserts via computed `outlineStyle`/`outlineColor`. Status text + glyph (§Semantic Colors note; Component Patterns #4) binds to SC 1.4.1 not-color-alone. The no-transition focus ring and reduced-motion drop (§Motion) bind to SC 2.3.3.
- design ↔ tests (SR leg): the shipped empty and miss strings and the `aria-current` row (Component Patterns #3/#5/#6) are what the `sr*` rows' expected utterances and the DOM-half `expectActiveCoverageRow()` check read. The design plan owns the strings and the state treatment; the test plan owns the grading.

## Acceptance criteria contributions
- If the remedy changes any webview CSS or markup: the focus ring is still `2px solid var(--color-focus)` `outline` under `:focus-visible`, with no `box-shadow` and no transition on it. The routine a11y arm's ring assertion stays green (per design-system §Depth Strategy; §Motion).
- If the remedy changes any webview code: the coverage matrix is still one tab stop, and its keyboard-current row still carries `aria-current` + the `--border-emphasis` P-ID-cell edge (per design-system §Surface: desktop-webview → Component Patterns #3).
- The shipped state strings `No coverage data.`, `No scenarios found.`, `No scenarios match.` and `No run yet` are byte-unchanged over the chunk base (`git diff 2c9b37d` on the rendering source shows no edit to them) (per design-system §Surface: desktop-webview → Component Patterns #3/#5/#6).
- Any CSS a remedy adds or edits uses `var(--…)` tokens only, with no raw hex, px or ms literal (per design-system §Self-Validation Protocol → 4. Token Test; §Surface: desktop-webview → Tokens).
