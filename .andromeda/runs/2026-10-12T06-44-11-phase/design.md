# design extract

## Relevance
partial — the chunk removes panel-shaped types (no design surface left to them: per design-system §Surface: desktop-webview the window's tokens/components are retired), but its second half gives four functions new command-line entries (catalogue listing, graceful mid-run stop, past-run load-envelope standing, hold `[[checklist]]` items), and those render on the one live surface, §Surface: cli.

## Constraints
- Any new or extended cli output that carries status pairs color with its ASCII text prefix from the closed per-P-ID set (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`), never color alone; ANSI applies only behind the per-stream `std::io::IsTerminal` gate with `NO_COLOR` unset and `TERM != dumb` (per design-system §Surface: cli → Tokens (platform-specific)).
- A past run's load-envelope standing, printed from the command line, uses the run-level `[ENVIRONMENT-SUSPECT]` caption in ANSI 246, printed once and outside the lamp column — a qualifier on the run, never a seventh lamp or a sixth `ReportState` (per design-system §Surface: cli → Component Patterns, pattern 4; the caption SET is fixed in §Surface: cli → Tokens (platform-specific)). Whether `report` already prints this caption for a past run is research's question.
- Hold `[[checklist]]` items printed at the CLI hold follow the `ManualCheck` treatment: ANSI 146 lavender, the `[MANUAL]` prefix and the `?` glyph (TTY only); `inquire` y/n when interactive, recorded unconfirmed when headless — never in the green/amber/red triad (per design-system §Surface: cli → Component Patterns, pattern 4; the Manual row in §Color Palette → Semantic Colors).
- A graceful mid-run stop keeps the signature: the `indicatif` spinner STOPS in place at its exact value, never hides or animates to 100%; on abort the count stops and dims to blocked slate-violet (ANSI 60) (per design-system §Brand Identity, Signature element; per design-system §Surface: cli → Component Patterns, pattern 1).
- A catalogue listing renders as a dense one-row-per-entry `comfy-table` sized from the detected terminal width — never hardcoded widths, never arbitrary wraps; IDs (P-IDs, run_id) in ANSI 117 mono ID cyan; like `conductor coverage` (a separate, narrower table) it carries no verdict/state column unless it shows a state (per design-system §Surface: cli → Component Patterns, pattern 3; per design-system §Surface: cli → Platform-Specific Notes).
- Raw artifact data goes on stdout and human messages on stderr; a new verb or flag that refuses (e.g. a stop with no run in progress, an unknown past run) prints `error: <short>` + `hint: <fix>` to stderr, sanitized, with `error:` in ANSI 203 and `hint:` in ANSI 246, each gated on stderr's own `IsTerminal` (per design-system §Surface: cli → Component Patterns, pattern 5; per design-system §Anti-Patterns → Per-Surface Bans, cli).
- The headless agent-driven path never blocks on a prompt: any operator confirm a new stop or checklist entry adds is `isatty`-gated, and the non-interactive decision is recorded to the artifact (per design-system §Surface: cli → Component Patterns, pattern 2).

## Patterns to follow
- The coverage-matrix table (`conductor coverage`: P-ID · Title · Category · Mode, no bracket prefix, ending in a roll-up caption) is the nearest pattern for a catalogue listing verb (per design-system §Surface: cli → Component Patterns, pattern 3).
- The `ManualCheck` verdict line `? P-035  Manual  …` (ANSI 146, `[MANUAL]`) is the pattern for printing a hold's checklist items (per design-system §Surface: cli → Component Patterns, pattern 4).
- The run-level `[ENVIRONMENT-SUSPECT]` caption above the verdict lines is the pattern for a past run's envelope standing (per design-system §Surface: cli → Component Patterns, pattern 4).
- Output stays linear top-to-bottom stdout — no cursor manipulation or full-screen redraw — for any new verb or flag (per design-system §Surface: cli → Navigation Pattern).

## Anti-patterns to avoid
- Showing a stopped or never-reached check as a red error: a stop that leaves checks unmeasured renders them `Blocked` slate-violet (or `—` measurement cells), never collapsed into `Fail` (per design-system §Anti-Patterns → Rejected Defaults, "Conflating 'no result yet' with 'failed'").
- Hiding the spinner, animating it to 100%, or flashing/pulsing on a stop or abort (per design-system §Anti-Patterns → Rejected Defaults; per design-system §Anti-Patterns → Per-Surface Bans, cli).
- Emoji in piped output, or a new color outside the ANSI token map — every color a new entry prints must trace to a state in the map (per design-system §Anti-Patterns → Universal Bans, "NEVER use color purely for decoration"; per design-system §Anti-Patterns → Per-Surface Bans, cli).

## Contract bindings
- design ↔ security: the `error:`/`hint:` output of any new verb or flag must hold the no-host-path / no-struct-name / no-stack-trace sanitization (design-system §Surface: cli → Component Patterns, pattern 5 ↔ security-plan §Error Handling).
- design ↔ layouts: a catalogue listing verb's screen and the coverage roll-up caption live in layout-templates' cli primary screens (design-system §Surface: cli → Component Patterns, pattern 3 cites §cli Primary screens); the per-screen shape is the layouts extractor's.
- design ↔ tests: the not-color-alone rule (CLAUDE.md's "Status is never color-alone") is assertable on piped output — the ASCII prefix/caption is present with zero ANSI bytes (design-system §Surface: cli → Platform-Specific Notes).

## Acceptance criteria contributions
- (design) Every status a new or extended cli entry prints (catalogue listing, stop outcome, envelope standing, checklist items) carries its ASCII prefix or caption, and its piped output holds zero ANSI escape bytes and no emoji (per design-system §Surface: cli → Tokens (platform-specific)).
- (design) A past run's load-envelope standing prints as the run-level `[ENVIRONMENT-SUSPECT]` caption, once, outside the lamp column — no new lamp or `ReportState` value is introduced (per design-system §Surface: cli → Component Patterns, pattern 4).
- (design) A graceful mid-run stop leaves unmeasured checks as `[BLOCKED]` (or `—` cells), never `[FAIL]`, and the spinner stops in place rather than hiding or completing (per design-system §Anti-Patterns → Rejected Defaults).
- (design) Any new error path prints `error:`/`hint:` labels on stderr, sanitized, never to stdout (per design-system §Surface: cli → Component Patterns, pattern 5).
