# design extract

## Relevance
Partial (narrow) — this chunk renders nothing new; design bears only on the semantic status values the killing assertions pin, and on any rendered/artifact text a new test reads back.

## Constraints
- `ReportState::Blocked` must stay a distinct outcome that never collapses into `Fail` and is never shown as a red error — design-system §Color Palette (Verdict-vs-ReportState note) + §Anti-Patterns → Rejected Defaults require this, so the `declares` survivor's killing assertion must pin the distinct typed `Blocked`, not merely "not Pass" (per design-system §Color Palette).
- A `Blocked` row is required to carry the **named precondition string**, with measurement columns rendering `—`/null rather than an error (per design-system §Surface: cli → Component Patterns 3), so the value asserted for an unmet `shell-declaration` term must be the one that names the term — whether the current code already produces a term-naming `Blocked` is research's question.
- Status signalling is required to be text-first: the closed ASCII prefix set `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` plus `✓`/`✗`/`?`/`~`/`•`/`→`, color never alone (per design-system §Surface: cli → Tokens + §Per-Surface Bans → cli). This chunk must introduce no seventh state string, lamp, or color token.
- Human-facing error output is required to be `error: <short>` + detail + `hint: <fix>`, sanitized of absolute host paths, internal struct names and stack traces (per design-system §Surface: cli → Component Patterns 5 + §Per-Surface Bans → cli) — this is the design half of the chunk's "no host paths or internal struct names in any artifact a test writes or reads back" invariant, and it governs any `VerifyError` surfaced by the `MAX_LINE_BYTES` bound.
- CLI color is required to be TTY-gated and ANSI-stripped when piped, honouring `NO_COLOR`/`TERM=dumb` (per design-system §Surface: cli → Tokens + §Per-Surface Bans → cli), so captured (non-TTY) output under either runner is plain ASCII — assertions must match labels/prefixes, never ANSI sequences.
- The headless, agent-driven path is required never to block on an interactive `inquire` prompt and every prompt is required to be `isatty`-gated (per design-system §Surface: cli → Component Patterns 2 + §Platform-Specific Notes); any new deterministic test must not depend on TTY-only rendering.

## Patterns to follow
- The per-P-ID verdict/report-state line shape (`✓ P-009  Pass  1840ms <slo_tier>` … `• P-022  Blocked  <named precondition>`) with its always-paired bracket label (per design-system §Surface: cli → Component Patterns 4) — the existing shape any assertion over rendered result text should match rather than invent.
- Results/SLO table vs `conductor coverage` table are separate surfaces; the coverage table carries no verdict/state column and no bracket prefix (per design-system §Surface: cli → Component Patterns 3) — do not assert a state column against coverage output.
- The recessive Residual-mute tier (ANSI 246 ↔ `var(--status-residual)`) is the shared by-name pair for non-lamp de-emphasis (`hint:` label, out-of-scope Mode cell, `[ENVIRONMENT-SUSPECT]` caption) where the always-rendered text carries the signal (per design-system §Surface: cli → Tokens) — the precedent for adding meaning without adding a state.
- Signature/motion discipline is untouched by this chunk: no rendering, no animation, no token surface is added (per design-system §Motion hard limits).

## Anti-patterns to avoid
- NEVER conflate "no result yet"/never-measured with failed, or let `Blocked` read as red `Fail` (per design-system §Anti-Patterns → Rejected Defaults) — including in test fixtures or expected-value literals.
- NEVER rely on color alone, and never add a status string outside the closed bracket set (per design-system §Per-Surface Bans → cli).
- NEVER let stack traces, absolute host paths, or internal struct names reach normal-mode output or a written artifact (per design-system §Per-Surface Bans → cli).

## Contract bindings
- design §Surface: cli → Component Patterns 5 (sanitized `error:`/`hint:`) ↔ **security** plan's sanitization rule ↔ this chunk's artifact invariant — design mandates the rendered shape, security owns the redaction rule.
- design §Color Palette (Verdict 3 / ReportState 5) ↔ **architecture**'s run-report envelope (design cites it explicitly) — the typed `Verdict`/`ReportState`/`VerifyError` values the killing tests assert are arch-owned; design owns only how they must read on the surface.
- design §Surface: cli → Tokens TTY/`anstream` gating ↔ **obs/tests harness** — the same process-global vs per-test isolation question raised by the runner-portability item; whether captured CLI output is already stripped and stable under `cargo test` is research's question.

## Acceptance criteria contributions
- (design) The `declares` killing assertion pins a distinct typed `Blocked` carrying the named unmet term — never `Fail`, never an untyped "not Pass" (per design-system §Color Palette, Verdict-vs-ReportState note).
- (design) No new status/verdict string, lamp state, ANSI entry, or color token is introduced; the closed `[PASS]/[HOLD]/[FAIL]/[MANUAL]/[RESIDUAL]/[BLOCKED]` set is unchanged (per design-system §Surface: cli → Tokens).
- (design) Any assertion over rendered CLI text matches the ASCII label/prefix, not ANSI codes, so it holds under `NO_COLOR`, piped capture, and both runners (per design-system §Surface: cli → Tokens + §Platform-Specific Notes).
- (design) Artifacts written or read back by new tests contain no absolute host paths, internal struct names, or stack traces (per design-system §Surface: cli → Component Patterns 5 + §Per-Surface Bans → cli).

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** — recorded that the `error:`/`hint:` labels reuse shipped tokens (Fail red 203 / Residual mute 246) on a stderr-specific `IsTerminal` gate, labels always present, never color-alone. Relevant because this chunk's "no host paths / no struct names in artifacts" invariant lands on exactly that surface, and because the correction there was "reuse the existing token, add no palette row".
- **2026-08-09-out-of-scope-classification-treatment** — split the Results/SLO table from the 4-column `conductor coverage` table (which carries no state column) and recorded Residual-mute's non-lamp reuses. Relevant so any assertion targets the right table and adds no state column.
- **2026-08-09-sut-load-envelope** — `[ENVIRONMENT-SUSPECT]` was admitted as a run-level qualifier, explicitly "never a seventh lamp or a sixth `ReportState`", and the non-lamp uses were named as a SET rather than a count. The governing precedent for this chunk: add meaning without adding a state.
- **2026-08-18-error-baseline-spike-live-proof** — the cli Pattern 4 sample tier was de-literalized to `<slo_tier>` from the closed set. Relevant if any killing/regression test asserts a result line: do not bake a per-scenario literal.
- **Cross-cutting precedent** (2026-06-15 `@theme`→`:root`, 2026-06-26 200ms→`--motion-micro`) — the standing "spec-illustration → sound-impl reconciliation" routine: where an illustrative spec value is unrealizable, source wins and the divergence is recorded as an amendment. This matches this chunk's own promotion rule that source beats the audit's characterisation.
