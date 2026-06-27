# Report — 2026-06-26-component-primitives-library

**Chunk:** Component primitives library — six status-lamp variants + reusable Radix dialog scaffold + operator-checklist primitive; presentational React 19 under `ui/src/components`, design-token-driven, UI-only zero engine/seam change (conductor-tauri).
**Date:** 2026-06-26T23:47Z
**Commits:** (uncommitted at report time — wrap commits) `feat(2026-06-26-component-primitives-library)`.

## Changes (structured — detectors read this)
- **Files:**
  - NEW: `crates/conductor-tauri/ui/src/lamp.ts` · `src/components/StatusLamp.{tsx,css}` · `src/components/OperatorPauseDialog.{tsx,css}` · `src/components/OperatorChecklist.{tsx,css}` · `src/Gallery.tsx`
  - MOD: `crates/conductor-tauri/ui/src/main.tsx` · `package.json` · `package-lock.json`
- **Symbols / APIs:** TS frontend only (no Rust). Exports: `Lamp` union + `LAMP_META` + `LampMeta` + `LAMP_ORDER` (`lamp.ts`, mirrors `conductor-core/src/lamp.rs`); default `StatusLamp` (`{lamp, size?}`); default `OperatorPauseDialog` (`{open,onOpenChange,title,body,proceedLabel?,abortLabel?,onProceed,onAbort,allowNoGo?}`); default `OperatorChecklist` + `ChecklistItem` type (`{items,onToggle}`); default `Gallery` (DEV-only). **No Rust symbols, no `#[tauri::command]`, no IPC method, no port/socket, no env var.**
- **Crates / modules:** none added/removed/changed. (The `ui/` subtree is not a Cargo member; the 9-crate workspace is untouched.)
- **Dependencies:** ADDED npm runtime `@radix-ui/react-alert-dialog ^1.1.17` (`ui/package.json` + `package-lock.json`). Its transitive Radix deps were already present via `cmdk` (only 1 package added). No cargo deps added/bumped.
- **Schema / config:** none (no migration, no config key, no violation schema, no `runs.db`/journal change).
- **Coverage of new surfaces:**
  - `StatusLamp` (UI element — six-variant status lamp) → validation n/a · instrumentation n/a · PII n/a · tests build-gated (tsc+vite; GUI unit-test deferred ch9) · a11y ✓ (never-color-alone: token color + glyph + visible text label = accessible name) · tokens ✓ (`var(--count-*/--status-*)`)
  - `OperatorPauseDialog` (UI element — Radix AlertDialog scaffold) → validation n/a · instrumentation n/a · PII n/a · tests build-gated (GUI deferred ch9) · a11y ✓ (Radix `alertdialog` role + aria-modal + focus-trap + Escape + focus-restore; `--color-focus` ring; reduced-motion global) · tokens ✓ (`var(--color-raised-3/--border-subtle/--radius-lg/--motion-micro/--ease-quiet)`)
  - `OperatorChecklist` (UI element — native-checkbox rows) → validation n/a · instrumentation n/a · PII n/a · tests build-gated (GUI deferred ch9) · a11y ✓ (native checkbox implicit `aria-checked` + Space-toggle + visible focus ring) · tokens ✓ (`var(--color-raised-1/--status-manual/--color-focus)`)
  - `Gallery` (DEV-only render-all surface — not shipped to prod) → validation n/a · instrumentation n/a · PII n/a · tests visual-smoke (headless-carried) · a11y n/a (dev affordance) · tokens ✓
  - npm dep `@radix-ui/react-alert-dialog` → supply-chain: `npm audit` ✓ 0 vulnerabilities · `package-lock.json` committed ✓ · no `shell-open` / no remote iframe / no new Tauri capability (app commands unchanged)

## Deviations from intent
1. **Lamp color via inline `var(--token)`** (sourced from `LAMP_META.token`) instead of the plan's per-variant CSS classes — keeps the token→state map single-sourced in `lamp.ts` (the chunk's "single TS lamp-truth" goal), avoiding a CSS duplicate that could drift from `lamp.rs`; matches `App.tsx`'s inline-var precedent. Same six-variant outcome. JUSTIFIED.
2. **Dialog fade uses `--motion-micro` (150ms)**, not the design-system §Motion-stated **200ms** — no 200ms token exists in `tokens.css`; bound the only motion token per the never-hardcode rule (the P2 design distiller itself mapped dialog-fade → `--motion-micro`). 50ms diff immaterial. → **flag for the design distiller:** reconcile design-system §Motion's "200ms dialog fade" to `--motion-micro`, or add a `--motion-dialog: 200ms` token. Implement authored nothing.
3. **Checklist is binary** (`checked: boolean`), no `indeterminate` — the plan's gallery prose mentioned it, but a yes/no operator observation is binary (indeterminate = a ch7 roll-up concern); dropped per "no abstraction beyond need." `aria-checked` + Space-toggle criteria met. JUSTIFIED.
4. **Overlay scrim** = `var(--color-inset)` + `opacity: 0.65` (no scrim token exists; avoided `color-mix` to minimize build risk). Token-driven + semi-transparent + universally supported. JUSTIFIED.

## Decisions & corrections
- **P4 AskUserQuestion:** dialog scaffold dep = `@radix-ui/react-alert-dialog` (alertdialog semantics for the ch8 go/no-go; user selected the recommended option).
- **P5 review:** plan approved (Apply).
- **Gallery mechanism** (decided, not asked): DEV-gated `#gallery` hash-mount in `main.tsx`, re-using `vite-env.d.ts` — resolves the prior "vite-env.d.ts now unused" follow-up.
- **Component-test tooling:** confirmed deferred to the ch9 a11y harness (route decision + the GUI-test-deferral playbook rule); verified now via `tsc` + `vite build` + the gallery.
- **Design-system note** (for the design distiller): dialog-fade duration — spec §Motion says 200ms, token reality is `--motion-micro` 150ms (deviation #2).

## Outcome
- **Acceptance criteria: all met.** Lamps byte-match `conductor-core/src/lamp.rs` (`Pass`/`Fail`/`HOLD`/`Manual`/`Residual`/`Blocked` + `[PASS]`…`[BLOCKED]`); never-color-alone (label + glyph); dialog = Radix AlertDialog (focus-trap/Escape/restore); checklist `aria-checked` + Space-toggle + `--status-manual`; colors/space/radius/motion all token-bound; reduced-motion global; `npm audit` 0.
- **Gates green (1 iteration, no fixes):** `npx tsc --noEmit` (0 errors) · `npm run build` (tsc + vite, exit 0, `ui/dist` produced) · `npm audit` (0 vulnerabilities) · `bash scripts/agent-run.sh run` (exit 0 — workspace nextest 402 [unchanged] + doctests + clippy `-D` + `ensure_frontend`; `conductor-tauri` compiles against the fresh `ui/dist`).
- **Smoke (boot-path changed):** release-gate smoke ✓ (`agent-run.sh run` — the real `conductor-tauri`-compiles-against-bundle proof). **Visual** gallery smoke (`npm run dev` → `#gallery`) carried headless (no display) — same posture as Epoch-9 ch1–ch4.
