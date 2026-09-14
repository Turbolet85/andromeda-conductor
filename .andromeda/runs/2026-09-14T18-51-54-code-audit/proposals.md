# Code Audit — Conductor · Epoch 1 — Foundation: the measurements the closures rest on · 2026-09-14T19:55:37Z
mode **trend** · HEAD `6861eb6` · baseline `0f780c62` (Epoch 6b — Polish & ship) · span 1
**Overshoot 0 — this run sits exactly ON the boundary.** The pickaxe on the epoch's last-promoted
marker (`2026-09-13-p-025-measurement-contract-for-pulse`) returns HEAD itself, so `boundary_sha == sha`
and no overshoot fork applied. The BASELINE record, by contrast, sat **6 commits past its own boundary**
with one source file in its delta (`crates/conductor-report/tests/matrix_ledger_gate.rs` +61/−6) — that
delta is attributed to Epoch 6b and is not re-diffed here.
No ancestry break. **No trend-break:** every tool version is identical to the baseline's.

Ledger: **6 records**, 0 unparseable. Source tree clean at collection (only the friction ledger, the
handoff and an untracked run dir were dirty — all excluded by the source-path definition).

---

## Proposals

Three findings ship. **None of them fired a threshold** — the threshold table's checks all passed, and
`monotonic` fired on none of the six tracked scalars. The two that follow are *absolute* findings about a
unit and symbols entering the series for the first time, which is what `audit-pass.md` prescribes for a
subject with no prior record; the third is a defect in this skill's own reference. Said plainly: **the code
this epoch produced got structurally better on every tracked axis, and the audit's value here is what it
found in a surface nobody had ever measured.**

### M1 — first measurement · `mutation` · `conductor-emit` = **45.71** (57 survivors, 10 timeouts)

**Movement:** none computable — `conductor-emit` has **no prior score in the ledger**. It was `declined` at
the Epoch 6b boundary as out-of-scope with its size recorded (458 mutants), so `mutation-drop` cannot fire.
This is shard **1/4** of the unit: 115 mutants tested, 48 caught, **57 missed**, **10 timed out**, 0 unviable.
The other three shards are untested (`budget-exhausted`), so the unit-wide figure is unmeasured — 45.71 is
the measured shard's score and is named as such.

**Evidence:** all 57 survivors, and they concentrate in exactly **two files**:

`crates/conductor-emit/src/exception.rs` — 36 survivors

| line:col | mutation |
|---|---|
| `316:46` | replace + with - in is_absolute_path_start |
| `316:46` | replace + with * in is_absolute_path_start |
| `323:9` | replace && with || in skip_absolute_path |
| `322:9` | replace && with || in skip_absolute_path |
| `321:9` | replace && with || in skip_absolute_path |
| `320:14` | replace < with <= in skip_absolute_path |
| `320:10` | replace + with - in skip_absolute_path |
| `320:10` | replace + with * in skip_absolute_path |
| `323:51` | replace == with != in skip_absolute_path |
| `323:46` | replace + with - in skip_absolute_path |
| `323:46` | replace + with * in skip_absolute_path |
| `327:11` | replace += with *= in skip_absolute_path |
| `329:13` | replace < with <= in skip_absolute_path |
| `340:7` | replace += with *= in skip_line_number_suffix |
| `341:13` | replace < with <= in skip_line_number_suffix |
| `342:11` | replace += with -= in skip_line_number_suffix |
| `344:44` | replace && with || in skip_line_number_suffix |
| `344:10` | replace < with == in skip_line_number_suffix |
| `344:10` | replace < with > in skip_line_number_suffix |
| `344:10` | replace < with <= in skip_line_number_suffix |
| `344:36` | replace == with != in skip_line_number_suffix |
| `344:53` | replace < with == in skip_line_number_suffix |
| `344:53` | replace < with > in skip_line_number_suffix |
| `344:53` | replace < with <= in skip_line_number_suffix |
| `344:49` | replace + with - in skip_line_number_suffix |
| `344:49` | replace + with * in skip_line_number_suffix |
| `344:78` | replace + with - in skip_line_number_suffix |
| `344:78` | replace + with * in skip_line_number_suffix |
| `345:11` | replace += with -= in skip_line_number_suffix |
| `345:11` | replace += with *= in skip_line_number_suffix |
| `346:31` | replace && with || in skip_line_number_suffix |
| `346:17` | replace < with == in skip_line_number_suffix |
| `346:17` | replace < with > in skip_line_number_suffix |
| `346:17` | replace < with <= in skip_line_number_suffix |
| `347:15` | replace += with -= in skip_line_number_suffix |
| `347:15` | replace += with *= in skip_line_number_suffix |

`crates/conductor-emit/src/latency.rs` — 21 survivors

| line:col | mutation |
|---|---|
| `42:9` | replace LatencyProfile::p50_ms -> u64 with 0 |
| `42:9` | replace LatencyProfile::p50_ms -> u64 with 1 |
| `47:9` | replace LatencyProfile::p95_ms -> u64 with 0 |
| `47:9` | replace LatencyProfile::p95_ms -> u64 with 1 |
| `52:9` | replace LatencyProfile::p99_ms -> u64 with 0 |
| `52:9` | replace LatencyProfile::p99_ms -> u64 with 1 |
| `113:31` | replace + with - in sample_durations_nanos |
| `125:20` | replace + with - in quantile |
| `125:20` | replace + with * in quantile |
| `125:34` | replace / with % in quantile |
| `125:34` | replace / with * in quantile |
| `125:27` | replace - with + in quantile |
| `125:27` | replace - with / in quantile |
| `126:10` | replace < with == in quantile |
| `126:10` | replace < with <= in quantile |
| `128:17` | replace < with <= in quantile |
| `130:17` | replace < with == in quantile |
| `130:17` | replace < with > in quantile |
| `130:17` | replace < with <= in quantile |
| `133:36` | replace / with % in quantile |
| `133:36` | replace / with * in quantile |

**Suspected shape:** 36 of the 57 sit in `exception.rs`'s three host-path scrubbing helpers
(`is_absolute_path_start`, `skip_absolute_path`, `skip_line_number_suffix`) — every survivor is an
arithmetic or comparison-boundary mutation (`+`→`-`, `<`→`<=`, `&&`→`||`, `+=`→`-=`) that no test
discriminates. That surface is the stacktrace scrubber behind the project's standing ban on host paths in
committed artifacts, so its boundary behaviour is exactly what a test ought to pin. The remaining 21 sit in
`latency.rs`, 11 of them inside `quantile` — the percentile math the emission path shapes latency with.
The 10 timeouts are all in this unit and have a named slot in `counts` (`audit-pass.md` records timeouts
carrying unowned across two boundaries when they have none).

**Proposal:** a direction, not a verdict — add killing tests for the two scrubber helpers' boundaries
(the off-by-one and the `&&`/`||` arms are the cheap ones: each is a one-line assertion on a crafted
stacktrace), and decide whether `quantile` wants a property test rather than 11 individual kills. Worth
pairing with a scope decision: this unit is the largest untested mutation surface in the workspace and one
shard of it already holds ~3× the whole workspace's previously-tracked survivor population.

### M2 — cross-metric agreement · `dead` + `mutation` · three `LatencyProfile` accessors

**Movement:** `dead.zero_ref_candidates` 34 → 33 (improving, fires nothing). This finding is not a
movement — it is two independent collectors naming the same three symbols.

**Evidence:** `LatencyProfile::p50_ms`, `p95_ms` and `p99_ms` (`crates/conductor-emit/src/latency.rs`) are
**simultaneously**:

| symbol | A5 dead-code | C1 mutation |
|---|---|---|
| `LatencyProfile::p50_ms` | zero-ref candidate | `:42:9` → `0` and → `1`, both survive |
| `LatencyProfile::p95_ms` | zero-ref candidate | `:47:9` → `0` and → `1`, both survive |
| `LatencyProfile::p99_ms` | zero-ref candidate | `:52:9` → `0` and → `1`, both survive |

`p95_ms` has been in the dead list since the Epoch 6b record (rank 2 of its `top`). A symbol with no
caller that no test notices returning a constant is the one case where the two metrics corroborate rather
than merely coexist — and it is the case A5's false-positive classes do NOT explain: these are plain
inherent methods, not entry points, trait-dispatch targets, derive-invoked or format-string captures.

**Suspected shape:** the accessors read three fields a scenario's `[phases.emission]` sets, and the
emission path appears to read the fields directly rather than through them.

**Proposal:** confirm against the emission call path, then either delete the three accessors or give them
one assertion each — the choice is the founder's; the finding is only that both metrics agree they are
currently inert.

### M3 — mechanism · `collectors.md` A5's pinned filter is stated as half of the filter that is deployed

**Evidence, measured this run:** A5 pins the dead-code classing chain as excluding test symbols "by PATH
SEGMENT `tests/` anywhere in the **SYMBOL** path", and parenthetically warns that a *file*-path filter
misses inline `#[cfg(test)]` modules. Applied as written against 886 raw zero-ref rows it leaves **240**.
The baseline record's own stored chain reads `898 → 38 → 34`. The filter that reproduces it is the **union**
of a `tests/` segment in the symbol path **and** in the file path: 886 → **37** → **33**, and the residual
list matches the baseline's stored `top` entries item for item.

**Suspected shape:** integration-test functions carry a bare symbol name (`a_call_error_is_..()`) with no
path component at all — the `tests/` marker lives only in their FILE path — while inline `#[cfg(test)]`
symbols carry it only in the SYMBOL path. Each half of the union catches what the other misses; the
reference documents only the half that was non-obvious.

**Proposal:** state the rule as the union in `collectors.md` A5. The stake is the rule's own stated
purpose — "reuse verbatim or `dead-growth +5` fires on filter drift, not real growth". Following the
reference literally this run would have reported `zero_ref_candidates` 34 → 240 and fired `dead-growth`
at **+206** against a codebase whose real figure improved by 1. This run's record carries the union in
`dead.filter_note` and in `commands.dead_symbols`.

---

## Informational

- **Mutation moves (4 units with a prior score, all flat or improving):** `conductor-verify` 88.71 → **91.80**
  (+3.09, survivors 14 → 10) · `conductor-cli` 96.81 → **97.87** (+1.06, missed 3 → 2) · `conductor-run`
  94.32 → **94.38** (+0.06, the same 5 survivors) · `conductor-core` 100.0 → **100.0** on the same pinned
  shard 1/4 (126/126 caught). No unit moved down; `mutation-drop` fired nowhere.
- **Second first-measurement:** `conductor-report` = **91.14** (89 mutants, 7 survivors). The Epoch 6b record
  explicitly noted this unit as "first measurement still owed" — that debt is now discharged, and it is the
  crate the epoch's final chunk landed in.
- **`count-under-ratio` is silent, for the first time in the ledger's history.** `audit-pass.md` records that
  clones rose at all four prior diffs while `pct` fell at every one. Here both fall together: clones −4
  (106 → 102), duplicated lines −200 (1049 → 849), `pct` −0.62 pt, against a population that barely moved
  (`total_lines` −0.105 % relative). Split by path now src 52 / test 40 / mixed 10 (test pairs −4).
- **Duplication top-10 turned over completely — by removal, not by growth.** The baseline's entire top-5 was
  `conductor-emit` test clones at 36–44 lines; three are gone outright and `egress || multi_service_topology`
  shrank 38 L → 9 L. The largest clone pair anywhere in the workspace is now **17 lines**, and **no pair is
  ≥ 20 lines**. The src-side `journal.rs || record.rs` pair also shrank, 24 L → 9 L.
- **Six duplication top-10 "entrants" are displacement, not new duplication** — all six sit at 14 lines, below
  the baseline's 10th place (15 L), and total duplicated lines fell by 200, so the list cannot have grown.
  They became visible because the head of the list was removed. Named because a reader scanning entrants
  would otherwise read six new clone families: `expected.rs || slo.rs` · `pause.rs || operator_pause.rs` ·
  `run_record.rs || db.rs` · `logs.rs || pii.rs` · `topology.rs || multi_service_topology.rs` ·
  `canary.rs || preflight.rs`. (Basis: the falling totals plus the 14 L < 15 L cutoff — the baseline's full
  clone list is not an input to this run, so this is inference from the stored aggregates, not a lookup.)
- **One `conductor-emit` test file was not folded into the shared module.** 19 clone pairs still involve
  `conductor-emit/tests/`, all ≤ 14 L, and one is `tests/common/mod.rs || tests/pii_payload_corpus.rs` at
  11 L — the epoch's chunk collapsed five pairs onto the shared module and this sixth file still carries an
  echo of it.
- **Hotspots are not comparable across this boundary.** The score is `commits × max cognitive`, and this
  window is 3 commits against the previous window's many, so the top score falls 62.0 → 7.0 with no code
  change behind it. All ten hotspot rows read as "entrants" for the same reason. The window length, not the
  code, moved this metric.
- **Churn is 0.0 %** (17 files touched, none touched twice across the 3 commits) — one chunk per commit with
  disjoint file sets. Informational by construction.
- **One fan-in top-20 "entrant" is a tie-boundary artifact:** `conductor-verify common/StubConfig#` (39)
  entered and `conductor-core phase_spec/EmissionShape#` (39) left, at identical values on a `LIMIT 20`.
  Real fan-in movement is small and upward: `conductor-core crate/` 139 → 141, `VerifyError#` 43 → 45.
- **`dead_web` flat 12 → 12** (exports 3, types 9), and the Epoch 6b record's measured caveat is carried
  forward verbatim: knip scored 20 findings / **20 false positives** on its first run because WebdriverIO
  discovers specs through `wdio.conf.ts` and resolves devDependencies through its own plugin machinery,
  neither an import edge knip can see. All 12 items sit in that same surface plus two component exports.
  Reported as candidates against an unconfigured tool, never as dead code.
- **A clone row's natural key is the file pair PLUS the fragment size, not the pair alone.** The render-time
  integrity check failed first time round with `n=10 rows=10 unique=9`: jscpd reports several distinct clone
  fragments between one file pair, and `latency.rs || rate.rs` occupies two top-10 rows (17 L and 15 L) — in
  this record and in the baseline's. `proposal-template.md` names "a file PAIR" as that table's natural key,
  which under-counts exactly as a survivor key would if it ignored the mutation. The check did its job; the
  key wanted widening. Same family as M3.
- **`tool_versions.lizard` differs only in trailing prose** ("see recipes.lizard" vs "see
  skips.complexity-web") across records. The version token is identical (1.24.0) and the tool is
  uncollected, so this is not a trend-break — but a raw string compare reports one, which would suppress
  thresholds for a metric that has no values. This run's judge compares the leading version token. Worth
  keeping prose out of that field.

---

## Below threshold — no action

| check | movement | verdict |
|---|---|---|
| new-cycle | `graph.cycles` 0 → 0 (cross-unit edges 16 → 16, fan-out set identical) | no fire |
| duplication-up | `pct` 3.245 → 2.629 (-0.616 pt) | improved |
| complexity-creep | `over_ceiling` 6 → 6; top-10 offenders **identical** in composition and value (max `serve_stub` cg 35) | flat |
| dead-growth | `zero_ref_candidates` 34 → 33; chain 886 → 37 → 33 | improved |
| coverage-drop | line 94.3 % → **94.4 %** (10953/11598); 909 tests, 909 passed | improved |
| mutation-drop | no unit's score fell; lowest move +0.06 | no fire |
| monotonic | all six tracked scalars evaluable, none worse at both diffs (see below) | no fire |
| span > 1 | span = 1, consecutive boundaries | n/a |
| trend-break | every tool version token identical to baseline | n/a |

`monotonic` chain, by sha (Epoch 6a → 6b → this):
`duplication.pct` 3.431 → 3.245 → 2.629 · `complexity.over_ceiling` 6 → 6 → 6 ·
`dead.zero_ref_candidates` 35 → 34 → 33 · `sizes.file_max` 1423 → 1170 → 1170 ·
`sizes.over_800` 2 → 1 → 1 · `coverage.line` 93.94 → 94.3 → 94.4.
Every one flat or improving at **both** diffs.

Population context (never read as a movement): `totals.loc` 25077 → 25005 · `totals.files` 135 → 136 ·
`duplication.total_lines` 32324 → 32290. Sizes unchanged: `file_p50` 115, `file_max` 1170
(`conductor-core/src/scenario.rs`), `over_800` 1 — top-10 byte-identical to baseline. `file_p90` 458 → 473.

Survivors held from the baseline (not new): `conductor-cli` render.rs `stdout_color`/`stderr_color` (both
ratified `member-5` roster entries) · `conductor-run` canary.rs `delete ! in preflight` and execute.rs
`delete field degraded` (both ratified). Full 81-row survivor set is in `record.json`'s
`mutation.survivors` and per-unit in `c-mutation-*.json`.

---

## Skips

| metric | reason | note |
|---|---|---|
| `mutation-web` | tool-missing | StrykerJS absent and not declared in `package.json` → `npm i -D @stryker-mutator/core` in `crates/conductor-tauri/ui` |
| `complexity-web` | declined | lizard 1.24.0 IS available; not collected, matching the deliberate decline at both prior boundaries, to keep the Rust-only complexity series comparable across all six records |
| `mutation-conductor-core` | budget-exhausted | shard 1/4 (126 tested) — the baseline's pinned form, kept for comparability; 3 shards untested |
| `mutation-conductor-emit` | budget-exhausted | shard 1/4 (115 tested of 458 in the unit) — sharded for the same wall-clock reason; 3 shards untested, so M1's figure is the shard's |

**Out of scope rather than skipped:** `conductor-tauri`, `conductor-timeline` and `conductor-faults` have no
source or manifest change in `0f780c62..HEAD`, so the touched-unit rule excludes them. `conductor-tauri`'s
last measured score (90.0, Epoch 6b) stands unchanged.

**Correction carried in this run's record** (prior records are never edited): the Epoch 6b record's
`head_overshoot` held a prose note but no structured `commits`/`files` and a null
`commands.head_overshoot`. Recomputed over `b54e6ec..0f780c62` → 6 commits, one source file
(`matrix_ledger_gate.rs` +61/−6), which **confirms its stored note exactly**. Filed as a schema-gap fill in
`corrections[]`.

**Exit self-check — one dropping surfaced, not touched.** `git status` gained nothing outside this run dir
and the ledger. A `mutants.out/` directory sits at the PROJECT ROOT, but it is **not this run's**: its
newest file is dated `2026-09-03 07:16` and **zero** files inside it are newer than this run's start, while
all six of this run's units wrote to `{run_dir}/mutants-{unit}/` as the firing form records. It is
gitignored (`.gitignore:49`), so it never reaches `git status` and cannot ride a commit. Left in place —
outside `{run_dir}` this skill moves or surfaces, never deletes. Also noted for the founder's eye: `target/`
holds 18 stale `mutants-*` output dirs from 2026-09-03 → 09-05 sessions, likewise gitignored and part of the
76 GB build cache.

**Defect in THIS run's own record, surfaced after the append (2026-09-14).** Eight `commands` entries in
the record appended at `6861eb6` — `duplication`, `complexity`, `dead_symbols`, `graph`, `coverage`,
`mutation`, `mutation_conductor-core`, `mutation_conductor-emit` — spell their `--output` / run-dir argument
as an **absolute host path** (`{drive}:/dev/projects/conductor/.andromeda/runs/…`). All five prior records
write these repo-relative, and `commands` is the reproducibility contract, so an argument naming this
operator's drive is both less reproducible and inconsistent with the series. Scope stated accurately: this
is **not** a breach of the project's absolute-host-path ban, which the 2026-06-27 security ruling confines
to Conductor-GENERATED artifacts (the run report, `runs.db`, the journal, self-obs logs) — `.andromeda/` is
pipeline bookkeeping. It is a reproducibility and consistency defect only.

The record is **not** being rewritten: `audit-pass.md` §Modes makes a landed record immutable "whoever
directs it", and names a dated proposals.md line as the channel exactly when a correction surfaces *after*
the append. This is that line. It should ride the NEXT ledger-mode record as
`corrections: [{target_sha: "6861eb63439875520599180c5a726564e0f9884a", field: "commands", …}]` with the
eight keys re-stated repo-relative. The values those commands produced are unaffected — every population
and firing form is otherwise identical to the baseline's. The same absolute prefix appears in this run dir's
summarizer scripts and `c-mutation-*.json` `command` strings; those are left as the true record of what ran
(the host rule prescribes native absolute paths across the bash↔native boundary, and editing evidence after
the fact would falsify it).

**Size discipline applied:** raw tool output deleted inside this run dir after summarization — 445.5 MB →
**287 KB** (the `rust-code-analysis` walk alone was 19 467 JSON files / 436 MB, and the six
`cargo-mutants` output trees 1 468 files). Kept: the capped `c-*.json` twins, the full survivor and
candidate lists inside them, `record.json`, `judge.json`, the graph adoption trace, the summarizer scripts
(the reproducibility triple regenerates everything else at `6861eb6`), and the tier's per-unit timing log.
