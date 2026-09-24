# layouts extract

## Relevance
Partial. The chunk adds no webview region, dialog, focusable element, breakpoint or empty state. The only layout surface it can touch is the cli harness entry, `scripts/agent-run.{sh,ps1}`, and only if the scanner gets a local harness entry (scope item A.2 / "Surfaces touched", marked `[inferred]`). `ci.yml`, `.gitignore` and `.claude/settings.json` are outside layout-templates.

## Constraints
- The layout plan names the harness as a closed 5-command set (`boot`/`run`/`status`/`cleanup`/`logs`) with run stage flags `--unit`/`--integration`/`--e2e`/`--live`, default = the full bundled gate (per layout-templates §Surface: cli — Primary screens, the `scripts/agent-run.{sh,ps1}` bullet). A local scanner entry must fit into this set. Either it joins the bundled default gate, or it is a deliberately registered addition. It must not appear silently as a sixth command or a fifth stage flag. Whether the default gate already runs any static scan leg is a question for research.
- Both shells must ship at identical semantics. Parity is a property of the scripts together with the invoking environment (per layout-templates §Surface: cli — Primary screens, the `agent-run` bullet, citing test-plan §3). Any scanner entry lands in both `.sh` and `.ps1`.
- Hit and error output goes to stderr, sanitized: no absolute host paths, no internal struct names, no stack traces. The shape is `error: <short>` + detail + `hint: <fix>` (per layout-templates §Surface: cli — Component — Footer / terminator + error output). This matches the scope's "file + line, never the matched secret echoed back" rule. Hit locations should be repo-relative.
- Pipe discipline: ANSI is stripped when piped, `NO_COLOR` / `TERM=dumb` are honored, no emoji in machine-parseable output, and ASCII prefixes only there (per layout-templates §Surface: cli — IA notes, "Pipe discipline").
- Headless invariant: the agent-driven source-of-truth path must never block on a prompt (per layout-templates §Surface: cli — IA notes, "Headless invariant"). The scanner and any allowlist flow must be non-interactive.
- The `conductor` verb set is whatever `Commands` declares, and it is enumerated only in §Primary screens (per layout-templates §Surface: cli — Component — Primary navigation). The scope puts the scanner under `scripts/`, not in `conductor-cli`, so no verb enumeration should change. A scanner implemented as a `conductor` verb would be a layout-surface change.

## Patterns to follow
- Refuse and skip are separate outcome classes in the harness. A refusal is a non-zero exit with one host-path-free line per cause. A skip is exit 0 with a precondition `error:`/`hint:`. The strict CI arm exits non-zero where the lax local arm skips (per layout-templates §Surface: cli — Primary screens, `agent-run` bullet, the `--live` / `CONDUCTOR_A11Y_STRICT` arms). A secret hit is a hard failure on both arms, so it must not be modeled as a skip.
- Assert the printed verdict, not just the exit code (per layout-templates §Surface: cli — Primary screens, the `--e2e` clause). This fits the scope's seeded known-bad input that proves the step can fail (items A.1 / B.7).
- The bracket-label set is closed: six per-P-ID lamps plus a named run-level non-lamp qualifier SET (per layout-templates §Surface: cli — Component — Primary content block 2). Scanner output is harness/CI output, not a Conductor run artifact. It should not reuse `[PASS]`/`[FAIL]`/`[BLOCKED]` in a way that reads as a per-P-ID verdict.

## Anti-patterns to avoid
- Adding a harness command or stage flag without registering it on the cli surface. Every prior harness addition (`--live`, the `real-model` selector, `.ps1`) was recorded because an unregistered stage "was an undocumented cli-surface region" (per layout-templates §Surface: cli — Primary screens; see amendments 2026-09-06, 2026-09-22).
- Echoing matched secret text or absolute host paths in stderr or CI logs (per layout-templates §Surface: cli — Component — Footer / terminator + error output).
- Baking a literal count into any documented sample, for example "0 hits over N files". The plan and its history de-literalize derived counts to set-naming or placeholder forms (per layout-templates §Surface: cli — Primary screens, the `conductor coverage` "never a literal" rule).

## Contract bindings
- layouts ↔ tests: the harness stage set and shell parity cite test-plan §3 (per layout-templates §Surface: cli — Primary screens, `agent-run` bullet). A scanner leg in the harness binds to test-plan's stage/parity definition.
- layouts ↔ security: stderr sanitization and "security guardrails honored at the layout level" (per layout-templates §Decisions Log, the security-guardrails bullet and §Surface: cli — Footer error output). The rules for what may be printed come from security-plan §Secret Management.

## Acceptance criteria contributions
- (layouts) If the scanner is reachable through `scripts/agent-run.{sh,ps1}`, both shells expose it with identical invocation and exit semantics. It is part of the bundled default gate or a registered addition, never an unregistered sixth command or stage flag (per layout-templates §Surface: cli — Primary screens, `agent-run` bullet).
- (layouts) On the seeded known-bad input, the scanner exits non-zero and prints to stderr one line per hit naming a repo-relative file and line number. It never prints the matched secret in full or any absolute host path (per layout-templates §Surface: cli — Component — Footer / terminator + error output).
- (layouts) Scanner output piped or under `NO_COLOR=1` contains no ANSI escape sequences and no emoji, and the run never waits on stdin (per layout-templates §Surface: cli — IA notes, Pipe discipline + Headless invariant).

## Relevant amendment history
- 2026-06-23-5-command-agent-run-harness: registered the 5-command `agent-run.sh` set and the `run` stage flags. This is the origin of the closed harness set a scanner entry must fit.
- 2026-09-01-webview-self-verify-windows-host: named `.ps1` alongside `.sh` because test-plan §3 requires parity. It stated the stage's target and guard shape rather than a command literal so harness edits would not make it stale.
- 2026-09-06-operator-gated-live-suite: `--live` was registered because an added stage was "an undocumented cli-surface region". It also fixed the refuse-vs-skip distinction.
- 2026-09-07-a11y-ci-gate: the nearest precedent, a CI gate chunk. It added the strict CI arm (`CONDUCTOR_A11Y_STRICT` exits non-zero instead of skipping) and the printed-verdict assertion, because an exit code alone cannot tell a pass from a total skip. Both sentences on the same line were amended in one edit to avoid a leftover stale claim on that line.
- 2026-09-07-sr-findings-fixed: the shell-parity claim was conditioned on the invoking environment.
- 2026-09-22-interpretation-proven-live: the `--live real-model` selector was recorded as "neither a sixth command nor a new stage flag". This is how the plan sizes additions to the harness set. An unknown selector exits 2 before any probe, in both shells.
