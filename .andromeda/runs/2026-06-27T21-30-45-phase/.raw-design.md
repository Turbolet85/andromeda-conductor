# design extract

## Relevance
partial — CLI surface only; no GUI rendering (headless chunk), so desktop-webview typography/motion/component patterns do not apply.

## Constraints
1. §Surface: cli / §Tokens: map ANSI 256 colors to semantic roles — nominal green (ANSI 114), hold amber (179), fail red (203), blocked violet (60), manual lavender (146), residual mute (246), mono ID cyan (117) (design-system §Surface: cli)
2. §Surface: cli / §Component Patterns §4: Status messages pair color with ASCII text prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]` + `✓`/`✗`/`?`/`~`/`•`/`→` on TTY; no emoji in piped output) — never color-alone (design-system §Anti-Patterns cli)
3. §Surface: cli / §Component Patterns §5: Error output to stderr uses `error:` label (reuse Fail red, ANSI 203) + `hint:` label (reuse Residual mute, ANSI 246), TTY-gated via distinct `IsTerminal` gate on stderr, ASCII labels always present (design-system-amendments §2026-06-24-sanitized-stderr-agent-mode-logging)
4. §Surface: cli / §Component Patterns §2: Operator-pause go/no-go prompts MUST guard with `isatty` check; headless agent path is NEVER blocked on an interactive prompt — decision recorded to artifact per non-interactive policy (design-system §Surface: cli)
5. §Brand Identity: mission-control personality — calm, resolute, terse status callouts with no alarm in tone even on Fail verdicts (design-system §Brand Identity)
6. §Surface: cli / §Anti-Patterns: detect terminal width dynamically for `comfy-table` output; never hardcode widths or wrap at arbitrary points (design-system §Anti-Patterns cli)
7. §Surface: cli / §Anti-Patterns: respect `NO_COLOR`, `TERM=dumb`, and piped-stdout ANSI stripping; raw artifact data on stdout (parseable), human messages on stderr (design-system §Anti-Patterns cli)

## Patterns to follow
1. Status prefixes (prefix text + color pair) — `[PASS]` green, `[HOLD]` amber, `[FAIL]` red, `[BLOCKED]` violet, `[MANUAL]` lavender, `[RESIDUAL]` mute; never color-alone (design-system §Surface: cli §4 + §Anti-Patterns cli)
2. ANSI color bindings fixed per palette (114→nominal, 179→hold, 203→fail, 60→blocked, 146→manual, 246→residual, 117→ID cyan); no custom ANSI codes (design-system §Surface: cli / §Tokens)
3. TTY-gated output (anstream auto-strips ANSI when piped, honors NO_COLOR on stderr independently) (design-system §Surface: cli / Platform-Specific Notes)
4. `indicatif` spinner halts in place on operator-pause (never hides, never animates-to-100%) — the paused-count signature (design-system §Surface: cli §1)

## Anti-patterns to avoid
1. Do not mix stdout (raw data) and stderr (human messages) without intention; never emit machine-parseable output to stderr (design-system §Anti-Patterns cli)
2. Do not use interactive `inquire` prompts without an `isatty` check; the headless agent path blocks on no prompt (design-system §Anti-Patterns cli + §Surface: cli §2)
3. Do not hardcode terminal widths or arbitrary wrap points — detect width dynamically; do not rely on color alone (design-system §Anti-Patterns cli)

## Contract bindings
- CLI error output (sanitization: no host paths / struct names / stack traces) binds to security plan; calm-under-load tone (no flashing red, no alarm on Fail) binds to Brand Identity mission-control personality (design-system §Brand Identity + §Surface: cli / Platform-Specific Notes)

## Acceptance criteria contributions
1. (design) CLI output uses only ANSI colors from the defined palette (114/179/203/60/146/246/117), never hardcoded ANSI codes or defaults.
2. (design) Every status message and error message pairs color with ASCII text prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`error:`/`hint:` + TTY-gated glyphs), never color-alone.
3. (design) Interactive prompts guard with `isatty` check; headless agent path records decision to artifact without blocking on input.
4. (design) Error output respects `NO_COLOR`, `TERM=dumb`, and piped-output ANSI stripping; stderr has distinct `IsTerminal` gate from stdout.

## Relevant amendment history
- 2026-06-24-sanitized-stderr-agent-mode-logging: error:/hint: labels reuse Fail red (ANSI 203) and Residual mute (ANSI 246) on stderr with distinct TTY gate; ASCII labels always present (no color-alone on error edge) — applies to CLI error output surface in this headless chunk (design-system-amendments §2026-06-24-sanitized-stderr-agent-mode-logging).
