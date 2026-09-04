# design extract

## Relevance
Partial — cli surface only (`conductor preconditions` output + the `boot` arm); no desktop-webview surface, so no color/typography/spacing/motion/radius/iconography tokens are in play.

## Constraints
- The `conductor preconditions` verb's caption is already registered as a **run-level non-lamp** element: `[PRECONDITION]` in the Residual-mute tier (ANSI 246), outside the lamp column, neither a lamp state nor a sixth `ReportState` — per design-system.md §Surface: cli / Tokens (status-prefix line + Residual-mute row). Per-handle grading must not promote a handle outcome into the closed six-member lamp set or add a palette row. Whether the shipped probe already renders that caption in that tier is research's question.
- Every status/caption must be ASCII-labelled, never color-alone, and colorization must pass the per-stream `std::io::IsTerminal` gate (terminal AND `NO_COLOR` unset AND `TERM != dumb`), decided independently for stdout and stderr — per design-system.md §Surface: cli / Tokens (ANSI-map preamble) and §Anti-Patterns / Per-Surface Bans (cli).
- An unmet precondition is a **named-precondition / never-measured** state, not an error: the row carries the named precondition string and measurement columns render `—`/null, never a red error — per design-system.md §Surface: cli / Component Patterns 3 (Results / SLO table) and §Rejected Defaults ("Conflating 'no result yet' with 'failed' … showing Blocked as a red error").
- Any stderr diagnostic the fix emits or reshapes uses `error: <short>` + detail + `hint: <fix>`, sanitized — **no absolute host paths**, no internal struct names, no stack traces outside `--debug`/`-v` — per design-system.md §Surface: cli / Component Patterns 5 (Error output). This binds directly here because the path handle `ANDROMEDA_PULSE_DATA_DIR` carries a filesystem path that an "unmet" message could leak.
- The `boot` path is the headless agent-driven source of truth: it must never be gated on an interactive `inquire` prompt, and any prompt path requires an `isatty` check first — per design-system.md §Surface: cli / Component Patterns 2 (Operator-pause prompt) and §Per-Surface Bans (cli).
- No emoji in machine-parseable/piped output; ASCII prefixes only there (bears on the probe's human vs `--json` split, which the scope expects unchanged) — per design-system.md §Anti-Patterns / Per-Surface Bans (cli).

## Patterns to follow
- **Run-level non-lamp caption** (`[ENVIRONMENT-SUSPECT]` / `[PRECONDITION]`, both ANSI 246, both outside the lamp column) — the established shape for a qualifier on the run rather than a per-P-ID verdict; design-system.md §Surface: cli / Tokens.
- **`owo-colors` + `std::io::IsTerminal`, per stream** as the sole styling gate (the shipped mechanism; `anstream` is retired) — design-system.md §Surface: cli / Toolkit + Platform-Specific Notes.
- **Recessive-tier reuse by name**: ANSI 246 ↔ `var(--status-residual)` is one by-name pair covering `hint:`, the out-of-scope Mode cell, `[ENVIRONMENT-SUSPECT]` and `[PRECONDITION]` — reuse the existing pair, never mint a new color for a new grading outcome; design-system.md §Surface: cli / Tokens.
- **Blocked row convention**: named precondition string carried in place of measurements — design-system.md §Surface: cli / Component Patterns 3 + 4.

## Anti-patterns to avoid
- Rendering the unmet `handles-declared` subject (or a newly-satisfiable one) as Fail red / a flashing or alarm-shaped edge — design-system.md §Rejected Defaults and §Color Palette (Fail red held muted, "calm under load, no alarm").
- Introducing a new ANSI entry, palette row, or a seventh lamp state to express "presence-satisfied" vs "truthy-satisfied" grading — design-system.md §Surface: cli / Tokens (lamp set closed at six) + §Anti-Patterns (color only ever communicates Verdict/ReportState).
- Blocking the headless `agent-run.{sh,ps1}` boot path on a prompt, or colorizing without the pipe/`NO_COLOR`/`TERM` check — design-system.md §Per-Surface Bans (cli).

## Contract bindings
- **cli ↔ desktop-webview token pairing**: ANSI 246 and `var(--status-residual)` are bound by name (design-system.md §Surface: cli / Tokens); if this chunk changes what the probe prints in that tier, the webview half stays the same named token — no new value on either side.
- **cli ↔ a11y (Use of Color, SC 1.4.1)**: the `[PRECONDITION]` / `[BLOCKED]` ASCII labels carry the signal; tint only de-emphasizes — design-system.md §Surface: cli / Tokens.
- **cli ↔ security/tests harness**: the "never block the headless source-of-truth path on an interactive prompt" rule is shared with the security plan and is what the `boot`-reaches-preflight proof exercises — design-system.md §Surface: cli / Platform-Specific Notes.

## Acceptance criteria contributions
- Any preconditions output touched keeps `[PRECONDITION]` (and any `[BLOCKED]` row) as an ASCII label in the existing Residual-mute/246 tier — no new ANSI entry, no new lamp state, signal never color-alone (per design-system.md §Surface: cli / Tokens).
- An unmet handle renders as a named-precondition/never-measured state with the handle name and `—`/null measurements, never as a red `Fail`/error line (per design-system.md §Surface: cli / Component Patterns 3 + §Rejected Defaults).
- Any `error:`/`hint:` diagnostic emitted for the path handle is sanitized — no absolute host path, no struct names, no stack trace outside `--debug`/`-v` — and reuses ANSI 203/246 behind the stderr-specific `IsTerminal` gate (per design-system.md §Surface: cli / Component Patterns 5).
- The `boot` proof runs the probe with no `isatty`-gated prompt on the headless path in either `agent-run.sh` or `agent-run.ps1` (per design-system.md §Surface: cli / Component Patterns 2 + §Per-Surface Bans (cli)).

## Relevant amendment history
- **2026-09-03-live-pulse-preconditions-probed** (§Surface: cli — Toolkit · Tokens · Platform-Specific Notes) — same verb, immediately prior: `anstream`/`anstyle` retired as the named cli gate in favour of `owo-colors` + per-stream `std::io::IsTerminal`; `[PRECONDITION]` added to the run-level non-lamp caption SET and to the Residual-mute (246) reuse list; lamp set kept closed at six, no new palette row. This is the amendment that makes the probe's rendering already specified — this chunk should change grading, not presentation.
- **2026-08-09-sut-load-envelope** (§Color Palette, Residual-mute) — added `[ENVIRONMENT-SUSPECT]` as the third non-lamp reuse and established the **set-framing rule**: name the set, never a literal count, so the entry cannot re-stale. Applies if this chunk adds any further caption or reuse.
- **2026-08-09-out-of-scope-classification-treatment** (§Surface: cli Tokens + Component Patterns 3) — established that the always-rendered text label carries the signal while the 246 tint only de-emphasizes, and named `var(--status-residual)` as the webview half of the by-name pair.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Surface: cli / Error output) — recorded that `error:`/`hint:` REUSE shipped tokens (203/246) with a stderr-specific `IsTerminal` gate and no new palette entry; the precedent to follow if this chunk reshapes the unmet-handle diagnostic.
