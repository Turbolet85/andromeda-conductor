# Session Handoff

**Last Updated:** 2026-09-18T10:14Z
**Branch:** `build/conductor-0.3.0` · **0 ahead** of `origin/build/conductor-0.3.0` as read at this wrap's
Setup; this wrap's own commit and push land after this line is written, so a next session measuring 0 ahead
with HEAD == upstream reads the push as landed.
**Status:** clean — drift 0, 2 amendments applied, 1 escalation raised and resolved with the operator.
**Last Commit:** `feat(2026-09-18-real-model-leg-posture-and-grading-rule)` — see below.

## Position

- **Done: `2026-09-18-real-model-leg-posture-and-grading-rule`.** The posture half of Epoch 4's pair:
  `contracts/pulse-real-model-leg-posture.md` fixes, before any drive, the launch posture, the emission
  profile, the grading rule and the per-leg quiet window for a live leg with deterministic L4 OFF.
- **Next:** `Interpretation proven live` — `working-route.md:44`, head of the markerless tail, **Epoch 4**.
  It carries no PREREQ; it gained a CONTEXT block naming the posture artifact and the three clauses that
  shape the leg's design. `v3-09` is its to claim.
- **Coverage 6/11 verified · 5 unclaimed** — unchanged. This chunk claimed nothing by design.

## Work done

One new file, 181 lines, zero `.rs` delta, zero dependency delta: a **reader-less** `contracts/` member (the
second, after the P-025 contract). The chunk's substance came from P3 research tracing the hypothesis
observable to its writer in the SUT: **a single-storm real-model leg has no hypothesis observable at all** —
`retrieve_report` renders `## Hypotheses` only when the incident's resolution-summary text parses as an
`L4Output`, and neither writer fires on first incident creation. Two consequences also fixed in the posture:
the assertion needs no new machinery (the report markdown already reaches the composed observation text, so
`Contains` expresses it), and the leg cannot be graded on any SLO tier (the ladder caps at 90 000 ms against
~110 s formation), so it lands declare-only plus a harvest-tier hard assertion.

**The PREREQ is discharged.** `cargo clippy --workspace --all-targets -- -D warnings` ran green, closing a
two-deep deferral chain one re-pin short of the age trigger. Its log shows it fully cached
(`Finished dev profile … in 0.36s`), so the green re-asserts over unchanged sources rather than being a fresh
analysis — worth knowing, since the chain is now CLOSED and the tail carries no PREREQ.

## Drift resolved — 2 amendments, 0 open

`architecture.md` 2, both from one detector's primary + dependent pair; the other six masters returned clean.

- **§Occupied Resources** gained a registry row for the new reader-less member, recording its regime and its
  **per-clause** provenance (Pulse coordinates transcribed at HEAD `83d4060`; the ~110 s figure a carried
  measurement to confirm at the first drive).
- **§Infrastructure Patterns** — the directory tree's "(the one member no Rust code reads)" retired for "the
  two members", the uniqueness claim the new member falsifies. Cascade re-derived two leaves: `CLAUDE.md:14`
  and `.claude/docs/conventions.md:9` — the second is the case the flow's own note warns about, since the
  cascade table does not name it for the sections amended here.

**One escalation, resolved with the operator:** no playbook rule governed a NEW *reader-less* `contracts/`
member (both nearest rules require a runtime-parsed artifact, a qualifier this class fails). Approved: apply
both amendments and mint the rule — **playbook 48 → 49**.

## Curation

T1 0 · **T2 1** · T3 1. Filters: 2 rejected.

- **T2 is a CORRECTION** (cap-exempt, per curation-guide §Corrections): `verification-harness.md`'s claim that
  the deterministic-L4 fixture pins `evidence_refs` to `[]` was **true when written and stale the next day** —
  Pulse commit `efabe8e` (2026-08-17) replaced the empty array with a populated `det-*` triple. Corrected in
  place with the dated tag and the basis.
- **T3**: the baseline for a host-tool or advisory reading is the NEWEST recorded one, found by a bare sweep.
- Filtered: the sweep-shape recurrence (duplicate of an existing Tier-1 clause); the cross-repo class, which
  the operator scoped to the report finding plus the T2 correction.

## Notes

- **Class finding, stated not mechanised (operator's call).** A Conductor rule file cites another
  repository's `file:line`, and nothing in either pipeline re-checks that citation when the other repository
  moves. This one stood stale a month and surfaced only because a chunk happened to read the cited file. The
  asymmetry that makes it bite: the cascade sweeps the seven masters and re-derives their leaves — so
  `architecture.md`, `test-plan.md` and `obs-plan.md` all already carried the corrected reading — while a
  **rule file** carrying the same kind of citation is swept by nothing.
- **The code-graph gap from P3 was a STALE INDEX, not a missing symbol.** `execute_scenario` was absent from
  the plane a query-time regenerate had produced (which reported `db_state: fresh`); after this wrap's full
  refresh it measures **9 call sites / 7 callers**. A query-time regenerate is not equivalent to a full
  refresh. CLAUDE.md's 2026-09-02 figure of 7/5 matches neither and remains stale.
- **Three sweep-shape near-misses in one session**, each caught before it reached a conclusion: a `tail -8`
  clip, an unmeasured site count asserted in the report, and a trailing `.{60}` regex context anchor that
  suppressed a line-final hit the doc-agent's own sweep found. Filtered from curation as a duplicate of the
  standing Tier-1 clause, but worth knowing the family recurs under new mechanisms.
- **Still open from prior sessions:** the n=1 deferred escalation class · the audit-debt chunk's discarded
  wrap `gates` evolve record · the `quantile` 14-vs-11 correction for `code-metrics.ndjson` · `v3-08` BLOCKED
  on a Pulse release.
- **Last failed command:** none.
