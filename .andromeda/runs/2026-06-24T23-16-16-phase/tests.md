# tests extract

## Relevance
Partial — the chunk touches the Tauri UI (`Titlebar.tsx` / `.css`) with visual state rendering (heartbeat animation, freeze/tint behavior) but adds NO test-harness surface, NO new commands, NO database schema, NO critical path changes. The test strategy does NOT escalate; only existing E2E + a11y verification surfaces are touched.

## Constraints
- per test-plan §3: "the GUI is a thin shell over the same core commands" — no new harness discipline required; no new 5-command bindings; the existing cli release-gate path remains canonical.
- per test-plan §4 (unit coverage): "conductor-tauri/ui (frontend SPA): no Rust/nextest unit tests — the React 19 + Tailwind token bundle is **build-gated** (`tsc --noEmit` + `vite build` + `npm audit`)" — the Titlebar state model is a TS/React local component, not a Rust test surface.
- per test-plan §6 (E2E webview): "tauri-driver (`wdio run` headless under `xvfb`)" with "role/text/`aria-live` selectors ONLY (no visual-pixel / Percy)" and selector safety via the Color-Only a11y rule (text + glyph pairing).
- per test-plan §4 amendment 2026-06-15-design-token-typography-bundle: "frontend (ui/) tests build-gated ... frontend coverage (webview E2E via tauri-driver) is deferred to Epoch 9" — this chunk adds HTML/CSS/state, but the webview-automation harness arrives in the closing Epoch-9 chunks.
- per test-plan §11 anti-patterns: "NEVER use Percy / Chromatic / Applitools (visual regression with human review)"; motion/animation assertions live in a11y / E2E, not Percy.

## Patterns to follow
- per test-plan §3 Log format + §1 artifact-sanitization: all status labels (`[PASS]`, `[FAIL]`, `[HOLD]`, `[BLOCKED]`) are text-paired (never color-alone) and render consistently across cli + webview surfaces; the Run-report envelope shares verdict/state fields across both (parity gate, §6 Critical Path 7).
- per test-plan §6 E2E selector strategy: state labels keyed to brand anchors (`[PASS]`/`[HOLD]`/`[BLOCKED]` text literals + `aria-live="assertive"` on status flips); expect the titlebar count to render via `aria-live` when state transitions to `hold`.
- per test-plan §11 E2E: "NEVER use sleep(N) for synchronization — wait for an explicit signal" — the freeze/tint/resume is asserted by element role/`aria-live`/CSS-class binding (e.g. `--count-hold`), not by timing assumptions.

## Anti-patterns to avoid
- per test-plan §11 Unit: "NEVER test implementation details (private fns / internal struct fields)" — the TS run-state model is internal to the Titlebar component; assert rendered output (text flip + `aria-live` + CSS class), not React state hooks.
- per test-plan §11 E2E: "NEVER use brittle selectors that drift across iterations (hashed Tailwind/CSS class names)" — Titlebar classes must be explicit (e.g. `.titlebar--hold`) or data-testid-backed, never hash-dependent.
- per test-plan §11 Mocking: the UI state driver is local React state only; do NOT mock or spy on the backend Channel/Tauri command layer this chunk doesn't touch.

## Contract bindings
- **obs ↔ tests harness (log-format):** per test-plan §3 amendment 2026-06-24-frameless-window-shell, the Tauri backend self-obs sink (`logs/conductor-tauri.jsonl`) is live; no test harness change, but coordinate any state field name with obs-plan §3 if added (obs owns the sink schema; tests own the envelope).
- **a11y ↔ tests harness:** per test-plan §3 bootstrap, the A11y CI gate binds to a11y §Bootstrap — the Titlebar's `aria-live="assertive"` HOLD flip + text-only status signals are a11y test responsibilities (axe / tauri-driver, closing Epoch-9 chunks); this chunk implements the HTML+ARIA markup, the a11y-plan owns the gate.

## Acceptance criteria contributions
- **(ui build gate)** `ui/` `tsc --noEmit` (strict, no `any`/`as`) + `vite build` → `ui/dist` compiles; `npm audit` clean (no new deps expected; `package-lock.json` committed only if a dep changed). (test-plan §4)
- **(both-surface parity, §6 Critical Path 7)** The Titlebar renders the run-states identically in concept to the cli operator-pause spinner; the webview render is verified by a future E2E role/text/`aria-live` selector (closing Epoch-9) — this chunk satisfies the parity constraint by ensuring the HTML/CSS state renderings exist.
- **(a11y compliance — deferred)** non-color-alone signalling (text + `aria-live`), reduced-motion drop, token-bound colors; the actual axe + tauri-driver assertions are closing Epoch-9 chunks. (test-plan §4 amendment)
- **(no Rust test surface)** `cargo nextest` / `clippy` `-D warnings` / `doctest` remain green (UI-only change; `conductor-tauri` Rust layer untouched). (test-plan §3/§4)

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** (§4 Unit Test Strategy): frontend (ui/) tests build-gated only; frontend coverage via tauri-driver E2E deferred to Epoch 9. Titlebar is part of this convenience-only webview; no Rust unit tests added this chunk.
- **2026-06-24-frameless-window-shell** (§3 Test Harness Contract): registered `logs/conductor-tauri.jsonl` as the Tauri backend self-obs sink. Titlebar state transitions may appear in this sink; coordinate obs-plan §3 field naming if needed, but no test-plan harness change.
