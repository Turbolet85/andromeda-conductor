# Cascade dispositions — 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09

The step-2 sweep ran after every body of this pass was applied: `cascade.py sweep` over
`cascade-patterns.toml`, baseline `29adafa1`, exit 0. Twelve patterns, each with its control fired on the pre-pass
masters. A thirteenth, `fourth series` (case-insensitive), was refused by the tool (its control never fires in a
master) and run by hand over `CLAUDE.md`, `.claude/rules/*.md`, `.claude/docs/*.md`, `playbook.md` and
`drift-base.md`: 0 hits.

Not looked for: a restatement of the amended claims in none of these words. The masters were also read at each
amended line by offset.

## Rows

| Pattern | Site | Disposition |
|---|---|---|
| `pin-5f77859` | `security-plan.md:121` | amended — the provenance list now ends "at `5f77859` and at `f70be92`"; the old pin stays as the 2026-10-06 series' dated pin |
| `pin-5f77859` | `test-plan.md:260` | no change at this token — it sits in the dated 2026-10-06 record, true as written; the line gained the 2026-10-07 record |
| `pin-5f77859` | `obs-plan.md:221` | no change at this token — the dated 2026-10-06 observation; the line gained the 2026-10-07 one |
| `pin-5f77859` | `verification-harness.md:47`, `:73` (curation) | no change — "since Pulse `5f77859` the shipped L4 model is …" and "a build at `5f77859` … stayed up" are dated facts that still hold; preserve-verbatim |
| `pin-5f77859` | `docs/session-learnings.md:336` (curation) | no change — the same dated "since" fact |
| `pin-5f77859` | `docs/tests-summary.md:22` (leaf) | re-derived — the 2026-10-07 series added beside the 2026-10-06 one |
| `series-0610` | `security-plan.md:121`, `:338` | amended — both now carry the 2026-10-07 series beside it |
| `series-0610` | `test-plan.md:260` ×2 | one standing (the dated record), one new (this pass's "same split … as the 2026-10-06 series") |
| `series-0610` | `obs-plan.md:221` | standing dated record; the line amended after it |
| `series-0610` | `docs/tests-summary.md:22` (leaf) | re-derived, as above |
| `series-list` | `architecture.md:184` | amended — the list ends `2026-10-06, 2026-10-07` |
| `series-list` | `security-plan.md:121` ×2 | this pass's own text — both lists now end "2026-10-06 and 2026-10-07" |
| `sets-nor-reads` | `architecture.md:203` | amended — "A registered handle …" |
| `sets-nor-reads` | `architecture.md:204` | this pass's new bullet (the tool reads the line number's old occupant as standing) |
| `sets-nor-reads` | `playbook.md:137`, `:139`, `:145` (base) | no change — the rule that governs this registration and a sibling rule discussing its clause; neither quotes the retired "The one registered handle" |
| `registered-only` | `architecture.md:184` | no change — the clause is true as written now that both handles are registered |
| `boot-target`, `emit-fn-name` | `test-plan.md:391` | no change — the carrier of the sentence architecture dropped; it states the TARGET, its level and both fields |
| `boot-target`, `emit-fn-name` | `verification-harness.md:60` (curation) | no change — a second carrier; preserve-verbatim |
| `report-body` | `security-plan.md:338` ×2 | one standing (the 2026-10-01 series' clause), one amended (the 2026-10-06 clause now followed by the 2026-10-07 one) |
| `prompt-v25` | `test-plan.md:260` | this pass's own text |
| `prompt-v25` | `obs-plan.md:221` | standing — the dated 2026-10-06 observation |
| `not-met-0610` | `test-plan.md:260` | standing — the 2026-10-06 series' verdict test, still shipped |
| `one-handle` | — | 0 rows; control fired at `architecture.md:203` pre-pass. A statement about this wording only |
| `ns-claim` | — | 0 rows; control fired at `architecture.md:204` pre-pass. The three clauses are gone and nothing else used the phrase |

## Leaves (cascade step 3)

- `architecture.md` → `CLAUDE.md` `GENERATED:setup:*`: recomputed against the amended §Occupied Resources. No block
  carries the env-handle registry, the series list or the per-series pin (`ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`
  and `pulse-real-model-leg-posture`: 0 hits in `CLAUDE.md` and in every `.claude/docs/*.md`); the overview's
  `contracts/` line names the posture contract by role and stays true. No text moves.
- `test-plan.md` → `docs/tests-summary.md:22`: re-derived (above). `rules/testing.md`, `rules/verification-harness.md`
  generated bodies: no series record in either; no text moves.
- `security-plan.md` → `docs/security-summary.md`, `rules/security.md`: neither carries the capture row's series
  lists, the derivation's pin list or the per-series inventory (`series-list`, `report-body`: 0 leaf rows). No text
  moves.
- `obs-plan.md` → `docs/obs-summary.md`, `rules/observability.md`: no dated real-model observation in either
  (`series-0610`, `prompt-v25`: 0 leaf rows there). No text moves.
- Lateral binds: test-plan §3 ↔ obs-plan §3 and a11y ↔ obs schema — no harness, envelope or schema fact moved.
