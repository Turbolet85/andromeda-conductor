# design extract

## Relevance
partial — hard-signals scenarios are TOML configuration and scenario fixtures; **no webview/CLI surface added by this chunk** (the verdict/report-state class encoded here is consumed by Epoch 8/9 surfaces).

## Constraints
1. Per design-system §Typography: P-IDs / run_id / SLO timings use the reserved JetBrains Mono data tier — reflected in scenario names (`P-005`..`P-008`) and fixture prose.
2. Per design-system §Color Palette / §Semantic Colors: the expected-check `class` field carries operational meaning — `Hard` binds to the green/amber/red verdict triad; `CalibrationRegion` binds to the manual-check neutral lavender `#A9B1D6`.
3. Per design-system §Anti-Patterns: never conflate "no result yet" with "failed" — P-008 `CalibrationRegion` routes to `ManualCheck`, never red `Fail` (the v2.1 amendment reclassification).
4. Per design-system §Brand Identity: the Verdict/ReportState triad + `CalibrationRegion` drive operator-pause holds + coverage-matrix rendering; TOML must carry the state precisely so Epoch 8/9 evaluators surface the correct tint + glyph.

## Patterns to follow
1. Scenario naming: `P-005`..`P-008` in the mono ID tier — reflected in `.toml` filename hint + `p_ids` fields.
2. Expected-check class selection: P-005/P-006/P-007 `class = "Hard"`; P-008 `class = "CalibrationRegion"` (routes to operator-checklist `ManualCheck` glyph, not the verdict lamp triad).
3. SLO tier encoding: `<5s` for all hard-signals aligns with the <500ms-p99 detection budget (design-system §Spacing/motion immediacy).

## Anti-patterns to avoid
1. Do NOT introduce new `ComparisonKind`/`ClaimClass` variants beyond scope — reuse Hard / CalibrationRegion; if P-007's two-sided ≥17/<17 boundary needs a new kind, defer routing to the Epoch-8 CLI evaluator.
2. Do NOT conflate P-008's `CalibrationRegion` with `Fail` or render it red — the amendment reclassifies it to neutral-lavender `ManualCheck`.
3. Do NOT hardcode SLO values outside the `<5s`/`<20s`/`<90s` scale.

## Contract bindings
- **design ↔ cli/config**: scenario TOML → `conductor-cli` evaluator reads the expected-check `class` + verdict/report-state mapping to color status prefixes (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`) (design-system §Surface: cli / Semantic Colors).
- **design ↔ desktop-webview**: scenario fixture → Run-report + Coverage-matrix lamps render the triad glyph based on resolved state (design-system §Surface: desktop-webview / Component Patterns 3, 4, 6). Both are Epoch 8/9, not this chunk.

## Acceptance criteria contributions
1. (design + test) Scenario TOML fixtures for P-005..P-008 deserialize with zero syntax errors; all P-IDs render in the mono ID tier in later CLI/desktop surfaces.
2. (design) P-005/P-006/P-007 `class = "Hard"` maps to the green/amber/red verdict lamp; P-008 `class = "CalibrationRegion"` maps to the neutral-lavender `ManualCheck` glyph, never red `Fail` (design-system §Semantic Colors).
3. (design + CLI binding) Status is never color-alone — `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]` prefixes pair with ANSI color (design-system §Motion, §Anti-Patterns cli).

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle:** token block shifted from Tailwind `@theme` to plain `:root` + `@import "tailwindcss"` (34/34 token emit). No direct impact this chunk (no frontend render until Epoch 8/9); the TOML schema stays token-contract-aware for future evaluators.
