# layouts extract

## Relevance
relevant — The chunk implements two desktop React views (run-report verdict lines + operator-checklist) directly described in the plan's desktop-webview surface.

## Constraints
1. Desktop-webview surface rules (per §Surface: desktop-webview) — Tauri 2 frameless window, React 19 / Tailwind v4.1 / shadcn components, expression level 0.3 (no animation library, color transitions only)
2. Verdict lamp never color-alone (per §Component — Primary content block 2) — each lamp must pair color + status text + glyph; state is never color-encoded exclusively
3. Verdict-first rendering (per §Component — Primary content block 2) — lamp precedes scenario label / verdict text in the row hierarchy
4. ManualCheck operator-checklist uses neutral-lavender checkbox glyphs, NOT the pass/fail/hold verdict-triad colors (per §Component — Operator-checklist)
5. Blocked rows render measurement columns as — / null, never red error (per §Component — Primary content block 2) — Blocked ≠ Fail
6. Each status state (Pass / CalibrationRegion / Fail / ManualCheck / KnownResidual / Blocked) carries its own distinct glyph + text pairing, never conflated (per §Component — Primary content block 2 + §IA notes)
7. Reuse StatusLamp + OperatorChecklist primitives from component-primitives-library; do not author new lamp/checklist components (per scope non-goals)

## Patterns to follow
1. Verdict-lamp glyph coding: filled dot (Pass/CalibrationRegion/Fail) · hollow ring (Blocked) · neutral checkbox (ManualCheck) · muted dashed dot (KnownResidual) (per §Component — Primary content block 2)
2. Operator-checklist two-column layout: induced-state text (left) + yes/no observer claim checkbox (right), native checkboxes (per §Component — Operator-checklist)
3. Color-to-text pairing: every lamp color mapped to its status text label so the signal survives stripped color / assistive tech (per §Component — Primary content block 2)
4. Data sourcing: reuse `lampForRecord` projection + read-only `#[tauri::command]` returning existing `conductor-core`/`conductor-report` types, never re-author the shape in TS (per scope folded carries)

## Anti-patterns to avoid
1. Status color-only rendering — never use color without text label + glyph pair
2. Treating Blocked as a Fail / red error — Blocked is "never measured," never in the failure family (per §Component — Primary content block 2)
3. Animated / blinking verdict lamps — verdict changes resolve via motionless `motion-micro` color transitions only (per §Component — Primary content block 2)
4. Treating ManualCheck as a machine verdict — it is operator-confirmed observation, not automated (per §Component — Operator-checklist)
5. Treating KnownResidual as a failure — it is a measured pre-accepted gap (muted dashed dot, never red)
6. Rebuilding lamp/checklist primitives instead of consuming the shipped components from component-primitives-library

## Contract bindings
- StatusLamp + OperatorChecklist component reuse ↔ implementation must honor their a11y + interaction contracts (focus visibility, accessible names, text-label pairing)
- Verdict-first lamp rendering ↔ a11y: status text + glyph readable by assistive tech, announced on state change

## Acceptance criteria contributions
1. (layouts) Run-report verdict lines render per-P-ID with verdict-first StatusLamp in the run-report card region (layout-templates §Wireframe — Run report)
2. (layouts) Each verdict line pairs: glyph (filled/hollow/checkbox/dashed) + status text (Pass/CalibrationRegion/Fail/ManualCheck/KnownResidual/Blocked) + color, never color-alone (layout-templates §Component — Primary content block 2)
3. (layouts) Blocked rows: measurement columns render — / null, hollow-ring glyph, never red (layout-templates §Component — Primary content block 2)
4. (layouts) Operator-checklist renders neutral-lavender checkbox glyphs with induced-state text (left) + yes/no claim (right); space toggles rows (layout-templates §Component — Operator-checklist)
5. (layouts) ManualCheck rows render ? glyph + [MANUAL] status label, as operator-observation record not machine verdict; unticked count surfaces in footer (layout-templates §Component — Operator-checklist)

## Relevant amendment history
2026-06-24-frameless-window-shell — titlebar height space-lg → space-xl (§Component — Header): The run-report view's wireframe includes the frameless titlebar; this amendment reconciled titlebar height from 20px to 32px to accommodate the Heading-tier phase line + window-control glyphs. Applies to all desktop-webview surfaces using the header component. (Layout surface consistency; implementation-spec reconciliation.)
