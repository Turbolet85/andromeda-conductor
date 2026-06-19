# Wrap fan-out — 2026-06-19-emission-gap-resume

7 Explore doc-agents, one per spec source, each read `report.md` + its doc + its scoped drift-base detectors.
Raw returns preserved in the wrap transcript. Parsed result:

| doc | detectors evaluated | result |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `proposals: []` — EmissionGap/FaultError are internal library symbols in the already-registered `conductor-faults` crate; no new deps/runtime |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | `proposals: []` — constructor-validated input (no external boundary/deserialize); no sidecar touch; no deps |
| design-system | D-design-tokens | `proposals: []` — no UI |
| layout-templates | D-layout-surface | `proposals: []` — no user-facing surface |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `proposals: []` — 8 unit + doctest at the unit tier; nextest runner; harness/envelope untouched |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | `proposals: []` — no telemetry (fault.silence deferred); no OTel SDK; no logging/path leak |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` — no UI; no schema change |

**Aggregate:** 0 proposals · 0 amendments · 0 escalations · no cascade (no spec source changed).
Drift = 0 — clean wrap (matches the self-contained backend-library shape of the chunk).
