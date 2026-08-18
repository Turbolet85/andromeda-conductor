# design extract

## Relevance
Partial — no webview work in scope; design applies only to the `cli` surface's rendered leg output (state prefixes, `slo_tier` cell, Blocked / envelope-caption treatment) and to the headless-path prompt rule that the operator-gated legs run under.

## Constraints
- The `slo_tier` cell renders the scenario's **declared** value drawn from the closed `<5s`/`<20s`/`<90s` set — never a baked per-scenario literal; a measured re-declaration (this chunk anticipates `<20s` → `<90s`) is a TOML-value change, not a rendering change (per design-system §Surface: cli / Component Patterns 4).
- A `Blocked` outcome must render as its own slate-violet state carrying the **named precondition string**, with measurement columns as `—`/null — never a red error and never collapsed into `Fail` (per design-system §Surface: cli / Component Patterns 3 + §Anti-Patterns / Rejected Defaults "Conflating 'no result yet' with 'failed'"). This governs CARRY fact 2's "the read-back honestly Blocks".
- Never color alone: every per-P-ID state carries its ASCII bracket prefix from the closed set `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`, so the signal survives `NO_COLOR`, `TERM=dumb`, and piped capture (per design-system §Surface: cli / Tokens + §Anti-Patterns / Per-Surface Bans: cli).
- Envelope classification, when recorded, belongs to the **run-level** `[ENVIRONMENT-SUSPECT]` caption in the Residual-mute tier (ANSI 246), printed once above the per-P-ID lines and outside the lamp column — it is a qualifier on the run, never a seventh lamp or a sixth `ReportState` (per design-system §Surface: cli / Component Patterns 4 + §Surface: cli / Tokens). Whether that caption is already emitted on breach is research's question.
- The headless agent-driven path must never be gated on an interactive prompt: `inquire` prompts are `isatty`-checked, and when stdin is not a TTY the prompt is skipped per the configured non-interactive policy with the decision **recorded to the artifact** (per design-system §Surface: cli / Component Patterns 2 + §Surface: cli / Platform-Specific Notes). This binds the "operator-gated proof leg" driven through `scripts/agent-run.sh`.
- Live-run progress must **stop in place**, never hide and never animate-to-100%, at any operator-pause; the `indicatif` carrier is TTY-gated so agent-captured artifacts stay clean (per design-system §Surface: cli / Component Patterns 1 + §Motion high-impact moment + §Anti-Patterns / Per-Surface Bans: cli).
- stdout carries raw artifact data, stderr human messages; no emoji in machine-parseable piped output; `comfy-table` width detected dynamically, never hardcoded (per design-system §Anti-Patterns / Per-Surface Bans: cli).

## Patterns to follow
- **Results / SLO table** (`comfy-table`, 6 columns: P-ID · scenario · `state` · `slo_tier` · `latency_ms` · fingerprints) is the per-check leg surface; the coverage matrix is a *separate* 4-column table with no verdict/state column and no bracket prefix — do not conflate the two when writing expectations against leg output (per design-system §Surface: cli / Component Patterns 3).
- **Verdict / report-state lines** as the per-P-ID in-place form (`✓ P-0xx  Pass  <latency> <slo_tier>` / `• P-0xx  Blocked  <named precondition>`), color always paired with the ASCII prefix (per design-system §Surface: cli / Component Patterns 4).
- **Operator-pause prompt** pattern: TTY confirm for a human operator, skipped-and-recorded on the headless leg (per design-system §Surface: cli / Component Patterns 2).
- **Sanitized stderr error edge**: `error: <short>` + detail + `hint: <fix>`, reusing Fail red (ANSI 203) and Residual mute (ANSI 246) on a stderr-specific `IsTerminal` gate; no absolute host paths or stack traces outside `--debug`/`-v` (per design-system §Surface: cli / Component Patterns 5).
- **Spec-illustration → sound-impl reconciliation**: where a measurement contradicts an illustrative literal in the plan, the plan's literal is amended to the measured/by-name form rather than the impl bending to the illustration (the routine established across four prior amendments).

## Anti-patterns to avoid
- Rendering a `Blocked`/never-measured leg (or an incomplete/auto-resolved read-back) as red `Fail`, or graying "no result yet" and "failed" identically (per design-system §Anti-Patterns / Rejected Defaults).
- Blocking the headless source-of-truth path on an `inquire` prompt, or colorizing/emitting emoji without the `NO_COLOR`/`TERM`/pipe checks — either silently breaks the release gate or the machine-parseable capture (per design-system §Anti-Patterns / Per-Surface Bans: cli).
- Hiding or animating the progress spinner to 100% at a hold, and hardcoding table widths or wrap points in leg output (per design-system §Anti-Patterns / Per-Surface Bans: cli).

## Contract bindings
- **Never-color-alone ↔ a11y**: the closed ASCII prefix set is the a11y-side guarantee (`NO_COLOR` / screen-reader / piped capture); design fixes the glyph+label set, a11y owns the criterion.
- **`slo_tier` closed set ↔ scenario/data-model domain**: design renders the declared value; the closed `<5s`/`<20s`/`<90s` vocabulary and the re-declaration decision live in the scenario TOML contract.
- **`Blocked` precondition string + `Verdict`-vs-`ReportState` pair ↔ arch's run-report envelope**: a report row carries BOTH; design's six lamp treatments consume the envelope's states and must not invent a seventh.
- **Piped-output stability ↔ tests/harness**: harvest predicates pinned to verbatim leg captures must key off the ANSI-stripped ASCII form (bracket prefixes / labels), not TTY-only glyphs or color.

## Acceptance criteria contributions
- Any re-declared `slo_tier` is one of the closed `<5s`/`<20s`/`<90s` set and reaches output from the scenario's declared value, not a baked literal (per design-system §Surface: cli / Component Patterns 4).
- Any leg recorded as `Blocked` shows the `[BLOCKED]` ASCII prefix plus its named precondition string with measurement columns null/`—`, and never renders as red `Fail` (per design-system §Surface: cli / Component Patterns 3–4 + §Anti-Patterns / Rejected Defaults).
- Recorded envelope classification, on breach, appears as the single run-level `[ENVIRONMENT-SUSPECT]` caption above the per-P-ID lines in the Residual-mute tier — not as a new lamp/report state (per design-system §Surface: cli / Component Patterns 4 + §Surface: cli / Tokens).
- The `agent-run.sh` headless leg completes with no interactive prompt gate, and any operator-pause decision is recorded to the run artifact (per design-system §Surface: cli / Component Patterns 2 + §Surface: cli / Platform-Specific Notes).

## Relevant amendment history
- **2026-08-18-error-baseline-spike-live-proof** (§cli Component Patterns 4) — the immediately-prior sibling family re-declared `<5s` → `<90s`, so the sample line was de-literalized to `<slo_tier>` drawn from the declared value (placeholder-over-literal per the derived-count rule). Directly on point: this chunk's anticipated `<20s` → `<90s` move should therefore owe **no** design amendment, because the spec no longer names a per-scenario tier literal.
- **2026-08-09-sut-load-envelope** (§Color Palette / Residual-mute entry) — registered `[ENVIRONMENT-SUSPECT]` as the tier's third NON-lamp reuse and switched the body to naming the set rather than a literal count, so a fourth reuse does not re-stale it. Relevant because this chunk records envelope classification per leg.
- **2026-08-09-out-of-scope-classification-treatment** (§cli Tokens + Component Patterns 3) — retitled Pattern 3 to **Results / SLO table** and stated explicitly that `conductor coverage` is a separate 4-column table with no verdict/state column, fixing a conflation that had the 6-column results set described under a coverage heading. Relevant when writing leg-output expectations so they target the correct table.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§cli Component Patterns 5) — recorded that `error:`/`hint:` REUSE shipped tokens (203/246) rather than adding a palette row; the correction (a proposed new "Hint grey" row was rejected at validation) is the precedent for any error-edge output this chunk's legs produce: reuse the existing tier, add no color.
