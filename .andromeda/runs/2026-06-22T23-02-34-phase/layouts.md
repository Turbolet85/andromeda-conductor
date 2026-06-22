# layouts extract

## Relevance
Partial — the chunk defines declarative scenario catalog entries (P-025/026/027/032/036) and test wiring for the scenario model; layout coverage applies only to P-025, P-026, P-027 (operator-checklist/ManualCheck display surface) and P-032 (KnownResidual verdict lamp); P-036 (cross-run fingerprint) is backend-only.

## Constraints
1. Per layout-templates §Desktop-webview / Component — Primary content block 2: verdict lines pair each verdict lamp with its state text and SLO tier; ManualCheck renders as a neutral checkbox glyph (`status-manual`) in the operator-checklist row, not a machine verdict.
2. Per layout-templates §Desktop-webview / Component — Operator-checklist: each ManualCheck P-ID renders a card row with induced state (left) and expected-observation yes/no (right); space toggles a row; ticking resolves that item's verdict to Pass/Fail while report-state stays ManualCheck.
3. Per layout-templates §Desktop-webview / Component — Primary content block 2: KnownResidual renders a muted dashed-ring dot (`status-residual`) in-place per P-ID, paired with "expected until {named fix}" note (e.g. P-032's `recent_commits stub until v0.3.0`), present-but-accepted, never a red Fail.
4. Per layout-templates §CLI / Component — Primary content block 2: verdict lines pair color with ASCII prefix; ManualCheck renders `? P-ID Manual halo→burgundy? · no OS toast?` (ANSI 146, neutral lavender); KnownResidual renders `~ P-ID Residual recent_commits stub → v0.3.0` (ANSI 246, muted), never red.
5. Per layout-templates §Desktop-webview / Component — Primary content block 1 / Header strip: while a run is held, the matrix header echoes the frozen step-index in `count-hold` (signature reinforcement); new scenarios must not disrupt this reinforcement surface.
6. Per layout-templates §Decisions Log: the signature placement strategy requires the operator-pause hold-point to be legible at the decision moment; ManualCheck rows and KnownResidual notes must not obscure or compete with the frozen-count display in the operator-pause dialog (layout-templates §Desktop-webview / Component — Hero / signature output).

## Patterns to follow
1. Layout-templates §Desktop-webview / Component — Primary content block 2: every status above is paired with an ASCII text prefix (`[PASS]` / `[HOLD]` / `[FAIL]` / `[MANUAL]` / `[RESIDUAL]` / `[BLOCKED]`) so the signal is never color-alone and survives NO_COLOR / piping; apply the same pattern to any new verdict or report-state display (P-025/026/027 ManualCheck, P-032 KnownResidual, P-036 recurrence detection).
2. Layout-templates §Desktop-webview / Wireframe — Run report (terminal): the run-report card lists per-P-ID verdict lines with distinct non-result states (Empty / In-progress / ManualCheck / KnownResidual / Blocked), each rendered with its own lamp glyph and text, never downgraded to Fail — the same "no surprise failure" rule applies to KnownResidual as Blocked.

## Anti-patterns to avoid
1. Do NOT render ManualCheck (P-025/026/027) as a red Fail or any severity indicator; the checklist claim is an operator observation, not a machine verdict — it renders neutral (`status-manual`), never under the verdict triad (Pass/CalibrationRegion/Fail).
2. Do NOT render KnownResidual (P-032) as a red Fail; pre-accepted gaps are present-but-accepted, never a surprise failure — render muted (`status-residual`), paired with the "expected until {named fix}" note.
3. Do NOT use color alone to distinguish ManualCheck or KnownResidual from other report states; every surface must pair color with an ASCII/text prefix (desktop: glyph + text; CLI: bracket prefix + text) to survive NO_COLOR and piping.

## Contract bindings
- **ManualCheck operator-checklist display** ↔ layout-templates §Desktop-webview / Component — Operator-checklist (the `ManualCheck` render — drive+observe surface): P-025/026/027 must render in the card-row checklist surface with induced-state label (left) and expected-observation yes/no (right); space toggles, ticking resolves verdict to Pass/Fail while report-state stays ManualCheck.
- **KnownResidual verdict lamp** ↔ layout-templates §Desktop-webview / Component — Primary content block 2 (run-report view + verdict lamp): P-032 must render the muted dashed-ring lamp paired with the "recent_commits producer stub until v0.3.0" note; layout-templates §CLI / Component — Primary content block 2: CLI must render `~ P-032  context-grounding     Residual` with ANSI 246 muted color and the stub note.
- **Operator-pause go/no-go dialog** ↔ layout-templates §Desktop-webview / Component — Hero / signature section: any scenario that gates an operator-pause hold (none in this chunk's 5 P-IDs, but the path is available) must use shadcn AlertDialog with the frozen count snapshot in the header.
- **Cross-run fingerprint "Previously seen" recurrence** (P-036) ↔ layout-templates §Desktop-webview / Wireframe — Run report (terminal) / layout-templates §CLI / Component — Primary content block 2: P-036's recurrence detection lives in the run-report view (already present in the wireframe as existing MCP read-back contract; no new surface required).

## Acceptance criteria contributions
1. (layouts) ManualCheck operator-checklist rows (P-025/026/027) render in the run-report card, each with neutral `status-manual` checkbox glyph, induced-state label (left), expected-observation yes/no toggle (right), space-to-toggle (layout-templates §Desktop-webview / Component — Operator-checklist).
2. (layouts) KnownResidual lamp (P-032) renders as muted dashed-ring dot `status-residual` in the run-report card, paired with text "residual: recent_commits producer stub until v0.3.0 (known)" (layout-templates §Desktop-webview / Component — Primary content block 2).
3. (layouts) CLI verdict lines for P-025/026/027 render `? P-ID {scenario} Manual` with ANSI 146 + [MANUAL] bracket prefix; P-032 renders `~ P-032 context-grounding Residual` with ANSI 246 + [RESIDUAL] bracket prefix; no color-alone (layout-templates §CLI / Component — Primary content block 2).
4. (layouts) Operator-pause dialog frozen-count header (if P-032 or any scenario triggers a hold) remains the loudest signature element, unobscured by ManualCheck rows or KnownResidual notes during the hold (layout-templates §Desktop-webview / Wireframe — Run console (HOLD) — signature moment).

## Relevant amendment history
(none) — amendment history file does not exist; this is a fresh project, expected behavior on first amendment-capable chunk.