# Codebase Research — 2026-06-24-paused-count-hold-point-signature

## Scope
- **Depth:** moderate (small UI footprint) · **Reads:** 6 (Titlebar.tsx, Titlebar.css, tokens.css, App.tsx, ui/package.json, conductor-tauri/src/main.rs) · **Globs/Greps:** 1 (ui/src tree)
- **Code-graph:** NOT queried — the graph is a Rust SCIP index (rust-analyzer) and does not cover `ui/src/` TypeScript; this chunk is frontend-only. The sole Rust file in `conductor-tauri` (`src/main.rs`) is a thin shell, unchanged. (cold-start-analogue: derive from arch §Frontend + the P2 extracts.)

## Files inspected
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` (full) — **the signature surface.** `<header class="titlebar" data-tauri-drag-region>` with `.titlebar__label` (`Conductor · idle`), `.titlebar__count` (static `00:00:00`, `type-data`), `.titlebar__controls` (min/close `<button>`s, `aria-label`). No props, no state today. This is where run-state rendering + heartbeat/freeze/tint land.
- `crates/conductor-tauri/ui/src/components/Titlebar.css` (full) — `.titlebar__count { margin-inline-start:auto; color: var(--count-nominal); }`; no animation yet. Heartbeat `@keyframes` + per-state classes (`--live`/`--hold`/`--aborted`) get added here, binding `--count-hold` / `--count-blocked` / `--motion-micro` / `--ease-quiet`.
- `crates/conductor-tauri/ui/src/styles/tokens.css` (full) — confirms `--count-nominal` #7EE787 · `--count-hold` #E3B341 · `--count-blocked` #565F89 · `--motion-micro` 150ms · `--ease-quiet`. **L78–83: global `@media (prefers-reduced-motion: reduce){ *{ animation:none!important; transition:none!important } }`** — so any heartbeat keyframe / tint transition is dropped under reduced-motion automatically; the freeze/tint/dim END STATES (static color + text) must carry the signal on their own.
- `crates/conductor-tauri/ui/src/App.tsx` (full) — composition root + a **token-showcase gallery** (STATUS_TIERS incl. `{label:'HOLD', token:'--count-hold'}`, TYPE_TIERS, SURFACES). Renders `<Titlebar />` (no props) + a `<main>` with a SEPARATE `00:00:00 · step 0` `type-display` showcase element. **The App-body `00:00:00` is a GALLERY item, NOT the signature** — the signature is the TITLEBAR count only (design-system §Signature). If run-state is lifted here, App gains `useState<RunState>` + the dev driver + passes a prop to `<Titlebar>`.
- `crates/conductor-tauri/ui/package.json` (full) — React ^19, react-dom ^19, @tauri-apps/api ^2, @fontsource/*; devDeps tailwind ^4.1, vite ^8.0.16, typescript ^5.6. **No framer-motion / animation library** → the CSS-only mandate adds no dep (`npm audit` + `package-lock.json` untouched). `build` = `tsc --noEmit && vite build`. **No component test runner (no vitest/jest)** — "exercise the states" this chunk = the running-app dev driver + manual/visual + the future tauri-driver E2E, NOT a unit test.
- `crates/conductor-tauri/src/main.rs` (full) — thin Tauri shell: `init_observability("conductor-tauri", None, obs_sink())` + `tauri::Builder::default().run(generate_context!())`. No `#[tauri::command]`, no `Channel`, no state passed to the webview. **Confirms zero Rust change** (run-state is webview-local until ch4).

## Graph impact
- Cold-start-analogue (TS surface, Rust-only graph): no Rust symbols in scope; `conductor-tauri/src/main.rs` unchanged → no callers/edges affected.

## Patterns detected
- **Token-by-name binding** (Titlebar.css:9,21; App.tsx:36,55): every color/space/motion value is `var(--token)`, zero raw hex/px/ms. The heartbeat must follow — `var(--count-hold)`, `var(--motion-micro)`, `var(--ease-quiet)`.
- **Global reduced-motion kill-switch** (tokens.css:78–83): one `*{animation/transition:none}` block already satisfies reduced-motion; per-component re-declaration is unnecessary (the a11y extract confirms "no re-declaration needed unless a specific transition needs an explicit drop").
- **Semantic `<button>` + `aria-label`; `banner` landmark** (Titlebar.tsx:6,17,24): a11y-first markup already in place; the HOLD text flip + `aria-live="assertive"` extends it.
- **Showcase-gallery App** (App.tsx): App is currently a token gallery, not a wired console; the run console / report views are later Epoch-9 chunks. Keep the signature scoped to the titlebar.

## Conventions to follow
- CSS-only motion, no animation library (package.json confirms none; frontend.md hard ban).
- Tokens by name (Titlebar.css / App.tsx precedent) — no raw hex/px/ms.
- Strict TS, no `any`/`as` (global CLAUDE.md; build runs `tsc --noEmit`).
- Explicit, non-hashed class names for E2E selector stability (testing.md / test-plan §11) — e.g. `.titlebar__count--hold`.
- Not-color-alone + `aria-live="assertive"` HOLD flip; abort motionless (a11y).

## New files to create
- (none expected) — possibly a tiny `ui/src/runState.ts` for the `RunState` union type if not co-located in Titlebar/App (P4 decision).

## Files to modify
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` — accept `runState` (+ count); render per-state class + label/phase-line text flip + `aria-live="assertive"`.
- `crates/conductor-tauri/ui/src/components/Titlebar.css` — heartbeat `@keyframes` + `.titlebar__count--{live,hold,aborted}` classes (tints + freeze) bound by token name.
- `crates/conductor-tauri/ui/src/App.tsx` — IF state lifted to App (decision A): `useState<RunState>` + dev-only driver + pass prop. If state self-contained in Titlebar (B), App is untouched.

## Open questions
- **Run-state driver shape** (P4 AskUserQuestion): state-in-App + dev cycler (A, recommended) vs state-in-Titlebar (B) vs prop-only-no-driver (C). Drives whether `App.tsx` is touched and how the 4 states are demonstrated with no live Channel.
