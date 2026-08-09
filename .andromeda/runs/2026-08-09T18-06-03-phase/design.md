# design extract

## Relevance
Partial — the posture decision itself is out of domain, but wherever it lands (run-report Markdown / `conductor-cli` / webview) it renders state, and Branch A adds a verdict row; the `cargo audit` PREREQ is entirely out of domain.

## Constraints
- A deferral/uncovered-claim caveat MUST NOT render as `Fail` red. Use the distinct non-verdict treatments: `KnownResidual` muted dashed dot carrying "expected until {named fix}" (here: the named owner), `Blocked` slate-violet hollow ring for *never measured*, `ManualCheck` neutral-lavender checkbox for *awaiting the operator* — per design-system.md §Color Palette (the Verdict-3-vs-ReportState-5 note) and §Anti-Patterns → Rejected Defaults ("Conflating 'no result yet' with 'failed'").
- A report row carries BOTH a `Verdict` and a `ReportState`; the caveat must be expressible in that pair, not as free prose bolted beside it — per §Color Palette (Verdict vs ReportState, "per arch's run-report envelope").
- A real-model interpretation leg is the amber case the palette already names: `Verdict::CalibrationRegion` = "model-interpretive, report-for-human" (`--count-hold` / ANSI 179) — do not force a model-interpretive hypothesis assertion into a binary green/red — per §Color Palette (Verdict vs ReportState).
- Zero new colors, hexes, ANSI codes or token names. Bind by name to the shipped set (`--status-residual` / `--count-blocked` / `--status-manual` / `--text-tertiary`; ANSI 246 / 60 / 146) — per §Surface: desktop-webview → Tokens, §Surface: cli → Tokens, and §Self-Validation → Token Test.
- Never color-alone: every state carries its ASCII/text label (`[RESIDUAL]` `~` · `[BLOCKED]` `•` · `[MANUAL]` `?`; webview lamp + text + `aria-live`), and Markdown — having no color channel — uses emphasis/text as the surface-adapted counterpart — per §Surface: cli → Tokens (status prefixes) + §Iconography (Rule) + §Per-Surface Bans (cli).
- CLI output stays pipe-safe: `NO_COLOR` / `TERM=dumb` / `!isatty` stripping, data on stdout vs human messages on stderr, no emoji in machine-parseable output, terminal width detected not hardcoded — per §Surface: cli → Platform-Specific Notes + §Per-Surface Bans (cli).
- Nothing animates: no pulse/blink/glow on any status, no flashing on `Fail`, run-report renders in place; any transition added must drop under `prefers-reduced-motion: reduce` — per §Motion (Hard limits for this expression level).

## Patterns to follow
- §Surface: cli → Component Patterns #4 (Verdict / report-state lines): `~ P-032  Residual  recent_commits stub → v0.3.0` is the shipped precedent for a pre-accepted gap that must never read red — a named-owner deferral line should mirror this exact shape (glyph + `[RESIDUAL]` + P-ID + note).
- §Surface: desktop-webview → Component Patterns #6 (Run-report view): `KnownResidual` rows = dashed muted lamp + "expected to fail until {named fix}" note; the same view already distinguishes "no run yet" and "run in progress" prose from a red verdict — reuse that discipline for "green does not cover interpretation".
- §Surface: cli → Component Patterns #3 (Results / SLO table): the 6-column results table carries the verdict/state column; `conductor coverage` is a **separate** 4-column table (P-ID · Title · Category · Mode) with no verdict column and no bracket prefix — a P-033 verdict row belongs in the former, a classification/Mode annotation in the latter.
- §Surface: desktop-webview → Component Patterns #7 (Operator-checklist) + §Color Palette: if the posture resolves by operator confirmation rather than machine read-back (v2-05 method `manual`), that is `ManualCheck` — outside the green/amber/red triad, with the footer roll-up surfacing unticked items so an incomplete manual pass never reads as a finished run.
- §Surface: cli → Component Patterns #5 (Error output): the `error:`/`hint:` labels are the precedent for adding a new textual edge by *reusing* two shipped tokens (203/246) rather than introducing a palette row.

## Anti-patterns to avoid
- No red/`Fail` styling, no flashing/pulsing banner, and no native OS toast for the deferral or an unverified `Auto`-classified P-033 row — per §Anti-Patterns → Rejected Defaults (flashing red on Fail; Blocked-as-red) and §Per-Surface Bans.
- No new "caveat" color, token, or ANSI entry, and no raw hex/ms literal in any render — per §Anti-Patterns → Universal Bans (color-as-decoration) + §Self-Validation → Token Test.
- No color-alone signal and no emoji in piped output for the caveat — per §Per-Surface Bans (cli).

## Contract bindings
- **a11y §Use of Color (SC 1.4.1)** ← every caveat/report-state color pairs with a text label + glyph (webview lamp + `aria-live`; cli ASCII prefix; Markdown emphasis).
- **a11y §Contrast (SC 1.4.3)** ← the muted tiers used here (`--status-residual` `#9A93A8`, `--count-blocked` `#565F89`, `--text-tertiary` `#717AA0`; ANSI 246/60) are the lowest-contrast pairs in the system — flag for verification on both dark and light variants.
- **architecture / run-report envelope** ← the Verdict(3)+ReportState(5) pair is the shared shape the caveat must serialize into so it travels with a green artifact across all three surfaces.
- **docs / `conductor-report` Markdown** ← color-free surface; the design contract is that the always-rendered text label (plus emphasis) carries the signal there.
- **a11y §Animation (SC 2.3.3)** ← only if any transition is added; the global `prefers-reduced-motion` reset in §Tokens already covers it.

## Acceptance criteria contributions
- (design) The recorded posture/caveat renders using existing tokens bound by name only — zero new hex values, ANSI codes, or token names across `tokens.css`, the cli ANSI map, and the Markdown report.
- (design) The caveat (Branch B) and any unverified `Auto`-classified P-033 row render in a non-`Fail` treatment — dashed `--status-residual`/ANSI 246 `[RESIDUAL]`, hollow-ring `--count-blocked`/ANSI 60 `[BLOCKED]`, or `--status-manual`/ANSI 146 `[MANUAL]` — carrying the named owner / "expected until {fix}" string, and never flashes, pulses, or glows.
- (design) Every colored state pairs with its ASCII/text label; output is identical in meaning under `NO_COLOR` / piped stdout, and the Markdown report states the "Conductor green ≠ interpretation trustworthy" caveat in text (no color dependence).
- (design) A Branch-A verdict row lands in the 6-column Results/SLO table (with `CalibrationRegion` available as the model-interpretive amber outcome); the 4-column `conductor coverage` table gains no verdict/state column.

## Relevant amendment history
- **2026-08-09-out-of-scope-classification-treatment** (immediately prior chunk; this chunk folds its PREREQ) — recorded the Residual-mute/ANSI 246 tier's second NON-lamp reuse (the `hint:` label and the coverage-matrix out-of-scope Mode cell), named `var(--status-residual)` as the webview half of the same by-name pair, and established that Markdown uses emphasis as its color-free counterpart; also retitled cli Component Pattern #3 to **Results / SLO table** and stated explicitly that `conductor coverage` is a separate 4-column table with no verdict column. Both halves bear directly on where a caveat may render and in what tier.
- **2026-08-09-current-sut-coverage-classification** — de-hardcoded the last literal-60 coverage counts when the classification widened to 82 rows; the standing rule is *name the set, never the new literal*. Relevant because this chunk touches P-033's coverage classification.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — the governing precedent that a new textual edge REUSES shipped tokens (203/246) and adds **no** palette row; a proposed "hint grey" row was corrected away at validation. Apply the same correction if a "caveat" color is proposed here.
- **2026-08-08-sut-capability-manifest** — de-hardcoded the fixed `P-001..P-060` range out of the mono/ANSI-117 usage examples; keep P-ID references generic when citing the status tier.
