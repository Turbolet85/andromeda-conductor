# design extract

## Relevance
partial. The chunk renders no UI and changes no token, typography, motion or component. Its only design contact is how verdicts get rendered: the `[BLOCKED]` surface reached when a fifth `required_tools` name is pinned, and the verdict labels the round's graded output and committed evidence carry.

## Constraints
- Status is never color-alone. Every per-check result carries its text label. On cli it is the ASCII prefix from the closed per-P-ID set `[PASS]` / `[HOLD]` / `[FAIL]` / `[MANUAL]` / `[RESIDUAL]` / `[BLOCKED]`. Markdown has no color channel, so it carries the label as text or in its bracket form (per design-system §Surface: cli → Tokens (platform-specific); §Anti-Patterns → Per-Surface Bans → cli).
- That per-P-ID prefix set is CLOSED, and the run-level captions (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) are neither lamp states nor a `ReportState`. The scope's "absent sample is UNGRADED" therefore needs care. If any shipped cli or webview surface would render it, it must not become a seventh lamp or a new bracket prefix without a design-system amendment. If it stays inside test output and evidence prose, it is not a design-surface change. Which of the two holds is research's question (per design-system §Surface: cli → Tokens (platform-specific); §Component Patterns (cli) #4).
- `Blocked` stays a distinct "never measured" state that carries its named precondition string. It is never collapsed into a red `Fail` or a generic error. This covers a preflight against a pre-S Pulse that is now `Blocked` on the missing-tool precondition, if `retrieve_incident_events` joins `required_tools`. The named string should identify the precondition. Whether the shipped missing-tool string names the absent tool is research's question (per design-system §Component Patterns (cli) #3; §Anti-Patterns → Rejected Defaults "Conflating no result yet with failed").
- A `Fail` stays motionless and in place, with no alarm treatment. In this round a FAIL is a recorded Pulse finding, not an incident for Conductor's surfaces (per design-system §Anti-Patterns → Rejected Defaults "flashing / pulsing red banner").
- If any operator-facing output from the leg reaches a terminal, it must follow the cli rules: ASCII prefixes only in piped or headless output (no emoji), ANSI only behind the per-stream `IsTerminal` + `NO_COLOR` + `TERM` gate, and the headless path never blocks on a prompt (per design-system §Surface: cli → Platform-Specific Notes; §Anti-Patterns → Per-Surface Bans → cli).

## Patterns to follow
- Verdict / report-state line shape: glyph (TTY only), then P-ID in the mono ID tier, then the state label, then the measured value (for example `latency_ms`). Assertions 3-6 carry a P-ID (P-025 / P-027 / P-037 / P-045) and a measured value, so any per-assertion rendering follows this line shape (per design-system §Component Patterns (cli) #4).
- Results / SLO table: a `Blocked` row carries the named precondition string, and its measurement columns render as `—`/null, never as a red error. The UNGRADED (absent-sample) case should follow the same "no measurement, not a failure" treatment wherever it renders (per design-system §Component Patterns (cli) #3).
- Markdown counterpart: in a committed `.md` evidence or report file, state is carried by emphasis or the bracket label, never by color (per design-system §Surface: cli → Tokens (platform-specific), the ANSI 246 note).

## Anti-patterns to avoid
- Never let an absent or UNGRADED sample, or a pre-S `Blocked` preflight, read as `Fail`. Never let an absent sample read as `Pass` (per design-system §Anti-Patterns → Rejected Defaults "Conflating no result yet with failed").
- Never use emoji in machine-parseable or piped output. Evidence that Pulse cites from Conductor's files is parsed and quoted by another repo (per design-system §Anti-Patterns → Per-Surface Bans → cli).

## Contract bindings
- design ↔ a11y: the not-color-alone label/prefix binds to a11y §Use of Color (SC 1.4.1). The scope expects no rendered surface change, so no contrast or motion binding is touched.
- design ↔ security / arch: the five-precondition `Blocked` strings are owned by security-plan / arch §Standard Contracts (the preflight gate). Design owns only that they render as the distinct `Blocked` treatment with their string. The string's wording stays with its owners.

## Acceptance criteria contributions
- (design) Any per-assertion verdict the round prints or commits carries a text label from the closed set (`[PASS]` / `[FAIL]` / `[BLOCKED]` …) or its word form. No verdict is conveyed by color alone (per design-system §Surface: cli → Tokens (platform-specific)).
- (design) An absent-sample (UNGRADED) assertion is never rendered as `[PASS]` or as `[FAIL]`. If it reaches a shipped surface, the closed per-P-ID prefix set is not silently widened (per design-system §Anti-Patterns → Rejected Defaults; §Component Patterns (cli) #4).
- (design) If `retrieve_incident_events` is pinned in `required_tools`, a preflight against a Pulse lacking it renders as the distinct `[BLOCKED]` state with its named missing-tool precondition string, never as a red `Fail` or the generic corpus-empty text (per design-system §Component Patterns (cli) #3).
