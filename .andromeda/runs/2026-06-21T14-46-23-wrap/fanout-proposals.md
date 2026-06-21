# P2 fan-out — drift reconcile (2026-06-21-otlp-egress-liveness-check)

7 Explore doc-agents, one per spec source, each evaluating its scoped drift-base detectors against
`chunks/2026-06-21-otlp-egress-liveness-check/report.md`. **Result: 7/7 `proposals: []` → drift = 0.**

| doc | detectors evaluated | verdict | one-line rationale |
|---|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `proposals: []` | library symbols on the already-registered `:4317` path; arch §Standard Contracts already specifies the Liveness equivalent; std::time + existing tonic |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | `proposals: []` | endpoint is internal (not a garde config surface); no sidecar/data-dir touch; 0 new deps; Cargo.lock un-drifted |
| design-system | D-design-tokens | `proposals: []` | no UI rendered (n/a) |
| layout-templates | D-layout-surface | `proposals: []` | no user-facing surface/region added (n/a) |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `proposals: []` | 3 tests at unit/integ tier via cargo-nextest + loopback stub; harness/JSONL obs-bind unchanged |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | `proposals: []` | probe is not a must-trace op (bounded span-set has no liveness/connect span); no OTel SDK; EmitError Display carries no path/struct leak |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` | no interactive UI element; no schema change (n/a) |

Validation: 0 proposals → 0 playbook checks · 0 cross-contradictions · intent-consistent. No spec body
amended; no sidecar appended; cascade not triggered (no changed source). drift = 0.

(Raw agent returns consolidated here — all 7 stripped to `proposals: []`; full reasoning held in the wrap
conversation transcript.)
