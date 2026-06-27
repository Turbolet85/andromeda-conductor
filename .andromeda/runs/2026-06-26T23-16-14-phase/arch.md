# arch extract

## Relevance
partial — design semantics + workspace placement apply; no seam/contract/core changes

## Constraints
- (arch) Frontend stack pinned React 19.x + Vite ≥8.0.16 + Tailwind v4.1 (Oxide); package manager npm with `package-lock.json` committed + `npm audit` clean gate (per §Inherited Defaults Frontend + §Stack and Technologies Desktop-frontend)
- (arch) Lamp variant set MUST exactly mirror the six canonical states: `Pass`/`Fail`/`Hold` (CalibrationRegion, verdict-first)/`ManualCheck`/`KnownResidual`/`Blocked` per §Standard Contracts Run report envelope + §Read-Back Dependency Posture (per 2026-06-21 amendment: `Verdict::default_report_state` mapping, verdict-first precedence so calibration row renders HOLD, not Manual)
- (arch) Design-token-sourced styling only; no hardcoded colors — enforce the *never-color-alone* invariant (color + text label + glyph per state) via component-level ARIA + semantic markup (per §Occupied Resources §Design tokens declared on `:root`)
- (arch) Code lives in `crates/conductor-tauri/ui/src/components/` per workspace boundary (§Occupied Resources Frontend asset subtree + §Inherited Defaults Frontend)
- (arch) No live data wiring / Tauri `Channel` / `#[tauri::command]` — primitives are presentational; deferred to ch7 (Run-report + operator-checklist views) per scope
- (arch) TypeScript strict (no `any`, functional components + hooks) — aligns with frontend development discipline

## Patterns to follow
- Design-token consumption from established `crates/conductor-tauri/ui/src/styles/tokens.css` `:root` namespace; extend only if a `design-system.md` lamp token is not yet present (per scope + 2026-06-15 amendment)
- Radix Dialog (shadcn/Radix lineage per ch3's picker precedent) for the dialog scaffold — focus-trapped, ESC/overlay dismiss, ARIA-labelled, token-styled
- Accessibility baked into each component at definition time — keyboard semantics (Tab/arrow/Enter), ARIA labels, focus management, color + text + glyph triples — not bolted on later
- Render-all gallery (presentational; behind a dev affordance) to exercise all variants/states visually + via `tsc strict` + `vite build` type-safety gate

## Anti-patterns to avoid
- No live data / Channel subscription / `#[tauri::command]` wiring (out of scope; deferred to ch7)
- No color-alone state encoding (universal *never-color-alone* accessibility mandate)
- No new seam crates / env vars / occupied resources (UI-only, zero engine/model change per scope)

## Contract bindings
- Lamp semantics ↔ report-seam + CLI: both define the 6-state canonical set; the frontend lamp design MUST byte-match the `ReportState` enum + `Verdict::default_report_state` mapping
- Frontend asset subtree ↔ build system: npm `run build` must complete before workspace `cargo build conductor-tauri` (per 2026-06-24 amendment §Infrastructure Patterns Build system — `generate_context!` resolves `build.frontendDist` at COMPILE time)

## Acceptance criteria contributions
- (arch) Lamp variants exactly mirror the 6 canonical states (`ReportState` enum + `Verdict::default_report_state` mapping) with text label + glyph per state, never color-alone
- (arch) Dialog scaffold uses Radix Dialog (ARIA-aware, focus-trapped, token-styled)
- (arch) All component code TypeScript strict; `tsc` + `vite build` gates pass
- (arch) Design tokens consumed from `styles/tokens.css` `:root` namespace; new lamp tokens added only if `design-system.md` defines them

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** — registered React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide) frontend stack + npm discipline + `crates/conductor-tauri/ui/` asset subtree (not a Cargo member)
- **2026-06-21-run-report-envelope-serializer** — `Verdict::default_report_state` mapping recorded (`Pass→Pass`/`Fail→Fail`/`CalibrationRegion→ManualCheck`), verdict-first lamp precedence (calibration row renders HOLD, not Manual) — CRITICAL for this chunk's lamp set
- **2026-06-24-frameless-window-shell** — build-order coupling: npm `run build` before workspace `cargo build conductor-tauri`