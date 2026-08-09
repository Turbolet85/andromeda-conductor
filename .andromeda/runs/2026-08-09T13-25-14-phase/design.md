# design extract

## Relevance
partial — the chunk is Rust classification work, but it touches three rendering surfaces (`conductor-report` Markdown, `conductor-cli` coverage table, Tauri coverage view + `ui/src/lamp.ts`) and requires two edits to design-system.md's own prose (`:7`, `:257`).

## Constraints
- No new hex, ANSI code or palette row may be introduced for the fourth `CoverageMode` variant — any color it touches must be an existing shipped token referenced by name (`var(--…)` / named ANSI constant), never a raw literal (per design-system.md §Surface: desktop-webview / Tokens + §Surface: cli / Tokens).
- The new mode must never be rendered with `--status-fail` (red) or `--count-blocked` (slate-violet) semantics — `Blocked` is reserved for *never measured, named precondition* and Fail for the muted alarm edge; a classification is not a failure (per design-system.md §Anti-Patterns / Rejected Defaults — "Conflating 'no result yet' with 'failed'").
- `CoverageMode` is a *classification*, not a `Verdict` or `ReportState` — the green/amber/red status tier carries run state only, so the mode column must not borrow the verdict triad (per design-system.md §Color Palette, Verdict-vs-ReportState note; §Anti-Patterns Universal Bans "NEVER use color purely for decoration").
- CLI coverage-table changes keep the color-plus-ASCII-prefix pairing and dynamic terminal-width detection — never color alone, never hardcoded widths, ASCII only in machine-parseable piped output (per design-system.md §Surface: cli / Component Patterns 3 + §Anti-Patterns Per-Surface Bans / cli).
- The webview coverage matrix stays a dense single-row-per-P-ID list as the row count grows 60 → 82: `--space-md` row padding, `1px --border-subtle` dividers, virtual scroll, no shadow, no shift to a card/tile grid (per design-system.md §Surface: desktop-webview / Component Patterns 3 + §Depth Strategy).
- P-IDs in every touched renderer stay in the reserved mono status tier — JetBrains Mono `--color-id-cyan` in webview, ANSI 117 in cli; widening the set changes no type role (per design-system.md §Typography, Data row).
- Doc reconciliation must make the two named sites count-agnostic rather than restating "82": the §Brand Identity domain anchor and the Coverage-matrix component entry should read as "the manifest's accepted set", not a new literal (per design-system.md §Brand Identity + §Surface: desktop-webview / Component Patterns 3).

## Patterns to follow
- The six-state lamp set already differentiates by **shape** (filled dot · hollow ring · checkbox glyph · dashed-ring), not color alone — if `ui/src/lamp.ts` must acknowledge a fourth mode, follow shape-differentiation before reaching for a color.
- The bracketed-ASCII cli status form (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]`) is the established label shape for any new mode text in the cli table.
- `Blocked` row precedent in the cli coverage table: measurement columns render as `—`/null and carry the named precondition string, never a red error — the same "present, not measured, not failed" treatment shape the new mode will later extend.
- Token-by-name discipline throughout: the 34-token `:root` contract and the cli ANSI map are the only sources for color in the diff.

## Anti-patterns to avoid
- Inventing a new color/palette entry for the fourth mode (the 2026-06-24 stderr amendment set the precedent: reuse an existing token + a use-site note, never a new palette row).
- Any red / flashing / alarm treatment, or letting the not-Conductor's classification collapse visually into `Fail` (per §Anti-Patterns Rejected Defaults, "calm under load, no alarm").
- Re-rendering the widened matrix as a KPI-card / tile grid because the row count grew — the dense list is the explicit rejection of the Grafana tile wall.

## Contract bindings
- **Token contrast → a11y §Contrast (SC 1.4.3)**: whatever existing token the new mode reuses must still meet 4.5:1 on `--color-raised-1` (webview) and stay off dark-blue/dark-red-on-black (cli).
- **Mode label + color → a11y §Use of Color (SC 1.4.1)**: webview text label / cli bracketed ASCII prefix mandatory alongside any color signal.
- **Docs → layouts + architecture domains**: the design-system `:7` / `:257` edits are one third of the same 60→82 sweep that also hits `layout-templates.md:37/:121` and `architecture.md:33`; phrasing should be consistent across all three.
- **Motion**: none added by this chunk — no new `prefers-reduced-motion` obligation beyond the shipped global rule.

## Acceptance criteria contributions
- (design) The diff introduces zero new hex or ANSI literals for the fourth mode; every color reference in the three touched renderers resolves to an existing token by name (per design-system.md §Surface: desktop-webview / Tokens + §Surface: cli / Tokens).
- (design) In `conductor-report`, the cli coverage table and the Tauri coverage view, the new mode renders as neither `Fail` red nor `Blocked` violet, and does not adopt the green/amber/red verdict triad (per design-system.md §Color Palette Verdict-vs-ReportState note + §Anti-Patterns / Rejected Defaults).
- (design) Every rendered mode value is paired with a text label (webview) or bracketed ASCII prefix (cli) — no color-only signal, and no emoji in piped output (per design-system.md §Iconography + §Surface: cli / Tokens).
- (design) `design-system.md` §Brand Identity domain anchors and §Surface: desktop-webview / Component Patterns 3 contain no literal "60" / "P-001..P-060" after the change (per design-system.md §Brand Identity + §Surface: desktop-webview / Component Patterns 3).

## Relevant amendment history
- **2026-08-08-sut-capability-manifest** — de-hardcoded the P-ID range in token-usage examples across §Color Palette (Primary), §Typography (Data row), §cli ANSI map, §Brand Identity; why: the accepted set moved into the manifest, so illustrative examples must not pin a range. That sweep left two literal-60 sites untouched (§Brand Identity domain anchor "all 60 capabilities"; Component Patterns 3 "the full 60-row wall") — exactly the two this chunk reconciles. Same rationale, same phrasing style: name the set, not the count.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — a new cli surface (`error:`/`hint:`) was corrected at validation from "add a palette row" to "reuse the existing ANSI 246 Residual-mute token + a use-site note." This is the governing precedent for how the fourth mode acquires (or does not acquire) a color.
- **2026-06-15-design-token-typography-bundle / 2026-06-26-component-primitives-library** — established the routine "spec-illustration → sound-implementation reconciliation" rule (amend the plan's prose when shipped code proves the invariant, keep names/values as the contract). The doc-vs-code 60/82 reconciliation in this chunk is that same routine applied to a count, not a token.
