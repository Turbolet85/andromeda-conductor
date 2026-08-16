# design extract

## Relevance
**partial** — this is backend OTLP span-emission work (`conductor-run` / `conductor-emit` / wire test); design binds only at the one seam where it surfaces operator-visible output: the preflight gate's CLI rendering of the re-run live leg (`ready:true` / not-ready, and any new diagnostic the fix adds). No webview surface, no tokens/typography/motion/iconography work.

## Constraints
- Any status Conductor prints for the leg must come from the **closed CLI prefix set** `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` (+ the run-level non-lamp `[ENVIRONMENT-SUSPECT]` caption) — no seventh lamp, no new state word invented for "canary spans enumerated to zero" (per design-system.md §Surface: cli / Tokens + §Component Patterns #4).
- The scope's **honest exit** (measurement recorded, `v2-10` un-claimed) must render as the distinct un-measured state, not as failure: §Rejected Defaults bans "conflating 'no result yet' with 'failed'"; a `Blocked` row carries the **named precondition string** and renders measurement columns as `—`/null, never a red error (per §Surface: cli / Component Patterns #3).
- Fingerprints, `run_id`, span/trace ids, SLO timings and `latency_ms` are the **reserved mono status tier** — ANSI 117 in CLI (`var(--color-id-cyan)` / `#7DCFFF` webview side), never a generic accent and never mono-everywhere (per §Color Palette / Primary + §Typography Data row + §Rejected Defaults "monospace everywhere").
- Color is never the signal: every status carries its ASCII prefix, and output must honor `NO_COLOR` / `TERM=dumb` / piped-stdout ANSI stripping via `anstream` (per §Surface: cli / Platform-Specific Notes). Whether the preflight path already routes through the `anstream`-gated renderer is research's question.
- Diagnostic output added while discriminating the two leads (span bytes, id dumps, appender-shape traces) is **`--debug`/`-v` only** — normal mode prints `error: <short>` + detail + `hint: <fix>` on stderr, sanitized, no stack traces, no absolute host paths (per §Surface: cli / Component Patterns #5 + Per-Surface Bans (cli)).
- The re-run leg goes through the headless agent path: §Surface: cli / Component Patterns #2 + Platform-Specific Notes require that the headless source-of-truth path is **never gated on an interactive `inquire` prompt** and that any prompt site is `isatty`-checked, the decision recorded to the artifact instead.
- If any new label/tint is needed for the diagnosis, it must **reuse an existing token** — the recessive tier is ANSI 246 / `var(--status-residual)` (already the `hint:` label, out-of-scope Mode cell, and `[ENVIRONMENT-SUSPECT]` caption); §Color Palette registers no room for an ad-hoc "diagnostic grey" (per §Surface: cli / Tokens, Residual-mute entry).

## Patterns to follow
- **`error:` / `hint:` stderr pair** — Fail red (ANSI 203) for `error:`, Residual mute (ANSI 246) for `hint:`, on a stderr-specific `IsTerminal` gate distinct from the stdout gate; ASCII labels always present (§Surface: cli / Component Patterns #5).
- **stdout = data, stderr = human messages** — raw artifact/measurement data stays pipe-parseable on stdout so the leg's recorded measurement survives agent capture (§Per-Surface Bans (cli), first entry).
- **Results / SLO table shape** (`comfy-table`, width detected dynamically, P-ID cyan, right-aligned `latency_ms`, fingerprints column) is the existing home for per-check leg results — it is a *separate* table from the 4-column `conductor coverage` matrix, which carries no state column (§Surface: cli / Component Patterns #3).
- **`indicatif` spinner discipline** if the re-run leg shows live progress: spinner appears only after ~200ms, TTY-gated so agent-captured artifacts stay clean, and **stops in place** rather than hiding or animating to 100% at any hold (§Surface: cli / Component Patterns #1).
- **`✓`/`✗`/`?`/`~`/`•`/`→` glyphs are TTY-only** — ASCII prefixes only in machine-parseable piped output, never emoji (§Surface: cli / Tokens).

## Anti-patterns to avoid
- Do NOT introduce a new color, ANSI code, or palette row for the canary-gap diagnostic — reuse the shipped pair (203 / 246) per the standing "spec-illustration → sound-impl reconciliation, never a duplicate color" precedent (§Color Palette; amendments 2026-06-24, 2026-08-09 ×2).
- Do NOT render an un-reached `ready:true` as a red `Fail` or a bare grey blank — §Rejected Defaults bans collapsing empty-state into error-state; `Blocked` is its own state with the named precondition.
- Do NOT print stack traces / raw struct-name dumps in normal mode, and do NOT emit a native OS toast or blocking prompt from the leg (§Per-Surface Bans (cli); §Platform-Specific Notes "Conductor MUST NOT emit native OS toasts").

## Contract bindings
- **a11y — Use of Color (SC 1.4.1):** the ASCII prefix requirement above is the design half of the not-color-alone rule; a11y owns the criterion, design owns the closed prefix set (§Surface: cli / Tokens).
- **Verification/tests harness:** the `[BLOCKED]` + named-precondition rendering is what a `v2-10`-unclaimed leg must produce; whatever the P5 matrix / report asserts on must match the closed state vocabulary, not a free-text status.
- **Architecture:** the scope's expected `.andromeda/architecture.md §Occupied Resources` amendment is arch's, not design's — design only binds how that outcome is *displayed*.

## Acceptance criteria contributions
- (design) Any status the re-run leg prints uses the closed prefix set `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` with color always paired to the ASCII label — no new state word, no color-alone signal (per design-system.md §Surface: cli / Component Patterns #4).
- (design) A leg that does not reach `ready:true` for an unmeasured precondition renders as `Blocked` with the named precondition string and `—` measurement columns, never a red `Fail` and never a blank/grey placeholder (per design-system.md §Rejected Defaults + §Surface: cli / Component Patterns #3).
- (design) Fingerprints / ids / `run_id` / SLO timings / `latency_ms` in any printed output use the reserved mono status tier (ANSI 117 / `--color-id-cyan`); no other output is tinted cyan (per design-system.md §Color Palette Primary + §Typography Data row).
- (design) No new color, ANSI code or palette entry is added for diagnostics; new operator-visible text reuses `error:` (203) / `hint:` (246) on stderr, sanitized, with stack traces only under `--debug`/`-v` (per design-system.md §Surface: cli / Component Patterns #5).

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§cli / Error output) — recorded that `error:`/`hint:` **reuse** Fail red (203) and Residual mute (246) on a stderr-specific `IsTerminal` gate; a proposed new "Hint grey" palette row was **corrected away** at validation. Directly binds any diagnostic text this chunk adds.
- **2026-08-09-out-of-scope-classification-treatment** (§cli Tokens + Component Patterns #3) — Residual mute logged its non-lamp reuses and Pattern #3 was retitled **Results / SLO table**, stating explicitly that `conductor coverage` is a separate 4-column table with no verdict/state column. Prevents this chunk's leg results from being written into the wrong table.
- **2026-08-09-sut-load-envelope** (§Color Palette) — added the run-level `[ENVIRONMENT-SUSPECT]` caption as a *qualifier on the run*, explicitly **not** a seventh lamp or sixth `ReportState`, and named the recessive tier's uses as a set rather than a count. This is the standing precedent for how a run-level environmental caveat (relevant if the live leg's Pulse environment is implicated) is expressed without a new state.
- Webview-side amendments (`@theme`→`:root`, `--motion-heartbeat`, dialog fade 200ms→`--motion-micro`) are **not** relevant — no webview surface in this chunk.
