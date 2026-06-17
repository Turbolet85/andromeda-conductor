# P2 fan-out — drift detection (2026-06-17-determinism-replay-harness)

7 Explore doc-agents, one per spec source, each evaluating its scoped detectors against `report.md`.
**Result: 7/7 → `proposals: []` (zero drift).**

| doc | detectors evaluated | verdict |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `proposals: []` — no new resource (test-only); start_paused/proptest/insta all within §Stack |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | `proposals: []` — no new input surface (fixture via existing garde-validated `from_toml_str`); sidecar untouched; **no new dep** |
| design-system | D-design-tokens | `proposals: []` — no UI element rendered |
| layout-templates | D-layout-surface | `proposals: []` — no user-facing surface added |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `proposals: []` — nextest+proptest+insta(assert)+start_paused match §2/§4; golden excludes journal stamps so §3↔obs §3 stay aligned |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | `proposals: []` — no new must-trace op; no OTel SDK; golden captures only `Vec<PhaseTransition>`, no host-path/struct leak |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` — no interactive UI; no violation-schema change |

**Validation (main):** 0 proposals → 0 playbook checks, 0 cross-contradictions, 0 escalations. Intent-consistency: report aligns with the working-route entry + plan acceptance criteria (all met; deviations minor/justified). **Cascade: not triggered (no spec body amended). drift = 0.**

Playbook rule 6 (dismiss a dep-bump for an untouched dep) was pre-armed against a D-security-deps `tauri` misfire — but the detector correctly read the report's "no new dependency" and did not fire. No new playbook rule or detector warranted (no escalation occurred).
