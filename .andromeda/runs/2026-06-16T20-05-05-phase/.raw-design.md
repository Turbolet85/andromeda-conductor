# design extract

## Relevance
Partial — backend scenario-config model is data/schema layer; design system governs rendering surfaces (desktop-webview / cli outputs), not configuration data structure.

## Constraints
1. **Phase identity must fit mono status tier** (design-system §Typography / §Color Palette) — the phase kind string (e.g., "error-baseline-spike", "HOLD") displays in phase-line label (IBM Plex Sans 600, 18px) and must legible + terse enough for titlebar space; mono-ID-cyan reserved strictly for count/P-IDs/run_id/fingerprints, not phase kind.
2. **Phase ordering is signature-critical** (design-system §Brand Identity, Paused-count hold-point) — the sequence must be deterministic and immutable per run; if a scenario declares a phase list, its order preservation maps 1:1 to the timeline's frozen-count readout sequence.
3. **Timing values (duration/gap) must be expressible in the phase-line + titlebar count context** (design-system §Motion, Expression level 0.3) — durations in the millisecond range; no fractional seconds that lose precision in the hearbeat tick (the frozen count reads exact elapsed / step index, never rounded).
4. **Config structure must NOT encode color/status rendering logic** (design-system §Color Palette, Color-as-status rule) — the emission spec is data only; color/verdict assignment is downstream (verdict engine, Epoch 5); config must never hardcode `#7EE787` or map a phase kind to "Pass".
5. **Phase-sequence bounds must be consistent with the coverage-matrix 60-P-ID wall** (design-system §Component Patterns, Coverage matrix) — the scenario's declared phases flow into the single-row-per-P-ID report; phase count must not break the matrix density assumptions.

## Patterns to follow
1. **Declarative schema → rendered output** — the config (serde structs) is data; rendering (titlebar phase-line, phase-line label, report rows) is downstream (frontend/cli); separate concerns.
2. **Deterministic, order-preserving conversion** — the `Scenario → PhaseTimeline` mapping must be total and reproducible; no hidden side effects, no reordering, no conditional branching based on runtime state (the frozen count signature depends on this).

## Anti-patterns to avoid
1. **DO NOT embed color / verdict rendering rules in the config model** — phase kind is a string identifier, not a semantic status; verdict (Pass/Fail/CalibrationRegion/Blocked) is assigned by the verdict engine downstream, never pre-baked in config.
2. **DO NOT model phase duration in a way that loses precision for titlebar heartbeat ticks** — if the phase duration is in integer milliseconds (per timing spec), keep it precise through config deserialize → timeline → count updates; rounding/averaging in config is a foot-gun.

## Contract bindings
- **Config → phase-line rendering** (design-system §Component Patterns, Frameless titlebar) — phase kind string must render in the 18px IBM Plex Sans heading slot without truncation/wrapping; Epoch 2 chunk 2 implementation must verify titlebar space can fit declared phase kinds; if a scenario declares a phase longer than available titlebar width, that is a config-validation warning (forward-link to design).
- **Phase sequence → Coverage matrix row count** — the scenario's phase list contributes to the run's row count in the coverage matrix (single-row-per-P-ID wall); config must not declare phases in a way that breaks the 60-P-ID assumption or matrix density expectations (forward-link to Epoch 5 verdict harness).

## Acceptance criteria contributions
1. **(design) Phase-kind string legible in titlebar.** The phase `kind` field must be under 40 characters (matching the headroom in the frameless titlebar at 18px IBM Plex Sans 600, leaving space for the count on the right); config validation rejects longer kind strings with a hint suggesting a shorter alias.
2. **(design) Phase sequence is deterministic and immutable.** The `Scenario → PhaseTimeline` conversion is order-preserving and reproducible; given the same config + seed, the phase sequence reads identical in the titlebar count / report / coverage matrix (the frozen heartbeat signature depends on this).

## Relevant amendment history
(none) — no prior amendments to design-system.md touch the scenario-config model area; the initial design-system.md (2026-06-14) established the Brand Identity + signature (paused-count) and Component Patterns (phase-line rendering in titlebar). The one amendment (2026-06-15-design-token-typography-bundle) concerns token CSS syntax, not scenario modeling.
