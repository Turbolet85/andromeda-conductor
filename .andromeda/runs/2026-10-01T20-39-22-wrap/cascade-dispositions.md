# Cascade dispositions — 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix

The search: `cascade.py sweep` over `cascade-patterns.toml` (baseline `1fe46a1`, the pre-CI parent), run after every
body of the pass was applied (A1, A2, T1, S1-S3) and before any sidecar entry. Eight patterns, each spanning the
amended claims' tokens AND their phrasings — the latest-series claim (`2026-09-30 series`, `then 2026-09-30`, the
regex `graded (none|0 drives|no drive)`), the Pulse coordinate (`fcc31b2`, `re-pinned`), the elision guarantee
(`fingerprint-shaped`), the residual set (`stated residual`, case-insensitive) and the report-body enumeration
(`read none`). Every control fired on the pre-pass masters. Lines over 2 000 chars were resolved by offset
(`cascade.py window`), never from the grep view.

## Rows

| Row | Disposition |
|---|---|
| `security-plan.md:335` series-0930 @c1135 | amended (S3) — the enumeration now continues to the 2026-10-01 series; the 2026-09-30 clause stays true |
| `test-plan.md:336` series-0930 @c2707 | amended (T1) — history clause, true; the 2026-10-01 series appended after it |
| `architecture.md:182` fcc31b2 (new) | amended (A2) |
| `security-plan.md:121` fcc31b2 @c2807 | amended (S1) — `at fcc31b2, unchanged at a2addb3` |
| `test-plan.md:336` fcc31b2 @c2782 | no change — the 2026-09-30 series' own coordinate, true history |
| `architecture.md:113` residual @c4347 | **amended in this pass (cascade fold, routine)** — §Standard Contracts summarizes security-plan's Data Protection record and named only the 2026-09-22 residual; it now names the second, overseer-ruled, founder ratification pending |
| `security-plan.md:335` residual @c2036 | amended (S2) — the second residual added after this sentence |
| `test-plan.md:336` residual @c2101 | amended (T1 — the same residual restated in tests §6) |
| `.claude/rules/security.md:10` residual + fp-shaped | leaf — re-derived (the qualification and the second residual) |
| `.claude/docs/security-summary.md:24` residual | leaf — re-derived (the second residual) |
| `security-plan.md:121` fp-shaped @c1758 | amended (S1) — the definition stated, all-digit runs pass |
| `security-plan.md:335` fp-shaped @c819 | amended (S2) |
| `.claude/rules/security.md:18` fp-shaped @c1604 | leaf — re-derived |
| `.claude/docs/security-summary.md:11` fp-shaped @c1685 | leaf — re-derived (plus the `incident.skipped` witness lines) |
| `security-plan.md:335` read-none (new) | amended (S3) — the 2026-09-30 clause is true history |
| `architecture.md:85` re-pinned @c1386 | no change — a true claim sharing the token (a sub-tier figure "re-pinned as a literal") |
| `architecture.md:182` re-pinned (new) | amended (A2) |
| `CLAUDE.md:132` re-pinned | curation home — no change: the token names a BLOCKED-ON annotation's re-pin, unrelated |
| `.claude/rules/security.md:53` re-pinned | curation home — no change: the supply-chain deferral's re-pins, unrelated |
| `test-plan.md:336` graded-none @c2792 | no change — the 2026-09-30 series graded none, true history |
| `then-0930` | 0 rows after the pass (control fired pre-pass at `architecture.md:182`) — the retired wording is gone |

## Leaves re-derived (step 3)

- `.claude/docs/tests-summary.md:22` (test-plan's leaf) — was already behind (it named the 2026-09-29 series last);
  re-derived from `test-plan.md:336`: the 2026-09-30 and 2026-10-01 outcomes, `v3-09` not met by any series, the
  `skip_reason` field recorded never graded.
- `.claude/docs/security-summary.md:11`, `:24` and `.claude/rules/security.md:10`, `:18` (security-plan's leaves,
  body text above `## Session Additions` at `:47`).
- Read and left: `CLAUDE.md` `GENERATED:setup:*` (the `contracts/` description carries no series count or Pulse sha),
  `.claude/docs/commands.md:12` and `.claude/rules/verification-harness.md:19` (the `--live real-model` firing order —
  unchanged). No obs, a11y, design or layout leaf restates a moved claim.

## Not a cascade edit
- `.claude/rules/verification-harness.md:67` (2026-09-23, `## Session Additions`: "Pulse logs NO line when its model
  DISMISSES a digest") — a preserve-verbatim curation home: routed to P3 as an in-place time-axis correction.
