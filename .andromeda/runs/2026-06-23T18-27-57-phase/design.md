# design extract

## Relevance
Partial — the chunk implements the headless CLI harness (surface not directly rendering), but inherits design constraints on error output, state messaging, and ANSI color discipline.

## Constraints
- Per design-system §Anti-Patterns / Per-Surface Bans (cli): "NEVER use emoji in machine-parseable (piped) output; ASCII prefixes only" — status lines must use `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]` + `✓`/`✗`/`?`/`~`/`•`/`→` glyphs on TTY only, never in agent-captured artifacts.
- Per design-system §Surface: cli / Component Patterns: "Paused-count hold-point (signature, CLI mirror)" — the `boot` and `run` commands' `indicatif` spinner must **STOP (not hide)** at the hold value on operator-pause; the count does not animate-to-100% or vanish.
- Per design-system §Color Palette §Semantic Colors + §Surface: cli / Tokens: status output must pair ANSI color with ASCII text prefix; `Blocked` state surfaces distinctly (ANSI 60 violet `#565F89`, never silently collapsed into `Fail`).
- Per design-system §Anti-Patterns / Per-Surface Bans (cli): "NEVER use interactive `inquire` prompts without an `isatty` check; NEVER block the headless agent-driven source-of-truth path on a prompt" — operator-pause go/no-go **must** skip in non-TTY contexts (agent-run.sh drives it, no blocking).
- Per design-system §Surface: cli / Component Patterns: "Error output. To stderr, sanitized (no absolute host paths / internal struct names / stack traces per security plan): `error: <short>` + contextual detail + `hint: <fix>`. Never colorized when piped; stack traces only under `--debug`/`-v`."

## Patterns to follow
- Status-line formatting: color-paired ASCII prefix + P-ID in mono ID cyan (ANSI 117) + state label + timing/SLO data (design-system §Surface: cli / Component Patterns: "Verdict / report-state lines").
- Readiness gate: `boot` command carries the MCP `initialize` protocol (`2024-11-05`) check + OTLP `:4317` egress liveness, surfacing **Blocked** with named precondition string (never collapsing into Pass/Fail) — per design-system §Color Palette: "`Blocked` is a distinct slate-violet state carrying the named precondition string — present-but-greyed, never measured, never a silent downgrade to red Fail."
- Spinner state (agent-run harness): colored phase-line prefix (ANSI 114 green on-timeline, ANSI 179 amber on HOLD) + dynamic count heartbeat; on operator-pause hold, spinner STOPS at the exact value (not hidden, not animated-to-100%), embodying the frozen-count signature (design-system §Brand Identity: "the absence of motion is the signature").

## Anti-patterns to avoid
- NEVER emit colors in non-TTY contexts (piped output, agent-captured logs) — strip ANSI when `!isatty(1)` via `anstream` (design-system §Anti-Patterns: "NEVER colorize without checking `NO_COLOR` / `TERM` / pipe status").
- NEVER hide the `indicatif` spinner or animate it to completion on operator-pause — it must remain visible and frozen at the hold value (design-system §Anti-Patterns: "NEVER hide the progress spinner or animate it to 100% on the operator-pause — it must STOP in place").
- NEVER use emoji in machine-parseable output; save `?`/`~`/`→` glyphs for TTY-only paths (design-system §Anti-Patterns: "NEVER mix stdout (data) and stderr (messages) without intention; raw artifact data on stdout, human messages on stderr").

## Contract bindings
- **Status → a11y (Color-Only):** Status lines pair ANSI color with ASCII `[PASS]`/`[FAIL]`/`[BLOCKED]` prefixes, honoring `NO_COLOR` and screen-reader access (design-system §Anti-Patterns binding to a11y §Use of Color SC 1.4.1).
- **Operator-pause → motion (prefers-reduced-motion):** If future spinner motion is added, it must honor `@media (prefers-reduced-motion: reduce)` equivalently via shell/Rust env (design-system §Motion binding to a11y SC 2.3.3).
- **Harness exit codes + status → test-plan §3 + verification-harness.md:** The `boot/run/status/cleanup/logs` five-command discipline and exit-code contract are the agent's integration point (scope boundary: §Surfaces / contracts it touches).

## Acceptance criteria contributions
- (design/cli) Status output uses `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]` ASCII prefixes; colors strip cleanly when piped (`anstream`/`!isatty` gating).
- (design/cli) `boot` command surfaces **Blocked** state distinctly with named precondition (MCP or OTLP gate), never collapsing into red Fail.
- (design/cli) `indicatif` spinner on operator-pause STOPS (not hides or animates-to-100%) at the exact hold value, color-tinting nominal-green → hold-amber per phase line.
- (design/cli) Error output sanitized (no absolute paths / internal struct names), colorization stripped when piped, stack traces only under `--debug`; all errors pair short message + contextual detail + hint.

## Relevant amendment history
(none) — design-system.md is stable; amendments file shows token declaration clarification (§Tokens) on 2026-06-15, unrelated to CLI harness surface (affects desktop-webview Tailwind only).
