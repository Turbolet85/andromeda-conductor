# layouts extract

## Relevance
Partial — this chunk establishes CSS design tokens and typography that will be consumed by layout components in later UI epochs.

## Constraints
- Token names must match `design-system.md` exactly for Epoch 9 component binding (per scope §Surfaces/contracts touched)
- `@media (prefers-reduced-motion: reduce)` global reset applies at the token layer (per layout-templates §Motion, which cascades the 0.3-budget expression level)
- Desktop-webview surfaces only; CLI ANSI token mappings are out-of-scope for this chunk (per scope §Boundaries)
- Tailwind v4.1 Oxide static stylesheet via `@tailwindcss/vite`; zero runtime font fetches (per layout-templates §Surface: desktop-webview tooling + scope §Boundaries Minimal-tier offline hardening)
- Typography role scale (Display / Heading / Body / Label / Code / Data) must support both `--font-sans` (IBM Plex Sans) and `--font-mono` (JetBrains Mono) with ligatures OFF and `tabular-nums` on Data role (per layout-templates §Typography, which carries the role definitions)
- Color palette includes the signature placement tokens across all three placements: `--count-nominal` (primary heartbeat), `--count-hold` (freeze moment), `--count-blocked`, `--status-fail`, `--status-manual`, `--status-residual` (the supporting verdict convention) plus surfaces (`--color-raised-1..3`, `--color-base`/`-inset`), `--color-id-cyan`, and `--color-focus` (per layout-templates §Primary Surfaces wireframes and Component sections)

## Patterns to follow
- Token definitions verbatim from `design-system.md` §Tokens / §Color Palette / §Typography (no interpretation, no renaming)
- Auto mode via `@media (prefers-color-scheme: light)` override block (per layout-templates IA notes "multi-surface coordination")
- Spacing scale `4px-base` with tokens `space-xs/sm/md/lg/xl` used in layout wireframes (per layout-templates component padding / gap / height references throughout)
- Motion tokens `--motion-micro` and `--ease-quiet` tied to the 0.3-budget expression level (subtle color transitions on verdict lamps, no spring/parallax/staggered entrances)

## Anti-patterns to avoid
- Do NOT introduce runtime CDN font fetches or Google Fonts — only vendored Fontsource WOFF2
- Do NOT rename design-system tokens; Epoch 9 component chunks bind `var(--…)` by exact name
- Do NOT include Tauri window config, React component scaffolding, or IPC command tokens in this bundle

## Contract bindings
`design-system.md` §Tokens / §Color Palette / §Typography / §Surface: desktop-webview §Tooling ↔ this chunk's token + font output; Epoch 9 desktop component chunks ↔ token variable names (1:1 binding required); `security-plan.md` §Minimal-tier offline hardening ↔ font supply chain (Fontsource pinned + locked)

## Acceptance criteria contributions
- (layouts) Token stylesheet compiles via `@tailwindcss/vite` with zero runtime font CDN (per layout-templates §Expression level 0.3 + scope intent)
- (layouts) Both font families resolve from vendored WOFF2; `tabular-nums` applied to Data role, ligatures OFF globally (per layout-templates §Typography + scope §What it builds)
- (layouts) `@media (prefers-color-scheme: light)` and `@media (prefers-reduced-motion: reduce)` blocks present and functional (per layout-templates §Motion + a11y SC 2.3.3)
- (layouts) Token names in compiled stylesheet match `design-system.md` exactly (per scope §Surfaces/contracts + Epoch 9 binding requirement)

## Relevant amendment history
(none)
