# layouts extract

## Relevance
Relevant — chunk builds the desktop operator-pause go/no-go dialog (§Component — Hero/signature), a modal overlay that gates the HOLD moment.

## Constraints
- Dialog is shadcn AlertDialog (Radix `alertdialog` role) with focus-trap + Escape dismiss + visible focus restoration (layout-templates.md §Component — Hero/signature).
- Dialog header displays frozen count snapshot (Display role, `count-hold` color) at decision point — signature placement #2 (layout-templates.md §Wireframe Run console HOLD, §Component — Hero/signature).
- Proceed button is primary (`count-nominal` accent); Abort button is `status-fail` text; both text-labeled (never icon-only) (layout-templates.md §Component — Hero/signature).
- Dialog entrance: 200ms fade, no spring/pulse (layout-templates.md §Component — Hero/signature, §Decisions Log). [NOTE: design amendment 2026-06-26 reconciles this to 150ms `--motion-micro` — design is authoritative.]
- Titlebar count remains frozen while dialog open (carried from existing signature); dialog complements not replaces that freeze (layout-templates.md §Wireframe Run console HOLD, §Component — Header).
- Console behind dialog is dimmed and focus-inert; only dialog controls are focusable (layout-templates.md §Wireframe Run console HOLD).
- Dialog is the single modal for the hold — no stacking modals (layout-templates.md §Primary screens HOLD).

## Patterns to follow
- Single-station overlay (no route, no URL): dialog is an overlay state within the one Run console surface, not a navigation target (layout-templates.md §Primary screens, §Component — Primary navigation).
- Signature placement strategy: dialog header echoes the titlebar's frozen count so the held value is legible at the proceed/abort decision (layout-templates.md §Decisions Log — signature placement #2 within §Component — Hero/signature).
- Motion-is-the-event: count freeze (absent motion in titlebar) + dialog fade are the only motion; no verdict-lamp flash in the held state (layout-templates.md §Decisions Log).
- Status-with-text: Proceed/Abort buttons always have text labels; No color-alone affordances (layout-templates.md §Component — Hero/signature).

## Anti-patterns to avoid
- No flashing / blinking verdict lamps — the dialog and hold state are motionless (layout-templates.md §Component — Hero/signature, §Decisions Log).
- No user-dismissible-without-decision modal — Proceed/Abort are required; Escape dismisses as Abort (layout-templates.md §Component — Hero/signature).
- No overlapping/cascading modals during hold (layout-templates.md §Primary screens HOLD).

## Contract bindings
- Focus trap + Escape dismiss → a11y §Modal focus trap (visible focus ring, Escape restores focus to console).
- Focus order (Proceed then Abort) → a11y §Focus Order SC 2.4.3 (visible reading order matches focus order).
- Dialog fade entrance + count tint `motion-micro` → design §Motion (color-only transition, no spring).

## Acceptance criteria contributions
- "(layouts) Dialog renders as overlay over Run console surface, focus-trapped (layout-templates.md §Wireframe Run console HOLD)."
- "(layouts) Dialog header displays frozen count in `count-hold` color at decision point (layout-templates.md §Component — Hero/signature)."
- "(layouts) Proceed then Abort buttons are focusable; Escape dismisses as Abort (layout-templates.md §Component — Hero/signature, §Component — Primary navigation)."
- "(layouts) Dialog enters via fade (`--motion-micro`); count tint transitions over `motion-micro` (layout-templates.md §Component — Hero/signature, §Wireframe Run console HOLD)."

## Relevant amendment history
- **2026-06-24-frameless-window-shell** (layout-templates.md §Wireframe Run console HOLD, §Component — Header): titlebar height reconciled `space-lg` → `space-xl` to accommodate the `Heading`-tier phase line + window-control glyphs. Relevant: the HOLD wireframe (which shows the dialog + frozen titlebar) was updated in lock-step with the Component-Header spec; this chunk wires the dialog alongside that titlebar state.
