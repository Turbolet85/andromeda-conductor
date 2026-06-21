# P2 fan-out — drift detection + validation (2026-06-21-expected-outcome-slo-timing-model)

## Doc-agent results (7 detectors over 7 spec sources)
| doc | detectors run | result |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | **1 proposal** (D-arch-resources → §Standard Contracts); D-arch-decisions clean |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | `proposals: []` |
| design-system | D-design-tokens | `proposals: []` |
| layout-templates | D-layout-surface | `proposals: []` |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | **1 proposal** (D-tests-obs-harness); coverage+framework clean |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | `proposals: []` |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` |

## Proposal 1 — arch D-arch-resources (warning)
**Proposed:** register `ExpectedCheck`/`SloOutcome`/`CheckOutcome`/`evaluate_slo`/`evaluate_check`/`compare`/`ComparisonKind`/`SloTier::deadline_ms` + the `ClaimClass` move in arch §Standard Contracts.

**Validation → DISMISS (routine).** The established library-symbol over-reach (playbook rule line 37). arch §Standard Contracts tracks the readiness-gate + run-report-envelope *shapes*, NOT per-crate public API surface. The chunk's real resources (the `conductor-core`/`conductor-verify` crates) are already in §Occupied Resources; the run-report envelope's `verdict ∈ {Pass,Fail,CalibrationRegion}` + `slo_tier`/`latency_ms` are already in §Standard Contracts (untouched by this chunk — `RunRecord` reused, not reshaped). Precedent: 8 conductor-emit chunks registered zero library symbols.

**Recurrence note:** the prior handoff (verdict-assertion-policy-split) predicted this exact recurrence on §Standard Contracts and instructed "broaden the playbook rule's wording if it recurs." → broaden the rule from "§Occupied Resources" to "ANY arch registry section (§Occupied Resources / §Standard Contracts)".

## Proposal 2 — tests D-tests-obs-harness (warning)
**Proposed:** clarify test-plan §3 into two record shapes (Run-report envelope vs self-obs log line) + field order, to match obs-plan §3.

**Validation → DISMISS (not this chunk's drift).** The proposal's own rationale admits "the report does not introduce NEW fields (`latency_ms`, `slo_tier` already exist in both schemas)". The chunk changed no harness command, no envelope shape (`RunRecord` untouched), no log format. The flagged inconsistency is a PRE-EXISTING test-plan↔obs-plan §3 divergence (obs-plan gained the "two record shapes" clarification on 2026-06-15-structured-logging-stack; test-plan didn't) — generalizes playbook line 31 ("report is the single source of what changed this chunk; dismiss what it never touched"). Also: per observability.md rules (2026-06-17), **test-plan §3 OWNS the envelope; obs-plan §3 REPRODUCES it** — so obs's extra self-obs-base-line clarification is obs's own territory, not an obligation on the owner. → carry as a follow-up (visibility), do not amend a binding-contract doc for non-chunk drift.

## Cross-contradiction / intent-consistency
- No cross-contradiction (the two proposals touch different docs).
- Intent-consistency: the report faithfully reflects the chunk intent; the 4 deviations are sanctioned "/implement finalizes" finalizations (no intent divergence). No intent amendment.

## Disposition (pending user confirm on the binding-contract one)
- Proposal 1: dismiss + broaden playbook rule (pre-authorized by prior handoff).
- Proposal 2: dismiss as not-this-chunk's-drift + carry follow-up.
- Spec-body amendments applied: **0** → cascade is a no-op.
