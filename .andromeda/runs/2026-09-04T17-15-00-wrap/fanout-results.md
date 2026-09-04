# Fan-out results — 2026-09-04-preconditions-probe-reads-path-handles-by-presence

7 Explore doc-agents, one per spec source, one parallel batch. Returns were entity-decoded
(`&lt;`/`&gt;`/`&amp;` → literal — the transport escaped YAML block-scalar indicators `>-` as
`&gt;-` and `<redacted>` as `&lt;redacted&gt;`); `entities=0` on every saved body afterwards.
Per-doc raw twins beside this file for the five docs that carried proposals.

## Verdicts

| doc | proposals | detectors that fired | disposition |
|---|---|---|---|
| arch | 4 | D-arch-resources ×2 · D-platform-claim ×2 | routine → apply |
| security-plan | 3 | D-security-input ×3 (**escalate severity**) | **ESCALATED** |
| design-system | 3 | D-platform-claim ×3 | **ESCALATED** (suspected mis-fire) |
| layout-templates | 3 | D-layout-surface ×2 · D-platform-claim ×1 | routine → apply |
| test-plan | 10 | D-platform-claim ×8 · D-tests-derived-count ×2 | routine → apply |
| obs-plan | 0 | — | clean |
| a11y-plan | 0 | — | clean |

**Total 23.** Clean returns (obs-plan, a11y-plan) are recorded here as their sanctioned audit artifact.

## Validate — the six checks

1. **Playbook.** arch / layout-templates / test-plan (17) → routine: D-arch-resources registering a public
   library API symbol is the `:37` rule; the D-platform-claim and D-layout-surface retirements are the
   `:28` spec-wording→shipped-impl and `:88` operationalizes-a-gate rules; the two D-tests-derived-count
   are the `:127` code-owned-set rule. security-plan (3) → **no exact rule at escalate severity**.
   design-system (3) → **no rule matches**; `:46` (dismiss a pre-existing inconsistency the chunk did not
   introduce) is NO MATCH because its `pattern` names plan↔plan-bind detectors and D-platform-claim is not one.
2. **Cross-contradiction.** None. Two proposals land on arch §Standard Contracts (Liveness equivalent) —
   a D-arch-resources primary retiring "UNSATISFIABLE" and a D-platform-claim dependent retiring the
   "short-circuited before every preflight" restatement — same direction, complementary, applied together.
3. **Intent-consistency.** No divergence: the report matches the working-route entry and every plan
   acceptance criterion (all measured MET, see report §Outcome).
4. **Absence needs evidence.** Every "no further sites" claim cites its search: arch names 3 sites and
   disambiguates 4 unrelated hits (`Commands::declares`, scenario `declare`, the gate's own precondition
   strings, `bootstrap`/`BootstrapState`); obs-plan reports zero occurrences of `precondition` /
   `handles-declared` / `boot` / `ReadyState`; a11y-plan and design-system each name the greps they ran.
   **Orchestrator re-derivation:** all 9 claimed sites independently verified to carry the retired wording
   (arch `:114` UNSATISFIABLE + short-circuited + route-owned, `:184` UNCONDITIONAL, `:188` can-ever-be-met +
   permanently-unmet; security-plan `:114`, `:120`, `:367`; layout-templates `:186`, `:187`, `:281`).
5. **Expected-amendments reconciliation.** The plan listed **9** entries; all 9 reconcile, none under-ran:
   #1→arch 1+4 · #2→arch 3 · #3→arch 2 · #4→security 1+3 (+ a bonus spawn-row duplicate, #2) ·
   #5→test 1,2,3 + the five §6 step-1 lines (4–8) · #6→test 9,10 · #7→layout 1,2 (+ a bonus content-block-2
   duplicate, #3) · #8→obs-plan returned `[]`, consistent with "not carried" · #9→a11y-plan returned `[]`,
   consistent with "not carried".
6. **Disproved-claims disposition.** The report's `Spec claims disproved by measurement` bullet is **none** —
   nothing to dispose. (This chunk retires recorded-defect statements in the OTHER direction; those ride the
   Expected-amendments channel above.)

## Escalations raised to the operator

**E1 — security-plan ×3 (D-security-input, escalate severity).** Retire the recorded defect in favour of the
shipped fix at `:114` (READ SET row), `:120` (spawn-propagation row parenthetical) and `:367`
(Anti-Patterns → Universal, the withdrawn "not a downgrade" bullet). Escalated because the detector is
escalate-severity and no playbook rule covers "the master's own text names this chunk as the fix's owner and
the chunk shipped it".

**E2 — design-system ×3 (D-platform-claim).** Proposes naming the harness as the two-shell SET at `:282`
(Platform), `:318` (Component Pattern 2) and `:328` (Navigation Pattern). Suspected mis-fire: the detector's
invariant is a *capability verdict* ("Linux only", "requires X"), and none of the three sites states one —
`:282`'s actual platform verdict is "Windows / macOS / Linux terminal", which this chunk corroborates rather
than falsifies; the `agent-run.sh` mentions are illustrative entry-point pointers. `agent-run.ps1` has existed
since `2026-06-23-5-command-agent-run-harness`, so the single-script naming is pre-existing and this chunk did
not introduce it.

## Dispositions (operator-resolved 2026-09-04)

- **E1 → APPLY all three + mint a playbook rule.** Applied at security-plan `:114`, `:120`, `:367`; sidecar
  entry appended naming the ratification. New rule appended to `playbook.md` ("a chunk SHIPS the fix its
  master's own body names as route-owned"), carrying an explicit **defer-to-`Boundary widening`-first**
  clause and the discriminator that settled it here.
- **E2 → DISMISS all three as a mis-fire.** No body edit, no sidecar entry. design-system states no
  single-runner capability verdict at `:282` / `:318` / `:328`; its actual platform verdict
  ("Windows / macOS / Linux terminal") is corroborated by this chunk, not falsified.

**Applied: 20** (arch 3 sections / 4 proposals · security-plan 3 · layout-templates 3 · test-plan 10).
**Dismissed: 3** (design-system). **Escalations open: 0.**

## Cascade (single pass, fixed DAG)

Step 2 — cross-master + intra-master citation sweep, patterns derived from ALL amendments after the last
one was authored, keyed on the claim's MECHANISM as well as its tokens (`UNSATISFIABLE`, `cannot exit 0`,
`never declare`, `permanently unmet`, `short-circuit… before every`, `not currently reach`, `route-owned`,
`presence-only`, plus `declares()`, `one boolean … gate`, `boot … skip`, `skipping the preflight`,
`exit 1 in both shells`):

- **One intra-master duplicate caught and fixed** — `layout-templates.md:186`'s leading parenthetical
  ("no longer boot's first act — the probe below leads and short-circuits past it") contradicted the
  amendment 60 characters later on the SAME line. This is exactly the same-line duplicate a verbatim
  single-proposal apply leaves standing.
- Every other master hit dispositioned as unrelated: `architecture.md` `unconditional` ×3 (Blocked-lands,
  data-dir equality, obs sink), `:174` `bootstrap`/`BootstrapState`; `obs-plan.md` `unconditional` ×4 (sink
  paths); `layout-templates.md:189` (the 5-command list); `security-plan.md:114` `declares()` (the chunk's
  own new text); `route-owned` hits in five docs (other route items).
- **Preserve-verbatim homes**: `CLAUDE.md USER:session-learnings` `:128` is an unrelated
  "structurally unsatisfiable" (connection-lifecycle recovery); `verification-harness.md` `## Session
  Additions` `:54` is the sidecar-resolution entry. Neither touched.
- **Judgment bases**: `playbook.md` `:123`/`:129`/`:132` hits are incidental chunk-name mentions, not quotes
  of retired wording. `drift-base.md` clean. No cascade edit to either (the rule minted for E1 went through
  propose→approve→append).
- **`master-route.md:117`** carries the retired claim inside the `sr-findings-remediation` record — immutable
  history, accurate about what THAT chunk measured. Correctly left.

Step 3 — leaves re-derived by provenance: `CLAUDE.md` `GENERATED:setup:warnings` (`:37`, the Preflight-integrity
line) and `.claude/rules/verification-harness.md` (`:18`, the `boot` bullet). `:15`'s phrasing was already
conditional and needed no change. All nine `.claude/docs/*` leaves and the other rule files re-checked clean.

**Post-cascade re-sweep:** 0 hits for every retired phrase across the 7 masters and all leaves, excluding the
two correct survivors above (immutable master-route history, unrelated curated learning).

