# Codebase Research — 2026-06-26-component-primitives-library

## Scope
- **Depth:** moderate–deep (mature frontend tree) · **Reads:** 10 · **Globs/Greps:** 4
- TypeScript-frontend chunk; modifies **no Rust**. The Rust lamp mapping is read-only reference (the byte-match contract the TS lamps mirror).

## Files inspected
- `crates/conductor-core/src/lamp.rs` (full) — **THE single lamp-truth source.** Module doc: *"One source of lamp truth, reused by the Markdown report, coverage matrix, cli, and **desktop**."* `enum Lamp { Pass, Fail, Hold, Manual, Residual, Blocked }`; `.label()` → `Pass`/`Fail`/`HOLD`/`Manual`/`Residual`/`Blocked`; `.status_prefix()` → `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`; `for_record()` is the **verdict-first projection** (Blocked/KnownResidual state-driven first, then verdict Pass/Fail/CalibrationRegion→Hold, then verdict-less ManualCheck→Manual). The TS lamp variant set + labels MUST match these spellings.
- `crates/conductor-core/src/report_state.rs` (full) — `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` (5-way; serde PascalCase).
- `crates/conductor-core/src/verdict.rs` (full) — `Verdict { Pass, Fail, CalibrationRegion }`; `CalibrationRegion.label()=="HOLD"`; `default_report_state` maps CalibrationRegion→ManualCheck.
- `crates/conductor-cli/src/render.rs` (full) — the cli analogue: `lamp_code(Lamp)`→xterm-256 (Pass 114 / Fail 203 / Hold 179 / Manual 146 / Residual 246 / Blocked 60); reuses `Lamp::for_record` verbatim; "status is never color-alone — ASCII prefix always present, color a tty-gated overlay." Desktop is the CSS-token analogue of this exact mapping.
- `crates/conductor-tauri/ui/src/styles/tokens.css` (full) — **all six lamp tokens already materialized** on `:root`: `--count-nominal #7EE787` (Pass) / `--count-hold #E3B341` (Hold) / `--status-fail #F85149` (Fail) / `--count-blocked #565F89` (Blocked) / `--status-manual #A9B1D6` (Manual) / `--status-residual #9A93A8` (Residual). Also present: `--color-raised-3` (dialog fill), `--border-subtle`, `--color-focus #7DCFFF`, `--border-emphasis`, `--radius-sm/md/lg/full`, `--motion-micro 150ms`, `--ease-quiet`, the `.type-label`/`.type-data` role classes, a light-mode `@media`, and a **global** `@media (prefers-reduced-motion: reduce) { * { animation:none; transition:none } }`. **No new tokens needed.**
- `crates/conductor-tauri/ui/package.json` (full) — runtime deps: `cmdk ^1.1.1`, `react/react-dom ^19`, `@tauri-apps/api ^2`, `@fontsource/*`. **No `@radix-ui/*`, no `lucide-react`.** Build script `tsc --noEmit && vite build`.
- `crates/conductor-tauri/ui/src/components/ScenarioPicker.tsx` (full) — the shadcn-via-primitive precedent: `import { Command } from 'cmdk'` used directly + `./ScenarioPicker.css`; BEM-ish classes + `type-*` role classes; `aria-current` on selection; typed inline props, default-export fn.
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` + `Titlebar.css` (full) — the component + CSS convention: per-component `.css` import; tokens by name; state-variant classes (`titlebar__count--${runState}`); `:focus-visible { outline: 2px solid var(--color-focus); outline-offset: -2px }`; `transition: color var(--motion-micro) var(--ease-quiet)`; `aria-live` on the status label; reduced-motion honored globally.
- `crates/conductor-tauri/ui/src/App.tsx` (full) — **DEV cycler already retired** (ch4); App drives the real `Channel<RunEvent>`. Single-window layout (no router); `var(--…)` inline styles for layout; `role="alert"` on error prose. Mount point is `main.tsx`.

## Graph impact
- **Cold-start equivalent for this chunk:** the code-graph (`tree.db`) indexes **Rust via SCIP/rust-analyzer**; the TS frontend tree is outside it. This chunk modifies **zero Rust symbols** — `conductor_core::Lamp`/`Verdict`/`ReportState` are consumed only as a read-only naming/label contract (no Rust caller-impact). So no `code-graph.py query` was run (it would return nothing for `ui/src/*.tsx`); the relevant "impact" is the lamp-spelling contract above.

## Patterns detected
- **shadcn-via-primitive** (`ScenarioPicker.tsx:1`): use the underlying Radix/cmdk primitive directly + per-component `.css` binding `var(--…)`; do NOT vendor shadcn-cli Tailwind-utility files (no `@/` alias; tokens-on-`:root`). The dialog scaffold follows this with the Radix AlertDialog primitive.
- **Per-component CSS + token-by-name** (`Titlebar.css:19-23`, `ScenarioPicker.css`): `.block__el--variant` classes; colors/space/radius/motion all `var(--token)`; never raw hex/px.
- **Focus ring** (`Titlebar.css:90-93`): `:focus-visible { outline: 2px solid var(--color-focus); outline-offset: -2px }` — the visible-focus pattern for all interactive controls.
- **State-variant class** (`Titlebar.tsx:30` + `Titlebar.css:37-54`): `className={\`base base--${state}\`}` + one CSS rule per variant binding its token — the exact shape the six-variant lamp reuses.
- **Verdict-first lamp projection** (`lamp.rs:37-48`): the `(state, verdict)` → `Lamp` logic lives in Rust; the report/coverage VIEWS (ch6/ch7) will project records — this chunk's lamp primitive takes a `Lamp` value directly and renders label+glyph+color.

## Conventions to follow
- **Lamp spellings are the contract** (`lamp.rs:64-73`): labels `Pass`/`Fail`/`HOLD`/`Manual`/`Residual`/`Blocked`; ASCII anchors `[PASS]`…`[BLOCKED]` (testing.md E2E selectors key off these). Mirror them in a single TS module, never re-spell.
- **Never color-alone** (a11y.md / lamp.rs:50): each lamp pairs token color + text label + glyph.
- **Tokens by name only** (frontend.md): no hardcoded hex/px; reduced-motion is already global (tokens.css) — components add no per-file reduced-motion handling but MAY use `motion-reduce:` for specifics.
- **Semantic HTML + Radix, never hand-roll** (a11y.md): AlertDialog for the dialog (role=alertdialog, aria-modal, focus-trap, Escape, restore-focus = Radix defaults); native `<button>`/checkbox for actions/checklist rows.
- **npm-audit gate** (security.md): any new dep → `npm audit` 0 + `package-lock.json` committed.
- **Build-order** (frontend.md): `npm run build` before any `cargo build` of `conductor-tauri` (CI + agent-run `ensure_frontend`).

## New files to create
- `crates/conductor-tauri/ui/src/lamp.ts` — `type Lamp = 'Pass'|'Fail'|'Hold'|'Manual'|'Residual'|'Blocked'` + `LAMP_META: Record<Lamp, { label; prefix; token; glyph }>` mirroring `lamp.rs` (single TS lamp-truth).
- `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` + `StatusLamp.css` — the six-variant lamp (color token + glyph + label; `role="status"`/accessible name).
- `crates/conductor-tauri/ui/src/components/OperatorPauseDialog.tsx` (or `Dialog.tsx`) + `.css` — the Radix-AlertDialog scaffold (title/body/action slots; focus-trap/Escape/restore via Radix).
- `crates/conductor-tauri/ui/src/components/OperatorChecklist.tsx` + `.css` — checklist rows (native checkbox, `aria-checked`, Space-toggle, card rows).
- `crates/conductor-tauri/ui/src/Gallery.tsx` — DEV-only render-all gallery (all six lamps + dialog open/close + checklist states).

## Files to modify
- `crates/conductor-tauri/ui/package.json` + `package-lock.json` — add the Radix dialog dep (P4 decision; npm-audit must stay 0).
- `crates/conductor-tauri/ui/src/main.tsx` — DEV-gated gallery mount (`import.meta.env.DEV` + e.g. `#gallery` hash → `<Gallery/>` else `<App/>`), re-using the existing `vite-env.d.ts` (resolves the handoff's "now-unused" follow-up). Production bundle unchanged (DEV-stripped).

## Open questions
- **Dialog scaffold primitive/dep** — `@radix-ui/react-alert-dialog` (the ch8 go/no-go base; a11y rule names AlertDialog) vs generic `@radix-ui/react-dialog` vs both. → **P4 AskUserQuestion** (it's the chunk's only new runtime dep + shapes the scaffold API). Lean: AlertDialog.
- **Gallery mechanism** — DEV-gated hash-mount in `main.tsx` (recommended; stripped from prod) vs a separate vite entry. → decide at P4 (lean: hash-mount).
- **Component-test tooling** — RESOLVED: defer vitest/Testing-Library to the ch9 a11y harness (route decision + tests/a11y distillers + the GUI-test-deferral playbook rule); verify now via `tsc` + `vite build` + the gallery.
