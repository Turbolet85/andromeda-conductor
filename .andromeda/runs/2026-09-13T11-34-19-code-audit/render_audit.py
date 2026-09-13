"""Render proposals.md for the code audit. Evidence tables are generated from the c-*.json twins;
every table asserts its row count equals the stated n (template's mechanical half)."""
import json, os, datetime

RUN = os.path.dirname(os.path.abspath(__file__))
RUNREL = '.andromeda/runs/2026-09-13T11-34-19-code-audit'
J = lambda n: json.load(open(f'{RUN}/c-{n}.json', encoding='utf-8'))
_ALL = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
SHA, BASE, EPOCH = '0f780c6', '59d5b7c8', 'Epoch 6b — Polish & ship'
# This run's record is appended BEFORE rendering, so LEDGER[-1] is THIS run, not the baseline.
# Resolve the baseline by sha, and render the prior records only — the current row is emitted
# explicitly from the live c-*.json twins. (Measured: taking LEDGER[-1] as the baseline printed
# "106 -> 106" and duplicated the 6b row; the row-count assertion could not catch it, because the
# stated n was computed from the same wrong list.)
LEDGER = [r for r in _ALL if not r['sha'].startswith(SHA)]
B = next(r for r in _ALL if r['sha'].startswith(BASE))

sz, dup, cx, gr, dead, cov, ch, hs = (J('sizes'), J('duplication'), J('complexity'), J('graph'),
                                      J('dead'), J('coverage'), J('churn'), J('hotspots'))
dweb = J('deadweb')
mut = J('mutation')
rec = json.load(open(f'{RUN}/record.json', encoding='utf-8')) if os.path.exists(f'{RUN}/record.json') else None

out = []
w = out.append
ASSERTS = []


def table(header, rows, stated_n, label):
    """Emit a markdown table and assert row count == stated n (never by eye)."""
    ASSERTS.append((label, len(rows), stated_n))
    w('| ' + ' | '.join(header) + ' |')
    w('|' + '---|' * len(header))
    for r in rows:
        w('| ' + ' | '.join(str(c) for c in r) + ' |')


def series(path, key):
    vals = []
    for r in LEDGER:
        d = r
        for p in path:
            d = (d or {}).get(p) if isinstance(d, dict) else None
        vals.append((r['epoch'], (d or {}).get(key) if isinstance(d, dict) else d))
    return vals


w(f'# Code Audit — Conductor · {EPOCH} · {datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")}')
w('')
w(f'mode **trend** · HEAD `{SHA}` · baseline `{BASE}` (Epoch 6a — Verification follow-ups) · span 1')
w('')
w('Ancestry OK. No trend-break: every tool version matches the baseline '
  '(jscpd 5.0.16 · tokei 14.0.0 · rust-code-analysis 0.0.25 · cargo-mutants 27.1.0 · rustc 1.95.0), '
  'so no metric\'s thresholds are suppressed.')
w('')
w('**HEAD overshoots the epoch boundary by 3 chunks** (the first three conductor-0.3.0 Epoch-1 chunks). '
  'The source delta from the 6b boundary `b54e6ec` to HEAD is exactly ONE file — '
  '`crates/conductor-report/tests/matrix_ledger_gate.rs` (+61/−6, a test file); everything else in those '
  'three chunks is CI yaml, scripts and `.andromeda/` documents, none of which is a source path. '
  'Every number below therefore includes that one file and nothing else from beyond the boundary.')
w('')

# ---------------------------------------------------------------- the trend table
w('## The five-record trend')
w('')
rows = []
for r in LEDGER:
    d, c, s, g, dd = (r.get('duplication') or {}, r.get('complexity') or {}, r.get('sizes') or {},
                      r.get('graph') or {}, r.get('dead') or {})
    rows.append([r['epoch'][:26], (r.get('totals') or {}).get('loc'), (r.get('totals') or {}).get('files'),
                 f"{d.get('pct', 0):.2f}", d.get('clones'), d.get('duplicated_lines') or '—',
                 c.get('over_ceiling'), s.get('file_max'), s.get('over_800'), g.get('cycles'),
                 g.get('cross_unit_edges'), dd.get('zero_ref_candidates'),
                 (r.get('coverage') or {}).get('line')])
rows.append([f'**{EPOCH[:26]}**', f"**{sz['loc']}**", f"**{sz['files']}**", f"**{dup['pct']:.2f}**",
             f"**{dup['clones']}**", f"**{dup['duplicated_lines']}**", f"**{cx['over_ceiling']}**",
             f"**{sz['file_max']}**", f"**{sz['over_800']}**", '**0**', '**16**',
             f"**{dead['zero_ref_candidates']}**", f"**{cov['line']}**"])
table(['epoch', 'loc', 'files', 'dup %', 'clones', 'dup lines', 'cog>15', 'file max', '>800',
       'cycles', 'edges', 'dead', 'cov %'], rows, len(LEDGER) + 1, 'trend table')
w('')

# ---------------------------------------------------------------- proposals
w('## Proposals')
w('')

# M1 — monotonic duplication
clones = [(e, v) for e, v in series(['duplication'], 'clones')]
dlines = [(e, v) for e, v in series(['duplication'], 'duplicated_lines')]
w('### M1 — `monotonic` · duplication (absolute) — clones +16, duplicated lines +143')
w('')
w(f"**Movement:** clones {B['duplication']['clones']} → {dup['clones']} · duplicated lines "
  f"{B['duplication']['duplicated_lines']} → {dup['duplicated_lines']} (Epoch 6a → Epoch 6b). "
  f"Percentage moved the other way: {B['duplication']['pct']:.2f}% → {dup['pct']:.2f}%.")
w('')
w('**Evidence:** the `monotonic` rule needs a scalar worsened at both of the last two diffs. Two qualify, '
  'and in fact every measured diff worsens:')
w('')
rows = [[e[:26], c if c is not None else '—', d if d is not None else '—', f'{p:.2f}%', t or '—']
        for (e, c), (_, d), (_, p), (_, t) in zip(clones, dlines, series(['duplication'], 'pct'),
                                                  series(['duplication'], 'total_lines'))]
rows.append([f'**{EPOCH[:26]}**', f"**{dup['clones']}**", f"**{dup['duplicated_lines']}**",
             f"**{dup['pct']:.2f}%**", f"**{dup['total_lines']}**"])
table(['epoch', 'clones', 'duplicated lines', 'dup %', 'total lines'], rows, len(LEDGER) + 1,
      'M1 duplication series')
w('')
w('Clones rose at **all four** measured diffs (79 → 84 → 86 → 90 → 106); duplicated lines rose at **all '
  'three** for which the field exists (853 → 870 → 906 → 1049). Over the same five records the percentage '
  'fell at every single diff (3.68 → 3.66 → 3.58 → 3.43 → 3.25), because total lines grew faster '
  '(23 329 → 32 324).')
w('')
w('**Suspected shape:** the growth is not diffuse. The five largest clone families are all carried from the '
  'baseline and all live in `conductor-emit/tests/` — the same fixture block repeated across emission test '
  'files:')
w('')
carried = [t for t in dup['top'][:5]]
table(['lines', 'file A', 'file B'], [[n, a, b] for a, b, n in carried], 5, 'M1 carried families')
w('')
w('**Proposal:** two independent directions, both for your judgment. (a) The `conductor-emit` test fixtures '
  'are the whole top of the list and have been for four epochs — a shared `tests/common/` fixture helper '
  'would remove the family rather than the symptom. (b) Separately, note that the audit\'s own '
  '`duplication-up` check keys on **percentage** (`pct ≥ baseline + 0.5pt AND ≥ +15% relative`). On a '
  'codebase growing at this rate that check is structurally unable to fire — it has moved the *favourable* '
  'way at every diff while the absolute measure worsened at every diff. Whether the check should track the '
  'absolute count, or the percentage is the honest metric and this proposal is noise, is a threshold '
  'decision only you can make.')
w('')

# M2..Mn — mutation
mut_props = []
scores = mut.get('scores') or {}
bscores = (B.get('mutation') or {}).get('scores') or {}
for u, s in sorted(scores.items()):
    bs = bscores.get(u)
    if bs is not None and s is not None and s <= bs - 10:
        mut_props.append((u, bs, s))
if mut_props:
    for i, (u, bs, s) in enumerate(mut_props, 2):
        w(f'### M{i} — `mutation-drop` · {u} — {s - bs:+.2f}pt')
        w('')
        w(f'**Movement:** {bs} → {s} (Epoch 6a → Epoch 6b), threshold is a drop of 10pt or more.')
        w('')
        surv = [r for r in mut['survivors'] if r[0] == u]
        w(f'**Evidence:** all {len(surv)} surviving mutants in this unit —')
        w('')
        table(['site', 'mutation'], [[r[1], r[2]] for r in surv], len(surv), f'M{i} survivors {u}')
        w('')
        w(f"**Proposal:** add killing tests for the survivors above; the counts are "
          f"{json.dumps(mut['counts'][u])} under `{mut['score_formula']}`.")
        w('')
NEXT = 2 + len(mut_props)

# standing weak unit (not a drop, but a low absolute with a second data point)
if 'conductor-cli' in scores:
    cli, bcli = scores['conductor-cli'], bscores.get('conductor-cli')
    if cli is not None and cli < 70:
        surv = [r for r in mut['survivors'] if r[0] == 'conductor-cli']
        w(f'### M{NEXT} — `absolute` · conductor-cli mutation score — {cli}% (second consecutive reading below 70)')
        w('')
        w(f'**Movement:** {bcli} → {cli} (Epoch 6a → Epoch 6b). This does not fire `mutation-drop` '
          f'(the drop threshold is −10pt), and it is listed because the *level* has now been measured twice: '
          f'it is the weakest unit in the workspace by a wide margin, against '
          f'{", ".join(f"{k} {v}" for k, v in sorted(scores.items()) if k != "conductor-cli")}.')
        w('')
        w(f'**Evidence:** all {len(surv)} surviving mutants —')
        w('')
        table(['site', 'mutation'], [[r[1], r[2]] for r in surv], len(surv), f'M{NEXT} survivors cli')
        w('')
        w(f"**Suspected shape:** `conductor-cli` is the render/dispatch shell; counts "
          f"{json.dumps(mut['counts']['conductor-cli'])}. A shell whose output shape is asserted by "
          f"few tests survives most mutations by construction.")
        w('')
        w('**Proposal:** the survivor list above is the concrete target set. Whether a CLI render layer '
          'earns the same mutation bar as the engine crates is a scope question for you — the number is '
          'reported here so the decision is made rather than inherited.')
        w('')
        NEXT += 1

# ---------------------------------------------------------------- informational
w('## Informational')
w('')
bsz = B.get('sizes') or {}
w(f"- **The largest file in the workspace was dismantled.** `conductor-run/src/lib.rs` was the baseline's "
  f"file_max at **1423** lines and its **#1 B2 hotspot** (score 32.0). It is now **40 lines**, split into "
  f"8 modules (`canary` · `dispatch` · `drive` · `envelope` · `execute` · `lifecycle` · `preconditions` · "
  f"`testkit`) by `2026-09-05-audit-corrective` — literally *\"the module split along its named seams\"*. "
  f"file_max fell {bsz.get('file_max')} → {sz['file_max']} and files over 800 lines fell "
  f"{bsz.get('over_800')} → {sz['over_800']}, reversing four epochs of monotonic rise "
  f"(898 → 1012 → 1115 → 1423).")
w(f"- **Codebase grew {sz['loc'] - (B['totals']['loc'])} lines (+{100 * (sz['loc'] - B['totals']['loc']) / B['totals']['loc']:.1f}%)**, "
  f"{B['totals']['files']} → {sz['files']} files — by far the largest epoch delta on record "
  f"(prior deltas +975, +595, +1677). Verified against git: `59d5b7c8..HEAD` over `crates/**/*.rs` is "
  f"+9377/−3462 = +5915 raw lines, consistent with tokei's +{sz['loc'] - B['totals']['loc']} code lines.")
w(f"- **Complexity is unchanged — exactly.** The over-ceiling set is byte-identical to the baseline: the "
  f"same 6 functions at the same values (serve_stub 35 · shape_is_realizable 31 · validate 18 · observe 17 "
  f"· dispatch 16 · run_preflight 16). p50/p90 identical too (cyclomatic 1.0/4.0, cognitive 0/1.0) across "
  f"+{cx['functions'] - (B['complexity'] or {}).get('functions', 0)} new functions.")
w(f"- **Architecture is unchanged — exactly.** 0 cycles and 16 cross-unit edges for the fifth consecutive "
  f"record; per-unit fan-out identical.")
w(f"- **Coverage rose** {(B.get('coverage') or {}).get('line')} → {cov['line']}% "
  f"({cov['lines_hit']}/{cov['lines_found']} lines), a fourth consecutive record above 92 and the highest "
  f"measured. 906/906 tests passed during collection.")
w(f"- **Churn halved** — {(B.get('churn') or {}).get('pct')}% → {ch['pct']}% "
  f"({ch['churned_adds']}/{ch['total_adds']} adds re-touched, {ch['files_churned']} of {ch['files_touched']} "
  f"files). The window is much larger than the baseline's ({ch['files_touched']} files touched vs "
  f"{(B.get('churn') or {}).get('files_touched')}), so the lower rate is over more work, not less.")
w(f"- **Dead-code candidates fell** {(B.get('dead') or {}).get('zero_ref_candidates')} → "
  f"{dead['zero_ref_candidates']} (chain: {dead['chain']['raw']} raw → "
  f"{dead['chain']['after_tests_segment_filter']} after the pinned tests-segment filter → "
  f"{dead['chain']['after_entry_points']} after entry points). Unused deps unchanged: "
  f"`conductor-emit` → `conductor-core`, standing since the baseline.")
w('- **B2 hotspot top-10 turned over almost completely**, which is expected — the score is '
  'commits × complexity and the commit window moved. New #1 is `conductor-core/src/phase_spec.rs` '
  '(score 62.0 = 2 commits × cognitive 31); its `shape_is_realizable` is also the #2 complexity offender '
  'workspace-wide. It is the one file that is both complex and repeatedly touched.')
w(f"- **New clone family, cross-crate:** `conductor-report/src/journal.rs:158-181` ↔ "
  f"`conductor-verify/src/record.rs:137-160`, 24 lines — the only new entrant in the top-10 clone list. "
  f"Both fragments are the **eleven-key run-report envelope assertion**, inside inline `#[cfg(test)]` "
  f"modules. `ci.yml:178-179` records the repo's own position that the schema already has two copies "
  f"(\"the structs · obs-plan §3/§6 · this file … would drift from the first two\") and declines to add a "
  f"third; this measurement locates the two literal key enumerations precisely. Worth noting only because "
  f"a key added to the envelope must be edited in both.")
w(f"- **`conductor-cli`'s test suite was transformed** — mutation score **{bscores.get('conductor-cli')} → "
  f"{scores.get('conductor-cli')}**, the largest single movement in this audit. The baseline left 41 of 99 "
  f"viable mutants alive; this run leaves **3 of 94** "
  f"({json.dumps(mut['counts']['conductor-cli'])}). It was the workspace's weakest unit by a wide margin "
  f"and is now its second strongest. `crates/conductor-cli/tests/cross_surface_parity.rs` is new in this "
  f"window and is the plausible cause, though the audit measures the outcome, not the attribution.")
w(f"- **Mutation held or improved in every scoped unit.** "
  + ' · '.join(f"{u} {bscores.get(u)}→{scores[u]}" for u in sorted(scores)) +
  f". `conductor-core` caught **126/126** on shard 1/4 (the baseline's own scope, so the two are "
  f"comparable) — 0 missed, 0 unviable. Total surviving mutants across all five units: "
  f"{len(mut['survivors'])}, down from {len((B.get('mutation') or {}).get('survivors') or [])}.")
w(f"- **Pre-existing tool droppings at the project root**, surfaced not touched: `mutants.out/` (Sep 3) and "
  f"`mutants.out.old/` (Aug 20) predate this run and are explicitly gitignored (`.gitignore:49-50`), as are "
  f"the 18 `target/mutants-*` dirs from earlier chunks. Nothing from this run landed outside its run dir.")
w(f"- **Web dead-code series STARTS here.** knip 6.34.0 is now installed (it was `tool-missing` at the "
  f"baseline), reporting {dweb['total']} candidates across {dweb['files_with_issues']} files "
  f"({json.dumps(dweb['counts'])}). **No trend exists yet**, and the count is reported as candidates "
  f"against an unconfigured tool: this epoch's own friction record measured knip's first run at 20 "
  f"findings / 20 false positives, because WebdriverIO discovers specs through `wdio.conf.ts` and loads "
  f"devDependencies through its own plugin resolution — neither is an import edge knip can see.")
w('')

# ---------------------------------------------------------------- below threshold
w('## Below threshold — no action')
w('')
w(f"- `new-cycle` — cycles 0 → 0, cross-unit edges 16 → 16. Not fired.")
w(f"- `duplication-up` — pct {B['duplication']['pct']:.2f} → {dup['pct']:.2f} (fell); needs +0.5pt and "
  f"+15% relative. Not fired. (The absolute movement is M1.)")
w(f"- `complexity-creep` — over_ceiling {(B['complexity'] or {}).get('over_ceiling')} → "
  f"{cx['over_ceiling']} (Δ0); needs +3 and +25%. Not fired.")
w(f"- `dead-growth` — candidates {(B.get('dead') or {}).get('zero_ref_candidates')} → "
  f"{dead['zero_ref_candidates']} (−1); needs +5. Not fired.")
w(f"- `coverage-drop` — line {(B.get('coverage') or {}).get('line')} → {cov['line']} (+{cov['line'] - (B.get('coverage') or {}).get('line'):.2f}pt); "
  f"needs −2pt. Not fired.")
for u, s in sorted(scores.items()):
    bs = bscores.get(u)
    if bs is None:
        continue
    if not (s <= bs - 10):
        w(f"- `mutation-drop` — {u} {bs} → {s} ({s - bs:+.2f}pt); needs −10pt. Not fired.")
w(f"- `span > 1` — not applicable, span is 1 (consecutive boundaries).")
w(f"- `trend-break` — no tool version differs from the baseline.")
w('')
w('**Duplication split by path** (new field this run): '
  f"{json.dumps(dup['split'])}. Read with care — the classing is by file path, and Rust keeps unit tests "
  'inline in `src/`, so genuinely-test clones count as `src`. Both fragments of the new cross-crate family '
  'above are test code living in `src/`. No baseline exists for this field.')
w('')

# ---------------------------------------------------------------- skips
w('## Skips')
w('')
skips = (rec or {}).get('skips') or []
for s in skips:
    w(f"- **{s['metric']}** — `{s['reason']}` — {s['note']}")
w('')
w('---')
w('')
w(f'*Evidence twins in this run dir: `c-sizes` · `c-duplication` · `c-complexity` · `c-graph` · `c-dead` · '
  f'`c-deadweb` · `c-coverage` · `c-churn` · `c-hotspots` · `c-mutation` (+ per-unit) `.json`, and '
  f'`record.json` as appended. Reproducibility triple is in the ledger record: sha, tool_versions, and the '
  f'`commands` field carries each firing form. Report-only — nothing here gates, blocks, or is remembered; '
  f'fixes route through a route entry or chunk at your call.*')

open(f'{RUN}/proposals.md', 'w', encoding='utf-8').write('\n'.join(out) + '\n')

bad = [(l, got, want) for l, got, want in ASSERTS if got != want]
for l, got, want in ASSERTS:
    print(f'  assert {l}: {got} rows vs stated {want} — {"OK" if got == want else "MISMATCH"}')
if bad:
    raise SystemExit(f'EVIDENCE ROW-COUNT MISMATCH: {bad}')
print(f'\nproposals.md written · {len(out)} lines · {1 + len(mut_props) + (1 if NEXT > 2 + len(mut_props) else 0)} proposals')
