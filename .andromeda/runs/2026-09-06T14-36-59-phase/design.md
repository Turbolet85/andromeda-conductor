# design extract

## Relevance
Partial — the chunk is backend/CI, but it touches the `cli` design surface (`commands/coverage.rs`, `render.rs` coverage_table/coverage_summary, gate-failure output) and, if CARRY 2 resolves to commit, the color-free Markdown counterpart surface; no `desktop-webview` rendering is in scope.

## Constraints
- The `conductor coverage` table is a SEPARATE, narrower table from the results/SLO table: 4 columns (P-ID · Title · Category · Mode), **no verdict/state column and therefore no bracket prefix**, terminating in a roll-up caption. Raising this path to a gate must not add a state/lamp column to it (per design-system.md §Surface: cli / Component Patterns 3).
- The per-P-ID lamp set is **closed at six** (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`). A gate verdict/caption is a run-level qualifier and must join the existing non-lamp caption SET (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`, ANSI 246, outside the lamp column) — never a seventh lamp state, never a sixth `ReportState`, never a new palette row (per §Surface: cli / Tokens).
- Gate-failure text must follow the shipped error shape: stderr, `error: <short>` + contextual detail + `hint: <fix>`, `error:` reusing Fail red (ANSI 203) and `hint:` the Residual mute (ANSI 246) behind stderr's own `std::io::IsTerminal` gate; sanitized — no absolute host paths, internal struct names, or stack traces outside `--debug`/`-v` (per §Surface: cli / Component Patterns 5 + §Platform-Specific Notes).
- Color never carries a signal alone: every status/gate marker carries its ASCII label, and output must be ANSI-free when `NO_COLOR` is set, `TERM=dumb`, or stdout/stderr is piped (CI capture) — decided independently per stream (per §Surface: cli / Tokens preamble + §Anti-Patterns → Per-Surface Bans: cli).
- The out-of-scope Mode cell (`not-conductors`) is tinted with the existing Residual-mute pair (ANSI 246 ↔ `var(--status-residual)`); **Markdown, having no color channel, uses emphasis (`_not-conductors_`) or a bracket label in a blockquote** — this governs `coverage-matrix.md` if CARRY 2 resolves to commit or keep generating it (per §Surface: cli / Tokens, Residual-mute row).
- A gap, `Blocked`, or out-of-scope row must never render as red `Fail` — "no result yet" and "never measured" are distinct states carrying their own named string; the gate's own red is reserved for a real classification breach (per §Anti-Patterns → Rejected Defaults, "Conflating 'no result yet' with 'failed'").
- `comfy-table` width is detected dynamically from the terminal; never hardcode widths or wrap at arbitrary points; stdout carries raw artifact data, stderr human messages (per §Surface: cli / Platform-Specific Notes + Per-Surface Bans).

## Patterns to follow
- **Recessive-tier reuse by name** — any new de-emphasized gate text reuses the shipped ANSI 246 / `--status-residual` pair rather than introducing a color; the always-rendered ASCII label carries the signal, the tint only de-emphasizes (§Surface: cli / Tokens).
- **Run-level caption above the per-P-ID lines** — `[ENVIRONMENT-SUSPECT]` (load envelope, CARRY 4's surface) and `[PRECONDITION]` are the precedent shape for a run-scope qualifier printed once, outside the lamp column (§Surface: cli / Component Patterns 4 + Tokens).
- **Roll-up as a terminating caption** — the coverage table ends in the shipped roll-up caption (`coverage_summary`); the gate's counts should read off that same caption rather than growing a parallel rendering (§Surface: cli / Component Patterns 3).
- **Set-framing over baked literals** — rendered prose names the manifest's accepted capability set, never a literal count (the 60→82 move already re-staled two sites); any gate message quoting counts must derive them (§Brand Identity Domain anchors, as amended 2026-08-08 / 2026-08-09).

## Anti-patterns to avoid
- Never add a new ANSI code, palette row, or lamp state for the gate — the six-lamp set and the ANSI map are closed; reuse 203/246/117 (§Anti-Patterns → Universal Bans "color purely for decoration" + §Surface: cli / Tokens).
- Never rely on color alone and never emit emoji in machine-parseable/piped output — ASCII prefixes only there (§Anti-Patterns → Per-Surface Bans: cli).
- Never print stack traces or absolute host paths in normal-mode gate failure, and never mix data (stdout) with human messages (stderr) (§Anti-Patterns → Per-Surface Bans: cli).

## Contract bindings
- **design ↔ obs** — obs-plan §4's `coverage_percent` (in-scope denominator) vs `p_id_count_expected` (full manifest) split is rendered by the roll-up caption this domain owns (§Surface: cli / Component Patterns 3); the caption's wording and the observable's fields must not disagree.
- **design ↔ a11y** — the never-color-alone rule (ASCII `[…]` prefixes, `NO_COLOR` path) is design's expression of SC 1.4.1; contrast choices (ANSI 117/114 on dark terminals, avoiding dark-blue/dark-red-on-black) bind to a11y §Contrast (§Surface: cli / Platform-Specific Notes).
- **design ↔ tests/CI** — the CI step captures piped output, so the TTY gate must yield ANSI-free, parseable text in the CI log; whether the shipped per-stream `IsTerminal` gate already covers the gate's new output path is research's question.
- **design ↔ arch (CARRY 2)** — if `coverage-matrix.md` is committed, it becomes a color-free rendered surface bound by the Markdown emphasis counterpart rule (§Surface: cli / Tokens, Residual-mute row).

## Acceptance criteria contributions
- Gate-failure output is stderr-only in the shipped `error: <short>` / `hint: <fix>` shape, sanitized (no absolute paths, struct names, or stack traces outside `--debug`/`-v`), reusing ANSI 203/246 with no new color (per design-system.md §Surface: cli / Component Patterns 5).
- The gate introduces no new ANSI code, no new palette row, and no seventh lamp state; any run-level gate caption sits outside the lamp column with the existing Residual-mute tint (per design-system.md §Surface: cli / Tokens).
- `conductor coverage` still renders the 4-column P-ID · Title · Category · Mode table with no verdict/state column and no bracket prefixes, terminating in its roll-up caption, at dynamically detected terminal width (per design-system.md §Surface: cli / Component Patterns 3).
- Every status/gate signal is readable with color stripped (`NO_COLOR`, `TERM=dumb`, piped CI capture): ASCII labels always present, no emoji in piped output, and no unclassified/Blocked/out-of-scope row rendered as red `Fail` (per design-system.md §Anti-Patterns → Per-Surface Bans: cli + Rejected Defaults).

## Relevant amendment history
- **2026-08-09-out-of-scope-classification-treatment** — retitled cli Pattern 3 to **Results / SLO table** and stated explicitly that the coverage matrix is a separate 4-column table with no verdict/state column, after research surfaced a pre-existing conflation between the two; also recorded the out-of-scope Mode cell as the second non-lamp reuse of ANSI 246 / `var(--status-residual)` with the Markdown emphasis counterpart. Directly governs this chunk's coverage-table and `coverage-matrix.md` decisions.
- **2026-08-09-sut-load-envelope** — third non-lamp reuse (`[ENVIRONMENT-SUSPECT]`, the run-level load-envelope caption) recorded as a SET rather than a count. This is CARRY 4's rendered surface; whether the shipped caption text already names what the rate term actually measures is research's question.
- **2026-09-03-live-pulse-preconditions-probed** — retired `anstream` as the named cli gate in favour of `owo-colors` + per-stream `std::io::IsTerminal`, and added `[PRECONDITION]` to the non-lamp caption set (lamp set still closed at six, no new palette row). Sets the mechanism any new gate output must use.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — established that `error:`/`hint:` REUSE shipped tokens (203/246) rather than adding a "hint grey" row, on a stderr-specific TTY gate. Precedent for the gate's failure output adding zero tokens.
- **2026-08-08-sut-capability-manifest** and **2026-08-09-current-sut-coverage-classification** — de-hardcoded literal P-ID counts/ranges from token-usage examples and Coverage-matrix prose ("the manifest's accepted capability set", "one row per manifest capability"), because the classification widened 60 → 82. Any count the gate renders must stay derived, never a literal.
