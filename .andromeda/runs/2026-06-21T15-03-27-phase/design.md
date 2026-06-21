# design extract

## Relevance
Partial — verdict classification is non-UI logic, but state enumerations (Verdict, ReportState) are visually rendered per the brand identity, and the hard vs. calibration-region policy affects surface-level signals (color choice, labeling, absence of red alarm).

## Constraints
- **design-system.md §Verdict / ReportState — Semantic Colors:** The three-state verdict triad (green `Pass` / amber `CalibrationRegion` / red `Fail`) + blocked slate-violet + manual lavender + residual muted must be applied consistently; `Fail` is held muted with no flashing per "calm under load, no alarm" (§Brand Identity).
- **design-system.md §Brand Identity:** The "mission-control patience" personality forbids alarm-style pulsing, flashing, or broadcast alarms on any verdict — Fail red is an in-place motionless verdict lamp.
- **design-system.md §Iconography:** Every status (Verdict/ReportState) must be paired with a text label (never color alone per WCAG 1.4.1 use-of-color rule); the verdict lamp is a supporting convention backed by `Pass`/`Fail`/`Blocked`/`Manual`/`Residual` text + `aria-live` announcement.
- **design-system.md §Motion:** At expression level `0.3`, verdict state changes transition via in-place CSS color (150ms ease-out), never animation-library-driven spring or stagger; `prefers-reduced-motion: reduce` must drop all transitions (binding with a11y SC 2.3.3).
- **design-system.md §Anti-Patterns / Per-Surface Bans (cli):** Never use emoji in piped (machine-parseable) output; ASCII prefixes `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]` paired with color (ANSI 114/179/203/60/146/246) for NO_COLOR + screen-reader friendliness.

## Patterns to follow
- **design-system.md §Component Patterns / Verdict lamp:** The 6-state status-light treatment — filled dots for machine verdicts (Pass green / CalibrationRegion amber / Fail red), hollow ring for Blocked, neutral checkbox for ManualCheck, dashed-ring for KnownResidual — each resolves motionless (color transition only, never flashing).
- **design-system.md §Surface: cli / Verdict lines:** Per-P-ID result line format: `✓ P-009 Pass 1840ms <5s` (green) / `✗ P-014 Fail …` (red) / `? P-035 Manual halo→burgundy?` (lavender) / `~ P-032 Residual recent_commits stub → v0.3.0` (muted) / `• P-022 Blocked mcp-server feature + …` (violet) — color + ASCII prefix, never color alone.
- **design-system.md §Color Palette / Semantic Colors:** Verdict state palette is pre-defined (§Semantic Colors table); classification logic must emit exactly these mapped values (no ad-hoc color derivation or hex tweaks).

## Anti-patterns to avoid
- **design-system.md §Anti-Patterns / Rejected Defaults:** NEVER flash / pulse / blink / glow on a Fail verdict; the verdict lamp resolves in place, motionless. NEVER conflate "no result yet" with "failed" (graying both the same or showing Blocked as a red error).
- **design-system.md §Anti-Patterns / Per-Surface Bans (cli):** NEVER use emoji in piped output or rely on color alone; NEVER skip the ASCII prefix for accessibility + NO_COLOR compatibility.

## Contract bindings
**a11y ↔ color + text pairing:** Every verdict/state color must pair with a text label + aria-live announcement (WCAG 1.4.1 use-of-color; design-system.md §Iconography). **a11y ↔ motion:** All transitions must drop under `prefers-reduced-motion: reduce` (design-system.md §Motion; SC 2.3.3 binding). **CLI ↔ NO_COLOR / TTY detection:** verdict-classification output (piped or interactive) must respect `NO_COLOR` / ANSI stripping when not a TTY (design-system.md §Surface: cli / Platform-Specific Notes).

## Acceptance criteria contributions
- "(design) Verdict state enum (Pass/Fail/CalibrationRegion) values map to the semantic color palette (§Color Palette §Semantic Colors) with no ad-hoc color derivation."
- "(design) Hard vs. calibration-region policy is declared as a property of the assertion, not guessed at runtime — the classification mechanism enforces the split."
- "(design) CLI verdict lines pair ASCII prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/etc.) with ANSI color; never color alone. Respect `NO_COLOR` and piped-output ANSI stripping (design-system.md §Surface: cli)."

## Relevant amendment history
(none)
