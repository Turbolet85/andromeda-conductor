# layouts extract

## Relevance
partial — shape (a) (per-run `trace_id`/`span_id` in `conductor-emit`) changes no surface; only shape (b) (a harness-enforced minimum gap between same-seed drives) can reach the cli surface, and only if it prints an operator-visible wait or refusal. Neither the desktop-webview nor the Markdown report's layout is in scope.

## Constraints
- If shape (b) refuses a too-soon same-seed drive on the `scripts/agent-run.{sh,ps1} run --live real-model` path, layout-templates §Surface: cli → §Primary screens (the `agent-run` bullet) requires it to follow that path's existing refusal shape: an incomplete live-Pulse environment is a REFUSAL at exit 1 with one host-path-free line per unmet subject, no leg fires, and the behaviour is the same in both shells. It is not a new stage flag, not a new selector and not a sixth command (per layout-templates §cli Primary screens).
- Any new operator-facing qualifier must stay outside the lamp column. layout-templates §cli Component — Primary content block 2 requires the per-P-ID lamp set to stay closed at six, with run-level qualifiers forming a non-lamp SET (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`). A gap refusal is neither a seventh lamp nor a sixth `ReportState`; whether it reuses `[PRECONDITION]` or prints no bracket label at all is P4's decision.
- Error or refusal text goes to stderr in the `error: <short>` + detail + `hint: <fix>` shape, sanitized (no absolute host paths, no internal struct names), and stdout stays reserved for artifact data (per layout-templates §cli Component — Footer / terminator + error output; §cli IA notes → Pipe discipline).
- Headless invariant: a minimum-gap guard must never block on an interactive prompt on the agent-driven path. It waits or refuses non-interactively (per layout-templates §cli IA notes → Headless invariant; §cli Component — Hero / signature output, the `inquire` isatty gate).
- Output structure is stable because downstream agents parse it. This chunk adds no `comfy-table` column, no new verb and no change to the `conductor run` output skeleton (per layout-templates §cli IA notes → Command model; §cli Output structure — `conductor run <scenario>`).
- If a wait is shown, the honest-progress rule from layout-templates §Surface: cli applies: Expression level 0.3, an `indicatif` spinner only after about 200ms, no animate-to-100%, no cursor manipulation. Whether the harness has any existing progress line on this test-binary path is research's question.

## Patterns to follow
- The `--live` refusal precedent: the leading `conductor preconditions --for real-model-interpretation` probe refuses with `[PRECONDITION]` lines and exit 1 before any leg fires. This is the in-tree layout for "this leg cannot run now" (per layout-templates §cli Primary screens, `agent-run` and `conductor preconditions` bullets).
- The refusal-vs-skip distinction: an unmet live-Pulse condition is a refusal (non-zero), while a missing host tool is a skip at exit 0. A same-seed-too-soon condition belongs to the refusal class, never the skip class (per layout-templates §cli Primary screens, `agent-run` bullet).
- The indented-detail-line precedent: an explanation rides as an indented plain-text line under its subject, with no new column and no new ANSI or token entry (per layout-templates §cli Component — Primary content block 2, Per-check detail region).

## Anti-patterns to avoid
- Minting a new bracket label, lamp or ANSI mapping for the gap condition. The lamp set is closed at six, and run-level qualifiers reuse the Residual-mute mapping (ANSI 246) (per layout-templates §cli Component — Primary content block 2).
- Printing a host path (a runs dir, a data dir or a journal path) in a refusal or wait line (per layout-templates §cli Component — Footer / terminator + error output).
- Making the guard an interactive confirm on the headless path (per layout-templates §cli IA notes → Headless invariant).

## Contract bindings
- cli refusal output ↔ security (artifact hygiene): the "sanitized per the security plan" clause in layout-templates §cli Component — Footer / terminator + error output binds any new stderr line to security-plan §Error Handling (no host paths, no internal names).
- cli `--live` shell parity ↔ tests: the both-shells-identical-semantics clause in layout-templates §cli Primary screens (`agent-run` bullet) binds to test-plan §3. A shell-side gap guard must land in both `agent-run.sh` and `agent-run.ps1` or in neither.
- Status-never-color-alone ↔ a11y: any new line carries its ASCII text, not only colour (per layout-templates §Surface: cli, signature-placement paragraph).

## Acceptance criteria contributions
- (layouts) Only if shape (b) prints operator-visible output: a too-soon same-seed drive on `run --live real-model` is refused at a non-zero exit before any leg fires, with a host-path-free stderr line, identically in both shells (per layout-templates §cli Primary screens, `agent-run` bullet; §cli Component — Footer / terminator + error output).
- (layouts) No new bracket label, lamp, `comfy-table` column or verb is added. The per-P-ID label set stays at six and the `conductor run` output skeleton is unchanged (per layout-templates §cli Component — Primary content block 2; §cli IA notes → Command model).
- (layouts) Under shape (a), no cli, webview or report layout delta: the chunk's diff touches no rendering code (per layout-templates §Surface: cli and §Surface: desktop-webview).
