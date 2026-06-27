# layouts extract

## Relevance
Relevant — builds new desktop-webview surface component (Coverage-matrix view) under Epoch 9.

## Constraints
1. §desktop-webview §Tooling context — React 19, Tailwind v4.1 `@theme` Oxide, shadcn/ui (Radix 1.x), Lucide icons; expression level 0.3 (color transitions only, no animation library); Tauri 2 frameless.
2. §Primary content block 1 (coverage matrix) — dense single-row-per-P-ID list (NOT a KPI-card grid); `color-raised-1` container, `radius-md`, `1px` `border-subtle` row dividers, `space-md` row padding; virtual-scroll for 60 rows.
3. §Primary content block 1 — per-row structure: status-lamp glyph · scenario label · P-ID (`color-id-cyan`) · slo_tier · latency_ms (`color-id-cyan`, right-aligned).
4. §Primary content block 1 — header strip carries matrix counts (`color-id-cyan`) + frozen step-index in `count-hold` while held (signature reinforcement §Signature placement #3).
5. §Primary content block 1 — empty state: "No scenarios loaded" prose (Body, `text-tertiary`), never a gray placeholder.
6. §Primary content block 1 — selected row `border-emphasis` left edge; hover `color-raised-1` lift `motion-micro`; every lamp paired with verdict/state text (never color-only).
7. §IA notes §Global model — frameless window is the surface; operator navigates by run state (no routes, no breadcrumbs); live-channel via Tauri `Channel`; deny-by-default Tauri capabilities honored at layout.

## Patterns to follow
1. §StatusLamp component — glyph + paired text; 6-state lamp (filled dot Pass/CalibrationRegion/Fail · hollow ring Blocked · checkbox ManualCheck · dashed muted KnownResidual); `radius-full` at icon-grid size; never color-alone.
2. §Primary navigation — keybindings first-class (start/stop/proceed/abort bound to shortcuts); focus ring `color-focus` visible on every control; active picker item `border-emphasis` edge (focus never color-only).
3. §Header strip pattern — label row above dense list, echoing counts + dynamic hold-state indicators during run.

## Anti-patterns to avoid
1. Do NOT build a KPI-card grid (§Primary content block 1 §Explicitly Rejected Default).
2. Do NOT use color-only status indicators (pair every lamp with glyph + text per §Primary content block 1 + a11y §Focus Order SC 2.4.3).
3. Do NOT apply shadows (borders-only depth: `border-subtle` seams only, no shadow per §Wireframe annotations).

## Contract bindings
- **StatusLamp component** (§Primary content block 1, §Primary content block 2) — reuse existing `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` + `ui/src/lamp.ts` `LAMP_META` (byte-mirror of `conductor-core::lamp.rs`); verdict-first projection via `Lamp::for_record` semantics (scope §2).
- **a11y bindings** — focus order visible (a11y §Focus Order SC 2.4.3); status never color-alone (a11y compliance); verdict/state changes announced via live-region (scope accessibility requirement).
- **Tauri capabilities** — deny-by-default posture honored (§IA notes §Security guardrails); coverage matrix is read-only, no new ACL gates required per scope.

## Acceptance criteria contributions
- "(layouts) Coverage-matrix view renders in main canvas region of desktop-webview surface (layout-templates §Primary content block 1 wireframe)."
- "(layouts) Rows support keyboard navigation via focus-ring + border-emphasis active indicator; focus never color-only (layout-templates §Primary navigation + a11y §Focus Order SC 2.4.3)."
- "(layouts) Each status lamp paired with text label; no color-only state (layout-templates §Primary content block 1)."
- "(layouts) Empty state renders as prose 'No scenarios loaded', not placeholder (layout-templates §Primary content block 1)."

## Relevant amendment history
- **2026-06-24 frameless-window-shell** (§Wireframe + §Component — Header): titlebar height reconciled `space-lg` → `space-xl` (32px) to accommodate 18px Heading phase line + 20px window-control glyphs. Relevant: coverage-matrix view renders below the titlebar; grid/padding calculations depend on correct titlebar height (space-xl).
- **2026-06-23 line-oriented-output-rendering** (§Surface: cli — Primary screens): `conductor coverage [--write]` verb registered; `report` output switched to colored `comfy-table` results view (reads JSONL journal). Context: desktop coverage-matrix view is parallel implementation (same 60-P-ID coverage domain); amendment shows CLI pattern settled and data sourced from runs.db, which informs scope's "data source" open question (P4 decision needed on live vs. presentational desktop variant).