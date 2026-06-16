# Scope — Design-token + typography bundle

**Marker:** `2026-06-15-design-token-typography-bundle`
**Version:** conductor-0.1.0 · **Epoch:** 1 — Foundation (first frontend/UI chunk)
**Working-route entry:** _Design-token + typography bundle — Tailwind v4.1 @theme tokens, JetBrains Mono + IBM Plex Sans_

## What it builds
The frontend design-system foundation for Conductor's optional Tauri 2 + React 19 desktop control
panel: the **Tailwind CSS v4.1 `@theme` design-token layer** (the `design-tokens-bundle-init` bundle
from `design-system.md` §Surface: desktop-webview) plus the **self-hosted typography stack**
(JetBrains Mono status tier + IBM Plex Sans UI/prose, vendored WOFF2 via Fontsource). This is
deliberately the first frontend chunk (Epoch 1 — Foundation): land the tokens early so every later
UI chunk (the Epoch 9 desktop control panel) binds a single stable, audited token source rather than
reinventing palette / type / spacing.

Concretely:
- A **Tailwind v4.1 build via `@tailwindcss/vite`** (Oxide engine, static zero-runtime offline
  stylesheet) introduced under `crates/conductor-tauri/`.
- The **`@theme` CSS custom-property block** carried verbatim from `design-system.md` §Tokens:
  surfaces (`--color-base`/`-raised-1..3`/`-inset`), the functional status tier
  (`--count-nominal`/`-hold`/`-blocked`, `--status-fail`/`-manual`/`-residual`), the reserved mono
  **ID-cyan** tier + `--color-focus`, text hierarchy, border progression, the **4px-base** spacing
  scale, the radius scale, and motion tokens (`--motion-micro` / `--ease-quiet`).
- **Auto mode** — `@media (prefers-color-scheme: light)` overrides carrying the light-variant values
  verbatim (dark is the default block).
- **`@media (prefers-reduced-motion: reduce)`** global transition/animation reset (binds a11y SC 2.3.3).
- **Self-hosted fonts** — `@fontsource/jetbrains-mono` + `@fontsource/ibm-plex-sans` vendored into the
  bundle (no Google Fonts / CDN at runtime — Minimal-tier offline hardening); fallback stacks
  `"JetBrains Mono", ui-monospace, monospace` and `"IBM Plex Sans", ui-sans-serif, sans-serif`.
- The **typography role scale** (Display / Heading / Body / Label / Code / Data — sizes, weights,
  tracking from `design-system.md` §Typography) expressible from `--font-mono` / `--font-sans`;
  ligatures OFF for ID legibility; `tabular-nums` for the Data tier.

## Boundaries (deferred — NOT this chunk)
- **No Tauri window / frameless config** (`decorations:false`, drag-region, deny-by-default
  capabilities) — Epoch 9 "Frameless window shell".
- **No React components / shadcn/ui / Radix / Lucide / status-lamp components** — Epoch 9 component chunks.
- **No Tauri IPC commands / live-counter `Channel`** — Epoch 9.
- **No CLI surface** — the cli expresses brand via ANSI / `owo-colors`, not webview typography
  (`design-system.md` §Surface: cli). This chunk is **desktop-webview only**.
- **No a11y verification harness** (axe / Lighthouse / contrast runner) — Epoch 9 "Desktop a11y harness".
- Does **not** alter the Rust workspace build/release path: the Tauri bundle stays convenience-only;
  the headless CLI remains the release gate.

## Surfaces / contracts touched
- `design-system.md` §Color Palette / §Typography / §Surface: desktop-webview §Tokens — the
  authoritative token + type source this chunk makes literal (token names MUST match exactly so
  Epoch 9 binds `var(--…)` without renaming).
- `crates/conductor-tauri/` — the frontend asset layer is introduced here (today only `Cargo.toml`
  + `src/main.rs` exist).
- `security-plan.md` — Minimal-tier **offline hardening** (no runtime CDN); supply chain (new
  npm / Fontsource deps must be pinned + lockfiled).
- Stack: Tailwind v4.1 + `@tailwindcss/vite` (Oxide); React 19.x / Vite SPA per design-system Toolkit
  (extent of the JS/React scaffold is the open question below).
- Path-scoped rules `frontend.md` + `a11y.md` begin applying once `crates/conductor-tauri/**` files land.

## Open scope question — RESOLVED at P4 (user, AskUserQuestion ×2)
How much frontend scaffold does this chunk stand up to make the tokens **buildable + testable**?
- (a) Tokens + fonts + Tailwind v4.1 via Vite — a CSS-first build that compiles the stylesheet, **no
  React app yet** (lightest; matches the "bundle" framing + Epoch-1-Foundation placement).
- (b) Full Vite + React 19 SPA scaffold (package.json / vite.config / tsconfig / entry) so the token
  stylesheet renders in a real app shell now.
- (c) Tokens + fonts as static assets + Tailwind config only; Vite/React entirely deferred to Epoch 9.

**Resolved → (b) Full Vite + React 19 SPA scaffold** under `crates/conductor-tauri/ui/`, package manager
**npm** (committed `package-lock.json`, `npm audit` gate). The token stylesheet renders now in a minimal React
19 `App.tsx` **token smoke view** (raw `var(--…)` elements — NOT shadcn/Radix/Lucide/IPC); the Tauri frameless
window + real components remain deferred to Epoch 9's window-shell chunk. (Scope's earlier lean was (a); the
user chose the more cohesive (b) — recorded here as the val-1 anchor.)

## Acceptance intent (val-1 north star)
A Tailwind v4.1 build under `crates/conductor-tauri` compiles the `@theme` token block to a static
stylesheet with **zero runtime font fetches**; both font families resolve from vendored WOFF2;
light-mode + reduced-motion media overrides are present; the emitted token names match
`design-system.md` exactly. No Tauri window or React app behavior is in scope.
