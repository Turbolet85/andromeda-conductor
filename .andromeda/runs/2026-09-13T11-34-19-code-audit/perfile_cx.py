"""Per-file max cognitive/cyclomatic over the FULL rca output (not the capped top list),
so B2 hotspots use the same population the baseline's scores imply."""
import json, os

RUN = os.path.dirname(os.path.abspath(__file__))
maxcog, maxcyc = {}, {}
for dirpath, _, names in os.walk(f'{RUN}/_rca_head'):
    for nm in names:
        if not nm.endswith('.json'):
            continue
        try:
            d = json.load(open(os.path.join(dirpath, nm), encoding='utf-8'))
        except Exception:
            continue
        src = (d.get('name') or '').replace('\\', '/')
        if not src.endswith('.rs'):
            continue
        # key by the repo-relative path, as churn reports it
        key = src[src.index('crates/'):] if 'crates/' in src else src
        stack = list(d.get('spaces') or [])
        while stack:
            sp = stack.pop()
            stack.extend(sp.get('spaces') or [])
            if sp.get('kind') != 'function':
                continue
            m = sp.get('metrics') or {}
            cog = ((m.get('cognitive') or {}).get('sum')) or 0
            cyc = ((m.get('cyclomatic') or {}).get('sum')) or 0
            maxcog[key] = max(maxcog.get(key, 0), cog)
            maxcyc[key] = max(maxcyc.get(key, 0), cyc)
json.dump({'max_cognitive': maxcog, 'max_cyclomatic': maxcyc},
          open(f'{RUN}/c-_perfile_cx.json', 'w', encoding='utf-8'), ensure_ascii=False)

# B2 hotspots, pinned: score = commits x max cognitive in file
commits = json.load(open(f'{RUN}/c-_commits_per_file.json', encoding='utf-8'))
rows = []
for p, n in commits.items():
    val = maxcog.get(p)
    basis = 'cognitive'
    if val is None:
        val = maxcyc.get(p)
        basis = 'cyclomatic-fallback'
    if val is None:
        basis = 'no-complexity (non-rust or unscanned)'
        val = 0
    rows.append([p, n * val, n, val, basis])
rows.sort(key=lambda r: -r[1])
out = {'population': 'commits(B1 window) x max cognitive per file over the FULL rca population; '
                     'fallback cyclomatic, then 0 for non-Rust (e.g. .ts) files',
       'top': [[r[0], r[1]] for r in rows[:10]], 'detail': rows[:12]}
json.dump(out, open(f'{RUN}/c-hotspots.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
print('files with complexity:', len(maxcog))
for r in rows[:10]:
    print(f'  {r[1]:>7}  {r[0]:<52} commits={r[2]} maxcog={r[3]} ({r[4]})')
