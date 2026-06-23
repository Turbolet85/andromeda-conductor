# design extract

## Relevance
Partial — chunk builds the CLI output surface for run/suite/report verbs; design tokens and status-lamp conventions apply; typography/color/motion/spacing apply narrowly to text output and error messaging.

## Constraints
1. Status never color-alone: every `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` prefix is paired with ASCII text, never color (design-system §Tokens cli / §Anti-Patterns cli)
2. ANSI 256 color mapping per Color World (design-system §Tokens cli) — `#7DCFFF` → ANSI 117 for mono status tier (P-IDs, run_id, SLO timings); `#7EE787` → ANSI 114 (nominal green); `#E3B341` → ANSI 179 (hold amber); `#F85149` → ANSI 203 (fail red); `#565F89` → ANSI 60 (blocked violet)
3. Respect `NO_COLOR`, `TERM=dumb`, piped-stdout ANSI stripping (design-system §Tokens cli / §Navigation Pattern cli)
4. Error output to stderr, sanitized — no host paths / internal struct names / stack traces (design-system §Component Patterns cli / §Anti-Patterns cli)
5. ASCII prefixes (`[PASS]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`) + glyphs (`✓`/`✗`/`?`/`~`/`•`), never emoji in piped output (design-system §Tokens cli)
6. Headless operator-pause default — `inquire` prompt never blocks when stdin is not TTY, decision recorded to artifact (design-system §Component Patterns cli §2 / scope.md §34)

## Patterns to follow
1. Per-P-ID result format: `PREFIX P-NNN STATE TIMING SLO_TIER` (design-system §Component Patterns cli §4 — "✓ P-009  Pass   1840ms <5s") with colored prefix + neutral text
2. Coverage-matrix / SLO table via `comfy-table`: P-ID (ANSI 117 cyan) · scenario · state (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]` colored) · slo_tier · latency_ms (cyan) · fingerprints, width detected dynamically (design-system §Component Patterns cli §3)
3. Verdict/report-state lamp rendering: state label + optional context (e.g., "Residual: recent_commits stub → v0.3.0"; "Manual: halo→burgundy? · no OS toast?") mirrors the desktop-webview lamp's six visual treatments (design-system §Component Patterns webview §4)
4. Heartbeat/phase-line signature mirror: `indicatif` spinner STOPS in place (not hides) at hold, colored amber "HOLD — operator pause" phase line prints above the `inquire` prompt (design-system §Component Patterns cli §1)

## Anti-patterns to avoid
1. NEVER use emoji in machine-parseable (piped) output; ASCII only (`✓`/`✗`/`?`/`~`/`•`) (design-system §Anti-Patterns cli / scope.md §33)
2. NEVER colorize without checking `NO_COLOR` / `TERM` / pipe status; NEVER rely on color alone (design-system §Anti-Patterns cli)
3. NEVER print stack traces in normal mode — only under `--debug`/`-v` (design-system §Anti-Patterns cli / scope.md §38)

## Contract bindings
**design ↔ a11y:** Token contrast (ANSI color pair contrast on dark terminals; design-system §Color Palette → a11y §Contrast binding per focus guide); status-color + text label binding per a11y §Use of Color SC 1.4.1 (design-system §Iconography); `NO_COLOR` + screen-reader friendliness (design-system §Tokens cli).

## Acceptance criteria contributions
1. (design) Output carries ASCII status prefixes (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]`) — never color alone; color is optional decoration, not required for comprehension.
2. (design) ANSI color codes respect the Color World mapping (`--count-nominal` → ANSI 114, `--status-fail` → ANSI 203, mono ID-cyan → ANSI 117, etc.); output auto-strips ANSI when piped (`!isatty(1)`).
3. (design) Error output to stderr, sanitized (no host paths / internal struct names / stack traces in normal mode).
4. (design) Heartbeat signature: `indicatif` spinner STOPS in place at operator-pause hold, never hides or animates-to-100%.

## Relevant amendment history
2026-06-15-design-token-typography-bundle — §Tokens css syntax `:root` vs `@theme` (desktop-webview only; CLI surface unaffected, but cascaded to rules/frontend.md for consistency).