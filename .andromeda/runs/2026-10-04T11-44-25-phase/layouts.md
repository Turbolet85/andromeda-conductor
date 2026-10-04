# layouts extract

## Relevance
partial — no desktop-webview surface and no `conductor` verb output changes; the only layout-governed surface this chunk can touch is the `scripts/agent-run.{sh,ps1}` command model, and only if P4 places the scope item 4 `stub-server` `cargo check --tests` gate in the harness rather than in `.github/workflows/ci.yml` alone.

## Constraints
- Where the harness gets the `stub-server` gate, layout-templates §Surface: cli → Primary screens (`scripts/agent-run.{sh,ps1}`) requires the 5-command `boot`/`run`/`status`/`cleanup`/`logs` set to stay fixed. A new leg reaches `run` as a stage flag or a selector of one (the `--live` / `--live real-model` precedent) and never as a sixth command. Whether the existing `--unit` stage or the bundled default should carry a `cargo check --tests --features stub-server` compile step instead is a P4 placement question.
- Per layout-templates §Surface: cli → Primary screens (`scripts/agent-run.{sh,ps1}`), both shells ship at identical semantics. A gate added to one script is added to the other with the same exit-code and skip/refuse behaviour. The scope re-verified that `agent-run.sh` and `agent-run.ps1` each name `stub-server` 0 times today, so both need the change if either gets it.
- Per layout-templates §Surface: cli → Primary screens (agent-run, the skip-vs-refusal contrast), an incomplete environment is a refusal (non-zero exit) and a missing host tool is a skip (exit 0, with the `CONDUCTOR_A11Y_STRICT` strict arm). The `secret_scan_gate` "no `.git`" skip (scope item 3) is a cargo test-binary behaviour and not a harness stage, so this rule does not bind it. A harness-level `stub-server` gate needs neither skip nor refusal: compile failure is a plain gate failure.
- Per layout-templates §cli → Component — Primary navigation (verb structure) and §cli IA notes (Headless invariant), the agent-driven path never blocks on an interactive prompt. Any new harness step runs non-interactively.
- Per layout-templates §cli → Component — Footer / terminator + error output, harness-side error text goes to stderr as `error: <short>` + `hint: <fix>`. It is sanitized (no absolute host paths, no internal struct names) and never colorized when piped.

## Patterns to follow
- The `--live` / `--live real-model` selector shape (layout-templates §cli Primary screens): a new optional leg extends an existing verb's flag grammar, and an unknown selector prints a `usage:` line and exits 2 before any probe, in both shells.
- The `--e2e` verdict-reading precedent (layout-templates §cli Primary screens): it asserts the runner's PRINTED verdict rather than an exit code alone. Apply it if the gate's output is ever parsed for a pass/skip distinction.
- Pipe discipline (layout-templates §cli IA notes): ASCII prefixes only in piped output, ANSI stripped when piped, and data on stdout separate from messages on stderr.

## Anti-patterns to avoid
- Minting a sixth harness command, or a new `[...]` bracket label or lamp state, for the compile gate. The lamp set is closed at six and the run-level qualifier set is `[ENVIRONMENT-SUSPECT]` / `[PRECONDITION]` (per layout-templates §cli → Component — Primary content block 2).
- Letting the two shells diverge in exit semantics for the new step (per layout-templates §cli Primary screens, `scripts/agent-run.{sh,ps1}`).

## Contract bindings
- cli harness ↔ tests: the agent-run stage grammar here mirrors test-plan §3 (the 5-command harness), which layout-templates §cli Primary screens names as the owner of both shells' identical semantics. A harness-placed `stub-server` gate binds both documents.
- The renamed `--test real_model_harvest` targets (scope item 1) are not a layout concern. They become one only if a harness stage names a target by its old name, so that sweep belongs to tests/research.

## Acceptance criteria contributions
- (layouts) If the `stub-server` gate lands in the harness, `scripts/agent-run.sh` and `scripts/agent-run.ps1` both carry it, and the harness still exposes exactly the five commands `boot`/`run`/`status`/`cleanup`/`logs` (per layout-templates §Surface: cli → Primary screens).
- (layouts) Any new harness step runs headless with no interactive prompt, and its failure text reaches stderr as `error:`/`hint:` lines with no absolute host path (per layout-templates §cli → Component — Footer / terminator + error output).
