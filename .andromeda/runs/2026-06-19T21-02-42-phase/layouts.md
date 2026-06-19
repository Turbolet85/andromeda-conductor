# layouts extract

## Relevance
Partial — chunk produces a fault helper (no layout surface); layout-templates apply only to how verdict/state is rendered if/when the gap/resume helper produces output through the run console (a LATER epoch, not this chunk).

## Constraints
- Layout structure defined per layout-templates §Primary Surfaces (desktop-webview + cli only; no auth/credential/responsive layouts)
- If verdict emits to run-report: must render per layout-templates §Component — Primary content block 2, with status-lamp glyph + text paired so state is never color-alone (layout-templates §Run report wireframe §Verdict line §supporting convention)
- If fault helper produces live state during a run: coverage-matrix rows render per layout-templates §Component — Primary content block 1, status lamp tinted in status-color mapping, row header echoes frozen step-index if held (layout-templates §Signature placement reinforcement)
- Footer roll-up tally includes P-015 verdict in its status-color map (layout-templates §Footer §verdict roll-up) — unticked if ManualCheck, colored if verdicted

## Patterns to follow
- Verdict delivery through dense single-row-per-P-ID matrix (no KPI cards; layout-templates §Coverage matrix) — status lamp glyph (filled/hollow/checkbox/muted) paired with text, P-ID in color-id-cyan
- Status never color-alone; all verdicts paired with ASCII prefix (`[PASS]` / `[HOLD]` / `[FAIL]` / `[BLOCKED]`) on cli; css color + text on desktop (layout-templates §Verdict convention)
- Verdict announced via live-region on state change (desktop) or line-oriented output (cli); downgrading to Fail is forbidden for non-verdict ReportStates (layout-templates §Distinct non-result states)

## Anti-patterns to avoid
- No KPI cards, no card grids — matrix must stay dense single-row (layout-templates §Rejected Default)
- Never emit a verdict lamp that is color-alone; never blink or flash lamps (layout-templates §Motion §Verdict lamps resolve motionless)
- Never collapse KnownResidual / Blocked / ManualCheck into a red Fail (layout-templates §Distinct non-result states)

## Contract bindings
- **desktop-webview run-report view ↔ verdict rendering** — P-015 verdict row composes per layout-templates §Run report §verdict line structure (lamp + text + P-ID/latency/slo in Data role / color-id-cyan)
- **cli `comfy-table` SLO table ↔ verdict state** — P-015 row renders with status-bracket prefix + state text, measurement cols render `—` if never measured / Blocked, row detail carries precondition if applicable
- **footer roll-up tally ↔ verdict status** — P-015 verdict counted in status-color tally and paired with label text (layout-templates §Footer §roll-up)

## Acceptance criteria contributions
- (layouts) P-015 verdict renders in coverage-matrix per surface (desktop: dense row + lamp glyph + text; cli: comfy-table row with status-bracket prefix; layout-templates §Component — Primary content block 1/2).
- (layouts) P-015 verdict displayed with status-lamp glyph tinted in status-color mapping, paired with readable text so state never relies on color alone (layout-templates §Verdict §supporting convention).
- (layouts) On cli: P-015 state cell includes `[PASS]` / `[HOLD]` / `[FAIL]` / `[BLOCKED]` bracket prefix paired with color (layout-templates §cli §verdict lines).
- (layouts) Footer roll-up includes P-015 in its tally, each count in its status-color map with paired label text (layout-templates §Footer §roll-up).

NOTE (orchestrator): all layouts contributions describe LATER-epoch render surfaces (Epoch 8 cli / Epoch 9 desktop), not this backend chunk — treat as low-signal for THIS chunk's plan.

## Relevant amendment history
(none)
