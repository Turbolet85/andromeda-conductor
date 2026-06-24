# layouts extract

## Relevance
Partial — the chunk modifies CLI output surfaces (error formatting, agent-mode logging), but does not change page structure, component placement, or responsive behavior.

## Constraints
- **design-system §cli "Error output"** — `error: <short>` + contextual detail + `hint: <fix>` format; never colorized when piped; stack traces only under `--debug`/`-v`.
- **layout-templates §263** — error output sanitized per security-plan: no absolute host paths, internal struct names, or stack traces in stderr or artifacts; stdout reserved for raw data.
- **layout-templates §268** — pipe discipline: raw artifact data on stdout, human messages on stderr; ANSI auto-stripped when piped; `NO_COLOR`/`TERM=dumb` honored; emoji never in machine-parseable output.
- **layout-templates §227** — MCP preflight readiness line (protocol version, tool count, canary result) uses `[OK]`/`[BLOCKED]` prefix so the gate reads without color.
- **obs-plan §3** — dual-sink self-obs wiring: stderr pretty-print in dev mode, file `logs/agent-latest.jsonl` in `--agent-mode`; `CONDUCTOR_AGENT_MODE=1` env handle visible to subscriber/sink.

## Patterns to follow
- Error messages on stderr with sanitized detail only; verb-specific help in clap output routed appropriately.
- ASCII bracket prefixes (`[PASS]`, `[HOLD]`, `[FAIL]`, `[BLOCKED]`, `[OK]`) for status indication that survives piping and `NO_COLOR`.
- Headless invariant (per layout-templates §269): interactive `inquire` prompts gated on `isatty`; agent-mode never blocks on a prompt.
- Self-obs artifact (JSON to file) kept separate from emission journal (run-report JSONL); schemas and paths distinct.

## Anti-patterns to avoid
- Do NOT colorize error output when piped; auto-strip ANSI based on `isatty` or presence of pipe.
- Do NOT emit stack traces to stderr in normal mode (only under `--debug`/`-v`).
- Do NOT mix stdout (data) and stderr (messages) — stdout strictly for artifact data.
- Do NOT leak host paths, internal struct names, or field names in sanitized errors.

## Contract bindings
- **obs-plan ↔ cli error edge:** self-obs sink selection by `--agent-mode` flag + `CONDUCTOR_AGENT_MODE=1` env; redaction enforced at subscriber layer (processor stage), not sink-only.
- **security-plan ↔ anyhow edge:** error sanitization (no stack traces / paths / struct names); `--debug` flag as the escape hatch for tracing.
- **design-system/layout-templates ↔ cli surfaces:** error format, pipe discipline, headless invariant; status prefixes color-paired with ASCII.
- **test-plan ↔ cli:** `assert_cmd` captures stderr; self-obs and emission journal asserted as distinct artifacts.
- **scripts/agent-run.{sh,ps1} ↔ cli:** `--agent-mode` flag or env triggers release-gate path (never-block proof).

## Acceptance criteria contributions
- (layouts) CLI error messages render `error: <short>` + detail + `hint: <fix>` on stderr, sanitized per layout-templates §263 (no host paths / struct names).
- (layouts) `--agent-mode` flag forces Headless resolver so operator-pause never blocks regardless of TTY (layout-templates §269 headless invariant).
- (layouts) Self-obs sink routed to file `logs/agent-latest.jsonl` in agent-mode; no pretty-print to stderr (obs-plan §3 dual-sink).
- (layouts) All status indicators (`[OK]`, `[BLOCKED]`, etc.) paired with ASCII prefixes; output survives `NO_COLOR` and piping (layout-templates §268).

## Relevant amendment history
**2026-06-23-5-command-agent-run-harness** — `conductor preflight` verb registered; §cli Primary screens. Agent-mode interacts with preflight as the readiness gate; no layout change to preflight output itself.
**2026-06-23-line-oriented-output-rendering** — `conductor coverage` verb + `report` output switch to colored table registered; agent-mode does not affect coverage or report-verb output structure.
