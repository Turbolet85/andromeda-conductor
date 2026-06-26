# design extract

## Relevance
Partial — applies to the picker UI, button states, and container styling; excludes live counters and operator-pause dialog.

## Constraints
1. Design tokens by NAME only: colors (`--color-*`, `--count-*`, `--status-*`), typography (font stack + size token), spacing (scale multiples of 4px), motion (CSS transitions only, 150ms ease-out) — no hardcoded hex/pixel values (design-system §Spacing, §Motion, §Surface tokens).
2. Expression level 0.3 governs all motion — functional only, no animation libraries, no spring physics, no staggered reveals; all motion must drop under `@media (prefers-reduced-motion: reduce)` (design-system §Brand Identity, §Motion).
3. Picker is keyboard-operable with visible focus ring (`--color-focus` inset ring, no shadow except `0 0 0 2px`) and semantic `<button>` controls (design-system §Depth Strategy, §Surface: desktop-webview / Component Patterns §5, §a11y-plan.md).
4. Picker rows and button text use text-first display, never color-alone; selection state is signaled by border emphasis + text label, not tint (design-system §Color Palette §Semantic Colors, §Component Patterns §5).
5. Inputs and picker container fill `--color-raised-2` with `1px --border-standard` outline + `--color-inset` input backgrounds; picker rows styled with `--radius-sm` (design-system §Surface Scale, §Border Radius, §Depth Strategy).
6. Start/Stop buttons inherit label styling (`IBM Plex Sans 500, 12px`) and enabled/disabled semantics via `--text-primary` (enabled) / `--text-muted` (disabled) text, never color-signal-alone (design-system §Typography §Label, §Anti-Patterns §Universal Bans).
7. All status-lamp + dialog primitives deferred to ch5/ch8; ch3 picker uses shadcn Command/Select only (design-system §Component Patterns §5, §scope.md boundaries).

## Patterns to follow
1. Scenario/suite picker: shadcn Command/Select (Radix-driven) over `--color-raised-2`, P-ID column in ID-cyan, mono-tier text per status hierarchy (design-system §Component Patterns §5).
2. Start/Stop buttons: semantic `<button>` with enabled-state gate (Start: selection present, Stop: run in flight); 150ms hover background lift (design-system §Motion §Hover/Focus, §Surface: desktop-webview §Keyboard-first run control).
3. Focus ring: single `0 0 0 2px var(--color-focus)` inset (no shadow, no blur — borders-only depth strategy) on all interactive elements (design-system §Depth Strategy, §Border Radius).

## Anti-patterns to avoid
1. NEVER use color-alone to signal selection or disabled state; always pair with text label (e.g., "disabled" label text `--text-muted`, not greyed background) (design-system §Anti-Patterns §Universal Bans, §Component Patterns §5).
2. NEVER add animation libraries (framer-motion, etc.) or spring physics; CSS transitions only, max 150ms ease-out (design-system §Motion §Hard limits, §Brand Identity §Expression 0.3).
3. NEVER introduce new color tokens not listed in the palette; reuse semantic slots (e.g., error → `--status-fail`, hint → `--status-residual`) (design-system §Anti-Patterns §Universal Bans, §amendments-log §2026-06-24-sanitized-stderr).

## Contract bindings
**Tauri `#[tauri::command]` + `capabilities/*.json`**: the list/start/stop commands need deny-by-default capability entries (security-plan.md); **keyboard operability + focus order**: roving focus + type-ahead from Radix primitives bind to a11y-plan.md §Keyboard §Forms-controls; **run-state visibility**: Start/Stop enabled state mirrors the backend lifecycle (titlebar `RunState` prop mirrors the picker's start-gate state).

## Acceptance criteria contributions
1. **(design) Picker uses only named tokens** — all colors are `var(--color-*)` / `var(--count-*)` / `var(--status-*)`, spacing is multiples of `var(--space-*)`, no hex or px literals in styles.
2. **(design) Motion respects `prefers-reduced-motion: reduce`** — all 150ms hover transitions drop under the `@media` block; no animation plays when reduced-motion is active.
3. **(design) Selection and disabled state paired with text** — enabled/disabled buttons render text in `--text-primary` / `--text-muted`; picker selection state pairs text label + border emphasis (not color-alone tint).
4. **(design) Picker rows follow density scale** — `--space-sm` (8px) internal padding, `1px --border-subtle` dividers, `--radius-sm` (4px) corners (design-system §Spacing, §Border Radius, instrument-panel density).

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle**: token block declared on `:root` (not `@theme`); all 34 tokens emit (no tree-shaking losses); `--space-*`, `--radius-*`, `--motion-micro`, `--ease-quiet` preserved (affects Tailwind v4 integration care for ch3's shadcn landing).
- **2026-06-24-paused-count-hold-point-signature**: `--motion-heartbeat: 1600ms` registered in motion-tokens table (ch3 defers live counter to ch4; focus here is picker/button 150ms hover + `prefers-reduced-motion` override).
