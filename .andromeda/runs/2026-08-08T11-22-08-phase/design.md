# design extract

## Relevance
Partial — the chunk builds no rendered surface (core validation + a `.andromeda/refs/` data artifact, which design has no styling authority over), but its user-visible outcome (the malformed/absent-manifest "named precondition", and P-ID-bearing rejection messages) must use the already-fixed status vocabulary and CLI output shapes.

## Constraints
1. If open question 1 resolves toward "reporting blocked" (as the route line says), the outcome MUST render as the existing `ReportState::Blocked` treatment — slate-violet `--count-blocked` (`#565F89` / ANSI 60), `[BLOCKED]` ASCII prefix + `•` glyph, hollow-ring lamp — carrying the **named precondition string**, per `design-system.md` §Color Palette (Semantic Colors / the Verdict-vs-ReportState note), §Iconography, §Surface: cli §Component Patterns 4. `Blocked` means *never measured*; it must never collapse into `Fail`.
2. If it instead resolves as a harness fault, the CLI shape is already fixed: stderr, sanitized (no absolute host paths, no internal struct names, no stack traces outside `--debug`/`-v`), `error: <short>` + contextual detail + `hint: <fix>`, `error:` reusing Fail red (ANSI 203) and `hint:` Residual mute (ANSI 246) — per §Surface: cli §Component Patterns 5 (Error output). Those are the only two sanctioned treatments; there is no third.
3. Zero new tokens. The six report-state colors + the ANSI map are a closed set; a new failure edge reuses shipped token names, never a new palette row — per §Surface: desktop-webview §Tokens (the 34-token binding contract) and §Anti-Patterns Universal Bans ("never use color purely for decoration").
4. Never color-alone: any status line about the manifest pairs color with its ASCII text label, and stays meaningful under `NO_COLOR` / `TERM=dumb` / piped stdout — per §Surface: cli §Tokens and §Anti-Patterns Per-Surface Bans (cli).
5. Every `P-NNN`, manifest version stamp, and fingerprint that appears in a message or table is the reserved mono status tier: JetBrains Mono / `--color-id-cyan` `#7DCFFF` (ANSI 117); surrounding prose stays IBM Plex Sans — per §Typography (Data row) and §Color Palette (Core Colors / Primary). Mono is a tier, not a default.
6. A `Blocked` row's measurement columns render `—`/null, never a red error value — per §Surface: cli §Component Patterns 3 (Coverage-matrix / SLO table).
7. Nothing may encode "60 P-IDs" as a rendering assumption: the matrix is a dense single-row-per-P-ID list sized from the set, with dynamically detected terminal width (CLI) and virtual scroll (webview) — per §Component Patterns 3 (both surfaces) and §Surface: cli §Platform-Specific Notes.

## Patterns to follow
- **CLI report-state line** (§Surface: cli §Component Patterns 4) — the `• P-0NN  Blocked  <named precondition>` shape is the existing precedent for exactly this "precondition not met, never measured" case; the manifest precondition string slots into it verbatim in form.
- **CLI error output** (§Surface: cli §Component Patterns 5) — the sanitized `error:` / `hint:` pair, each with its own stdout/stderr `IsTerminal` gate, for the harness-fault branch.
- **Coverage-matrix / SLO table** (§Surface: cli §Component Patterns 3) — the `Blocked` row already carries a named precondition string and dashes its measurement columns; follow it rather than inventing a manifest-specific row.
- **Verdict / report-state lamp** (§Surface: desktop-webview §Component Patterns 4) — hollow ring for `Blocked`, resolves motionless (150ms color transition, no flashing), always paired with text, for when this state later reaches the webview matrix/run-report.
- **Token reuse over token addition** (amendment 2026-06-24-sanitized-stderr-agent-mode-logging) — the established routine when a new failure surface appears: map it onto an existing token, add a use-site note, do not add a palette entry.

## Anti-patterns to avoid
- NEVER render the malformed/absent manifest as red `Fail` or as a blank/gray "no result" — "conflating 'no result yet' with 'failed', or showing `Blocked` as a red error" is a named Rejected Default (§Anti-Patterns / Rejected Defaults), and the same discipline is why `ManualCheck`/`KnownResidual` are separate treatments.
- NEVER introduce a new color, glyph, or prefix for this failure edge (e.g. a "manifest error" hue) — reuse `--count-blocked`/ANSI 60 or the `error:`/`hint:` pair (§Anti-Patterns Universal Bans; §Surface: cli §Tokens).
- NEVER use emoji in machine-parseable/piped output, and never colorize without the `NO_COLOR`/`TERM`/pipe check — ASCII prefixes only there (§Anti-Patterns / Per-Surface Bans / cli).

## Contract bindings
- **a11y §Use of Color (SC 1.4.1)** — the `Blocked` slate-violet and the `error:` red must each be paired with their ASCII label/text; design supplies the token, a11y owns the criterion.
- **a11y §Contrast** — `--count-blocked` / ANSI 60 is the dimmest token in the set; §Surface: cli §Platform-Specific Notes ("avoid dark-blue-on-black") means the ASCII prefix, not the hue, carries the meaning in the terminal.
- **arch — the verdict/error wall (scope open question 1)** — design does not decide the side; it constrains the treatment of each side (constraints 1 and 2) so whichever the arch extract fixes, the vocabulary is already determined. Flag if arch picks a third path.
- **a11y §Announcements** — if this state ever reaches the webview matrix/run-report, the lamp carries the paired text + `aria-live` announcement (§Component Patterns 4); out of this chunk's build, bound for `v2-03`.

## Acceptance criteria contributions
1. (design) A malformed / absent / unparseable manifest surfaces as `Blocked` using `--count-blocked` / ANSI 60 with the `[BLOCKED]` + `•` prefix and the named precondition string — never `--status-fail` / ANSI 203, never a blank or gray "no result" (design-system §Color Palette, §Surface: cli §Component Patterns 3–4).
2. (design) This chunk introduces zero new color/typography/motion tokens; every status color used traces by name to the shipped `:root` block or the CLI ANSI map (design-system §Tokens — Token Test).
3. (design) Every manifest-related status or error line pairs its color with an ASCII text label and remains fully meaningful under `NO_COLOR=1` and when stdout is piped (design-system §Surface: cli §Tokens, §Per-Surface Bans).
4. (design) On the harness-fault branch, stderr matches `error: <short>` + detail + `hint: <fix>`, sanitized — no absolute host paths, no internal struct names, no stack traces outside `--debug`/`-v` (design-system §Surface: cli §Component Patterns 5).

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Surface: cli / Error output) — directly on point. A new CLI error surface fired the design-token detector, which *proposed a new "Hint grey" palette row*; that was **corrected at validation** because ANSI 246 was the existing Residual-mute token. Outcome: no palette entry added, the edge reuses two shipped tokens (203/246), documented as a use-site note. This chunk adds another new failure edge — take the same route: reuse `--count-blocked`/ANSI 60 or `error:`/`hint:`, do not add a token.
- **2026-06-15-design-token-typography-bundle** and **2026-06-24-paused-count-hold-point-signature** — relevant only as the established "spec-illustration → sound-impl reconciliation" routine (amend the plan at wrap when implementation proves the invariant, rather than deviating silently). Applies if this chunk's message wording or precondition string needs a use-site note on §cli Error output.
- No prior amendment touches the P-ID set size, the `Blocked` semantics, or the coverage-matrix row shape — those sections are original plan truth.
