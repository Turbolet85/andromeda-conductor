# design extract

## Relevance
partial — the chunk ships no new UI surface; design binds only where the chunk touches rendered status (the coverage matrix's committed render and roll-up caption, and the real-model leg's per-P-ID verdict/report-state lines for P-031/P-033/P-034/P-044 and a canary `Blocked`).

## Constraints
- The coverage matrix is a SEPARATE, narrower table: 4 columns (P-ID · Title · Category · Mode), NO verdict/state column and therefore NO bracket prefix, terminating in a roll-up caption. Moving four ids out of `UNBACKED_AUTO` changes caption counts only, never the matrix's column shape (per design-system §Surface: cli → Component Patterns #3). Whether `coverage-matrix.md` is produced by that same `conductor coverage` render path is research's question.
- Where the Markdown render has no color channel, a de-emphasized/non-lamp cell uses emphasis or a bracket label in a blockquote as the surface-adapted counterpart — never a new color or glyph (per design-system §Surface: cli → Tokens, Residual-mute note).
- Status is never color-alone: every per-P-ID result line pairs its state with the closed ASCII prefix set `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` (+ TTY-only glyph); no new lamp state or `ReportState` may be minted for the real-model leg (per design-system §Surface: cli → Tokens; §Color Palette → Semantic Colors note).
- A model-interpretive outcome maps to amber `CalibrationRegion` (report-for-human), distinct from `Fail`; a canary that never produced a measurable incident is `Blocked` (slate-violet, never measured) carrying its named precondition string, never downgraded to red `Fail` (per design-system §Color Palette → Semantic Colors note; §Anti-Patterns → Rejected Defaults "Conflating no result yet with failed").
- A `Blocked` row renders its measurement columns as `—`/null with the named precondition string, never a red error (per design-system §Surface: cli → Component Patterns #3).
- Any run-level qualifier a real-model drive needs (e.g. a posture note) follows the non-lamp caption precedent (`[ENVIRONMENT-SUSPECT]` / `[PRECONDITION]`, ANSI 246, outside the lamp column) — a qualifier on the run, never a seventh lamp or sixth `ReportState` (per design-system §Surface: cli → Tokens; Component Patterns #4).
- CLI output stays TTY-gated per stream (`NO_COLOR` / `TERM=dumb` / pipe), no emoji in piped output, and the headless agent path never blocks on an interactive prompt — the real-model leg runs headless only (per design-system §Surface: cli → Platform-Specific Notes; §Per-Surface Bans → cli).

## Patterns to follow
- Verdict/report-state line shape: glyph + mono P-ID (ANSI 117) + state label + SLO tier from the closed `<5s`/`<20s`/`<90s` set, never a baked per-scenario literal (per design-system §Surface: cli → Component Patterns #4).
- Results/SLO table for the four ids: P-ID · scenario · state prefix · slo_tier · latency_ms · fingerprints, width detected dynamically (per design-system §Surface: cli → Component Patterns #3).
- Sanitized stderr error shape `error:` (ANSI 203) + `hint:` (ANSI 246) for any new canary-diagnostic harness fault, never new colors, no host paths (per design-system §Surface: cli → Component Patterns #5).
- Webview coverage-matrix and run-report components bind state to existing tokens (`--count-hold`, `--count-blocked`, `--status-*`) by name — reuse, not extension, if the desktop surface ever shows these rows (per design-system §Surface: desktop-webview → Component Patterns #3, #4, #6).

## Anti-patterns to avoid
- Showing a model-side canary miss or an ungraded real-model row as red `Fail` (per design-system §Anti-Patterns → Rejected Defaults, "Conflating no result yet with failed").
- Adding a verdict/state column or bracket prefix to the coverage matrix render (per design-system §Surface: cli → Component Patterns #3).
- Relying on color alone, or minting a new color for a new caption (per design-system §Per-Surface Bans → cli; §Anti-Patterns → Universal Bans "color purely for decoration").

## Contract bindings
- design ↔ a11y: state label + glyph pairing binds to a11y §Use of Color SC 1.4.1 (not-color-alone); any token reused on the webview must hold its a11y §Contrast pairing.
- design ↔ arch: the Verdict (3) vs ReportState (5) mapping is carried per arch's run-report envelope (per design-system §Color Palette → Semantic Colors note) — the real-model harvest's typed outcomes must land on that closed set.
- design ↔ layouts: the coverage matrix's roll-up caption wording is owned by layout-templates §cli Primary screens (cited from design-system §Surface: cli → Component Patterns #3); the caption text change (`8 unbacked` → `4 unbacked`) is layouts' to grade.

## Acceptance criteria contributions
- (design) The committed coverage-matrix render keeps its 4-column shape with no state column or bracket prefix; only the roll-up counts change (per design-system §Surface: cli → Component Patterns #3).
- (design) Every per-P-ID result line the real-model leg emits for P-031/P-033/P-034/P-044 carries a closed-set ASCII prefix + label, never color alone, and no new lamp/`ReportState` is introduced (per design-system §Surface: cli → Tokens).
- (design) A canary that forms no incident renders as `[BLOCKED]` with its named precondition string and `—` measurement cells, never `[FAIL]` (per design-system §Color Palette → Semantic Colors note; §Surface: cli → Component Patterns #3).
