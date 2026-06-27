# layouts extract

## Relevance
relevant — this chunk builds component primitives (status lamps, dialog scaffold, operator-checklist) the desktop-webview surface composes into views (coverage matrix, run-report, operator-pause dialog)

## Constraints
1. Per layout-templates §Component — Primary content block 1: Status-lamp glyph renders at radius-full icon-size grid; encodes one of six distinct states via glyph shape (filled dot / hollow ring / checkbox / dashed dot).
2. Per layout-templates §Component — Primary content block 2: Each lamp must pair color with text label + glyph (never color-only); preserve distinct state taxonomy (Pass/Hold/Fail/ManualCheck/KnownResidual/Blocked) — no silent collapse to Fail.
3. Per layout-templates §Component — Hero/signature section: Dialog scaffold uses shadcn AlertDialog (Radix); focus-trap on open, Escape dismiss, restore focus on close; 200ms fade; color-raised-3 fill + border-subtle + radius-lg.
4. Per layout-templates §Component — Operator-checklist: Rows render as cards (color-raised-1, radius-md, border-subtle); left induced-state; right yes/no observation checkbox; Space toggles; status-manual neutral lavender.
5. Per layout-templates §Surface: desktop-webview — Tooling: shadcn/ui + Radix + Lucide React; Tailwind v4.1; consume established styles/tokens.css tokens (count-hold/count-blocked/status-fail/status-manual/status-residual/color-focus/motion-micro); add lamp tokens only if design-system.md defines new ones.
6. Per layout-templates §Component — Primary content block 2: Lamps resolve motionless (motion-micro color-transition only); never flash/blink/pulse; expression 0.3.

## Patterns to follow
1. Per layout-templates §Component — block 2: Pair lamp glyph + text label + color; render motionless color transitions.
2. Per layout-templates §Component — Hero section: Dialog header carries Display-role frozen-count snapshot (count-hold color) at decision moment; focus-trapped with visible color-focus ring.
3. Per layout-templates §Component — blocks + nav: Lucide React glyphs at icon-size grid; accessible names on icon-only controls; visible focus indicator (color-focus) on all interactive elements.

## Anti-patterns to avoid
1. Never render status lamps color-only (no glyph / no text label) — universal invariant per layout-templates §Component block 2.
2. Never collapse ManualCheck/KnownResidual/Blocked into Fail — preserve distinct state taxonomy per layout-templates §Component block 2.
3. Never animate lamps beyond motion-micro color transition; no pulse/blink/glow; 0.3 expression budget per layout-templates §Surface desktop-webview.

## Contract bindings
- **a11y ↔ layouts (focus/keyboard/live-region):** Dialog focus-trap (Escape dismiss, restore focus); visible color-focus ring; accessible names on icon-only glyphs; live-region announcement for verdict changes; checklist Space-toggle.
- **design-system ↔ layouts (lamp tokens):** Lamp color+glyph+label consume established styles/tokens.css tokens; extend only if design-system.md defines new ones.
- **status-semantics ↔ layouts (taxonomy):** Lamp taxonomy stays byte-consistent with ReportState/Verdict mapping; verdict-first precedence; shared vocabulary across desktop/CLI/Markdown.

## Acceptance criteria contributions
1. (layouts) Status-lamp component renders all six states with paired glyph+label+color; no state renders color-only.
2. (layouts) Dialog scaffold opens with 200ms fade, traps focus, dismisses on Escape/overlay click, restores focus on close; title/body/action slots render in color-raised-3 + border-subtle + radius-lg.
3. (layouts) Operator-checklist renders rows as cards (color-raised-1, radius-md, border-subtle); Space toggles yes/no per row; status-manual neutral lavender glyph (not verdict colors).
4. (layouts) Lamp taxonomy aligns with CLI [PASS]/[HOLD]/[FAIL]/[BLOCKED]; verdict-first precedence (CalibrationRegion renders as HOLD, not Manual).

## Relevant amendment history
**2026-06-24-frameless-window-shell** — Titlebar height reconciled space-lg → space-xl. Affects desktop-webview vertical budget; icon-size-grid lamps respect this when composed into views (ch6+).