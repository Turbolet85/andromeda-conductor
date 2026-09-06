# design extract

## Relevance
Partial — this chunk adds no new rendering surface, but design binds to the invocation's CLI output lexicon, the operator gate's prompt discipline, and the color-free status rendering in its evidence artifact.

## Constraints
- If the invocation lands as a `conductor-cli` / `agent-run` verb (an admissible W1 shape), design-system §Surface: cli / Tokens + §Platform-Specific Notes require styling via `owo-colors` behind a per-stream `std::io::IsTerminal` gate — decided independently for stdout and stderr, each requiring a TTY **and** `NO_COLOR` unset **and** `TERM != dumb` — so piped/agent-captured output carries zero ANSI. Whether the existing four firing forms already do this is research's question.
- design-system §Surface: cli / Tokens requires status to be ASCII-prefix + color, never color alone: the closed per-P-ID lamp set `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`, plus the run-level non-lamp caption set (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) which sits outside the lamp column and is neither a lamp state nor a `ReportState`.
- The lamp set is closed at six and the palette takes no new rows: W1's "refuses to run a partial env rather than degrading to a silent `[BLOCKED]`" must render either as the `[BLOCKED]` lamp carrying its **named precondition string**, or as a run-level caption in the existing Residual-mute tier (ANSI 246 ↔ `var(--status-residual)`) in the `[PRECONDITION]` shape — no seventh lamp, no new ANSI entry (per design-system §Surface: cli / Tokens Residual-mute entry + §Color Palette Verdict-vs-ReportState note).
- design-system §Anti-Patterns → Rejected Defaults ("conflating no-result-yet with failed") + §cli Component Patterns 3 require a `Blocked`/refused leg to keep measurement columns as `—`/null and never downgrade to a red error — the never-measured state stays distinct from `Fail`.
- design-system §Surface: cli / Component Patterns 2 + §Per-Surface Bans (cli) require that any `inquire` operator gate be behind an `isatty` check and that the headless agent-driven path is **never** blocked on a prompt — the decision is skipped per the configured non-interactive policy and recorded to the artifact instead.
- If the invocation carries an `indicatif` spinner/count through a hold, design-system §Brand Identity (Signature element) + §cli Component Patterns 1 require the spinner to **STOP in place at the exact value** (never hide, never animate to 100%), with a bold amber `HOLD — operator pause` line printed above the prompt; spinner appears only after ~200ms and is TTY-gated so captured artifacts stay clean.
- design-system §cli Component Patterns 5 requires stderr error output to be sanitized (no absolute host paths, no internal struct names, no stack traces outside `--debug`/`-v`) with `error:` reusing Fail red (203) and `hint:` the Residual mute (246) — this is the design face of W2's host-path-free artifact-hygiene requirement.

## Patterns to follow
- cli Component Pattern 2 (operator-pause prompt) — the existing design shape for "operator-gated": TTY confirm for a human, recorded non-interactive decision for the headless path; the same shape W1's gate should compose rather than invent.
- cli Component Pattern 1 (Paused-count hold-point, CLI mirror) — the stops-not-hides discipline for any live progress the composed legs surface.
- cli Component Patterns 3 + 4 — `comfy-table` with terminal width detected dynamically (never hardcoded), the `slo_tier` cell drawn from the scenario's declared value in the closed `<5s`/`<20s`/`<90s` set, and the results table kept distinct from the 4-column `conductor coverage` table (no verdict/state column there).
- Set-framing for run-level captions — when a new run-level qualifier is needed, it joins the named Residual-mute non-lamp SET with an always-rendered bracket label carrying the signal and the tint only de-emphasizing (design-system §cli Tokens, Residual-mute entry).
- Markdown evidence rendering — design-system §cli Tokens states Markdown has no color channel, so the surface-adapted counterpart is emphasis (`_…_`) or the bracket label in a blockquote; W2's evidence record should use that, not an invented convention.

## Anti-patterns to avoid
- Never hide the progress spinner or animate it to 100% at the operator gate (design-system §Per-Surface Bans → cli; §Anti-Patterns → Rejected Defaults, the frozen-heartbeat rejection).
- Never colorize without the `NO_COLOR` / `TERM` / pipe checks, never rely on color alone, and never use emoji in machine-parseable piped output — ASCII prefixes only there (design-system §Per-Surface Bans → cli).
- Never block the headless source-of-truth path on an interactive prompt, and never mix raw artifact data (stdout) with human messages (stderr) unintentionally (design-system §Per-Surface Bans → cli).

## Contract bindings
- design ↔ a11y: the ASCII bracket prefixes are the not-color-alone (SC 1.4.1) mechanism for the cli surface; if the driven a11y arm exercises the shipped webview, the token-contrast and `prefers-reduced-motion` obligations stay owned by a11y — this chunk adds no motion.
- design ↔ tests/harness (W2): the piped-stdout ANSI-strip gate is what makes leg lines parseable for the harvest-tier frozen-line grading; an ungated ANSI escape in captured output would break the capture/grade split.
- design ↔ security/artifact hygiene (W2): the sanitized `error:`/`hint:` stderr shape (no host paths, no stack traces in normal mode) is the design rendering of the host-path-free evidence invariant.
- design ↔ obs: a `[BLOCKED]`/refused leg must render the *named* precondition string the run records — the design contract assumes such a reason string exists to print.

## Acceptance criteria contributions
- (design) Every status the invocation prints carries its ASCII bracket label from the closed set, and output that is piped or run under `NO_COLOR`/`TERM=dumb` contains zero ANSI on both stdout and stderr (per design-system §Surface: cli / Tokens + §Anti-Patterns → Per-Surface Bans → cli).
- (design) The refuse-on-partial-env path renders as either the `[BLOCKED]` lamp with its named precondition string or an existing Residual-mute (ANSI 246) run-level caption in the `[PRECONDITION]` shape — no new palette row, no new ANSI entry, no seventh lamp state (per design-system §Surface: cli / Tokens, Residual-mute entry + §Color Palette Verdict-vs-ReportState note).
- (design) Any operator gate is `isatty`-checked; the headless invocation runs to completion without prompting and records the decision to the artifact (per design-system §Surface: cli / Component Patterns 2).
- (design) If the invocation surfaces an `indicatif` spinner/count and reaches a hold, the spinner stops in place at the frozen value with the amber `HOLD — operator pause` line above the prompt — never hidden, never animated to 100% (per design-system §Surface: cli / Component Patterns 1 + §Motion, high-impact moments).

## Relevant amendment history
- **2026-09-03-live-pulse-preconditions-probed** (nearest neighbour — same live-Pulse/preflight area): `[PRECONDITION]` joined the run-level non-lamp caption SET and the Residual-mute (246) reuse list; `anstream`/`anstyle` retired as the named cli gate in favour of `owo-colors` + per-stream `std::io::IsTerminal`. Why it matters here: this chunk composes the preconditions/preflight legs, so the refusal signal has an established, already-registered rendering — reuse it rather than mint one.
- **2026-08-09-sut-load-envelope**: `[ENVIRONMENT-SUSPECT]` added as a run-level caption in the same Residual-mute tier, recorded as a SET rather than a count. Establishes the precedent that new run-level qualifiers are captions, not lamp states.
- **2026-08-09-out-of-scope-classification-treatment**: recorded the Residual-mute tier's non-lamp reuses (with `var(--status-residual)` as the webview half of the by-name pair) and disambiguated the 6-column results/SLO table from the 4-column `conductor coverage` table.
- **2026-08-18-error-baseline-spike-live-proof** (prior live-proof chunk): the cli Pattern 4 sample tier was de-literalized to `<slo_tier>` from the closed set — no baked per-scenario literal in printed leg lines.
- **2026-06-24-sanitized-stderr-agent-mode-logging**: fixed `error:`/`hint:` as reuse of existing tokens (203/246) on a stderr-specific `IsTerminal` gate, with the ASCII labels always present — the standing shape for this chunk's failure/refusal messaging.
