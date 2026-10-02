# layouts extract

## Relevance
partial — the chunk builds no surface, region, dialog or focusable element. It touches only the cli surface's existing output contract: the `--live` harness verb that the span-subdir CARRY edits, and the verdict and per-check lines a graded round prints and writes to `runs/<run_id>.md`. Pulse's Report window and Findings counter (P-037/P-045) are SUT UI and outside this plan.

## Constraints
- The per-P-ID verdict/report-state bracket labels form a closed set of six, and so does the lamp set. A run-level qualifier (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) sits outside the lamp column and is never a seventh state. So the round's "UNGRADED" outcome for an absent sample must be expressed inside the existing set or its detail lines, never as a new label (per layout-templates §Surface: cli → Component — Primary content block 2). Which existing state a current grader maps an absent sample to is a question for research.
- In `runs/<run_id>.md`, per-check grading renders as ONE indented plain-text line per expected check: ordinal, comparison kind, own verdict, own `latency_ms`, and the deadline judged against. This adds no column, no bracket label and no ANSI or token entry. A scenario that graded nothing contributes no line (per layout-templates §Surface: cli → Component — Primary content block 2, "Per-check detail region").
- The run/suite results/SLO table has exactly 6 columns. Downstream agents parse it, so adding a column without a `--format` flag is a breaking change (per layout-templates §Surface: cli → Component — Primary content block 1; §Surface: cli → IA notes).
- `--live` is a stage flag of `agent-run run` and never a sixth command. Variants ride it as a third-token selector, and both shells ship identical semantics. Any change to the live-suite capture dir (the span-subdir CARRY) must keep that verb/flag shape in `.sh` and `.ps1` alike (per layout-templates §Surface: cli → Primary screens, `scripts/agent-run.{sh,ps1}`).
- A `Blocked` row carries its named precondition in its detail and renders its measurement fields as `—`/null, never as a red error (per layout-templates §Surface: cli → Component — Primary content block 1 / block 2).
- Headless invariant: the agent-driven path never blocks on an `inquire` prompt. A `ManualCheck` on the headless path is recorded unconfirmed (per layout-templates §Surface: cli → IA notes; §Surface: cli → Component — Hero / signature output).
- Pipe discipline: raw artifact data goes to stdout and messages go to stderr. Errors take the form `error:` + `hint:`, sanitized of host paths and struct names. ASCII prefixes are used in piped output (per layout-templates §Surface: cli → Component — Footer / terminator + error output; §Surface: cli → IA notes).

## Patterns to follow
- Indented-detail-line precedent (the `Blocked` precondition string, the `KnownResidual` note, the per-check line). A round needing to say why an assertion is ungraded or which measured value it read should reuse this shape rather than add a cell (per layout-templates §Surface: cli → Component — Primary content block 2).
- `[PRECONDITION]` both-arms caption: one ANSI-246 line per unmet subject with its causes, or the single satisfied line. It is the live suite's refusal or go signal and mints no Verdict or row (per layout-templates §Surface: cli → Primary screens, `conductor preconditions`).
- Run terminator `run report → runs/<run_id>.md` in the dim mapping, path not colorized when piped. This is the stable evidence pointer a caller can cite (per layout-templates §Surface: cli → Component — Footer / terminator + error output).
- An absence state is told apart from a failure by TEXT, never tint alone ("Not yet run" prose). Several records naming one P-ID roll up to the WORST lamp (`Fail > Blocked > Hold > Manual > Residual > Pass`) (per layout-templates §Surface: desktop-webview → Component — Primary content block 1). This applies if the round's records reach the matrix.
- The harness asserts the runner's PRINTED verdict, not its exit code alone (per layout-templates §Surface: cli → Primary screens, the `--e2e` clause). The same reading is apt for a live leg's graded output.

## Anti-patterns to avoid
- Do not mint a new bracket label, lamp state, table column or ANSI/token entry (an `[UNGRADED]` label, a "measured value" column) to carry the round's grading (per layout-templates §Surface: cli → Component — Primary content block 2; §Surface: cli → IA notes).
- Do not add a new `agent-run` command or stage flag for the round or for the span-subdir clear (per layout-templates §Surface: cli → Primary screens).
- Never render status by color alone, and never render an absent sample in a form indistinguishable from `Pass` (per layout-templates §Surface: cli, signature-placement paragraph "paired with an ASCII text prefix"; §Surface: desktop-webview → Component — Primary content block 1).

## Contract bindings
- layouts ↔ tests: the printed-verdict shape (bracket labels, per-check detail lines, `[PRECONDITION]` lines) is what a harness or graded test id asserts on. Changing it breaks parsers (§Surface: cli → IA notes, command model).
- layouts ↔ security/obs: the stderr `error:`/`hint:` output and the run-report artifact carry no absolute host paths (§Surface: cli → Component — Footer / terminator + error output). This binds the live round's evidence path and any capture-dir messaging.
- layouts ↔ design/a11y: status uses only the existing ANSI mappings (114/179/203/60/146/246/117) and is always paired with text (§Surface: cli → IA notes, multi-surface coordination).

## Acceptance criteria contributions
- The round's graded output (stdout and `runs/<run_id>.md`) uses only the six existing per-P-ID labels plus run-level qualifiers and indented per-check detail lines, with no new label, column or ANSI entry (per layout-templates §Surface: cli → Component — Primary content block 2).
- An assertion with no sample is shown as not met by its TEXT. It never renders as `[PASS]`, and its reason is carried in a detail line (per layout-templates §Surface: cli → Component — Primary content block 2; §Surface: desktop-webview → Component — Primary content block 1).
- The span-subdir CARRY leaves `agent-run.{sh,ps1}`'s command and stage-flag set unchanged, with identical semantics in both shells. An unmet live precondition still refuses with one host-path-free `[PRECONDITION]` line per subject (per layout-templates §Surface: cli → Primary screens).
- The headless live round reaches its terminator without blocking on any interactive prompt (per layout-templates §Surface: cli → IA notes, headless invariant).
