# design extract

## Relevance
partial — design system applies only to the runtime error-surface display (verdict/report-state color+label bindings); the port-occupier is pure infrastructure (no UI).

## Constraints
- Per design-system §Color Palette, bind-failure surfaces a typed error condition (verdict/error wall) rendering with `--status-fail` (`#F85149` dark / `#CF222E` light) + muted text label (never color alone — Color Palette §Semantic Colors "Error" row); paired with text "Could not bind" (design-system §Iconography "icons clarify, not decorate").
- Per design-system §Iconography + §Component Patterns #4, any error-state display from this fault uses the custom lamp glyph system or a text-only prefix (`✗`/`[FAIL]`), not a bare red dot (state-color + label binding from a11y §Use of Color SC 1.4.1).
- Per design-system §Anti-Patterns "Conflating no-result-yet with failed," if the occupier is not active yet, render as neutral (not error-red); only active bind-failure gets the fail semantic color.

## Patterns to follow
- Design-system §Verdict/report-state lamp: any runtime error reporting this fault re-uses the fail-state glyph (filled red dot `--status-fail`) + text label `Bind failed` in error red, not a banner or toast (design-system §Brand Identity "calm under load, no alarm").
- Design-system §Surface: cli §Component Patterns #4: CLI error output on stderr, colorized via `owo-colors` ANSI 203 (fail red), prefixed `✗` + text (TTY-gated), never emoji in piped output.

## Anti-patterns to avoid
- NEVER emit the bind error as a flashing/pulsing alert or native OS toast (design-system §Anti-Patterns "flashing / pulsing red banner").
- NEVER show the error in color alone — always pair error color with text prefix (`✗` CLI / `[FAIL]` log) or a label (desktop-webview verdict lamp).

## Contract bindings
a11y §Use of Color SC 1.4.1 (state color + label mandatory, not color-alone) · CLI §§Component Patterns + Platform-Specific Notes (ANSI gating + TTY checks + piped output sanitization).

## Acceptance criteria contributions
- (design) Bind-failure error surfaces in `--status-fail` color with an accompanying text label (not color-alone) per a11y §Use of Color SC 1.4.1.
- (design) CLI error output respects `NO_COLOR`, `TERM=dumb`, and piped-stdout ANSI stripping; prefixed `✗` (TTY) or `[FAIL]` (logs), never emoji in machine-parseable output.
- (design) No animated alerts, toasts, or flashing on bind-failure (expression level 0.3 / calm-under-load discipline).

## Relevant amendment history
(none) — design-system has been stable since 2026-06-14; amendment 2026-06-15 was token-rendering only (Tailwind v4 `:root` fix), not related to fault surface or error-display patterns.
