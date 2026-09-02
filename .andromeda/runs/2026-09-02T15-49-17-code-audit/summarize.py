"""Code-audit Tier A/B summarizers — pinned per the Epoch-4 record's `recipes`.

Writes c-{metric}.json under the run dir. Reproduction of the baseline's own
recipe is the contract; any deviation is reported, never silently absorbed.
"""
import json, os, sys, math, subprocess, collections, glob, re

RD = '.andromeda/runs/2026-09-02T15-49-17-code-audit'
BASELINE_SHA = 'b8f3332df9f12c679dd50fd967f5aaa18b84f657'


def nearest_rank(sorted_vals, p):
    """Pinned percentile: nearest-rank on ascending values, 1-based index."""
    if not sorted_vals:
        return None
    k = max(1, math.ceil(p / 100.0 * len(sorted_vals)))
    return sorted_vals[k - 1]


def w(name, obj):
    with open(os.path.join(RD, f'c-{name}.json'), 'w', encoding='utf-8') as fh:
        json.dump(obj, fh, indent=1, ensure_ascii=False)


# ---------------- A3 sizes (tokei; Rust *.rs only under crates/) ----------------
def sizes():
    d = json.load(open(os.path.join(RD, '_tokei.json'), encoding='utf-8'))
    rust = d.get('Rust') or {}
    reports = rust.get('reports') or []
    per_file = []
    for r in reports:
        path = r['name'].replace('\\', '/')
        per_file.append((path, r['stats']['code']))
    vals = sorted(v for _, v in per_file)
    top = sorted(per_file, key=lambda x: -x[1])[:10]
    out = {
        'population': 'tokei --output json crates/ ; language=Rust (*.rs) only',
        'file_p50': nearest_rank(vals, 50), 'file_p90': nearest_rank(vals, 90),
        'file_max': max(vals) if vals else None,
        'over_800': sum(1 for v in vals if v > 800),
        'top': [[p, v] for p, v in top],
        'totals': {'loc': rust.get('code'), 'files': len(reports)},
    }
    w('sizes', out)
    return out, dict(per_file)


# ---------------- A1 duplication (jscpd, Rust-only) ----------------
def duplication():
    d = json.load(open(os.path.join(RD, 'jscpd_rs', 'jscpd-report.json'),
                       encoding='utf-8'))
    tot = d['statistics']['total']
    dups = d.get('duplicates') or []

    def norm(p):
        p = p.replace('\\', '/')
        i = p.find('crates/')
        return p[i + len('crates/'):] if i >= 0 else p

    def is_test(p):
        return '/tests/' in p or p.startswith('tests/') or '/test/' in p

    pairs = []
    split = {'both_test': {'pairs': 0, 'lines': 0},
             'both_src': {'pairs': 0, 'lines': 0},
             'mixed': {'pairs': 0, 'lines': 0}}
    for c in dups:
        a, b = norm(c['firstFile']['name']), norm(c['secondFile']['name'])
        lines = c.get('lines') or 0
        pairs.append([a, b, lines])
        ta, tb = is_test(a), is_test(b)
        key = 'both_test' if (ta and tb) else ('both_src' if not (ta or tb) else 'mixed')
        split[key]['pairs'] += 1
        split[key]['lines'] += lines
    pairs.sort(key=lambda x: -x[2])
    out = {
        'label': 'head',
        'population': 'jscpd --format rust over crates/ (Rust only)',
        'pct': round(tot['percentage'], 2),
        'duplicated_lines': tot['duplicatedLines'],
        'total_lines': tot['lines'],
        'clones': tot['clones'],
        'sources': tot['sources'],
        'top': pairs[:10],
        'all_pairs_count': len(pairs),
        'split': split,
    }
    w('duplication', out)
    return out, pairs


# ---------------- A2 complexity (rust-code-analysis; Rust functions) ----------------
def complexity():
    files = [p for p in glob.glob(os.path.join(RD, '_rca_head', '**', '*.rs.json'),
                                  recursive=True)]
    fns = []          # (name, file, cognitive, cyclomatic)
    per_file_max_cog = {}
    for fp in files:
        try:
            d = json.load(open(fp, encoding='utf-8'))
        except Exception:
            continue
        src = (d.get('name') or '').replace('\\', '/')
        i = src.find('crates/')
        src = src[i:] if i >= 0 else src

        def walk(space):
            if space.get('kind') == 'function':
                m = space.get('metrics') or {}
                cog = (m.get('cognitive') or {}).get('sum')
                cyc = (m.get('cyclomatic') or {}).get('sum')
                if cog is not None and cyc is not None:
                    fns.append((space.get('name'), src, float(cog), float(cyc)))
                    per_file_max_cog[src] = max(per_file_max_cog.get(src, 0.0),
                                                float(cog))
            for s in (space.get('spaces') or []):
                walk(s)

        for s in (d.get('spaces') or []):
            walk(s)
        per_file_max_cog.setdefault(src, 0.0)

    cogs = sorted(f[2] for f in fns)
    cycs = sorted(f[3] for f in fns)
    over = [f for f in fns if f[2] > 15]
    top = sorted(fns, key=lambda x: -x[2])[:10]
    mx = top[0] if top else None
    out = {
        'population': ('rust-code-analysis kind==function spaces under crates/; '
                       'ceiling cognitive > 15; top sorted by COGNITIVE desc'),
        'functions': len(fns), 'rs_files_analyzed': len(files),
        'cyclomatic_p50': nearest_rank(cycs, 50), 'cyclomatic_p90': nearest_rank(cycs, 90),
        'cognitive_p50': nearest_rank(cogs, 50), 'cognitive_p90': nearest_rank(cogs, 90),
        'over_ceiling': len(over),
        'over_ceiling_list': [[f[0], f[1], f[2], f[3]] for f in
                              sorted(over, key=lambda x: -x[2])],
        'max': ({'fn': mx[0], 'file': mx[1], 'val': mx[2]} if mx else None),
        'top': [[f[0], f[1], f[2], f[3]] for f in top],
    }
    w('complexity', out)
    return out, per_file_max_cog


# ---------------- B1/B2 churn + hotspots ----------------
SRC_RE = re.compile(r'^crates/.*\.(rs|ts|tsx)$')


def churn_hotspots(per_file_max_cog, per_file_loc):
    raw = subprocess.run(
        ['git', 'log', '--numstat', '--format=commit %H',
         f'{BASELINE_SHA}..HEAD', '--',
         'crates/**/*.rs', 'crates/**/*.ts', 'crates/**/*.tsx'],
        capture_output=True, text=True, encoding='utf-8', errors='replace').stdout

    commits_per_file = collections.Counter()
    adds_per_file = collections.defaultdict(list)   # ordered by commit
    for line in raw.splitlines():
        if line.startswith('commit '):
            continue
        parts = line.split('\t')
        if len(parts) != 3:
            continue
        a, _d, path = parts
        if a == '-':                       # binary
            continue
        if '=>' in path:                   # rename: count to the NEW path
            m = re.search(r'\{.*=>\s*(.*?)\}', path)
            path = (path[:path.find('{')] + m.group(1)) if m else path.split('=>')[-1]
            path = path.replace('//', '/').strip()
        path = path.replace('\\', '/')
        if not SRC_RE.match(path):
            continue
        commits_per_file[path] += 1
        adds_per_file[path].append(int(a))

    total_adds = sum(sum(v) for v in adds_per_file.values())
    churned_adds = sum(sum(v[1:]) for v in adds_per_file.values())  # 2nd..nth touch
    files_churned = sum(1 for v in adds_per_file.values() if len(v) > 1)
    churn = {
        'population': ("git log --numstat b8f3332..HEAD -- crates/**/*.rs "
                       "crates/**/*.ts crates/**/*.tsx"),
        'pct': round(100.0 * churned_adds / total_adds, 2) if total_adds else None,
        'files_churned': files_churned,
        'files_touched': len(adds_per_file),
        'total_adds': total_adds, 'churned_adds': churned_adds,
        'top_touched': commits_per_file.most_common(10),
    }
    w('churn', churn)

    scored = []
    for path, n in commits_per_file.items():
        cog = per_file_max_cog.get(path)
        if cog is None:
            cog = per_file_loc.get(path, 0) / 1000.0   # fallback: file KLOC
        scored.append([path, round(n * cog, 2), n, cog])
    scored.sort(key=lambda x: -x[1])
    hot = {'formula': 'commits x per-file max cognitive (fallback file KLOC)',
           'top': [[s[0], s[1]] for s in scored[:10]],
           'detail': scored[:10]}
    w('hotspots', hot)
    return churn, hot


if __name__ == '__main__':
    sz, per_file_loc = sizes()
    print('SIZES  ', {k: sz[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800')},
          sz['totals'])
    dup, pairs = duplication()
    print('DUP    ', {k: dup[k] for k in ('pct', 'clones', 'duplicated_lines',
                                          'total_lines', 'sources')}, dup['split'])
    cx, pfmc = complexity()
    print('CPLX   ', {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90',
                                         'cognitive_p50', 'cognitive_p90',
                                         'over_ceiling', 'functions',
                                         'rs_files_analyzed')}, cx['max'])
    ch, ho = churn_hotspots(pfmc, per_file_loc)
    print('CHURN  ', {k: ch[k] for k in ('pct', 'files_churned', 'files_touched',
                                         'total_adds')})
    print('HOTSPOT', ho['top'][:5])
