"""Tier A summarizers (pinned per collectors.md): sizes, complexity, duplication, unused deps, web dead code, web complexity."""
import csv, json, math, os, sys

RD = sys.argv[1]
J = lambda name, obj: json.dump(obj, open(os.path.join(RD, f'c-{name}.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)


def nr(vals, p):  # nearest-rank percentile on sorted values
    v = sorted(vals)
    return v[max(1, math.ceil(p / 100 * len(v))) - 1] if v else None


def in_pop(path):  # Rust source population: tracked .rs under crates/, never node_modules / target
    return path.startswith('crates/') and path.endswith('.rs') and '/node_modules/' not in path and '/target/' not in path


# ---- A3 sizes: tokei, language Rust only, crates/
tk = json.load(open(os.path.join(RD, '_tokei.json'), encoding='utf-8'))
files = [(r['name'].replace('\\', '/'), r['stats']['code']) for r in tk['Rust']['reports']]
files = [(p if p.startswith('crates/') else 'crates/' + p.split('crates/', 1)[-1], c) for p, c in files]
files = [(p, c) for p, c in files if in_pop(p)]
codes = [c for _, c in files]
top = sorted(files, key=lambda x: -x[1])[:10]
J('sizes', {'population': 'tokei --output json crates/ ; language=Rust (*.rs) only, node_modules/target excluded',
            'totals': {'loc': sum(codes), 'files': len(files), 'units': 9},
            'file_p50': nr(codes, 50), 'file_p90': nr(codes, 90), 'file_max': max(codes), 'over_800': sum(c > 800 for c in codes),
            'top': [[p, c] for p, c in top]})

# ---- A2 complexity: rust-code-analysis, kind==function spaces, ceiling cognitive > 15
fns, scanned = [], 0
base = os.path.join(RD, '_rca_head')
for root, _, fs in os.walk(base):
    for f in fs:
        if not f.endswith('.rs.json'): continue
        rel = os.path.relpath(os.path.join(root, f), base).replace('\\', '/')[:-5]
        if not in_pop(rel): continue
        d = json.load(open(os.path.join(root, f), encoding='utf-8'))
        before = len(fns)
        def walk(s):
            if s['kind'] == 'function':
                fns.append((s['name'], rel[len('crates/'):], s['metrics']['cognitive']['sum'], s['metrics']['cyclomatic']['sum']))
            for c in s.get('spaces', []): walk(c)
        walk(d)
        scanned += len(fns) > before
cog = [f[2] for f in fns]; cyc = [f[3] for f in fns]
topc = sorted(fns, key=lambda f: (-f[2], -f[3], f[1], f[0]))[:10]
mx = topc[0]
J('complexity', {'population': 'rust-code-analysis kind==function spaces under crates/ (node_modules/target excluded); ceiling cognitive > 15; top sorted by COGNITIVE desc',
                 'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90), 'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
                 'over_ceiling': sum(c > 15 for c in cog), 'max': {'fn': mx[0], 'file': mx[1], 'val': mx[2]},
                 'functions': len(fns), 'rs_files_scanned': scanned,
                 'over_ceiling_list': [[f[0], f[1], f[2]] for f in sorted(fns, key=lambda f: -f[2]) if f[2] > 15],
                 'top': [[f[0], f[1], f[2], f[3]] for f in topc]})
# per-file max cognitive for hotspots
pf = {}
for n, fl, c, y in fns: pf['crates/' + fl] = max(pf.get('crates/' + fl, 0), c)
json.dump(pf, open(os.path.join(RD, '_file_maxcog.json'), 'w'), indent=0)

# ---- A1 duplication: jscpd report
jr = json.load(open(os.path.join(RD, 'jscpd_rs', 'jscpd-report.json'), encoding='utf-8'))
tot = jr['statistics']['total']
is_test = lambda n: '/tests/' in '/' + n
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
dups = []
for d in jr['duplicates']:
    a, b, n = d['firstFile']['name'].replace('\\', '/'), d['secondFile']['name'].replace('\\', '/'), d['lines']
    k = 'test' if is_test(a) and is_test(b) else 'src' if not is_test(a) and not is_test(b) else 'mixed'
    split[k]['pairs'] += 1; split[k]['lines'] += n
    dups.append([a, b, n, d['firstFile']['start'], d['secondFile']['start']])
dups.sort(key=lambda x: (-x[2], x[0], x[1], x[3]))
J('duplication', {'population': 'jscpd --format rust over crates/ (Rust only)', 'pct': tot['percentage'],
                  'duplicated_lines': tot['duplicatedLines'], 'total_lines': tot['lines'], 'clones': tot['clones'],
                  'sources': tot['sources'], 'split': split, 'top': [x[:3] for x in dups[:10]],
                  'top_with_starts': dups[:10]})

# ---- A5 unused deps: cargo machete
mo = open(os.path.join(RD, '_machete.out'), encoding='utf-8').read()
J('dead-deps', {'unused_deps': {} if "didn't find any unused dependencies" in mo else {'raw_tail': mo[-600:]}})

# ---- A5 web dead code: knip
try:
    kn = json.load(open(os.path.join(RD, '_knip.json'), encoding='utf-8'))
    counts, fwi = {}, 0
    for issue in kn.get('issues', []):
        had = False
        for k, v in issue.items():
            if isinstance(v, list) and v:
                counts[k] = counts.get(k, 0) + len(v); had = True
            elif isinstance(v, dict) and v:
                counts[k] = counts.get(k, 0) + len(v); had = True
        fwi += had
    for k in ('files',):
        if isinstance(kn.get(k), list) and kn[k]: counts[k] = len(kn[k])
    J('dead-web', {'tool': 'knip', 'counts': counts, 'total': sum(counts.values()), 'files_with_issues': fwi})
except Exception as e:
    J('dead-web', {'error': type(e).__name__})

# ---- A2 web complexity: lizard (no cognitive; cyclomatic ceiling 15)
rows = list(csv.reader(open(os.path.join(RD, '_lizard.csv'), encoding='utf-8')))
ccn = [(int(r[1]), r[7], r[6]) for r in rows if len(r) > 7]
cv = [c[0] for c in ccn]
J('complexity-web', {'population': 'lizard (auto-detect .ts/.tsx) over crates/conductor-tauri/ui/src (no cognitive: field null)',
                     'functions': len(cv), 'cyclomatic_p50': nr(cv, 50), 'cyclomatic_p90': nr(cv, 90),
                     'cognitive_p50': None, 'cognitive_p90': None, 'over_ceiling': sum(c > 15 for c in cv),
                     'top': [[n, f.replace('\\', '/'), c] for c, n, f in sorted(ccn, key=lambda x: -x[0])[:10]]})
print('ok')
