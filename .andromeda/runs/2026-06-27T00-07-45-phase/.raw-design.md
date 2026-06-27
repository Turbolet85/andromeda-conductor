# design extract

## Relevance
Relevant. The coverage-matrix-view chunk is a desktop-webview React 19 component rendering a dense list of 60 P-IDs with verdict/report-state lamps — squarely within design scope (typography, colors, spacing, borders, iconography, component composition).

## Constraints
- Per §Brand Identity: The Coverage matrix is a signature domain anchor — "the dense single-row-per-P-ID wall of all 60 capabilities" is *the* mission-control console instrument, never a dashboard grid; the dense pattern anchors the entire aesthetic.
- Per §Component Patterns §3: Coverage matrix is "a single-row-per-P-ID list (Linear instrument-panel density), 12px row padding, `1px --border-subtle` dividers, no shadows" — NOT a KPI-card grid (explicit Rejected Default §Anti-Patterns).
- Per §Typography: P-IDs render in JetBrains Mono (Data tier: 13px / 500 weight / tabular-nums / ID-cyan); scenario titles + coverage-mode labels in IBM Plex Sans (Label tier: 12px / 500); supporting text in IBM Plex Sans Body (14px / 400). No Inter / Roboto / system-ui fallback (§Anti-Patterns "NEVER use generic font families").
- Per §Color Palette + §Iconography: StatusLamp is never color-alone — every row's lamp glyph must pair with a text label (`Pass` / `HOLD` / `Fail` / `Blocked` / `Manual` / `Residual`); lamp vocabulary comes from `conductor-core::Lamp` (verdict-first via `Lamp::for_record` precedence, per the CARRY annotation).
- Per §Depth Strategy + §Border Radius: Rows use `1px solid --border-subtle` inset borders (no shadows); coverage-matrix container `--radius-md` (6px); flat elevation only.
- Per §Spacing: Row padding `space-md` (12px); 1px dividers between rows; no excess whitespace — the 60-P-ID wall reads as one cohesive dense list, not stacked cards.

## Patterns to follow
1. **Dense single-row-per-P-ID list** (§Component Patterns §3): Each row a horizontal strip with P-ID (mono ID-cyan, tabular-nums) · scenario title (sans label) · coverage mode (sans label) · StatusLamp glyph + label; no card expansion, no multi-line wrapping, no tile grid.
2. **Never-color-alone status encoding** (§Color Palette / §Iconography / a11y binding): Pair the StatusLamp glyph with its text label (`Pass`/`HOLD`/`Fail`/`Blocked`/`Manual`/`Residual`) on every row; removing the label is a failure (SC 1.4.1 Use of Color).
3. **Reuse StatusLamp primitive unchanged** (CARRY annotation, scope §Boundaries): Consume `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` + `ui/src/lamp.ts` `LAMP_META` as-is; project RunRecord → Lamp verdict-first using `conductor-core::Lamp::for_record` semantics in TypeScript (byte-consistent with Rust, shared vocabulary — never re-spell lamp palette/glyph/label).
4. **Design-token sourcing** (§Surface Tokens / §Anti-Patterns "Token Test"): All color / spacing / radius values trace to the `:root` token contract (per amendment 2026-06-15); no hardcoded hex / px values (run `tsc` strict + `npm audit` 0 to gate).

## Anti-patterns to avoid
1. **Cookie-cutter card grid** (§Anti-Patterns): Reject any spacing/padding/elevation between rows that breaks instrument-panel density; the coverage matrix is a tight list, not a card wall.
2. **Color-only status signaling** (§Anti-Patterns / a11y binding): Do not render a status lamp without its text label; a dot-only or color-only row is an accessibility failure (SC 1.4.1) and violates the never-color-alone rule.
3. **Font fallback to generic families** (§Anti-Patterns "NEVER use generic font families"): P-IDs and data must use JetBrains Mono (not monospace system fallback as primary); titles must use IBM Plex Sans (not ui-sans-serif default) — otherwise the mission-control identity collapses into generic dev-tool styling.

## Contract bindings
- **Layout (layout-templates.md)**: The chunk integrates as a view component under the App / window shell per Epoch-9 pattern; the placement (always-on panel vs switchable view) and column/row layout are layout decisions, not design constraints here.
- **Accessibility (a11y harness ch9)**: The component bakes semantic table/list structure, keyboard row navigation, and never-color-alone lamp encoding at the component level; the a11y harness (ch9) verifies contrast / Lighthouse / axe compliance and gates on ch9-level GUI-integration tests.
- **StatusLamp + conductor-core::Lamp (component-primitives ch5 + conductor-core)**: The verdict/report-state lamp vocabulary is consumed unchanged from `StatusLamp.tsx` and `LAMP_META`; any lamp affordance gap is a plan call, not a re-spell in the coverage-matrix-view.

## Acceptance criteria contributions
1. **(design) Token-sourced colors only** — P-ID text `--color-id-cyan`, row text `--text-primary` / `--text-secondary`, row borders `--border-subtle`, container border-radius `--radius-md`; zero hardcoded `#` values (audit: `grep -E '#[0-9A-Fa-f]{6}' src/` must be empty except tokens.css import).
2. **(design) StatusLamp + label always paired** — every P-ID row's lamp glyph rendered alongside its text (`Pass`/`HOLD`/`Fail`/`Blocked`/`Manual`/`Residual`); if a lamp renders alone, the component fails and must be reworked.
3. **(design) Typography grid locked** — P-IDs in JetBrains Mono (13px / 500 / tabular-nums), scenario titles in IBM Plex Sans (12px / 500), supporting text in IBM Plex Sans (14px / 400); no font-family `@apply` overrides that deviate (verify via component snapshot test).
4. **(design) Dense list pattern confirmed** — row padding `space-md` (12px), row dividers `1px solid --border-subtle`, no box-shadow, no excess spacing; the 60-P-ID wall visually reads as a cohesive Linear instrument-panel, not a card grid (visual regression test: compare to reference screenshot at 1920×1080).

## Relevant amendment history
1. **2026-06-15-design-token-typography-bundle (§Surface Tokens)**: Token declaration moved from `@theme` to `:root` with `@import "tailwindcss"` engine load; the 34-token binding contract now emits 34/34 (not 23/34 tree-shaken). Why: Tailwind v4 `@theme` nested inside `@media` fails + `@theme` tree-shakes non-namespace tokens. **Relevant**: the coverage-matrix-view component must declare all token-sourced values using the `:root` contract (the canonical 34-token inventory); no magic numbers, all values sourced from the root CSS custom properties.
2. **2026-06-24-paused-count-hold-point-signature (§Motion tokens table added)**: `--motion-heartbeat: 1600ms` and `--ease-quiet` registered as named motion tokens. Why: the Epoch-9 count component added heartbeat period to `tokens.css` but was unregistered in the design plan, drifting D-design-tokens. **Relevant**: the token contract expanded; the coverage-matrix-view inherits StatusLamp color transitions (using `--motion-micro` 150ms ease-out from the lamp component), and the motion-token inventory is now complete in the root `:root` block.