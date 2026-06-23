# design extract

## Relevance — relevant

This chunk (line-oriented output rendering, CLI surface) directly activates the design system's §Surface: cli section and critical parts of Brand Identity, Color Palette, and Typography that govern the terminal presentation layer.

## Constraints

1. **Status prefixes are ASCII-paired, never color-alone** (design-system §Color Palette / Anti-Patterns): every status line MUST render the text prefix (`[PASS]` / `[FAIL]` / `[HOLD]` / `[BLOCKED]` / `[MANUAL]` / `[RESIDUAL]`) with color as an optional overlay; stripped color output remains unambiguous for piped/agent contexts.

2. **Mono ID-cyan ANSI 117 reserved for status tier** (design-system §Surface: cli / Tokens): P-IDs, run_id, SLO timings, latency_ms, fingerprints render in `#7DCFFF` → ANSI 117; this tier is strictly functional, never used for decoration.

3. **Verdict/ReportState lamp precedence** (design-system §Color Palette, per scope.md): a row carries BOTH Verdict (machine triad: Pass/CalibrationRegion/Fail) and ReportState (operator states: Blocked/ManualCheck/KnownResidual); Verdict renders first in lamp precedence — a CalibrationRegion row shows `[HOLD]`, never `[MANUAL]` (consistent with existing Markdown renderer in conductor-report).

4. **Paused-count hold-point CLI mirror** (design-system §Brand Identity / §Motion): during operator-pause, the `indicatif` spinner STOPS in place (not hides, not animates-to-100%), displaying the frozen count value; this embodies the signature "paused count shows where it stopped."

5. **TTY gating + NO_COLOR / TERM=dumb honor** (design-system §Surface: cli / Platform-Specific Notes): output auto-strips ANSI when piped (`!isatty(1)`), respects `NO_COLOR` + `TERM=dumb`; interactive `inquire` prompts ALWAYS check `isatty` first — the headless agent path is never blocked.

6. **Terminal width dynamic detection** (design-system §Surface: cli): `comfy-table` SLO/coverage table never hardcodes widths; width detected from terminal at runtime.

7. **Color contrast on dark terminals** (design-system §Surface: cli / Platform-Specific Notes): avoid dark-blue-on-black and dark-red-on-black; ANSI 117 (mono ID-cyan) and ANSI 114 (nominal green) chosen for legibility on dark terminal defaults.

## Patterns to follow

1. **ANSI 256 color mapping** (design-system §Surface: cli / Tokens): map the functional status tier — nominal-green `#7EE787` → ANSI 114, hold-amber `#E3B341` → ANSI 179, fail-red `#F85149` → ANSI 203, blocked-violet `#565F89` → ANSI 60, manual-lavender `#A9B1D6` → ANSI 146, residual-mute `#9A93A8` → ANSI 246, mono ID-cyan `#7DCFFF` → ANSI 117.

2. **Status line construction** (design-system §Surface: cli / Component Patterns 3–4): per-P-ID result line carries the ASCII prefix (`✓ P-009 Pass 1840ms <5s` / `✗ P-014 Fail …` / `? P-035 Manual …` / `~ P-032 Residual …` / `• P-022 Blocked …`) with color applied to both prefix and value (TTY only); glyph is ASCII not emoji in piped mode.

3. **Coverage matrix table layout** (design-system §Surface: cli / Component Patterns 3): `comfy-table` 6 columns — P-ID (ANSI 117 cyan) · scenario · state (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]` colored prefix) · slo_tier (`<5s`/`<20s`/`<90s`) · latency_ms (cyan, right-aligned) · fingerprints; Blocked rows render named precondition string in measurement columns; nothing renders as a red error state.

4. **Honest progress spinner** (design-system §Brand Identity / Motion): `indicatif` spinner appears only after ~200ms to avoid churn on fast scenarios; spin stops (not hides) at the frozen hold value; no animation libraries, pure library-native behavior.

## Anti-patterns to avoid

1. **Color-alone encoding** — never rely on color to communicate status (each line pair color + ASCII prefix); no emoji in piped/non-tty output; no exception for decorative use of the status colors.

2. **Silent agent-mode breakage** — never emit interactive prompts (`inquire`) without `isatty` checks; the agent-driven headless path must complete without blocking (interactive TTY gets the prompt, piped/agent context auto-resolves per config); silent hangs or color corruption in agent output are security/reliability failures.

3. **Grafana-style KPI tables** — the coverage matrix is a dense single-row-per-P-ID *list*, not a card grid or metric tile wall; keep information density high and layout linear (top-to-bottom).

## Contract bindings

**a11y ↔ CLI surface** — the "never color-alone" rule (ASCII prefix + color overlay) binds to WCAG SC 1.4.1 (Use of Color); color is an optional enhancement, not the sole encoder. **Supply chain ↔ verification** — three new crates (owo-colors, indicatif, comfy-table) enter `Cargo.lock`; `cargo audit` + `cargo deny` must stay green before merge.

## Acceptance criteria contributions

1. **(design) Status lines render ASCII prefix ALWAYS present; color is optional overlay.** Piped output (via `NO_COLOR` or pipe detection) is unambiguous without color.

2. **(design) Mono ID-cyan (ANSI 117) used ONLY for P-IDs, run_id, SLO timings, latency_ms, fingerprints.** No other value renders in the reserved status tier.

3. **(design) Verdict/ReportState lamp precedence honored.** A CalibrationRegion row shows `[HOLD]`, never `[MANUAL]` or `[RESIDUAL]` — consistent with conductor-report Markdown lamp mapping.

4. **(design) Paused-count spinner stops in place at hold, never hides or animates-to-100%.** Signature behavior preserved: "paused count shows where it stopped."

## Relevant amendment history

**2026-06-15-design-token-typography-bundle**: §Tokens declaration moved from `@theme` to `:root` for Tailwind v4 compat. **Not directly applicable** — this chunk is CLI (terminal ANSI 256, not web CSS tokens) — but the amendment's principle (spec-illustration → sound-impl reconciliation) applies if any owo-colors or indicatif integration deviates from the design-system illustration.