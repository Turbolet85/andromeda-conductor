# layouts extract

## Relevance
partial — the chunk creates and modifies NO surface (no webview region, no new cli verb, table, column or label); it only DRIVES the existing cli harness path `scripts/agent-run.sh run --live real-model`, so layout-templates binds it as a set of do-not-move guardrails on the cli surface. The desktop-webview surface is untouched and nothing is extracted from it.

## Constraints
- The series' drives enter through the existing `--live real-model` third-token selector, which the plan requires to be a selector of `--live` — neither a sixth harness command nor a new stage flag; the chunk adds no verb, flag or selector (per layout-templates §Surface: cli → Primary screens (commands), the `scripts/agent-run.{sh,ps1}` bullet).
- The plan requires that selector to lead with the non-priming `conductor preconditions --for real-model-interpretation` probe and to REFUSE at exit 1 on any unmet subject, with no leg firing; the chunk's pre-leg checks and attempt ledger must treat a refusal as a drive that did not fire, never as a graded drive (per layout-templates §Surface: cli → Primary screens (commands), the `conductor preconditions` and `scripts/agent-run.{sh,ps1}` bullets). Whether the shipped scripts behave so on this Linux host is research's question.
- Under a `--for` real-model scenario the plan requires the L4 handle to be graded for ABSENCE (unmet when either side's truthiness rule declares it); the launch posture the new contract section re-pins must be compatible with that grading (per layout-templates §Surface: cli → Primary screens (commands), the `conductor preconditions` bullet).
- The plan requires both shells to ship at identical semantics; any edit this chunk makes to `scripts/agent-run.sh` owes its `.ps1` twin (per layout-templates §Surface: cli → Primary screens (commands), the `scripts/agent-run.{sh,ps1}` bullet). Whether the chunk needs any script edit at all, and whether the `.ps1` leg is measurable on this host, are research's questions.
- The per-P-ID bracket-label set is closed at six and the run-level non-lamp qualifier set is fixed; a series verdict (`Identified` / `NotIdentified` / not graded) lives in the harvest test and the evidence ledger, and must not surface as a new bracket label, lamp or `ReportState` on cli stdout (per layout-templates §Surface: cli → Component — Primary content block 2 (verdict / report-state lines)).
- The cli output structure is required to stay stable because downstream agents parse it — no new results-table column without a format flag (per layout-templates §Surface: cli → IA notes, "Command model"; §Surface: cli → Component — Primary content block 1 (results / SLO table)).
- The headless agent-driven path is required never to block on a prompt; no step of the drive sequence may introduce an interactive gate (per layout-templates §Surface: cli → IA notes, "Headless invariant"; §Surface: cli → Component — Hero / signature output).

## Patterns to follow
- Refusal vs skip: an incomplete live-Pulse environment is a refusal (exit 1, one host-path-free `[PRECONDITION]` line per unmet subject with candidate causes), a missing host-tool handle is a skip — the pre-leg checks read the leg's outcome in those terms (per layout-templates §Surface: cli → Primary screens (commands), the `scripts/agent-run.{sh,ps1}` bullet).
- `[PRECONDITION]` has two reachable arms (per-unmet-subject lines, or the single satisfied line); a ledger row or probe keying on the bracket label must not read it as failure-only (per layout-templates §Surface: cli → Primary screens (commands), the `conductor preconditions` bullet; §Surface: cli → Component — Primary content block 2).
- Pipe discipline: raw artifact data on stdout, human messages on stderr, ANSI stripped when piped — the form in which a drive's console output is captured or quoted into evidence (per layout-templates §Surface: cli → IA notes, "Pipe discipline").
- Operator-facing error lines take the `error:` / detail / `hint:` shape on stderr, sanitized (per layout-templates §Surface: cli → Component — Footer / terminator + error output).

## Anti-patterns to avoid
- Adding a new harness command, stage flag or selector value to carry the fourth series (per layout-templates §Surface: cli → Primary screens (commands); §Surface: cli → Component — Primary navigation (verb structure)).
- Minting a bracket label, lamp state or table column for the series verdict or for the model/prompt versions graded (per layout-templates §Surface: cli → Component — Primary content block 2; §Surface: cli → IA notes, "Command model").
- Gating a drive on an interactive prompt, or letting an absolute host path reach stderr / stdout from any new line (per layout-templates §Surface: cli → IA notes, "Headless invariant"; §Surface: cli → Component — Footer / terminator + error output).

## Contract bindings
- layouts ↔ tests: the both-shells-identical-semantics requirement on `scripts/agent-run.{sh,ps1}` is stated by layout-templates as owned by test-plan §3 — the tests extract is the authority on the harness contract; this extract only binds the cli surface shape.
- layouts ↔ security: the sanitized error-output requirement (no absolute host paths, internal struct names or stack traces) is stated by layout-templates as "per the security plan" — the security extract is the authority on the redaction chain for captures.
- layouts ↔ a11y: color-never-alone on cli (every status paired with its ASCII bracket prefix, `NO_COLOR` honored) binds to the a11y plan; it applies only if the chunk emits any new status line, which the constraints above forbid.

## Acceptance criteria contributions
- The diff against the chunk base `0b07b2c` adds no clap verb, no harness command, no `run` stage flag and no `--live` selector value; every drive is launched through the existing `run --live real-model` form (per layout-templates §Surface: cli → Primary screens (commands)).
- The diff adds no bracket label, lamp state or `ReportState`, and no column to the results/SLO or coverage tables (per layout-templates §Surface: cli → Component — Primary content block 2 (verdict / report-state lines); §Surface: cli → IA notes, "Command model").
- A drive whose leading `conductor preconditions --for real-model-interpretation` probe exits non-zero is recorded in the attempt ledger as refused / not fired, with its `[PRECONDITION]` lines host-path-free, and is not counted as a graded drive (per layout-templates §Surface: cli → Primary screens (commands), the `conductor preconditions` and `scripts/agent-run.{sh,ps1}` bullets).
- If `scripts/agent-run.sh` is edited, `scripts/agent-run.ps1` carries the same semantic change in the same chunk (per layout-templates §Surface: cli → Primary screens (commands), the `scripts/agent-run.{sh,ps1}` bullet).
