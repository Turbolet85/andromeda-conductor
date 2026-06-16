# design extract

## Relevance
Relevant — this chunk embeds the complete design-system token layer and typography stack that downstream UI components will bind.

## Constraints
1. **Token names must match exactly** — `design-system.md` §Color Palette / §Typography / §Tokens specify the authoritative names (`--color-base`, `--count-nominal`, etc.); Epoch 9 components bind via `var(--…)` without renaming (per scope.md contract).
2. **Tailwind v4.1 `@theme` block verbatim** — the `@theme` CSS custom-property block carries the dark-default values + `@media (prefers-color-scheme: light)` light-variant overrides from `design-system.md` lines 193–241 (design-system.md §Surface: desktop-webview).
3. **Expression level 0.3 motion tokens** — `--motion-micro: 150ms` + `--ease-quiet: cubic-bezier(0,0,0.2,1)` are hardcoded per the Brand Identity expression level (design-system.md §Motion); no expression-level flexibility in this chunk.
4. **Self-hosted fonts only** — `@fontsource/jetbrains-mono` + `@fontsource/ibm-plex-sans` vendored WOFF2, no Google Fonts / runtime CDN (design-system.md §Typography + security-plan Minimal-tier offline hardening).
5. **Ligatures OFF for JetBrains Mono** — the status tier (P-IDs, count, run_id) requires strict glyph legibility; `font-feature-settings: "liga" 0` on the mono tier (design-system.md §Typography, tabular-nums for Data role).
6. **`@media (prefers-reduced-motion: reduce)` reset required** — global `animation: none !important; transition: none !important` block per design-system.md line 239 (binds a11y SC 2.3.3; design-system.md §Motion / Anti-Patterns).
7. **Fallback stacks declared** — `"JetBrains Mono", ui-monospace, monospace` and `"IBM Plex Sans", ui-sans-serif, sans-serif` ensure graceful degradation (design-system.md §Typography Loading).

## Patterns to follow
1. **Token hierarchy by role** — surfaces, status tier, text tiers, borders, spacing, radius scale (design-system.md §Color Palette line 29–71, §Spacing, §Border Radius); every downstream component queries this single source.
2. **Auto mode dark-first structure** — dark values in the default `@theme` block, light overrides under `@media (prefers-color-scheme: light)` (design-system.md line 223–236); no forced dark-mode class, respect OS preference.
3. **4px base unit for spacing scale** — `space-micro` (2px, exception for micro gaps), `space-xs` (4px), `space-sm` (8px), `space-md` (12px), `space-lg` (20px), `space-xl` (32px); all multiples of 4px except the 2px exception (design-system.md §Spacing line 99–108).
4. **Typography role scale expressible from font custom properties** — define `--font-mono` / `--font-sans` so Display (JetBrains Mono 500 28px), Heading (IBM Plex Sans 600 18px), Body (400 14px), Label (500 12px), Code (JetBrains Mono 400 13px), Data (500 13px, `tabular-nums`) compose without redundant declarations (design-system.md §Typography lines 79–86).
5. **Mono ID-cyan as reserved status tier** — `--color-id-cyan: #7DCFFF` (dark) / `#0969DA` (light); reserved for P-IDs, run_id, SLO timings, fingerprints, focus rings; never as a generic brand accent (design-system.md §Color Palette line 32, §Anti-Patterns line 328).

## Anti-patterns to avoid
1. **NEVER hardcode hex values or pixel values in components** — all downstream UI must query `var(--color-*)` / `var(--space-*)` / `var(--radius-*)` / `var(--font-*)` from this bundle (design-system.md §Self-Validation line 376, §Anti-Patterns §Universal Bans).
2. **NEVER use Inter, Roboto, Arial, Helvetica, or system-ui as primary faces** — Inter is explicitly the banned Linear-anchor face; this chunk must not provide fallback confusion (design-system.md §Anti-Patterns §Universal Bans line 324).
3. **NEVER apply motion tokens (duration/easing) without wrapping in `@media (prefers-reduced-motion: reduce)`** — even this foundation chunk's motion tokens must be gated; downstream will enforce, but the reset here prevents leakage (design-system.md §Motion line 169).

## Contract bindings
- **a11y §Contrast** — dark-default colors and light-variant text pairs must meet WCAG SC 1.4.3 4.5:1 (the design-system specifies this; Epoch 9 a11y harness will verify).
- **a11y §Animation (SC 2.3.3)** — `@media (prefers-reduced-motion: reduce)` reset in this chunk is the foundational binding; downstream motion use in Epoch 9 depends on this global override.
- **security-plan §Offline hardening** — fonts vendored WOFF2 via Fontsource (no CDN); npm/Fontsource deps pinned + lockfiled.
- **frontend.md + a11y.md scope rules** — begin applying once `crates/conductor-tauri/**` files land (this chunk introduces them).

## Acceptance criteria contributions
1. **(design) Tailwind v4.1 `@theme` block compiles to static CSS** — the `@tailwindcss/vite` Oxide engine (zero-runtime) emits a stylesheet from the `@theme` token definitions with zero runtime font fetches.
2. **(design) Token names match `design-system.md` exactly** — `--color-base`, `--count-nominal`, `--color-id-cyan`, `--space-xs`, `--radius-md`, `--font-mono`, `--motion-micro`, `--ease-quiet`, etc. are declared with values verbatim from the plan (no renames, no substitutions).
3. **(design) Light-mode and reduced-motion overrides are present** — `@media (prefers-color-scheme: light)` block with all light-variant hex values from design-system.md line 228–236; `@media (prefers-reduced-motion: reduce)` reset from line 239.
4. **(design) Fonts resolve from vendored WOFF2, no runtime CDN calls** — `@fontsource/jetbrains-mono` + `@fontsource/ibm-plex-sans` WOFF2 loaded locally; browser Network tab shows zero requests to Google Fonts or external CDN.

## Relevant amendment history
(none) — the amendments file does not exist (normal on a fresh project); this is the initial design-system establishment.
