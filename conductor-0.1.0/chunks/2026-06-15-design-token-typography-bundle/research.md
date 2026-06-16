# Codebase Research — 2026-06-15-design-token-typography-bundle

## Scope
- **Depth:** moderate (frontend cold-start: the Rust workspace is mature at 6 chunks, but the `conductor-tauri` frontend asset layer is greenfield — zero JS/CSS/config files exist) · **Reads:** 7 · **Globs/Greps:** 1

## Files inspected
- `crates/conductor-tauri/Cargo.toml` (full) — placeholder bin; **only** `conductor-core.workspace = true`. `tauri` is NOT yet a dependency of this crate (the Rust-side Tauri wiring is also deferred to Epoch 9). Inherits `version/edition/rust-version/publish` from workspace.
- `crates/conductor-tauri/src/main.rs` (full) — 6-line placeholder: `fn main()` calls `conductor_core::init_observability("conductor-tauri", None)`. Comment explicitly says "full Tauri wiring lands in Epoch 9". This chunk does **not** touch it.
- `Cargo.toml` (full) — workspace manifest; `resolver = "3"`, edition 2024, rust-version 1.94.1, `publish = false`. `[workspace.dependencies]` pins `tauri = "2.10.3"` (line 56, "Epoch 9; unused until then"). No npm/JS tooling here (Cargo only — as expected).
- `rust-toolchain.toml` (full) — channel `1.95.0` + clippy/rustfmt.
- `.claude/docs/stack.md` (full) — §Frontend (lines 29-31) is the canonical pin set: **React 19.x (Vite SPA; Preact 10.x fallback) · Tailwind CSS v4.1 (`@theme`, Oxide) · shadcn/ui (Radix) · Lucide React · Tauri 2 ≥2.10.3 frameless · JetBrains Mono + IBM Plex Sans (self-hosted WOFF2 via Fontsource)**.
- `.andromeda/context/dependency-tree.md` (full) — `conductor-tauri` depends only on `conductor-core`; no frontend graph exists yet.
- `.andromeda/context/api-surface.md` (full) — `conductor-tauri` is a bin (no public API); no IPC methods/event topics yet. Confirms the GUI surface is unbuilt.
- `.gitignore` (read in new-session) — has `/target/`, `/runs/`, `.env*`, `.claude/backup/`, `.claude/settings.local.json`. **No `node_modules/` entry and no frontend build-output (`dist/`) entry** — this chunk is the first to introduce them.

## Patterns detected
- **Placeholder-bin convention** (`crates/conductor-tauri/src/main.rs:1-5`): seam/bin crates ship a minimal `main` that calls `conductor_core::init_observability(...)` and defer real wiring to their owning epoch — mirrors the lean-scope split used across Epoch 1.
- **Shared-dep pinning in `[workspace.dependencies]`** (`Cargo.toml:20-56`): all versions are pinned centrally and inherited via `.workspace = true`; `tauri 2.10.3` is already parked here unused (line 56) — the same "pin now, wire later" discipline this chunk applies to the npm/Fontsource side.
- **Glob for frontend files returned empty** — confirms cold-start: no `package.json`, `vite.config.*`, `tauri.conf.json`, `index.html`, `tsconfig*.json`, or `*.css` anywhere in the tree.

## Conventions to follow
- **Frontend assets live under `crates/conductor-tauri/`** (arch §Inherited Defaults workspace boundary; arch extract). Rust `src/` is occupied by `main.rs`, so JS/CSS assets need a sibling subdir (NOT inside `src/`) — proposed `crates/conductor-tauri/ui/` (resolve at P4).
- **Token names verbatim from `design-system.md` §Tokens** — the literal `@theme` block (design-system.md lines 196-241) carried exactly; Epoch 9 binds `var(--…)` by these names (design + a11y + arch all assert this as the binding contract).
- **TypeScript strict + ES modules** for any config/source (global CLAUDE.md: no `any`, no `as` casts unless justified; ESM `import`/`export`; `vite.config.ts` in TS).
- **Lockfile committed** (`package-lock.json` / equivalent) — security extract constraint #2; mirrors the committed-`Cargo.lock` supply-chain discipline.
- **Zero runtime CDN** — fonts vendored WOFF2 via `@fontsource/*`, no Google Fonts / `@import url(...)` (security + design + a11y all assert; Minimal-tier offline hardening).

## New files to create (exact set depends on the P4 scaffold-extent decision)
- `crates/conductor-tauri/ui/package.json` — npm manifest pinning `tailwindcss@^4.1`, `@tailwindcss/vite`, `vite`, `@fontsource/jetbrains-mono`, `@fontsource/ibm-plex-sans` (+ `react@^19` / `react-dom@^19` / `typescript` only if the full-SPA option is chosen).
- `crates/conductor-tauri/ui/package-lock.json` — committed lockfile (deterministic install + auditability).
- `crates/conductor-tauri/ui/vite.config.ts` — Vite + `@tailwindcss/vite` plugin (Oxide), build emits a static stylesheet.
- `crates/conductor-tauri/ui/src/styles/tokens.css` (name TBD) — `@import "tailwindcss";` + the `@theme` block + `@media (prefers-color-scheme: light)` + `@media (prefers-reduced-motion: reduce)` + the Fontsource `@import`s + the typography role utilities (ligatures-off mono, `tabular-nums` Data tier).
- `crates/conductor-tauri/ui/index.html` (+ a minimal `main.ts`/`main.tsx` entry) — Vite needs an entry to produce the build; extent depends on CSS-only vs full-SPA.
- `crates/conductor-tauri/ui/tsconfig.json` — only if TS source/entry is included.

## Files to modify
- `.gitignore` — add `node_modules/` and the frontend build-output dir (e.g. `crates/conductor-tauri/ui/dist/`); neither is currently ignored. (No Rust file changes: `Cargo.toml`/`main.rs` stay as-is — Tauri Rust wiring is Epoch 9.)

## Open questions
- **(P4 — AskUserQuestion) Scaffold extent:** (a) Tailwind v4.1 + Fontsource + Vite, CSS-first build, **no React app** (lightest; matches "bundle" + Foundation placement); (b) full Vite + React 19 SPA scaffold now (entry + App) so the token sheet renders in a real shell; (c) tokens+fonts as static assets + Tailwind config only, Vite/React deferred to Epoch 9. Scope leans (a).
- **(P4) Frontend directory + package manager:** propose `crates/conductor-tauri/ui/` and **npm** (`package-lock.json`, matches the security extract's wording) — confirm, or prefer pnpm/yarn / a different dir (e.g. `frontend/`).
- **(deferred — Epoch 9, not this chunk)** Tauri `frontendDist` wiring to the built stylesheet, `tauri.conf.json`, and adding `tauri` to `conductor-tauri/Cargo.toml` — noted so the dir choice here stays compatible.
