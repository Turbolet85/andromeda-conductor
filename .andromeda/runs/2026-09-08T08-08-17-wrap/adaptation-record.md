# Adaptation Record — 0-pending wrap

_Session 127 · 2026-09-08T08:08:17Z · branch `build/conductor-0.2.0`_

## Path taken

**No-op path** (Setup step 6). Master-route carries **0 `pending`** records and the working tree was
**clean** at entry — this session wrapped a `/andromeda-setup-project` re-run (commit `a25a425`), not a
chunk. No chunk marker exists to attribute work to, so P1 (report), P2 (fan-out + reconcile), P4
(code-graph), and P7's gates/flip do not run.

## Items and dispositions

| Item | Disposition |
|---|---|
| Chunk detection | 0 pending → no-op path. No report, no fan-out, no master flip. |
| P2 amendments (`facts THIS wrap produced or measured`) | **NONE.** The no-op path may apply an amendment whose subject is a fact this wrap itself measured. Checked: this session measured `.claude/` materialization and hook behaviour — nothing that any of the seven `.andromeda/` masters states. No body edit, no sidecar entry, no cascade. A drift the fan-out would have found still waits for its chunk wrap, by design. |
| P3 curation | **RAN** (conversation-sourced, not git-sourced). 3 candidates → 2 applied, 1 logged as recurrence. Detail below. |
| P5 route-resolve | **NOT RUN.** No operator route-adaptation request in this session; `working-route.md` untouched, markerless tail unchanged. |
| Code-graph refresh | **NOT FIRED.** This session's delta is source-free (CLAUDE.md + two run-dir artifacts + curation targets; zero `crates/` or `scripts/` changes), so the index still reflects the same source. `tree.db.commit` is re-pointed to the new HEAD after the commit instead, per the step-6 clause — a rebuild here would be a no-op that costs a full index. |
| Verification matrix | Untouched. Coverage stays **28/32 verified · 4 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-27`). No capability claimed or flipped — the coverage gate is P7's and does not run here. |
| Bookkeeping | `state.yaml` (`last_wrap`, `session_count` 126 → 127; `tree_db_refreshed_at` unchanged — no refresh ran), handoff overwrite, this record. Committed so the path exits on a clean tree. |

## Curation detail

**Candidate A — "validate a diagnostic form where the real path passes".**
Filter 1 matched an existing CLAUDE.md entry minted one session earlier, and that entry records a
DEFECT. Per the recurrence clause this is **`recurrence-despite-learning`**, not a duplicate and not a
new entry ("a third entry is not a remedy") → logged to the handoff's Deferred learnings and to the
friction stream as `recall.corpus-recurrence`. Second consecutive wrap carrying one. Caught before it
reached any written claim this time, unlike the prior instance which reached a committed matrix note —
the entry fired late (at diagnosis) rather than preventively (at authoring).

**Candidate B — the bash↔Windows-native boundary changes paths AND environment.**
Filter 1: similarity against `host-win32.md`'s generated body (`/tmp` invisible to native tools), but
naming two facets that body LACKS — MSYS drive paths at every depth, and the PATH of a shell spawned by
a native process. Additive facet against a GENERATED-BODY match, so it lands as a new
`## Session Additions` entry rather than an in-place body amend (a cascade re-derivation would clobber
the latter). Confidence **0.9** (measurement +0.4, repeated pattern +0.3, technical detail +0.2).
Tier 2 → `.claude/rules/host-win32.md` (Tier-2 home registry: host/shell mechanics). **Applied.**

**Candidate C — the `.claude/` leaves are project-tailored renders, not template copies.**
No coverage anywhere. Confidence **0.6 exactly** — the threshold's documented mass point, which rejects
by default. The `no-other-home` conditional +0.2 fired on its stated preconditions: this wrap runs no
P2, so no `.andromeda/` master amendment, no playbook or drift-base rule and no matrix ledger note was
available, and the fact rides no route annotation. → **0.8**. Tier 3 →
`.claude/docs/session-learnings.md`. **Applied.**

**Filters:** 0 duplicates · 0 task-specific · 0 conflicts · 0 deferred by cap (2 of 3 slots used) ·
1 recurrence. No Tier 1 entry, so `CLAUDE.md` stays 135/200.

## Gate status

No light gate (no chunk plan, so no `## Test Commands`). No drift gate (no fan-out ran). No coverage
gate (no capability claimed). No master flip — the route is unchanged and every record stays as it was.
