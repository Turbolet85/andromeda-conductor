# design extract

## Relevance
Partial — chunk is architecture/verification domain; design system applies only to surfaces rendered by the chunk's downstream consumers (run-report / verdict lamp on coverage matrix), NOT to the chunk's core verification logic.

## Constraints
1. Any rendered verdict/report-state lamp must use design tokens for color: `--count-nominal`, `--count-hold`, `--status-fail`, `--count-blocked`, `--status-manual`, `--status-residual` per design-system §Color Palette / Semantic Colors; never hardcoded hex.
2. State color in rendered output MUST pair with text label (`Pass`/`CalibrationRegion`/`Fail`/`Blocked`/`ManualCheck`/`KnownResidual`) — color alone violates a11y SC 1.4.1 (design-system §Iconography + Focus Guide binding).
3. CLI output must use ANSI color codes mapped to the Color World (design-system §Surface: cli / Tokens): green ANSI 114 (`--count-nominal`), amber ANSI 179 (`--count-hold`), red ANSI 203 (`--status-fail`), violet ANSI 60 (`--count-blocked`), lavender ANSI 146 (`--status-manual`), muted ANSI 246 (`--status-residual`), ID-cyan ANSI 117 (`--color-id-cyan` for P-IDs / run_id / timings / fingerprints).
4. Desktop-webview rendered Assessment/verdict lamps resolve with in-place color transitions (150ms `ease-out`, no animation library) per design-system §Motion / Expression 0.3.
5. Journal-relative latency timestamps must render in mono (JetBrains Mono), size 13px, in ID-cyan color (`--color-id-cyan`), per design-system §Typography / Data tier.

## Patterns to follow
1. Verdict/ReportState lamp precedent: six distinct visual treatments (filled dot for Pass/CalibrationRegion/Fail; hollow ring for Blocked; checkbox glyph for ManualCheck; dashed-ring dot for KnownResidual) per design-system §Iconography + Component Pattern 4.
2. SLO tier display model follows the existing closed enum (`<5s`, `<20s`, `<90s`) already named in the run-report envelope; no additional SLO state hierarchy beyond what design-system §Component Pattern 6 (Run-report view) already renders.
3. Status-text pairing pattern for `ManualCheck` rows: neutral-lavender checkbox glyph + text (`ManualCheck`), NOT the green/amber/red verdict triad (design-system §Color Palette Semantic Colors / ReportState vs Verdict).

## Anti-patterns to avoid
1. NEVER hardcode color values; all colors must trace back to design tokens (--color-*) and CLI ANSI mappings.
2. NEVER use color alone to communicate state — every status must pair color with text label/prefix (design-system §Anti-Patterns: Per-Surface Bans / Universal Bans / Color-Only rule).
3. NEVER animate latency values or SLO tier displays beyond the in-place status-lamp color transition (150ms); no progress bars, spinners, or staggered reveals on comparison results (expression level 0.3 forbids framer-motion, spring physics, parallax).

## Contract bindings
- **Verdict lamp ↔ a11y§Color+Label** — rendered lamps must pair color with text per SC 1.4.1 (flagged in focus guide).
- **Motion tokens ↔ a11y§Animation** — any transition on Assessment/verdict outcome must respect `prefers-reduced-motion: reduce` (design-system §Motion, enforced by media query @media (prefers-reduced-motion: reduce) { * { transition: none !important; } }).
- **Typography (latency/SLO display) ↔ design-system§Typography§Data** — mono ID-cyan tier for SLO timings / latency_ms / run_id / P-IDs (binding contract per design-system §Surface: desktop-webview / Tokens and CLI equivalents).

## Acceptance criteria contributions
1. (design) Verdict/ReportState lamp renders with the correct color token (--count-nominal / --count-hold / --status-fail / --count-blocked / --status-manual / --status-residual) — no hardcoded hex.
2. (design) Verdict lamp color ALWAYS paired with text label or glyph; never color alone (a11y SC 1.4.1).
3. (design) CLI status prefixes render in correct ANSI 256 codes per Color World mapping; ANSI stripped when piped / NO_COLOR set.
4. (design) SLO tier display (latency_ms / <5s / <20s / <90s) renders in mono (JetBrains Mono 13px on desktop) or terminal mono with ID-cyan color (ANSI 117 on CLI).
5. (design) In-place status-lamp color transition respects prefers-reduced-motion: reduce (motion 150ms disabled, no flashing / pulsing).

## Relevant amendment history
2026-06-15-design-token-typography-bundle (§Surface: desktop-webview / Tokens): Token declarations migrated from `@theme` to `:root` with `@media (prefers-color-scheme)` overrides; all 34 tokens now emit correctly. **Implication for this chunk:** any desktop-webview token usage (color, spacing, motion) must declare on `:root`, not `@theme`; light-mode variants automatically override via `@media`. This affects how Assessment/verdict components declare color references in the Tailwind stylesheet.

## Phase note (orchestrator)
Design coverage here is FORWARD-LOOKING — it governs Epoch 6/8/9 rendering of `slo_tier`/`verdict`/`latency_ms`, NOT this chunk's backend logic. This chunk builds no UI. Carry these as binding constraints for the downstream render chunks; they impose no implementation work here beyond keeping `slo_tier` a closed enum + the verdict/state values intact.
