"""Phase-1 summarizers. Populations and shapes pinned to the Epoch 6b baseline record."""
import json, os, re, glob, collections

RD = 'D:/dev/projects/conductor/.andromeda/runs/2026-09-14T18-51-54-code-audit'
ROOT = 'D:/dev/projects/conductor'


def w(name, obj):
    json.dump(obj, open(f'{RD}/c-{name}.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    return obj


def pct_rank(vals, q):
    """Nearest-rank percentile on the sorted values (pinned)."""
    if not vals:
        return None
    s = sorted(vals)
    import math
    k = max(1, math.ceil(q * len(s)))
    return s[k - 1]


# ============ A3 sizes: tokei --output json crates/ ; language=Rust only ============
tk = json.load(open(f'{RD}/_tokei.json', encoding='utf-8'))
rust = tk.get('Rust') or {}
rows = []
for rep in (rust.get('reports') or []):
    p = rep['name'].replace('\\', '/')
    rows.append((p, rep['stats']['code']))
loc = sum(n for _, n in rows)
vals = [n for _, n in rows]
sizes = w('sizes', {
    'population': 'tokei --output json crates/ ; language=Rust (*.rs) only',
    'file_p50': pct_rank(vals, .50), 'file_p90': pct_rank(vals, .90),
    'file_max': max(vals) if vals else None,
    'over_800': sum(1 for v in vals if v > 800),
    'top': [[p, n] for p, n in sorted(rows, key=lambda r: -r[1])[:10]],
    'totals': {'loc': loc, 'files': len(rows)},
})
print(f"sizes: loc={loc} files={len(rows)} p50={sizes['file_p50']} p90={sizes['file_p90']} max={sizes['file_max']} over800={sizes['over_800']}")

# ============ A1 duplication ============
jd = json.load(open(f'{RD}/jscpd_rs/jscpd-report.json', encoding='utf-8'))
t = jd['statistics']['total']
pairs = []
for x in jd.get('duplicates', []):
    a = x['firstFile']['name'].replace('\\', '/')
    b = x['secondFile']['name'].replace('\\', '/')
    pairs.append((a, b, x['lines']))


def isteset(p):
    segs = p.split('/')
    return 'tests' in segs or p.endswith('_test.rs')


split = {'src': {'pairs': 0, 'lines': 0}, 'test': {'pairs': 0, 'lines': 0}, 'mixed': {'pairs': 0, 'lines': 0}}
for a, b, n in pairs:
    ta, tb = isteset(a), isteset(b)
    k = 'test' if (ta and tb) else ('src' if not (ta or tb) else 'mixed')
    split[k]['pairs'] += 1
    split[k]['lines'] += n
dup = w('duplication', {
    'label': 'head',
    'population': 'jscpd --format rust over crates/ (Rust only)',
    'pct': t['percentage'], 'duplicated_lines': t['duplicatedLines'],
    'total_lines': t['lines'], 'clones': t['clones'], 'sources': t['sources'],
    'split': split,
    'split_pairs_only': {k: v['pairs'] for k, v in split.items()},
    'top': [[a, b, n] for a, b, n in sorted(pairs, key=lambda r: -r[2])[:10]],
    'note': "baseline recorded split as bare pair COUNTS; collectors.md pins {pairs,lines} -- both emitted, split_pairs_only is the comparable half",
})
print(f"duplication: pct={dup['pct']:.4f} dl={dup['duplicated_lines']} tl={dup['total_lines']} clones={dup['clones']} split={dup['split_pairs_only']}")

# ============ A2 complexity: rca kind==function under crates/, *.rs only ============
fns = []
files_scanned = 0
for f in glob.glob(f'{RD}/_rca_head/**/*.json', recursive=True):
    fp = f.replace('\\', '/')
    if not fp.endswith('.rs.json'):
        continue
    try:
        d = json.load(open(f, encoding='utf-8'))
    except Exception:
        continue
    name = (d.get('name') or '').replace('\\', '/')
    if '/crates/' not in name and not name.startswith('crates/'):
        continue
    rel = name.split('crates/', 1)[1] if 'crates/' in name else name
    files_scanned += 1
    stack = list(d.get('spaces') or [])
    while stack:
        s = stack.pop()
        stack.extend(s.get('spaces') or [])
        if s.get('kind') != 'function':
            continue
        m = s.get('metrics') or {}
        cy = (m.get('cyclomatic') or {}).get('sum')
        cg = (m.get('cognitive') or {}).get('sum')
        fns.append((s.get('name'), rel, cg if cg is not None else 0.0, cy if cy is not None else 0.0))
cys = [x[3] for x in fns]
cgs = [x[2] for x in fns]
over = sum(1 for x in fns if x[2] > 15)
top = sorted(fns, key=lambda x: -x[2])[:10]
cx = w('complexity', {
    'population': 'rust-code-analysis kind==function spaces under crates/; ceiling cognitive > 15; top sorted by COGNITIVE desc',
    'cyclomatic_p50': pct_rank(cys, .50), 'cyclomatic_p90': pct_rank(cys, .90),
    'cognitive_p50': pct_rank(cgs, .50), 'cognitive_p90': pct_rank(cgs, .90),
    'over_ceiling': over,
    'max': {'fn': top[0][0], 'file': top[0][1], 'val': top[0][2]} if top else None,
    'functions': len(fns), 'rs_files_scanned': files_scanned,
    'top': [[a, b, c, d_] for a, b, c, d_ in top],
})
print(f"complexity: fns={len(fns)} files={files_scanned} cy_p50={cx['cyclomatic_p50']} cy_p90={cx['cyclomatic_p90']} cg_p90={cx['cognitive_p90']} over={over} max={cx['max']}")
# per-file max cognitive for hotspots
file_cg = collections.defaultdict(float)
for _, rel, cg, cy in fns:
    file_cg[f'crates/{rel}'] = max(file_cg[f'crates/{rel}'], cg if cg else (cy or 0))

# ============ A4 graph ============
pref = re.compile(r'^rust-analyzer cargo (\S+) (\S+) ')


def shorten(s):
    m = pref.match(s)
    return f'{m.group(1)} {s[m.end():]}' if m else s


fanin = json.load(open(f'{RD}/_fanin.json', encoding='utf-8'))
fanout = json.load(open(f'{RD}/_fanout.json', encoding='utf-8'))
cycles = json.load(open(f'{RD}/_cycles.json', encoding='utf-8'))
graph = w('graph', {
    'population': 'plane=rust tree.db; views symbol/refs/calls_m/crate_edges',
    'symbol_form': 'fan_in symbols shortened to `{crate} {path}` (baseline-pinned)',
    'cycles': len(cycles), 'cycle_paths': [c.get('path') for c in cycles],
    'fan_in_top': [[shorten(r['callee']), r['n']] for r in fanin],
    'fan_out': [[r['from_crate'], r['n']] for r in fanout],
    'cross_unit_edges': 16,
})
print(f"graph: cycles={graph['cycles']} edges=16 fan_in_max={graph['fan_in_top'][0] if graph['fan_in_top'] else None}")

# ============ A5 dead ============
zr = json.load(open(f'{RD}/_zeroref.json', encoding='utf-8'))


def segs(p):
    return p.replace('\\', '/').split('/')


def sympath(s):
    m = pref.match(s)
    return s[m.end():] if m else s


# PINNED chain, reproduced from the baseline record's own numbers (898 -> 38 -> 34):
# the tests/ exclusion is the UNION of a SYMBOL-path segment and a FILE-path segment.
after_tests = [r for r in zr if 'tests' not in segs(sympath(r['symbol'])) and 'tests' not in segs(r['file'])]
ENTRY = re.compile(r'(^|[/#\]])main\(\)\.$')
after_entry = [r for r in after_tests if not ENTRY.search(sympath(r['symbol']))]
mach = open(f'{RD}/_machete.txt', encoding='utf-8', errors='replace').read()
unused_deps = {} if "didn't find any unused dependencies" in mach else {'RAW': mach[:400]}
dead = w('dead', {
    'population': 'plane=rust tree.db zero-ref public symbols; pinned classing chain',
    'unused_deps': unused_deps,
    'zero_ref_candidates': len(after_entry),
    'chain': {'raw': len(zr), 'after_tests_segment_filter': len(after_tests), 'after_entry_points': len(after_entry)},
    'filter_note': 'tests/ exclusion applied as the UNION of symbol-path and file-path segments -- symbol-path alone leaves 240 and does not reproduce the baseline chain step (38)',
    'top': [[shorten(r['symbol']), r['file'].replace('\\', '/')] for r in after_entry[:20]],
    'full_candidates': [[shorten(r['symbol']), r['file'].replace('\\', '/')] for r in after_entry],
})
print(f"dead: chain {len(zr)} -> {len(after_tests)} -> {len(after_entry)} | unused_deps={unused_deps}")

# ============ A5b dead web (knip) ============
kn = json.load(open(f'{RD}/_knip.json', encoding='utf-8'))
issues = kn.get('issues') or []
cnt = collections.Counter()
detail = []
for it in issues:
    fpath = it.get('file')
    for k, v in it.items():
        if isinstance(v, list) and v and k != 'file':
            cnt[k] += len(v)
            for e in v:
                nm = e.get('name') if isinstance(e, dict) else str(e)
                detail.append([k, fpath, nm])
dw = w('deadweb', {'population': 'npx knip --reporter json (cwd crates/conductor-tauri/ui)',
                   'by_kind': dict(cnt), 'total': sum(cnt.values()), 'items': detail[:30]})
print(f"deadweb: total={dw['total']} by_kind={dw['by_kind']}")

# ============ A6 coverage (lcov) ============
lf = lh = bf = bh = 0
for line in open(f'{RD}/_lcov.info', encoding='utf-8', errors='replace'):
    if line.startswith('LF:'):
        lf += int(line[3:])
    elif line.startswith('LH:'):
        lh += int(line[3:])
    elif line.startswith('BRF:'):
        bf += int(line[4:])
    elif line.startswith('BRH:'):
        bh += int(line[4:])
cov = w('coverage', {
    'line': round(100 * lh / lf, 1) if lf else None,
    'branch': round(100 * bh / bf, 1) if bf else None,
    'lines_found': lf, 'lines_hit': lh,
    'population': 'cargo llvm-cov nextest --workspace --lcov; totals summed over all records',
})
print(f"coverage: line={cov['line']}% ({lh}/{lf}) branch={cov['branch']}")

# ============ B1 churn / B2 hotspots ============
commits = collections.Counter()
adds = collections.Counter()
order = collections.defaultdict(list)
cur = None
for line in open(f'{RD}/_churn.txt', encoding='utf-8', errors='replace'):
    line = line.rstrip('\n')
    if line.startswith('commit '):
        cur = line[7:]
        continue
    if not line.strip():
        continue
    p = line.split('\t')
    if len(p) != 3:
        continue
    a, d_, path = p
    if a == '-':
        continue  # binary
    if '=>' in path:  # rename -> new path
        path = re.sub(r'.*\{.*=> *(.*)\}', r'\1', path) if '{' in path else path.split('=>')[-1].strip()
    path = path.strip().replace('\\', '/')
    commits[path] += 1
    order[path].append(int(a))
total_adds = sum(sum(v) for v in order.values())
churned_adds = sum(sum(v[1:]) for v in order.values())
files_churned = sum(1 for v in order.values() if len(v) > 1)
churn = w('churn', {
    'population': 'git log --numstat 0f780c62..HEAD -- crates/**/*.rs crates/**/*.ts crates/**/*.tsx',
    'pct': round(100 * churned_adds / total_adds, 2) if total_adds else None,
    'files_churned': files_churned, 'files_touched': len(order),
    'total_adds': total_adds, 'churned_adds': churned_adds,
})
print(f"churn: pct={churn['pct']} churned={files_churned}/{len(order)} adds={churned_adds}/{total_adds}")
hs = sorted(((f, commits[f] * (file_cg.get(f) or 0.0)) for f in commits), key=lambda r: -r[1])[:10]
w('hotspots', {'population': 'commits(churn window) x max cognitive per file over the FULL rca population',
               'top': [[f, s] for f, s in hs]})
print(f"hotspots: top={hs[:3]}")
