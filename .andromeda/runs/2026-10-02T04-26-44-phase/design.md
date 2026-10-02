# design extract

## Relevance
partial: the chunk is a headless live round plus a harness-shell subdir CARRY, with no webview and no token surface. Design binds only where the round PRINTS or COMMITS graded status: the six assertions' PASS/FAIL/UNGRADED lines in test output and the committed evidence. Whether the shipped graders emit any status text at all is research's question.

## Constraints
- A graded status line is never color alone. Every per-assertion outcome carries its ASCII bracket prefix from the closed cli set (`[PASS]` / `[FAIL]` / `[HOLD]` / `[BLOCKED]` / `[MANUAL]` / `[RESIDUAL]`), and any tint is TTY-gated per stream behind `NO_COLOR` / `TERM` / IsTerminal (per design-system §Surface: cli → Tokens (platform-specific); §Surface: cli → Platform-Specific Notes).
- "Not measured" must stay visually and textually distinct from "failed". The round's UNGRADED outcome (an absent sample, round-request §Grading posture) must never render as or collapse into `[FAIL]`. The plan's nearest existing state is `Blocked`, "never measured", which carries a named reason (per design-system §Anti-Patterns → Rejected Defaults, "Conflating 'no result yet' with 'failed'"; §Color Palette → Semantic Colors, the Verdict (3) vs ReportState (5) note). The plan defines no UNGRADED lamp or prefix. Whether the round maps UNGRADED onto `Blocked` or prints it as a non-lamp caption is a P4 decision, and it must not mint a seventh lamp state (per design-system §Surface: cli → Component Patterns #4, "never a seventh lamp or a sixth `ReportState`").
- A FAIL is recorded calmly: an in-place `[FAIL]` line with no alarm treatment, no blink and no flashing (per design-system §Brand Identity → Personality; §Anti-Patterns → Rejected Defaults, "flashing / pulsing red banner").
- The headless source-of-truth path is never blocked on an interactive prompt. A live leg driven through `agent-run` / nextest must not gate on `inquire` (per design-system §Surface: cli → Component Patterns #2; §Per-Surface Bans → cli).
- Data and messages stay on separate streams: raw artifact or measured data goes to stdout or the committed evidence file, and human messages and `error:`/`hint:` labels go to stderr, sanitized of host paths (per design-system §Surface: cli → Component Patterns #5; §Per-Surface Bans → cli).
- Machine-parseable or piped output carries no emoji and no ANSI. A committed evidence file is piped-equivalent and uses ASCII prefixes only (per design-system §Per-Surface Bans → cli; §Surface: cli → Tokens, "TTY only — never emoji in machine-parseable piped output").

## Patterns to follow
- The per-P-ID verdict line shape: glyph, P-ID, state word, measured `latency_ms` (or the budget value), then the tier or bound. For this round each of P-025/P-027/P-037/P-045 is graded at its measured worst value against its stated bound (per design-system §Surface: cli → Component Patterns #4).
- A `Blocked`-class row carries its named reason, and its measurement cells render as `—`/null rather than as a red error. Use the same treatment for an UNGRADED assertion and name the absent metric (per design-system §Surface: cli → Component Patterns #3).
- When the round has to qualify the whole run rather than one assertion (for example, the expected no-`app.exit` after a `Stop-Process -Force` teardown), use the run-level non-lamp caption convention. That convention is ANSI 246, sits outside the lamp column and carries its text label (per design-system §Surface: cli → Tokens, the `[ENVIRONMENT-SUSPECT]` / `[PRECONDITION]` caption set). Whether a caption is warranted at all is P4's call.

## Anti-patterns to avoid
- Rendering an absent or UNGRADED sample as `[FAIL]` or in fail red, or counting it as met (per design-system §Anti-Patterns → Rejected Defaults, "Conflating 'no result yet' with 'failed'").
- Emoji or ANSI escape codes in the committed evidence or in piped test output (per design-system §Per-Surface Bans → cli).
- A new status color or prefix invented for this round outside the closed set (per design-system §Anti-Patterns → Universal Bans, "NEVER use color purely for decoration"; §Surface: cli → Tokens).

## Contract bindings
- design ↔ a11y: the not-color-alone bracket prefixes bind to a11y §Use of Color (SC 1.4.1) for the NO_COLOR / screen-reader reading of the round's output.
- design ↔ arch/tests: the state vocabulary binds to the Verdict/ReportState typed values in the run-report envelope (arch §Standard Contracts; test-plan §3). An UNGRADED outcome must map onto that envelope without minting a sixth `ReportState` (per design-system §Color Palette → Semantic Colors note).
- design ↔ security/obs: the stderr `error:`/`hint:` sanitization (no absolute host paths) binds to security-plan §Error Handling and the obs field-allowlist (per design-system §Surface: cli → Component Patterns #5).
- design ↔ Pulse (out of Conductor's UI scope): P-037 (the Report window) and P-045 (the Findings counter) are PULSE's UI surfaces. Conductor's design system does not govern them, and the scope law forbids automating Pulse's UI. Their samples are read back, never driven by Conductor UI automation (per design-system §Surface: cli, which scopes Conductor's own surfaces only).

## Acceptance criteria contributions
- (design) Every graded assertion line the round prints or commits carries a text state prefix from the closed set, never color alone, and contains no ANSI or emoji when piped or committed (per design-system §Surface: cli → Tokens (platform-specific); §Per-Surface Bans → cli).
- (design) An assertion with an absent sample reads as a distinct not-measured state that names the missing metric, and never reads as `[FAIL]` (per design-system §Anti-Patterns → Rejected Defaults, "Conflating 'no result yet' with 'failed'").
- (design) The live round completes without blocking on any interactive prompt when stdin is not a TTY (per design-system §Surface: cli → Component Patterns #2).
