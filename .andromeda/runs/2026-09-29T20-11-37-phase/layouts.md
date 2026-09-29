# layouts extract

## Relevance
partial — the chunk builds no Conductor surface element (a live re-drive, a harvest-tier test grade, a contract document and a scenario header comment); layouts applies only as guards on how `halo-hue-encoding`'s existing outcomes render on the two surfaces, and the COMPACT-WIDGET window the leg needs is Pulse's UI, outside this plan entirely.

## Constraints
- The per-P-ID lamp / bracket-label set is closed at six (`Pass` / `CalibrationRegion`-HOLD / `Fail` / `ManualCheck` / `KnownResidual` / `Blocked`); a new grade must not mint a seventh lamp or label, and a run-level qualifier rides OUTSIDE the lamp column (per layout-templates §cli → Component — Primary content block 2 (verdict / report-state lines); §Decisions Log).
- A scenario that declares a `[[checklist]]` item renders as `[MANUAL]` / the neutral `status-manual` checkbox glyph — an operator-checklist item, never a machine verdict; headless, it is recorded unconfirmed. Moving P-025's hard grade to the harvest tier leaves that render untouched, since the grade is not an `[[expected]]` check (per layout-templates §cli → Output structure — `conductor run <scenario>`; §desktop-webview → Component — Operator-checklist).
- The per-check detail region in `runs/<run_id>.md` carries one indented line per EXPECTED check only; "a scenario that graded nothing (blocked, or declare-only) contributes no such line", so the harvest-tier hue grade must not surface as a per-check detail line or a new results/SLO column (per layout-templates §cli → Component — Primary content block 2, "Per-check detail region").
- The results/SLO table has exactly 6 columns and the `latency_ms` cell is the Conductor-side journal-relative value; binding the hue `duration_ms` into it would conflict with the scope's boundary on `latency_ms` / `budget_ms` (per layout-templates §cli → Component — Primary content block 1 (results / SLO table)).
- The `--live` stage is the operator-gated live-Pulse path, reachable from no default or CI path, leading with the non-priming `conductor preconditions` probe that refuses at exit 1 with `[PRECONDITION]` lines; the headless path never blocks on a prompt. Whether the re-drive runs through `--live`, a dedicated operator-gated test binary, or `conductor run` on a TTY is research's question (per layout-templates §cli → Primary screens (commands); §cli → Component — Primary navigation (verb structure)).
- A `CalibrationRegion` renders as `[HOLD]` / amber, "never silently a Fail"; the chunk's move from calibration-region routing to a hard `Fail` breach lives in the harvest test's pass/fail, and whether any rendered surface currently shows P-025 as `CalibrationRegion` is research's question (per layout-templates §cli → Component — Primary content block 2; §desktop-webview → Wireframe — Idle with a report).

## Patterns to follow
- If the live leg runs interactively, the hold point that declares `[[checklist]]` items shows checklist rows in the operator-pause go/no-go dialog, between Body and Actions as a sibling of the Description. On cli the `inquire` y/n prompt is TTY-gated. Both are already specified, and the chunk reuses them unchanged (per layout-templates §desktop-webview → Component — Hero / signature section; §cli → Component — Hero / signature output).
- The run-level-qualifier precedent: `[ENVIRONMENT-SUSPECT]` and `[PRECONDITION]` sit outside the lamp column and alter no per-check state. If research finds the re-drive needs any operator-facing note (e.g. the leg's precondition that the widget window be mounted), it follows this shape, never a new lamp (per layout-templates §cli → Component — Primary content block 2).
- Error output goes to stderr, sanitized: `error: <short>` + `hint: <fix>`, with no host paths. Any refusal the re-drive's harness path prints follows this shape (per layout-templates §cli → Component — Footer / terminator + error output).

## Anti-patterns to avoid
- Minting a seventh lamp, a new bracket label or a 7th results-table column for the hue-shift grade (per layout-templates §cli → Component — Primary content block 1 / 2).
- Collapsing the scenario's `ManualCheck` checklist render into a machine verdict because the budget now grades hard elsewhere; the checklist half stays an operator observation (per layout-templates §desktop-webview → Component — Operator-checklist).
- An interactive prompt on the headless path; it would silently break the release gate (per layout-templates §cli → Component — Primary navigation (verb structure)).

## Contract bindings
- layouts ↔ tests: the hard grade lives in a `conductor-run` harvest test over a committed capture, outside the run-report lamp column and the per-check detail region. The layout contract holds only if the grade never routes through `[[expected]]` (per layout-templates §cli → Component — Primary content block 2; scope Boundaries).
- layouts ↔ a11y: this binding applies only if the leg drives the desktop operator-pause dialog. Its checklist rows bind to a11y's modal focus trap and focus restore (per layout-templates §desktop-webview → Component — Hero / signature section).

## Acceptance criteria contributions
- (layouts) `halo-hue-encoding`'s rendered report state is unchanged: `[MANUAL]` on cli and the `status-manual` checkbox glyph on desktop. No new lamp, bracket label or results-table column is introduced by the chunk (per layout-templates §cli → Component — Primary content block 2 (verdict / report-state lines)).
- (layouts) The run report for the re-drive contributes no per-check detail line for the hue grade, because the scenario declares no `[[expected]]` check (per layout-templates §cli → Component — Primary content block 2, "Per-check detail region").
- (layouts) The re-drive's harness path never blocks on an interactive prompt when stdin is not a TTY, and any refusal prints host-path-free `error:` / `hint:` or `[PRECONDITION]` lines (per layout-templates §cli → Primary screens (commands); §cli → Component — Footer / terminator + error output).
