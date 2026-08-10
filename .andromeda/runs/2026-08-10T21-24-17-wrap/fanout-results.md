# Fan-out results — 2026-08-10-pulse-run-contract

7 Explore doc-agents, one per spec source, one parallel batch. All 7 returned clean YAML; none needed
stripping, so no `.raw-fanout-*.md` twins were warranted and this file is the sanctioned audit artifact.

## Per-doc verdicts

| doc | proposals | verdict |
|---|---|---|
| arch | 3 (D-arch-resources ×3, warning) | all applied |
| security-plan | 2 (D-security-subprocess, D-security-input — both escalate-severity) | both applied as routine (playbook) |
| design-system | 0 | **correct** — verified by grep, see below |
| layout-templates | 1 (D-layout-derived-count, warning) | applied |
| test-plan | 1 (D-tests-obs-harness, warning) | applied |
| obs-plan | 0 | **correct** — verified by grep, see below |
| a11y-plan | 0 | correct — no interactive UI element, no violation-schema change |

**7 amendments applied across 4 docs · 0 escalations · 0 open.**

## Validation (the 5 named checks)

1. **Playbook.** `contracts/` boundary row → the 2026-08-10 rule matches verbatim (routine-apply); its own trap
   note was honoured — the security-plan BODY was grepped rather than the sidecar trusted, and both sibling
   manifests were already present, so no widening beyond the new artifact was owed. The precondition-count
   reconciliations (arch + security) matched no rule but are pre-declared, report-substantiated count moves with
   a direct precedent one chunk back (the same bullets moved 3 → 4); not structural, not surprising → routine.
2. **Cross-contradiction.** None — no two proposals touch the same section in opposing directions.
3. **Intent-consistency.** The report matches the chunk's working-route entry and plan acceptance criteria; all
   five deviations carry justifications, and the one intent divergence (scope Term C) was already classified
   intent-incomplete and amended at phase P5.
4. **Absence needs evidence.** design-system and obs-plan returned bare `proposals: []` while the report named
   both as stating the `(N unbacked)` literal — so the absence was verified rather than accepted:
   - `grep -n unbacked .andromeda/design-system.md` → **no match**; the doc never states it.
   - `.andromeda/obs-plan.md:364` states it as a **derived** qualifier ("read from `conductor_core::UNBACKED_AUTO`",
     "neither is ever a literal") — which the detector's own rule calls correct and NOT a hit.
   - `.andromeda/layout-templates.md:178` **did** bake `(10 unbacked)` → the one real hit, applied.
   Both empty returns were therefore right. The over-claim was in the report's own Expected list, not in the
   detectors.
5. **Expected-amendments reconciliation.** The plan listed 5 expected entries; entries 1-4 were each proposed by
   a detector and applied. Entry 5 named three docs and only one bakes the literal, so it landed as one
   amendment rather than three — a narrowing established by evidence (check 4), not an under-run. One amendment
   arrived BEYOND the expected list (test-plan §3 `boot` timeout), which the floor permits.

## Cascade

Bodies edited (8 edits / 7 logical amendments), then sidecars appended (arch 3 · security 2 · layouts 1 ·
tests 1) after re-reading each edited section, so no entry claims more than its body shows.

Cross-master citation sweep for the amended wording: the only hits were historical `*-amendments.md` sidecar
entries, which record the past and never feed the cascade. No spec body cited another's stale wording.

Leaf re-derivation (6 sites): `CLAUDE.md` §Key directories + §Critical Warnings preflight bullet ·
`.claude/rules/security.md` ×2 (manifest enumeration + never-downgrade bullet) ·
`.claude/docs/security-summary.md` (threat-model line + a new committed-manifests line) ·
`.claude/rules/verification-harness.md` (`boot` timeout). `USER:*` and `## Session Additions` preserved
verbatim; CLAUDE.md 128/200 lines, 20 markers balanced.

Closure verified: zero residual `FOUR named preconditions` and zero residual `(10 unbacked)` across every spec
body and leaf.
