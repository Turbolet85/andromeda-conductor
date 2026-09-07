# Fan-out results — 2026-09-07-dependency-polish

7 doc-agents, one per spec source. **23 proposals returned**, +5 raised by the orchestrator under
Validate check 5. All applied; 0 rejected; 0 escalations open.

| doc | proposals | verdict |
|---|---|---|
| arch | 11 | applied (1 was detector-severity `escalate` — the E1 qualifier; operator-directed) |
| security-plan | 4 | applied (1 detector-severity `escalate`; operator-directed wording) |
| obs-plan | 1 | applied (detector-severity `escalate`; playbook 28-30 governs) |
| layout-templates | 2 | applied |
| test-plan | 5 | applied |
| design-system | 0 | clean per its detectors; 1 raised by orchestrator (check 5) |
| a11y-plan | 0 | clean — verified no indicatif/knip/otel-proto/tokio/crate-count/RUSTSEC claim exists |

## Validation
1. **Playbook.** The pre-existing version corrections (tokio, Tauri, inquire, crate counts) looked like a
   two-rule collision: 31-33 (dependency the report did not modify -> dismiss) vs 28-30 (reconcile spec value
   to sound impl -> routine). **31-33 does NOT govern**: its qualifying clause "the manifest already satisfies
   the spec's stated FLOOR" fails, because these are false present-tense version STATEMENTS, not floors. Per
   amendment-flow ("a subject match with any qualifier false is NO MATCH"), 28-30 governs -> routine. No
   collision escalation, no discriminator rule needed.
2. **Cross-contradiction.** None. The two layout-templates proposals edit one line in the same direction.
3. **Intent-consistency.** Aligned: the folded CARRY routes these reconciliations to this chunk explicitly
   ("reconcile the docs to the resolved artifacts here"), and the plan's Expected-amendments list (operator-
   reviewed at P5) enumerates them.
4. **Absence needs evidence.** Each non-hit carries its search: a11y enumerated the six claim classes it
   lacks; security-plan reasoned :177 as a dated historical probe record and :185 as imposing no allowlist.
5. **Expected-amendments reconciliation.** 10 plan entries; 5 sites no detector proposed were raised by the
   orchestrator — obs-plan crate count x2 and tokio x2 (obs detectors are scoped to instrumentation/stack/
   redaction), and design-system inquire x1 (its detectors key on tokens/counts/platform, not library
   versions). Both blind classes are structural, not agent error.
6. **Disproved-claims disposition.** 3 report entries, all DISPOSED: #1 -> obs-plan §3 amendment;
   #2 -> architecture [Module Boundaries] amendment; #3 (`profiles` never enabled) corrects this chunk's own
   plan text, no master states it, recorded in the report.

## Detector blind spots worth noting
- The report's grep `8 workspace crates` could not see test-plan §12, which says **"8 seam crates"**. The
  test-plan agent's occurrence sweep found it. A single-site apply would have left the retired count alive
  in the decisions log.
- test-plan §9 baked `the lock held at 564 packages`. De-baked to the PROPERTY (zero added package nodes)
  rather than re-substituted with 562, per the derived-count detectors' own rule.

## Cascade
Derived tier re-derived: `stack.md` x3 (tokio, indicatif, Tauri), `gotchas.md` x1 and `commands.md` x1
(audit figures 564->562, 18->17). **`.claude/rules/security.md:50` carries the same retired figures but sits
inside `## Session Additions` — preserve-verbatim, routed to P3 curation as an in-place extension, NOT
cascade-edited.**
