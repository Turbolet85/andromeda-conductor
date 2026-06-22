# Fan-out results — 2026-06-22-activity-floor-restart-suppression-scenarios

7 doc-agents (Explore), one per spec source. Report = the single source.

| doc | detectors | verdict |
|---|---|---|
| architecture | D-arch-resources · D-arch-decisions | `proposals: []` — no new resource/crate/dep; reuses Absent/Contains; all-Hard per §Probabilistic-Assertion Policy |
| security-plan | D-security-input · D-security-subprocess · D-security-deps | `proposals: []` — 3 TOMLs validated via garde (from_toml_str); spawn untouched; no new dep |
| design-system | D-design-tokens | `proposals: []` — no UI (config only) |
| layout-templates | D-layout-surface | `proposals: []` — no new surface |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness | `proposals: []` — rstest/nextest tier ✓; harness/envelope untouched |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction | **1 proposal** (D-obs-instrumentation, warning) — see below |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | `proposals: []` — no UI; no schema change |

## Escalation (1) — resolved

**D-obs-instrumentation (warning):** proposed editing obs-plan §4 to caveat the restart-suppression
must-trace spans (`bypass_triggered`/`path_type`) as Epoch-8-deferred / not-yet-instrumented.

- **Resolution (user-confirmed): DISMISS.** No obs §4 edit. The chunk adds scenario CONFIG (declarative
  TOML + fixture tests), NOT the runtime operation — the must-trace op is the live scenario under the Epoch-8
  driver, so the detector's "new must-trace operations the chunk adds" precondition is unmet. obs §4 correctly
  describes the TARGET (forward-looking, target-state present tense); the §11 span-name entry lands in the
  wiring epoch.
- **Playbook rule appended** (generalizing the seam-primitive deferred-span dismiss to scenario-config chunks)
  — pre-empts re-fire on the remaining Epoch-7 scenario chunks (fingerprint-storm, severity-lifecycle, …).

## Drift accounting
- Spec-body amendments applied: **0** · escalations resolved: **1** (dismiss + playbook rule) · cascade: none
  (no spec source body changed). **drift = 0.**
