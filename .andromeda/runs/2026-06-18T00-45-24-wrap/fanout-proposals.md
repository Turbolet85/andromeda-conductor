# Fan-out drift verdicts — 2026-06-18-exception-events-fingerprint-control

7 Explore doc-agents, one per spec source. Report = single source.

| doc | detectors run | verdict |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | **1 proposal** (D-arch-decisions, warning) |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | `proposals: []` |
| design-system | D-design-tokens | `proposals: []` |
| layout-templates | D-layout-surface | `proposals: []` |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `proposals: []` |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | `proposals: []` |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` |

## The one proposal (arch / D-arch-decisions · warning)
- **section:** §Infrastructure Patterns directory tree (+ CLAUDE.md §Modules distillation)
- **change:** clarify fingerprint ownership — per-exception fingerprint PRIMITIVE in `conductor-emit` (co-located with exception events); fingerprint-STORM fault stays `conductor-faults` (Epoch-7).
- **rationale:** report §Decisions — user ratified the crate-seam in /andromeda-phase; arch §Modules / dir-tree previously attributed all "fingerprint generation" to faults without distinguishing the primitive from the storm composition.

## Validation (main)
- **Playbook:** matches the 2026-06-15 rule "reconciles a spec's illustrative mechanism/wording to the sound shipped impl, invariant preserved" → **routine**. Reinforced: the crate-seam was already user-ratified in the phase (AskUserQuestion), so this is documentation alignment to a settled decision, not a re-litigation.
- **Cross-contradiction:** none (single proposal).
- **Intent-consistency:** consistent — the plan explicitly chose emit; the proposal documents that choice. Justified.
- **Verdict:** routine apply. No escalation, no HALT.

## Apply targets
- BODY: `architecture.md` §Infrastructure Patterns dir-tree lines (conductor-emit + conductor-faults comments).
- SIDECAR: `architecture-amendments.md` (append).
- CASCADE: CLAUDE.md `GENERATED:setup:modules` lines 23-24 (re-derive to match).
