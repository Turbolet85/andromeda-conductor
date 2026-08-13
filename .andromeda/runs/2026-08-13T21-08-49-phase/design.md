# design extract

## Relevance
Partial — CLI surface only (the `conductor preflight` / `agent-run boot` output edge + the Markdown evidence file); no desktop-webview work, no new tokens expected.

## Constraints
- `Blocked` is a distinct state that must never render as a red `Fail` or as an empty/gray "no result": the live preflight's expected outcome (`ready:false` + named precondition) renders in the blocked slate-violet tier (ANSI 60) carrying its precondition string, with measurement columns as `—`/null, never a red error (per design-system.md §Color Palette → Verdict-vs-ReportState note; §Anti-Patterns → Rejected Defaults "Conflating no-result-yet with failed").
- Status is never color-alone: every per-check line carries an ASCII prefix from the **closed** set `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` (+ the run-level non-lamp `[ENVIRONMENT-SUSPECT]` caption). This chunk adds no seventh lamp state (per design-system.md §Surface: cli → Tokens; §Component Patterns 4).
- Reuse the shipped ANSI map only — no new color entries: mono ID-cyan 117 for P-IDs / `run_id` / SLO timings / fingerprints, green 114 Pass, violet 60 Blocked, muted 246 for the recessive non-lamp tier. Any live-response diagnostic text uses an existing tier (per design-system.md §Surface: cli → Tokens).
- stdout carries raw parseable data (`--json` `ReadyState`), stderr carries human messages as `error: <short>` + `hint: <fix>`, sanitized (no absolute host paths, no internal struct names, no stack traces outside `--debug`) (per design-system.md §Surface: cli → Component Patterns 5; §Per-Surface Bans → cli).
- Color is TTY-gated end to end: honor `NO_COLOR`, `TERM=dumb`, and piped-stdout ANSI stripping via `anstream`; the stderr edge has its own `IsTerminal` gate distinct from stdout's — the agent-captured artifacts of this live leg must stay ANSI-clean (per design-system.md §Surface: cli → Platform-Specific Notes).
- The headless source-of-truth path is never gated on an interactive prompt — `inquire` prompts stay `isatty`-checked; a live leg run under `agent-run boot` must not acquire a blocking prompt (per design-system.md §Surface: cli → Component Patterns 2; §Per-Surface Bans → cli).
- Markdown has no color channel: the appended `two-launch-verdict.md` arm rows express state through the bracket label / emphasis counterpart, never an implied tint (per design-system.md §Surface: cli → Tokens, Residual-mute entry).

## Patterns to follow
- The existing preflight render shape in `D:\dev\projects\conductor\crates\conductor-cli\src\commands\preflight.rs` — `render::paint(Lamp::X.status_prefix(), render::lamp_code(Lamp::X))` — is the sanctioned by-name binding of prefix + ANSI code; extend through `Lamp`/`render`, never a literal escape or a bare string (per design-system.md §Surface: cli → Component Patterns 4).
- Results/SLO table pattern for any per-check dump: `comfy-table`, terminal width detected dynamically, P-ID column in ANSI 117, `latency_ms` right-aligned, never hardcoded widths (per design-system.md §Surface: cli → Component Patterns 3).
- If the long canary poll (`min_canary_poll_seconds` floor) gets any progress display: the `indicatif` spinner appears only after ~200ms, and **stops in place** rather than hiding or animating to 100% (per design-system.md §Surface: cli → Component Patterns 1).
- Recessive non-lamp text (a caption, a key-diff note, a "not measured" qualifier) uses the Residual-mute tier (ANSI 246 / `var(--status-residual)`) with an always-rendered text label carrying the signal — the tint only de-emphasizes; it is not a lamp state (per design-system.md §Surface: cli → Tokens, Residual-mute entry).

## Anti-patterns to avoid
- No emoji in machine-parseable/piped output (the `--json` and agent-captured legs); ASCII prefixes only (per design-system.md §Per-Surface Bans → cli).
- No stack traces in normal mode and no arbitrary text wrapping / hardcoded widths in the recorded evidence output (per design-system.md §Per-Surface Bans → cli).
- No new palette/ANSI entry or a sixth `ReportState` invented to describe a live divergence — the key divergence is recorded as an existing `Blocked` precondition string, not a new visual state (per design-system.md §Surface: cli → Tokens; §Color Palette Verdict-vs-ReportState note).

## Contract bindings
- **a11y §Use of Color (SC 1.4.1)** — the ASCII bracket-prefix pairing is the cli surface's not-color-alone mechanism; also serves `NO_COLOR` and screen readers.
- **a11y §Contrast** — ANSI 117 (cyan) / 114 (green) / 60 (violet) were chosen for dark-terminal legibility; avoid dark-blue-on-black / dark-red-on-black in any new line.
- **tests harness** — the closed prefix set is a load-bearing E2E selector (`.claude\rules\testing.md` brand anchors, `crates\conductor-cli\tests\cli_smoke.rs`); changing a prefix string breaks assertions in the same commit.
- **obs/security redaction** — the design rule "never leak absolute host paths / internal struct names" is the render-side half of the artifact-hygiene boundary this chunk's scope also states (`data_dir` redacted, cwd named by role).

## Acceptance criteria contributions
- The live `ready:false` result renders as the `[BLOCKED]` prefix + its named precondition (slate-violet tier), never a red `Fail` and never a blank/gray placeholder (per design-system.md §Color Palette → Verdict vs ReportState).
- Every status line emitted by the live leg pairs color with its ASCII prefix from the closed set; zero new lamp states or ANSI entries are introduced (per design-system.md §Surface: cli → Tokens).
- Captured/piped artifacts from the live run and the `--json` leg contain no ANSI escapes and no emoji, and stderr text stays sanitized `error:`/`hint:` (per design-system.md §Surface: cli → Component Patterns 5 + Platform-Specific Notes).
- Rows appended to `two-launch-verdict.md` carry their state as a text label (bracket/emphasis), with cwd named by role and no absolute path (per design-system.md §Surface: cli → Tokens, Residual-mute Markdown counterpart).

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§cli Component Patterns 5) — the `error:` / `hint:` stderr edge REUSES shipped tokens (Fail red ANSI 203 / Residual mute ANSI 246) on a stderr-specific `IsTerminal` gate; the detector's proposal to add a new "Hint grey" palette row was corrected at validation. Precedent: a new message edge reuses existing tokens rather than adding a color — directly governs any new diagnostic this chunk emits.
- **2026-08-09-out-of-scope-classification-treatment** (§cli Tokens + Component Patterns 3) — recorded ANSI 246's non-lamp reuses, named `var(--status-residual)` as the by-name webview half, established that Markdown (no color channel) uses emphasis or a bracket label in a blockquote as the surface-adapted counterpart; also disambiguated the Results/SLO table (6 cols, has a state column) from `conductor coverage` (4 cols, no state column, no bracket prefix).
- **2026-08-09-sut-load-envelope** (§Color Palette, Residual-mute entry) — added the run-level `[ENVIRONMENT-SUSPECT]` caption as a third non-lamp reuse of the recessive tier, explicitly "a qualifier on the run, never a seventh lamp or a sixth `ReportState`". The set is named rather than counted, so a fourth recessive-tier caption (e.g. a live-environment qualifier from this chunk) extends the set without a new token — but must not become a lamp state.
