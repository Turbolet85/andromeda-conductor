# Adaptation Record — 0-pending wrap · 2026-09-14T20:52:00Z

Path: **Setup step 6 no-op** — master-route carries 0 `pending` records (126 complete), and the tree was
dirty only with the documented bookkeeping set (`code-metrics.ndjson`, `friction-log.ndjson`,
`session-handoff.md`) plus two untracked `.andromeda/runs/` dirs, which the dirty-guard counts as clean.
No chunk was wrapped: no P1 report, no P2 fan-out, no light gate, no coverage gate, no master flip, no
flip-compaction.

HEAD at entry: `6861eb6` (pushed — 0 ahead of `origin/build/conductor-0.3.0`).

## Items and dispositions

### 1. Route adaptation — one entry inserted (APPLIED)

**Disposition: applied, trajectory gate satisfied by recorded pre-direction.** The operator relay named
both the entry and its placement, which is route-resolve's one sanctioned substitute for a trajectory HALT.
No dialogue rounds.

- **Where:** first entry under `### Epoch 2 — Scenario assertion hygiene`, directly before
  `Structurally-dead assertion class retired`. Anchor-only — diff is exactly **2 added / 0 removed**
  (the entry and its `   ↓` separator); nothing else in the tail moved and no `[{marker}]`-frozen line
  was touched.
- **Annotation migration:** none required. The working-route carries **0** `PREREQ:` annotations anywhere,
  and the displaced head entry carried no annotation of any class, so the insertion re-pinned nothing.
- **Title:** `Emit scrubber and percentile math under test` — 19 words to the scope hint's end, WHAT-not-HOW,
  no implementation verbs (route register).
- **Freight:** one `EVIDENCE:` annotation (route-resolve's instance-idiom format class), carrying the
  measurement, the full survivor decomposition with coordinates, the floor discipline, and the next-audit
  targets. The targets are labelled predictions, not measurements, per the causal-claim rule.

### 2. Three corrections to the dictated freight (APPLIED)

Each citation in the relay was resolved against the artifact it named before being written into the entry.
Three did not survive. All three are FACTUAL corrections — the WHAT and WHERE of the entry are unchanged —
so they applied under route-resolve's factual→AUTO gradient rather than re-opening the trajectory question.

| # | As dictated | As measured | Basis |
|---|---|---|---|
| 1 | `quantile` has **11** survivors | **14** | `c-mutation-conductor-emit.json` carries 14 quantile rows; the audit's own table at `proposals.md:86-99` lists the same 14. The prose at `:106` (repeated `:112`) says 11, and only 14 closes the "remaining 21 sit in `latency.rs`" arithmetic that same sentence states: 36 + 14 + 6 + 1 = 57. |
| 2 | the 11-line clone pair is in `c-duplication.json` | **absent from that file** | The string `pii_payload_corpus` does not occur anywhere in `c-duplication.json`. Its `top` is the ten longest pairs with a 14 L floor; the pair is 11 L, below the cut. Its actual home is `proposals.md:189`. |
| 3 | `c-dead.json` — "the accessors under top" | **1 of 3 under `top`** | Only `p95_ms` appears among the 20 `top` rows. All three accessors are in `full_candidates` (33 rows) — which is also what the `zero_ref_candidates` 33 → 30 target depends on. |

**Channel for correction 1:** the audit's prose is a run-dir record of that run and no wrap channel edits
it. Per the relay's own instruction the correction rides the **next `code-metrics.ndjson` record's
`corrections[]`**, beside the already-queued commands-field correction. `proposals.md` was NOT edited.

### 3. Verified, unchanged

Everything else in the relay reproduced exactly against the artifacts:

- shard **1/4** · score **45.71** · 115 mutants · 48 caught · **57** survivors · **10** timeouts
  (`completion_state: complete`; the other three shards are `budget-exhausted`, so the unit-wide figure
  is unmeasured — every count in the entry is scoped to shard 1/4 for that reason).
- scrubber trio **36** survivors, coordinates exact: `is_absolute_path_start` :316 (2) ·
  `skip_absolute_path` :320-329 (11) · `skip_line_number_suffix` :340-347 (23).
- accessors at `latency.rs` :42 / :47 / :52, six survivor rows (`-> 0` and `-> 1` for each).
- `dead.zero_ref_candidates` = **33**.
- the clone pair's size, **11 L**, and its identity.
- the ten timeouts carry no per-mutant rows in the twin — only `counts.timeout` — so the entry states
  that the chunk's own run re-measures them, rather than asserting a cause for the absence.

### 4. Context corrections carried into the handoff (APPLIED)

- `6861eb6` is **pushed**: measured `git rev-list --count @{u}..HEAD` = 0 against
  `origin/build/conductor-0.3.0`. The previous handoff's "1 ahead" was a write-time fact, per the
  session-state contract.
- The root `mutants.out/` and `target/mutants-*` dirs the audit surfaced are **gone** — confirmed absent.

### 5. Amendments (NONE)

The no-op path MAY apply an amendment whose subject is a fact this wrap itself measured. None applies:
the three corrections above are about run-dir artifacts and an operator relay, not about any of the seven
`.andromeda/` masters. Per the relay, nothing else this wrap.

## Curation

One candidate survived the five filters and landed as an **additive facet**, amended in place into the
Tier-1 `2026-08-21` entry (`CLAUDE.md` `USER:session-learnings`, 11225 → 12793 chars; file still 135/200
lines, markers parse). The facet the matched entry lacked: a pointer can RESOLVE — real file, real key —
and still not carry the claimed row, because a top-N or prose roll-up silently excluded it. Three
instances this session, items 2.1–2.3 above.

One candidate was dedup-rejected: "verify a dictated citation against the artifact" is already Tier-1
(`2026-08-09`, extended `2026-08-15`). Not logged as `recurrence-despite-learning` — the entry did not
fail here, it is the reason the citations were checked at all.

Tier counts: **T1 0 · T2 0 · T3 0 · extended 1 · rejected 1 (duplicate)**.

## Gates

Not applicable on this path — no chunk, so no plan `## Test Commands` to re-run, no drift fan-out to
close, and no capability to flip. Coverage stands unchanged at **2/11 verified · 9 unclaimed**.
