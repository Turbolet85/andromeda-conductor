# Scope — Component primitives library

**Marker:** `2026-06-26-component-primitives-library`
**Version / Epoch:** conductor-0.1.0 · Epoch 9 (Desktop control panel) · ch5/10
**Working entry:** _Component primitives library — six status-lamp variants + dialog scaffold + operator-checklist primitive_
**Folded annotations:** none (the taken-up working line carries no `PREREQ:`/`CARRY:`).

## Intent (one line)
Build the reusable, presentational React 19 primitives the later Epoch-9 views compose — the **six status-lamp variants**, a **dialog scaffold**, and an **operator-checklist primitive** — design-token-driven and accessible, with no live data wiring.

## What it builds

### 1. Six status-lamp variants
A single reusable status-lamp component (+ its variant set) covering the canonical run-report lamp states the desktop renders. The six are the verdict-first lamp surface mirrored from the Markdown report / CLI `[PASS]/[FAIL]/[HOLD]/[BLOCKED]` mapping, expanded to the full desktop set:
- **Pass** · **Fail** · **Hold** (CalibrationRegion, verdict-first) · **ManualCheck** · **KnownResidual** · **Blocked**.
- Each variant pairs its design-token color with a **text label + glyph** — the universal *status-is-never-color-alone* invariant (color encodes run state, never decoration). The exact palette + glyph per state is pinned from `design-system.md` §Color Palette (the P2 design distiller) and must match the existing report/CLI lamp semantics (`Verdict::default_report_state` mapping; verdict-first precedence so a calibration row is HOLD, not Manual).
- Consumes the established `styles/tokens.css` `:root` tokens (incl. the `--count-hold` / `--count-blocked` family the titlebar already uses); adds lamp tokens only if `design-system.md` defines ones not yet materialized.

### 2. Dialog scaffold
A reusable, accessible modal/dialog primitive (shadcn/Radix Dialog or AlertDialog, the same Radix lineage ch3's picker introduced) — focus-trapped, ESC/overlay dismiss, ARIA-labelled, token-styled. This is the **scaffold** the later **operator-pause go/no-go dialog** (ch8 — AlertDialog gating each committed timeline step) builds on; this chunk ships the primitive + its open/close + title/body/action slots, not the go/no-go wiring or any command binding.

### 3. Operator-checklist primitive
A reusable component for rendering an **operator checklist** — the ManualCheck / DriveObserve path where a human verifies a visual claim and renders a go/no-go. Presentational checklist (item rows with checked/unchecked/indeterminate state + accessible labelling/keyboard semantics); composed later by the **Run-report + operator-checklist views** (ch7). No scenario data, no Channel, no persistence here.

## Boundaries

**In scope:** the three presentational primitives above as reusable components under `crates/conductor-tauri/ui/src/components/` (co-located CSS per the existing `Titlebar`/`RunControls`/`ScenarioPicker` convention, or a `components/ui/` primitives subtree if that better fits shadcn vendoring — a plan decision); a way to exercise **all variants/states** (a render-all gallery or equivalent) so each lamp/dialog/checklist state is visually inspectable; TypeScript strict (no `any`, functional components + hooks); design-token-sourced styling; the *never-color-alone* + ARIA/keyboard accessibility baked into each primitive at the component level.

**Out of scope (deferred — do not build here):**
- Composing these primitives into actual views — Coverage-matrix view (ch6), Run-report + operator-checklist views (ch7), Operator-pause go/no-go dialog wiring (ch8).
- Any live data / Tauri `Channel` / `#[tauri::command]` wiring; no backend or `conductor-*` Rust crate change (UI-only, zero engine/seam model change).
- The desktop **a11y harness + verification** (axe/Lighthouse/contrast/keyboard, tauri-driver) — that is ch9; this chunk bakes accessibility *into* the components but does not stand up the harness, and the deferred `tauri::test` GUI-integration tests remain pinned to ch9.
- New scenarios / P-IDs / seam crates.

## Surfaces & contracts touched
- `crates/conductor-tauri/ui/src/components/` (new primitive components + CSS) — possibly a `components/ui/` subtree.
- `crates/conductor-tauri/ui/src/styles/tokens.css` — read; extend only if a `design-system.md` lamp token is not yet present.
- `crates/conductor-tauri/ui/src/App.tsx` and/or `main.tsx` — only if a render-all gallery needs a mount point (presentational; behind a dev affordance, not the live run surface).
- Status semantics contract: the lamp set must stay byte-consistent with the report/CLI lamp mapping (`ReportState` / `Verdict`, verdict-first lamp precedence) — shared vocabulary, not a re-definition.
- Frontend gates only: `tsc` (strict) + `vite build` + `npm audit` (0). No Rust workspace gate change expected.

## Open questions for the plan (P4)
- **Component-test tooling:** introduce a frontend unit-test runner (e.g. vitest + Testing Library) now, or verify via `tsc` + `vite build` + the render-all gallery and defer all GUI test tooling to the ch9 a11y harness (consistent with the route's established GUI-integration-test deferral)? — lean: defer (route decision), verify presentationally now.
- **Primitive home:** co-located `components/*.tsx` (existing convention) vs a `components/ui/` shadcn-style primitives subtree (cleaner for the dialog + future shadcn vendoring).
- **Lamp glyph source:** confirm `design-system.md` pins the per-state glyph + label (vs. CLI ASCII only) so desktop ManualCheck/KnownResidual get distinct non-color encodings.
