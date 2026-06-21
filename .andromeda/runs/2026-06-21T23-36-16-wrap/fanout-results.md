# Wrap fan-out results — 2026-06-21-hard-signals-scenarios

7 Explore doc-agents, one per spec source, run against `report.md` alone. **All clean — `proposals: []`.**

| doc | detectors evaluated | result |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `proposals: []` — zero new resources (no port/socket/endpoint/IPC/event/env-var/crate); scenario TOMLs are config content in the already-registered `scenarios/` dir; no new lib/runtime; reuses existing types. |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | `proposals: []` — the 4 scenario TOMLs are garde-validated via `Scenario::from_toml_str` (input boundary covered); sidecar/preflight/data-dir untouched; zero new dependencies. |
| design-system | D-design-tokens | `proposals: []` — no UI rendered (tokens n/a). |
| layout-templates | D-layout-surface | `proposals: []` — no new user-facing surface/region. |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `proposals: []` — unit tests at the mandated tier (rstest round-trip); nextest+rstest framework matches §2/§4; harness/envelope/JSONL untouched (the smoke-command note is a procedural learning, not §3 drift). |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | `proposals: []` — no new runtime operation (no span needed); no OTel SDK; no logging/artifact write to redact. |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` — no interactive UI; no schema change. |

**Aggregate:** 0 proposals · 0 escalations · 0 amendments · no cascade (no spec body changed). **Drift = 0.**
