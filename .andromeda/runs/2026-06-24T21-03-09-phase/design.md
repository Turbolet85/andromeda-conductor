# design extract

## Relevance
Partial — only the desktop-webview frameless titlebar + deny-by-default capabilities apply; the signature element (frozen heartbeat), commands, and content panels are out-of-scope for THIS chunk.

## Constraints
- Per design-system §Brand Identity: frameless titlebar hosts the phase-line label (IBM Plex Sans 600, 18px, `--text-secondary`) left, count (JetBrains Mono 500, 28px, `--count-nominal` while on-timeline) center; both tinted in-place via 150ms CSS color transition on state change (design-system §Motion, expression-level 0.3).
- Per §Depth Strategy: borders-only elevation; all seams use `1px solid --border-subtle`; NO drop shadows, NO `backdrop-filter`, NO blur.
- Per §Spacing: titlebar padding uses `space-xs` (4px) tight gaps within the count/phase-line cluster, `space-lg` (20px) for major section separation (titlebar ↔ matrix below).
- Per §Border Radius: titlebar controls (minimize/close if included) use `radius-sm` (4px); frameless window declaration is `decorations: false` in `tauri.conf.json` (design-system §Surface: desktop-webview).
- Per §Typography: JetBrains Mono 500 (28px, tabular-nums, `--color-id-cyan`) reserved strictly for status-tier count/P-IDs/run_id/SLO timings; IBM Plex Sans prose tier; both self-hosted WOFF2 via `@fontsource/jetbrains-mono` + `@fontsource/ibm-plex-sans`, locally vendored into the Tauri bundle.
- Per §Component Patterns (desktop): custom titlebar is a `banner` landmark; window controls are semantic `<button>` elements with visible `--color-focus` ring on keyboard focus, NOT `<div role>`.
- Per §Color Palette: titlebar text uses `--text-secondary` for the phase line; count tints between `--count-nominal` and status-state colors over 150ms ease-out on state change; Auto mode mandatory (dark default + light variant `@media (prefers-color-scheme: light)`).

## Patterns to follow
- Reserved mono ID-cyan (`--color-id-cyan`) is the typographic-tier color for count/run_id/status tokens — when the count is NOT frozen by the operator-pause (this chunk is the static shell; the freeze animation is the next chunk), it renders in `--count-nominal` and updates via `transition: color 150ms`.
- Frameless window drag region established via `data-tauri-drag-region` attribute on the titlebar (Tauri 2 native); window controls (minimize/close) positioned right, never left, and NEVER float over the phase-line/count; use logical-properties (`inset-inline-end`) for macOS traffic-light compat.
- Deny-by-default capabilities block in `capabilities/*.json` opens with ONLY window-management permissions (e.g. drag, possibly minimize/close if trivially required); later Epoch-9 chunks introduce command permissions.

## Anti-patterns to avoid
- NEVER ship unstyled web scrollbars; NEVER allow the Chromium context-menu / devtools / text-selection on non-text elements to leak (suppress via attributes / pointer-events rules).
- NEVER use Inter / Roboto / Arial / Helvetica / system-ui default typefaces; NEVER mono-everywhere (mono-as-status-tier-only is the reserved discipline).
- NEVER add drop shadows, glassmorphism, or `backdrop-filter` — borders-only flat elevation only (design-system §Depth Strategy ban).
- NEVER use the count/phase-line as decoration (color = state, motion = state-change, never mood or animation library).

## Contract bindings
- a11y §Landmarks: titlebar renders as `<header role="banner">` (or semantic HTML5 element) with visible `--color-focus` ring on window-control button focus (SC 2.4.7).
- security-plan §Tauri GUI: `tauri.conf.json` includes `build { frontendDist, devUrl }`; deny-by-default `capabilities/*.json` with no blanket or remote-origin permissions; Tauri ≥ 2.10.3 enforced.
- obs-plan §3: Tauri backend `tracing` → `logs/conductor-tauri.jsonl` via extended `ObsSink`; `service.name` = `conductor-tauri`.

## Acceptance criteria contributions
- (design) Frameless titlebar renders with `data-tauri-drag-region` drag region; window is draggable; phase-line + count tint in static (non-animated) `--text-secondary` + `--count-nominal` and update via `transition: color 150ms var(--ease-quiet)` on state change.
- (design) All titlebar text + colors trace back to design-system tokens (`--color-*`, `--text-*`, `--count-*`, `--space-*`, `--radius-*`, `--motion-*`, `--ease-*`); no hardcoded hex/px/ms values.
- (design) Typography uses `@fontsource/jetbrains-mono` + `@fontsource/ibm-plex-sans` self-hosted (no CDN runtime); font-sizes + weights match design-system table (count = 28px Mono 500, phase-line = 18px Sans 600).
- (design) Auto mode respected: dark ground `--color-base` + light variant applied via `@media (prefers-color-scheme: light)` on `:root`; titlebar legend always human-readable.

## Relevant amendment history
- 2026-06-15-design-token-typography-bundle: `:root` token declaration (not `@theme`) to preserve all 34 tokens through tree-shaking; titlebar inherits this pattern — tokens on `:root`, never inline hex.
- 2026-06-24-sanitized-stderr-agent-mode-logging: CLI error:/hint: labels (ANSI 203/246) reuse Fail + Residual tokens — no new colors; titlebar is desktop-only (cli error path applies to stderr, distinct gate).