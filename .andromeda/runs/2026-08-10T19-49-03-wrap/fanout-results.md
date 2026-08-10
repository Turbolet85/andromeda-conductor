# Fan-out results — 2026-08-10-workspace-key-divergence-probe

7 Explore doc-agents, one per spec source, in one parallel batch. Report is the single source of change.

| Doc | Verdict | Detectors evaluated | Outcome |
|---|---|---|---|
| architecture.md | **1 proposal** | D-arch-resources (fired) · D-arch-decisions (clean) | applied (routine) |
| security-plan.md | **1 proposal** | D-security-input (clean) · D-security-subprocess (fired) · D-security-deps (clean) | applied (routine) |
| design-system.md | clean | D-design-tokens · D-design-derived-count | `proposals: []` |
| layout-templates.md | clean | D-layout-surface · D-layout-derived-count | `proposals: []` |
| test-plan.md | clean | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count | `proposals: []` |
| obs-plan.md | clean | D-obs-instrumentation · D-obs-stack · D-obs-redaction | `proposals: []` |
| a11y-plan.md | clean | D-a11y-surface · D-a11y-obs-schema | `proposals: []` |

Raw twins kept for the two docs that carried proposals (both returns needed stripping):
`.raw-fanout-arch.md` · `.raw-fanout-security-plan.md`. The five clean returns are recorded here.

## Validation (5 checks)

1. **Playbook** — both routine.
   - arch: playbook:37 (D-arch-resources library-symbol over-reach) does **not** apply — that rule states
     §Standard Contracts *does* track the readiness-gate SHAPE, which is exactly what moved.
   - security-plan: playbook:28 (spec wording → shipped impl, invariant preserved) matches; playbook:58
     (escalate-detector demanding re-confirmation of *unchanged* consumed infra) does **not** apply, because
     this chunk genuinely changed the precondition set — the detector's precondition is met.
2. **Cross-contradiction** — none. The two proposals state the same fact in their own doc's voice.
3. **Intent-consistency** — the report matches the working-route entry + plan acceptance. Its one divergence
   (probe inconclusive on the key axis) is justified and recorded in `two-launch-verdict.md`.
4. **Absence needs evidence** — all five clean returns cite their searches (line numbers, and explicit
   "no 494/495/67/68 anywhere in the file" style greps). Accepted.
5. **Expected-amendments reconciliation** — the plan's list has exactly ONE entry (arch §Standard Contracts,
   fourth precondition). It **was** proposed by D-arch-resources. Coverage floor met, nothing under-ran.

**Escalations: 0.**

## Applied

- **architecture.md** §Standard Contracts — named-precondition set 3 → 4; the fourth's two candidate causes
  named; every precondition string host-path-free. Sidecar appended.
- **security-plan.md** §Security Anti-Patterns → Universal — the never-downgrade ban restated over four named
  preconditions, never the generic corpus string. **Also removed the stale `keychain read-while-write`
  cause**, replaced with an explicit statement that it does not apply. This second half is a *cascade step 2*
  fix: the bullet cites `(Standard Contracts: Readiness gate)` verbatim, so it is a cross-master citation of
  the amended passage, and its keychain clause had been stale since the 2026-06-27 arch corpus-access
  correction. Sidecar appended.

## Cascade

- **Step 2 — lateral binds + cross-master citations.** test-plan §3 ↔ obs-plan §3 and a11y ↔ obs schema:
  untouched this chunk, no action. Grepped all masters for the amended wording: `obs-plan:637`,
  `security-plan:117`, `security-plan:308`, `test-plan:241` state "empty canary ⇒ `blocked`" — still **true**
  (the outcome is unchanged; only the precondition string moved), so **checked and correctly unchanged**.
  The one genuine hit (`security-plan:358`) was fixed above.
- **Step 3 — leaves re-derived** (3 edited, 2 checked-and-correctly-unchanged):
  - `CLAUDE.md` §Critical Warnings (leaf of architecture.md) — preflight-integrity warning now names four.
  - `.claude/docs/stack.md` (leaf of architecture.md) — **checked, correctly unchanged** (no preflight prose).
  - `.claude/rules/security.md` (leaf of security-plan) — four preconditions; keychain mode marked N/A.
  - `.claude/docs/security-summary.md` (leaf of security-plan) — zero-incident corpus names the key agreement.
  - `.claude/docs/gotchas.md` — **checked, correctly unchanged**: its "Surface as `Blocked` with the named
    precondition" line is about the version-mismatch gotcha and stays accurate.
