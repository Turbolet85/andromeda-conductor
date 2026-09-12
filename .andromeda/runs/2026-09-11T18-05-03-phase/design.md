# design extract

## Relevance
Partial (thin) — the chunk's primary artifacts (`.github/workflows/ci.yml`, `scripts/webview2-cause-probe.ps1`, evidence records) sit outside both surfaces design-system.md declares (§Surface: desktop-webview, §Surface: cli — scoped to `conductor-cli` / `agent-run.sh`); design binds only on status/label vocabulary in any text the chunk emits or relocates, and on how a null/un-claimed outcome may be shown.

## Constraints
- The per-P-ID bracket-label set is CLOSED at six (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`), with run-level captions (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) the sanctioned form for a non-lamp qualifier — "a qualifier on the run, never a seventh lamp or a sixth `ReportState`" (per design-system.md §Surface: cli / Tokens). The scope's marker split (`[probe] complete: 3/3 sections`, whose gate contract it says must be settled explicitly) must not resolve by minting a string that reads as a lamp state.
- A not-yet-measured or retired-hypothesis outcome must never collapse into `Fail`: `Blocked` is its own slate-violet "present-but-greyed, never measured" state carrying a named precondition, and `ManualCheck` sits deliberately outside the green/amber/red triad (per design-system.md §Color Palette — the Verdict(3)-vs-ReportState(5) note). `v3-01` is `manual` mode and currently un-claimed; whether any surface already renders it — and in which state — is research's question.
- Color is never the sole signal and colorization is gated per stream: terminal AND `NO_COLOR` unset AND `TERM != dumb`, ASCII prefixes always standing, no emoji in machine-parseable (piped) output (per design-system.md §Surface: cli / Tokens + §Surface: cli / Platform-Specific Notes).
- Operator-facing error text is sanitized — `error: <short>` + `hint: <fix>` to stderr, no absolute host paths, internal struct names or stack traces outside `--debug`/`-v` (per design-system.md §Surface: cli / Component Patterns 5). This is the design-side echo of the scope's own `ModuleName` + `FileVersion`-only, never-`FileName` rule.
- No new palette row / ANSI entry / CSS token may be introduced for a new output edge — the existing map is reused by name (per design-system.md §Color Palette Rationale + §Surface: cli / Tokens, where `var(--status-residual)` ↔ ANSI 246 is a by-name pair).
- Recorded evidence rendered as Markdown has no color channel; its surface-adapted counterpart is emphasis or the bracket label in a blockquote, i.e. the always-rendered text label carries the signal and tint only de-emphasizes (per design-system.md §Surface: cli / Tokens, Residual-mute entry).

## Patterns to follow
- Run-level non-lamp caption: ANSI 246 (Residual mute), printed once above the per-P-ID lines, outside the lamp column, neither a lamp state nor a `ReportState` — the established precedent for adding a run-scoped qualifier string (design-system.md §Surface: cli / Tokens + Component Patterns 4).
- New output edge reuses shipped tokens rather than earning a color: the `error:`/`hint:` pair (ANSI 203 / 246) added a stderr surface with zero new palette entries, on a stderr-specific `IsTerminal` gate distinct from the stdout gate (design-system.md §Surface: cli / Component Patterns 5).
- Reserved mono ID tier: run ids, P-IDs/capability ids, SLO timings, version strings and fingerprints render in mono ID-cyan (ANSI 117 / `--color-id-cyan`), never as generic accent (design-system.md §Typography Data row + §Surface: cli / Tokens).
- `[MANUAL]` / `?` operator-checklist treatment for a claim with no programmatic read-back — induced state plus the expected observation, resolved by the operator, with the report-state staying `ManualCheck` (design-system.md §Component Patterns 7 + §Surface: cli / Component Patterns 4). Arm B is exactly this shape: an operator-run reading, explicitly "not modelled as a gate the fix-loop can drive."

## Anti-patterns to avoid
- Never rely on color alone and never use emoji in machine-parseable piped output (design-system.md §Anti-Patterns — Per-Surface Bans: cli).
- Never conflate "no result yet" with "failed" — showing a retired hypothesis, a null arm-B result or an un-claimed `v3-01` as red `Fail`, or as a gray placeholder, is an explicit Rejected Default (design-system.md §Anti-Patterns — Rejected Defaults).
- Never print stack traces in normal mode or leak absolute host paths / internal struct names to stderr (design-system.md §Anti-Patterns — Per-Surface Bans: cli + §Surface: cli / Component Patterns 5).

## Contract bindings
- **Never-color-alone → a11y SC 1.4.1 (Use of Color):** the ASCII bracket label, not the ANSI tint, is what satisfies it — design-system.md §Surface: cli / Tokens delegates the signal to the always-rendered label; a11y owns the criterion.
- **ANSI 246 ↔ `var(--status-residual)` by-name pair:** a recessive caption added on the cli side binds its webview counterpart by token name (design-system.md §Surface: cli / Tokens, Residual-mute entry) — no webview work is in this chunk's scope, so this binds only if a new caption is introduced.
- **Marker ↔ committed gate contract** (`[probe] complete: 3/3 sections`) is arch/testing's to settle; design constrains only the label vocabulary if the resulting string reads as a status (design-system.md §Surface: cli / Tokens).
- No motion / typography / spacing / radius / depth binding is triggered — this chunk renders no webview surface, so §Motion (incl. the `prefers-reduced-motion` override), §Typography and §Spacing do not apply.

## Acceptance criteria contributions
- No new lamp state or `ReportState` label is introduced by the marker split; the per-P-ID bracket set stays closed at six and any new run-scoped string is a non-lamp caption outside the lamp column (per design-system.md §Surface: cli / Tokens).
- Any status text this chunk adds or relocates carries its ASCII label independent of color, with colorization gated per stream on terminal + `NO_COLOR` unset + `TERM != dumb`, and no emoji in piped output (per design-system.md §Surface: cli / Platform-Specific Notes).
- Arm B's null outcome (elevation retired) and a still-un-claimed `v3-01` are represented as not-measured (`Blocked` / `[MANUAL]`), never as `Fail` red and never as a gray placeholder (per design-system.md §Color Palette — Verdict vs ReportState note).
- Zero new color/ANSI/CSS tokens: any tint on new or moved output reuses the shipped map by name (per design-system.md §Color Palette Rationale).

## Relevant amendment history
- **2026-09-03-live-pulse-preconditions-probed** — `[PRECONDITION]` joined the run-level non-lamp caption SET (ANSI 246), framed as a set rather than a count, with no new palette row and the lamp set still closed at six. The closest precedent for this chunk: a new probe-adjacent string was absorbed as a non-lamp caption, not a seventh lamp. The same entry retired `anstream` as the named cli gate in favour of `owo-colors` + `std::io::IsTerminal`, and deliberately refused to touch library versions outside its detector's scope.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — the `error:`/`hint:` edge REUSED ANSI 203/246; the detector's proposal to add a "Hint grey" palette row was CORRECTED at validation. Establishes the reuse-don't-add-color rule that constraint 5 above states.
- **2026-08-09-sut-load-envelope** and **2026-08-09-out-of-scope-classification-treatment** — successive non-lamp reuses of the Residual-mute tier, each recorded by NAMING THE SET rather than a literal count, and the cli tables disambiguated (results/SLO vs coverage).
- **2026-09-10-release-build-and-bundle** and **2026-08-18-error-baseline-spike-live-proof** — the derived-count rule: a measured figure (bundle size, `slo_tier`) is de-literalized and cited to its record, never baked into the doc. Directly applicable, since this chunk's deliverable is a recorded measurement; any design-doc restatement of the reading would be the drift these entries corrected.
