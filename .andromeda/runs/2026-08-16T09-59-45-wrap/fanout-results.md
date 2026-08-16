# Fan-out results — 2026-08-16-canary-fingerprint-derivation-aligned

7 Explore doc-agents, one per spec source, one parallel batch. **16 proposals · 14 applied · 2 rejected ·
2 escalations resolved with the operator · drift = 0 on exit.**

| doc | proposals | verdict |
|---|---|---|
| arch | 5 | all applied (1 escalated→ratified, 4 routine incl. 3 `dependent-of`) |
| security-plan | 5 | all applied (1 escalated→ratified, 4 routine incl. 2 `dependent-of`) |
| test-plan | 3 | all applied (1 corrected in scope at validation) |
| obs-plan | 3 | 1 applied · **2 rejected** (mis-scoped — would have introduced drift) |
| design-system | 0 | clean (`proposals: []`, greps cited) |
| layout-templates | 0 | clean (`proposals: []`, greps cited) |
| a11y-plan | 0 | clean (`proposals: []`, greps cited) |

## Escalations (resolved with the operator before any apply)

1. **arch §Established Decisions [Read-Back Dependency Posture]** — locked-decision reversal, escalation
   mandated by playbook line 97. Operator ratified "apply, dependents in lockstep". The section's canary
   mechanism was contradicted by the live SUT's field provenance; invariants preserved, only the carrier
   changed. The four consequential restatements then applied as routine lockstep.
2. **security-plan §Dependency Security** — no rule covered admitting a NEW dependency under a red audit
   (the 2026-08-09 bounded-deferral rule presumes a zero-delta chunk). Operator chose "record the admission
   rule + extend the playbook". Both applied; the new playbook rule is appended.

## Rejected proposals (recorded, not silently dropped)

- **obs `D-obs-instrumentation` → §6 Boundary-call wrappers** and its `dependent-of` twin **→ §1
  Instrumentation-scope table**: both proposed removing `retrieve_telemetry_slice` from Conductor's MCP
  client/must-log tool lists on the grounds that "the read-back path no longer calls it". **False** — only
  `assert_canary` stopped calling it; per-check read-back extraction (`conductor-verify/src/extract.rs`)
  still does, so Conductor's client tool set is unchanged at four and the pinned contract manifest is
  untouched. Applying these would have created new drift. The `dependent-of` group was rejected atomically
  per the amendment-flow rule.
  **Root cause is a report gap, not agent error:** the report's Symbols bullet said "`assert_canary` — no
  longer calls `retrieve_telemetry_slice`" without stating that another caller remains. The report was
  corrected at validation so the fact is carried for future detectors.

## Validation checks (all six run)

1. **Playbook** — line 97 matched the arch reversal (escalate); the consequential mentions took its
   "reconcile in lockstep as routine" clause. No rule covered the dependency-under-red-audit case → escalated.
2. **Cross-contradiction** — none. arch §Standard Contracts' two proposals touch different paragraphs
   (prose vs the FIVE-precondition enumeration); security §Anti-Patterns and arch agree in direction.
3. **Intent-consistency** — the report diverges from the frozen working-route entry (which predicted the
   derivation alignment would open the precondition). Divergence is JUSTIFIED by measurement ⇒ intent was
   incomplete; `scope.md` was already amended at the P3 premise closure, and the master desc is rewritten at
   the flip. No unjustified divergence.
4. **Absence needs evidence** — the arch agent's "P-017 appears nowhere in architecture.md" was
   independently re-derived at wrap: `grep -n 'P-017'` across **all seven masters** returns zero hits; the
   only `insensitiv` hits are `.andromeda/input.md:58` (asserting LINE-insensitivity, still true) and a
   `master-route.md` chunk description — both outside the amendment surface. This **falsified the report's
   own claim** that architecture.md states the clause; the report was corrected.
5. **Expected-amendments reconciliation** — the plan's three entries (arch [Read-Back Dependency Posture],
   arch §Standard Contracts Readiness gate, arch §Stack hashing row) were each matched by a proposal. No
   under-run.
6. **Disproved-claims disposition** — all three report entries disposed:
   - *canary fidelity carrier* → 11 amendments across arch / security / test / obs.
   - *P-017 clause (c) path-insensitivity* → **no spec-master states it** (check 4). Source doc comments were
     corrected in implement; `scenarios/fingerprint-storm.toml`'s header prose is source/config and is
     carried at route-resolve (the `scheduler.rs` precedent); the curation-home hits route to P3.
   - *warm-up purpose* → already recorded disproved in arch; the source twin was corrected in implement.

## Cascade (single pass, fixed DAG)

- **Cross-master citation grep** for every retired phrase (`fingerprint_refs` · `canary fingerprint not
  found` · `computed to match` · `FNV-1a` · `16 hex` · `retrieve_telemetry_slice` ·
  `fingerprints_matched_count`) across all seven masters: the retired claims were concentrated in
  `architecture.md` and are now cleared. `retrieve_telemetry_slice` mentions in security/test/obs are
  CORRECT (the tool is still called) and were left standing. `obs-plan:644` ("empty canary ⇒ blocked") is
  still true and needed no amendment — a genuine absence-of-drift finding.
- **Leaf re-derivation:** `architecture.md` → `CLAUDE.md GENERATED:setup:warnings` (FIVE-precondition list)
  + `.claude/docs/stack.md` (blake3 row); `security-plan.md` → `.claude/rules/security.md`
  (FIVE-precondition list) + `.claude/docs/security-summary.md` (dependency-admission condition).
  `test-plan` / `obs-plan` leaves carried no stale wording (checked before their `## Session Additions`).
- **Preserve-verbatim homes** (never cascade-edited): hits for the retired wording exist in
  `CLAUDE.md USER:session-learnings`, `.claude/rules/verification-harness.md`, `.claude/rules/testing.md`
  and `.claude/docs/session-learnings.md`. All routed to P3 curation as in-place extensions.
