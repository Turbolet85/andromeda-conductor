# design extract

## Relevance
partial — The chunk is pure types with no rendering surface; however, it defines the `Verdict`/`ReportState` status enums that the design system's entire color/convention system builds on.

## Constraints
- Per design-system §Color Palette, `ReportState` and `Verdict` must carry distinct semantic color meanings in their serialized forms (e.g., `Pass` = `--count-nominal` `#7EE787` green in rendering, `Fail` = `--status-fail` `#F85149` red, `CalibrationRegion` = `--count-hold` `#E3B341` amber, `Blocked` = `--count-blocked` `#565F89` violet, `ManualCheck` = `--status-manual` `#A9B1D6` lavender, `KnownResidual` = `--status-residual` `#9A93A8` muted). Each variant must be a distinct identity at the type level so rendering code has no ambiguity.
- Per design-system §Iconography, each `ReportState` and `Verdict` must support a distinct status-lamp glyph (filled dot for Pass/CalibrationRegion/Fail; hollow ring for Blocked; checkbox for ManualCheck; dashed ring for KnownResidual). The enum must expose these as `From`/`Into` associations or accessor methods.
- Per design-system §Typography, the mono ID-cyan `#7DCFFF` typographic tier (P-IDs, run_id, SLO timings) is rendered only via data values, never as status variants — enums have no dependency on typography.
- Per design-system §Anti-Patterns Universal Bans, status colors are never decoration; every enum variant must have a semantic meaning (calm verdict logic, not alarm-driven). `Fail` and `Blocked` are visually distinct (red vs. violet), never conflated.
- Per design-system §Motion / Expression Level 0.3, the `Verdict`/`ReportState` enums carry no animation/transition directives — motion is applied at render time (color transitions on status change are 150ms ease-out, applied by the consumer surface, not the enum).

## Patterns to follow
- Per design-system §Component Patterns #4 (Verdict/report-state lamp) and #6 (Run-report view), each enum variant must encode both its text label (e.g., `"Pass"`, `"HOLD"`, `"Fail"`) and its ASCII prefix for cli (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]`). Implement a trait method `as_label(&self) -> &'static str` and `as_status_prefix(&self) -> &'static str` so rendering code never embeds these strings.
- Per design-system §Color Palette §Semantic Colors table, the five `ReportState` rows (Success, Warning, Error, Info/Blocked, Manual, Residual) each define a unique color tuple (background, border, text); the enum's serialized form must map 1:1 to these rows so CSS can bind `data-state="Pass"` to the correct color layer.
- Serde round-trip the enums to their PascalCase canonical names (`"Pass"`, `"Fail"`, `"CalibrationRegion"`, `"Blocked"`, `"ManualCheck"`, `"KnownResidual"`), matching the run-report envelope exactly per design-system §Standard Contracts.

## Anti-patterns to avoid
- NEVER flatten `ReportState` and `Verdict` into a single enum — they are logically distinct (machine verdict triad vs. report outcome quintet) and design-system §Color Palette explicitly separates them as "(3) Verdict…vs. (5) ReportState."
- NEVER use `Option<Verdict>` to represent missing verdict — use the explicit `ReportState::Blocked` ("never measured") instead. `None` elides intent and risks silent misreading as an error.
- NEVER add animation/easing/timing fields to the enum — the design system applies 150ms ease-out color transitions at the renderer, not the type.

## Contract bindings
**status-rendering (design ↔ color system):** The enums' serialized names and semantic meanings bind directly to design-system §Color Palette and §Iconography. Each `ReportState` variant must map to exactly one semantic color row (e.g., `ReportState::ManualCheck` → neutral-lavender `--status-manual` `#A9B1D6` + checkbox glyph, never green/red). This binding is **not encoded in code** (the enum carries no color values), but it **must be documented in code comments** so rendering-phase code can audit the mapping.

**cli never-color-alone rule (design ↔ cli contract):** Per design-system §Surface: cli and CLAUDE.md "Status is never color-alone," every status must include both an ASCII prefix (`[PASS]`/`[FAIL]`/etc.) and a text label. The enum's accessor methods (`as_label()`, `as_status_prefix()`) enforce this at compile time — rendering code that skips either accessor is a compile error.

## Acceptance criteria contributions
- (design) `Verdict { Pass, Fail, CalibrationRegion }` enum defined with no variants outside this set; each serializes to PascalCase name matching run-report envelope.
- (design) `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` enum defined with all five variants; each is visually distinct per design-system §Color Palette §Semantic Colors (no two map to the same color tuple).
- (design) Both enums expose `as_label(&self) -> &'static str` returning the human text (`"Pass"`, `"HOLD"`, `"Fail"`, etc.) and `as_status_prefix(&self) -> &'static str` returning the cli ASCII prefix (`[PASS]`, `[HOLD]`, `[FAIL]`, `[BLOCKED]`, `[MANUAL]`, `[RESIDUAL]`). Rendering code must call both — never render color alone.
- (design) Each enum variant has a documented comment binding it to the semantic color row from design-system §Color Palette (e.g., `/// Verdict::Pass — renders as --count-nominal #7EE787 green filled dot + "[PASS]" label.`).
- (design) Serde round-trip test verifies enums serialize/deserialize to canonical PascalCase names and remain bit-compatible with the run-report envelope spec.

## Relevant amendment history
(none) — this is the initial design-system plan; no amendments yet.
