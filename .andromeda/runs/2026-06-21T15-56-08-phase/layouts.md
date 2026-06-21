# layouts extract

## Relevance
partial — chunk adds per-scenario expected-outcome blocks but doesn't create/modify UI surfaces; layout patterns apply only to envelope data model (latency_ms, slo_tier fields) and run-report rendering contract.

## Constraints
- Per layout-templates §Surface: desktop-webview | cli, verdict lamp + state tier render in the run-report view; `latency_ms` and `slo_tier` are Data-role fields that must survive the piped artifact model (per cli §Pipe discipline).
- Per layout-templates §Component — Primary content block 2 (run-report view), each P-ID verdict line pairs its slo_tier (`<5s` / `<20s` / `<90s`) with a color-mapped status, never color-alone (per layout-templates §IA notes multi-surface coordination).
- Per layout-templates §Component — Footer (status strip), the verdict roll-up tally and the hold-step-index echo reflect the frozen step-index in `count-hold` while a run is held (signature reinforcement #3 in §Decisions Log). The expected-outcome model must feed latency + tier into this footer roll-up correctly.
- Per layout-templates §Component — Header (frameless titlebar + Paused-count heartbeat) §Motion trigger placement, a deterministic/hard deadline breach hard-fails; a sample-count-floor miss routes to `CalibrationRegion` (the amber `count-hold` tint, never red `status-fail`). This distinction must surface in the lamp color selection.
- Per layout-templates §Component — Primary content block 1 (coverage matrix), the matrix header strip echoes the frozen step-index in `count-hold` (signature placement #3); the expected-outcome evaluator must not break the step-index tracking that feeds this header.

## Patterns to follow
- Per layout-templates §Decisions Log | Cross-surface IA decisions, the status tier maps 1:1 to typed run outcomes: `Pass` / `CalibrationRegion`-HOLD / `Fail` (machine-verdict triad) plus `ManualCheck` / `KnownResidual` / `Blocked` (non-verdict ReportStates). The expected-outcome evaluator's `matched` + `class` must resolve into this tier unambiguously.
- Per layout-templates §Component — Footer + run-report, every verdict state pairs its color token with an ASCII bracket prefix (`[PASS]` / `[HOLD]` / `[FAIL]` / `[BLOCKED]`), so the signal never relies on color alone (per layout-templates §IA notes | Pipe discipline). The expected-outcome model must not introduce a state that lacks an ASCII label.
- Per layout-templates §Component — Hero / signature section (operator-pause dialog) + cli §Component — Hero / signature output, the frozen count + step-index + the decision moment all surface at the go/no-go hold. The expected-outcome evaluator's SLO-timing outcome must not corrupt or omit the step-index in the hold-moment rendering.

## Anti-patterns to avoid
- Do NOT introduce a verdict/state that is never color-alone (collapses into `Fail`, maps ambiguously to desktop lamp or cli bracket prefix). All run-report verdict states must map 1:1 to design tokens (per layout-templates §IA notes).
- Do NOT measure latency from wall-clock-from-test-start or any non-journal-relative timestamp. Latency must be `read_back_observed_at − journal_emitted_at` to remain decoupled from run-start wall-clock (per layout-templates §IA notes | Multi-surface coordination and scope requirement).
- Do NOT introduce a sample-count-floor breach that hard-fails. A floor miss → `CalibrationRegion` (count-hold amber, never red status-fail), per the motion-trigger-placement strategy (layout-templates §Decisions Log | Motion trigger placement).

## Contract bindings
obs ↔ run-report envelope — `latency_ms` + `slo_tier` fields computed here are rendered in the desktop run-report card (layout-templates §Component — Primary content block 2) and cli verdict lines + footer roll-up; cli footer roll-up (layout-templates §Component — Footer / terminator) also consumes the tier for color-mapping. The expected-outcome evaluator is upstream of the run-report serializer (Epoch 6).

## Acceptance criteria contributions
- (layouts) Run-report verdict line renders P-ID + latency_ms (Data, color-id-cyan) + slo_tier (`<5s`/`<20s`/`<90s`) + verdict state with matching lamp glyph (layout-templates §Component — Primary content block 2 wireframe).
- (layouts) Footer roll-up tally (`3 Pass · 1 Calib · 1 Fail · 1 Manual · 1 Residual · 1 Blocked`) surfaces the slo_tier each count reflects; each verdict state token tinted in its status-color token (layout-templates §Component — Footer wireframe).
- (layouts) Hard deadline breach → `Fail` (status-fail red lamp); sample-floor miss → `CalibrationRegion` (count-hold amber lamp, never red) (layout-templates §Decisions Log | Motion trigger placement).
- (layouts) Latency rendered journal-relative only; never from test-start wall-clock. Timestamps derive from `std::time` (layout-templates §Component — Header | Paused-count heartbeat and scope acceptance anchor).

## Relevant amendment history
(none)

## Phase note (orchestrator)
Layouts coverage here is FORWARD-LOOKING — it governs Epoch 6/8/9 rendering of the run-report verdict line + footer roll-up, NOT this chunk's backend logic. This chunk builds no UI/CLI surface. The binding that DOES bear on this chunk: keep `slo_tier` a closed `{<5s,<20s,<90s}` enum, keep latency journal-relative, and ensure every verdict/state resolves 1:1 (hard breach → Fail; sample-floor miss → CalibrationRegion).
