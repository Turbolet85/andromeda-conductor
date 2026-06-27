# design extract

## Relevance
Relevant — implements the operator-pause go/no-go dialog UI per design-system.md §Component Patterns §2.

## Constraints
- Dialog surface fill: `--color-raised-3` (`#2A2D42` dark / `#FFFFFF` light); `1px solid --border-subtle` border; `--radius-lg` (8px) (design-system.md §Component Patterns §2)
- Dialog entrance animation: 150ms fade (`--motion-micro` ease-out); drop entirely under `@media (prefers-reduced-motion: reduce)` (design-system.md §Motion "This project's values" + §2)
- Button semantics: **Proceed** action styled with `--count-nominal` (`#7EE787` dark) accent; **Abort** action styled with `--status-fail` (`#F85149` dark) text; both must pair color with text label, never color alone (design-system.md §Component Patterns §2 + Color Palette "Verdict vs ReportState")
- Focus ring: visible `--color-focus` (`#7DCFFF` dark / `#0969DA` light) inset ring on keyboard focus; 150ms micro-interaction ease-out on focus/hover (design-system.md §Motion "Micro-interactions")
- Typography: IBM Plex Sans for dialog body text; body weight 400, 14px / 1.5 line-height (design-system.md §Typography)
- Dialog must be driven by the titlebar paused-count signature (count freezes at hold value, text tints nominal-green → hold-amber via 150ms color transition, phase line flips to "HOLD — operator pause") — the dialog **complements** that motion-is-the-event cue, never replaces it (design-system.md §Brand Identity "Paused-count hold-point")

## Patterns to follow
- Reuse Radix AlertDialog scaffold from component-primitives-library (already shipped; do not rebuild) with controlled `open`/`onOpenChange` + `title`/`body`/`onProceed`/`onAbort`/`allowNoGo` props (design-system.md §Component Patterns §2 + scope.md CARRY note)
- Dialog opens on hold-signal backend→frontend; prompt from `HoldPoint.prompt` passes into the dialog body (scope.md)
- Proceed/Abort buttons pair color with text — Proceed: `--count-nominal` text + "Proceed" label; Abort: `--status-fail` text + "Abort" label (design-system.md §Iconography rule + Color Palette §Verdict)

## Anti-patterns to avoid
- NEVER animate entrance with framer-motion / spring physics / staggered reveals — CSS fade only at expression 0.3 (design-system.md §Motion "Hard limits")
- NEVER use color alone on buttons — Proceed/Abort must pair functional color with text label (design-system.md §Iconography + a11y binding)
- NEVER animate/hide the titlebar paused-count during the hold — the frozen count at `--count-hold` amber tint is the signature; the dialog is the *interaction surface*, not the status signal (design-system.md §Brand Identity)

## Contract bindings
- Motion fade binds to a11y §Animation SC 2.3.3 (prefers-reduced-motion must drop transition entirely)
- Button color pair (green/red + text) binds to a11y §Use of Color SC 1.4.1 (never color alone for state)
- Focus ring contrast (`--color-focus` + surface) binds to a11y §Contrast SC 1.4.3 (4.5:1 minimum — token pair pre-validated per design-system.md)

## Acceptance criteria contributions
1. (design) Uses only design tokens — colors from `{--color-raised-3, --color-focus, --border-subtle, --count-nominal, --status-fail}`, spacing from `--space-*`, motion from `--motion-micro` (no hardcoded hex / pixel values).
2. (design) Dialog fade (150ms ease-out) respects `@media (prefers-reduced-motion: reduce)` — transition drops to `transition: none`.
3. (design) Proceed button renders `--count-nominal` text + "Proceed" label; Abort renders `--status-fail` text + "Abort" label (color paired with text, never color alone per SC 1.4.1).
4. (design) Focus ring visible with `--color-focus` inset `0 0 0 2px` on keyboard focus; 150ms ease-out micro-transition on focus/hover (design-system.md §Motion).

## Relevant amendment history
- **2026-06-26-component-primitives-library:** Operator-pause dialog fade reconciled to `--motion-micro` (150ms) from the spec's aspirational 200ms. The OperatorPauseDialog scaffold shipped from component-primitives-library uses `var(--motion-micro)` for the entrance fade because no 200ms token exists in tokens.css (spec-illustration → sound-impl routine). The generic 0.3 expression-scale ceiling "200ms fades at most" remains (150ms satisfies it); apply `--motion-micro: 150ms` to all dialog transitions per this chunk's implementation. (Cascade to this chunk's motion token usage.) [SUPERSEDES layouts extract's "200ms fade" — design amendment is authoritative.]
