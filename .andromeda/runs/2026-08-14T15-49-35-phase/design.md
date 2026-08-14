# design extract

## Relevance
Partial — no webview UI and no new tokens; only the CLI/artifact operator-facing rendering of the capture (fingerprints, outcome labels, error/hint edges) falls in this domain.

## Constraints
- Any operator-facing terminal output must go through the existing ANSI-256 map via `owo-colors`/`anstream` (TTY-gated, `NO_COLOR` / `TERM=dumb` / piped-stdout stripping honored); no new color is introduced for a "capture"/"diagnostic" concept (per design-system §Surface: cli / Tokens).
- Fingerprints are members of the reserved mono status tier — rendered ID-cyan (ANSI 117 / `--color-id-cyan`) alongside P-IDs / run_id / SLO timings, never as a generic brand accent (per design-system §Typography, Data row; §Color Palette, Primary).
- Never color-alone: every status/outcome carries its ASCII label from the closed set (`[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`, glyphs `✓ ✗ ? ~ • →`), and no emoji in machine-parseable piped output (per design-system §Surface: cli / Tokens; §Anti-Patterns / Per-Surface Bans / cli).
- Channel split holds for the capture: raw artifact data on stdout, human messages on stderr as sanitized `error: <short>` + `hint: <fix>` reusing Fail red (203) and Residual mute (246) behind a stderr-specific `IsTerminal` gate — no host paths, no internal struct names, stack traces only under `--debug`/`-v` (per design-system §Surface: cli / Component Patterns 5).
- A hand-off outcome (spans arrive intact → question moves to Pulse) must not render as red `Fail`: "not measured / not ours" is the slate-violet `Blocked` treatment carrying its named precondition string, or the muted dashed `KnownResidual` with its "expected until {fix}" note (per design-system §Color Palette, Verdict-vs-ReportState note; §Rejected Defaults, "Conflating no-result-yet with failed").
- If the live capture drives a spinner/live counter, it appears only after ~200ms and STOPS in place at any hold — never hidden, never animated-to-100% (per design-system §Surface: cli / Component Patterns 1; §Motion, High-impact moments).
- Markdown capture artifacts have no color channel: use emphasis or the bracket label in a blockquote as the surface-adapted counterpart of the tint (per design-system §Surface: cli / Tokens, Residual-mute entry).

## Patterns to follow
- `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs` is the single rendering seam — `paint`, `lamp_code`, `status_line`, `error_block`, `envelope_caption`, `spinner`, plus the dual `stdout_color()` / `stderr_color()` gates. Any capture output reuses these rather than adding a fresh print path (per design-system §Surface: cli / Component Patterns 4–5).
- The Results/SLO table already owns a `fingerprints` column (`render.rs` `fingerprints()`, cyan) — that is the established home for fingerprint identity; the coverage matrix is a separate 4-column table with no verdict/state column, so do not add a capture column there (per design-system §Surface: cli / Component Patterns 3).
- `D:\dev\projects\conductor\crates\conductor-cli\src\commands\preflight.rs` is the precedent command shape for a machine-readable diagnostic: serialized JSON to stdout, human framing separate (per design-system §Surface: cli / Navigation Pattern, "pipe-friendly").
- Per-item verdict-line form (`✓ P-009  Pass  1840ms <5s` / `• P-022  Blocked  <named precondition>`) is the template for per-span capture outcomes if any are printed per-item (per design-system §Surface: cli / Component Patterns 4).

## Anti-patterns to avoid
- Do NOT add a new palette row / ANSI entry for capture, diagnostic, or "wire evidence" — the recessive tier is the existing Residual mute (246) and identity is the existing ID-cyan (117) (per design-system §Surface: cli / Tokens; §Anti-Patterns / Universal Bans, "color purely for decoration").
- Do NOT render the contradiction as an alarm — no red banner, no flashing/pulsing, no native OS toast; Fail red stays the muted, motionless reserved edge (per design-system §Rejected Defaults; §Motion / Hard limits).
- Do NOT introduce any new webview surface, KPI tile, or "capture dashboard" panel for this evidence (per design-system §Anti-Patterns / Universal Bans; §Rejected Defaults, KPI-card grid).

## Contract bindings
- **Not-color-alone → a11y SC 1.4.1**: the ASCII `[LABEL]` prefixes are the shared artifact — a11y owns the criterion, design owns the closed label set (design-system §Surface: cli / Tokens).
- **Sanitized `error:`/`hint:` output → security plan §artifact hygiene / redaction**: the design rule (no host paths, no internal struct names, no stack traces outside `--debug`) is the render-side face of the same redaction boundary the scope names for the capture artifact.
- **ID-cyan / Fail-red on dark terminals → a11y §Contrast**: ANSI 117 + 114 were chosen for contrast on dark terminals; a11y verifies, design supplies the pair (design-system §Surface: cli / Platform-Specific Notes).
- **Motion tokens ↔ a11y SC 2.3.3** — not engaged: this chunk has no webview animation surface, so the `prefers-reduced-motion` override binding does not apply here.

## Acceptance criteria contributions
- (design) Capture output introduces zero new color values — every tint resolves to an existing entry in the cli ANSI map (117 / 114 / 179 / 203 / 246 / 60) or an existing `var(--…)` token (per design-system §Surface: cli / Tokens).
- (design) Every capture status or outcome pairs its color with an ASCII bracket label, and the output's meaning is unchanged under `NO_COLOR` / `TERM=dumb` / piped stdout (per design-system §Surface: cli / Tokens; §Anti-Patterns / Per-Surface Bans / cli).
- (design) Fingerprints, run_id and P-IDs in any rendered capture output use the reserved mono status tier (ANSI 117 / `--color-id-cyan`), never prose styling or a generic accent (per design-system §Typography, Data row).
- (design) A Pulse-side (hand-off) or unmeasured result renders as `Blocked`/`KnownResidual` with its named precondition or "expected until {fix}" note — never as a red `Fail` and never silently greyed the same as an empty state (per design-system §Color Palette, Verdict-vs-ReportState note).

## Relevant amendment history
- **2026-08-09-out-of-scope-classification-treatment** (§cli Tokens, Residual-mute; §cli Component Patterns 3): a prior chunk needing a new de-emphasis treatment REUSED ANSI 246 / `var(--status-residual)` with zero new tokens and zero new ANSI entries; the same chunk retitled the results table and recorded that the coverage matrix is a separate 4-column table with no verdict column. Directly governs this chunk: a capture's recessive/qualifier text reuses that pair, and capture data does not belong in the coverage table.
- **2026-08-09-sut-load-envelope** (§Color Palette, Residual-mute): the run-level `[ENVIRONMENT-SUSPECT]` caption was added as a *third non-lamp reuse* of 246 — a run-level qualifier, explicitly "never a seventh lamp or a sixth ReportState". Precedent for how this chunk should express a capture-level qualifier if one is needed.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§cli Component Patterns 5): established that `error:`/`hint:` reuse Fail red (203) + Residual mute (246) on a stderr-specific `IsTerminal` gate, and that a proposed new "Hint grey" palette row was CORRECTED away at validation. Applies verbatim to any error/hint edge this capture adds.
- **2026-08-08-sut-capability-manifest** / **2026-08-09-current-sut-coverage-classification** (§Brand Identity, §Typography, §cli ANSI map, §Component Patterns 3): de-hardcoding sweeps — name the set, never a fresh literal count. Applies if capture prose records span/fingerprint counts (e.g. "9 spans", "six exception spans") in any design-owned copy.
