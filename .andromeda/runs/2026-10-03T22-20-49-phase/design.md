# design extract

## Relevance
partial. The chunk adds no desktop-webview surface, token, motion or component. It runs a live MCP read-back round and pins a fifth required tool. Design binds only where the round writes or prints a graded status: committed evidence verdicts, any live-leg summary line, and the existing preflight `Blocked` row that a missing `retrieve_incident_events` would raise.

## Constraints
- Every graded status the round renders carries its text label. The CLI form is an ASCII prefix from the closed set (`[PASS]` / `[FAIL]` / `[BLOCKED]` / ...). Markdown evidence has no color channel, so the bracket label or emphasis carries the signal. The seven assertions' verdicts in this round's committed evidence follow the same rule (per design-system §Surface: cli → Tokens (platform-specific); per design-system §Color Palette → Semantic Colors).
- `Blocked` (never measured) and an UNGRADED absent sample must never collapse into `Fail`. Blocked is its own state and carries the named precondition string. The round's grading posture says an absent sample is UNGRADED and never met. Its rendering must therefore stay distinct from a FAIL verdict and must not be greyed the same as a FAIL (per design-system §Anti-Patterns → Rejected Defaults, "Conflating 'no result yet' with 'failed'"; per design-system §Color Palette → Semantic Colors, the Verdict vs ReportState note).
- A preflight against a Pulse that lacks the newly pinned tool surfaces through the EXISTING missing-tool precondition. On the CLI it renders as a `Blocked` row: the named precondition string, measurement columns as `—`/null, never a red error (per design-system §Surface: cli → Component Patterns 3 and 4). Whether the shipped renderer already names the missing tool generically, so the fifth name needs no rendering change, is research's question.
- Any human-facing line the leg prints (for example a summary) obeys the CLI rules. Data goes to stdout and messages to stderr. Piped output has no emoji, and color is TTY-gated behind `NO_COLOR` / `TERM` / `IsTerminal`. Whether `p075_round_live` prints anything beyond test-harness output is research's question (per design-system §Surface: cli → Platform-Specific Notes; per design-system §Anti-Patterns → Per-Surface Bans, cli).
- Fail is a calm, motionless verdict: no alarm framing, no flashing. A Pulse-side FAIL recorded by this round is stated in place as a finding (per design-system §Anti-Patterns → Rejected Defaults, "flashing / pulsing red banner"; per design-system §Color Palette → Semantic Colors).

## Patterns to follow
- The verdict/report-state line shape: glyph (TTY only) + P-ID + state label + measured value + tier (per design-system §Surface: cli → Component Patterns 4). This is the model for how a per-assertion result is stated beside its measured value.
- The Blocked row carries the named precondition string, never the generic corpus-empty text (per design-system §Surface: cli → Component Patterns 3).
- The Markdown counterpart to a color-only cue is emphasis or the bracket label in a blockquote (per design-system §Surface: cli → Tokens (platform-specific), the Residual-mute note).

## Anti-patterns to avoid
- Color-alone status, or emoji, in any piped or committed artifact (per design-system §Anti-Patterns → Per-Surface Bans, cli).
- Rendering an UNGRADED or Blocked assertion as a Fail (per design-system §Anti-Patterns → Rejected Defaults).

## Contract bindings
- The state label alongside color binds to the a11y plan's use-of-color rule (SC 1.4.1, not color alone).
- The rendering of the Blocked precondition string binds to the security plan's sanitization of operator-facing strings: no host paths and no internal struct names.
- The per-assertion verdict labels bind to the tests domain's round evidence. Those are the stable test ids and the committed evidence path Pulse cites.

## Acceptance criteria contributions
- (design) Each of the seven assertion verdicts in this round's committed evidence carries an explicit text label (PASS / FAIL / UNGRADED or equivalent), never a color or glyph alone (per design-system §Surface: cli → Tokens (platform-specific)).
- (design) An absent sample is recorded with a label that is distinct from FAIL, and a missing-tool preflight renders as `Blocked` with its named precondition, never as Fail (per design-system §Anti-Patterns → Rejected Defaults).
- (design) Any line the live leg prints carries no emoji and no ANSI when piped (per design-system §Anti-Patterns → Per-Surface Bans, cli).
