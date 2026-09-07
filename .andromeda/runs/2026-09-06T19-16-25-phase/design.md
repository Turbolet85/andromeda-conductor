# design extract

## Relevance
**Partial** — no webview surface is touched, but the chunk's live-leg CLI output (re-tiered `slo_tier` cell, P-025 verdict/report-state line, load-envelope qualifier, operator-pause hold) is governed by `design-system.md` §Surface: cli.

## Constraints
- The lamp set is **closed at six** (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`) and the run-level **non-lamp caption set** (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`, ANSI 246, outside the lamp column) is separate. Whatever this chunk's re-drive yields must land inside those sets — no seventh lamp, no sixth `ReportState`, no new palette row or ANSI entry (per design-system.md §Surface: cli / Tokens; §Color Palette "Verdict (3) vs ReportState (5)").
- The result line's tier cell must render the scenario's **declared** `slo_tier` from the closed `<5s`/`<20s`/`<90s` set, never a baked per-scenario literal — so the §3.2 re-tier must move the declared value, not a rendering literal. Whether the shipped cli already reads the declared value rather than a literal is research's question (per design-system.md §Surface: cli / Component Patterns 4).
- P-025 is `DriveObserve`; `ManualCheck` sits **deliberately outside** the green/amber/red verdict triad (ANSI 146, `?` glyph, `[MANUAL]`) because there is no machine verdict. Resolving `[[checklist]]` vs `[[expected]]` must keep the manual half from collapsing into a machine `Fail` (per design-system.md §Color Palette Verdict-vs-ReportState; §Surface: cli / Component Patterns 4).
- If a second over-budget measurement is recorded as a pre-accepted gap, the sanctioned rendering is `~` / `[RESIDUAL]` in the Residual-mute tier carrying an "expected until {named fix}" note — **visually not red** (per design-system.md §Color Palette Semantic Colors; §Surface: cli / Component Patterns 4).
- Mono ID-cyan is a reserved typographic tier: P-IDs, run_id, SLO timings, `latency_ms`, fingerprints render ANSI 117 (webview `var(--color-id-cyan)`); it is never a generic accent (per design-system.md §Typography Data row; §Surface: cli / Tokens).
- Color must never carry signal alone, and ANSI must be gated per stream (`owo-colors` + `std::io::IsTerminal`, terminal AND `NO_COLOR` unset AND `TERM != dumb`) so agent-captured live-leg artifacts stay clean (per design-system.md §Surface: cli / Tokens + Platform-Specific Notes).
- The headless agent-driven path must never block on an `inquire` prompt, and at an operator-pause the `indicatif` spinner must **stop in place**, not hide or animate to 100% — the freeze is the signature (per design-system.md §Surface: cli / Component Patterns 1 & 2; §Motion High-impact moments).

## Patterns to follow
- **§cli Component Patterns 3 (Results / SLO table)** — the 6-column `comfy-table` shape (P-ID cyan · scenario · state prefix · `slo_tier` · `latency_ms` right-aligned cyan · fingerprints), width detected dynamically. Note the plan states the `conductor coverage` matrix is a *separate* 4-column table with no state column and therefore no bracket prefix.
- **§cli Component Patterns 4 (Verdict / report-state lines)** — the per-P-ID in-place line format with glyph + bracket label + colored tier, including the run-level `[ENVIRONMENT-SUSPECT]` caption printed **once above** those lines when a run's profile breaches the load envelope. This chunk's sustained emitting window is exactly the shape that can trip it.
- **§cli Component Patterns 5 (Error output)** — sanitized stderr `error:` (ANSI 203) / `hint:` (ANSI 246), ASCII labels always present, own stderr `IsTerminal` gate; the reuse route for any new precondition/sizing failure message.
- **Residual-mute reuse precedent** — every new recessive qualifier so far has bound the existing `ANSI 246 ↔ var(--status-residual)` pair by name rather than adding a color (per design-system.md §Surface: cli / Tokens, Residual-mute entry).
- **§Brand Identity expression `0.3`** — functional-only motion, in-place resolution, no new animation surface; applies to any live-leg output this chunk adds.

## Anti-patterns to avoid
- No flashing / pulsing / blinking on a `Fail`, and no animated alert on the re-drive's outcome — Fail red resolves in place, motionless (per design-system.md §Anti-Patterns Rejected Defaults; §Motion Hard limits).
- Never conflate "no result yet" / `Blocked` / a documented residual with `Fail` — `Blocked` carries its named precondition string with measurement columns as `—`, never a red error (per design-system.md §Anti-Patterns Rejected Defaults; §Surface: cli / Component Patterns 3).
- No emoji in piped machine-parseable output and no hardcoded table widths or arbitrary wrap points (per design-system.md §Anti-Patterns Per-Surface Bans → cli).

## Contract bindings
- **Never-color-alone + `NO_COLOR`/pipe stripping** binds to a11y (Use of Color SC 1.4.1) — the ASCII bracket prefix, not the tint, must carry the state in the live-leg artifact.
- **ANSI 117 / 114 on dark terminals** binds to a11y §Contrast — the palette avoids dark-blue-/dark-red-on-black deliberately; any new tint choice inherits that pair.
- **Headless-never-blocks + TTY-gated spinner** binds to the tests/harness domain (the live-leg firing form `scripts/agent-run.{sh,ps1} run --live`) and to the security plan's "an interactive prompt there would silently break the release gate" rule.
- **Closed `<5s`/`<20s`/`<90s` tier set** binds to architecture's scenario model — the re-tier must land on a member of that set, not a new tier.

## Acceptance criteria contributions
- The re-driven `halo-hue-encoding` result line renders its declared `slo_tier` drawn from the closed `<5s`/`<20s`/`<90s` set, with no per-scenario literal introduced at the render site (per design-system.md §Surface: cli / Component Patterns 4).
- Every state this chunk renders (including a second disproof) carries its ASCII bracket label + glyph from the closed six-member lamp set, with color stripped when piped and the label still standing (per design-system.md §Surface: cli / Tokens · §Anti-Patterns Per-Surface Bans → cli).
- A recorded pre-accepted gap renders `~` / `[RESIDUAL]` in the Residual-mute tier with its "expected until {named fix}" note, and a `DriveObserve` manual item renders `?` / `[MANUAL]` — neither is downgraded to red `[FAIL]` (per design-system.md §Color Palette Verdict-vs-ReportState).
- A load-envelope breach by the sustained window prints the run-level `[ENVIRONMENT-SUSPECT]` caption once, above the per-P-ID lines and outside the lamp column, adding no new lamp state or palette row (per design-system.md §Surface: cli / Tokens status-prefix line · §Component Patterns 4).

## Relevant amendment history
- **2026-08-18-error-baseline-spike-live-proof** — the closest precedent: `error-baseline-spike` re-declared `<5s` → `<90s`, and the fix was to *de-literalize* the §cli Pattern 4 sample to `<slo_tier>` so it cannot re-stale. This chunk's §3.2 re-tier is the same shape; the placeholder is already in place, so a tier move alone should owe no design-doc edit.
- **2026-08-09-sut-load-envelope** — added `[ENVIRONMENT-SUSPECT]` to the Residual-mute non-lamp set when the load-envelope work landed, and named the reuse set rather than a literal count. This chunk sizes against the same envelope, so that caption is live here.
- **2026-09-03-live-pulse-preconditions-probed** — `[PRECONDITION]` joined the same non-lamp caption set, and the named cli gate was corrected to `owo-colors` + per-stream `std::io::IsTerminal` (`anstream` retired). Any precondition/bootstrap-posture messaging on the live leg inherits both.
- **2026-08-09-out-of-scope-classification-treatment** + **2026-06-24-sanitized-stderr-agent-mode-logging** — twice established that a new recessive/qualifier surface reuses the existing ANSI 246 ↔ `var(--status-residual)` pair (and 203 for `error:`) instead of adding a palette row; the correction pattern to follow if this chunk's output needs a new de-emphasized string.
