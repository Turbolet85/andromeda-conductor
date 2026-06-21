# layouts extract

## Relevance
partial — chunk is verdict classification logic, not surface rendering; layouts apply only to report-state display downstream

## Constraints
- (per layout-templates.md §Primary Surfaces) Desktop-webview surface is single-station console; CLI is line-oriented stdout; no new surfaces created by verdict classification
- (per layout-templates.md §Component — Primary content block 2) Verdict/report-state lamps (Pass/CalibrationRegion/Fail/Blocked) paired with ASCII text prefixes; status never color-alone
- (per layout-templates.md §Wireframe — Run report (terminal)) Verdict outcomes (Pass/Fail/CalibrationRegion) render in report lines with lamp + label + text
- (per layout-templates.md §Surface: cli §Component — Primary content block 2) CalibrationRegion maps to `[HOLD]` ANSI 179 bracket prefix, never silent Fail
- (per layout-templates.md §Decisions Log) Same design tokens by name across surfaces; verdict tier reads identically on desktop CSS vars and CLI ANSI codes
- (per layout-templates.md §Component — Primary content block 1) Coverage matrix header strips verdict counts (Pass/Fail/CalibrationRegion/Manual/Residual/Blocked) in status-color mappings

## Patterns to follow
- Three-state verdict triad (Pass / CalibrationRegion / Fail) as typed Verdict enum output; never collapse model-interpretive assertions to silent Fail
- CalibrationRegion treated as reportable outcome (not error), displayed with hold-amber tint on both surfaces
- Paired color + ASCII prefix convention (e.g. ANSI 179 `[HOLD]` for CalibrationRegion) ensures status survives NO_COLOR and piping

## Anti-patterns to avoid
- Do not emit Fail for model-interpretive assertions; route to CalibrationRegion instead
- Do not make verdict classification runtime-guessed; policy class (hard vs calibration-region) is declared up front as claim property
- Do not introduce new run-state surfaces or modal patterns; verdict classification is logic-layer only

## Contract bindings
- Verdict type consumed from `conductor-core` (existing); no new cross-crate edges
- Downstream: verdict classification feeds report-state mapping (Epoch 6) and CLI/desktop rendering (Epochs 8/9) — layout-templates §Footer / §Primary content block 2 expect typed verdicts with consistent color mappings
- A11y binding: verdict changes are announced to assistive tech (live-region); layout-templates §Component — Primary content block 1 states "a verdict change is announced"

## Acceptance criteria contributions
- (layouts) Verdict classification outputs Pass/Fail/CalibrationRegion as typed Verdict enum (layout-templates §Decisions Log verdict tier definition)
- (layouts) CalibrationRegion verdict never produced as silent outcome — always reported in run-report view (layout-templates §Wireframe — Run report (terminal) CalibrationRegion lamp + count-hold tint)
- (layouts) Hard-path comparison is deterministic; same inputs always produce same Verdict (layout-templates §IA notes security guardrails)

## Relevant amendment history
(none)
