# layouts extract

## Relevance
partial — this chunk builds the CLI verb skeleton, which touches the cli surface from layout-templates §Surface:cli, but defers rendering polish and interactive components to later chunks.

## Constraints
1. **CLI output structure** — per layout-templates §Surface:cli / §Primary screens, the verb must output: colored header (bold + ANSI 117 title) → live content → artifact terminator, structured top-to-bottom with no cursor manipulation (ratatui omitted).
2. **Status prefix pairing** — layout-templates §Component Primary content block 2, every verdict/state cell pairs color with ASCII bracket prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) so it survives NO_COLOR / piping; ch1 prints plain + minimal form with these prefixes.
3. **Token mapping across surfaces** — layout-templates §IA notes / Decisions Log, CLI ANSI codes mirror desktop CSS vars by name: `count-nominal` ↔ ANSI 114, `count-hold` ↔ ANSI 179, `status-fail` ↔ ANSI 203, `count-blocked` ↔ ANSI 60, `color-id-cyan` ↔ ANSI 117, journal-text ↔ ANSI 146 (actual color assignment is design domain, but the mapping structure applies).
4. **Headless-default operator-pause policy** — layout-templates §Component Hero, the operator-pause hold fires only at the go/no-go decision (scope defers interactive `inquire` pairing to ch4); ch1 uses the shipped headless never-block default.
5. **Verb-noun CLI model** — layout-templates §Component Primary navigation, clap structure is flat one-level-deep (`conductor {run|suite|report} {arg}`), help verb-prefixed; output structure stable across versions because downstream agents parse it.
6. **No new surfaces created** — this chunk wires existing seams (timeline/emit/verify/report); layout is re-use of desktop-mirror CLI shapes, not new layout territory.

## Patterns to follow
1. **Frameless-window CLI mirror** — the CLI signature placement (paused-count hold-point) parallels the desktop titlebar freeze: `indicatif` heartbeat spinner STOPS at the hold with a colored phase-line prefix (ANSI 114), mirrored by a bold hold-amber HOLD phase line above the proceed/abort confirm carrying the frozen count as text (layout-templates §Surface:cli / §Component Hero).
2. **Verb output as state navigation** — the operator's path is fixed: colored header → live content → artifact terminator; the run-state envelope carries verdict + reportstate + metadata to the report verb (one envelope per run, layout-templates §Component Primary content block 2).
3. **Pipe discipline** — raw artifact data on stdout (for agent parsing), human messages on stderr; ANSI auto-stripped when piped; this chunk enforces `!isatty(1)` checks early so downstream ch3 rendering respects the contract (layout-templates §IA notes).

## Anti-patterns to avoid
1. **Do not use cursor manipulation or full-screen redraw** — ratatui is omitted by design; line-oriented stdout only (layout-templates §Surface:cli / §Tooling context).
2. **Do not colorize status without ASCII prefix** — every verdict/state must pair color with `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` or glyph (`✓`/`✗`/`?`/`•`) so NO_COLOR / piping / screen readers work; ch1 does not render colors, only prefixes (layout-templates §Component Primary content block 2 / IA notes).
3. **Do not block the headless path on interactive prompts** — operator-pause holds must check `isatty` before `inquire`; the agent-driven source-of-truth never suspends (scope §Headless-default operator-pause; layout-templates §IA notes / Headless invariant).

## Contract bindings
**cli surface → a11y** — ASCII prefix pairing (layout-templates IA notes) binds to a11y no-color-alone requirement; verdict changes announced to assistive tech (a11y derives the live-region attribute; ch1 does not add ARIA, only announces intent for downstream).

**cli surface → design tokens** — ANSI code mapping (layout-templates Decisions Log) binds verdict/state tinting to design §Status tier and design §Color tokens; the mapping names are frozen here (ANSI 114/179/203/60/117/146), but color assignment is design's responsibility at render time.

## Acceptance criteria contributions
1. (layouts) CLI verb output follows top-to-bottom linear structure per layout-templates §Primary screens (no cursor manipulation, no full-screen redraw).
2. (layouts) Every verdict/state rendered with ASCII bracket prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) paired with color (or monochrome in headless mode); NO_COLOR / piping honored.
3. (layouts) Operator-pause hold uses headless-default never-block policy (interactive `inquire` pairing deferred to ch4 per layout-templates §Component Hero).
4. (layouts) CLI output uses stable verb-noun structure (`conductor {run|suite|report}`) with pipe-friendly artifact terminator for downstream agent parsing.

## Relevant amendment history
(none)