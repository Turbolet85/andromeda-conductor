# Fan-out results — 2026-08-16-fingerprint-storm-live-proof

7 Explore doc-agents, one per spec source, one parallel batch. **7 proposals** (4 primary + 3 `dependent-of`)
from 4 docs; 3 clean.

| doc | detectors | verdict |
|---|---|---|
| arch | 2 | **2 proposals** — D-arch-decisions primary (§RBDP narrowing) + 1 `dependent-of` (sidecar sweep). D-arch-resources clean, evidenced (no new port/socket/env-var/crate; nine members unchanged; `serde_json` already in §Stack). |
| security-plan | 3 | clean — `proposals: []` |
| design-system | 2 | clean, evidenced (greps: no `unbacked` literal, tier occurrences name the closed SET only, no test tally; near-miss `12×/30s` checked and cleared — counts unchanged this chunk) |
| layout-templates | 2 | **1 proposal** — D-layout-derived-count (line 178 bakes `(9 unbacked)`). D-layout-surface clean (no user-facing surface added). |
| test-plan | 4 | **2 proposals** — D-tests-coverage §6 primary + §1 `dependent-of`. Framework / obs-harness / derived-count clean, evidenced. |
| obs-plan | 3 | **2 proposals** — D-obs-instrumentation §4 primary + §1 `dependent-of`. D-obs-stack + D-obs-redaction clean, affirmatively evidenced. |
| a11y-plan | 2 | clean, evidenced (no interactive element; violation schema unchanged; tier SET quoted, not a per-scenario value) |

## Validation (6 checks)

1. **Playbook** — all routine except the arch §RBDP class, which rule@97 (`escalate`) governs as a
   ratify-once step; pre-directed by the operator, confirmed in flow.
2. **Cross-contradiction** — none; each proposal edits a distinct doc/section.
3. **Intent-consistency** — report matches the chunk's working-route entry + plan acceptance; all 5
   deviations carry justifications.
4. **Absence needs evidence** — every clean return cited its greps except security-plan (bare `[]`);
   accepted, since the report affirmatively covers its three invariants (no spawn/data-dir change; the one
   dependency is a dev-dep edge with zero new `[[package]]`; the new config member is `garde✓`).
   Orchestrator re-verified the two load-bearing claims independently: `layout-templates:178` is the sole
   baked `(9 unbacked)` site, and NO master states a per-scenario `<20s` for these scenarios (`obs-plan:131`
   names the field, not a value).
5. **Expected-amendments reconciliation** — the plan's 3 (arch §RBDP · test-plan §6 · obs-plan §4) are all
   covered by proposals. No under-run.
6. **Disproved-claims disposition** — all 5 disposed: #1 → arch §RBDP + sidecar supersession; #2 → test-plan
   §6/§1 + obs-plan §4/§1; #3 → test-plan §6; **#4 (tier `<20s`) → NO doc states it** (verified by grep
   across all seven masters — code-only change, nothing to amend); #5 → already a matrix `notes`
   PREMISE-CORRECTION.

## Escalations (3, all resolved with the operator in flow)

1. **Sidecar sweep method** — the arch `dependent-of` proposed EDITING the prior chunk's sidecar entry,
   colliding with the append-only contract. Resolved: the prior entry stays as written; this chunk's NEW
   entry records the supersession. Operator-approved.
2. **Playbook trim (founder-ratified 2026-08-16)** — the three fault spans leave the deferred-span rule's
   list, marked SPENT; the any-seam-primitive class and the still-deferred `hold.wait_resolve` /
   `db.insert_run` / `report.generate` stay. Operator-approved as drafted.
3. **Recurring derived literal** — `(9 unbacked)` is the THIRD occurrence of the same amendment
   (11→10, 10→9, 9→8), each prior fix substituting a literal that re-staled, on a line whose own prose says
   "every number manifest-derived (never a literal)". Resolved: de-literalize to `(N unbacked)` + a new
   playbook rule for the class. Operator-approved.

## Applied

**6 spec-body edits across 4 masters** + **4 sidecar entries**:
- `architecture.md` §Established Decisions [Read-Back Dependency Posture] — leading-segment narrowing
- `layout-templates.md` §Surface: cli Primary screens — de-literalized sample
- `test-plan.md` §6 + §1 (dependent pair) — fingerprint-storm verification signal re-based
- `obs-plan.md` §4 + §1 (dependent pair) — `fingerprints` present-may-be-empty

**Playbook:** trim applied (31 → 32 rules: deferred-span rule trimmed + SPENT clause; new derived-count
de-literalization rule).

**Cascade:** grep of all seven masters + the three preserve-verbatim curation homes for the retired wording
returned **zero** hits in masters. One leaf re-derived: `.claude/docs/tests-summary.md:20`
("fingerprints populated" → the measured shape). `obs-summary.md` names the field without the claim — no
change. CLAUDE.md / stack.md / rules bodies carry no amended fact. **One hit in a preserve-verbatim home** —
`.claude/rules/testing.md` §Session Additions 2026-06-22, whose prescribed `Contains`/`Absent` mechanism this
chunk measured ungradeable — routed to P3 curation as an in-place extension, NOT edited by the cascade.

**Drift = 0 on exit.**
