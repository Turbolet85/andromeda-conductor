# P2 Fan-out — drift proposals (2026-06-15-log-error-boundary-redaction)

7 doc-agents, one per spec source, each evaluating its drift-base detectors against `report.md`.

| doc | detectors run | result |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `proposals: []` — no new port/crate/env-var; no new dep; anyhow-free in core (consistent with §Error handling) |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | `proposals: []` — sanitizers are not external-input boundaries; sidecar untouched; zero new deps, lock un-drifted |
| design-system | D-design-tokens | `proposals: []` — no new UI rendered |
| layout-templates | D-layout-surface | 1 proposal — **REJECTED (false positive)** |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `proposals: []` — nextest used; harness/envelope unchanged; noted the reconcile is obs-plan's, not test-plan's |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | 1 proposal — **D-obs-redaction (escalate) → APPLIED after user resolution** |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` — no UI; obs §6 envelope unchanged |

## Proposal 1 — D-obs-redaction (obs-plan) — APPLIED (escalated, user-resolved)
- **Escalated** because drift-base severity is `escalate` (touches the artifact-hygiene Critical Warning) and no playbook rule downgraded it.
- **User decision (AskUserQuestion, 2026-06-15 wrap):** "Reconcile §11 + add playbook rule."
- **Applied:** obs-plan §6 CI conformance gate (line ~528) + §11 Anti-Patterns (Logs line ~591, PII Scrubbing line ~604) reconciled to the implemented model — value scrub anchors on absolute host-FILE paths → `<redacted>` (NOT `::`-tokens); struct-name guard = field-name allowlist drop + `Display`-not-`Debug` at the `anyhow` edge; allowlisted `target` module path preserved. Sidecar appended (`obs-plan-amendments.md`). Playbook rule added (sound redaction-reconciliation = routine). Cascaded to `obs-summary.md` + `observability.md`.

## Proposal 2 — D-layout-surface (layout-templates) — REJECTED (main's validation)
- The agent proposed adding the self-obs JSON log-line redaction to the cli wireframe component.
- **Rejected as not-drift:** the self-obs JSON *log line* is a machine-parseable stream, NOT a UI surface/wireframe/region. The chunk added no new UI; the cli error-output surface (which already documents the sanitization contract) is untouched this chunk — its wiring is deferred to Epoch 8. Log-format detail belongs to obs-plan/test-plan, not layout-templates (category error). No edit.

## Outcome
1 amendment applied (obs-plan §6/§11, cascaded ×2) · 1 escalation resolved (user) · 1 proposal rejected · 1 playbook rule added. **Drift = 0.**
