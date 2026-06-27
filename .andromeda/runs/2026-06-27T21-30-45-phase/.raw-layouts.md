# layouts extract

## Relevance
partial — uses existing CLI verbs and verdict-output patterns; makes no layout changes to surfaces, wireframes, or focus order.

## Constraints
1. CLI output must preserve the verb-noun flat command model (`conductor run|suite|preflight|coverage|report`) — no new routing or interactive layering (layout-templates §Surface: cli § Component — Primary navigation).
2. Every verdict/state cell must pair ANSI color with ASCII bracket prefix (`[PASS]` / `[HOLD]` / `[FAIL]` / `[BLOCKED]` / `[MANUAL]` / `[RESIDUAL]`) so it survives `NO_COLOR` and piping (layout-templates §Surface: cli § Component — Primary content block 2).
3. Report artifact output routes to stdout (machine-parseable), messages to stderr; ANSI auto-strips when piped (layout-templates §Surface: cli § Component — Footer / terminator).
4. Headless/agent-mode path must never block on an interactive `inquire` prompt — isatty-gated only; operator-pause decision logged to artifact (layout-templates §Surface: cli § IA notes § Headless invariant).
5. Every verdict state carries its canonical ANSI mapping and text label: Pass(114) / Fail(203) / CalibrationRegion(179) / Blocked(60) / ManualCheck(146) / KnownResidual(246) (layout-templates §Surface: cli § Decisions Log).
6. Report container routes verdicts via the §Component — Primary content block 2 line format (glyph + prefix + color + identifiers) — never downgrade `Blocked`/`ManualCheck`/`KnownResidual` to `Fail` (layout-templates §Surface: cli § IA notes).

## Patterns to follow
1. Verdict line format: glyph (✓/✗/⚠/•/?/~) + ASCII prefix + ANSI color + Body role text + Data-role identifiers (P-ID / run_id / latency_ms) in ANSI 117 (layout-templates §Surface: cli § Component — Primary content block 2).
2. Blocked/ManualCheck/KnownResidual precondition detail: per-row note in `text-tertiary` (ANSI 146) mapping, never collapsed to red (layout-templates §Surface: cli § Output structure — verdict lines; Component — Primary content block 2).
3. Coverage/SLO table header: six-column fixed (`P-ID` / scenario / state / slo_tier / latency_ms / fingerprints), P-ID in ANSI 117 cyan, header dimmed (layout-templates §Surface: cli § Component — Primary content block 1).

## Anti-patterns to avoid
1. Never emit machine verdicts (`Pass`/`Fail`) for `Blocked` / `ManualCheck` / `KnownResidual` — each is a distinct non-verdict ReportState with its own glyph/color (layout-templates §Surface: cli § Decisions Log § Cross-surface IA decisions).
2. Never use color-alone — every state cell must pair ANSI with text label so it survives `NO_COLOR` / screen readers / piping (layout-templates §Surface: cli § IA notes § Pipe discipline).
3. Never block the headless path on an interactive prompt — the `agent-run.sh` source-of-truth must skip `inquire` prompts and record decisions to the artifact (layout-templates §Surface: cli § IA notes § Headless invariant).

## Contract bindings
- **CLI → a11y (Conductor a11y specialist):** every verdict state carries an ASCII glyph + text label so status is never color-alone; `NO_COLOR` compliance required (binds to a11y §NO_COLOR support).
- **CLI → design (Conductor design system specialist):** ANSI color mappings are stable cross-releases (count-nominal ↔ 114, count-hold ↔ 179, status-fail ↔ 203, count-blocked ↔ 60, color-id-cyan ↔ 117); if design tokens change, ANSI mappings follow (layout-templates §Surface: cli § Decisions Log).
- **CLI → run-engine:** verb-noun structure pinned for agent parsing; output stable across versions (layout-templates §Surface: cli § IA notes § Command model).

## Acceptance criteria contributions
1. (layouts) Every verdict state (Pass / Fail / CalibrationRegion / Blocked / ManualCheck / KnownResidual) renders in its designated glyph + ASCII prefix + ANSI color + text label (layout-templates §Surface: cli § Component — Primary content block 2).
2. (layouts) Verdict output survives `NO_COLOR` and piping without losing semantic information (layout-templates §Surface: cli § IA notes § Pipe discipline).
3. (layouts) Headless agent path (non-interactive TTY) never blocks on operator-pause prompt; decision recorded to artifact (layout-templates §Surface: cli § IA notes § Headless invariant).
4. (layouts) Report line structure complies with the established verdict-line format (P-ID + scenario label + verdict text + latency_ms + slo_tier in designated color + text pairings) (layout-templates §Surface: cli § Component — Primary content block 2).

## Relevant amendment history
1. **2026-06-23-5-command-agent-run-harness:** `conductor preflight [--json]` verb registered; exits 0 iff `ready:true`, else non-zero (layout-templates §Surface: cli § Primary screens (commands)). This chunk's canary bridge will exercise this verb to flip readiness from `Blocked` → `ready`.
2. **2026-06-23-line-oriented-output-rendering:** `conductor coverage [--write]` verb and `conductor report` colored `comfy-table` output registered (layout-templates §Surface: cli § Primary screens (commands) · §Output structure). This chunk will populate the report via the established `report` verb and table format.
