# design extract

## Relevance
Partial — no webview surface moves; only the `cli` output tier (status prefixes, ANSI token reuse, SLO-tier cell, sanitized error edge, headless/TTY gating) is in my domain for this chunk.

## Constraints
- The port-occupier failure edge is an already-named design surface: design-system §Color Palette (Semantic Colors, Error row) assigns "`Fail` / port-occupier alarm edge" to `#F85149` / ANSI 203 — this chunk's occupier outcomes must render through that existing token, held muted, not a new alarm treatment.
- Every occupier/connection-state outcome rendered by the CLI must carry an ASCII prefix from the closed set `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` (plus the run-level non-lamp `[ENVIRONMENT-SUSPECT]` caption) — color is never the sole signal (per design-system §Surface: cli / Tokens and §Component Patterns #4).
- Zero new colors or palette rows: any status surface this chunk introduces must reuse the shipped ANSI map (114 green · 179 amber · 203 fail · 246 residual · 60 blocked · 117 ID-cyan) — the amendment record shows new-edge surfaces reuse shipped tokens rather than adding entries (per design-system §Surface: cli / Tokens).
- If a family TOML's SLO tier is re-calibrated, the rendered tier cell must come from the scenario's declared value inside the closed `<5s`/`<20s`/`<90s` set, never a baked per-scenario literal (per design-system §Surface: cli / Component Patterns #4, as amended 2026-08-18).
- A `Blocked` outcome (unmet live precondition — e.g. Pulse not up, port already held) must render as the distinct slate-violet/ANSI-60 state carrying its named precondition string with measurement columns as `—`, never downgraded to red `Fail` (per design-system §Surface: cli / Component Patterns #3 and §Anti-Patterns / Rejected Defaults).
- The headless agent-driven path must never be gated on an interactive prompt, and TTY-only decoration (spinner, `✓`/`✗`/`?`/`~` glyphs, ANSI) must be stripped from piped/captured output (per design-system §Surface: cli / Component Patterns #2 and §Per-Surface Bans: cli). Whether the current `conductor-cli` run path already satisfies this on the occupier leg is research's question.
- P-IDs (`P-001`..`P-004`), run_id, latency_ms and fingerprints in run output belong to the reserved mono status tier — ANSI 117 ID-cyan, mono only, never a generic accent (per design-system §Typography, Data row, and §Surface: cli / Tokens).

## Patterns to follow
- §Surface: cli / Component Patterns #4 (Verdict / report-state lines) — the per-P-ID in-place line shape `✓ P-00x  Pass  <latency> <slo_tier>` with prefix + color pairing; the pattern already models `•  Blocked  <named precondition>`, which is the shape an unmet live precondition on this family should take.
- §Surface: cli / Component Patterns #3 (Results / SLO table) — the 6-column `comfy-table` results table (P-ID · scenario · state · slo_tier · latency_ms · fingerprints) with dynamically detected width; explicitly NOT the separate 4-column `conductor coverage` table (that one carries no state column).
- §Surface: cli / Component Patterns #5 (Error output) — a failed occupier bind that surfaces to the operator goes to stderr as sanitized `error: <short>` + detail + `hint: <fix>` (no host paths, struct names, or stack traces outside `--debug`/`-v`), reusing ANSI 203 / 246 behind a stderr-specific `IsTerminal` gate.
- §Surface: cli / Component Patterns #1 (Paused-count hold-point, CLI mirror) — if any operator-pause fires in this walk, the `indicatif` spinner STOPS in place at the exact value under a bold amber `HOLD — operator pause` line; TTY-gated so agent-captured artifacts stay clean.
- §Color Palette (Verdict vs ReportState note) — a report row carries BOTH a 3-valued `Verdict` and a 5-valued `ReportState`; the receiver-failed leg's classification must land in that two-axis vocabulary rather than a new status word.

## Anti-patterns to avoid
- No flashing, pulsing, blinking, or animated alert on the `Fail`/receiver-failed edge — the verdict resolves in place, motionless (per design-system §Motion "Hard limits" and §Anti-Patterns / Rejected Defaults).
- Never collapse "no result yet" / `Blocked` (never measured) into red `Fail`, and never let a pre-accepted `KnownResidual` render red (per design-system §Anti-Patterns / Rejected Defaults).
- No emoji in machine-parseable piped output, no hardcoded table widths, no colorization without `NO_COLOR`/`TERM`/pipe checks (per design-system §Per-Surface Bans: cli).

## Contract bindings
- **a11y §Use of Color (SC 1.4.1)** — the ASCII bracket-prefix requirement on every occupier/connection-state line is the cli half of the not-color-alone rule; token contrast pairs (ANSI 117/114 on dark ground) bind to a11y §Contrast.
- **tests/harness** — §Surface: cli Pattern 1's TTY gating and `anstream` ANSI stripping are what keep agent-captured artifacts clean; if this chunk's live proof pins verbatim SUT/CLI output in a harvest test, the pinned text must be the ANSI-stripped, spinner-free form. The stability of those pins depends on the de-literalized `<slo_tier>` rule above.
- **obs** — the run-report envelope's per-P-ID rows are the data this domain renders (`Verdict` + `ReportState` + `slo_tier` + `latency_ms`); the rendering contract assumes obs supplies both axes, not a single collapsed status.

## Acceptance criteria contributions
- (design) Every connection-lifecycle / port-occupier outcome line carries an ASCII prefix from the closed `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` set — no color-only signal, output legible under `NO_COLOR` and when piped (per design-system §Surface: cli / Tokens + §Component Patterns #4).
- (design) Zero new color tokens / ANSI map entries: any new status text introduced by the occupier leg reuses the shipped tier (203 fail · 246 residual · 60 blocked · 117 ID-cyan · 114 nominal · 179 hold) (per design-system §Surface: cli / Tokens).
- (design) Any re-declared SLO tier for the P-001..P-004 TOMLs stays inside the closed `<5s`/`<20s`/`<90s` set and is rendered from the scenario's declared value, never a baked literal (per design-system §Surface: cli / Component Patterns #4).
- (design) An unmet live precondition or failed occupier bind renders as `Blocked` with its named precondition and `—` measurement cells, or as a sanitized stderr `error:`/`hint:` pair — never a flashing/red-alarm treatment and never a silent downgrade of "not measured" to `Fail` (per design-system §Anti-Patterns / Rejected Defaults + §Surface: cli / Component Patterns #3, #5).

## Relevant amendment history
- **2026-08-18-error-baseline-spike-live-proof** (§Surface: cli Pattern #4) — the sample verdict line's SLO tier was de-literalized to `<slo_tier>` after error-baseline-spike re-declared `<5s`→`<90s`. Directly load-bearing here: this chunk anticipates the same tier re-calibration on the connection-lifecycle TOMLs, so the placeholder-over-literal rule (derived-count rule) applies to anything this chunk documents or pins.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Surface: cli, Error output) — established that a new error edge REUSES the shipped `error:` ANSI 203 / `hint:` ANSI 246 pair with a stderr-specific `IsTerminal` gate, and that the proposed new "Hint grey" palette row was CORRECTED away at validation. Precedent for the occupier-bind-failure output: reuse, do not add a palette entry.
- **2026-08-09-out-of-scope-classification-treatment** (§Surface: cli Tokens + Pattern #3) — retitled Pattern #3 to "Results / SLO table" and pinned that the coverage matrix is a separate 4-column table with no state column. Relevant because this chunk's live results land in the 6-column results table, not the coverage table; conflating them is the exact drift that amendment fixed.
- **2026-08-09-sut-load-envelope** (§Color Palette, Residual-mute entry) — registered `[ENVIRONMENT-SUSPECT]` as a run-level non-lamp caption in the recessive ANSI-246 tier, never a seventh lamp state. Relevant if a live Pulse run of this family breaches the load envelope: the caption is a run qualifier above the per-P-ID lines, not a per-P-ID state.
