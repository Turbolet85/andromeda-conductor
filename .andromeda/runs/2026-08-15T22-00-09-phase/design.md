# design extract

## Relevance
Partial — guardrail-only: the chunk changes a Rust constant, its doc comment, tests, and Markdown/ledger records (no webview or new CLI render), but the operator-gated live `conductor preflight` leg exercises the design-owned cli status surface, so the token/never-color-alone/no-new-state rules bind as constraints on how any outcome is surfaced.

## Constraints
- **Zero new color tokens or ANSI entries.** Any status the leg surfaces must reuse the shipped closed set (nominal 114 / hold 179 / fail 203 / blocked 60 / manual 146 / residual 246 / ID-cyan 117; webview `--count-*` / `--status-*` by name) — per design-system §Surface: cli / Tokens + §Color Palette. A new preflight outcome does NOT earn a palette row.
- **A leg that does not reach `ready:true` must not render as `Fail`.** design-system §Surface: cli / Component Patterns 3 requires a `Blocked` row to carry the named precondition string with measurement columns as `—`/null, never a red error; §Anti-Patterns / Rejected Defaults bans conflating "no result yet" with "failed". Whether the current preflight render already does this is research's question.
- **Never color alone.** design-system §Surface: cli / Tokens + §Per-Surface Bans (cli) require every status to carry its ASCII prefix from the closed set `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`, plus `NO_COLOR` / `TERM=dumb` / piped-stdout ANSI stripping and `✓`/`✗`/`?`/`~`/`•` TTY-gating.
- **The load-envelope caption stays a run-level qualifier.** If the raised canary count interacts with the envelope, design-system §Surface: cli / Component Patterns 4 + §cli Tokens (Residual mute) require `[ENVIRONMENT-SUSPECT]` to print once, above the per-P-ID lines, in the existing ANSI 246 tier — "never a seventh lamp or a sixth `ReportState`". This binds the scope's `pulse-load-envelope.toml` premise.
- **Headless path never blocked; spinner stops, not hides.** design-system §Surface: cli / Component Patterns 1–2 + §Per-Surface Bans (cli) require `isatty`-gated `inquire` prompts, a non-blocking agent-driven path, and an `indicatif` spinner that STOPS in place at a hold (never hidden, never animated-to-100%). Directly relevant to the operator-gated live leg run headlessly.
- **Markdown has no color channel.** design-system §Surface: cli / Tokens (Residual-mute entry) requires color-free surfaces to use the bracket label or emphasis as the surface-adapted counterpart — applies to the two verdict-doc closure appends.
- **Mono ID-cyan is a reserved typographic tier.** design-system §Color Palette (Primary) + §Typography (Data row) reserve ID-cyan/ANSI 117 for P-IDs, `run_id`, SLO timings, `latency_ms`, fingerprints — never as a generic accent on new evidence prose.

## Patterns to follow
- §Surface: cli / Component Patterns 3 (**Results / SLO table**) — `comfy-table`, terminal width detected dynamically, `Blocked` row carries its precondition, measurement cells null; the `conductor coverage` table is a separate 4-column table with no verdict column.
- §Surface: cli / Component Patterns 4 (**Verdict / report-state lines**) — glyph + bracket prefix + P-ID in cyan, in-place, motionless.
- §Surface: cli / Component Patterns 5 (**Error output**) — stderr-only, sanitized (no absolute host paths / struct names / stack traces), `error:` reusing 203 and `hint:` reusing 246 on a stderr-specific `IsTerminal` gate; the natural home for the scope's "evidence-reading trap" guidance if it is ever surfaced to the operator.
- **Reuse-over-new-row reconciliation** (amendment precedent, §Relevant amendment history below) — when a new edge needs styling, bind an existing token by name rather than adding a palette/ANSI entry.

## Anti-patterns to avoid
- Adding a palette row / ANSI code for a "preflight not ready" or "canary storm undersized" outcome instead of reusing the shipped tier (design-system §Surface: cli / Tokens; the 2026-06-24 amendment corrected exactly this proposal).
- Flashing / pulsing / red-banner treatment on a failed live leg — Fail red is held muted and motionless (design-system §Anti-Patterns / Rejected Defaults; §Motion hard limits).
- Emoji in machine-parseable piped output, hardcoded table widths, or stack traces outside `--debug`/`-v` (design-system §Per-Surface Bans / cli).

## Contract bindings
- **design ↔ a11y:** bracket-prefix pairing is the not-color-alone contract (a11y SC 1.4.1); the `NO_COLOR` / piped-stdout path is its enforcement surface.
- **design ↔ architecture:** the 3-valued `Verdict` / 5-valued `ReportState` vocabulary is arch's run-report envelope; design owns only its rendering. The `v2-10` decision written to `verification-matrix.json` at P5 is a ledger value — design binds only where that value is later rendered (coverage matrix / run-report / cli verdict lines).
- **design ↔ tests:** if any test asserts CLI output text, the closed bracket-prefix set is the contract string surface; `canary_wire.rs` call sites are wire-count assertions and carry no design binding.

## Acceptance criteria contributions
- (design) No new color token, palette row, or ANSI entry is introduced by this chunk; anything surfaced binds an existing token by name (per design-system §Surface: cli / Tokens).
- (design) A preflight leg that does not reach `ready:true` renders as `[BLOCKED]` (ANSI 60) with its named precondition string and null measurement cells, never as red `Fail` (per design-system §Surface: cli / Component Patterns 3 + §Anti-Patterns / Rejected Defaults).
- (design) Every status emitted by the leg pairs color with its ASCII bracket prefix and remains readable under `NO_COLOR` / piped stdout (per design-system §Surface: cli / Tokens + §Per-Surface Bans / cli).
- (design) If the raised canary count trips the load envelope, `[ENVIRONMENT-SUSPECT]` prints once at run level in the Residual-mute tier, outside the lamp column — not a new lamp or `ReportState` (per design-system §Surface: cli / Component Patterns 4).

## Relevant amendment history
- **2026-08-09-sut-load-envelope** — Residual-mute (ANSI 246 ↔ `var(--status-residual)`) recorded its third non-lamp reuse: the run-level `[ENVIRONMENT-SUSPECT]` load-envelope caption. Directly governs this chunk's load-envelope premise: the envelope's operator-facing signal already has a token and a placement; do not invent a state for it.
- **2026-08-09-out-of-scope-classification-treatment** — recorded the Residual-mute tier's non-lamp uses as a set and disambiguated the **Results / SLO table** (6 columns, verdict prefix) from the separate 4-column `conductor coverage` table. Why it matters here: the live leg's evidence lands in the results table, not the coverage table — do not add a state column to the latter.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — a detector proposed a new "Hint grey" palette row for the new stderr edge; it was CORRECTED to a reuse of the existing ANSI 246, with a use-site note instead of a new color. This is the standing precedent for any styling need arising from this chunk's failure/hint paths.
- **2026-08-08-sut-capability-manifest / 2026-08-09-current-sut-coverage-classification** — de-hardcode-don't-substitute discipline (name the set, never bake a fresh literal into prose). Relevant as a habit for the rewritten `CANARY_STORM_COUNT` doc comment: the prior comment went stale precisely because it encoded a numeric rationale (`>=5 in 30s`) that later moved.
