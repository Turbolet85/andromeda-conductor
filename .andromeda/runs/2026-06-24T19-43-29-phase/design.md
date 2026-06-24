# design extract

## Relevance
Partial — the chunk is CLI-surface output sanitization + agent-mode logging; design contribution is limited to error messaging format + no-color discipline, not token/branding/motion.

## Constraints
- **design-system §cli "Error output"** — `error: <short>` + contextual detail + `hint: <fix>` format; never colorized when piped; stack traces only under `--debug`/`-v`.
- **design-system §Color Palette & Anti-Patterns §Universal Bans** — never use color alone for meaning; every status must pair with ASCII prefix (e.g., `[PASS]`/`[FAIL]`) for NO_COLOR + a11y.
- **design-system §Surface: cli / Platform-Specific Notes** — respect `NO_COLOR`, `TERM=dumb`, pipe ANSI auto-stripping; ANSI 256 color codes (ANSI 117 cyan for ID tier) only when `isatty(1)`.
- **design-system §Anti-Patterns §Per-Surface Bans (cli)** — never colorize without checking `NO_COLOR` / `TERM` / pipe status; never use emoji in machine-parseable (piped) output; never wrap at arbitrary points.

## Patterns to follow
- **design-system §Surface: cli / Component Patterns §5** — headless path must check `isatty` before interactive prompts; agent-driven source-of-truth path is never blocked.
- **design-system §cli / Navigation Pattern** — linear top-to-bottom stdout; raw artifact data on stdout, human messages on stderr (§352); TTY-gated coloring.
- **design-system §cli / Tokens (platform-specific)** — ANSI color codes (114 green for nominal, 179 amber for hold, 203 red for fail, 60 violet for blocked, 117 cyan for ID tier); prefix-based state labeling (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) for color-independence.

## Anti-patterns to avoid
- NEVER rely on color alone for error severity or state; always pair with text labels or ASCII prefixes.
- NEVER emit emoji in piped (machine-parseable) output; restrict to ASCII prefixes + ANSI codes.
- NEVER colorize error messages when stderr is piped; auto-strip respecting `NO_COLOR` + `TERM=dumb`.

## Contract bindings
- **obs-plan §3** ↔ self-obs sink wiring (dual-sink selection by `--agent-mode`, JSON-only to `logs/agent-latest.jsonl` in agent mode).
- **security-plan §Error Handling** ↔ sanitization at the anyhow edge (no stack traces / absolute paths / internal struct names to stderr or artifacts).
- **a11y SC 1.4.1 (Use of Color)** ↔ state color + text label binding (every colored status carries an ASCII prefix for screen-reader + NO_COLOR friendliness).

## Acceptance criteria contributions
- **(design) Sanitized stderr never leaks host paths, struct names, or stack traces; errors render as `error: <short>` + hint** (security-plan binding).
- **(design) All piped output respects `NO_COLOR`/`TERM=dumb`; ANSI codes stripped when `!isatty(1)`** (§cli Tokens, via the existing `stdout_color()` gate).
- **(design) Error format uses ASCII prefixes (never color alone) so a11y + NO_COLOR compliance are met** (§cli Component Patterns + Anti-Patterns §Universal Bans).
- **(design) Headless `--agent-mode` path never blocks on interactive prompts; `isatty` gate overridden** (design-system §cli §Component Patterns + scope §release-gate-never-blocks invariant).

## Relevant amendment history
(none)

> Synthesis note (orchestrator): the distiller suggested `anstream`/`anstyle` for ANSI-stripping. The ACTUAL stack is owo-colors gated by ONE shared `stdout_color()` bool (`std::io::IsTerminal` + `NO_COLOR` unset + `TERM != dumb`), per the 2026-06-23 line-oriented-output-rendering chunk (session-learnings). Do NOT introduce anstream; reuse the existing gate.
