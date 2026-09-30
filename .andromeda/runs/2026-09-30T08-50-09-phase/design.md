# design extract

## Relevance
partial — design-system owns the tokens and component states the four claims are asserted against (focus ring, selection marking, keyboard-first shortcuts); the claim ownership, WCAG criteria and suite placement are a11y/tests territory.

## Constraints
- design-system §Border Progression (Focus row) and §Surface: desktop-webview → Tokens require the focus ring to be `--color-focus`, which is theme-dependent (the dark default block and the `prefers-color-scheme: light` block carry different values). An SC 2.4.7 assertion therefore has to compare against the token as resolved in the active theme, never a single hard-coded hex.
- design-system §Depth Strategy (Values, last bullet) permits `box-shadow` only as a single `0 0 0 2px var(--color-focus)` focus ring on keyboard focus. The chunk scope reports the shipped controls using `:focus-visible { outline: 2px solid var(--color-focus) }` instead. Whether the code uses outline, box-shadow or both is research's question. The ring assertion should key on the ring being present and on its colour being `--color-focus`, not on which CSS property draws it, unless P4 decides to align code and spec.
- design-system §Border Progression (Emphasis row) and §Component Patterns #3 (States) require a selected coverage-matrix row to be marked by a `--border-emphasis` left edge, and require hover to use `--color-raised-1`. The selection marking must stay distinct from the `--color-focus` ring, so focus and selection read as two separate states. Whether any row-selection state exists in the matrix today is research's question (the scope's survey found `--border-emphasis` only on the picker).
- design-system §Navigation Pattern and §Component Patterns #5 require keybindings for start/stop/proceed/abort to be first-class, keyboard-first, mirroring k9s/lazygit and exposed via Radix primitives. Whether any shortcut handler exists is research's question (the scope's survey found none).
- design-system §Platform-Specific Notes and §Per-Surface Bans (desktop-webview) require OS shortcuts (Ctrl+W / Alt+F4 / Cmd+Q) to keep working. Any new shortcut binding must not capture or swallow them.
- design-system §Motion ("This project's values", and the Motion tokens table) sets focus-ring fade-in to `--motion-micro` with `--ease-quiet`. §Motion (Hard limits) and the §Surface: desktop-webview reduced-motion block require every transition to be dropped under `prefers-reduced-motion: reduce`.
- design-system §Component Patterns #3 states "Virtual-scroll for the full wall". The scope reports that no virtualization ships. That clause is a second spec site behind a11y §5's "off-screen virtual-scrolled rows". If P4 takes the re-state fork, this master is an amendment candidate for the wrap's flow. Whether the claim holds is research's question.

## Patterns to follow
- Picker selection marking per design-system §Component Patterns #5 and §Border Progression (Emphasis: "active picker item"). It is the one site the scope reports with `aria-selected`/`aria-current` + `--border-emphasis`. Row selection in the coverage matrix (and in the run report, if built) should reuse that token pairing rather than mint a new one.
- Focus-ring treatment per design-system §Component Patterns #2 (dialog: focus trapped, visible `--color-focus` ring) and #5 (inputs: `--color-focus` ring on focus). This is the same token on every interactive control. The SC 2.4.7 assertion can therefore be one shape applied across controls.
- Disabled-state rendering per design-system §Component Patterns #5 (`--text-muted`, no colour-only signal). This applies if a shortcut targets a control that is disabled in the current run state.
- Keyboard-first row interaction per design-system §Component Patterns #7 (Space toggles a checklist row). This is the existing in-spec precedent for key-driven row behaviour.

## Anti-patterns to avoid
- Hard-coded hex or pixel values for the ring or the selection edge in either code or test expectations. design-system §Self-Validation Protocol (4. Token Test) and §Surface: desktop-webview → Tokens bind by `var(--…)` name.
- Decorative or pulsing emphasis on the focused or selected row (glow, blink, animated highlight). design-system §Motion (Hard limits) bans pulse/blink/glow, and §Anti-Patterns Universal Bans bans colour purely for decoration.
- Hover-only row affordances without a keyboard equivalent, per design-system §Per-Surface Bans (desktop-webview).

## Contract bindings
- Token contrast ↔ a11y: `--color-focus` against its adjacent surfaces binds to a11y's non-text contrast row. The scope reports that the routine arm's existing ring-adjacent row proves the ratio, not that the ring renders. The same holds for `--border-emphasis` as a selection indicator, per design-system §Border Progression.
- Motion ↔ a11y SC 2.3.3 ↔ tests: the 150 ms focus-ring fade-in (design-system §Motion) means a computed-style probe taken immediately after focus can read a mid-transition colour. The harness must either run under reduced motion, where the §Surface: desktop-webview block drops transitions, or settle past `--motion-micro` before reading.
- Selection state + label ↔ a11y SC 1.4.1: the `--border-emphasis` edge must be paired with `aria-selected`/`aria-current`, never colour or border alone, per design-system §Iconography (Rule) and §Anti-Patterns Universal Bans.

## Acceptance criteria contributions
- The ring on the active element resolves to the active theme's `--color-focus` value, read from the token, not a literal. (per design-system §Border Progression; §Surface: desktop-webview → Tokens)
- A selected coverage-matrix row, if selection ships, carries a `--border-emphasis` left edge distinct from the `--color-focus` ring, paired with `aria-selected`/`aria-current`. (per design-system §Component Patterns #3; §Border Progression)
- No new colour, spacing or motion literal is introduced in the chunk's CSS/TSX delta. Every value traces to a `:root` token. (per design-system §Self-Validation Protocol, 4. Token Test)
- Any added shortcut binding leaves the OS shortcuts (Ctrl+W / Alt+F4) unintercepted. (per design-system §Per-Surface Bans, desktop-webview)
