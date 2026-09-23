"""Pinned summarizers for this code-audit run (collectors.md A1-A6, B1-B2).
Run from the repo root: python -X utf8 {run_dir}/summarize.py {run_dir} {baseline_sha}
Writes {run_dir}/c-{metric}.json only."""
import json, os, re, subprocess, sys, collections

R, BASE = sys.argv[1], sys.argv[2]
def out(name, obj):
    with open(os.path.join(R, 'c-%s.json' % name), 'w', encoding='utf-8', newline='') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
def nr(vals, p):  # nearest-rank percentile
    s = sorted(vals)
    if not s: return None
    k = max(1, -(-p * len(s) // 100))
    return s[int(k) - 1]
def is_test(path):
    return '/tests/' in '/' + path.replace('\\', '/')

# A3 sizes -- tokei, language Rust only, crates/
tk = json.load(open(os.path.join(R, '_tokei.json'), encoding='utf-8'))
files = [(r['name'].replace('\\', '/'), r['stats']['code']) for r in tk['Rust']['reports']]
files = [('crates/' + p.split('crates/', 1)[1] if 'crates/' in p else p, c) for p, c in files]
locs = [c for _, c in files]
top = sorted(files, key=lambda x: -x[1])[:10]
out('sizes', {"population": "tokei --output json crates/ ; language=Rust (*.rs) only",
              "file_p50": nr(locs, 50), "file_p90": nr(locs, 90), "file_max": max(locs),
              "over_800": sum(c > 800 for c in locs), "top": [list(t) for t in top],
              "totals": {"loc": sum(locs), "files": len(locs), "units": 9}})

# A1 duplication -- jscpd rust
jd = json.load(open(os.path.join(R, 'jscpd_rs', 'jscpd-report.json'), encoding='utf-8'))
tot = jd['statistics']['total']
def rel(n): n = n.replace('\\', '/'); return n.split('crates/', 1)[1] if 'crates/' in n else n
pairs, split = [], {k: {"pairs": 0, "lines": 0} for k in ('src', 'test', 'mixed')}
for d in jd['duplicates']:
    a, b, ln = rel(d['firstFile']['name']), rel(d['secondFile']['name']), d['lines']
    pairs.append([a, b, ln])
    ta, tb = is_test(a), is_test(b)
    k = 'test' if ta and tb else ('src' if not ta and not tb else 'mixed')
    split[k]['pairs'] += 1; split[k]['lines'] += ln
pairs.sort(key=lambda x: (-x[2], x[0], x[1]))
out('duplication', {"population": "jscpd --format rust over crates/ (Rust only)",
    "pct": tot['percentage'], "duplicated_lines": tot['duplicatedLines'], "total_lines": tot['lines'],
    "clones": tot['clones'], "sources": tot['sources'], "split": split,
    "split_pairs_only": {k: v['pairs'] for k, v in split.items()}, "top": pairs[:10],
    "all_pairs": pairs})

# A2 complexity -- rca, kind==function, *.rs
fns = []
for root, _, fs in os.walk(os.path.join(R, '_rca_head')):
    for f in fs:
        if not f.endswith('.rs.json'): continue
        p = os.path.join(root, f)
        relp = rel(os.path.relpath(p, os.path.join(R, '_rca_head'))[:-5])
        d = json.load(open(p, encoding='utf-8'))
        st = [d]
        while st:
            s = st.pop()
            if s.get('kind') == 'function':
                m = s['metrics']
                fns.append((s['name'], relp, m['cognitive']['sum'], m['cyclomatic']['sum']))
            st.extend(s.get('spaces', []))
cog = [f[2] for f in fns]; cyc = [f[3] for f in fns]
topc = sorted(fns, key=lambda x: (-x[2], -x[3], x[1], x[0]))
per_file = collections.defaultdict(float)
for n, p, c, y in fns: per_file['crates/' + p] = max(per_file['crates/' + p], c)
out('complexity', {"population": "rust-code-analysis kind==function spaces under crates/; ceiling cognitive > 15; top sorted by COGNITIVE desc",
    "cyclomatic_p50": nr(cyc, 50), "cyclomatic_p90": nr(cyc, 90), "cognitive_p50": nr(cog, 50), "cognitive_p90": nr(cog, 90),
    "over_ceiling": sum(c > 15 for c in cog), "over_ceiling_list": [list(t) for t in topc if t[2] > 15],
    "max": {"fn": topc[0][0], "file": topc[0][1], "val": topc[0][2]}, "functions": len(fns),
    "rs_files_scanned": len({f[1] for f in fns} | set()), "top": [list(t) for t in topc[:10]],
    "per_file_max_cognitive": per_file})

# A6 coverage -- lcov totals
lf = lh = 0
for line in open(os.path.join(R, '_lcov.info'), encoding='utf-8'):
    if line.startswith('LF:'): lf += int(line[3:])
    elif line.startswith('LH:'): lh += int(line[3:])
out('coverage', {"line": round(100.0 * lh / lf, 1), "branch": None, "lines_found": lf, "lines_hit": lh,
                 "population": "cargo llvm-cov nextest --workspace --lcov; totals summed over all records"})

# A4 graph
def q(n): return json.load(open(os.path.join(R, '_g_%s.txt' % n), encoding='utf-8'))
PFX = re.compile(r'^rust-analyzer cargo (\S+) \S+ ')
def short(s): return PFX.sub(lambda m: m.group(1) + ' ', s)
cyc_rows = q('cycles')
out('graph', {"population": "plane=rust tree.db; views symbol/refs/calls_m/crate_edges",
    "symbol_form": "fan_in symbols shortened to `{crate} {path}` (baseline-pinned)",
    "cycles": len(cyc_rows), "cycle_paths": [r['path'] for r in cyc_rows],
    "fan_in_top": [[short(r['callee']), r['n']] for r in q('fanin')],
    "fan_out": [[r['from_crate'], r['n']] for r in q('fanout')],
    "cross_unit_edges": q('edges')[0]['count_star()']})

# A5 dead -- pinned classing chain
raw = q('dead')
def sympath(s): return PFX.sub('', s)
seg = lambda p: p.startswith('tests/') or '/tests/' in p
step1 = [r for r in raw if not (seg(sympath(r['symbol'])) or seg(r['file'].replace('\\', '/')))]
step2 = [r for r in step1 if not sympath(r['symbol']).endswith('main().')]
knip = json.load(open(os.path.join(R, '_knip.json'), encoding='utf-8'))
kc = collections.Counter(); kf = 0
for iss in knip.get('issues', []):
    hit = False
    for k, v in iss.items():
        if isinstance(v, list) and v: kc[k] += len(v); hit = True
    kf += hit
out('dead', {"population": "plane=rust tree.db zero-ref public symbols; pinned classing chain",
    "unused_deps": {}, "zero_ref_candidates": len(step2),
    "chain": {"raw": len(raw), "after_tests_segment_filter": len(step1), "after_entry_points": len(step2)},
    "top": [[short(r['symbol']), r['file'].replace('\\', '/')] for r in step2][:20],
    "all_candidates": [[short(r['symbol']), r['file'].replace('\\', '/')] for r in step2],
    "dead_web": {"tool": "knip 6.34.0", "counts": dict(kc), "total": sum(kc.values()), "files_with_issues": kf}})

# B1 churn + B2 hotspots
log = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=commit %H', BASE + '..HEAD', '--',
                      'crates/**/*.rs', 'crates/**/*.ts', 'crates/**/*.tsx'],
                     capture_output=True, text=True, encoding='utf-8').stdout
seen = collections.Counter(); adds = churned = 0
for line in log.splitlines():
    parts = line.split('\t')
    if len(parts) != 3 or parts[0] == '-': continue
    a, path = int(parts[0]), parts[2]
    if '=>' in path:
        path = re.sub(r'\{[^{}]*=> ([^{}]*)\}', r'\1', path).replace('//', '/')
        if ' => ' in path: path = path.split(' => ')[1]
    if seen[path] >= 1: churned += a
    seen[path] += 1; adds += a
out('churn', {"population": "git log --numstat %s..HEAD -- crates/**/*.rs crates/**/*.ts crates/**/*.tsx" % BASE[:8],
    "pct": round(100.0 * churned / adds, 2) if adds else 0.0, "files_churned": sum(1 for v in seen.values() if v > 1),
    "files_touched": len(seen), "total_adds": adds, "churned_adds": churned, "commits_per_file": dict(seen)})
hs = sorted(((p, n * per_file.get(p, 0.0)) for p, n in seen.items()), key=lambda x: (-x[1], x[0]))
out('hotspots', {"formula": "commits(churn window) x max cognitive per file over the FULL rca population (fallback 0 for non-Rust)",
                 "top": [list(h) for h in hs[:10]]})
print('ok', len(fns), 'fns', len(raw), 'raw dead', len(step2), 'cands')
