# design extract

## Relevance
Partial — chunk is CLI-centric headless work with logic/MCP plumbing as core; design-system's CLI surface patterns apply to output formatting + error handling, but no new CLI surfaces are introduced.

## Constraints
- Emit error/verdict output to stderr with ASCII prefixes (`error:`/`hint:` / `[PASS]`/`[FAIL]`/`[BLOCKED]`) paired with color, never color-alone (design-system §Surface: cli, Tokens; amended D-design-tokens 2026-06-24-sanitized-stderr-agent-mode-logging).
- Interactive prompts (operator-pause go/no-go) MUST check `isatty()` first; the headless `--agent-mode` path is NEVER blocked on a prompt (design-system §Surface: cli, Component Patterns §2 + Platform-Specific Notes).
- Report envelope (JSONL journal + runs.db + Markdown) routes verdict-first states (`Pass`/`Fail`/`ManualCheck`/`KnownResidual`/`Blocked`) using semantic colors from the §Color Palette (design-system §Surface: cli, Component Patterns §4).
- Spinners/progress indicators must STOP in place (not hide or animate-to-100%) on operator-pause, preserving the paused-count signature (design-system §Motion, amended D-component-primitives 2026-06-26).
- Sanitize all error artifacts: no absolute host paths / internal struct names / stack traces in normal mode; `--debug`/`-v` only (design-system §Surface: cli, Component Patterns §5).

## Patterns to follow
- Verdict/report-state output: prefix + P-ID (mono cyan ANSI 117) + state label + latency (design-system §Surface: cli, Component Patterns §4 → `✓ P-009 Pass 1840ms` / `✗ P-014 Fail` / `• P-022 Blocked`).
- Error output to stderr: `error: <short>` (Fail red ANSI 203) + contextual detail + `hint: <fix>` (Residual mute ANSI 246), TTY-gated via stderr-specific `IsTerminal` check (design-system §Surface: cli, Component Patterns §5, amended 2026-06-24-sanitized-stderr-agent-mode-logging).
- Canary/preflight readiness output: reuse paused-count hold-point signature (count display + phase-line state) if live polling occurs, honoring `prefers-reduced-motion: reduce` (design-system §Motion, Component Patterns §1).

## Anti-patterns to avoid
- NEVER use emoji in machine-parseable piped output; ASCII prefixes + ANSI color only (design-system §Surface: cli, Per-Surface Bans).
- NEVER block the headless agent-driven `--agent-mode` path on an interactive `inquire` prompt; the non-TTY path must skip and record the decision (design-system §Surface: cli, Platform-Specific Notes).
- NEVER output stack traces or internal struct names in normal mode (design-system §Surface: cli, Per-Surface Bans; amended 2026-06-24-sanitized-stderr-agent-mode-logging).

## Contract bindings
CLI verdict/error output ↔ design-system §Color Palette (semantic colors for `Pass`/`CalibrationRegion`/`Fail`/`Blocked`/`ManualCheck`/`KnownResidual`, never color-alone) · Headless path `isatty` gate ↔ design-system §Surface: cli Platform-Specific Notes (prevent release-gate breakage on interactive prompts) · Error output formatting ↔ a11y (NO_COLOR + screen-reader compliance via ASCII `error:`/`hint:` prefixes, not color labels).

## Acceptance criteria contributions
- (design) CLI verdict/error output uses only design-system color tokens (ANSI 203/114/179/246/117 mapped to `--status-fail`/`--count-nominal`/`--count-hold`/`--status-residual`/`--color-id-cyan`), paired with ASCII prefixes (`[PASS]`/`[FAIL]`/`[BLOCKED]`) — never color-alone (design-system §Color Palette).
- (design) Preflight readiness output (canary round-trip progress/result) respects `prefers-reduced-motion: reduce` and `NO_COLOR`; interactive prompts check `isatty()` before firing (design-system §Motion / §Surface: cli).
- (design) Report artifacts (JSONL journal + Markdown) route verdict states via design-system semantic colors + text labels; error handling applies sanitized `error:` + `hint:` pattern with tty-gating per stderr `IsTerminal` (design-system §Surface: cli Component Patterns §4/§5).

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** — error output (`error:` ANSI 203 / `hint:` ANSI 246) ties to this chunk's `--agent-mode` flag and headless artifact production (Blocked-row NULL rule, stderr sanitization). Instructs reuse of existing Residual-mute token, no new palette entry.
- **2026-06-24-paused-count-hold-point-signature** — registered `--motion-heartbeat: 1600ms` token; if the chunk's preflight/canary polling shows a live spinner, the motion/halt signature applies (value-ticking ties to live Epoch-9 counter).
- **2026-06-26-component-primitives-library** — operator-pause dialog fade reconciled to `--motion-micro` (150ms); relevant if the headless path triggers a paused-count hold via MCP async wait (though likely not in this chunk's scope, it documents the binding).
