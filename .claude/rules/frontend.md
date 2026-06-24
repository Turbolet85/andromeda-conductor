---
paths:
  - "crates/conductor-tauri/**/*.tsx"
  - "crates/conductor-tauri/**/*.jsx"
  - "crates/conductor-tauri/**/*.ts"
  - "crates/conductor-tauri/**/*.css"
  - "crates/conductor-tauri/ui/**"
---

# Frontend Rules

Path-scoped rules for the Tauri 2 desktop-webview UI. The GUI is convenience only — the cli/headless path is the release gate; never let GUI work block or replace the headless contract.

**Authoritative source:** `.andromeda/design-system.md` + `.andromeda/layout-templates.md` (desktop-webview surface). Reference design tokens by NAME; raw values live in design-system.md.

## Framework & structure
- React 19.x (Vite 8.x SPA; Preact 10.x size fallback) + Tailwind CSS v4.1 (Oxide via `@tailwindcss/vite`, static zero-runtime stylesheet) + shadcn/ui (Radix Primitives, locally vendored) + Lucide React icons. Tauri 2 (≥2.10.3) frameless window (`decorations:false`, `data-tauri-drag-region` titlebar).
- **Design tokens live on `:root`** (`ui/src/styles/tokens.css`), NOT Tailwind `@theme` — v4 `@theme` tree-shakes non-namespace tokens (drops `--space-*`/`--motion-*`/`--ease-*`) and forbids `@media` nesting; keep `@import "tailwindcss"` for the engine. Bind every token by name (`var(--…)`) — never hardcode hex/px (design-system.md §Tokens).
- **No router, no breakpoints, no browser nav** — the single frameless window IS the surface; the four "screens" (idle / live / HOLD / report) are one window in different run-states. The operator moves by run-state, not routes.

## Design system (expression 0.3 — functional motion only)
- Ground is cool Tokyo-Night slate `--color-base`; color is functional status (Verdict/ReportState), never decoration or mood. Borders-only depth (`--border-subtle` seams, NO drop shadows / `backdrop-filter`).
- Typography: JetBrains Mono is the reserved STATUS tier (count / P-IDs / run_id / SLO timings / fingerprints, `--color-id-cyan`); IBM Plex Sans for phase line + prose. Self-hosted WOFF2 via Fontsource — no runtime CDN.
- Signature: the **Paused-count hold-point** — the titlebar count freezes at the hold value, tints `--count-nominal`→`--count-hold`, resumes on proceed / dims `--count-blocked` on abort. The absence of motion is the event.
- Coverage matrix is a dense single-row-per-P-ID list (Linear instrument-panel density), NOT a KPI-card grid.

## Live data
- One Tauri `Channel` streams live counters / target status backend→frontend — no polling/SSE/URL surface. Conductor MUST NOT emit native OS toasts (that's Pulse behavior it observes via the operator checklist).
- Always render explicit loading / empty / in-progress states as real prose — never implicit, never a gray skeleton, never downgrade in-progress to `Fail`.

## Security guardrails (honored at the layout level)
- Deny-by-default Tauri capabilities (only start/stop · picker · run-report · operator-pause + the one Channel); no `shell-open` with scenario-derived strings; no remote-origin iframes; Tauri ≥2.10.3.
- No `alert()`/`confirm()`/`prompt()` — use the styled shadcn `AlertDialog`. Style the webview scrollbar; suppress the Chromium context menu / devtools / text-selection on non-text.

## Hard bans (design Anti-Patterns)
- NEVER use generic fonts (Inter / Roboto / Arial / system-ui), purple-gradient-on-white "AI gradient", glassmorphism, KPI-card grids, flashing/pulsing on `Fail`, or a progress bar that hides/animates-to-100% on the operator-pause (it must freeze).
- NEVER convey state by color alone (see `a11y.md`); NEVER add `framer-motion` or any animation library (CSS transitions only at expression 0.3).

## Accessibility
- See `a11y.md` (auto-loads on the same paths) for the full WCAG contract. Semantic HTML first; keyboard-first; visible focus ring; not-color-alone; reduced-motion respected.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-06-24: `conductor-tauri`'s `tauri::generate_context!` resolves `build.frontendDist` (`ui/dist`) at COMPILE time — so the webview bundle MUST be built (`npm run build` in `crates/conductor-tauri/ui`) before ANY workspace `cargo build`/`nextest`/`clippy` that compiles `conductor-tauri`, else the compile fails (`ui/dist` is git-ignored → regenerated per environment). A bare `cargo build` does NOT run `tauri.conf.json`'s `beforeBuildCommand` (only the Tauri CLI does), so the order is wired as an `ensure_frontend` step into `scripts/agent-run.{sh,ps1}` (the `run` verb) + the CI Rust job. Also: gitignore `crates/conductor-tauri/gen/` (tauri-build regenerates the ACL schemas each compile) and keep valid `bundle.icon` files present (PNG + ICO) — `generate_context!` validates them at compile time. (frameless-window-shell chunk)
- 2026-06-24: To demonstrate/exercise a **run-state-driven** webview component before its live data source (the Tauri `Channel`, Epoch-9 ch4) is wired, drive it with a typed local state lifted to `App` + a **DEV-only** cycler gated on `import.meta.env.DEV` (stripped from the production bundle), and pass the state as a PROP so the later `Channel` plugs into the SAME contract — don't bake the data source into the component. Gotcha: any use of `import.meta.env.*` requires `crates/conductor-tauri/ui/src/vite-env.d.ts` (`/// <reference types="vite/client" />`), else strict `tsc --noEmit` fails to typecheck it (`ImportMeta.env` is undefined without the vite/client lib reference). (paused-count-hold-point-signature chunk)
