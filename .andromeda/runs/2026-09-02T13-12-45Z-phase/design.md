# design extract

## Relevance
Partial — only the CARRY-2 desktop load-envelope banner leg (and its cross-surface token-parity framing) is in the design domain; the v2-25 DB-envelope field parity, the v2-28 rmcp wording sweep, and the rust gate are outside it.

## Constraints
- The over-envelope banner is a **run-level qualifier in the recessive Residual-mute tier, bound BY NAME** — `var(--status-residual)` on webview, ANSI 246 on cli — never `--status-fail`, never a new color (per design-system.md §Surface: cli / Tokens — Residual-mute entry; §Surface: desktop-webview / Tokens). Whether the shipped banner actually binds that token by name (vs a raw hex or a Fail-red tint) is research's question.
- The `[ENVIRONMENT-SUSPECT]` caption must **not become a seventh lamp state or a sixth `ReportState`** — it renders outside the lamp column, above the per-P-ID verdict lines, once per run (per design-system.md §Surface: cli / Component Patterns 4; §Surface: desktop-webview / Component Patterns 4).
- **Never color-alone**: the always-rendered `ENVIRONMENT-SUSPECT` text label carries the signal and the tint only de-emphasizes (per design-system.md §Surface: cli / Tokens; §Anti-Patterns / Universal Bans). This is exactly what the DOM-label assertion in the CARRY-2 spec exercises.
- **Motionless resolution**: no pulse / blink / glow / flashing entrance on the banner; at most an in-place `--motion-micro` (150ms) `--ease-quiet` color transition, and every transition must drop under `@media (prefers-reduced-motion: reduce)` (per design-system.md §Motion — Hard limits; §Surface: desktop-webview / Tokens reduce-motion block).
- If the banner is a container, depth is **borders-only**: `--color-raised-1` fill, `1px solid --border-subtle`, `--radius-md`, zero `box-shadow` / `backdrop-filter` (per design-system.md §Depth Strategy; §Border Radius; §Surface: desktop-webview / Component Patterns 6).
- Typography split holds inside the banner: caption prose in IBM Plex Sans; only the reserved status tier (run_id, timings, `latency_ms`, P-IDs) in JetBrains Mono at `--color-id-cyan` (per design-system.md §Typography; §Anti-Patterns / Rejected Defaults — "mono everywhere").
- **Cross-surface parity in this domain is parity of token NAME and label text, not of rendering** — the same semantic pair binds as ANSI 246 (cli) and `var(--status-residual)` (webview), with Markdown using emphasis or the bracket label as its color-free counterpart (per design-system.md §Surface: cli / Tokens). Design adds no visual claim to the v2-25 DB-field parity proof.

## Patterns to follow
- The cli run-level caption pattern — printed once, above the per-P-ID verdict lines, in ANSI 246, paired with its ASCII bracket label (design-system.md §Surface: cli / Component Patterns 4). The desktop banner is this caption's webview counterpart and must read as the same statement.
- The run-report view container spec — `--color-raised-1` card, `1px --border-subtle`, `--radius-md`, prose in `--text-secondary` with mono IDs/timings in `--color-id-cyan`, no shadow (design-system.md §Surface: desktop-webview / Component Patterns 6). The banner lives inside this surface.
- **Reuse the existing by-name pair rather than adding a palette row** — this domain has applied that resolution three times for the Residual-mute tier (design-system.md §Surface: cli / Tokens; see amendment history below).
- The "never downgrade to red" discipline already applied to `Blocked` / `KnownResidual` — an environment qualifier is a statement about the run, not a failure of it (design-system.md §Color Palette — Verdict-vs-ReportState note; §Anti-Patterns / Rejected Defaults).

## Anti-patterns to avoid
- A flashing / pulsing / animated alert banner (or a native OS toast) on an adverse run state — banned outright; Conductor must not emit OS toasts, it observes them (design-system.md §Anti-Patterns / Rejected Defaults; §Motion — Hard limits; §Surface: desktop-webview / Platform-Specific Notes).
- Raw hex, raw ms, or a fresh palette/ANSI entry for the envelope state — tokens are bound by `var(--…)` name only (design-system.md §Surface: desktop-webview / Tokens; §Self-Validation Protocol — Token Test).
- Collapsing the envelope qualifier into the verdict triad (rendering it as `Fail` red, or as a lamp glyph in the P-ID lamp column) (design-system.md §Surface: cli / Component Patterns 4).

## Contract bindings
- **Token contrast ↔ a11y §Contrast (SC 1.4.3)** — the design mandate is `--status-residual` legible over both `--color-raised-1` and `--color-base` in both themes; the CARRY-2 axe run against a populated banner state is where that render-independent pair is finally measured on a real DOM. a11y owns the ratio; design owns the token pair.
- **Label + color ↔ a11y §Use of Color (SC 1.4.1)** — the always-rendered `ENVIRONMENT-SUSPECT` text is design's not-color-alone guarantee; the a11y spec asserts it.
- **Motion tokens ↔ a11y §Animation (SC 2.3.3)** — the reduce-motion override is mandatory on any banner transition.
- **Design ↔ tests harness** — design owns the caption's label string; `test/a11y/accessibility.e2e.ts:270-280` (`report__envelope-label` expected to read `ENVIRONMENT-SUSPECT`) is the assertion site. The scope's seeded-row-vs-live-Pulse fork decides which state that token pair and label are measured on; design is indifferent to the fork's mechanism but requires the label and token to be identical in both.

## Acceptance criteria contributions
- The over-envelope banner renders in the Residual-mute tier bound by name (`var(--status-residual)` webview / ANSI 246 cli) with no new palette entry, no raw hex, and no `--status-fail` tint (per design-system.md §Surface: cli / Tokens · §Surface: desktop-webview / Tokens).
- The banner carries its always-rendered `ENVIRONMENT-SUSPECT` text label and sits outside the per-P-ID lamp column — not a seventh lamp, not a sixth `ReportState` (per design-system.md §Surface: cli / Component Patterns 4).
- The banner resolves motionless — no pulse/blink/glow/flash and no animated entrance; any transition is `--motion-micro`/`--ease-quiet` and is dropped under `prefers-reduced-motion: reduce` (per design-system.md §Motion — Hard limits).
- Any banner container uses `--color-raised-1` + `1px --border-subtle` + `--radius-md` with zero `box-shadow`/`backdrop-filter` (per design-system.md §Depth Strategy · §Surface: desktop-webview / Component Patterns 6).

## Relevant amendment history
- **2026-08-09-sut-load-envelope** — §Color Palette Residual-mute entry amended to name the tier's non-lamp uses as a SET, adding the run-level `[ENVIRONMENT-SUSPECT]` load-envelope caption and its Markdown counterpart. This is the direct precedent for this chunk's subject: the caption was registered as a *reuse of the existing tier*, not a new state.
- **2026-08-09-out-of-scope-classification-treatment** — the same Residual-mute entry was amended to record the tier's non-lamp uses and to name `var(--status-residual)` as the webview half of the by-name cli/webview pair; §cli Pattern 3 retitled to disambiguate the results/SLO table from the coverage matrix. Establishes the cross-surface by-name binding this chunk's parity framing rests on.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — the precedent that a new surface using ANSI 246 gets a *use-site note*, not a new palette row (a proposed "Hint grey" row was corrected away at validation). Applies if this chunk finds an unregistered banner treatment.
- **2026-09-01-desktop-a11y-sweep** — moved `--text-tertiary` / `--text-muted` for SC 1.4.3 and explicitly recorded `--status-residual` as untouched. Relevant because the CARRY-2 axe run is the first render-time exercise of the residual pair since that sweep.
- **2026-09-02-screen-reader-manual-spec** — corrected §Component Patterns 6 (run-report empty / in-progress) to the shipped strings and retired prose the code does not produce. Same surface as the banner, and the same doc-vs-shipped reconciliation channel this chunk may need if the banner's rendering deviates from spec.
