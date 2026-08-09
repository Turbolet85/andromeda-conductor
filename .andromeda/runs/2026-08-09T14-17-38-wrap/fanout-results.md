# Fan-out results — 2026-08-09-current-sut-coverage-classification

7 doc-agents, one per spec source, run in one parallel batch against `report.md`.

| doc | verdict | detectors evaluated |
|---|---|---|
| architecture.md | **1 proposal** (D-arch-decisions, warning) | D-arch-resources clean · D-arch-decisions fired |
| security-plan.md | `proposals: []` | D-security-input · D-security-subprocess · D-security-deps — all clean |
| design-system.md | `proposals: []` | D-design-tokens clean |
| layout-templates.md | **1 proposal** (D-layout-surface, warning) | D-layout-surface fired |
| test-plan.md | **1 proposal** (D-tests-coverage, warning) | D-tests-coverage fired · D-tests-framework clean · D-tests-obs-harness clean |
| obs-plan.md | `proposals: []` | D-obs-instrumentation · D-obs-stack · D-obs-redaction — all clean |
| a11y-plan.md | `proposals: []` | D-a11y-surface · D-a11y-obs-schema — all clean |

**Totals:** 3 proposals · 4 docs clean · 0 detector returned malformed YAML (no stripping needed; raw == stripped for all 7, so no `.raw-fanout-*` twins warranted).

## Validation (main)

| # | proposal | playbook | outcome |
|---|---|---|---|
| 1 | **D-arch-decisions** — §Established Decisions [Accepted Capability Set]'s "re-aiming … is a manifest edit with no Rust change" contradicted by the landed state | routine under `playbook:28-30` (spec-wording → sound-impl reconciliation; the decision's invariant — accepted set is DATA — fully preserved, only its consequence-wording moved) | **APPLIED** + §Occupied Resources echo in lockstep |
| 2 | **D-layout-surface** — coverage-matrix row anatomy documents no Mode cell | no clean rule match; main uneasy (the cell is PRE-EXISTING, shipped 2026-06-27; the doc enumerated no mode values, so nothing this chunk did went stale — but the file was already open and the operator's `:222` ruling favours fixing adjacent staleness in-pass) | **ESCALATED → operator chose "Apply it now"** over main's recommended carry-to-next-chunk; applied, accepting that the follow-on *Out-of-scope classification treatment* chunk will revisit the same block |
| 3 | **D-tests-coverage** — §1 Critical Path 6 reads "all 60 P-IDs" over three modes | routine (stale-derived-fact reconciliation; the chunk made both false) | **APPLIED**; the detector correctly left the `coverage-matrix.md` artifact reference alone → routed to route-resolve as a CARRY |

**Cross-contradiction:** none — the three proposals touch disjoint sections in the same direction (de-hardcode toward the manifest's accepted set).
**Intent-consistency:** the report matches the chunk's working-route entry + plan acceptance criteria; the deviations it records (spec bodies edited at /implement) are the operator-ruled instance repair this wrap completes, not an unjustified divergence.
**Absence-needs-evidence:** the "no other master cites the amended wording" claim is backed by the W12 grep below, not inference.

## W12 — cross-master citation grep (anchored on the DIFF, not the proposal set)

Grepped the six other masters for the OLD wording of every amended passage: `60-P-ID` · `all 60 capabilities` · `60-row wall` · `P-001..P-060` · `60/60` · `all 60 P-IDs` · the three-mode enumeration.

- **`test-plan.md:78`** — hit: "all 60 P-IDs" + `auto / drive+observe / static-only`. **Amended** (proposal 3).
- **`test-plan.md:39`** — hit: the **Source:** clause quotes arch Project Intent as *"60 claimed capabilities P-001..P-060"*, wording arch has not carried since 2026-08-08 — the citation no longer resolved. **Amended** (found by this grep, not by any detector). Its adjacent Creator-Brief quotation is left verbatim: it cites frozen `input.md:160`, which still says exactly that.
- **security-plan · design-system · layout-templates · obs-plan · a11y-plan** — no hits.
- Non-master hits deliberately untouched: `input.md:160` (frozen Creator Brief — the quotation's source of truth), `master-route.md:45` (immutable record), the `*-amendments.md` sidecars (historical log).

## Sidecar entries (T5) — authored regardless of the proposal set

Three spec bodies changed during /implement (`architecture.md`, `design-system.md`, `layout-templates.md`). An empty proposal set is a verdict about whether the **docs** are at current truth, not about whether **history** was recorded — T5 keys on "did this body change this chunk", which it did. Entries written to all three sidecars, plus `test-plan-amendments.md` for this wrap's own amendments. One entry per doc.

## Cascade

- `architecture.md` → **CLAUDE.md `GENERATED:setup:warnings`** re-derived (carried the superseded "manifest edit, not a code change"); `.claude/docs/stack.md` checked — clean.
- `design-system.md` + `layout-templates.md` → `.claude/docs/design-summary.md` checked — its coverage-matrix line is count-free and above the Mode cell's altitude, no re-derivation needed; `.claude/rules/frontend.md`'s only hit is inside `## Session Additions` (preserved verbatim, and accurate as history).
- `test-plan.md` → `.claude/docs/tests-summary.md` already manifest-relative; `.claude/rules/testing.md`'s `60`s are `--fail-under-lines` coverage percentages (false positives of the same class as `render.rs:42`), its other hit is `## Session Additions`.

**Drift = 0 on exit:** 3 proposals {2 applied routine, 1 escalated-and-resolved-then-applied} + 2 grep-found citation repairs applied + 4 sidecars written + cascade closed.
