# layouts extract

## Relevance
partial — the chunk creates scenario TOMLs (catalog data) with no UI/surface rendering; layout structure only touches the verdict lamps + decision gates referenced as cross-domain bindings, not the chunk itself.

## Constraints
None — per layout-templates §Primary Surfaces, Conductor owns two surfaces (desktop-webview frameless single-station + cli line-oriented); this chunk authors no surface modification, surface creation, or component placement. The scenario TOMLs are backend catalog data (Epoch-7 model), not rendered layout.

## Patterns to follow
(none)

## Anti-patterns to avoid
(none)

## Contract bindings
**Signature placement reinforcement (indirect):** The verdict lamps (layout-templates §Component — Primary content block 2, desktop-webview; §Component — Primary content block 2, cli) will later render the P-013..P-016/P-057 verdict outcomes (`Pass` / `CalibrationRegion` / `Fail` / `Blocked` / `ManualCheck` / `KnownResidual`) per the layout's status-tier convention. The absence-presence assertions this chunk declares (e.g., "15s burst suppressed" = no visible `Fail` lamp; "dual-condition bypass surfaces" = visible `relative-bypass` or `absolute-bypass` verdict) bind forward to the Epoch-8 evaluator's comparison logic and Epoch-9/10's rendered verdict display — but the TOMLs themselves impose no layout constraint.

## Acceptance criteria contributions
(none) — layout contributes no acceptance criteria to the chunk itself. Forward-binding: Epoch-8 verdict rendering will verify the verdict lamps render per layout-templates status-color convention (layout-templates §Component — Primary content block 2); Epoch-9 desktop + Epoch-10 CLI will verify the frozen-count signature holds during an operator-pause decision gate (layout-templates §Signature placement).

## Relevant amendment history
(none) — layout-templates-amendments.md does not exist; the layout-templates.md foundation (generated 2026-06-14) precedes this chunk's date (2026-06-22).