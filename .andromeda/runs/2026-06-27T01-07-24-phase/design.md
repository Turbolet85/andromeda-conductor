# design extract

## Relevance
Relevant — the chunk implements desktop run-report + operator-checklist views using verdict/state colors, typography, spacing, and component patterns from the design system.

## Constraints
1. per §Color Palette: Every verdict/report-state must use the semantic color row (Pass green / CalibrationRegion amber / Fail red / Blocked slate-violet / ManualCheck lavender / KnownResidual muted)
2. per §Color Palette: Status is never color-alone — every verdict/state row pairs the semantic color with text label + StatusLamp glyph
3. per §Typography: Run-report body uses IBM Plex Sans 400 14px; P-IDs/run_id/SLO timings use JetBrains Mono 500 13px tabular-nums in `--color-id-cyan`
4. per §Spacing: Run-report row padding uses `space-md` (12px); card internal gaps use tokens from the scale (space-sm / space-md / space-lg)
5. per §Depth Strategy: Run-report card + operator-checklist rows use `1px solid --border-subtle` + `--color-raised-1` fill + `--radius-md` (borders-only elevation, NO shadows)
6. per §Component Patterns §4: StatusLamp renders all 6 states (Pass/CalibrationRegion/Fail/Blocked/ManualCheck/KnownResidual) with distinct glyphs (filled dot / filled dot / filled dot / hollow ring / checkbox / dashed dot) + text label + aria-live announcement
7. per §Component Patterns §7: Operator-checklist renders ManualCheck rows with induced state + expected observation checklist (space toggles rows, ticking resolves verdict to Pass/Fail, text footer rolls up unticked count)

## Patterns to follow
1. per §Component Patterns §6: Run-report view structure — `--color-raised-1` card, `1px --border-subtle`, `--radius-md`, with P-ID / scenario / verdict lamp + text / latency / SLO columns; ManualCheck rows expand into operator-checklist (Pattern §7)
2. per §Component Patterns §4: StatusLamp glyph library — Pass/Fail/CalibrationRegion as filled dots in semantic colors (green / red / amber); Blocked as slate-violet hollow ring (empty); ManualCheck as neutral-lavender checkbox glyph (`☐`/`☑`); KnownResidual as muted dashed-ring dot; every lamp pairs with text (`Pass`/`Fail`/`CalibrationRegion`/`Blocked`/`ManualCheck`/`Residual`) and aria-live announcement
3. per §Component Patterns §7: Operator-checklist card structure — induced state + expected observation (`☐` / `☑`) rows, space-to-toggle keyboard control, ticking resolves item verdict while report-state remains ManualCheck, footer shows unticked count
4. per §Typography: Mono P-IDs rendered in ID-cyan (`--color-id-cyan`) with tabular-nums alignment, body text in secondary-grey (`--text-secondary`)
5. per §Spacing + §Depth Strategy: All containers (card, rows, checklist items) use `space-md` padding (12px), `1px --border-subtle` seams, `--radius-md` corners, never shadows

## Anti-patterns to avoid
1. Color-only state signaling — every verdict/state MUST pair color with text label + icon (per §Anti-Patterns §Universal Bans "NEVER use color purely for decoration")
2. Shadows, backdrop-filter, or glassmorphic elevation — borders-only depth only (per §Depth Strategy "Drop shadows: none")
3. Non-canonical fonts (Inter, Roboto, Arial, system-ui) — enforce IBM Plex Sans (body) + JetBrains Mono (data tier)

## Contract bindings
- **Semantic color contrast** → a11y §Contrast (each Verdict/ReportState color pair must meet SC 1.4.3 4.5:1 dark/light variant contrast when rendered on its respective surface fill)
- **StatusLamp + aria-live announcement** → a11y §Use of Color SC 1.4.1 + §Error Messages (verdict/state announced programmatically, not color-alone)
- **Checkbox toggle motion** → a11y §Animation SC 2.3.3 (if the operator-checklist checkbox render animates a transition, it must respect `prefers-reduced-motion: reduce`)

## Acceptance criteria contributions
1. (design) Verdict/ReportState colors match §Color Palette §Semantic Colors table (Pass/CalibrationRegion/Fail/Blocked/ManualCheck/KnownResidual rows, dark + light variants).
2. (design) StatusLamp renders all 6 states with correct glyphs + text label + aria-live announcement (Color-Only rule SC 1.4.1).
3. (design) Typography uses only IBM Plex Sans body + JetBrains Mono data tier per §Typography (no generic fonts).
4. (design) Run-report card + operator-checklist rows use `--color-raised-1` + `1px solid --border-subtle` + `--radius-md` (borders-only, no shadows).
5. (design) All pixel values (padding, radius, borders) cite spacing/radius tokens from §Spacing / §Border Radius (no hardcoded px).
6. (design) Operator-checklist checkbox toggle (if animated) respects `prefers-reduced-motion: reduce` (SC 2.3.3).

## Relevant amendment history
1. **2026-06-26-component-primitives-library (§Motion reconciliation)** — operator-pause dialog fade uses `--motion-micro` (150ms), not 200ms literal; if operator-checklist checkbox toggle animates, use `--motion-micro` (150ms ease-out) per the component-primitives precedent (tokens-by-name, no raw ms literals).
2. **2026-06-15-design-token-typography-bundle (§Tokens `:root` declaration)** — design tokens are declared on `:root` with `@import "tailwindcss"`, not `@theme` (Tailwind v4 preserves all 34 tokens on `:root`; `@theme` tree-shakes non-namespace tokens). No change to token NAMES/VALUES — the binding contract is stable.
