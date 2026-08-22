# Code Audit — Conductor · Epoch 4 — Lifecycle & delegated timing · 2026-08-22T13:31:54Z
mode **trend** · HEAD `b8f3332` · baseline `8cba57d` (Epoch 3) · span **1** · 9 commits
ancestry OK · **no trend-break** — all six baseline tools re-probed at identical versions
(jscpd 5.0.16 · tokei 14.0.0 · rust-code-analysis 0.0.25 · cargo-machete 0.9.2 ·
cargo-mutants 27.1.0 · cargo-llvm-cov 0.8.5)

**Headline: zero code-health thresholds fire.** Every tracked metric is flat or improving —
cycles 0, complexity ceiling flat, duplication down, coverage up, both mutation scores up
double digits with **zero genuinely new survivors** in either unit. The proposals below are
therefore all **measurement-integrity** findings about the trend line itself, not about the code.

---

## Trend integrity — the classing recipes, reproduced before any delta was read

Per the run directive, each metric's baseline population was re-derived at `8cba57d` (the tree
extracted with `git archive` into the run dir) and checked against the stored record **before**
any movement was interpreted.

| Metric | Recipe recovered? | Proof |
|---|---|---|
| **dead code** | **yes — exact** | `raw 745 → exclude a `tests` path SEGMENT in the SYMBOL path ∪ the FILE path → 36 → subtract entry points (3) → 33`. Replayed against the baseline tree: **31**, matching the record exactly. **Scope is workspace-wide — one query over all symbols, never per-crate.** The other FP families are **named, not subtracted** (this is what the record's own top-10 proves: all ten entries are trait-impl-dispatch shapes, which a subtracting recipe could not have listed). |
| **complexity** | **yes — exact** | all four percentiles, `over_ceiling` 6, `max` = shape_is_realizable 31, and the same top-10. Recovering it required reading the record's row order: rows are `[fn, file, **cognitive**, cyclomatic]` sorted by cognitive — verified at the raw metric (`dispatch` is cyclomatic 47 / cognitive 16, stored as `16.0, 47.0`). |
| **duplication** | **yes — exact on clones** | population is `jscpd --format rust` (Rust only): **79 clones** at the baseline tree, matching the record. (Its `pct` 3.68 does not reproduce — my recomputation is 3.88 — so pct alone rests on an unrecovered denominator; the clone count and the split are sound.) |
| **sizes / totals** | **no — unrecoverable** | top-10, `file_max` (898) and `over_800` (2) reproduce **byte-identically** under a Rust-only population, but the recorded `p50 88 / p90 347 / 16543 loc / 118 files` need a population ~14 files larger than the 104 Rust files that exist at that commit. **No combination of tokei's languages yields 16543/118.** |

**One population asymmetry found and fixed inside this run.** rust-code-analysis walked
`crates/conductor-tauri/ui/node_modules` and counted 2 vendored `.rs` files
(`@crabnebula/tauri-driver`) that the git-archive baseline tree **cannot** contain — so the two
sides of the diff were not the same population. Excluded; `rs_files_scanned` is now 111 at HEAD
and 104 at baseline, matching tokei on both sides. Effect on the reported numbers was nil
(functions 1924 → 1921; percentiles, `over_ceiling` and `max` unchanged) — but it would have
grown silently.

---

## Proposals

### M1 — `trend-integrity` · sizes + totals — population unrecoverable from the ledger
**Movement:** not judgeable against the stored values. Recorded baseline `16543 loc / 118 files /
p50 88 / p90 347`; the same commit measured under a declared Rust-only population is
`15651 / 104 / p50 101 / p90 359`; HEAD is `17518 / 111 / p50 104 / p90 360`.

**Evidence:** all 4 discriminating facts —

| fact | value |
|---|---|
| tracked `.rs` at `8cba57d` vs HEAD (git) | 104 → 111 (**7 added, 0 deleted**, all test files) |
| recorded baseline `files` | **118** — 14 more than any Rust population at that commit |
| exhaustive language-combination scan over tokei's 13 detected languages, both `crates/` and repo root | **no combination yields 16543 loc / 118 files**; nearest is Rust+TSX = 16502/116 |
| what *does* reproduce exactly | `sizes.top` (all 10 rows, same order, same counts), `file_max` 898, `over_800` 2 |

**Suspected shape:** the recorded `top`/`max`/`over_800` are Rust-only while the recorded
percentiles and totals come from a wider set — i.e. one metric block computed over two
populations, or a scope no longer inferable. Taking the record at face value would draw
`118 → 111 files`, a **decrease**, at a boundary where git proves 7 net additions.

**Proposal:** the general cause is that the ledger schema carries `tool_versions` but no
per-metric **population** or **command**, while this skill is forbidden to read prior audit run
dirs — so the only place a recipe could have been recovered from was unavailable by construction.
This run's record adds two fields, `recipes` and `commands`, carrying the firing form and the
declared population per metric. The direction for the founder is whether to promote those two
fields into the pinned schema in `audit-pass.md`, so run #3 never has to reverse-engineer a
population. Until then, judge `sizes`/`totals` against this record's recomputed baseline, not
against the stored one.

### M2 — `trend-integrity` · mutation — the baseline's own score and survivor list disagree
**Movement:** n/a — an internal inconsistency in the stored record.

**Evidence:** both units —

| unit | recorded score | recorded survivors | implied caught (`caught/(caught+missed)`) | reconciles? |
|---|---|---|---|---|
| conductor-run | 58.82 | 28 | 40 → 40/68 = 58.82 | **yes, exactly** |
| conductor-verify | 72.97 | 28 | 75.6 → **non-integer** | **no** |

**Suspected shape:** conductor-verify's stored score and stored survivor count cannot both be
right under the score formula the other unit satisfies exactly; one of the two was computed over
a different denominator (24 distinct survivor *sites* vs 28 survivor *mutations* is one candidate,
but neither yields 72.97 either).

**Proposal:** record the score's numerator and denominator, not just the percentage — this run's
record carries `counts{mutants, caught, missed, timeout, unviable}` per unit plus an explicit
`score_formula`, which makes the arithmetic checkable at the next boundary. The founder's call is
whether the baseline's verify score (72.97) should stand as the comparison point; this run treats
it as stated, so the reported +16.02 pt gain carries that caveat.

### M3 — `trend-integrity` · mutation — the schema has no slot for timeouts, so a carried defect was invisible
**Movement:** 2 timeouts at boundary #1, **2 timeouts at boundary #2** — unchanged, and absent
from the stored record at both.

**Evidence:** both, complete —

| unit | file:line | mutation |
|---|---|---|
| conductor-verify | `crates/conductor-verify/src/jsonrpc.rs:49:30` | `replace != with == in JsonRpcSession::request` |
| conductor-verify | `crates/conductor-verify/src/jsonrpc.rs:70:9` | `replace JsonRpcSession::write_message -> Result<(), VerifyError> with Ok(())` |

**Suspected shape:** `mutation` in the pinned schema is `{scoped_units, scores, survivors}` — a
timeout is neither caught nor missed, so it lands nowhere and leaves no trace. These two are the
ownerless follow-up carried since boundary #1; they were re-measured here only because the tier
was re-run, not because the ledger remembered them.

**Proposal:** add `timeout` and `unviable` to the schema's `mutation` block (this record already
carries both, plus the full timeout list). Separately: these two remain **ownerless** — neither
killed nor classified accepted-deliberate against a cited rule, which is the disposition
`.claude/rules/testing.md:19` requires of every named survivor. Assigning them an owner (or an
accepted-deliberate classification) is a founder call, not this run's.

---

## Informational

- **complexity `max` entrant** — `serve_stub` (`crates/conductor-verify/tests/common/mod.rs`)
  cognitive 27 → **34**, taking the max slot from `shape_is_realizable` (31, unchanged). It is a
  **test stub**, and it is also the top B2 hotspot (score 34). `over_ceiling` stayed at 6 because
  `is_host_path_token` (cognitive 15) sits just under the ceiling at both boundaries.
- **sizes** — `conductor-run/src/lib.rs` 898 → **1012** and `conductor-core/src/scenario.rs`
  850 → **1001**; `over_800` is flat at 2, but both members now exceed 1000. `conductor-report/src/db.rs`
  370 → 504. New top-10 entrant: `conductor-run/tests/severity_harvest.rs` (468).
- **duplication** — pct **3.88 → 3.66** (down) on the recomputed basis; clones 79 → 84. The +5 is
  **entirely test-side**: both_test 35 → 40 pairs, both_src 37 → **36**, mixed 7 → 8. New top-10
  clone pair: `conductor-run/tests/canary_obs_witness.rs ↔ canary_wire.rs` (21 lines).
- **mutation, conductor-verify** — 72.97 → **88.99** (+16.02). 135 mutants: 97 caught, 12 missed,
  24 unviable, 2 timeouts. **12 survivor sites killed, 0 genuinely new.** Remaining survivor
  composition: **8 of 12 are in `src/bin/stub_pulse_mcp.rs`**, a test-support binary.
- **mutation, conductor-run** — 58.82 → **70.59** (+11.77). 92 mutants: 48 caught, 20 missed,
  24 unviable, 0 timeouts. **8 genuinely killed, 0 genuinely new** (`now_ms` ×3, `now_unix_nanos`
  ×3, `severity_rank` ×2). Of the 20 survivors, **6 are accepted-deliberate** (below), leaving 14
  unaccepted.
- **A naive survivor diff over-reports.** Matching on `(path, mutation)` with line numbers
  included reported **5 false new survivors** in conductor-run — the same mutations shifted by
  edits above them (`409→419` ×2, `532→575` ×2, `570→613`). All survivor diffs here are computed
  on `(file, mutation text)` with `line:col` ignored.
- **dead code** — candidates 31 → **33** (+2, threshold +5). Both additions are
  `PauseResolver::kind()` trait impls (`conductor-tauri/src/pause.rs`, `conductor-cli/src/pause.rs`)
  added by the operator-pause chunk — members of the *named* trait-impl-dispatch FP family, so the
  growth is classification noise, not dead code.
- **unused deps 2 → 1** — `conductor-cli: tracing` resolved; `conductor-emit: conductor-core`
  remains (present at both boundaries).
- **coverage 92.26 → 92.62** (+0.36 pt), 746 tests passing.
- **churn 11.81%**, 3 files churned across 6 source-touching commits — informational by rule
  (the ledger held 1 record when this ran). The three: `conductor-run/src/lib.rs` (3 commits,
  177 adds), `conductor-core/src/scenario.rs` (3 commits, 300 adds), `conductor-core/src/lib.rs`
  (2 commits, 3 adds).
- **hotspots — first table** (baseline skipped it `no-baseline`), so all entries are entrants:
  `conductor-verify/tests/common/mod.rs` 34 · `conductor-run/src/lib.rs` 15 ·
  `conductor-core/src/load_envelope.rs` 13 · `conductor-core/src/scenario.rs` 12.
- **graph** — cycles **0 → 0**, cross-unit edges **16 → 16**, fan-out set identical
  (`conductor-run` 6, `conductor-cli` 4, `conductor-tauri` 2, four crates at 1). Fan-in grew across
  the board (`conductor-core crate/` 104 → 118), consistent with 7 new test files.

### Carried items made visible (per the run directive)

- **Accepted-deliberate class — 6 survivors, conductor-run, ratified 2026-08-21** under
  `.claude/rules/testing.md:19` (*"every named survivor ends killed OR classified
  accepted-deliberate against a cited rule — the score is a consequence, never the acceptance"*).
  All 6 still survive at HEAD, as expected:

  | file:line | mutation |
  |---|---|
  | `crates/conductor-run/src/lib.rs:206:5` | `replace observe_run_contract -> RunContractStatus with Default::default()` |
  | `crates/conductor-run/src/lib.rs:218:5` | `replace declares -> bool with true` |
  | `crates/conductor-run/src/lib.rs:218:5` | `replace declares -> bool with false` |
  | `crates/conductor-run/src/lib.rs:220:11` | `replace == with != in declares` |
  | `crates/conductor-run/src/lib.rs:220:21` | `replace \|\| with && in declares` |
  | `crates/conductor-run/src/lib.rs:220:26` | `replace == with != in declares` |

  Note the family is **6, not 5**: five `declares` mutations plus the `:206:5`
  `observe_run_contract` wrapper — which is exactly the row the baseline audit's own §B1
  undercount omitted ("all 28" stated, 27 enumerated). The ratified count and the measured family
  now agree.

- **Ownerless follow-up 1 — the 2 mutation timeouts:** unchanged, both in `jsonrpc.rs`. See **M3**.
- **Ownerless follow-up 2 — the command-field omission:** confirmed real. The baseline recorded the
  default `cargo mutants` form while the run required `--test-tool=nextest`, so replaying the
  recorded triple aborts rather than reproducing the scores. This run fired
  `cargo mutants -p {unit} --test-tool=nextest --jobs 2 --output {run_dir}/mutants-{unit}` and the
  record now carries a `commands` field with the firing form for **every** metric, not just
  mutation.
- **Known residue — `.andromeda/runs/_wrap_tmp`:** 7 tracked files, introduced at `01c6dac`
  (2026-08-15, the canary-spans chunk). Confirmed committed and pre-existing. **Surfaced, not
  cleaned** — outside this run's write surface and explicitly out of scope per the directive.
- **Already resolved since the last boundary:** the root `mutants.out/` + `mutants.out.old/`
  droppings are now gitignored (`.gitignore:46-47`) and pre-date this run (2026-08-20/21), so they
  cannot ride a future `git add -A`.

---

## Below threshold — no action

- `new-cycle` — cycles 0 at both boundaries; nothing to fire on.
- `duplication-up` — needs ≥ +0.5 pt **and** ≥ +15% relative; measured **−0.22 pt** (down).
- `complexity-creep` — needs `over_ceiling` +3 **and** +25%; measured **+0** (flat at 6).
- `dead-growth` — needs +5; measured **+2**, and both are named-FP-family members.
- `coverage-drop` — needs −2 pt; measured **+0.36 pt**.
- `mutation-drop` — needs a unit ≤ its last score −10 pt; measured **+11.77** and **+16.02**.
- `monotonic` — requires worsening at both of the last two diffs; only one diff exists (this is
  the first). Re-evaluable from run #3.
- `span > 1` — not applicable; span is 1 (Epoch 3 → Epoch 4, consecutive).
- `trend-break` — not applicable; every tool version matches the baseline exactly.
- Complexity percentiles — `cyclomatic_p50/p90` 1.0/4.0 and `cognitive_p50/p90` 0.0/1.0, all four
  flat at both boundaries.
- `conductor-emit: conductor-core` unused dep — present at both boundaries, unchanged, no growth.

---

## Skips

| metric | reason | recipe |
|---|---|---|
| complexity-web | tool-missing | `pip install lizard` (invoke as `python -m lizard` if the shim is off PATH) |
| dead-code-web | tool-missing | `npm i -D knip` |
| mutation-web | tool-missing | `npm i -D @stryker-mutator/core` |
| mutation — conductor-core, -cli, -tauri, -report, -timeline | declined | 5 of the 7 touched units, deliberately out of scope this run: only conductor-run and conductor-verify carry a baseline score, so only they can produce a `mutation-drop` judgment. The other five would be new absolute data with no trend value until run #3. |

**Coverage bound, stated rather than implied:** the mutation tier measured 2 of 9 units. Nothing
here is evidence about the other seven.

---

*Ledger: 2 records (`.andromeda/code-metrics.ndjson`), 0 unparseable. Evidence twins:
`c-sizes.json` · `c-complexity.json` · `c-duplication.json` · `c-graph.json` · `c-dead.json` ·
`c-dead-validation.json` · `c-coverage.json` · `c-churn.json` · `c-mutation.json` ·
`c-judgment.json` · `record.json`, plus the recomputed-baseline twins
`_sizes_baseline_recomputed.json` · `_complexity_baseline_recomputed.json` ·
`_duplication_baseline_recomputed.json`. Summarizers: `sum_sizes.py` · `sum_complexity.py` ·
`sum_dup.py` · `sum_dead.py` · `sum_churn.py` · `sum_mutation.py` · `perfile_cx.py` ·
`validate_dead.py` · `validate_dead2.py` · `judge.py` · `build_record.py`.*
