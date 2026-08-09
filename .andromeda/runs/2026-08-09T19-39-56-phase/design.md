# design extract

## Relevance
Partial — the scenario TOMLs and the `UNBACKED_AUTO` data edit are out of domain, but two rendering touchpoints are in it: the `(N unbacked)` roll-up qualifier on all three coverage surfaces, and the ManualCheck/operator-checklist treatment the two `DriveObserve` scenarios (P-067, P-072) route into.

## Constraints
1. **The coverage table is not the results table.** The `(N unbacked)` qualifier belongs to the 4-column coverage rendering (P-ID · Title · Category · Mode) that terminates in a roll-up caption and carries *no* verdict/state column and therefore no `[PASS]`-style bracket prefix — per design-system.md §Surface: cli / Component Patterns #3. Do not let the shrunk count leak into the 6-column results/SLO table.
2. **Zero new tokens for this chunk.** The count qualifier and any Mode-cell tint must reuse shipped tokens: the recessive tier is the ANSI 246 ↔ `var(--status-residual)` by-name pair, already recorded as a non-lamp de-emphasis use — per §Surface: cli / Tokens (Residual-mute entry). No new palette row, no new ANSI entry, no raw hex/px anywhere (§Surface: desktop-webview / Tokens; §Self-Validation Protocol #4 Token Test).
3. **P-067 / P-072 render on the ManualCheck tier, outside the verdict triad.** Neutral-lavender `--status-manual` checkbox glyph (webview) / ANSI 146 with `[MANUAL]` + `?` glyph (cli) — deliberately not green/amber/red, because there is no machine verdict — per §Color Palette (Verdict-vs-ReportState note), §Iconography, §Surface: cli / Component Patterns #4.
4. **Never color alone, on any of the three surfaces.** Every status/mode signal carries its always-rendered ASCII text label; Markdown, having no color channel, uses emphasis as the surface-adapted counterpart — per §Surface: cli / Per-Surface Bans and §Surface: cli / Tokens (Residual-mute entry).
5. **Mono ID-cyan is the reserved status tier.** P-IDs and any numeric count in the roll-up render JetBrains Mono / tabular-nums in `--color-id-cyan` (webview) or ANSI 117 (cli) — prose stays IBM Plex Sans / journal-lavender — per §Typography (Data + Code rows) and §Surface: cli / Tokens.
6. **Webview coverage stays a dense single-row-per-P-ID list**, `--border-subtle` seams, no shadows, no `backdrop-filter` — per §Depth Strategy and §Surface: desktop-webview / Component Patterns #3.
7. **CLI colorization stays gated.** `owo-colors` through `anstream`, honoring `NO_COLOR` / `TERM=dumb` / piped-stdout stripping; terminal width detected, never hardcoded — per §Surface: cli / Platform-Specific Notes.

## Patterns to follow
- **cli Component Pattern #3 (Results / SLO table)** — the retitled entry that fixed the coverage-vs-results conflation; the roll-up caption is the established home of the derived count.
- **cli Component Pattern #4 (Verdict / report-state lines)** — the `? P-0xx Manual …` shape with its `[MANUAL]` prefix; the headless path records unconfirmed rather than blocking on a prompt.
- **webview Component Pattern #7 (Operator-checklist)** — the existing render for `ManualCheck` P-IDs (induced state + expected observation as a ticked y/n, footer roll-up of unticked count); P-067's and P-072's rows compose into it rather than inventing a surface.
- **webview Component Pattern #6 (Run-report view)** — `ManualCheck` rows expand into #7; the "no result yet" states are prose, never a red or gray downgrade.
- **De-hardcode-don't-substitute precedent** — the two prior sweeps replaced literal capability counts with set names rather than the new literal; the `(10 unbacked)` qualifier should be derived at render time, not written into prose (§Brand Identity Domain anchors as amended).

## Anti-patterns to avoid
1. **Never collapse `ManualCheck` (or `Blocked`) into `Fail` red or a gray placeholder** — the "no result yet ≠ failed" ban, §Anti-Patterns / Rejected Defaults.
2. **Never add a new palette row or ANSI code for a de-emphasis need** — the shipped Residual-mute pair covers it; adding one is the exact drift corrected twice already (§Surface: cli / Tokens).
3. **Never render coverage as a KPI/metric-card grid or tile wall** — §Anti-Patterns / Universal Bans + Rejected Defaults ("a control surface, not a dashboard").

## Contract bindings
- **Color-only → a11y SC 1.4.1:** every ManualCheck/mode signal pairs its tint with `[MANUAL]` / the mode name text, plus `aria-live` announcement on the webview lamp (§Iconography Rule, §Surface: cli / Per-Surface Bans).
- **Token contrast → a11y §Contrast:** `--status-manual` (#A9B1D6) and `--status-residual` (#9A93A8) on `--color-raised-1`, and ANSI 146/246 on a dark terminal, must clear 4.5:1 in both dark and light variants.
- **Roll-up arithmetic → obs-plan §4:** `(N unbacked)` is a derived qualifier, not a fifth summand; design only governs its typographic tier and de-emphasis, not the count semantics — the four per-mode counts must still visibly sum to the row total.
- **Motion → a11y SC 2.3.3:** nothing here animates; if a lamp re-renders it resolves motionless via the existing 150ms `--motion-micro` color transition, dropped under `prefers-reduced-motion: reduce` (§Motion Hard limits).

## Acceptance criteria contributions
1. (design) The shrunk `(10 unbacked)` qualifier renders on all three surfaces using only shipped tokens — no new palette entry, no new ANSI code, no hardcoded hex or px (design-system §Surface Tokens, §Self-Validation #4).
2. (design) P-067 and P-072 surface on the `ManualCheck` treatment — `--status-manual` checkbox glyph / ANSI 146 `[MANUAL]` + `?` — and never render red, never as a bare gray placeholder (design-system §Color Palette Verdict-vs-ReportState, §Iconography).
3. (design) Every mode/state cell touched carries its text label with the tint only de-emphasizing; the Markdown surface uses emphasis in place of color (design-system §Surface: cli / Tokens, §Per-Surface Bans).
4. (design) The CLI coverage rendering stays the 4-column table + roll-up caption with no verdict column, and the webview stays a dense single-row-per-P-ID list (design-system §cli Component Patterns #3, §desktop-webview Component Patterns #3).

## Relevant amendment history
- **2026-08-09-out-of-scope-classification-treatment** — most directly relevant: retitled cli Component Pattern #3 to **Results / SLO table** and stated explicitly that the coverage matrix is a separate 4-column table without a verdict/state column, after the chunk's research surfaced a pre-existing conflation between the two; also recorded ANSI 246 ↔ `var(--status-residual)` as the by-name recessive pair for the out-of-scope Mode cell (zero new tokens) and named Markdown emphasis as the color-free counterpart. This chunk edits that same roll-up region — inherit both the table distinction and the reuse discipline.
- **2026-08-09-current-sut-coverage-classification** — de-hardcoded the last two literal-60 counts in the Coverage-matrix prose when the classification widened 60 → 82; the precedent is "name the set, never the new literal," which applies directly to writing `(10 unbacked)` anywhere as prose.
- **2026-08-08-sut-capability-manifest** — same de-hardcoding sweep one step earlier: token-usage examples name "P-IDs" rather than a fixed range; no token/hex/type-role changed.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — the reuse precedent that was *corrected at validation*: a proposed new "Hint grey" palette row was rejected because ANSI 246 already existed. Any impulse to add a color for the unbacked qualifier should hit the same correction.
