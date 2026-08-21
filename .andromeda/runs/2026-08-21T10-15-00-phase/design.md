# design extract

## Relevance
Partial — the chunk is `method: unit` representation/plumbing with no new surface, but its scope table names two render sites (`conductor-report/src/report.rs:147`, `conductor-cli/src/render.rs:185`) and `latency_ms` / SLO timings are named members of the reserved mono status tier, so design constrains only how the value is displayed.

## Constraints
- `latency_ms` and SLO timings are named members of the reserved mono **status tier** — they must render as the Data role (JetBrains Mono 500, 13px/1.4, `tabular-nums`) in `--color-id-cyan` on desktop-webview and ANSI 117 on cli (per design-system §Typography Data row + §Color Palette Core Colors/Primary + §Surface: cli Tokens). Whether the existing render sites already bind that tier is research's question.
- Any new finer-budget field is an **addition to an existing tier's usage list, not a new color** — design-system §Color Palette Rationale reserves ID-cyan as the mono typographic tier and bans generic accent use; the plan carries no "budget" palette entry, so introducing one would require an amendment rather than an implementation choice.
- The cli results table is a fixed 6-column contract (P-ID · scenario · state · `slo_tier` · `latency_ms` right-aligned cyan · fingerprints) with terminal-width detection, and the `conductor coverage` matrix is a **separate** 4-column table carrying no verdict/state column (per design-system §Surface: cli Component Patterns #3 + §Platform-Specific Notes). Per-check rows must extend that table, not re-conflate the two.
- A check with no measurement must render `—`/null in the measurement columns, never a red error and never a silent downgrade to `Fail` (per design-system §Surface: cli Component Patterns #3 Blocked row + §Anti-Patterns / Rejected Defaults "conflating no-result-yet with failed"). This binds open fork #2 — a latency the collapse could not attribute to a named check is a "not measured here" cell, not a zero.
- The `slo_tier` cell renders only the scenario's declared value from the closed `<5s`/`<20s`/`<90s` set — never a baked per-scenario literal (per design-system §Surface: cli Component Patterns #4, as amended 2026-08-18). A sub-5s budget is therefore a distinct rendered value, not a substitution inside the tier cell.
- Color never carries a signal alone: any over-budget / breach indication must pair with an ASCII bracket prefix or text label on cli and a text label on webview (per design-system §Anti-Patterns Per-Surface Bans/cli + §Iconography Rule).
- Webview values bind tokens **by name** (`var(--color-id-cyan)`, `var(--text-secondary)`, `--space-*`, `--radius-*`) — raw hex/px/ms literals are forbidden (per design-system §Surface: desktop-webview / Tokens, Tailwind-v4 `:root` note).

## Patterns to follow
- cli **Results / SLO table** (design-system §Surface: cli Component Patterns #3) — the established home for per-check `latency_ms`; extend its column semantics rather than inventing a second table.
- cli **Verdict / report-state lines** (design-system §Surface: cli Component Patterns #4) — the per-P-ID in-place line format that already carries latency + `<slo_tier>` beside the `[PASS]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` prefix set.
- webview **Coverage matrix row** and **Run-report view** (design-system §Surface: desktop-webview Component Patterns 3 and 6) — both already reserve an SLO-tier + `latency_ms` mono ID-cyan cell; per-check values slot into these, no new panel.
- The **recessive Residual-mute pair** (`--status-residual` ↔ ANSI 246, design-system §Surface: cli Tokens) is the sanctioned de-emphasis treatment for a non-lamp qualifier caption (e.g. attributing an inherited latency to a named check) — reuse by name; it is not a lamp state and the text label carries the signal.

## Anti-patterns to avoid
- No new palette row / ANSI entry for "budget" or "over-budget" — the tier set is closed at green/amber/red/slate-violet + manual/residual + ID-cyan (design-system §Color Palette + §Anti-Patterns Universal Bans "color purely for decoration").
- No flashing, pulsing, blinking or animated emphasis on a breached budget — `Fail` red resolves in place, motionless (design-system §Motion Hard limits + §Anti-Patterns Rejected Defaults).
- No widening of the mono face into prose to make timings "look technical" — mono-everywhere is a named Rejected Default; mono stays the status tier, IBM Plex Sans carries prose (design-system §Anti-Patterns Rejected Defaults).

## Contract bindings
- **design ↔ a11y:** `--color-id-cyan` / ANSI 117 pairing must meet SC 1.4.3 on both the slate and light grounds (design-system §Color Palette; cli notes call out dark-terminal contrast); any budget-breach signal binds SC 1.4.1 not-color-alone via its ASCII prefix.
- **design ↔ architecture (envelope):** design-system §Typography Data row and §Surface: cli Tokens quote the field name `latency_ms` verbatim in their usage lists. Scope open fork #1 (row-per-check vs per-check sub-structure) can rename or re-nest that field; if it does, the design plan's Data-row/ANSI-map usage lists are the cascade site and the change requires an amendment, not a silent divergence.
- **design ↔ cli output discipline:** raw artifact data on stdout, human messages on stderr, ANSI stripped when piped, no emoji in machine-parseable output (design-system §Anti-Patterns Per-Surface Bans/cli) — binds any new per-check row emitted for agent capture.

## Acceptance criteria contributions
- Every rendered per-check `latency_ms` and any declared sub-5s budget uses the reserved mono status tier — `--color-id-cyan` + Data type role on webview, ANSI 117 right-aligned in the `comfy-table` results table on cli — with no new color introduced (per design-system §Typography Data row + §Surface: cli Component Patterns #3).
- A check carrying no measurement renders `—`/null in the measurement column, never `0`, never red, never collapsed into `Fail` (per design-system §Surface: cli Component Patterns #3 + §Anti-Patterns Rejected Defaults).
- The `slo_tier` cell still renders only the scenario's declared value from the closed `<5s`/`<20s`/`<90s` set; a finer budget renders as its own value and is never baked into the tier cell as a literal (per design-system §Surface: cli Component Patterns #4).
- No hardcoded hex, px, ms or column widths land at either render site: tokens bound by name on webview, existing ANSI map entries on cli, `comfy-table` widths detected dynamically (per design-system §Surface: desktop-webview Tokens + §Surface: cli Platform-Specific Notes).

## Relevant amendment history
- **2026-08-18-error-baseline-spike-live-proof** — cli Component Patterns #4 sample tier de-literalized to `<slo_tier>` drawn from the scenario's declared closed-set value. Directly this chunk's area: it establishes that a scenario's timing declaration is rendered as a placeholder over the closed set, never a baked literal — the standing rule a new sub-5s budget must not violate.
- **2026-08-09-out-of-scope-classification-treatment** — Component Pattern #3 retitled **Results / SLO table** and explicitly separated from the 4-column `conductor coverage` matrix, after research found the spec conflated the two. Relevant because this chunk touches the latency column of exactly that results table; the disambiguation must not be undone.
- **2026-08-09-sut-load-envelope** and **2026-06-24-sanitized-stderr-agent-mode-logging** — both recorded a NEW recessive/error surface by **reusing** the existing ANSI 246 / ANSI 203 tokens and explicitly declining to add a palette row (the second was corrected at validation for proposing one). This is the standing precedent for any per-check latency qualifier or breach label here: reuse an existing tier by name, no new token.
- **2026-08-08-sut-capability-manifest** — de-hardcoded fixed P-ID ranges in the token-usage examples; with the 2026-08-09 counts sweep it sets the "name the set, never substitute a fresh literal" treatment for derived values in the plan's prose.
- **2026-06-15-design-token-typography-bundle** — established the spec-illustration → sound-impl reconciliation routine (token block moved `@theme` → `:root`, names/values unchanged). Applies if a render site cannot realize the plan's illustrative form: the invariant (token-bound, never color-alone) governs, and the divergence is amended rather than absorbed.
