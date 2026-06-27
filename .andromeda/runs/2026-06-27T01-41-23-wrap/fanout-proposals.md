# Fan-out proposals — 2026-06-27-run-report-operator-checklist-views

7 doc-agents (Explore), one per spec source. 2 proposals, both routine-dismiss; 5 clean.

## arch — D-arch-resources (warning)
Proposal: register the `run_report` `#[tauri::command]` name in arch §Occupied Resources (Tauri command surface).
**Disposition: DISMISS (routine)** — playbook 2026-06-26 Tauri-command-name over-reach rule. arch §Occupied Resources
registers the Tauri command surface at CATEGORY grain ("run-report view" is already listed); the handler fn name
`run_report` is the realization arch omits (same as list_scenarios/start_run/stop_run/coverage_matrix, none registered).
Identical to ch6's auto-dismiss of `coverage_matrix`. D-arch-decisions: clean (no new dep/runtime/decision).

## security — D-security-input (warning, agent-downgraded from escalate)
Proposal: add a "Tauri command parameters (run_report run_id)" row to security-plan §Input Validation table.
**Disposition: DISMISS (routine)** — playbook 2026-06-23 consuming-shipped-infra rule (names D-security-input +
resolve_under + "Epoch-9 Tauri consuming the same hardened entry points"). run_report's `run_id` IS validated via the
CONSUMED, unchanged `resolve_under` traversal-guard (report Coverage flag ✓) — no security gap, no new validation
mechanism. The Tauri-command-input-validation requirement is already documented in security-plan §Tauri GUI / §Code
Patterns ("validate the command's input … resolves dirs via resolve_under", from start_run ch3); the §Input Validation
table tracks input CATEGORIES, so a per-command row is the per-item over-reach flavor. start_run (ch3, `selection` param)
set the precedent — no §Input Validation table row was added. D-security-subprocess: clean (no sidecar/data-dir touch).
D-security-deps: clean (Dependencies NONE).

## design — D-design-tokens: proposals: [] (RunReport/OperatorChecklistView tokens ✓, never-color-alone ✓)
## layouts — D-layout-surface: proposals: [] (run-report maps to §Wireframe Run report / Primary content block 2; operator-checklist view is the documented ManualCheck component, gallery-only)
## tests — D-tests-coverage/framework/obs-harness: proposals: [] (run_journal unit-tested via nextest; envelope unchanged)
## obs — D-obs-instrumentation/stack/redaction: proposals: [] (run_report span ✓; no OTel SDK; envelope unmodified + sanitize_error edge)
## a11y — D-a11y-surface/obs-schema: proposals: [] (table/role=group + StatusLamp + role=status footer; RunRecord TS mirror matches the 11-field schema)

## Outcome
0 spec amendments · 0 escalations · 5 docs clean · drift = 0. No cascade (no spec body changed).
