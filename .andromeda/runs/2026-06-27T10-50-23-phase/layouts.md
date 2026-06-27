# layouts extract

## Relevance
partial — the chunk sets up a11y/GUI testing infrastructure for the desktop-webview surface, testing three existing layout components (RunReport, OperatorChecklistView, OperatorPauseDialog) but not creating or modifying layout structures.

## Constraints
1. Per §Component — Primary content block 2, "every lamp is paired with its status text (`Pass` / `CalibrationRegion` / `Fail` / `Blocked`) so state is never color-only" — the harness must verify this constraint in RunReport verdict rendering.
2. Per §Component — Primary content block 2, "Blocked rows ... measurement columns render `—` / null, **never a red error**" — the harness must verify Blocked verdicts do not render as visual errors.
3. Per §Component — Operator-checklist, "neutral-lavender `status-manual` checkbox glyphs (NOT the green/amber/red verdict triad)" — the harness must verify ManualCheck items use distinct non-verdict colors.
4. Per §Component — Hero / signature section (operator-pause dialog), "focus trapped with a visible `color-focus` ring" — the harness must verify dialog focus management and visible focus indicator.
5. Per §Wireframe (Run console HOLD) and §Component — Hero, "motionless over a `motion-micro` color transition — never flashing" — the harness must verify reduced-motion compliance for verdict lamp transitions.
6. Per §Component — Hero / signature section, "200ms fade entrance" — the harness must verify dialog entrance animation.

## Patterns to follow
1. Per §Component — Primary content block 2, verdict lamps render as `radius-full` dot at icon size grid, with color paired to text status — this pattern applies to all verdict rendering and must be verified in RunReport rows.
2. Per §Component — Operator-checklist, "Space toggles a row; keyboard-first, like the rest of the console" — OperatorChecklistView must support keyboard activation of ManualCheck items.
3. Per §Component — Hero / signature section, "shadcn `AlertDialog` ... focus trapped ... focus ring `color-focus`" — OperatorPauseDialog must implement focus trap and keyboard dismiss (Escape).
4. Per §Multi-surface coordination, the signature (paused-count hold-point) appears at three locations — titlebar, matrix header, dialog — with consistent visual treatment; the dialog header must echo the frozen count.

## Anti-patterns to avoid
1. Per §Component — Primary content block 2 and §Component — Primary content block 1, never render Blocked state as red or as a machine verdict failure; measurement columns for Blocked rows must render `—` / null.
2. Per §Component — Operator-checklist, never color-code ManualCheck items with the green/amber/red verdict triad; use only neutral-lavender `status-manual` glyphs.
3. Per §Wireframe (Run console HOLD) and §Component — Primary content block 2, never flash, blink, or pulse verdict lamps; all verdict state changes must resolve motionlessly.

## Contract bindings
- **Focus order / focus trap** (OperatorPauseDialog keyboard interaction) binds to a11y-plan §3 §Modal focus trap (Escape dismiss + focus restore) and a11y-plan §Focus Order SC 2.4.3.
- **Color contrast verification** (colorjs.io token-pair checking in the harness) binds to layout-templates §Multi-surface coordination (design tokens like `count-nominal`, `status-fail`, `color-focus` used consistently) and a11y-plan §3 (a11y harness contrast checking).
- **ARIA bindings for shadcn/Radix components** (RunReport, OperatorChecklistView, OperatorPauseDialog) bind to a11y-plan §3 (a11y harness) for semantic verification.

## Acceptance criteria contributions
- "(layouts) RunReport component renders verdict lines in primary-content-block region (main canvas area below matrix), with status lamp paired to text (layout-templates §Primary content block 2 wireframe)."
- "(layouts) OperatorPauseDialog renders as shadcn AlertDialog with focus-trapped modal over `color-raised-3` fill, `radius-lg` border, 200ms fade entrance, and visible `color-focus` ring (layout-templates §Hero / signature section)."
- "(layouts) Blocked verdicts render `—` / null for measurement columns, never red or as Fail-state rendering (layout-templates §Primary content block 2 component spec)."
- "(layouts) ManualCheck items in OperatorChecklistView render neutral-lavender `status-manual` checkbox glyphs, Space-toggleable, with unticked count surfaced in footer (layout-templates §Operator-checklist component spec)."

## Relevant amendment history
(none)