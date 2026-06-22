# design extract

## Relevance
Partial — chunk covers scenario catalog entries (declarative TOML) and test wiring; most design output is test/config, not rendered surface.

## Constraints
- Per design-system §Motion: hard limit at expression 0.3 — no animation libraries, no spring physics; all transitions MUST drop under `@media (prefers-reduced-motion: reduce)` (design-system §Motion, binding to a11y SC 2.3.3).
- Per design-system §Iconography: status lamps must be visually distinct (filled dot / hollow ring / checkbox glyph / dashed ring) so no state silently reads as another; ManualCheck renders neutral-lavender checkbox glyph + operator-checklist card (not the machine-verdict triad).
- Per design-system §Color Palette §Semantic Colors: KnownResidual renders muted dashed-ring dot (`#9A93A8` dark / `#6E6478` light) + "expected until {named fix}" note, never red (design-system §Anti-Patterns: "Conflating 'no result yet' with 'failed'").
- Per design-system §Anti-Patterns (universal): never use color alone — every status carries an ASCII prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]`); applies to CLI output and any report-state rendering.
- Per design-system §Component Patterns §Verdict / report-state lamp: in-place per-P-ID status light resolves motionless (150ms color transition, never flashing); always paired with text label (`Pass`/`Fail`/`Blocked`/`Manual`/`Residual`) + `aria-live` announcement (a11y SC 1.4.1 use of color).
- Per design-system §Typography: P-ID rendering reserved for JetBrains Mono 500, 13px, tabular-nums, color ID-cyan `#7DCFFF` (or `#0969DA` light mode); Data tier spans P-001..P-060, run_id, SLO timings, fingerprints.
- Per design-system §Brand Identity: signature is the *frozen* heartbeat (count freezes at hold value, tints green→amber, resumes on proceed / dims slate-violet on abort); any operator-checklist render must honor the "absence of motion" as the loudest signal — no auto-progression, no hidden spinners.

## Patterns to follow
- Per design-system §Component Patterns §Verdict / report-state lamp: KnownResidual and ManualCheck lamps are first instances in the catalog; render the dashed-ring (Residual) and checkbox (Manual) glyphs as specified, paired with the muted text label and "expected until {fix}" contextual note for Residual.
- Per design-system §Component Patterns §Operator-checklist: ManualCheck rows expand into a card row with induced-state label + expected-observation checkboxes (e.g., "halo shifted toward burgundy? ☐"); `--color-raised-1` card with `--radius-md`, neutral-lavender `--status-manual` checkbox glyphs; space-toggles each row; ticking resolves that item's verdict while report-state stays ManualCheck (operator-confirmed).
- Per design-system §Typography / Surface-conditional: CLI mirror for P-IDs is ANSI 117 cyan (`#7DCFFF`), rendered via `owo-colors`; operator-checklist items in CLI use `[MANUAL]` ASCII prefix (ANSI 146 lavender) + `?` glyph + the expected observation, never color alone.
- Per design-system §Component Patterns §Paused-count hold-point (signature, CLI mirror): the `indicatif` spinner STOPS in place (not hides, not animates-to-100%) at the frozen value on operator-pause; bold ANSI 179 amber `HOLD — operator pause` phase line prints above the `inquire` prompt. The stop — not a hide — embodies the signature.

## Anti-patterns to avoid
- NEVER render ManualCheck or KnownResidual as red failures or use them to trigger alarm-state animations; ManualCheck is neutral-lavender (`#A9B1D6`) outside the verdict triad, KnownResidual is muted dashed-ring (`#9A93A8`) with "expected until {fix}" context (design-system §Anti-Patterns: "Conflating error-state with no-verdict-yet").
- NEVER auto-progress or auto-tick operator-checklist items; the operator's explicit space-toggle (keyboard-first) is the only valid input — no spinners, no inferred states, no hidden resolution.
- NEVER use generic status prefixes or drop context — every catalog entry for P-025/026/027/032/036 must carry its P-ID in the expected-outcome expression and SLO tier; TOML must surface the intended report state (ManualCheck vs. Auto vs. KnownResidual) so test wiring guards it correctly.

## Contract bindings
- (a11y §Use of Color SC 1.4.1 + Contrast SC 1.4.3): ManualCheck lamp (`#A9B1D6`) + KnownResidual lamp (`#9A93A8`) paired with text labels + ASCII prefixes; contrast ratio must meet 4.5:1 on the base surface `#1A1B26` (design-system §Color Palette §Semantic Colors table specifies the values; verify in implementation).
- (a11y §Animation SC 2.3.3): any transition in the operator-checklist card or status-lamp render MUST drop under `@media (prefers-reduced-motion: reduce)` — this includes the 150ms color transition on lamp tint changes (design-system §Motion).
- (MCP retrieve_report + query_incident_list): P-032 context section surfaces last-5 git commits (Conductor detector, Pulse producer stub until v0.3.0 — KnownResidual pre-accepted deviation); P-036 "Previously seen" fingerprint recurrence read-back declared (storage seam exists, live wiring downstream Epoch 10).

## Acceptance criteria contributions
- (design) ManualCheck and KnownResidual lamps render with visually distinct glyphs (neutral checkbox for Manual, muted dashed-ring for Residual) + paired text label + ASCII prefix (`[MANUAL]`/`[RESIDUAL]`), never color alone (design-system §Iconography + Color Palette + Anti-Patterns).
- (design) Operator-checklist card (ManualCheck scenario rows) renders on `--color-raised-1` fill with `--radius-md`, uses space-toggle (keyboard-first, no auto-progression), ticking resolves verdict while report-state stays ManualCheck (design-system §Component Patterns §Operator-checklist).
- (design) KnownResidual P-032 context note ("expected until {named fix}") surfaces in run-report alongside the dashed-ring lamp, rendering the muted text tier (`--text-muted` / `#565F89`) to signal "documented pre-accepted gap, not a surprise failure" (design-system §Anti-Patterns + §Component Patterns §Run-report view).
- (design) Operator-checklist / ManualCheck / KnownResidual renders respect `@media (prefers-reduced-motion: reduce)` (no 150ms color transition, instant state change); a11y Motion binding verified (design-system §Motion + a11y SC 2.3.3).

## Relevant amendment history
2026-06-15 (§Surface: desktop-webview / Tokens): `:root` token declaration (not `@theme`) to preserve all 34 tokens; no impact on color/space/radius/motion values cited here — token NAMES + VALUES binding contract unchanged.