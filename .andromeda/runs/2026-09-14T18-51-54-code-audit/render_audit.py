"""Phase 5 render. Every evidence table's row count is asserted against an n from a source
INDEPENDENT of the row list, and rows are asserted unique on the table's natural key."""
import json, collections

RD = 'D:/dev/projects/conductor/.andromeda/runs/2026-09-14T18-51-54-code-audit'
LEDGER = 'D:/dev/projects/conductor/.andromeda/code-metrics.ndjson'
rec = json.load(open(f'{RD}/record.json', encoding='utf-8'))
jg = json.load(open(f'{RD}/judge.json', encoding='utf-8'))
dead = json.load(open(f'{RD}/c-dead.json', encoding='utf-8'))
dup = json.load(open(f'{RD}/c-duplication.json', encoding='utf-8'))
recs = [json.loads(l) for l in open(LEDGER, encoding='utf-8') if l.strip()]
base = [r for r in recs if r['sha'] == rec['baseline_sha']][-1]

A = []  # assertion log


def assert_table(label, n_independent, rows, key):
    ks = [key(r) for r in rows]
    uniq = len(set(ks))
    ok = (n_independent == len(rows) == uniq)
    A.append(f'{label}: n={n_independent} rows={len(rows)} unique={uniq} -> {"OK" if ok else "FAIL"}')
    if not ok:
        raise SystemExit(f'EVIDENCE TABLE INTEGRITY FAIL — {label}: n={n_independent} rows={len(rows)} unique={uniq}')


M = rec['mutation']
surv = M['survivors']
assert_table('survivors (all units)', sum(c['missed'] for c in M['counts'].values()), surv, lambda r: (r[0], r[1], r[2]))
emit = [s for s in surv if s[0] == 'conductor-emit']
assert_table('survivors conductor-emit', M['counts']['conductor-emit']['missed'], emit, lambda r: (r[1], r[2]))
assert_table('cycle paths', rec['graph']['cycles'], rec['graph']['cycle_paths'], lambda r: r)
assert_table('dead candidates (full)', rec['dead']['zero_ref_candidates'], dead['full_candidates'], lambda r: (r[0],))
# Natural key is the pair PLUS the fragment size: jscpd reports several distinct clone fragments
# between one file pair (latency.rs||rate.rs appears at 17 L and 15 L, in this record AND in the
# baseline's), so the pair alone is not unique — same shape as "two survivors at one site are two rows".
assert_table('duplication top', min(10, len(dup['top'])), rec['duplication']['top'], lambda r: (frozenset(r[:2]), r[2]))
assert_table('complexity top', min(10, len(rec['complexity']['top'])), rec['complexity']['top'], lambda r: (r[0], r[1]))
assert_table('ledger records', 6, recs, lambda r: r['ts'])

by = collections.defaultdict(list)
for u, site, mut in surv:
    by[(u, site.rsplit(':', 2)[0])].append((':'.join(site.split(':')[-2:]), mut))


def surv_block(unit):
    out = []
    for (u, f), rows in sorted(by.items()):
        if u != unit:
            continue
        out.append(f'\n`{f}` — {len(rows)} survivor{"s" if len(rows) != 1 else ""}\n')
        out.append('| line:col | mutation |')
        out.append('|---|---|')
        for lc, mt in rows:
            out.append(f'| `{lc}` | {mt} |')
    return '\n'.join(out)


d = jg['deltas']
fmt = lambda v: ('—' if v is None else (f'{v:.3f}' if isinstance(v, float) else str(v)))
ms = M['scores']
last = jg['mutation_last_scores']

doc = f"""# Code Audit — Conductor · {rec['epoch']} · {rec['ts']}
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
{surv_block('conductor-emit')}

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
| duplication-up | `pct` {fmt(d['duplication.pct']['baseline'])} → {fmt(d['duplication.pct']['now'])} ({fmt(d['duplication.pct']['delta'])} pt) | improved |
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
"""

open(f'{RD}/proposals.md', 'w', encoding='utf-8', newline='\n').write(doc)
print('\n'.join(A))
print(f'\nproposals.md written: {len(doc)} bytes')
