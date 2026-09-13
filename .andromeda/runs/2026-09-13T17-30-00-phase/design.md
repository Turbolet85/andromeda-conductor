# design extract

## Relevance
Partial — the chunk is Rust audit/test debt, but two of its twelve re-attempted roster members (`cli render.rs:161:5` `stdout_color`→false, `:171:5` `stderr_color`→false) sit directly on design-owned cli color-gate behavior, and the `cleanup.rs` kill plus the new `conductor-emit/tests/common/mod.rs` can touch cli output conventions.

## Constraints
- The cli color decision is design-mandated as a **per-stream** gate: `owo-colors` applied behind `std::io::IsTerminal`, decided **independently for stdout and stderr**, each arm requiring a terminal AND `NO_COLOR` unset AND `TERM != dumb`, so piped output carries no ANSI (per design-system.md §Surface: cli / Tokens + §Platform-Specific Notes). A re-attempt on the `stdout_color`/`stderr_color` pair must pin that semantics; the gate may not be widened or collapsed to make a mutant die. Whether the shipped `render.rs` already implements the two gates independently is research's question.
- Color is never the signal alone: every status carries its ASCII prefix from the closed set `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`, plus the run-level non-lamp captions `[ENVIRONMENT-SUSPECT]`/`[PRECONDITION]` (per §Surface: cli / Tokens + §Anti-Patterns → Per-Surface Bans → cli). Assertions that read cli output must key on the label, not on an ANSI code alone.
- Stream discipline is a design requirement, not just a convention: raw artifact data on stdout, human messages on stderr, errors sanitized as `error: <short>` + `hint: <fix>` reusing Fail red (ANSI 203) and Residual mute (ANSI 246) under stderr's own `IsTerminal` gate (per §Surface: cli / Component Patterns 5). Any `cleanup.rs` killing vehicle that observes output must respect which stream it reads.
- The ANSI map and the six-state lamp set are **closed**; new edges bind existing pairs by name (ANSI 246 ↔ `var(--status-residual)`) rather than introducing a new palette row, ANSI entry, or literal (per §Surface: cli / Tokens; §Anti-Patterns → Universal Bans "never use color purely for decoration").
- Mutation/nextest vehicles run with non-TTY stdio, so design's TTY-only affordances must not become the thing a test depends on: `inquire` prompts are always `isatty`-gated and the headless path is never blocked on a prompt (per §Surface: cli / Component Patterns 2 + §Platform-Specific Notes); TTY-only glyphs `✓ ✗ ? ~ • →` and any emoji are banned from machine-parseable piped output (per §Surface: cli / Tokens).
- The eleven-key envelope array and `RunRecord` carry no design tokens — design asserts nothing over where that const lives (per §Surface: cli/desktop-webview Tokens, which enumerate the only design-owned token contracts).

## Patterns to follow
- The two-gate pattern (stdout and stderr decided separately, each with its own `IsTerminal` + `NO_COLOR` + `TERM` conditions) per §Surface: cli / Tokens — the correct shape for exercising the `render.rs` pair is one arm per stream, not a single combined path.
- The `error:`/`hint:` label pattern that reuses two already-shipped ANSI tokens instead of adding a color (§Surface: cli / Component Patterns 5) — the model for any new cli-facing output the kills introduce.
- Text-carries-the-signal / tint-only-de-emphasizes (§Surface: cli / Tokens, Residual-mute entry) — the assertable surface is the always-rendered ASCII label, which is also what survives ANSI stripping under a piped test harness.
- Results/SLO table (6 columns, carries the bracket-prefixed `state`) vs `conductor coverage` (4 columns, no verdict/state column, no prefix) are separate surfaces (§Surface: cli / Component Patterns 3) — output assertions must target the right one.

## Anti-patterns to avoid
- NEVER colorize without checking `NO_COLOR`/`TERM`/pipe status, and NEVER rely on color alone (§Anti-Patterns → Per-Surface Bans → cli) — including in a test that pins behavior by ANSI bytes only.
- NEVER add a new palette row / ANSI entry for an edge an existing token already covers (§Anti-Patterns → Universal Bans; the 2026-06-24 `hint:` precedent below).
- NEVER print stack traces in normal mode (only under `--debug`/`-v`), and NEVER hide or animate-to-100% the `indicatif` spinner (§Anti-Patterns → Per-Surface Bans → cli) — applies if a killing vehicle enters the run path.

## Contract bindings
- **design ↔ tests/test-plan §12** — the color-gate *behavior* is design-owned; the roster member 5 (cli PAIR) ratification and its re-attempt discipline are test-plan's. A kill on that pair must assert design's stated gate semantics rather than redefine them.
- **design ↔ a11y** — never-color-alone binds a11y §Use of Color (SC 1.4.1); the ANSI-stripped `NO_COLOR`/piped path is also the screen-reader-safe path.
- **design ↔ arch** — the eleven-key envelope / `RunRecord` schema home is arch's contract; design claims no token or naming authority there.

## Acceptance criteria contributions
- (design) No new ANSI code, palette row, lamp state, caption, or hardcoded color literal is introduced by the five kills, the shared `conductor-emit/tests/common/mod.rs`, or the envelope-const move — existing tokens bound by name only (per design-system.md §Surface: cli / Tokens).
- (design) Any killing assertion over cli output asserts the ASCII label/prefix (`[PASS]`/`[FAIL]`/`error:`/`hint:`/…), never a color code alone (per design-system.md §Surface: cli / Component Patterns 4 + §Anti-Patterns → Per-Surface Bans → cli).
- (design) The `render.rs` `stdout_color`/`stderr_color` re-attempt leaves per-stream gate semantics intact — terminal AND `NO_COLOR` unset AND `TERM != dumb`, decided independently for each stream (per design-system.md §Surface: cli / Tokens + §Platform-Specific Notes); whether the shipped code already satisfies this is research's question.
- (design) Nothing in the chunk makes a test depend on TTY-only styling or on an interactive `inquire` prompt in the headless path (per design-system.md §Surface: cli / Component Patterns 2 + §Platform-Specific Notes).

## Relevant amendment history
- **2026-09-03-live-pulse-preconditions-probed** (§Surface: cli — Toolkit/Framework · Tokens · Platform-Specific Notes) — retired `anstream`/`anstyle` as the named gate; the mandated mechanism is `owo-colors` + `std::io::IsTerminal`, decided independently per stream with the `NO_COLOR` / `TERM != dumb` conditions. This is the amendment that fixes the exact behavior the `stdout_color`/`stderr_color` re-attempt must pin — the behaviour was held constant while the named mechanism moved, so a kill must assert behaviour, not a library.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Surface: cli / Error output) — recorded that `error:` reuses ANSI 203 and `hint:` ANSI 246 under a **stderr-specific** `IsTerminal` gate distinct from stdout's, and explicitly corrected a proposal to add a new "Hint grey" palette row. Precedent: a new cli output edge reuses shipped tokens; it does not earn a color.
- **2026-08-09-out-of-scope-classification-treatment** and **2026-08-09-sut-load-envelope** (§Color Palette / §Surface: cli Tokens) — the Residual-mute ANSI 246 ↔ `var(--status-residual)` pair is tracked as a named SET of non-lamp reuses (never a count), and the lamp set stays closed at six. Governs any new recessive-tier output this chunk produces.
- **2026-09-10-release-build-and-bundle** — the "name the set, never substitute a fresh literal that will re-stale" fix rule, the same discipline this chunk adopts for the accepted-deliberate roster (enumerated SET, never a count); noted as precedent only, no design site here.
