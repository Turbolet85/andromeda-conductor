# design extract

## Relevance
Partial — the derivation itself is out of design's domain, but the fingerprint value is a *rendered* artifact (results-table column, run-report line), and a width change (16 hex → 8-char prefix) lands on surfaces design governs.

## Constraints
- Fingerprints are members of the **reserved mono status tier**, not generic body text: JetBrains Mono 500 / 13px / tabular-nums bound to `var(--color-id-cyan)` on desktop-webview, and ANSI 117 on cli — the Data type role and the cli ANSI map both name "fingerprints" explicitly (per design-system.md §Typography, Data row + §Surface: cli / Tokens). Whether the existing render sites already bind that tier is research's question.
- The cli results table is a **6-column comfy-table with a `Fingerprints` column**, distinct from the 4-column `conductor coverage` table which carries no verdict/state column — a rendering change to fingerprints belongs to the former only (per design-system.md §Surface: cli / Component Patterns 3, "Results / SLO table").
- If the comparison surface narrows the rendered value (16 → 8 chars), the column must stay **terminal-width-detected**; hardcoding or wrapping at an arbitrary point is banned (per design-system.md §Surface: cli / Platform-Specific Notes + §Anti-Patterns / Per-Surface Bans: cli).
- A precondition that **never measured** must render as `Blocked` — slate-violet `[BLOCKED]` + the named precondition string, measurement cells as `—` — and must never be downgraded to red `Fail`; a *measured but pre-accepted* gap is `[RESIDUAL]` dashed/muted, also never red (per design-system.md §Color Palette, the Verdict-vs-ReportState note + §Anti-Patterns / Rejected Defaults, "Conflating 'no result yet' with 'failed'").
- Color never carries signal alone: every fingerprint/verdict line pairs its tint with the ASCII prefix or text label (`[PASS]`/`[FAIL]`/`[BLOCKED]`/`[RESIDUAL]`), and no emoji in piped output (per design-system.md §Surface: cli / Tokens + §Iconography, Rule).
- Any operator-facing failure from the new hashing path goes to stderr as sanitized `error:` + `hint:`, reusing Fail red (ANSI 203) and Residual mute (ANSI 246) — **no new colors**, no stack traces outside `--debug`/`-v` (per design-system.md §Surface: cli / Component Patterns 5).
- The `:root` token block is a binding 34-token contract; rendering binds tokens **by name**, never a raw hex or magic px (per design-system.md §Surface: desktop-webview / Tokens + §Self-Validation, Token Test).

## Patterns to follow
- **cli Results / SLO table** (per §Surface: cli / Component Patterns 3) — P-ID · scenario · state · slo_tier · latency_ms · fingerprints, cyan-right-aligned mono columns, blocked rows em-dashing never-measured cells. Design's mandates land at `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs`.
- **cli verdict / report-state lines** (per §Surface: cli / Component Patterns 4) — in-place per-P-ID line with glyph + bracket label + mono ID-cyan identifiers; `•` Blocked carries its precondition string, `~` Residual carries "expected until {fix}".
- **Run-report view** (per §Surface: desktop-webview / Component Patterns 6) — prose in `--text-secondary` with P-IDs / run_id / SLO timings / **fingerprints** in `--color-id-cyan`; each row carries lamp + text label. Webview render site: `D:\dev\projects\conductor\crates\conductor-tauri\ui\src\components\RunReport.tsx`.
- **Verdict lamp resolves motionless** (per §Surface: desktop-webview / Component Patterns 4) — a fingerprint-comparison outcome resolves via a 150ms in-place color transition, always paired with text.

## Anti-patterns to avoid
- No flashing / pulsing / animated alert or red banner if the fingerprint comparison comes back `Fail` — the lamp resolves in place, muted (per §Anti-Patterns / Rejected Defaults + §Motion, hard limits).
- No mono-everywhere creep and no new palette entry for a "fingerprint" color — mono ID-cyan is a *reserved tier*, and prior amendment precedent is that recessive/status needs REUSE existing tokens (per §Anti-Patterns / Rejected Defaults, "A monospace font used everywhere").
- Never hardcode the fingerprint column width or wrap the value at an arbitrary point to fit a narrowed hex string (per §Anti-Patterns / Per-Surface Bans: cli).

## Contract bindings
- **Token contrast → a11y §Contrast (SC 1.4.3):** ID-cyan `#7DCFFF` on `#1A1B26` / ANSI 117 on dark terminals is design's chosen pair (per §Surface: cli / Platform-Specific Notes); a11y owns the ratio check.
- **Color-only → a11y SC 1.4.1:** the `[PASS]`/`[FAIL]`/`[BLOCKED]`/`[RESIDUAL]` ASCII prefixes are the not-color-alone mechanism design requires; a11y owns the criterion.
- **Rendered width ↔ architecture/testing:** the comparison-surface decision (prefix vs full width) is architecture's call; design binds only the *tier* the resulting string renders in, at the render boundary (`crates\conductor-cli\src\render.rs`, `RunReport.tsx`). Fixture/golden values pinning a rendered fingerprint are testing's surface.
- **Sanitization ↔ security:** `error:`/`hint:` stderr output must carry no absolute host paths or internal struct names (per §Surface: cli / Component Patterns 5, deferring to the security plan).

## Acceptance criteria contributions
- (design) Any fingerprint value shown to the operator renders in the reserved mono status tier — `var(--color-id-cyan)` / JetBrains Mono on webview, ANSI 117 on cli — never in the prose face and never in a newly introduced color (per design-system.md §Typography, Data row + §Surface: cli / Tokens).
- (design) A preflight precondition that did not measure renders `[BLOCKED]` slate-violet with its named precondition string and `—` in measurement cells; it is never presented as a red `Fail`, and no state is signalled by color alone (per design-system.md §Color Palette, Verdict-vs-ReportState note + §Surface: cli / Component Patterns 4).
- (design) If the rendered fingerprint width changes, the `Fingerprints` column stays terminal-width-detected with no hardcoded width or arbitrary wrap, and remains in the 6-column results table (not the 4-column coverage table) (per design-system.md §Surface: cli / Component Patterns 3 + §Anti-Patterns / Per-Surface Bans: cli).
- (design) The chunk introduces no new color/spacing/motion token, no new ANSI entry, and no raw hex or magic pixel value; all rendering binds existing tokens by name (per design-system.md §Surface: desktop-webview / Tokens + §Self-Validation, Token Test).

## Relevant amendment history
- **2026-08-09-out-of-scope-classification-treatment** — retitled §cli Component Pattern #3 to **Results / SLO table** and stated explicitly that `conductor coverage` is a separate 4-column table with no verdict/state column. Directly relevant: the `Fingerprints` column this chunk touches lives in the results table only; the prior conflation is the exact trap to avoid when locating the rendering site.
- **2026-08-08-sut-capability-manifest** — de-hardcoded the P-ID range in the mono-tier usage examples (§Typography Data row, §cli ANSI map — the same rows that name "fingerprints"). Relevant as the de-hardcode-don't-substitute precedent if any tier example needs touching.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — recorded that `error:`/`hint:` **reuse** shipped tokens (ANSI 203 / 246) and that a proposed new palette row was CORRECTED away at validation. Relevant if the new hashing path surfaces an operator-facing error: reuse, do not add.
- **2026-08-09-sut-load-envelope** — Residual-mute records its non-lamp reuses as a *set*, not a count. Relevant if a `KnownResidual`/qualifier caption arises from a P-017 narrowing (scope boundary 3).
- **Standing precedent across 2026-06-15 / 2026-06-24 / 2026-06-26** — "spec-illustration → sound-impl reconciliation" is the routine treatment when a shipped implementation proves a plan illustration unrealizable. If the comparison-surface decision makes the plan's fingerprint-rendering illustration stale, that is a wrap-time amendment under this precedent, not a phase edit.
