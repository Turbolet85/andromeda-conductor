"""Pinned Tier A/B summarizers for the code audit (run by path; writes c-{metric}.json under the run dir).
Populations mirror the Epoch-5 record's `recipes` so run-to-run numbers stay comparable."""
import json, os, re, sys, glob, collections
RUN = sys.argv[1]
BASE = json.load(open(sys.argv[2], encoding='utf-8'))  # baseline record (prior ledger line), for entrants
def out(name, obj):
    json.dump(obj, open(os.path.join(RUN, f'c-{name}.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
def nr(vals, q):  # nearest-rank percentile on sorted values
    if not vals: return None
    s = sorted(vals); k = max(1, int(round(q * len(s) + 0.5)))
    return s[min(k, len(s)) - 1]
def norm(p):
    p = p.replace('\\', '/')
    i = p.find('crates/')
    return p[i:] if i >= 0 else p
def has_tests_seg(path):
    return 'tests' in [seg for seg in path.split('/') if seg]

# ---------- A3 sizes (tokei, Rust *.rs only) ----------
tk = json.load(open(os.path.join(RUN, '_tokei.json'), encoding='utf-8'))
rust = tk.get('Rust', {})
files = [(norm(r['name']), r['stats']['code']) for r in rust.get('reports', [])]
codes = [c for _, c in files]
sizes = {'population': 'tokei --output json crates/ ; language=Rust (*.rs) only',
         'file_p50': nr(codes, 0.5), 'file_p90': nr(codes, 0.9), 'file_max': max(codes) if codes else None,
         'over_800': sum(1 for c in codes if c > 800),
         'top': sorted(files, key=lambda x: -x[1])[:10]}
totals = {'loc': sum(codes), 'files': len(files), 'units': 9}
out('sizes', {'sizes': sizes, 'totals': totals})
file_kloc = {f: c / 1000.0 for f, c in files}

# ---------- A1 duplication (jscpd, Rust) ----------
jr = json.load(open(os.path.join(RUN, 'jscpd_rs', 'jscpd-report.json'), encoding='utf-8'))
st = jr['statistics']['total']
def short(p):
    p = norm(p); return p[len('crates/'):] if p.startswith('crates/') else p
pairs = []
for d in jr.get('duplicates', []):
    a, b = short(d['firstFile']['name']), short(d['secondFile']['name'])
    pairs.append([a, b, d.get('lines', 0)])
split = collections.defaultdict(lambda: {'pairs': 0, 'lines': 0})
for a, b, n in pairs:
    ta, tb = has_tests_seg(a), has_tests_seg(b)
    k = 'both_test' if ta and tb else ('both_src' if not ta and not tb else 'mixed')
    split[k]['pairs'] += 1; split[k]['lines'] += n
top = sorted(pairs, key=lambda x: -x[2])[:10]
base_pairs = {(x[0], x[1]) for x in BASE['duplication']['top']} | {(x[1], x[0]) for x in BASE['duplication']['top']}
new_top = [p for p in top if (p[0], p[1]) not in base_pairs]
out('duplication', {'label': 'head', 'population': 'jscpd --format rust over crates/ (Rust only)',
                    'pct': st.get('percentage'), 'duplicated_lines': st.get('duplicatedLines'),
                    'total_lines': st.get('totalLines', st.get('lines')), 'statistics_total_keys': sorted(st.keys()),
                    'clones': st.get('clones'), 'sources': st.get('sources'), 'top': top, 'split': dict(split),
                    'new_in_top_vs_baseline': new_top})

# ---------- A2 complexity (rust-code-analysis, kind==function, .rs only) ----------
fns = []  # (name, file, cognitive, cyclomatic)
file_max_cog, file_max_cyc = {}, {}
def walk(space, file):
    if space.get('kind') == 'function':
        m = space.get('metrics', {})
        cog = (m.get('cognitive') or {}).get('sum', 0.0) or 0.0
        cyc = (m.get('cyclomatic') or {}).get('sum', 0.0) or 0.0
        fns.append((space.get('name') or '?', file, cog, cyc))
        file_max_cog[file] = max(file_max_cog.get(file, 0.0), cog)
        file_max_cyc[file] = max(file_max_cyc.get(file, 0.0), cyc)
    for s in space.get('spaces', []) or []: walk(s, file)
nfiles = 0
for fp in glob.glob(os.path.join(RUN, '_rca_head', '**', '*.json'), recursive=True):
    try: j = json.load(open(fp, encoding='utf-8'))
    except Exception: continue
    name = norm(j.get('name', fp))
    if not name.endswith('.rs'): continue
    nfiles += 1; walk(j, name)
cogs = [f[2] for f in fns]; cycs = [f[3] for f in fns]
topc = sorted(fns, key=lambda f: (-f[2], -f[3]))[:10]
mx = topc[0] if topc else None
out('complexity', {'population': 'rust-code-analysis kind==function spaces under crates/; ceiling cognitive > 15; top sorted by COGNITIVE desc',
                   'cyclomatic_p50': nr(cycs, 0.5), 'cyclomatic_p90': nr(cycs, 0.9), 'cognitive_p50': nr(cogs, 0.5), 'cognitive_p90': nr(cogs, 0.9),
                   'over_ceiling': sum(1 for c in cogs if c > 15), 'max': {'fn': mx[0], 'file': mx[1], 'val': mx[2]} if mx else None,
                   'top': [list(t) for t in topc], 'functions': len(fns), 'rs_files_scanned': nfiles,
                   'over_ceiling_list': [list(f) for f in fns if f[2] > 15]})

# ---------- A4 graph + A5 dead (from the code-graph trace) ----------
trace = []
tp = os.path.join(RUN, 'tree-query-code-audit.json')
raw = open(tp, encoding='utf-8').read() if os.path.exists(tp) else ''
try:
    trace = json.loads(raw) if raw.lstrip().startswith('[') else [json.loads(l) for l in raw.splitlines() if l.strip()]
except Exception as e:
    print('trace parse problem', e)
def rows_for(tag):
    for r in trace:
        if tag in (r.get('sql') or ''):
            res = r.get('result')
            if res is None: res = r.get('rows_data') or []
            if isinstance(res, dict): res = [res]
            return [list(x.values()) if isinstance(x, dict) else list(x) for x in res]
    return None
def shorten(sym):
    m = re.match(r'^rust-analyzer cargo (\S+) (\S+) (.*)$', sym)
    return f'{m.group(1)} {m.group(3)}' if m else sym
cyc_rows = rows_for('WITH RECURSIVE walk') or []
fanin = rows_for('GROUP BY callee ORDER BY n DESC LIMIT 20') or []
fanout = rows_for('GROUP BY from_crate ORDER BY n DESC') or []
edges = rows_for('count(*) FROM crate_edges') or []
graph = {'population': 'plane=rust tree.db; views symbol/refs/calls_m/crate_edges',
         'symbol_form': 'fan_in symbols shortened to `{crate} {path}` (baseline-pinned)',
         'cycles': len(cyc_rows), 'cycle_paths': [list(r) if isinstance(r, (list, tuple)) else r for r in cyc_rows],
         'fan_in_top': [[shorten(r[0]), r[1]] for r in fanin],
         'fan_out': [[r[0], r[1]] for r in fanout],
         'cross_unit_edges': (edges[0][0] if edges and isinstance(edges[0], (list, tuple)) else edges[0]) if edges else None}
base_fanin = {x[0] for x in BASE['graph']['fan_in_top']}
graph['fan_in_entrants'] = [x for x in graph['fan_in_top'] if x[0] not in base_fanin]
out('graph', graph)
zr = rows_for('WHERE r.callee IS NULL') or []
raw_n = len(zr)
after_tests = []
for sym, file in zr:
    stripped = re.sub(r'^rust-analyzer cargo \S+ \S+ ', '', sym)
    if has_tests_seg(stripped) or has_tests_seg(norm(file)): continue
    after_tests.append((sym, file))
after_entry = [(s, f) for s, f in after_tests if not (s.endswith('main().') or '/bin/' in norm(f))]
mach = open(os.path.join(RUN, '_machete.txt'), encoding='utf-8', errors='replace').read()
unused = {}
for m in re.finditer(r'^(\S+) -- .*?:\s*\n((?:\t\S+\n)+)', mach, re.M):
    unused[m.group(1)] = [d.strip() for d in m.group(2).splitlines() if d.strip()]
out('dead', {'unused_deps': unused, 'zero_ref_candidates': len(after_entry),
             'chain': {'raw': raw_n, 'after_tests_segment_filter': len(after_tests), 'after_entry_points': len(after_entry)},
             'top': [[s, norm(f)] for s, f in after_entry[:20]],
             'all_candidates': [[s, norm(f)] for s, f in after_entry],
             'false_positive_families_named_not_subtracted': ['trait-impl methods reached by dispatch',
                 'derive/attr-invoked fns (serde defaults, #[from] variants)', 'runtime-invoked surfaces (MCP tools, #[tauri::command] IPC)',
                 'test-only helpers outside a tests/ path segment']})

# ---------- B1 churn + B2 hotspots ----------
per_file = collections.OrderedDict()  # file -> list of adds per touching commit (in log order = newest first)
cur = None
for line in open(os.path.join(RUN, '_numstat.txt'), encoding='utf-8', errors='replace'):
    line = line.rstrip('\n')
    if line.startswith('commit '): cur = line.split()[1]; continue
    if not line.strip(): continue
    parts = line.split('\t')
    if len(parts) < 3 or parts[0] == '-' or parts[1] == '-': continue
    adds = int(parts[0]); path = parts[2]
    if ' => ' in path:
        m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', path)
        path = (m.group(1) + m.group(3) + m.group(4)) if m else path.split(' => ')[-1]
    per_file.setdefault(path, []).append((cur, adds))
total_adds = sum(a for v in per_file.values() for _, a in v)
churned = 0; files_churned = 0
for f, v in per_file.items():
    if len(v) >= 2:
        files_churned += 1
        churned += sum(a for _, a in v[:-1])  # all but the FIRST (oldest) touching commit; log order is newest-first
churn = {'pct': round(100.0 * churned / total_adds, 2) if total_adds else None, 'files_churned': files_churned,
         'files_touched': len(per_file), 'total_adds': total_adds, 'churned_adds': churned,
         'population': f"git log --numstat {BASE['sha'][:7]}..HEAD -- crates/**/*.rs crates/**/*.ts crates/**/*.tsx",
         'per_file_commits': {f: len(v) for f, v in per_file.items()}}
out('churn', churn)
hot = []
for f, v in per_file.items():
    fn = norm(f)
    score = len(v) * (file_max_cog.get(fn) or file_max_cyc.get(fn) or (file_kloc.get(fn, 0.0) if fn.endswith('.rs') else 0.0))
    hot.append([fn, round(score, 2)])
hot.sort(key=lambda x: -x[1])
base_hot = {x[0] for x in BASE.get('hotspots', [])}
out('hotspots', {'top': hot[:10], 'entrants_vs_baseline': [h for h in hot[:10] if h[0] not in base_hot],
                 'recipe': 'commits x per-file max cognitive (rca, Rust); fallback max cyclomatic; fallback tokei KLOC (Rust only) — a TS/TSX file scores 0'})
print('sizes', totals, sizes['file_p50'], sizes['file_p90'], sizes['file_max'], 'over800', sizes['over_800'])
print('dup', st.get('percentage'), st.get('duplicatedLines'), st.get('totalLines', st.get('lines')), 'clones', st.get('clones'), 'split', dict(split))
print('cx fns', len(fns), 'p50/p90 cyc', nr(cycs, .5), nr(cycs, .9), 'cog', nr(cogs, .5), nr(cogs, .9), 'over', sum(1 for c in cogs if c > 15), 'max', mx)
print('graph cycles', graph['cycles'], 'edges', graph['cross_unit_edges'], 'fanout', graph['fan_out'][:3], 'fanin entrants', graph['fan_in_entrants'])
print('dead chain', raw_n, len(after_tests), len(after_entry), 'unused', unused)
print('churn', {k: v for k, v in churn.items() if k != 'per_file_commits'})
print('hot', hot[:10])
