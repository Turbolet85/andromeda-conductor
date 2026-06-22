# design extract

## Relevance — partial
Scenario-catalog (TOML) chunk with NO new UI surface, but severity-state semantics bind to the verdict/report-state lamp + CLI severity-token rendering (forward-looking to Epoch 9; not authored here). Per design-system §Color Palette / §Iconography.

## Constraints — domain rules that apply
1. Per design-system §Color Palette, the three-tier severity model (P-019 Autonomous/Suggested/Curious) maps to existing semantic-color strata or declares new tokens; `Resolved` is a lifecycle state (not a verdict) and must not collapse into the pass/fail triad.
2. Per design-system §Iconography / §Color Palette, every observable tier/state carries a paired text label — never color-alone.
3. Per design-system §Motion, any lifecycle state transition (Active→Resolved tint change) uses the micro-transition timing and drops entirely under `prefers-reduced-motion: reduce`.
4. Per design-system §Component Patterns (Surface: cli), severity tiers map to ASCII status prefixes (`[AUTONOMOUS]`/`[SUGGESTED]`/`[CURIOUS]`/`[RESOLVED]`) paired with color per the NO_COLOR + screen-reader rule.
5. Per design-system §Typography, `Resolved` status in the run-report uses IBM Plex Sans body (`--text-secondary`); per-P-ID status tokens in CLI use tabular-nums JetBrains Mono.
6. Per design-system §Border Radius / §Spacing, any new status container uses radius + 4px-base spacing tokens; no magic numbers.

## Patterns to follow — existing patterns relevant to implementation
1. Verdict/report-state lamp (design-system §Component Patterns) is the established per-P-ID status container; severity tiers are NOT verdicts — they are an incident attribute surfaced as a `Contains "Autonomous"`-style token, resolving to a `ManualCheck` lamp (severity choice is CalibrationRegion).
2. `Resolved` lifecycle status (design-system §Color Palette + §Motion) renders as neutral metadata ("Resolved after 127 s", `--text-secondary`), not a dedicated verdict glyph.
3. Multi-interpretation sustained incident (P-059, design-system §Component Patterns run-report) — each interpretation a distinct labeled row; CLI honors terminal density.
4. Cool-down absence (P-023, design-system §Motion) — the visual signal is the ABSENCE of a new P-ID row (silence = the signal); no new UI element.

## Anti-patterns to avoid — domain bans that apply
1. NEVER conflate severity tier (Autonomous/Suggested/Curious) with verdict class (Pass/Fail/CalibrationRegion) — tiers are an incident attribute; the verdict routes to `ManualCheck` (per design-system §Verdict-vs-ReportState distinction).
2. NEVER use color alone for severity tiers or lifecycle states — text label + color always (per design-system §Color Palette SC-1.4.1 alignment).
3. NEVER animate severity-tier transitions — tier choice is model output, not a user action; only the Active→Resolved tint transition is choreographed, and it drops under reduced-motion (per design-system §Motion).

## Contract bindings — where your domain ties into another
- **design ↔ a11y** — severity tiers + lifecycle states must meet contrast minimums and the not-color-alone rule (SC 1.4.1 / 1.4.11).
- **design ↔ operator-checklist** — if P-023 ack is undrivable (scope Q4), ack becomes an operator-checklist action using the `--status-manual` checkbox + checklist card.
- **design ↔ Epoch-8 evaluator** — design renders the declared tokens (`Contains "Autonomous"`/…/`Resolved`); evaluator correctness is out of design scope.

## Acceptance criteria contributions — concrete pass/fail checks your domain adds
1. (design) [Epoch-9 forward] Severity tiers rendered with paired text label + color, never color-alone (per design-system §Color Palette).
2. (design) [Epoch-9 forward] `Resolved` status uses design tokens + reduced-motion-safe transition (per design-system §Motion / §Typography).
3. (design) [Epoch-9 forward] Multi-interpretation (P-059) respects information density without double-counting rows as separate verdicts (per design-system §Component Patterns).
4. (design) [Epoch-9 forward] Cool-down absence renders as silence (no false "suppressed" glyph) (per design-system §Component Patterns).

## Relevant amendment history — prior amendments to your plan touching this chunk's area + why
- **2026-06-15-design-token-typography-bundle** — token declaration moved from `@theme` to plain `:root` + `@media (prefers-color-scheme: light)`. Any new severity-tier / resolved-state color tokens (if added in Epoch 9) follow the `:root` + `@media` pattern, NOT `@theme`.
