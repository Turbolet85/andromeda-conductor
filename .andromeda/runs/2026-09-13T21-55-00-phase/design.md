# design extract

## Relevance
Partial — no render surface is in the chunk's touchpoints, but the artifact names run/report states for P-025 / v3-08, and design-system owns that vocabulary (closed state set, `Blocked` semantics, color-free Markdown counterpart).

## Constraints
- The state vocabulary is a CLOSED six-member set (`Pass` / `CalibrationRegion` / `Fail` / `Blocked` / `ManualCheck` / `KnownResidual`); design-system.md §Color Palette (Verdict-vs-ReportState note) forbids a seventh state or a new label for a new outcome, and §Surface: cli Tokens requires run-level qualifiers (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) to stay OUTSIDE the lamp column as non-lamp captions. If the contract or a touched header needs a word for "unmeasurable until Pulse emits X", it must be one of the six, or a non-lamp caption — not a new term.
- design-system.md §Color Palette requires the `Blocked` / `KnownResidual` distinction be preserved on semantic grounds: `Blocked` = **never measured**, present-but-greyed, carrying the named precondition string; `KnownResidual` = **measured but pre-accepted**, carrying "expected until {named fix}", visually NOT red. This chunk's own framing ("the true hue-shift latency is at present measured by nothing") maps to `Blocked`, not `KnownResidual` — which of the two v3-08 / P-025 currently carries in the matrix and report artifacts is research's question.
- design-system.md §Surface: cli / Component Patterns 3 requires a `Blocked` row to carry the **named precondition string** and to render its measurement columns as `—`/null, never a red error. The contract this chunk writes IS the natural named precondition for v3-08 ("Pulse emits the contracted observable"), so its wording is the source text for that string.
- design-system.md §Surface: cli / Tokens requires that status signal be carried by the always-rendered text label, with color only de-emphasizing; for surfaces with **no color channel (Markdown)** the surface-adapted counterpart is emphasis (`_like-this_`) or the bracket label in a blockquote. A `.md` contract artifact therefore may not lean on any color convention.
- design-system.md §Surface: cli / Tokens + §Anti-Patterns (Per-Surface Bans, cli) forbid adding a new palette row or ANSI entry for a new edge — existing tokens are reused by name (the standing Residual-mute ANSI 246 ↔ `var(--status-residual)` pair being the recurring precedent).
- design-system.md §Typography (Data row) reserves the mono ID-cyan tier strictly for P-IDs, run_id, SLO timings, `latency_ms` and fingerprints. If the contracted observable ever renders a timing in Conductor output, it belongs to that tier — but that rendering is v3-08's concern, not this chunk's.

## Patterns to follow
- §Surface: cli / Component Patterns 4 (Verdict / report-state lines) — the shipped `• P-022  Blocked  mcp-server feature + ANDROMEDA_PULSE_MCP_ENABLED + matching data-dir` shape is the existing precedent for "a machine-unmeasurable P-ID carrying its precondition in prose"; P-025's contract reference is the analogous string.
- §Surface: cli / Component Patterns 3 (Results / SLO table) — the separation of the 6-column results/SLO table (verdict + `slo_tier` + `latency_ms`) from the 4-column `conductor coverage` table (no verdict column, no bracket prefix); any P-025 line the contract anticipates being graded against lands in the former, not the latter.
- §Color Palette (Verdict vs ReportState) + §Component Patterns 6 (Run-report view) — a report row carries BOTH a `Verdict` and a `ReportState` per arch's run-report envelope; a hard-grade predicate written for v3-08 should be expressible as a `Verdict` without collapsing the row's `ReportState`.
- §Iconography — every status treatment is paired with its text label and an `aria-label`/announcement; six visually distinct treatments exist precisely so none silently reads as another.

## Anti-patterns to avoid
- NEVER conflate "no result yet" with "failed" — showing `Blocked` as a red error or graying it identically to a failure is a named Rejected Default (§Anti-Patterns / Rejected Defaults). A contract whose absence of data downgrades P-025 to `Fail` violates this.
- NEVER invent a new lamp state, label, color token or ANSI entry for the contract's outcome — the lamp set is closed at six, run-level captions are the non-lamp escape hatch (§Surface: cli / Tokens; §Anti-Patterns / Per-Surface Bans, cli).
- NEVER rely on color alone, and NEVER use emoji in machine-parseable output — ASCII bracket prefixes only (§Anti-Patterns / Per-Surface Bans, cli).

## Contract bindings
- **Never-color-alone** binds a11y §Use of Color (SC 1.4.1) — the bracket label / Markdown emphasis is the carrier, not the tint (design-system.md §Surface: cli / Tokens).
- **State vocabulary** binds architecture's run-report envelope (Verdict-3 + ReportState-5 on the same row) and obs-plan §Delegated-timing family, which owns the unmeasurable finding this contract answers (design-system.md §Color Palette, Verdict-vs-ReportState note).
- **Named-precondition string** binds the CLI results-table and webview lamp render sites — this chunk authors the text those sites would display for v3-08 (design-system.md §Surface: cli / Component Patterns 3, 4).

## Acceptance criteria contributions
- (design) Every run/report state named for P-025 or v3-08 in the new artifact or any touched header comes from the closed six-member set; no seventh state and no new status label is introduced (per design-system.md §Color Palette / Verdict-vs-ReportState).
- (design) Where the artifact characterizes v3-08's ungraded status, it uses `Blocked` (never measured, carrying the named precondition) and not `Fail`, and does not silently reuse `KnownResidual` (measured-but-pre-accepted) for a quantity nothing has measured (per design-system.md §Color Palette; §Anti-Patterns / Rejected Defaults).
- (design) The chunk adds no new color token, palette row or ANSI entry; any status signal the artifact carries is text-label-borne — Markdown emphasis or a bracket label, never color-alone (per design-system.md §Surface: cli / Tokens).

## Relevant amendment history
- **2026-09-03-live-pulse-preconditions-probed** (§Surface: cli — Tokens, status-prefix line, Residual-mute row): `[PRECONDITION]` joined the run-level NON-lamp caption set alongside `[ENVIRONMENT-SUSPECT]`, both ANSI 246, both outside the lamp column — the lamp set stayed closed at six, with no new palette row and no count introduced. Directly governs how any "precondition not yet met" signal arising from this contract may be expressed. Its scope note also records that version claims outside a detector's scope were deliberately NOT applied.
- **2026-08-09-out-of-scope-classification-treatment** (§Surface: cli Tokens + Component Patterns 3): established that a NON-lamp use of the recessive tier carries its signal in the always-rendered text label, that Markdown (no color channel) uses emphasis or a bracket label in a blockquote as the counterpart, and disambiguated the 6-column results/SLO table from the 4-column coverage table. The Markdown-counterpart rule is the one that reaches a prose contract artifact.
- **2026-08-09-sut-load-envelope** + **2026-09-10-release-build-and-bundle** + **2026-08-18-error-baseline-spike-live-proof**: the recurring "name the SET, never bake a fresh literal" discipline applied to reuse lists, measured sizes and `slo_tier` samples — relevant if this chunk's measured figures (the 35 581 ms / 14 525.9 ms readings, the 2 000 ms bound) are ever restated on a design-governed surface.
