# design extract

## Relevance
partial. The chunk adds no UI surface, token or motion. Its design exposure is how the leg's outcome rows, its preconditions output and any harness faults render on the cli. On the webview it only adds data rows to the existing lamp and picker components. The plan limits all of these to a closed status vocabulary.

## Constraints
- **Closed status vocabulary.** design-system §Color Palette → Semantic Colors (the Verdict-3 vs ReportState-5 note), §Iconography (the six lamp treatments) and §Surface: cli → Tokens (the closed per-P-ID prefix set and the ANSI map) require every outcome the leg reports to render through the existing lamps, prefixes, ANSI codes and tokens. That covers the declare-only row and each posture grading outcome. Nothing leg-specific may be added.
- **A model miss or an unreached assertion is never red.** design-system §Color Palette → Semantic Colors maps the model-interpretive `CalibrationRegion` to amber. It keeps `ManualCheck` (lavender, outside the triad) and `Blocked` (slate-violet, never measured) separate from `Fail`. §Surface: cli → Component Patterns 3–4 require a `Blocked` row to carry its named precondition string, with `—`/null in the measurement cells. §Surface: cli → Component Patterns 5 makes the sanitized stderr `error:`/`hint:` pair the channel for a Conductor harness fault. It is not a verdict lamp.
- **The declare-only row renders as `KnownResidual`.** design-system §Surface: cli → Component Patterns 4 (`~` + `[RESIDUAL]`, ANSI 246) and §Surface: desktop-webview → Component Patterns 4 & 6 (`--status-residual` dashed-ring dot plus a note, never red) apply.
  - The plan defines the residual note as "expected until {named fix}", for a measured spec-vs-runtime gap. This leg's residual is a claim graded one tier out, with no named fix.
  - What note the row carries is research's question. So is whether the declare-only shape the posture says the system already uses renders one.
  - If the shipped meaning is broader than the plan's definition, that is a design-drift candidate at wrap.
- **The tier cell uses the closed set only.** design-system §Typography (Data row) and §Surface: cli → Component Patterns 3–4 require the tier cell to show the scenario's declared value from `<5s`/`<20s`/`<90s`, never a baked literal. There is no fourth tier and no formation-figure literal in any rendered cell. Research must answer whether a declare-only scenario still has to declare a tier. If it does, the row would print a deadline it is not graded against next to a latency above it.
- **Output stays clean when piped.** Everything the leg and the operator-gated arm print is governed by these rules, including any messages during the quiet-window wait:
  - design-system §Surface: cli → Tokens: a separate `std::io::IsTerminal` gate for stdout and for stderr. Color needs a terminal, `NO_COLOR` unset and `TERM != dumb`.
  - §Surface: cli → Component Patterns 1: the spinner is TTY-gated so agent-captured artifacts stay clean.
  - §Surface: cli → Navigation Pattern.
  - §Anti-Patterns → Per-Surface Bans → cli: data on stdout and messages on stderr, no emoji when piped, widths detected from the terminal, stack traces only under `--debug`/`-v`.
- **The headless path never waits on a prompt.** design-system §Surface: cli → Component Patterns 2 and Platform-Specific Notes require every `inquire` confirm to check `isatty`. On the agent-driven path the decision is recorded instead of prompted. §Surface: cli → Component Patterns 4 records a headless `ManualCheck` as unconfirmed. So the real-model arm's operator gate has to be part of how the arm is invoked, not an interactive prompt.
- **Run-level captions and sanitized errors.** These follow design-system §Surface: cli → Tokens (the Residual-mute row and the status-prefix line) and Component Patterns 4–5:
  - Any check the `preconditions` short-circuit newly surfaces (for example the deterministic-L4 shell-declaration) prints under the existing `[PRECONDITION]` caption: ANSI 246, outside the lamp column, neither a lamp nor a ReportState.
  - A storm profile that breaches the SUT load envelope prints `[ENVIRONMENT-SUSPECT]` once, above the verdict lines, never as a seventh lamp.
  - stderr never contains absolute host paths. That includes the operator's `ANDROMEDA_PULSE_DATA_DIR` value.

## Patterns to follow
- **Per-P-ID verdict line** (design-system §Surface: cli → Component Patterns 4; §Color Palette → Core Colors, Primary): glyph, then the P-ID in mono ID-cyan (ANSI 117), then the state word, then detail. The plan's `~ P-032 … Residual` and `• P-022 … Blocked` samples are the target shapes for this leg's residual and unreached rows.
- **Results/SLO table vs coverage matrix** (design-system §Surface: cli → Component Patterns 3):
  - A Blocked row carries its precondition string and `—`/null measurement cells, with widths taken from the terminal.
  - The coverage matrix is a separate 4-column table with no verdict column and a derived roll-up.
  - If naming a diagnostic-quality cluster P-ID moves the `UNBACKED_AUTO` classification, the change renders through that table unchanged. P3/P4 settle where that boundary falls.
- **Reuse the Residual-mute tier for non-lamp text** (design-system §Surface: cli → Tokens, Residual-mute row): the plan makes ANSI 246 ↔ `var(--status-residual)` the shared low-emphasis tier. A new caption joins it as a set member and never gets a new hue.
- **ManualCheck as an operator checklist** (design-system §Surface: desktop-webview → Component Patterns 6–7; §Surface: cli → Component Patterns 4):
  - If Conductor ever renders a graded miss, it takes the checklist shape: the induced state (the injected cause) plus the expected observation as a y/n item.
  - The plan defines ManualCheck as having no machine verdict. It does not say which lamp or prefix wins when a row pairs a `CalibrationRegion` verdict with a `ManualCheck` state, which is exactly the posture's miss row.
  - Research must establish whether Conductor renders the graded outcome at all, or whether it lives only in the harvest-tier test.
- **Placeholder over literal, name the set** (design-system §Surface: cli → Component Patterns 4 `<slo_tier>`; §Depth Strategy, where measured figures are recorded against the verification matrix): the confirmed formation figure is a measurement record. It never becomes a literal in a design doc.

## Anti-patterns to avoid
- Rendering the model's miss, a degraded report or a failed read-back as red `Fail`/`[FAIL]`, or treating `Blocked` as a failure. Per design-system §Anti-Patterns → Rejected Defaults: "conflating 'no result yet' with 'failed'" and the flashing or pulsing red Fail treatment.
- Minting a bracket label, lamp, color, ANSI code or tier literal just for this leg, such as a `[REAL-MODEL]` prefix or a tier above 90 s. Per design-system §Anti-Patterns → Universal Bans (no decorative color) and the closed sets in §Surface: cli → Tokens.
- Making the agent-driven drive wait on an interactive prompt, or letting ANSI codes, emoji or spinner frames into captured output. Per design-system §Anti-Patterns → Per-Surface Bans → cli.

## Contract bindings
- **design ↔ posture contract.** The closed status vocabulary (§Color Palette → Semantic Colors; §Surface: cli → Tokens) binds to `contracts/pulse-real-model-leg-posture.md` §The grading rule, whose outcome table is drawn from the closed verdict triad and the five-valued report state. It also binds to that contract's §What this posture does not do, which mints no verdict word, report-state, bracket label or colour. Both documents hold the same closed set.
- **design ↔ tests.** The pipe-clean cli output rules (§Surface: cli → Tokens, Navigation Pattern) bind to the harvest-tier test, which reads the leg's verbatim captures. Those captures must carry the ASCII labels and never signal by color alone. Whether the capture path is piped or a TTY is research's question.
- **design ↔ a11y.** State color plus text label (§Iconography rule; §Surface: cli → Platform-Specific Notes) binds to a11y §Use of Color SC 1.4.1 (not color alone) for every row the leg produces.
- **design ↔ posture ↔ security.** cli Component Patterns 2 (the headless path never waits on a prompt) binds to posture §The launch posture: operator-invoked is a property of how the arm is invoked, not a prompt. It also binds to the security plan, since a prompt on the headless path silently breaks the release gate.
- **design ↔ posture (desktop picker).** desktop-webview Component Patterns 5 (the scenario/suite picker) binds to posture §The launch posture: the leg is reachable only from the operator-gated arm, with no default path. Research must establish whether the desktop picker lists the `scenarios/` catalog and would expose the leg outside that arm. The design plan provides no picker affordance for this leg.

## Acceptance criteria contributions
- (design) Every line the leg renders uses only the closed per-P-ID prefix set and the existing ANSI map. The declare-only row renders as `~`/`[RESIDUAL]`/ANSI 246, and an unreached leg as `•`/`[BLOCKED]`/ANSI 60 with its named precondition. The chunk's diff adds no bracket label, ANSI code, CSS custom property or hex value (per design-system §Surface: cli → Tokens; §Color Palette → Semantic Colors).
- (design) No graded outcome of the leg (a miss, a degraded report, a failed read-back) renders `[FAIL]`, ANSI 203 or `--status-fail`. ANSI 203 appears only on the stderr `error:` label for a Conductor harness fault (per design-system §Surface: cli → Component Patterns 4–5; §Anti-Patterns → Rejected Defaults).
- (design) The leg's tier cell holds no value outside `<5s`/`<20s`/`<90s` and no baked formation-figure literal (per design-system §Surface: cli → Component Patterns 3–4).
- (design) The agent-driven drive's non-TTY captured output contains no ANSI escape, emoji or spinner frame, keeps every ASCII status label, and finishes without an interactive prompt (per design-system §Surface: cli → Tokens + Component Patterns 1–2; §Anti-Patterns → Per-Surface Bans → cli).

## Relevant amendment history
- **2026-08-18-error-baseline-spike-live-proof.** cli Pattern 4's sample tier became the `<slo_tier>` placeholder (closed set, never a per-scenario literal) because an earlier live-proof leg re-declared its tier. This chunk is the next live-proof leg to touch that cell, and its leg is not graded against any tier.
- **2026-09-03-live-pulse-preconditions-probed.** `[PRECONDITION]` joined the set of run-level non-lamp captions (ANSI 246, framed as a set, lamp set kept at six). The named cli color gate became `owo-colors` plus a separate `std::io::IsTerminal` check per stream. This chunk touches the same `preconditions` short-circuit.
- **2026-06-24-sanitized-stderr-agent-mode-logging, 2026-08-09-out-of-scope-classification-treatment, 2026-08-09-sut-load-envelope.** Each new cli annotation (`hint:`, the out-of-scope Mode cell, `[ENVIRONMENT-SUSPECT]`) reused the Residual-mute tier as a set member instead of adding a palette row. The out-of-scope entry also separated the 6-column Results/SLO table from the 4-column coverage matrix. These are the precedent for any caption or classification change in this chunk.
- **2026-08-08-sut-capability-manifest, 2026-08-09-current-sut-coverage-classification, 2026-09-10-release-build-and-bundle.** Together they set the "name the set, never the new literal" rule for counts and measured figures; for example, installer sizes are recorded against the verification matrix, not baked into the plan. The rule covers the confirmed formation figure and any coverage count that changes if a cluster P-ID is named.
