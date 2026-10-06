# Cascade dispositions — 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09

Written from the `cascade.py sweep` listing run AFTER every body of this pass was applied (trail
`cascade-2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09.json`; baseline `0b07b2c`, the pre-CI
commit's parent). Twelve patterns in `cascade-patterns.toml`; every pattern's control fired on the pre-pass
masters. What was searched: the retired model literal and its restatement (`llama-model`, `3b-llm`,
`l4-inference`), the posture contract's pin list and series enumeration (`pin-list`, `pin-a2addb3`, `series-enum`,
`series-pair`, `series-last`), the exception inventory's wording (`report-bodies`), the mask and the root set
(`mask-fn`, `named-roots`), and the canary's storm/line wording (`canary-storms`). What was NOT searched: prompt
version literals and the model's file name, which no master carried before this pass (grep at P1: 0 hits).

## Masters

- `llama-model`, `3b-llm` — 0 master rows after apply: the claim is retired at all four architecture sites.
- `pin-list`, `series-pair` — 0 rows, controls fired: both old wordings are gone from the masters.
- `pin-a2addb3` — security-plan `:121` @3821: amended (`unchanged at a2addb3 and at 5f77859`), the earlier pin
  stays as a true dated step. test-plan `:260` @3402: no change — the 2026-10-01 series' own dated sample.
- `series-enum` — architecture `:184`: amended (the list now ends `2026-10-06`).
- `series-last` — security-plan `:338` @1223 and test-plan `:260` @3327: no change to the 2026-10-01 clauses; each
  line gained the 2026-10-06 clause after them.
- `report-bodies` — security-plan `:338` ×3: the two standing clauses unchanged, the third is this pass's.
- `mask-fn` — security-plan `:121` @1711: amended (the named roots stated). `:338` @772: no change — it names the
  chain, not the roots.
- `named-roots` — security-plan `:121` @1771: this pass's text. architecture `:201` and security-plan `:123`: no
  change — the data dir's platform default, another subject. security-plan `:120`, obs-plan `:459`, `:523`: no
  change — `redact_value`'s token set, a different function this chunk did not touch.
- `canary-storms` — architecture `:70` @3738, security-plan `:121` @1303 and @1563, test-plan `:260` @1507: no
  change — each names the three-storm canary or the `canary:` lines truthfully. obs-plan `:221` @1336 standing,
  @1896 this pass's.
- `l4-inference` — architecture `:70` @2102 and @2195, `:93` @2343, `:200`: this pass's wording. architecture
  `:93` @1353 and obs-plan `:231`: no change — `the L4 model's evidence_refs`, a true claim sharing the token.

## Leaves (re-derived by recompute, not by wording)

- `.claude/docs/tests-summary.md:22` (`pin-a2addb3`, `series-last`, `canary-storms`): **re-derived** — its series
  enumeration ended at the 2026-10-01 series; it now carries the 2026-10-06 series' grades before `v3-09 not met
  by any series`. The `canary:` clause is unchanged and true.
- `.claude/docs/security-summary.md:11` and `.claude/rules/security.md:18` (`mask-fn`): **re-derived** — each
  states the capture's chain; each now names the mask's root set with the two temp roots.
- `.claude/rules/security.md:10` (`mask-fn`): no change — the exception's statement names the chain and carries
  no per-series inventory.
- `.claude/rules/observability.md:29`, `.claude/docs/obs-summary.md:30` (`named-roots`): no change —
  `redact_value`'s roots.
- architecture's leaves (CLAUDE.md `GENERATED:setup:*`, `docs/stack.md`, `docs/conventions.md`,
  `docs/commands.md`, `docs/gotchas.md`): recomputed, no change — none carries the posture contract's series or
  pin list or a model literal (0 leaf rows for `llama-model`, `3b-llm`, `l4-inference`, `pin-list`,
  `series-enum`; CLAUDE.md's overview names the posture contract by role only).
- obs-plan's leaves (`obs-summary.md`, `rules/observability.md`): recomputed, no change — neither carries the
  real-model posture's dated observations.

## Curation homes (never edited by the cascade → P3)

- `.claude/rules/verification-harness.md:47` @329, @421 and `.claude/docs/session-learnings.md:336`
  (`llama-model`, `3b-llm`, `l4-inference`): the 2026-06-27 entries name `Llama-3.2-3B` as Pulse's L4 model.
  Routed to P3 as in-place corrections.
- `.claude/rules/verification-harness.md:67` (`pin-a2addb3`, `series-last`): no change — a true dated statement
  about `a2addb3` and the 2026-10-01 series.
- `.claude/docs/session-learnings.md:337` (`l4-inference`), `:339`, `:345`, `.claude/rules/host-win32.md:130`,
  `.claude/rules/testing.md:80` (`named-roots`): no change — other subjects sharing a token.

## Judgment bases

- `playbook.md`, `drift-base.md`: 0 rows on every pattern.

## Lateral binds

- test-plan §3 ↔ obs-plan §3 and the a11y ↔ obs schema: untouched — no harness, envelope or schema change.
