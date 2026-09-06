# Fan-out results — 2026-09-06-coverage-completeness-gate

7 doc-agents, one per spec source. 26 proposals returned.

| doc | proposals | verdict |
|---|---|---|
| architecture | 2 (D-arch-resources ×2) | 1 routine · 1 routine with section RE-HOMED by the orchestrator |
| security-plan | 1 (D-security-input, escalate) | **ESCALATE (E1)** |
| design-system | 0 | clean — no hit on any of its 3 detectors |
| layout-templates | 0 | clean — no hit on any of its 3 detectors |
| test-plan | 11 (D-tests-coverage ×4 · D-platform-claim ×7) | 3 routine · 1 REJECTED · 7 **ESCALATE (E2)** |
| obs-plan | 2 (D-obs-instrumentation, 1 primary + 1 dependent) | routine |
| a11y-plan | 10 (D-platform-claim, 1 primary + 9 dependents) | **ESCALATE (E2)** |

## Validation (orchestrator)

**Re-derivation screen.** All proposals rest on facts the report carries. One citation slip noted, not
disqualifying: test-plan's D-platform-claim dependent for the §9 stage-table E2E row cites `ci.yml:190`,
a location the report does not carry — the material fact it rests on (`ci.yml:18` / `:178` both
`windows-latest`) IS report-carried, so the proposal survives on the carried fact and the stray pointer
is discarded.

**1 — Playbook check.**
- arch #1 (rate-term retirement) → **rule `:134`** matches on every clause: `architecture.md:174` records
  the defect, names the corrective route entry ("SURFACED, not authored — the fix … is owned by a
  working-route entry") and cites its measurement ("measured 2026-08-20"), and this chunk shipped the fix.
  Its `Boundary widening` deferral was checked FIRST and does **not** match — nothing new crosses a
  hardened boundary; the gate became stricter (it counts more), admitting no new input class. → **routine**,
  carrying the rule's three obligations (new measurement, state what it supersedes, preserve untouched
  properties).
- obs #1/#2 → **rule `:88`** matches (a chunk operationalizing a spec'd gate for the FIRST time surfaces
  that the spec's own description of that gate names a stale field-list); its note explicitly pre-empts
  re-fire "when the Epoch-10 envelope/A11y CI conformance gates operationalize their own spec'd checks".
  → **routine**.
- tests #1/#2/#3 → same `:88` class, scoped to the operationalizing doc's own sections (§1 Path 6 surface,
  §4 mandated tier, §6 mechanism). → **routine**.
- arch #2 (`.gitattributes`) → no rule matches; not structural or surprising (a new repo-root path a shipped
  gate depends on). → **routine, section RE-HOMED** (below).
- security #1 → **rule `:58` subject-matches but its qualifier is FALSE**: the rule covers a detector that
  "demands re-confirmation of a hardening/validation invariant"; this proposal demands no re-confirmation —
  it records a NEW READER of an already-hardened artifact. Per the amendment-flow's governing-clause rule, a
  qualifier-false rule is NO MATCH → the no-match branch, and the subject is security-sensitive. → **ESCALATE**.
- D-platform-claim ×17 → **rule `:46`** subject-matches (a token-keyed cross-doc detector firing on a site
  the chunk did not introduce) BUT its closing clause governs: "The dismissal never covers a site that STATES
  the retired claim." Several sites do state it in the present tense. Rule `:46` therefore does not dispose of
  the group, and no other rule matches. → **ESCALATE**.

**2 — Cross-contradiction.** None. The three §9 proposals touch distinct rows (a new stage row · Matrix
builds · the E2E row); no two proposals move one section in opposing directions.

**3 — Intent-consistency.** The routine set follows the chunk's intent. The D-platform-claim group is
ORTHOGONAL to it — the report's `ci.yml` measurement was incidental (motivation for the `.gitattributes`
pin), not an a11y/CI finding. That orthogonality is part of why it escalates rather than applies.

**4 — Absence needs evidence.** Satisfied for the derived-count sweeps: tests cites its grep → 0 hits;
design cites its grep → 0 hits; arch's sweep confirms `:174` sole. **One failure:** see the rejection below.

**5 — Expected-amendments reconciliation.** All four plan entries are carried by a proposal:
obs-plan §4 → obs #1/#2 · architecture §Occupied Resources (load envelope) → arch #1 ·
test-plan §1 Path 6 → tests #1 · test-plan §9 → tests #4 (whose conditional resolved, then failed
verification — below). Coverage floor met.

**6 — Disproved-claims disposition.** All four report entries (`architecture.md:174` ×4) are DISPOSED by
arch #1, which retires the dispatch basis, the samples/windows reading, the ~200× figure and the
surfaced-not-authored disposition in one amendment.

## Orchestrator rejections

- **tests #4 — add a "Coverage completeness" row to the §9 stage table. REJECTED.** Its rationale rests on
  a quotation that does not exist: it claims "§9 itself claims 'the stage table above is the complete
  inventory of gates CI enforces'". Verified —
  `awk '/^## 9\. CI Integration/,/^## 10\./' .andromeda/test-plan.md | grep -nE 'complete inventory|inventory of gates'`
  returns no such sentence. Without the completeness claim, absence of a row falsifies nothing. The standing
  precedent points the same way: the immediately prior chunk shipped an equivalent CI gate step
  (Run-journal conformance) and took no stage row — `grep -nE 'conformance|journal_conformance' .andromeda/test-plan.md`
  → 0 hits. Surfaced to the operator as escalation E3 rather than silently dropped.

## Orchestrator section re-homing

- **arch #2 (`.gitattributes`)** was proposed for §Occupied Resources → On-disk artifacts. That section holds
  RUNTIME artifacts Conductor produces or reads (`logs/*.jsonl`, `runs/`, `contracts/*.toml`,
  `coverage-matrix.md`). `.gitattributes` is a VCS control file whose siblings — `Cargo.lock`,
  `rust-toolchain.toml`, `rustfmt.toml` — live in §Infrastructure Patterns → Directory structure
  (`architecture.md:206-208`). Applied there, plus one Build-system sentence for the gate-correctness
  dependency. Per the apply rule, the text is re-derived from the invariant + the report's fact, never
  pasted from the proposal's `change` line.

## Raw twins

`.raw-fanout-arch.md` · `.raw-fanout-security.md` · `.raw-fanout-tests.md` · `.raw-fanout-obs.md` ·
`.raw-fanout-a11y.md` — saved for the five docs that returned proposals. design-system and
layout-templates returned `proposals: []` clean and are recorded by this file alone.
