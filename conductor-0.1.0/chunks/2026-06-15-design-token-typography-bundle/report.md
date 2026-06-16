# Report — 2026-06-15-design-token-typography-bundle

**Chunk:** Design-token + typography bundle — Tailwind v4.1 @theme token layer + self-hosted JetBrains Mono / IBM Plex Sans (desktop-webview foundation)
**Date:** 2026-06-15T22:05:00Z
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)
- **Files:**
  - NEW (8): `crates/conductor-tauri/ui/{package.json, package-lock.json, vite.config.ts, tsconfig.json, index.html}` + `crates/conductor-tauri/ui/src/{main.tsx, App.tsx, styles/tokens.css}`
  - MODIFIED (1): `.gitignore` (added `node_modules/` + `crates/conductor-tauri/ui/dist/`)
  - NO Rust files changed (`conductor-tauri/Cargo.toml` + `src/main.rs` untouched; `tauri` NOT added to the crate — Epoch 9).
- **Symbols / APIs:** No Rust public-API change (the 7 seam libs + 2 bins are byte-identical). New frontend = a Vite + React 19 SPA entry (`main.tsx` → `App.tsx`), no exported API/IPC. New design surface = **34 CSS custom-property tokens** on `:root` (the `var(--…)` binding contract for Epoch 9) + 6 typography role classes (`.type-display/-heading/-body/-label/-code/-data`).
- **Crates / modules:** No new Cargo crate; `conductor-tauri` gains a `ui/` frontend asset subtree (npm/Vite, invisible to cargo — not a workspace member).
- **Dependencies (npm — NEW ecosystem; cargo-audit/deny does NOT cover these, `npm audit` does):**
  - deps: `react@^19`, `react-dom@^19`, `@fontsource/jetbrains-mono@^5`, `@fontsource/ibm-plex-sans@^5`
  - devDeps: `tailwindcss@^4.1`, `@tailwindcss/vite@^4.1` (→4.3.1), `@vitejs/plugin-react@^6`, `vite@^8.0.16`, `typescript@^5.6`, `@types/react@^19`, `@types/react-dom@^19`
  - `package-lock.json` committed. `npm audit` → **0 vulnerabilities**. No Rust dep changes (`Cargo.toml`/`Cargo.lock` untouched).
- **Schema / config:** New frontend config (`package.json`, `tsconfig.json` strict, `vite.config.ts`); no `runs.db` schema, no scenario config, no violation schema.
- **Coverage of new surfaces:**
  - `conductor-tauri/ui` design-token + typography bundle (desktop-webview) → validation **n/a** (no external input) · instrumentation **n/a** (static build asset; obs-plan §1 frontend self-obs is console-JSON-only and deferred to Epoch 9; no browser OTLP) · PII **n/a** (no user data) · tests **build-gated** (`vite build` + `tsc --noEmit` + `npm audit` + browser render smoke; no unit tests — frontend test harness is a later chunk, tests-domain returned "No domain coverage") · a11y **partial✓** (`@media prefers-reduced-motion: reduce` reset present; status tiers paired with text labels = not-color-alone; full axe/contrast harness is Epoch 9) · tokens **design-token✓** (34 tokens emit; `App.tsx` binds `var(--…)` for all colors/type — a few demo-swatch px literals are throwaway smoke-view layout, not product UI).

## Deviations from intent
1. **Vite `^6`→`^8.0.16`, `@vitejs/plugin-react` `^4`→`^6`.** The `npm audit` gate flagged 3 high esbuild vulns (GHSA-gv7w-rqvm-qjhr), patched only in Vite 8.0.16+. **No spec pins a Vite major** (architecture.md §Stack + design-system.md + stack.md pin only Tailwind v4.1 + React 19.x), so this is an in-scope dependency fix. `@tailwindcss/vite@4.3.1` declares `vite: ^5.2.0 || ^6 || ^7 || ^8`; node v24 satisfies Vite 8's engine. Audit clean after.
2. **`@theme` → plain `:root` for the token block — SPEC↔REALITY (design detector should weigh).** Plan said carry the `@theme {…}` block verbatim incl. `@media (prefers-color-scheme: light) { @theme {…} }` (mirroring design-system.md §Surface: desktop-webview §Tokens). Tailwind v4 reality: (a) `@theme` is forbidden nested inside `@media`, and (b) `@theme` / `@theme static` **tree-shakes non-namespace tokens** — it emitted only 23/34, dropping all `--space-*`, three `--radius-*`, `--motion-micro`, `--ease-quiet`. To honor the actual contract ("all 34 token names emit for Epoch 9 `var(--…)`"), all tokens are declared on a plain author `:root` (Tailwind doesn't tree-shake author CSS; verified Vite's minifier preserves custom props). **Token names + values are verbatim from design-system.md §Tokens; only the wrapper at-rule changed.** `@import "tailwindcss"` retained so the Tailwind v4.1 engine is still established. → design-system.md §Tokens' illustrative CSS (the `@theme` / nested-`@media @theme` form) does not deliver complete token emission in Tailwind v4; candidate for a §Tokens body amendment to the `:root` form.

Minor (no spec impact): lightningcss normalized durations in the built artifact (`150ms`→`.15s`, equivalent; source verbatim); Fontsource per-weight imports bundle all subsets into `dist/` but the browser fetches only latin at runtime (unicode-range — disk-only cost); a benign `/favicon.ico` 404 in `vite preview` (no favicon in a token bundle — Tauri app icon is Epoch 9).

## Decisions & corrections
- **Scaffold extent (user, AskUserQuestion @ phase P4):** chose **Full Vite + React 19 SPA scaffold** over the scope's lean CSS-first lean (a) — Epoch 9 then only adds the Tauri window + components. Recorded in `scope.md`.
- **Package manager (user, AskUserQuestion @ phase P4):** **npm** — committed `package-lock.json`, `npm audit` gate. Sets the lockfile/audit convention for all future frontend chunks.
- **Frontend location:** `crates/conductor-tauri/ui/` (sibling to the Rust `src/`).
- **Tailwind v4 token-emission learning:** `@theme`/`@theme static` tree-shakes non-namespace tokens; a design-token *bundle* (must expose every token) belongs in a plain `:root`, with `@import "tailwindcss"` kept for the engine. (Candidate learning.)
- **npm-audit-driven major bump:** an `npm audit` advisory with no spec-pinned major is an in-scope dependency bump (Vite 6→8), parallel to the cargo-audit floor discipline. (Candidate learning.)

## Outcome
- **Acceptance criteria: all met.** 34/34 token names emit (verified against `dist/` CSS); light + reduced-motion media present; both fonts resolve from vendored WOFF2 (zero CDN); no host/workspace-path leak in compiled CSS; npm audit clean + lockfile committed; frontend under `crates/conductor-tauri/ui/`, no Rust change; not-color-alone in the smoke view.
- **Gates green:** `npm ci`/`install` ✓ · `npm audit` ✓ 0 vulns · `npm run build` (`tsc --noEmit` + `vite build`) ✓ · `npm run typecheck` ✓ · `cargo nextest run --workspace` ✓ 36/36 · `cargo clippy --workspace --all-targets -- -D warnings` ✓.
- **Smoke (boot-path = new web entry-point):** `vite preview` + chrome-devtools render smoke ✓ — React mounts, both fonts load (5 latin WOFF2 @ 200), all token tiers + status lamps render in dark mode; only the benign favicon 404.
