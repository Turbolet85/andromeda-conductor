"""Phase 3 + 4: assemble the ledger record, then diff and judge against the baseline.
Baseline is re-found by baseline_sha (never the ledger's last line); the monotonic chain is
current -> baseline -> baseline.baseline_sha, by sha."""
import json, glob, os, re, datetime

RD = 'D:/dev/projects/conductor/.andromeda/runs/2026-09-14T18-51-54-code-audit'
LEDGER = 'D:/dev/projects/conductor/.andromeda/code-metrics.ndjson'
SHA = '6861eb63439875520599180c5a726564e0f9884a'
BASE_SHA = '0f780c62651f1aaa9cd95b9316159f16288c3f6d'
EPOCH = 'Epoch 1 \u2014 Foundation: the measurements the closures rest on'

norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('\u2013', '-').replace('\u2014', '-')


def load(n):
    p = f'{RD}/c-{n}.json'
    return json.load(open(p, encoding='utf-8')) if os.path.exists(p) else None


recs, bad = [], 0
for line in open(LEDGER, encoding='utf-8'):
    if not line.strip():
        continue
    try:
        recs.append(json.loads(line))
    except ValueError:
        bad += 1
base = [r for r in recs if r.get('sha') == BASE_SHA][-1]
base2 = [r for r in recs if r.get('sha') == base.get('baseline_sha')]
base2 = base2[-1] if base2 else None

sizes, dup, cx, graph, dead, dweb, cov, churn, hs = (load('sizes'), load('duplication'), load('complexity'),
                                                     load('graph'), load('dead'), load('deadweb'),
                                                     load('coverage'), load('churn'), load('hotspots'))

# ---------- mutation ----------
SCOPED = ['conductor-cli', 'conductor-core', 'conductor-emit', 'conductor-report',
          'conductor-run', 'conductor-verify']
SHARDED = {'conductor-core': '1/4', 'conductor-emit': '1/4'}
mut = {'scoped_units': [], 'unit_states': {}, 'scores': {}, 'counts': {},
       'score_formula': 'caught/(caught+missed)  [timeout and unviable excluded]', 'survivors': []}
skips = []
for u in SCOPED:
    a = f'{RD}/c-mutation-{u}.json'
    x = f'{RD}/x-mutation-{u}.json'
    if os.path.exists(a):
        d = json.load(open(a, encoding='utf-8'))
        mut['scoped_units'].append(u)
        mut['unit_states'][u] = d['unit_state']
        mut['scores'][u] = d['score']
        mut['counts'][u] = d['counts']
        for s in d.get('survivors') or []:
            mut['survivors'].append([u] + s)
        if u in SHARDED:
            skips.append({'metric': f'mutation-{u}', 'reason': 'budget-exhausted',
                          'note': f"ran --shard {SHARDED[u]} (the baseline's pinned form for conductor-core, applied to conductor-emit for the same wall-clock reason); the other 3 shards are untested this run",
                          'tested': d['counts']['mutants'], 'planned_in_shard': d.get('planned_from_mutants_json')})
    elif os.path.exists(x):
        d = json.load(open(x, encoding='utf-8'))
        mut['unit_states'][u] = d.get('classified', 'incomplete')
        skips.append({'metric': f'mutation-{u}', 'reason': d.get('classified', 'budget-exhausted'),
                      'note': f"invocation state: {d.get('completion_state')}; elapsed {d.get('elapsed_s')}s; no partial score read"})
    else:
        mut['unit_states'][u] = 'not run'
        skips.append({'metric': f'mutation-{u}', 'reason': 'declined', 'note': 'driver did not reach this unit'})

skips += [
    {'metric': 'mutation-web', 'reason': 'tool-missing',
     'note': 'StrykerJS absent; not declared in package.json. Recipe: npm i -D @stryker-mutator/core in crates/conductor-tauri/ui'},
    {'metric': 'complexity-web', 'reason': 'declined',
     'note': "lizard 1.24.0 IS available (python -m lizard); not collected, matching the deliberate decline at both prior boundaries, to keep the Rust-only complexity series comparable across all six records"},
]

TOOLS = {'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25',
         'cargo-machete': '0.9.2', 'cargo-mutants': '27.1.0', 'cargo-llvm-cov': '0.8.5',
         'cargo-nextest': '0.9.133', 'code-graph': 'tree.db via scripts/code-graph.py',
         'knip': '6.34.0', 'lizard': '1.24.0 (available, NOT collected — see skips.complexity-web)',
         'rustc': '1.95.0 (59807616e 2026-04-14)'}

rec = {
    'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
    'epoch': EPOCH, 'mode': 'trend', 'sha': SHA, 'baseline_sha': BASE_SHA, 'span': 1,
    'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': SHA, 'commits': 0, 'files': [],
                       'note': 'AT the boundary: the pickaxe on the epoch\u2019s last-promoted marker (2026-09-13-p-025-measurement-contract-for-pulse) returns HEAD itself, so boundary_sha == sha and no overshoot fork applied.'},
    'tool_versions': TOOLS,
    'totals': {'loc': sizes['totals']['loc'], 'files': sizes['totals']['files'], 'units': 9},
    'duplication': {k: dup[k] for k in ('label', 'population', 'pct', 'duplicated_lines', 'total_lines', 'clones', 'sources', 'split', 'split_pairs_only', 'top', 'note')},
    'complexity': cx, 'sizes': {k: sizes[k] for k in ('population', 'file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': graph,
    'dead': {k: dead[k] for k in ('population', 'unused_deps', 'zero_ref_candidates', 'chain', 'filter_note', 'top')},
    'dead_web': {'tool': 'knip 6.34.0', 'counts': dweb['by_kind'], 'total': dweb['total'],
                 'files_with_issues': len({i[1] for i in dweb['items']}),
                 'population': 'npx knip --reporter json in crates/conductor-tauri/ui (no knip config present)',
                 'CAVEAT': "Carried forward from the Epoch 6b record: knip measured 20 findings / 20 FALSE POSITIVES on its first run (2026-09-07-dependency-polish) because WebdriverIO discovers specs through wdio.conf.ts and loads devDependencies through its own plugin resolution, neither an import edge knip can see. Reported as CANDIDATES against an UNCONFIGURED tool, never as dead code. All 12 items this run sit in that same WebdriverIO-discovered surface plus two component exports.",
                 'series': 'flat 12 -> 12 vs the Epoch 6b boundary (identical counts: exports 3, types 9)'},
    'coverage': cov, 'churn': churn, 'hotspots': hs['top'],
    'mutation': mut,
    'commands': {
        'sizes': 'tokei --output json crates/',
        'duplication': f'jscpd crates --format rust --reporters json --output {RD}/jscpd_rs --silent',
        'complexity': f'mkdir -p {RD}/_rca_head && rust-code-analysis-cli --metrics -O json -o {RD}/_rca_head -p crates   (summarizer filters to *.rs; the walk also visits the ui tree)',
        'dead_deps': 'cargo machete',
        'dead_symbols': f'python scripts/code-graph.py query {RD} code-audit "SELECT s.symbol, s.file FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL;" rust   (classing: tests/ exclusion = UNION of symbol-path AND file-path segments, then main() entry points)',
        'dead_web': 'npx --no-install knip --reporter json   (cwd crates/conductor-tauri/ui; exit 1 = findings present, knip convention)',
        'graph': f'python scripts/code-graph.py query {RD} code-audit "<cycles | fan-in LIMIT 20 | fan-out | count(*) FROM crate_edges>" rust',
        'coverage': f'cargo llvm-cov nextest --workspace --lcov --output-path {RD}/_lcov.info',
        'churn': "git log --numstat --format='commit %H' 0f780c62..HEAD -- 'crates/**/*.rs' 'crates/**/*.ts' 'crates/**/*.tsx'",
        'hotspots': 'commits(churn window) x max cognitive per file over the FULL rca population (fallback cyclomatic, then 0 for non-Rust)',
        'mutation': f'cargo mutants -p {{unit}} --test-tool=nextest --jobs 2 --output {RD}/mutants-{{unit}}',
        'mutation_conductor-core': f'cargo mutants -p conductor-core --test-tool=nextest --jobs 2 --output {RD}/mutants-conductor-core --shard 1/4',
        'mutation_conductor-emit': f'cargo mutants -p conductor-emit --test-tool=nextest --jobs 2 --output {RD}/mutants-conductor-emit --shard 1/4',
        'head_overshoot': "git log --format=%H -1 -S'2026-09-13-p-025-measurement-contract-for-pulse \u00b7 complete' -- .andromeda/master-route.md  |  git rev-list --count {boundary_sha}..HEAD  |  git diff --numstat {boundary_sha}..HEAD filtered to crates/**/*.{rs,ts,tsx}",
    },
    'corrections': [{
        'target_sha': BASE_SHA, 'field': 'head_overshoot',
        'was': {'boundary_sha': 'b54e6ec', 'note': '(note only \u2014 commits and files absent; commands.head_overshoot null)'},
        'now': {'boundary_sha': 'b54e6ec', 'commits': 6,
                'files': [['crates/conductor-report/tests/matrix_ledger_gate.rs', 61, 6]]},
        'note': "Schema-gap fill per audit-pass.md \u00a7Schema: the Epoch 6b record carried a prose note but no structured commits/files and no commands.head_overshoot. Recomputed over b54e6ec..0f780c62 \u2014 6 commits, one source file \u2014 which CONFIRMS the stored note exactly. The note was accurate; only the machine-readable fields were missing. Prior record NOT edited.",
    }],
    'skips': skips,
}
json.dump(rec, open(f'{RD}/record.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

# ================= Phase 4: diff + judge =================
def gp(r, *ks):
    x = r
    for k in ks:
        x = (x or {}).get(k) if isinstance(x, dict) else None
    return x


TRACKED = [('duplication.pct', ('duplication', 'pct'), 'up'),
           ('complexity.over_ceiling', ('complexity', 'over_ceiling'), 'up'),
           ('dead.zero_ref_candidates', ('dead', 'zero_ref_candidates'), 'up'),
           ('sizes.file_max', ('sizes', 'file_max'), 'up'),
           ('sizes.over_800', ('sizes', 'over_800'), 'up'),
           ('coverage.line', ('coverage', 'line'), 'down')]
judge = {'span': 1, 'trend_break': [], 'fires': [], 'informational': [], 'below': [], 'monotonic': {}}

# trend-break — compare the leading VERSION TOKEN only. Two records spell the lizard entry with
# different trailing prose ("see recipes.lizard" vs "see skips.complexity-web"); that is an
# annotation, not a version, and a raw string compare reports a break on an UNCOLLECTED tool,
# suppressing thresholds for a metric that has no values.
vtok = lambda s: (re.match(r'\s*v?([0-9][0-9A-Za-z.+_-]*)', s or '') or [None, None])[1]
for k, v in TOOLS.items():
    bv = (base.get('tool_versions') or {}).get(k)
    if bv is None:
        continue
    if vtok(bv) != vtok(v):
        judge['trend_break'].append({'tool': k, 'baseline': bv, 'now': v})
    elif bv != v:
        judge['informational'].append({'check': 'tool_versions-prose-differs',
                                       'detail': f'{k}: version token identical ({vtok(v)}); only the trailing annotation differs — not a trend-break'})

d = lambda cur, b: None if (cur is None or b is None) else round(cur - b, 4)
D = {}
for name, path, worse in TRACKED + [('duplication.clones', ('duplication', 'clones'), 'up'),
                                    ('duplication.duplicated_lines', ('duplication', 'duplicated_lines'), 'up'),
                                    ('duplication.total_lines', ('duplication', 'total_lines'), 'pop'),
                                    ('graph.cycles', ('graph', 'cycles'), 'up'),
                                    ('graph.cross_unit_edges', ('graph', 'cross_unit_edges'), 'up'),
                                    ('totals.loc', ('totals', 'loc'), 'pop'),
                                    ('totals.files', ('totals', 'files'), 'pop'),
                                    ('dead_web.total', ('dead_web', 'total'), 'up')]:
    cur, b = gp(rec, *path), gp(base, *path)
    D[name] = {'now': cur, 'baseline': b, 'delta': d(cur, b), 'worse_dir': worse}
judge['deltas'] = D

# thresholds
c_now, c_base = gp(rec, 'graph', 'cycles'), gp(base, 'graph', 'cycles')
if c_now is not None and c_base is not None and c_now > c_base:
    judge['fires'].append({'check': 'new-cycle', 'detail': f'{c_base} -> {c_now}'})
dp, bp = gp(rec, 'duplication', 'pct'), gp(base, 'duplication', 'pct')
if dp - bp >= 0.5 and (dp - bp) / bp >= 0.15:
    judge['fires'].append({'check': 'duplication-up', 'detail': f'{bp:.3f} -> {dp:.3f}'})
oc, ob = gp(rec, 'complexity', 'over_ceiling'), gp(base, 'complexity', 'over_ceiling')
if oc - ob >= 3 and (ob and (oc - ob) / ob >= 0.25):
    judge['fires'].append({'check': 'complexity-creep', 'detail': f'{ob} -> {oc}'})
zr, zb = gp(rec, 'dead', 'zero_ref_candidates'), gp(base, 'dead', 'zero_ref_candidates')
if zr - zb >= 5:
    judge['fires'].append({'check': 'dead-growth', 'detail': f'{zb} -> {zr}'})
cl, cb = gp(rec, 'coverage', 'line'), gp(base, 'coverage', 'line')
if cl is not None and cb is not None and cl <= cb - 2:
    judge['fires'].append({'check': 'coverage-drop', 'detail': f'{cb} -> {cl}'})
# mutation-drop: a unit's score vs its LAST AUDITED score (walk records backwards by sha chain)
chain, cur_r = [], base
while cur_r is not None:
    chain.append(cur_r)
    nxt = [r for r in recs if r.get('sha') == cur_r.get('baseline_sha')]
    cur_r = nxt[-1] if nxt else None
last_score = {}
for r in reversed(chain):
    for u, s in (gp(r, 'mutation', 'scores') or {}).items():
        if s is not None:
            last_score[u] = (s, norm(r.get('epoch')))
for u, s in mut['scores'].items():
    if s is None:
        continue
    prev = last_score.get(u)
    if prev is None:
        judge['informational'].append({'check': 'mutation-first-measurement', 'detail': f'{u} = {s} (no prior score in the ledger)'})
    elif s <= prev[0] - 10:
        judge['fires'].append({'check': 'mutation-drop', 'detail': f'{u} {prev[0]} ({prev[1]}) -> {s}'})
    else:
        judge['informational'].append({'check': 'mutation-move', 'detail': f'{u} {prev[0]} ({prev[1]}) -> {s} (delta {round(s - prev[0], 2)})'})
judge['mutation_last_scores'] = {k: list(v) for k, v in last_score.items()}

# count-under-ratio
if D['duplication.clones']['delta'] is not None:
    judge['count_under_ratio'] = {
        'fires': bool(D['duplication.clones']['delta'] > 0 and D['duplication.pct']['delta'] <= 0),
        'clones_delta': D['duplication.clones']['delta'],
        'duplicated_lines_delta': D['duplication.duplicated_lines']['delta'],
        'pct_delta': D['duplication.pct']['delta'],
        'population_total_lines_rel': round(100 * D['duplication.total_lines']['delta'] / D['duplication.total_lines']['baseline'], 3),
        'split_now': rec['duplication']['split_pairs_only'],
    }

# monotonic: worsened at BOTH of the last two diffs, chained by sha
for name, path, worse in TRACKED:
    v0 = gp(base2, *path) if base2 else None
    v1, v2 = gp(base, *path), gp(rec, *path)
    if v0 is None or v1 is None or v2 is None:
        judge['monotonic'][name] = {'evaluable': False, 'reason': 'a tracked scalar is null at one of the three chained records', 'vals': [v0, v1, v2]}
        continue
    w1 = (v1 > v0) if worse == 'up' else (v1 < v0)
    w2 = (v2 > v1) if worse == 'up' else (v2 < v1)
    judge['monotonic'][name] = {'evaluable': True, 'vals': [v0, v1, v2], 'worse_diff1': w1, 'worse_diff2': w2, 'fires': bool(w1 and w2)}
    if w1 and w2:
        judge['fires'].append({'check': 'monotonic', 'detail': f'{name} {v0} -> {v1} -> {v2}'})

# top-N entrants
def names(lst, n=2):
    return set(tuple(x[:n]) for x in (lst or []))


judge['entrants'] = {
    'sizes': sorted(names(rec['sizes']['top'], 1) - names(base['sizes']['top'], 1)),
    'complexity': sorted(names(rec['complexity']['top'], 2) - names(base['complexity']['top'], 2)),
    'duplication_pairs': sorted(set(frozenset(x[:2]) for x in rec['duplication']['top']) - set(frozenset(x[:2]) for x in base['duplication']['top'])),
    'fan_in': sorted(names(rec['graph']['fan_in_top'], 1) - names(base['graph']['fan_in_top'], 1)),
    'hotspots': sorted(names(rec['hotspots'], 1) - names(base['hotspots'], 1)),
}
judge['entrants']['duplication_pairs'] = [sorted(p) for p in judge['entrants']['duplication_pairs']]
json.dump(judge, open(f'{RD}/judge.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

print('=== RECORD assembled ===')
print(f"  loc {rec['totals']['loc']} files {rec['totals']['files']} | dup {rec['duplication']['pct']:.3f}% | cx over {oc} | dead {zr} | cov {cl} | cycles {c_now}")
print(f"  mutation units: {mut['scoped_units']}")
print(f"  scores: {mut['scores']}")
print(f"  survivors: {len(mut['survivors'])}")
print(f"  skips: {len(skips)}")
print('=== JUDGE ===')
print(f"  trend_break: {judge['trend_break']}")
print(f"  FIRES ({len(judge['fires'])}): {judge['fires']}")
print(f"  count_under_ratio: {judge.get('count_under_ratio')}")
print('  monotonic:')
for k, v in judge['monotonic'].items():
    print(f"    {k}: {v}")
print(f"  informational ({len(judge['informational'])}):")
for i in judge['informational']:
    print(f"    {i}")
print('  entrants:')
for k, v in judge['entrants'].items():
    print(f"    {k}: {v}")
