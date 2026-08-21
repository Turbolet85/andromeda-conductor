# Fan-out results — 2026-08-21-per-check-latency-measurement

7 Explore doc-agents, one per spec source, each reading ONLY the chunk report + its own doc + its
scoped drift-base detectors. **19 proposals returned; 1 raised by the orchestrator at validate check 5;
20 applied; 0 escalations; 0 open.**

## Per-doc verdicts

| doc | detectors | proposals | verdict |
|---|---|---|---|
| arch | 2 | **9** | applied (5 × D-arch-resources, 4 × D-arch-decisions) |
| security-plan | 3 | **4** | applied (1 primary + 3 `dependent-of`, one atomic group) |
| design-system | 2 | 0 | clean |
| layout-templates | 2 | **1** | applied (D-layout-surface) |
| test-plan | 4 | **5** | applied (1 primary + 3 `dependent-of` + 1 independent) |
| obs-plan | 3 | 0 | clean — **1 raised by orchestrator** (check 5) |
| a11y-plan | 2 | 0 | clean |

## Clean verdicts — reasoning, independently spot-checked

- **design-system.** All four new surfaces carry `tokens n/a`; nothing rendered introduces a token or
  color-only state. Moved counts (`runs.db` 2→3, nextest 693→719) appear nowhere in the doc.
  *Orchestrator re-grep:* `run_envelope|runs.db|693|719` → 0 hits. Confirmed.
- **obs-plan.** Dependencies bullet is empty (zero package admission), so D-obs-stack is clean; the new
  persistence surface reuses the shipped `db.insert_run` span name, so the bounded span-name set is
  unmoved; all three new artifact writes carry `redacted✓` with assertions.
- **a11y-plan.** The envelope a11y reproduces verbatim at **two** sites is byte-unchanged (11 fields), so
  neither reproduction went stale; no interactive UI element was added.
  *Orchestrator re-grep:* `latency_ms` → exactly 2 sites (lines 105, 242); `run_check|CheckRecord|budget_ms|check_index` → 0 hits. Confirmed.

## Validation (the 6 named checks)

1. **Playbook.** All 19 → `routine`. The arch resource/config proposals are genuine new-resource
   registrations (what §Occupied Resources exists for); every other proposal falls under the standing rule
   *"an amendment reconciles a spec's illustrative mechanism or wording to the sound implementation actually
   shipped, where the report demonstrates the invariant still holds — only the mechanism differs."*
   Considered and rejected: the escalate rule for *"a chunk REVERSES a locked arch §Established Decision"* —
   [Timing-Tolerance Model] is EXTENDED (a budget beneath the tier), not reversed; the tier still governs
   when no budget is declared, and the design was operator-selected at phase P4.
2. **Cross-contradiction.** None. The overlapping pairs are complementary, not opposing
   (arch §Occupied Resources registers the table / §Data model conventions qualifies its nullability;
   security §Input Validation and arch §Config conventions register `budget_ms` in different docs, consistently;
   tests §3 and arch §Standard Contracts describe the same 9-key shape).
3. **Intent-consistency.** Aligned — the report matches the working-route entry and the plan's acceptance,
   with the read-back-arity premise correction already ratified at P3/P4.
4. **Absence needs evidence.** ONE claim disproved: security's proposal 4 asserted §Threat Model Summary is
   a verbatim mirror of `threat-assessment.md` and must be applied in lockstep. **That file does not exist
   in this repo** (`ls` → absent), so the lockstep clause was DROPPED from the applied text and the reason
   recorded in the sidecar. Every other absence claim was independently re-grepped (above, and the four
   test-plan sites at lines 120/159/181/187, the four arch sites at 29/48/50/51/159).
5. **Expected-amendments reconciliation** (the plan's list is the coverage floor — 6 entries):
   - `test-plan` §3 Log format → **matched** (tests primary).
   - `obs-plan` §3/§6 → detector returned clean; **orchestrator raised it** (routine). obs §3 REPRODUCES the
     journal format test-plan §3 owns, and D-tests-obs-harness is an explicitly two-sided bind — the owner's
     amendment must land on the reproduction too, or the pair drifts one-sided.
   - `a11y-plan` §1+§3 → **legitimately not needed, reason recorded.** The predicted amendment assumed the
     envelope would change; the design deliberately put the per-check grain in its own record/table instead,
     so both verbatim reproductions remain accurate. Not forced.
   - `architecture.md` §Occupied Resources + §Standard Contracts → **matched** (arch #1, #3).
   - `security-plan` §Input Validation → **matched** (security primary).
   - `layout-templates` / `design-system` ("only if…") → layouts **matched**; design-system legitimately
     nothing (no token, palette or ANSI entry moved).
6. **Disproved-claims disposition.** The report's single entry (v2-19's two mechanism descriptors, authored
   at P5 before the code was read) is disposed twice over: the specs now state the shipped mechanism
   (arch [Validation Library] + [Scenario Config Format]; security ×4), and the matrix text itself is
   refined with a PREMISE-CORRECTION at the P7 coverage gate per the operator directive.

## Applied set (20)

**architecture.md ×9** — §Occupied Resources `runs.db` three tables · [ORM] roster + per-check-index label
moved to `run_check` · §Standard Contracts `CheckRecord` registered as the second shared shape ·
§Config conventions `budget_ms` registered · §Data model conventions nullability qualified per table ·
[Validation Library] the third route + its sibling boundary · §Stack Validation row narrowed ·
[Scenario Config Format] three-way load-error mapping · [Timing-Tolerance Model] tier as ceiling.

**security-plan.md ×4** (atomic `dependent-of` group) — §Input Validation row (what + how) ·
§Anti-Patterns → Input ban · §Bootstrap phases · §Threat Model Summary (mirror clause dropped).

**test-plan.md ×5** — §3 Log format Agent-parsing (primary) · §3 Status endpoint shape · §2 Agent-runnable
invariants · §6 E2E verification signal · §3 cleanup body + verification.

**obs-plan.md ×1** (orchestrator-raised) — §3 two-record-shapes gains the second report-seam line shape.

**layout-templates.md ×1** — §cli block 2 per-check detail region.

## Cascade

- **Step 2 — retired-wording sweep** across all 7 masters + the three preserve-verbatim curation homes +
  the two judgment bases: the only surviving hits for `two tables` / `the per-check index` /
  `CoreError::Validation` on a garde failure` / `cross-field invariants for scenario config` are the
  AMENDED text itself. Zero stale duplicates.
- **Step 3 — leaf re-derivation** (5 leaves; `USER:*` and `## Session Additions` untouched):
  `.claude/docs/stack.md` (Validation line) · `.claude/rules/security.md` (garde bullet + the new
  load-path arm) · `.claude/docs/obs-summary.md` (Log format) · `.claude/rules/observability.md`
  (record shapes) · `.claude/rules/verification-harness.md` (cleanup body + status envelope contract).
  **CLAUDE.md needed none** — recomputed from the amended arch it yields the same distillation at that
  altitude (it carries no table count, no cross-field mechanism claim, no error-variant mapping). 129/200.

## Carried forward (not a doc amendment)

The cleanup contract now names `run_check` and `run_envelope`, but `scripts/agent-run.{sh,ps1}` still runs
`DELETE FROM runs` alone — the `run_check` orphan is this chunk's, the `run_envelope` orphan is
pre-existing (since 2026-08-09-sut-load-envelope). Wrap writes no code; carried at route-resolve to the
entry that owns run-artifact integrity.
