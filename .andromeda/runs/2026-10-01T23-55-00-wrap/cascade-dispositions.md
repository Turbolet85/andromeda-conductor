# Cascade dispositions — 2026-10-01-per-run-span-identity-in-the-real-model-harness

**The search:** `cascade.py sweep` over `cascade-patterns.toml` (nine patterns), run AFTER every body amendment of this
pass (A1–A4 · S1–S5 · T1–T7). Every pattern's known-positive control fired on the pre-pass masters (baseline `2c97d3b`).
Scope: the seven masters, the curation homes, the judgment bases and the leaves. Patterns and the claim each targets:

- `logskip` / `colliding`: A2's retired collision consequence.
- `schema38`: the stale Pulse citation.
- `spanident` / `sameseed`: the identity-is-seed-pure claim A1 narrows to the unsalted tier.
- `livesuite`: A3/A4's harness-only ownership.
- `readers`: the S-group reader enumerations.
- `connect`: the two-arg signature.
- `quietwin`: T3's class enumeration.

The listing is in `.sweep.txt` (59 lines).

## Rows

| Row | Disposition |
|---|---|
| `logskip`, `colliding`: 0 rows | the retired "logs-and-skips a colliding row" survives nowhere (control `architecture.md:178` fired pre-pass) |
| `verification-harness.md:47` schema38 (curation) | **no change**: a dated curation record of an earlier Pulse revision's measurement, true at its date; a curation home is never cascade-edited |
| `architecture.md:259` spanident ×4 / sameseed (new) | **amended** (A1) |
| `test-plan.md:124`, `:400`, `:548` spanident (new) | **amended** (T1, T6, T3) |
| `test-plan.md:328` spanident (standing) | **no change**: "same-seed re-run yields identical stream SHAPE (insta golden)" stays true; the goldens run the unsalted tier |
| `test-plan.md:336` spanident ×3 (standing, edited) | **amended** (T4). The earlier clause "its span identity replaying d1's" is the d2 RECORD and stays true as history of that series |
| `.claude/rules/testing.md:51` spanident (curation, 2026-06-18) | **routed to P3 curation**: "The seed governs span identity + timing" now holds on the unsalted tier only. It becomes an in-place extension of that entry, never a cascade edit |
| `.claude/docs/tests-summary.md:21` (leaf) | **re-derived** in step 3 (test-plan changed) |
| `architecture.md:3`, `test-plan.md:93` sameseed (standing) | **no change**: both state "same scenario + seed ⇒ same stream SHAPE", which still holds (A1 keeps shape seed-pure) |
| `architecture.md:70` livesuite ×2 @c6682 (standing) | **no change**: window chars 6482-6882 cite the 2026-09-06 live-suite verdict evidence path, a true pointer |
| `architecture.md:171`, `:247` livesuite (edited) | **amended** (A3, A4) |
| `security-plan.md:122` livesuite (new) | **amended** (S3, the new ingest row) |
| `test-plan.md:155` livesuite ×2 (standing) | **no change**: §3's `--live` freeze description is true of the harness arm; the second writer lives in arch's registry |
| `CLAUDE.md:128`, `.claude/rules/security.md:55` livesuite (curation) | **no change**: both concern the 2026-09-06 recursive-delete ruling, true and unrelated to the writer class |
| `.claude/rules/verification-harness.md:19` livesuite ×2 (leaf) | **no change**: the rule body describes the `--live` arm's freeze, true. verification-harness.md is not a test-plan leaf by provenance (its header names the harness), so it is not re-derived; checked by reading |
| `security-plan.md:222` readers (standing) | **amended this pass**: a third restatement of the S1 enumeration ("two operator-gated captures" / "Those three readers") → three readers / four, folded as a dependent of S1 |
| `.claude/docs/security-summary.md:11` readers (leaf) | **re-derived** in step 3 (security-plan changed) |
| `architecture.md:259`, `test-plan.md:336`, `:423` connect (new) | **amended** (A1, T4, T7) |
| `test-plan.md:616` connect (standing) | **no change**: §12 Decisions Log, history; it records the signature as it stood |
| `.claude/rules/testing.md:72` connect (curation) | **no change**: the 2026-09-03 reachability entry's point (a fixed endpoint const gates reach) stays true with the third parameter |
| `architecture.md:182`, `test-plan.md:155`, `:307`, `a11y-plan.md:268` quietwin (standing) | **no change**: each describes its own leg's window (posture contract · `--live` 150 s · the generic ≥120 s + 30 s rule · the sr 170 s), all true; §11 (`:548`) is the class enumeration and is amended (T3) |
| `.claude/rules/verification-harness.md:50`, `:53` quietwin (curation) | **no change**: true |
| `.claude/rules/testing.md:25` quietwin (leaf) | **re-derived**: the rule body's class enumeration ("the `--live` suite's 150 s …, the live `sr` leg's 170 s …") gains the span-landing 180 s member, from test-plan §11 |
| `.claude/rules/a11y.md:39` quietwin (leaf) | **no change**: the sr leg's own 170 s, true |
| `.claude/docs/tests-summary.md:46` quietwin (leaf) | **re-derived** (test-plan changed) |

## Step-3 leaf set
- **architecture** (A1 §Cross-cutting, A2/A3 §Occupied Resources, A4 §Infrastructure tree): CLAUDE.md
  `GENERATED:setup:*` (recomputed structurally) · `.claude/docs/gotchas.md` (§Cross-cutting) · any leaf whose
  provenance header names architecture.md.
- **security-plan:** `.claude/docs/security-summary.md` · `.claude/rules/security.md` body · CLAUDE.md
  `GENERATED:setup:warnings`.
- **test-plan:** `.claude/docs/tests-summary.md` · `.claude/rules/testing.md` body · CLAUDE.md
  `GENERATED:setup:warnings`.
