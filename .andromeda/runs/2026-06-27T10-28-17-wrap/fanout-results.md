# Fan-out results — 2026-06-27-operator-pause-go-no-go-dialog wrap

7 doc-detectors, 2 proposals, 5 clean.

## arch — D-arch-resources (warning) — PROPOSAL
- change: register `resolve_operator_hold` command + a 2nd `Channel<HoldPrompt>` ("one Channel" → "two Channels") in §Occupied Resources.
- D-arch-decisions: clean (serde already a workspace dep, tokio sync a standard feature; no new library/decision).

## security — clean (`proposals: []`)
- D-security-input: `resolve_operator_hold` input is a closed serde-bounded `Decision` enum (validation present, not unvalidated). D-security-subprocess: no sidecar/data-dir touch. D-security-deps: no new Cargo.lock package; audit+deny green; tauri 2.11.3 ≥ floor.

## design — clean (`proposals: []`)
- D-design-tokens: App/Gallery use `var(--…)` only; dialog reuses tokened scaffold; Proceed/Abort pair color+text.

## layouts — clean (`proposals: []`)
- D-layout-surface: the operator-pause dialog maps to §Wireframe Run console HOLD + §Component Hero/signature (desktop + cli); no new undocumented surface.

## tests — clean (`proposals: []`)
- D-tests-coverage: gate logic unit-tested (arm/deliver); Tauri IPC integration explicitly deferred to ch9 per test-plan §5. D-tests-framework: cargo-nextest matches. D-tests-obs-harness: no harness/envelope/schema change.

## obs — D-obs-instrumentation (warning) — PROPOSAL
- change: add a run_id-exemption note to obs §3 for stateless commands (`resolve_operator_hold`/`list_scenarios`/`coverage_matrix` run outside a scenario run → no run_id). D-obs-stack: clean (tracing only). D-obs-redaction: clean (decision label only, no path/struct).

## a11y — clean (`proposals: []`)
- D-a11y-surface: dialog reuses the shipped accessible Radix `alertdialog` (role+focus-trap+Escape+restore); full axe/keyboard defers ch9. D-a11y-obs-schema: no schema change.

## Validation verdicts (main)
- **D-obs-instrumentation → DISMISS** per playbook 2026-06-27 (read-only/stateless command run_id rule) — which **explicitly pre-empts "ch8 (operator-pause command)"**. obs §3 "run_id on every line" already governs the scenario-run stream; a decision-post outside a run legitimately has none (same established pattern as list_scenarios/coverage_matrix). A synthetic run_id would create a NEW inconsistency. No amendment.
- **D-arch-resources, command-name part → DISMISS** per playbook 2026-06-26 (Tauri command-name over-reach; "operator-pause prompt" already registered at category grain; explicitly pre-empts "ch8 operator-pause dialog command").
- **D-arch-resources, 2nd-Channel part → ESCALATE** — new sub-case (a Channel transport, not a command name); no exact playbook rule. Recommend DISMISS (the 2nd Channel is the transport realization of the already-registered "operator-pause prompt" surface; the existing "one Channel for live counters" statement remains true). Surface to the user + propose a playbook rule.
