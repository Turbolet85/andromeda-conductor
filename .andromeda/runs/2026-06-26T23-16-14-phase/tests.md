# tests extract

## Relevance
partial — frontend build-gated only (tsc/vite/npm audit); E2E deferred to Epoch 9; not in test-plan §1-§4 Rust/nextest coverage

## Constraints
1. Frontend build-gated discipline per §4 amendment 2026-06-15: `tsc --noEmit` (strict, no `any`), `vite build`, `npm audit` (0 vulnerabilities), `vite preview` render smoke — no Rust/nextest unit tests
2. Status-lamp variants must maintain byte-consistency with report/CLI lamp mapping per chunk scope: verdict-first precedence via `Verdict::default_report_state`
3. Design-token sourcing from §1 stack (Tailwind v4.1 + shadcn/ui): consume established `styles/tokens.css` `:root` tokens; add lamp tokens only if `design-system.md` defines ones not yet present
4. Accessibility constraints at component level per chunk scope (never-color-alone + ARIA/keyboard) — harness verification deferred to Epoch 9 per amendment 2026-06-26

## Patterns to follow
1. Presentational verification via `tsc + vite build + render-all gallery` (chunk scope open question: lean toward deferring JS/TS unit runner to Epoch 9)
2. Design-system-first component design: palette + glyph pinned from `design-system.md` §Color Palette
3. Composable primitive pattern: scaffold reusable components (lamp + dialog + checklist) for later views to compose (precedent: existing `Titlebar`/`RunControls`/`ScenarioPicker`)

## Anti-patterns to avoid
1. No live data wiring, no Tauri `Channel` / `#[tauri::command]` integration, no backend seam crate changes (UI-only)
2. Do not build GUI-integration tests in this chunk; defer to Epoch 9 `tauri-driver` harness per amendment 2026-06-26
3. Do not introduce JavaScript/TypeScript unit test runner (vitest / Testing Library) now; defer per route decision (chunk scope open question)

## Contract bindings
Status-lamp surface ↔ report/CLI lamp semantics (verdict-first precedence, `Verdict::default_report_state` mapping); Design-token binding to `design-system.md` §Color Palette (P2 design distiller source of truth for palette + glyph per state)

## Acceptance criteria contributions
1. (tests) Frontend gates pass: `tsc --noEmit` strict, `vite build` success, `npm audit` 0 vulnerabilities
2. (tests) Render-all gallery: all six lamp variants + dialog states (open/close) + checklist states (checked/unchecked/indeterminate) visually inspectable
3. (tests) Status-lamp verdict mapping byte-consistent with report/CLI lamp semantics per chunk scope
4. (tests) Accessibility primitives: no color-only signals, ARIA-labels + keyboard semantics present (harness verification deferred to ch9)

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** (§4): frontend carry "no Rust/nextest unit tests; build-gated (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke); webview E2E via tauri-driver deferred to Epoch 9"
- **2026-06-26-live-counter-channel-stream** (§5): GUI integration + GUI-parity tests defer to Epoch-9 GUI test-harness chunk; run logic covered at unit tier + CLI parity E2E