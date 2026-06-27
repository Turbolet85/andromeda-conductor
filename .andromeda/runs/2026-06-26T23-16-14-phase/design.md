# design extract

## Relevance
Relevant — the chunk renders presentational React primitives that materialize design tokens, colors, typography, spacing, depth, motion, and accessibility patterns.

## Constraints
- (design-system §Color Palette) Every color must cite a `--color-X` or `--count-*` / `--status-*` token; status-lamp variants must pair color + text label (Color-Only a11y rule SC 1.4.1)
- (design-system §Typography) Label tier: IBM Plex Sans 500, 12px, 1.35 tracking for lamp labels and dialog copy
- (design-system §Spacing) Component padding uses 4px base scale (space-micro 2px / space-xs 4px / space-sm 8px / space-md 12px)
- (design-system §Border Radius) Status-lamp chips radius-sm 4px; dialogs radius-lg 8px; verdict-lamp dots radius-full 9999px
- (design-system §Depth Strategy) Borders-only (1px solid inset-border, no shadows except focus ring `0 0 0 2px var(--color-focus)` inset on keyboard focus)
- (design-system §Motion, expression 0.3) Dialog fade 200ms ease-out; hover/focus 150ms ease-out; NO animation libraries; all transitions MUST drop under `@media (prefers-reduced-motion: reduce)`
- (design-system §Iconography) Lucide React for controls (20px dialog actions, 16px status lamps) + custom SVG status-lamp glyphs; every icon paired with text label

## Patterns to follow
- (design-system §Component Patterns §4) Verdict/report-state lamps: radius-full status-light per-P-ID — six states mapped as `--count-nominal` (Pass) / `--count-hold` (Hold/CalibrationRegion) / `--status-fail` (Fail) / `--count-blocked` hollow-ring (Blocked) / `--status-manual` checkbox-glyph (ManualCheck) / `--status-residual` dashed-ring (KnownResidual); color transitions in-place at 150ms ease-quiet; always paired with text label + `aria-live`
- (design-system §Component Patterns §2) Dialog scaffold: shadcn/ui Dialog/AlertDialog (Radix), Surface-3 `--color-raised-3` fill, 1px `--border-subtle`, radius-lg, 200ms fade, focus-trapped, ESC/overlay dismiss, ARIA-labelled `role="alertdialog"`/`role="dialog"`, visible `--color-focus` ring
- (design-system §Component Patterns §7) Operator-checklist: card rows per item, induced state + expected observation yes/no, space-to-toggle keyboard-first, `--status-manual` neutral-lavender checkbox glyphs (NOT the verdict triad)

## Anti-patterns to avoid
- NEVER hardcode hex or pixel values — use design tokens only
- NEVER render color state alone without paired text label or icon (Color-Only a11y violation)
- NEVER use animation libraries (framer-motion, spring, parallax, stagger, 3D) — CSS transitions only; never skip `@media (prefers-reduced-motion: reduce)`

## Contract bindings
- **Token contrast**: status-lamp colors paired with text must meet SC 1.4.3 4.5:1 (contrast audit deferred to ch9 a11y harness)
- **Motion + a11y**: dialog 200ms fade + 150ms micro MUST drop entirely under `prefers-reduced-motion: reduce` (SC 2.3.3)
- **Color + label**: every status lamp + icon-only button MUST carry `aria-label` or paired text per SC 1.4.1

## Acceptance criteria contributions
- (design) All component colors source from `styles/tokens.css` `:root` tokens (no hardcoded hex)
- (design) All six status-lamp variants render with paired text label + glyph (Pass · Fail · Hold · ManualCheck · KnownResidual · Blocked) matching `design-system.md` §Color Palette / §Iconography §4 verdict-first precedence
- (design) Dialog scaffold surfaces use Surface Scale tokens (`--color-raised-3`), focus ring `var(--color-focus)`, fade drops to `transition: none` under `prefers-reduced-motion: reduce`
- (design) Operator-checklist renders space-to-toggle + `--status-manual` neutral-lavender checkbox glyphs per §Component Patterns §7 (no red Fail on unchecked)

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** — Tokens declared on `:root` (not `@theme`) with `@import "tailwindcss"`; all 34 tokens materialize. This chunk's `styles/tokens.css` import + Tailwind v4 config follow the `:root` pattern.
- **2026-06-24-paused-count-hold-point-signature** — Registers `--motion-micro: 150ms` + `--motion-heartbeat: 1600ms`; this chunk uses `--motion-micro` for dialog fade + checklist/hover focus; no raw `150ms` literals.