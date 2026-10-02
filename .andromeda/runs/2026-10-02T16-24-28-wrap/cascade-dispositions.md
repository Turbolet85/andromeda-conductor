# Cascade dispositions — 2026-10-02-captured-fingerprint-values-elided

The pass amended three masters: security-plan (`:121` the ingest row; `:336` Data Protection, two edits),
architecture (`:113`, the Corpus-access parenthetical) and test-plan (`:336`, the real-model leg).

## The search
- `cascade.py sweep` over `cascade-patterns.toml`, run after all five body edits were applied. It covered the
  seven masters, `.andromeda/registries/**`, the three curation homes, the two judgment bases and the leaf bodies.
  The pattern set keys on the claim's own words and on its mechanism:
  - `ratif-pending` — the status;
  - `keeps-prefix` — "keeps its … prefix", the frozen-file mechanism;
  - `by-definition` — "keeps … by definition" / "by that tool's definition", the elider passing all-digit runs;
  - `counts-exactly` — the harvest's exact residual count;
  - `d3-residual`;
  - `all-digit-pass` — "all-digit run is not one / passes / prefix";
  - `frozen-never` — "frozen evidence is never edited";
  - `stated-resid`.

  Every pattern's control fired on the pre-pass masters at `31f9d92`. A pattern for the retired test name
  `the_elided_copy_is_the_frozen_capture` was dropped: its control never fired, because no master ever named it.
  P1 measured that the name sits in past-chunk records only.
- A second, wider leaf sweep (session scratchpad `leafsweep.py`) covered `CLAUDE.md`, `.claude/docs/*` and
  `.claude/rules/*`, split into body and curation zones. Its pattern was
  `elide_fingerprints|fingerprint_hex|frozen 2026-09-22|rm-capture-d3|ratification pend|all-digit|un-elided|elided copy`.
  It was run before and after the leaf rewrite.

## Every row the sweep printed

| Row | Disposition |
|---|---|
| `architecture.md:53` by-definition (standing) | no change — "operator-confirmed by definition", an unrelated claim sharing the token |
| `architecture.md:113` d3-residual (standing, edited) · all-digit-pass (new) · stated-resid (new) | amended — this pass's replacement, which states both residuals fixed 2026-10-02; the window at `@c4281-4477` read |
| `test-plan.md:336` d3-residual (new) · all-digit-pass (new) | amended — this pass's replacement (`@c2332`, `@c2345`), which states d3 re-elided under the keyed rule |
| `security-plan.md:336` frozen-never (standing, edited) · stated-resid (new) | amended — this pass's replacement, which names the recorded exception and the "formerly stated residuals" |
| `.claude/rules/security.md:10` ratif-pending · keeps-prefix · d3-residual · all-digit-pass · stated-resid (leaf) | re-derived from security-plan `:336` and `:121` |
| `.claude/rules/security.md:18` all-digit-pass (leaf) | re-derived from security-plan `:121` — the elider's two-rule definition |
| `.claude/docs/security-summary.md:11` all-digit-pass (leaf) | re-derived from security-plan `:121` |
| `.claude/docs/security-summary.md:24` ratif-pending · d3-residual · stated-resid (leaf) | re-derived from security-plan `:336` (the Data-classification row) |

`counts-exactly` returned 0 rows with its control fired. That is a statement about this pattern only. Curation
homes: 0 rows on every pattern. Judgment bases: 0 rows.

## Leaves re-derived (step 3)
- **security-plan leaves:**
  - `.claude/rules/security.md`: two passages on `:10`, one on `:18`, with `## Session Additions` untouched;
  - `.claude/docs/security-summary.md`: `:11` and `:24`.

  The wider leaf sweep found no other leaf body carrying the claim. After the rewrite, every remaining hit is the
  re-derived text.
- **architecture leaves:** the `CLAUDE.md` `GENERATED:setup:*` blocks and `docs/commands.md` (§Standard Contracts)
  carry no Corpus-access residual (leaf sweep: 0 hits), so they were recomputed with no change.
- **test-plan leaves:** `tests-summary.md`, `rules/testing.md` and `rules/verification-harness.md` show 0 hits. No
  change.
- **Lateral binds:** the test-plan §3 ↔ obs-plan §3 harness and the a11y ↔ obs schema are both untouched by this pass.
- **The handoff** states the residuals and is rewritten whole at P6.
