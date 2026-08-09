# layouts extract

## Relevance
Partial — the check itself is an offline core artifact-vs-artifact assertion with no surface, but the scope leaves open (P4) whether it is reachable from the cli/agent-run harness; if it is, the cli surface rules bind, plus the standing manifest-sourced-label invariant.

## Constraints
- If the check gets any cli entrypoint, it must be a flat verb-noun verb one level deep with verb-prefixed help, registered in the verb list alongside `run|suite|report|preflight|coverage` — no sub-subcommands, no separate man/wiki surface (per layout-templates.md §Surface: cli — Component: Primary navigation (verb structure) + §Primary screens (commands)).
- Any drift status printed must pair its color with an ASCII bracket prefix (`[PASS]`/`[BLOCKED]`/`[FAIL]`) so the signal survives `NO_COLOR`, piping and screen readers — never color-alone (per §Surface: cli signature-placement preamble + §cli IA notes Pipe discipline).
- Pipe discipline: raw parseable artifact data on stdout, human messages on stderr, ANSI auto-stripped when `!isatty(1)`, emoji never in machine-parseable output (per §cli IA notes Pipe discipline).
- Drift failure text follows the fixed error shape — `error: <short>` + contextual detail + `hint: <fix>` on stderr, sanitized of absolute host paths / internal struct names / stack traces (traces only under `--debug`/`-v`) — which is exactly the shape the chunk's "name the ids + `sut_version`/`captured_at`, what is owed" message needs (per §Surface: cli — Component: Footer / terminator + error output).
- No emitted label, caption or header may name a wider or narrower span than the manifest's accepted set — the `P-001..P-060` literals were removed and are manifest-sourced (per §Surface: cli `conductor suite` header + §desktop-webview coverage header strip; amended 2026-08-08).
- Output structure is a stability contract because downstream agents parse it: adding a `comfy-table` column without a `--format` flag is a breaking change, so drift information belongs in a row/detail/caption, never a new column (per §cli IA notes Command model).
- Headless invariant: the check must never gate on an interactive `inquire` prompt; `isatty` is checked first and the agent-driven path is never blocked — a drift gate that prompts would silently break the release gate (per §cli IA notes Headless invariant + §Component: Hero / signature output).

## Patterns to follow
- The `conductor preflight [--json]` readiness-gate shape — emits structured JSON or a single `[PASS]`/`[BLOCKED]` line and exits 0 iff ready, non-zero on a failed precondition — is the plan's existing template for an offline gate; a surfaced drift check should mirror it rather than invent an output shape (§cli Primary screens).
- The `Blocked` detail pattern: the status line carries a **named precondition string** on an indented detail line beneath it, with measurement columns rendering `—`/null and never a red error — the same "line + named detail beneath" shape the drifted-id list should take (§cli Component: Primary content block 1 + block 2).
- Summary-caption roll-up: tallies live in the `comfy-table` caption / footer strip, each count paired with its label, never as extra columns — where a drift count belongs if surfaced (§cli Component: Primary content block 1; §desktop-webview Component: Footer (status strip)).
- Terminator line: a single closing dimmed line naming the artifact destination, left un-colorized when piped (§cli Component: Footer / terminator).
- Desktop mirror if it ever surfaces in the webview: manifest-derived counts live in the coverage-matrix **header strip** ("N loaded / N measured"), not a new region (§Wireframe — Run console (idle) + §Component: Primary content block 1).

## Anti-patterns to avoid
- Do not render drift with the verdict triad or a verdict lamp / `P-###  Pass|Fail` line — the status tier maps 1:1 to typed run outcomes, and a harness-side drift failure is neither a `Verdict` nor a `ReportState` (§Decisions Log — Cross-surface IA decisions + §cli Component: Primary content block 2). This is the layout-side form of the chunk's verdict/error-wall invariant.
- No KPI-card / dashboard widget or new panel for drift — the coverage surface is an explicitly Rejected-Default-free dense list, "a control surface, not a dashboard" (§Component: Primary content block 1 + §Decisions Log Key layout choices).
- No blink/flash, no full-screen redraw or cursor manipulation for a drift banner; ratatui-style live redraw is deliberately omitted (§Surface: cli Expression level + §Decisions Log Motion trigger placement).

## Contract bindings
- **layouts ↔ a11y**: the ASCII-prefix pairing rule is where a11y derives `NO_COLOR` / screen-reader conformance — layouts owns the placement, a11y owns the conformance claim (§cli signature preamble; focus-guide cross-domain bindings).
- **layouts ↔ design**: any color on a drift line is referenced by token-name mapping only (`count-blocked` ↔ ANSI 60, `color-id-cyan` ↔ ANSI 117, journal-text ↔ ANSI 146); the design plan owns the values, adapted not forked (§cli IA notes Multi-surface coordination).
- **layouts ↔ tests/harness**: cli output structure is parsed by `scripts/agent-run.sh` and asserted in `crates/conductor-cli/tests/cli_smoke.rs`; any surfaced drift line becomes part of that parsed contract (§cli IA notes Command model + Headless invariant).
- If P4 decides the check is **test-only**, all three bindings are inert and layouts has no involvement in this chunk.

## Acceptance criteria contributions
- (layouts) No hardcoded `P-001..P-060` or any literal id-range/count appears in any label, caption or message this chunk emits; spans read from the manifest's accepted set (layout-templates §cli `conductor suite` header, §desktop coverage header strip).
- (layouts) If surfaced on cli: the drift failure prints to **stderr** in the `error: <short>` / detail / `hint: <fix>` shape, sanitized (no absolute host paths, no internal struct names, no stack trace without `--debug`), with stdout left reserved for parseable data (layout-templates §cli Component: Footer / terminator + error output).
- (layouts) If surfaced on cli: every drift status token pairs its color with an ASCII bracket prefix and the message is fully legible under `NO_COLOR` and when piped (layout-templates §cli IA notes Pipe discipline).
- (layouts) The chunk adds no new `comfy-table` column, no new cli verb level, and no new webview region; if a new verb is added it is flat verb-noun and registered in §cli Primary screens (layout-templates §cli IA notes Command model + Primary navigation).

## Relevant amendment history
- **2026-08-08-sut-capability-manifest — De-hardcoded cli coverage range labels** (§desktop-webview coverage header strip, §cli `conductor suite` header / Primary screens): the `P-001..P-060` range labels and the "all 60 rows" virtual-scroll note were replaced with the manifest's accepted set, because a label must not name a span wider or narrower than the rendered, manifest-sourced row set. This is the immediately-prior chunk and the direct reason this chunk must not reintroduce a literal range anywhere it prints.
- **2026-06-23-5-command-agent-run-harness — `conductor preflight` verb registered** (§cli Primary screens): established the precedent that a genuinely-new gate verb (exits 0 iff ready, non-zero on a Blocked precondition) is registered in the verb list as a routine D-layout-surface amendment, with no cascade. This is the template if P4 surfaces the drift check as a verb.
- **2026-06-23-line-oriented-output-rendering — `conductor coverage [--write]` verb registered** (§cli Primary screens, tooling context): cli verb surface 4→5 for the verb that renders the coverage classification; a drift check attached to that verb's output would amend the same section, and the amendment also fixed a stale `report` output description — the same staleness risk applies if drift output changes an existing verb's shape.
