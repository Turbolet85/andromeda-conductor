# design extract

## Relevance
**Partial (thin)** — the chunk creates no UI, but fingerprints are an explicitly named member of the reserved mono status tier, so design retains authority over how any fingerprint value / storm state reaches operator-visible output.

## Constraints
- Fingerprints are a named member of the **reserved mono ID-cyan status tier** (alongside count / P-IDs / run_id / SLO timings). Any fingerprint value that reaches operator-visible output must render in JetBrains Mono + `--color-id-cyan` (`#7DCFFF`) on webview and ANSI 117 on cli — never generic prose, never a new color (per design-system.md §Color Palette *Primary*, §Typography *Data* row, §Surface: cli *Tokens*). Mono is a **tier**, not a global face (§Anti-Patterns *Rejected Defaults* — "mono everywhere").
- The cli **Results / SLO table** is the 6-column shape whose sixth column IS `fingerprints`; it is a *separate, narrower* table from `conductor coverage` (4 columns, no verdict/state column, no bracket prefix). Column widths are terminal-detected, never hardcoded (per §Surface: cli *Component Patterns #3*). If the re-alignment changes the storm's fingerprint cardinality, that column is where it surfaces.
- **Never color alone.** Any storm/verdict/report-state signal this chunk touches must carry its ASCII label from the closed set `[PASS]` / `[HOLD]` / `[FAIL]` / `[MANUAL]` / `[RESIDUAL]` / `[BLOCKED]`, holding under `NO_COLOR`, `TERM=dumb`, and piped stdout (per §Surface: cli *Tokens*, §Anti-Patterns *Per-Surface Bans → cli*).
- `[ENVIRONMENT-SUSPECT]` — invoked by this scenario's load-envelope header — is a **run-level non-lamp caption** in the Residual-mute tier (ANSI 246 ↔ `var(--status-residual)`), rendered once above the per-P-ID lines and outside the lamp column. It is a qualifier on the run, never a seventh lamp nor a sixth `ReportState` (per §Surface: cli *Tokens* Residual-mute entry, §Surface: cli *Component Patterns #4*).
- The lamp set is **closed at six treatments** and the webview token block is a **34-token `:root` binding contract**. Retiring `RelativePathVariant` / redrawing same-fp membership must mint **no** new color, ANSI code, lamp glyph, or `ReportState` — new distinctions reuse shipped tokens by name (per §Surface: desktop-webview *Tokens*, §Iconography).
- A gap that is measured-but-pre-accepted or never-measured must **never collapse into `Fail`**: `KnownResidual` renders as the muted dashed dot (ANSI 246) carrying "expected until {named fix}"; `Blocked` as the slate-violet hollow ring (ANSI 60) carrying its named precondition, measurement columns as `—` (per §Color Palette *Verdict (3) vs ReportState (5)* note, §Anti-Patterns *Rejected Defaults* — conflating "no result yet" with "failed").

## Patterns to follow
- §Surface: cli *Component Patterns #4* (Verdict / report-state lines) — glyph + bracket label + mono-cyan values; the `~ P-032 Residual recent_commits stub → v0.3.0` form is the existing precedent should this re-alignment produce a pre-accepted gap.
- §Surface: cli *Component Patterns #3* (Results / SLO table) — reuse this shape for any storm/fingerprint reporting rather than widening the coverage table; the two were explicitly disambiguated by amendment.
- §Surface: desktop-webview *Component Patterns #6* (Run-report view) — IBM Plex Sans prose with mono `--color-id-cyan` identifiers inline; fingerprints belong in that same identifier tier if the report view ever carries them.
- **Spec-illustration → sound-impl reconciliation** (the established routine across four amendments): when prose carries a superseded rule or a stale literal, **de-hardcode / name the set**; do not substitute a fresh literal that will re-stale. Directly applicable to the scenario prose and the P-017 clause rewrite.

## Anti-patterns to avoid
- Minting a new palette row / ANSI entry / lamp state for the redrawn variant taxonomy — the 2026-06-24 "Hint grey" correction is the standing precedent that a *reuse* is not a *new token* (§Anti-Patterns *Universal Bans*, §Self-Validation *Token Test*).
- Emoji or color-only signalling in machine-parseable piped output; ASCII prefixes only there (§Anti-Patterns *Per-Surface Bans → cli*).
- Letting a documented/expected residual render as red `Fail`, or hardcoding table widths / wrapping at arbitrary points (§Anti-Patterns *Rejected Defaults* + *cli bans*).

## Contract bindings
- **Token contrast** → a11y §Contrast SC 1.4.3: ANSI 117 / `#7DCFFF` and ANSI 246 / `--status-residual` on the slate ground must meet 4.5:1; the recessive Residual tier is the one most at risk if reused for new fingerprint text.
- **Never-color-alone** → a11y §Use of Color SC 1.4.1: the bracket label, not the tint, carries the signal (also the `NO_COLOR` / screen-reader path).
- **Motion** → no binding fires: this chunk animates nothing, so §Motion and the `prefers-reduced-motion` override (a11y SC 2.3.3) are not engaged.
- **Boundary:** the `.andromeda/architecture.md` §Read-Back Dependency Posture P-017 clause is arch's domain; design constrains only how the resulting states *render*.
- **Open for research:** whether the code already renders fingerprints in the mono ID-cyan / ANSI-117 tier (rather than as untinted table text) is research's question — design-system.md states the target, not the current state.

## Acceptance criteria contributions
- (design) Any fingerprint value reaching operator-visible output renders in the reserved mono status tier — JetBrains Mono + `--color-id-cyan` on webview, ANSI 117 on cli — never as generic prose or a new color (per design-system.md §Typography *Data* row + §Surface: cli *Tokens*).
- (design) The redrawn variant model introduces zero new colors, ANSI codes, lamp glyphs, or `ReportState` values; the closed 6-lamp set and the 34-token `:root` contract are byte-unchanged (per design-system.md §Surface: desktop-webview *Tokens* + §Iconography).
- (design) Every status touched in cli output keeps its ASCII bracket label paired with color, verified under `NO_COLOR` and piped stdout (per design-system.md §Surface: cli *Tokens* + §Anti-Patterns *Per-Surface Bans → cli*).
- (design) If the re-alignment leaves a P-017/P-018 claim unmeasurable or pre-accepted, it renders as `Blocked` (hollow ring / ANSI 60, named precondition, measurement columns `—`) or `KnownResidual` (dashed / ANSI 246, "expected until {named fix}"), never as red `Fail` (per design-system.md §Color Palette *Verdict (3) vs ReportState (5)*).

## Relevant amendment history
- **2026-08-09-out-of-scope-classification-treatment** — retitled cli Component Pattern #3 to **Results / SLO table** and stated explicitly that `conductor coverage` is a separate 4-column table with no verdict/state column. Directly relevant: the `fingerprints` column lives in the *results* table, and the pre-existing conflation this fixed is the exact trap a storm-reporting change could re-open.
- **2026-08-09-sut-load-envelope** — registered the run-level `[ENVIRONMENT-SUSPECT]` caption as the Residual-mute tier's third **non-lamp** reuse, named as a set rather than a count. Relevant because this scenario's header opens with the load-envelope clause that emits that caption.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — the detector proposed a NEW "Hint grey" palette row and was **corrected** at validation: ANSI 246 was an existing token, so no palette entry was added. Standing guard for this chunk: a redrawn variant taxonomy is a reuse, not a new color.
- **2026-08-08-sut-capability-manifest** / **2026-08-09-current-sut-coverage-classification** — both de-hardcoded stale literals in prose by *naming the set* instead of substituting a new number. Same discipline applies to rewriting the superseded fingerprint rule in the scenario prose and the P-017 clause.
- No amendment has ever touched fingerprint *semantics* — the plan has only ever treated fingerprints as a member of the mono ID-cyan tier and a results-table column.
