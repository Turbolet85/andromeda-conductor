# design extract

## Relevance
Partial — no webview surface is rendered; design applies only to the CLI/harness-facing failure output (the scope's open P4 question "test-only vs. reachable from the harness") and to the reserved status-tier tokens used when naming drifted ids.

## Constraints
- If the drift failure reaches an operator, it is the **harness error edge**, not a run result: `error: <short>` + contextual detail + `hint: <fix>` on **stderr**, gated by a stderr-specific `IsTerminal` check distinct from the stdout gate (per design-system.md §Surface: cli → Component Patterns 5 "Error output").
- Drifted capability ids are the **reserved mono status tier** — ANSI 117 ID-cyan only; `sut_version` / `captured_at` are metadata → dimmed journal text (ANSI 146). No new color may be minted for "drift" (per design-system.md §Surface: cli → Tokens; §Typography "Data" row).
- **Never color alone.** Every colored token carries an ASCII label/prefix that survives `NO_COLOR` (per design-system.md §Anti-Patterns → Per-Surface Bans / cli; §Surface: cli → Tokens "Status prefixes are ASCII text + color").
- Honor `NO_COLOR`, `TERM=dumb`, and piped-stdout ANSI stripping via `anstream`; no emoji in machine-parseable piped output (per design-system.md §Surface: cli → Platform-Specific Notes).
- The green/amber/red/slate-violet tier is **reserved for `Verdict`/`ReportState`** — a harness-side drift failure must NOT render as a `[FAIL]` verdict line, a `[BLOCKED]` row, or a status lamp; it reuses the shipped error-edge tokens (Fail red 203 for `error:`, Residual mute 246 for `hint:`) (per design-system.md §Color Palette → Verdict-vs-ReportState note; §Anti-Patterns "NEVER use color purely for decoration"). This is the design-side mirror of the scope's verdict/error-wall invariant.
- Sanitized output: no absolute host paths, internal struct names, or stack traces; stack traces only under `--debug`/`-v` (per design-system.md §Surface: cli → Component Patterns 5; §Per-Surface Bans / cli).
- If drifted ids are tabulated rather than listed inline, use the shipped `comfy-table` pattern with **dynamically detected terminal width** — never hardcoded widths or arbitrary wrap points (per design-system.md §Surface: cli → Component Patterns 3; §Spacing "Surface-conditional").

## Patterns to follow
- **Error output (§Surface: cli, Component Pattern 5)** — the existing `error:`/`hint:` two-label shape is the shipped home for this failure; extend it, do not invent a third message register.
- **Coverage-matrix / SLO table (§Surface: cli, Component Pattern 3)** — the P-ID-in-ANSI-117 column convention and the "unmeasured columns render `—`/null, never a red error" discipline are the precedent for presenting an id set that has no measurement.
- **Verdict / report-state lines (§Surface: cli, Component Pattern 4)** — borrow only the *structure* (color always paired with an ASCII prefix), not the `[PASS]`/`[FAIL]`/`[BLOCKED]` vocabulary itself.
- **CLI brand expression (§Typography → Surface-conditional guidance / cli)** — brand here is ANSI color + prefix patterns + `owo-colors` bold/dim header formatting + information density; no webview typography, spacing, radius, or motion tokens apply to this chunk.

## Anti-patterns to avoid
- Never add a new palette entry (e.g. a "drift" color) — reuse shipped tokens; this exact correction is on record (see amendment history below).
- Never blink, pulse, or alarm on the failure — Fail red is held muted, "calm under load, no alarm" (per design-system.md §Anti-Patterns → Rejected Defaults, flashing/pulsing alert; §Surface: cli Component Pattern 4 "no blink").
- Never colorize without the TTY/`NO_COLOR` check, and never block or reformat the headless agent-driven path (per design-system.md §Anti-Patterns → Per-Surface Bans / cli).

## Contract bindings
- **design ↔ arch (verdict/error wall):** design reserves the green/amber/red/violet tier for `Verdict`/`ReportState`; the chunk's invariant that drift is harness-side means it must render in the error edge, not the status tier. Both plans must agree the drift failure never acquires a `ReportState`.
- **design ↔ a11y (SC 1.4.1 not-color-alone):** satisfied by the ASCII `error:`/`hint:` labels and id text being present with color stripped; the `NO_COLOR`/piped path is the a11y fallback, not a degraded mode.
- **design ↔ security (artifact hygiene):** the "no absolute host paths / internal struct names / no stack traces outside `--debug`" rule is jointly owned — design §cli Error output and the security plan state the same constraint; the scope repeats it as an invariant.
- **design ↔ tests:** if P4 lands the check as test-only, no design surface is touched — assertion panic text is not an operator surface and these constraints attach only at the moment the failure is printed by `conductor-cli`/`agent-run.sh`.

## Acceptance criteria contributions
- (design) If the check emits operator-facing output, it goes to **stderr** in the `error:` + detail + `hint:` shape, with the ASCII labels present and legible under `NO_COLOR=1` and when piped (design-system §Surface: cli → Error output).
- (design) Drifted capability ids render in the reserved mono ID-cyan tier (ANSI 117) and manifest metadata dimmed (ANSI 146); **zero new colors** are introduced (design-system §Surface: cli → Tokens).
- (design) The failure does not use the `Verdict`/`ReportState` vocabulary — no `[PASS]`/`[FAIL]`/`[BLOCKED]` prefix, no status lamp, no blink (design-system §Color Palette → Verdict-vs-ReportState; §Anti-Patterns → Rejected Defaults).
- (design) Output contains no absolute host paths, internal struct names, or stack traces outside `--debug`/`-v` (design-system §Surface: cli → Error output / Per-Surface Bans).

## Relevant amendment history
- **2026-08-08-sut-capability-manifest** (§Color Palette Primary · §Typography Data row · §cli ANSI map · §Brand Identity) — the immediately preceding chunk in this area de-hardcoded the `P-001..P-060` range from all mono status-tier / ANSI-117 usage examples, so the plan now says "P-IDs" generically. Why: the illustrative examples carried the superseded range; no token, hex, or type-role changed. Directly relevant: this chunk's whole premise is that the id universe is manifest-data, and the design plan no longer implies a fixed 60-id range anywhere.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Surface: cli → Error output) — documented that `error:` reuses Fail red (ANSI 203) and `hint:` the Residual mute (ANSI 246), on a stderr-specific `IsTerminal` gate, ASCII labels always present. Why it matters here: the detector at that wrap proposed adding a new "Hint grey" palette row and was **corrected at validation** — the error edge reuses shipped tokens rather than adding color. Any drift-failure styling in this chunk must follow that precedent (reuse 203/246; add nothing).
- Not relevant (webview-only): 2026-06-15 `@theme`→`:root`, 2026-06-24 `--motion-heartbeat`, 2026-06-26 dialog fade 200ms→`--motion-micro`.
