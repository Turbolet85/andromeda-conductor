# Fan-out results — 2026-06-22-error-baseline-spike-latency-regression-scenarios wrap

7 doc-agents, one per spec source. Report = single source.

| doc | result |
|---|---|
| arch | **1 proposal** — D-arch-decisions (warning): amend §Probabilistic-Assertion Policy to explicitly classify sample-count floors (`CountAtLeast`) as hard claims + baseline-match ±% as calibration. |
| security-plan | `proposals: []` |
| design-system | `proposals: []` |
| layout-templates | `proposals: []` |
| test-plan | `proposals: []` |
| obs-plan | `proposals: []` (agent mused about deferred-instrumentation doc gap → concluded no drift) |
| a11y-plan | `proposals: []` |

## The arch proposal (verbatim)
```yaml
proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "Established Decisions — [Probabilistic-Assertion Policy]"
    change: "Deterministic claims (hard signals, baseline math, suppression/bypass logic, lifecycle timing, sample-count floors) are hard pass/fail; model-interpretive claims (..., baseline-match ±% tolerance) are calibration-region...; CountAtLeast and Contains comparisons are hard-classified; ±tolerance baseline-match floats to Epoch-8 calibration."
    sidecar: "P-009..P-012: sample-count floors (CountAtLeast) and candidate presence (Contains) reclassified as hard claims; baseline-match tolerance deferred to Epoch-8."
    rationale: "Report: all four P-009..P-012 checks class=Hard; sample-count floor is the declarable hard claim. arch §Probabilistic-Assertion Policy treats sample-count only implicitly."
```

## Orchestrator validation
- **Playbook check:** no EXACT rule matches (the existing over-reach rules cover deps / library-symbols / scenario-files / deferred-spans / plan-binds — none about assertion-policy classification of a comparison kind). Editing §Probabilistic-Assertion Policy is a **locked-decision / structural** edit → per the gradient, "no rule + structural → escalate."
- **Substance (orchestrator assessment): likely MISFIRE.** The chunk introduced NO new decision — `CountAtLeast` + `class=Hard` + the evaluator's unmet→calibration override are ALL pre-existing (built in verdict-assertion-policy-split + expected-outcome-slo-timing-model). The architecture ALREADY: (a) names "sample-count floors (50-sample latency / 10-span error-rate)" in §Timing-Tolerance Model, and (b) classifies "baseline math" as Hard in §Probabilistic-Assertion Policy. The agent read "all class=Hard" as "the floor hard-FAILS when unmet" — but `slo.rs:79` routes an UNMET `CountAtLeast` to calibration-region, so the architecture's "floors route to calibration instead of hard-failing" already holds. No contradiction; the proposed wording would also conflate class (declared) vs kind (CountAtLeast/Contains), muddying the spec.
- **Recurrence:** statistical-anomaly + activity-floor chunks (ch4 P-013..P-016, ch5 fingerprint-storm) reuse `CountAtLeast`/threshold floors → this WILL re-fire. A playbook rule is warranted.
- **Decision:** ESCALATE to user (recommend dismiss + add playbook rule).
