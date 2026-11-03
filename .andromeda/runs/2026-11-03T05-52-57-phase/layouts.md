# layouts extract

## Relevance
partial: no surface is created or re-laid. The chunk removes `scripts/agent-run.ps1`, which the cli surface's harness entry names as a twin of `scripts/agent-run.sh`. The new Linux CI job drives that harness headless.

## Constraints
- layout-templates §cli Primary screens (sec-wrso54gt, fct-4artks5b) requires `scripts/agent-run.sh` to stay the headless harness with 5 commands (`boot`/`run`/`status`/`cleanup`/`logs`). `run` takes the `--unit`/`--integration`/`--live` stage flags and defaults to the full bundled gate. Removing the `.ps1` twin must leave that verb set, those flags and the default unchanged in the remaining shell. The plan's wording "both shells ship at identical semantics" and the `.ps1` preflight measurement are plan text. They are re-stated by the wrap's amendment, never by a phase edit.
- layout-templates §cli Primary screens (sec-wrso54gt, fct-4artks5b) requires `--live` (and `--live real-model`) to be reachable from no default or CI path. The re-stated CI step `agent-run.sh run` must invoke the default bundled gate, never a `--live` selector.
- layout-templates §IA notes (sec-sl4gwnpv, headless invariant) requires every `inquire` prompt to check `isatty` first, so the agent-driven path never blocks on a prompt. The Linux runner's `agent-run.sh run` leg is that headless path. Whether the code already skips every prompt under a non-TTY Linux runner (as it did under `pwsh`) is research's question.
- layout-templates §Component — Header / banner (sec-5ddlfnbu) requires all styling to be gated by `std::io::IsTerminal` per stream, honouring `NO_COLOR` / `TERM=dumb`, so ANSI strips when piped. The Linux runner's environment (TERM, TTY state) may differ from the Windows runner's. Whether CI logs and uploaded artifacts stay ANSI-free there is research's question.
- layout-templates §Component — Primary navigation (sec-nhh5rksa) requires pipe discipline: raw artifact data on stdout, human messages on stderr. A CI step that changes from `pwsh` to `bash` must keep consuming those streams as they are, with no merged redirect that mixes them for a downstream parser.
- layout-templates §Component — Footer / terminator + error output (sec-u5stqm6j) requires `error:` / `hint:` output on stderr, sanitized (no absolute host paths). Any harness message re-stated for Linux (e.g. a path or usage line formerly shaped for Windows) must keep this.
- layout-templates §IA notes (sec-sl4gwnpv, command model) requires the output structure to stay stable across versions, because downstream agents parse it. Changing the runner OS must not change any table shape, bracket label or verdict line.

## Patterns to follow
- Retired-leg precedent, per layout-templates §cli Primary screens (sec-wrso54gt, fct-4artks5b): when the `--e2e` webview stage left with the window, its token fell to the unknown-token `usage:` exit 2. A removed leg adds no new verb or flag. It falls to the existing usage refusal.
- Retired-surface note precedent, per layout-templates §Surface: desktop-webview (sec-cz7jf7ht): a retired thing gets a one-line retirement record naming the entry and the last commit holding it. This is the shape for the wrap's amendment that drops the `.ps1` twin from the harness entry.
- Selector refusal before any probe, per layout-templates §cli Primary screens (sec-wrso54gt, fct-4artks5b): an unknown `--live` selector prints `usage: <script> run --live [real-model]` and exits 2. The plan states this "in both shells"; after this chunk it must hold in the one shell.

## Anti-patterns to avoid
- Adding a sixth harness command or a new stage flag to absorb a CI need. Banned per layout-templates §cli Primary screens (sec-wrso54gt, fct-4artks5b).
- An interactive gate on the CI path. Banned per layout-templates §IA notes (sec-sl4gwnpv, headless invariant): it would silently break the release gate.
- Colorized output when piped (e.g. forcing color in the CI step's environment). Banned per layout-templates §IA notes (sec-sl4gwnpv, pipe discipline).

## Contract bindings
- layouts ↔ tests: layout-templates §cli Primary screens (sec-wrso54gt, fct-4artks5b) cites test-plan §3 for the twin shells' identical semantics. Retiring `agent-run.ps1` moves both plans to one shell, by the wrap's amendments.
- layouts ↔ a11y: the bracket-prefix / `NO_COLOR` requirement in layout-templates §Surface: cli (sec-jzpiklrh, fct-4dp57me6) is what a11y's NO_COLOR compliance derives from. CI's non-TTY Linux logs exercise it.
- layouts ↔ obs / CI: the headless invariant (layout-templates §IA notes, sec-sl4gwnpv) binds to the CI job's `agent-run.sh run` step and to the run-journal conformance gate that reads its output.

## Acceptance criteria contributions
- (layouts) The Linux CI job runs `scripts/agent-run.sh run` with no `--live` token, and the step finishes without blocking on any prompt (per layout-templates §cli Primary screens (sec-wrso54gt) and §IA notes (sec-sl4gwnpv)).
- (layouts) `scripts/agent-run.sh run --live bogus` prints `usage: <script> run --live [real-model]` and exits 2 before any probe. `scripts/agent-run.sh run --e2e` exits 2 with a `usage:` line (per layout-templates §cli Primary screens (sec-wrso54gt)).
- (layouts) `scripts/agent-run.sh` still exposes exactly `boot`/`run`/`status`/`cleanup`/`logs`, and no reader in crates/ or scripts/ names `agent-run.ps1` except in a comment that names history (per layout-templates §cli Primary screens (sec-wrso54gt)).
- (layouts) The `agent-run.sh run` output in the CI log carries no ANSI escape sequences (per layout-templates §Component — Header / banner (sec-5ddlfnbu)).
