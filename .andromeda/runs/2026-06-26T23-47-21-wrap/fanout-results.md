# Fan-out results — 2026-06-26-component-primitives-library wrap

7 Explore doc-detector agents (one per spec source) run against `report.md`. drift = 0 on exit.

| doc | detectors | result |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `proposals: []` — no new crate/IPC/port/env-var; `@radix-ui/react-alert-dialog` is a React component lib within the §Stack-allowed frontend (React 19 + npm + audit gate). (Pre-empted by playbook 2026-06-26 frontend-component-package over-reach rule.) |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | `proposals: []` — no external-input surface, no sidecar touch; the new npm dep passes `npm audit` 0 + `package-lock.json` committed per §Dependency Security §Frontend supply chain. |
| design-system | D-design-tokens | **1 proposal (warning, routine, APPLIED)** — reconcile the operator-pause dialog fade `200ms → --motion-micro` (150ms). |
| layout-templates | D-layout-surface | `proposals: []` — StatusLamp/OperatorPauseDialog/OperatorChecklist all map to existing §Component wireframe entries; Gallery is DEV-only (not a user surface). |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `proposals: []` — build-gated frontend per §4 (tsc+vite+npm audit); GUI tests deferred ch9; no Rust/harness/schema change. |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | `proposals: []` — UI-only; no must-trace op, no OTel SDK, no logging/artifact/redaction surface. |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` — the three interactive elements carry built-in WCAG/focus/keyboard coverage (§4–§6); no violation-schema change. |

## Applied amendment
- **design-system.md** §Motion (This project's values, line 161) + §Component Patterns §2 (Operator-pause dialog, line 255): `200ms fade` → `150ms fade (--motion-micro)`. Generic expression-scale ceiling (line 18 "200ms fades at most" + line 154 `0.3-0.4 → 200ms fade` reference row) left unchanged — 150ms satisfies it.
- **design-system-amendments.md**: appended the reconciliation entry.
- **Validation:** routine per playbook (spec-illustration → sound-impl reconciliation; `@theme`→`:root` precedent). The never-hardcode/never-color-alone invariants hold (the fade is token-bound). Intent-consistent (plan.md itself specified `--motion-micro` for the dialog fade).
- **Cascade:** no-op — `design-summary.md` / `rules/{frontend,a11y}.md` carry no fade-duration literal (verified by grep; they reference motion by token/expression).

## Escalations
none (0).
