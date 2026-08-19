# design extract

## Relevance
Partial — no UI is built here, but the chunk's verdict/report-state rendering (cli lines, results table, Markdown report) and its "retire-with-measurement, never a vacuous green" rule fall squarely under the cli half of the design plan; desktop-webview, typography, spacing, motion and iconography are out of scope for this chunk.

## Constraints
- Any pii check retired to declare-only must be recorded as `KnownResidual` — the `~` / `[RESIDUAL]` recessive-mute (ANSI 246) treatment carrying its "expected until {named fix}" note, visually NOT red and NOT green (per design-system §Color Palette → Verdict-vs-ReportState note, and §Surface: cli → Component Patterns #4).
- A check whose precondition was never measured is `Blocked`, not `Fail`: `•` / `[BLOCKED]` ANSI 60, the named precondition string carried, measurement columns rendered `—`/null and never as a red error (per design-system §Surface: cli → Component Patterns #3, and §Anti-Patterns → Rejected Defaults "Conflating no-result-yet with failed").
- `ManualCheck` sits deliberately OUTSIDE the green/amber/red triad (`?` / `[MANUAL]`, ANSI 146): TTY → `inquire` y/n, headless → recorded unconfirmed. Note the plan's §Surface: cli Pattern 4 sample uses `P-035` as its ManualCheck exemplar while this chunk owns P-035 in the pii-scrub family — whether P-035 actually carries a machine verdict here (making that illustrative literal stale under the de-literalization precedent) is research's question (per design-system §Surface: cli → Component Patterns #4).
- Never color alone: every pii verdict line must carry its ASCII bracket prefix from the closed set `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`, and output must honor `NO_COLOR` / `TERM=dumb` / piped-stdout ANSI stripping (per design-system §Surface: cli → Tokens, and §Anti-Patterns → Per-Surface Bans / cli).
- The headless agent-driven leg must never be gated on an interactive `inquire` prompt; the non-interactive decision is recorded to the artifact instead (per design-system §Surface: cli → Component Patterns #2, and §Anti-Patterns → Per-Surface Bans / cli).
- Human-facing stderr stays sanitized — `error:` + contextual detail + `hint:`, no absolute host paths, no internal struct names, stack traces only under `--debug`/`-v`. This is the design-side surface where a raw PII excerpt could leak into a rendered message (per design-system §Surface: cli → Component Patterns #5).
- No new status color/ANSI entry may be introduced for pii outcomes; the lamp set is closed at 6, and run-level qualifiers render in the recessive Residual-mute tier outside the lamp column rather than becoming a seventh lamp (per design-system §Surface: cli → Tokens, Residual-mute entry).

## Patterns to follow
- Verdict / report-state line format — glyph + bracket prefix + ID-cyan (ANSI 117) P-ID + `<slo_tier>` drawn from the scenario's declared value in the closed `<5s`/`<20s`/`<90s` set (never a baked literal) + right-aligned `latency_ms` (per design-system §Surface: cli → Component Patterns #4).
- Results / SLO table via `comfy-table`, 6 columns, terminal width detected dynamically — distinct from the separate 4-column `conductor coverage` table, which carries no verdict/state column and therefore no bracket prefix (per design-system §Surface: cli → Component Patterns #3).
- The by-name recessive pair ANSI 246 ↔ `var(--status-residual)` for non-lamp de-emphasis, with Markdown — having no color channel — using emphasis (`_…_`) or the bracket label in a blockquote as its surface-adapted counterpart. Relevant because this chunk writes a Markdown report (per design-system §Surface: cli → Tokens, Residual-mute entry).
- Mono ID-cyan is a reserved status tier (P-IDs / run_id / SLO timings / latency_ms / fingerprints). A non-content scrub witness (category label, counter) is prose/label text, not the ID tier, unless it is itself an identifier (per design-system §Typography → Data row, and §Surface: cli → Tokens).
- `indicatif` heartbeat is TTY-gated and stops-in-place (never hides, never animates to 100%) at any operator-pause, so agent-captured leg artifacts stay clean and diffable for harvest pins (per design-system §Surface: cli → Component Patterns #1).

## Anti-patterns to avoid
- No emoji in machine-parseable / piped output — ASCII prefixes only; and never mix stdout (raw artifact data) with stderr (human messages) without intention (per design-system §Anti-Patterns → Per-Surface Bans / cli).
- Never render a residual, blocked, or manual pii outcome as red `Fail`, and never blink/flash/pulse any status (per design-system §Anti-Patterns → Rejected Defaults, "flashing/pulsing red on a Fail verdict" + "Conflating no-result-yet with failed").
- Never hardcode table widths or wrap at arbitrary points; never print stack traces in normal mode (per design-system §Anti-Patterns → Per-Surface Bans / cli).

## Contract bindings
- design ↔ a11y: the always-present ASCII bracket prefix is the not-color-alone witness (SC 1.4.1); ANSI 117/114 are the contrast-safe choices for dark terminals (a11y §Contrast).
- design ↔ tests harness: harvest-tier pins take leg lines verbatim, so the rendered prefix/glyph/column format above becomes the pinned contract — a format change breaks pins, and pins must cite category + count + a non-content witness, never a raw excerpt (scope §Boundaries).
- design ↔ security/arch: §Surface: cli Component Patterns #5 defers sanitization terms to the security plan; the "no excerpt content persisted" invariant is arch/security-owned, with design owning only the rendered-message half.
- design cli ↔ desktop-webview: report states bind by name across surfaces (ANSI 246 ↔ `var(--status-residual)`, ANSI 60 ↔ `--count-blocked`), so any new pii report-state usage must map onto the existing 6-state lamp set in §Surface: desktop-webview → Component Patterns 4.

## Acceptance criteria contributions
- (design) Every pii-scrub verdict/report-state line renders its ASCII bracket prefix + glyph alongside color, and the same signal survives `NO_COLOR` / piped stdout with ANSI stripped (per design-system §Surface: cli → Tokens + §Anti-Patterns → Per-Surface Bans / cli).
- (design) Any check retired to declare-only renders `[RESIDUAL]` / `~` in the recessive mute tier with its "expected until {named fix}" note — never `[FAIL]` red and never a green `[PASS]` (per design-system §Color Palette → Verdict-vs-ReportState note + §Surface: cli → Component Patterns #4).
- (design) A never-measured pii check renders `[BLOCKED]` with its named precondition string and `—`/null measurement columns, not a red error (per design-system §Surface: cli → Component Patterns #3).
- (design) Zero new color/ANSI tokens are introduced for pii outcomes; any new label reuses a shipped token pair by name, and any run-level qualifier stays outside the lamp column (per design-system §Surface: cli → Tokens, Residual-mute entry).

## Relevant amendment history
- **2026-08-18-error-baseline-spike-live-proof** (§Surface: cli / Component Patterns #4) — the immediately-prior Epoch-3 live-proof de-literalized the sample's `<5s` to `<slo_tier>` because that chunk re-declared its tier. Directly applicable: if pii-scrub re-shapes phases or re-declares a tier, the placeholder-over-literal rule already covers it, and the same rule flags the remaining `P-035` literal in that sample.
- **2026-08-09-out-of-scope-classification-treatment** (§Surface: cli / Tokens + Component Patterns #3) — established the Residual-mute non-lamp reuse (label carries the signal, tint only de-emphasizes) and split the 6-column results table from the 4-column coverage table. Applies because this chunk may add a residual/out-of-scope classification and writes into both tables' territory.
- **2026-08-09-sut-load-envelope** (§Color Palette, Residual-mute entry) — added the `[ENVIRONMENT-SUSPECT]` run-level caption as a non-lamp reuse and set the "name the set, never the literal" rule. Applies because scope allows phase re-shaping with a `check_load_envelope` re-run.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Surface: cli / Error output) — the precedent that a new error-adjacent surface REUSES ANSI 203/246 rather than gaining a palette row, on a stderr-specific `IsTerminal` gate. Applies to any new pii-hygiene error/hint messaging.
- **2026-08-08-sut-capability-manifest** (§Color Palette / §Typography / §cli ANSI map) — de-hardcoded P-ID literals in token-usage examples; the standing precedent behind the `P-035` sample-literal flag above.
