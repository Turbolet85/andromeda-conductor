# layouts extract

## Relevance
Partial — no surface, wireframe, component, or focus-order change; only the cli output-structure and pipe-discipline invariants that backend instrumentation can silently violate.

## Constraints
- Self-obs span JSON must never reach stdout: stdout is reserved for raw artifact data, human messages go to stderr, and `--agent-mode` moves the machine stream to `logs/agent-latest.jsonl` so a piped agent reads a clean stderr (per layout-templates §Surface: cli — IA notes / Pipe discipline; §Component — Footer / terminator + error output). Verified in-tree: `ObsSink::Stderr` | `ObsSink::File`, never stdout.
- cli stdout shape is a parsed contract — "output structure is stable across versions because downstream agents parse it"; adding a line or column without a flag is a breaking change. Instrumenting `execute_scenario` / `persist` / `drive_run` adds zero stdout lines (per layout-templates §Surface: cli — IA notes / Command model).
- The `conductor run` output order is fixed by the documented wireframe (header → preflight → optional `[ENVIRONMENT-SUSPECT]` → phase line → `indicatif` heartbeat → `[HOLD]` + `inquire` → verdict lines → terminator). New spans must not interpose or reorder any of it (per layout-templates §Surface: cli — Output structure `conductor run`).
- Anything the default (stderr) sink emits shares the sanitized error channel: no absolute host paths, no internal struct names, no stack traces. The new fields (`run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `row_count`) clear that bar (per layout-templates §Component — Footer / terminator + error output).
- Headless invariant holds unchanged: the agent-driven path is never gated on an interactive prompt, and instrumentation must not add a blocking or TTY-dependent step to it (per layout-templates §Surface: cli — IA notes / Headless invariant; §Component — Hero / signature output).
- desktop-webview is untouched: the frameless window is the whole surface, no routes, and the footer carries operational run state only — never diagnostics or trace state (per layout-templates §Surface: desktop-webview — IA notes; §Component — Footer (status strip)).

## Patterns to follow
- One engine, two surfaces adapted-not-forked — the plan already treats cli and desktop-webview as renderings of the same run flow, which is exactly why a single root span at the shared composition root yields cli↔Tauri parity with no per-surface fork (per layout-templates §desktop-webview IA notes — Multi-surface coordination; §cli IA notes — Multi-surface coordination).
- Run identity is already a rendered fact on both surfaces (`seed <n>` + `run_id` in the cli header/terminator, `seed 424242` + `run_id` in the webview footer / report card header). The root span's `run_id` / `seed` fields name the same facts the surfaces already label (per layout-templates §cli Component — Header / banner; §desktop-webview Component — Footer (status strip)).
- Verdict / report-state vocabulary is a closed set on every surface — the machine triad plus the three non-verdict ReportStates, always paired with text. `report.generate`'s `verdict` / `state` fields draw from that existing set rather than a parallel span vocabulary (per layout-templates §Component — Primary content block 2, both surfaces; §Decisions Log — Cross-surface IA decisions).
- The terminator (`run report → runs/<run_id>.md`) is the single closing line; `report.generate` and `db.insert_run` sit behind it at `Report::write` / `RunsDb::insert`, not beside it as new output (per layout-templates §cli Component — Footer / terminator + error output).

## Anti-patterns to avoid
- No new stdout line, caption, or bracket label from instrumentation — `[ENVIRONMENT-SUSPECT]` is the only run-level non-lamp stdout label, and the per-P-ID label set is closed at six (per layout-templates §cli Component — Primary content block 2).
- Do not surface span/trace state in the webview chrome (titlebar count, footer strip, coverage-matrix header) — those carry operational run state, not self-observation diagnostics (per layout-templates §desktop-webview Component — Header; §Component — Footer (status strip)).
- Do not let the self-obs stream default onto stdout "just for the run path" — that breaks pipe discipline for every downstream parser at once (per layout-templates §cli IA notes — Pipe discipline).

## Contract bindings
- layouts ↔ obs: obs-plan §4 owns span names and attributes; layouts owns only that the stream stays off stdout and that the documented `conductor run` / `suite` / `report` output shape is unchanged. The two sinks named in scope (`logs/agent-latest.jsonl`, `logs/conductor-tauri.jsonl`) are artifacts, not layout surfaces.
- layouts ↔ security: layout-templates §Component — Footer / terminator + error output is the layout-level statement of the redaction boundary the scope names (obs-plan §6/§11 + security-plan §Error Handling) — sanitized stderr, no host paths or struct names.
- layouts ↔ a11y: none new — this chunk adds no focusable element, no dialog, and no live-region-bearing component, so §Focus order and §Modal patterns do not engage.

## Acceptance criteria contributions
- (layouts) `conductor run <scenario>`, `conductor suite`, and `conductor report` stdout is unchanged by this chunk — no span line, no new caption, no reordering of the documented sequence (per layout-templates §Surface: cli — Output structure `conductor run` / IA notes — Command model).
- (layouts) Self-obs span JSON never appears on stdout: with `--agent-mode` it lands in `logs/agent-latest.jsonl`, otherwise on stderr; piping stdout yields raw artifact data only (per layout-templates §Surface: cli — IA notes — Pipe discipline).
- (layouts) No new span field emitted on the stderr sink contains an absolute host path, internal struct name, or stack trace (per layout-templates §Component — Footer / terminator + error output).
- (layouts) desktop-webview renders identically before/after — no new titlebar, footer, coverage-matrix, or run-report element — and the headless path is still never gated on a prompt (per layout-templates §Surface: desktop-webview — IA notes; §Surface: cli — IA notes — Headless invariant).

## Relevant amendment history
- `2026-08-09-sut-load-envelope` (§cli Output structure `conductor run` · Component — verdict / report-state lines) — a chunk added a new run-level stdout element (`[ENVIRONMENT-SUSPECT]`) that no wireframe showed; D-layout-surface fired and the caption had to be registered, plus an explicit statement that the six verdict labels are the closed per-P-ID set. Directly relevant as the standing bar: **if this chunk emits anything to cli stdout, layouts is owed an amendment** — the clean outcome here is that it emits nothing and no amendment is due.
- `2026-06-23-5-command-agent-run-harness` and `2026-06-23-line-oriented-output-rendering` (§cli Primary screens) — precedent that any genuinely-new cli surface (a verb, a stage flag, an output-mode switch) must be registered in §cli Primary screens. This chunk adds no verb and no flag, so the verb list stays at its current shape.
- `2026-08-10-pulse-run-contract` and `2026-08-09-in-lane-sut-scenarios` (§cli Primary screens — `conductor coverage` roll-up caption) — the `(N unbacked)` literal moved 11 → 10 → 9 because those chunks each named a P-ID in a scenario TOML, and D-layout-derived-count caught the stale sample. Relevant only as a guard: this chunk is instrumentation over existing values and names no new scenario, so `UNBACKED_AUTO` must **not** move and the `(9 unbacked)` caption must stay put — if P4 ends up naming a P-ID anywhere, that detector applies.
- No amendment has ever touched instrumentation, tracing, or log sinks — layout-templates has no self-observation content, which is consistent with this chunk being out of the doc's body.
