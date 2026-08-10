# design extract

## Relevance
partial — the chunk is backend (contracts manifest, preflight gate, core loaders), but its unmet-term output and harness-fault paths surface on the `cli` surface, which the design plan governs; no webview work.

## Constraints
- A newly named unmet term renders through the **existing `Blocked` treatment** — slate-violet ANSI 60, distinct state carrying the named precondition string, never a silent downgrade to red `Fail` (per design-system §Anti-Patterns → Rejected Defaults "Conflating 'no result yet' with 'failed'" + §Color Palette Verdict-vs-ReportState note).
- Never color-alone: every status line carries an ASCII prefix from the **closed set** `[PASS]/[HOLD]/[FAIL]/[MANUAL]/[RESIDUAL]/[BLOCKED]` (per design-system §Surface: cli / Tokens + §Per-Surface Bans → cli).
- **Zero new palette or ANSI entries.** Any de-emphasized qualifier reuses the Residual-mute tier (ANSI 246 ↔ `var(--status-residual)`), which is the shared recessive **non-lamp** tier — the always-rendered text label carries the signal, the tint only de-emphasizes (per design-system §Surface: cli / Tokens, Residual-mute entry).
- Contract-file absent/malformed is a harness fault → the **Error-output shape**: stderr, `error: <short>` + contextual detail + `hint: <fix>`, `error:` reusing Fail red (203) and `hint:` the Residual mute (246), tty-gated on the stderr-specific `IsTerminal` gate, sanitized (no host paths / internal struct names; stack traces only under `--debug`/`-v`) (per design-system §Surface: cli / Component Patterns 5).
- A blocked row's measurement columns render `—`/null, never a red error, and the row carries the named precondition string (per design-system §Surface: cli / Component Patterns 3 — Results / SLO table).
- The lamp set stays six and `ReportState` five: a new unmet term is a **precondition string inside `Blocked`**, never a seventh lamp; if the contract needs a run-level statement it takes the qualifier-caption shape (per design-system §Surface: cli / Component Patterns 4 + §Tokens).
- No interactive gate on the headless path: any operator-declared contract term must not become an `inquire` prompt without an `isatty` check, and the agent-driven source-of-truth path is never blocked on one (per design-system §Surface: cli / Platform-Specific Notes + Component Patterns 2).

## Patterns to follow
- `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs` — the lamp→ANSI map, the `[BLOCKED]` bracket prefix, and the em-dash null cell for never-measured columns; also the existing precedent that 246 is deliberately *not* fail (203) or blocked (60) for non-lamp uses.
- `D:\dev\projects\conductor\crates\conductor-cli\src\commands\preflight.rs` — an unmet precondition already prints under `[BLOCKED]` with the string carrying the signal and color as channel 3; extend this shape rather than inventing a term-specific rendering.
- The run-level `[ENVIRONMENT-SUSPECT]` load-envelope caption (ANSI 246, outside the lamp column) — the shipped shape for "a qualifier on the run, not a state" if a contract term needs run-level surfacing (per design-system §Surface: cli / Component Patterns 4).
- The `error:`/`hint:` stderr edge for loader/bounds-check faults on the two existing `contracts/` manifests — reuse it verbatim for the new manifest (per design-system §Surface: cli / Component Patterns 5).
- Markdown surfaces (contract doc / verdict notes) have no color channel: use emphasis or the bracket label in a blockquote as the surface-adapted counterpart (per design-system §Surface: cli / Tokens, Residual-mute entry).

## Anti-patterns to avoid
- Never render an unmet contract term as `[FAIL]`/red, and never blink, pulse or flash it — Fail red is the reserved muted alarm edge, resolved in place (per design-system §Anti-Patterns → Rejected Defaults; §Per-Surface Bans → cli).
- Never add a new color/ANSI entry, a seventh lamp, or a sixth `ReportState` for the run contract; reuse the shipped tiers by name (per design-system §Surface: cli / Tokens).
- Never colorize without `NO_COLOR` / `TERM=dumb` / pipe checks, never emoji in machine-parseable piped output, never mix data on stderr or messages on stdout (per design-system §Per-Surface Bans → cli).

## Contract bindings
- **Never-color-alone** binds a11y §Use of Color (SC 1.4.1) — the ASCII bracket prefix, not the ANSI code, is the accessible carrier for every new unmet-term line.
- **Sanitized `error:`/`hint:` output** binds the security plan's host-path-free / no-internal-struct-names rule; the design plan's Error-output pattern is the rendering half of the same requirement, and the scope's "host-path-free `Blocked` precondition string" is where they meet.
- **Blocked treatment ↔ webview** — if a new precondition ever reaches the desktop surface it must take `--count-blocked` + hollow-ring + text label (design-system §desktop-webview Component Patterns 4); per this chunk's Boundaries the run-report envelope stays untouched, so this binding is dormant, not waived.

## Acceptance criteria contributions
- (design) Any new unmet-term line carries its ASCII prefix from the closed bracket set and never relies on color alone (per design-system §Surface: cli / Tokens + §Per-Surface Bans → cli).
- (design) A newly named unmet term renders as `Blocked` (ANSI 60, `[BLOCKED]`, measurement cells `—`), never as `[FAIL]`/red and never as a new lamp state (per design-system §Surface: cli / Component Patterns 3–4 + §Anti-Patterns → Rejected Defaults).
- (design) Zero new palette/ANSI entries: any de-emphasized qualifier reuses the Residual mute 246 ↔ `var(--status-residual)` as a non-lamp tier (per design-system §Surface: cli / Tokens, Residual-mute entry).
- (design) Absent/malformed contract manifest surfaces as `error:` + `hint:` on stderr — sanitized, stderr-`IsTerminal`-gated, no stack trace outside `--debug` (per design-system §Surface: cli / Component Patterns 5).

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§cli / Error output): recorded that `error:` reuses Fail red 203 and `hint:` the Residual mute 246 on a stderr-specific gate — and that the proposed new "Hint grey" palette row was **corrected away** because 246 already existed. Directly binding: the new manifest's fault path must reuse this pair, not introduce a fault color.
- **2026-08-09-out-of-scope-classification-treatment** (§cli / Tokens + Component Patterns 3): the Residual mute recorded its non-lamp reuses, `var(--status-residual)` was named as the webview half of the by-name pair, and CP3 was retitled **Results / SLO table** with the explicit statement that `conductor coverage` is a separate 4-column table with no verdict/state column. Relevant because a new precondition belongs in the results table, not the coverage table.
- **2026-08-09-sut-load-envelope** (§Color Palette, Residual-mute entry): the third non-lamp reuse (`[ENVIRONMENT-SUSPECT]`) was added by **naming the set rather than a fresh literal count**. If this chunk adds a fourth non-lamp reuse, follow that same de-hardcode-don't-substitute treatment.
- Meta-precedent from **2026-06-15** / **2026-06-26** (spec-illustration → sound-impl reconciliation): where a spec literal is unrealizable against shipped tokens, reconcile to the token by name rather than hardcoding — applies if the contract manifest's rendering suggests any raw value.
