# design extract

## Relevance
Partial — no rendering surface in this chunk; design applies only where the gate (or the amended `cleanup` verb) emits operator-facing cli output, and where the gate's assertions touch the closed Verdict/ReportState vocabulary.

## Constraints
- The journal rows the gate judges carry BOTH a machine `verdict` and a `state`: design-system §Color Palette (Verdict (3) vs ReportState (5) note) requires the closed verdict triad (`Pass`/`Fail`/`CalibrationRegion`) and the 5-valued `ReportState` (`Pass`/`Fail`/`ManualCheck`/`KnownResidual`/`Blocked`), and forbids `ManualCheck`/`KnownResidual`/`Blocked` collapsing into `Fail`. A gate that normalizes or flattens either axis violates that rule (per design-system §Color Palette).
- If the journal producer is the hermetic Blocked run the scope proposes, design-system §Surface: cli / Component Patterns 3 requires a `Blocked` row to carry the named precondition string with measurement columns rendered `—`/null and "never a red error" — so schema-completeness must treat unmeasured measurement fields on a Blocked row as conformant, not as a missing field. Whether the shipped writer emits those keys present-but-null or omits them entirely is research's question (per design-system §Surface: cli / Component Patterns 3).
- `scripts/agent-run.{sh,ps1}` is inside the design domain's cli surface (design-system §Surface: cli names the headless `conductor-cli` / `scripts/agent-run.sh` as the platform), so the CARRY's cleanup edits inherit the cli output rules — they are not "just scripts" (per design-system §Surface: cli).
- Any operator-facing violation message must follow design-system §Surface: cli / Component Patterns 5 (Error output): stderr, `error: <short>` + contextual detail + `hint: <fix>`, sanitized of absolute host paths / internal struct names / stack traces, reusing Fail red (ANSI 203) for `error:` and Residual mute (ANSI 246) for `hint:` behind a stderr-specific `IsTerminal` gate — no new color may be minted (per design-system §Surface: cli / Tokens + Component Patterns 5).
- The status-prefix vocabulary is closed: the six per-P-ID lamp prefixes (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`) plus the run-level non-lamp caption set (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`). A gate result rendered as a bracketed status must reuse this set or be raised as an amendment, never silently added as a seventh lamp (per design-system §Surface: cli / Tokens).
- Per-stream discipline holds for anything the gate prints: raw artifact data on stdout, human messages on stderr; ANSI only after `NO_COLOR` / `TERM` / pipe checks; ASCII prefixes (never emoji) in machine-parseable piped output (per design-system §Anti-Patterns / Per-Surface Bans: cli).

## Patterns to follow
- The `error:` / `hint:` stderr shape with its per-stream `IsTerminal` gate is the shipped precedent for a new failure edge — reuse it verbatim rather than inventing a gate-specific report format (per design-system §Surface: cli / Component Patterns 5).
- The Results / SLO table's Blocked-row convention (`—`/null measurement cells + named precondition string, never red) is the reference for what a legitimate hermetic Blocked journal contains (per design-system §Surface: cli / Component Patterns 3).
- Reuse-don't-mint for the recessive tier: the Residual mute (ANSI 246 ↔ `var(--status-residual)`) already carries non-lamp uses as a named SET; a new run-level caption, if any, joins that set without a new palette row (per design-system §Surface: cli / Tokens).
- Color is never the signal: every status the gate prints pairs its tint with an always-rendered ASCII label (per design-system §Surface: cli / Component Patterns 4).

## Anti-patterns to avoid
- Never conflate "never measured" with "failed" — a gate that reds a `Blocked` row for absent measurements enacts the explicitly Rejected Default (per design-system §Anti-Patterns / Rejected Defaults).
- Never colorize without the `NO_COLOR` / `TERM` / pipe check, never rely on color alone, never emit emoji in piped output, never print stack traces outside `--debug`/`-v` (per design-system §Anti-Patterns / Per-Surface Bans: cli).
- Never add a new color, ANSI map entry, or lamp state for the gate's own output (per design-system §Surface: cli / Tokens).

## Contract bindings
- design ↔ security/redaction: design-system §Surface: cli / Component Patterns 5 already mandates "no absolute host paths" in operator-facing error output — the same notion the chunk asserts over journal rows via `conductor-core::redact::is_host_path_token`. Both halves must name one source; the gate's own failure output is itself subject to the rule it enforces.
- design ↔ a11y: the ASCII bracket-prefix requirement is design's half of the not-color-alone rule (a11y SC 1.4.1); the label, not the tint, carries the signal in `NO_COLOR` / piped / screen-reader contexts.
- design ↔ arch: the six lamp treatments and the Verdict/ReportState pairing consume the envelope's `verdict` + `state` fields (arch §Standard Contracts). The scope's "not a schema change" boundary protects this binding — any shape change to those two fields would ripple into the lamp set.

## Acceptance criteria contributions
- (design) The gate asserts `verdict` and `state` against the closed sets only — verdict triad and 5-valued `ReportState`, neither collapsed into the other (per design-system §Color Palette, Verdict vs ReportState note).
- (design) A `Blocked` / never-measured row does not fail the gate for absent or null measurement values; Blocked stays a distinct outcome, never downgraded to `Fail` (per design-system §Anti-Patterns / Rejected Defaults + §Surface: cli / Component Patterns 3).
- (design) Operator-facing violation output goes to stderr as `error: <short>` + `hint: <fix>`, sanitized of absolute host paths / internal struct names / stack traces, introducing no new color (reusing ANSI 203 / 246) (per design-system §Surface: cli / Component Patterns 5).
- (design) Any status text the gate or the amended `cleanup` verb emits uses the closed ASCII prefix set, never color alone, and no emoji in piped output (per design-system §Surface: cli / Tokens + §Anti-Patterns / Per-Surface Bans: cli).

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§cli Error output) — the last new stderr error edge: `error:` reuses Fail red 203, `hint:` reuses Residual mute 246, on a stderr-specific `IsTerminal` gate. The detector's proposed new "hint grey" palette row was REJECTED at validation as a duplicate of a shipped token. Directly precedential: a conformance gate's failure output is another new error edge and must reuse, not extend, the palette.
- **2026-09-03-live-pulse-preconditions-probed** (§cli Toolkit / Tokens / Platform notes) — `anstream` retired as the named cli gate; the shipped mechanism is `owo-colors` + `std::io::IsTerminal` decided independently per stream. Also added `[PRECONDITION]` to the run-level non-lamp caption SET while keeping the lamp set closed at six. Sets the shape for any new run-level caption this chunk might want.
- **2026-08-09-out-of-scope-classification-treatment** and **2026-08-09-sut-load-envelope** (§Color Palette / §cli Tokens, Residual-mute entry) — the recessive tier's non-lamp reuses are recorded as a named SET rather than a count, precisely so a fourth reuse costs no palette row and no re-staling literal.
- **2026-08-09-out-of-scope-classification-treatment** additionally disambiguated the Results/SLO table (6 columns, with state prefix) from `conductor coverage` (4 columns, no verdict column) — relevant when deciding which existing output convention a gate report resembles.
