# design extract

## Relevance
Partial — no new webview UI, motion, or type work; the chunk's operator-facing `Blocked` precondition string is governed by the CLI/report status-rendering conventions.

## Constraints
- The new divergence condition must render inside the **existing** `[BLOCKED]` lamp — the per-P-ID prefix set is closed (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`); a distinct *precondition string*, not a seventh lamp or a sixth `ReportState` (per design-system §Surface: cli / Tokens + §Component Patterns 4).
- `Blocked` keeps the slate-violet tier (ANSI 60 ↔ `var(--count-blocked)`) and must never degrade to Fail red — "present-but-greyed, never measured" is a first-class state distinct from failure (per design-system §Color Palette / Semantic Colors + §Anti-Patterns / Rejected Defaults).
- Color never carries the signal alone: the ASCII `[BLOCKED]` label (and TTY `•` glyph) is always rendered, survives `NO_COLOR` / `TERM=dumb` / piped stdout (per design-system §Surface: cli / Platform-Specific Notes).
- A `Blocked` row carries the **named precondition string**, and its measurement columns render `—`/null — never a red error value (per design-system §Surface: cli / Component Patterns 3).
- The string is sanitized operator-facing text: no absolute host paths, no internal struct names, no stack traces (per design-system §Surface: cli / Component Patterns 5).
- Zero new design tokens: bind the existing ANSI/CSS pair by name; no raw hex, no raw ANSI literal, no new palette row (per design-system §Self-Validation Protocol / Token Test + §Surface tokens "binding contract").
- If the condition is also narrated to the operator, messages go to stderr and raw artifact data stays on stdout (per design-system §Surface: cli / Anti-Patterns + §Navigation Pattern).

## Patterns to follow
- The existing compound-precondition `Blocked` line (`• P-022 Blocked <feature + env-var + matching data-dir>`) is the exact shape to imitate: env/feature/condition *names*, path-free (per design-system §Surface: cli / Component Patterns 4).
- The `[ENVIRONMENT-SUSPECT]` run-level caption precedent — a new operator-facing qualifier added with zero new lamps, zero new tokens, reusing a shipped recessive tier (per design-system §Surface: cli / Tokens + §Component Patterns 4).
- Results/SLO table `Blocked` row rendering — the 6-column results table (has a `state` column), *not* the 4-column `conductor coverage` table (no state column, no bracket prefix) (per design-system §Surface: cli / Component Patterns 3).
- Webview counterpart, if/when the string surfaces there: hollow-ring `--count-blocked` lamp + literal `Blocked` text + the precondition string in the run-report row (per design-system §Surface: desktop-webview / Component Patterns 4 and 6).

## Anti-patterns to avoid
- Rendering the divergence as a red `Fail`, an error, or an indistinguishable grey — the Rejected Default "conflating 'no result yet' with 'failed', or showing Blocked as a red error" (per design-system §Anti-Patterns / Rejected Defaults).
- Introducing a new color, lamp glyph, palette row, or ANSI entry for this condition — color communicates only Verdict/ReportState, and this reuses the Blocked tier (per design-system §Anti-Patterns / Universal Bans).
- Emitting the marker as color-only or as an emoji in machine-parseable piped output (per design-system §Anti-Patterns / Per-Surface Bans: cli).

## Contract bindings
- **design ↔ architecture**: the readiness result's `blocked_precondition` field (architecture §Standard Contracts) is the single carrier of the string design renders on two surfaces — the CLI `[BLOCKED]` line and the webview run-report row must show the *same* string.
- **design ↔ security**: the no-absolute-host-path / no-internal-names rule in design-system §cli Component Patterns 5 explicitly defers to the security plan (`.claude/rules/security.md` §Error handling) — design does not restate the sanitization rule, it consumes it.
- **design ↔ a11y**: the always-present `[BLOCKED]` ASCII prefix is design's half of Use-of-Color SC 1.4.1 (not-color-alone); the webview lamp additionally requires the paired text label + `aria-live` announcement (per design-system §Surface: desktop-webview / Component Patterns 4).
- **design tokens cross-surface**: ANSI 60 ↔ `var(--count-blocked)` are one by-name pair — a change on either surface must move both (per design-system §Surface: cli / Tokens).

## Acceptance criteria contributions
- (design) The divergence renders as the existing `[BLOCKED]` prefix + `•` glyph carrying a distinct named precondition string — no new lamp, no sixth `ReportState`, no new palette or ANSI entry (per design-system §Surface: cli / Tokens + §Component Patterns 4).
- (design) The condition is tinted only via the shipped Blocked pair referenced by name (ANSI 60 / `var(--count-blocked)`); no raw hex or raw ANSI literal appears in the new code (per design-system §Self-Validation Protocol / Token Test).
- (design) The condition never renders in Fail red and its measurement columns render `—`/null rather than an error value (per design-system §Surface: cli / Component Patterns 3 + §Anti-Patterns / Rejected Defaults).
- (design) With `NO_COLOR` set or stdout piped, the `[BLOCKED]` label and the precondition string are still fully legible and path-free (per design-system §Surface: cli / Platform-Specific Notes + §Component Patterns 5).

## Relevant amendment history
- **2026-08-09-sut-load-envelope** (§Color Palette / Residual-mute) — added the run-level `[ENVIRONMENT-SUSPECT]` caption as a *qualifier on the run, never a seventh lamp or a sixth ReportState*, and named the reuse SET instead of substituting a fresh literal count. Closest precedent for this chunk: a new operator-facing condition is expressed as a named string on an existing tier.
- **2026-08-09-out-of-scope-classification-treatment** (§cli Tokens + §Component Patterns 3) — established that "the always-rendered text label carries the signal, the tint only de-emphasizes", named `var(--status-residual)` as the by-name webview half of the ANSI pair, and **disambiguated the 6-column results/SLO table from the 4-column `conductor coverage` table** (the latter has no state column, so a `Blocked` precondition never appears there).
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§cli Component Patterns 5) — a proposed NEW palette row for the `hint:` label was *corrected at validation* to a reuse of the already-shipped ANSI 246; the sanitized-stderr edge reuses two shipped tokens and adds none. Directly warns against inventing a token for this chunk's new string.
- Not relevant to this chunk: the `@theme`→`:root`, `--motion-heartbeat`, and dialog-fade amendments (webview token/motion plumbing, untouched here).
