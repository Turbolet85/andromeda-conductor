"""A2 complexity summarizer (pinned): per-FUNCTION cyclomatic + cognitive from
rust-code-analysis JSON. Percentiles are nearest-rank over the per-function values.
Ceiling: cognitive > 15. Top list sorted by cyclomatic desc.
Usage: python sum_complexity.py <rca_out_dir> <out.json> <label> [strip_prefix]
"""
import json, sys, math, os


def nearest_rank(vals, pct):
    if not vals:
        return None
    s = sorted(vals)
    k = max(1, math.ceil(pct / 100 * len(s)))
    return s[k - 1]


def walk(space, path, acc):
    """Collect every kind=='function' space with its own sum metrics."""
    if space.get('kind') == 'function':
        m = space.get('metrics') or {}
        cy = (m.get('cyclomatic') or {}).get('sum')
        cg = (m.get('cognitive') or {}).get('sum')
        if cy is not None:
            acc.append({'fn': space.get('name'), 'file': path,
                        'cyclomatic': cy, 'cognitive': cg if cg is not None else 0.0})
    for child in (space.get('spaces') or []):
        walk(child, path, acc)


def main():
    root, out, label = sys.argv[1], sys.argv[2], sys.argv[3]
    strip = sys.argv[4] if len(sys.argv) > 4 else ''
    fns = []
    scanned = 0
    for dirpath, _dirnames, filenames in os.walk(root):
        for name in filenames:
            if not name.endswith('.rs.json'):
                continue
            full = os.path.join(dirpath, name)
            try:
                d = json.load(open(full, encoding='utf-8'))
            except ValueError:
                continue
            rel = os.path.relpath(full, root).replace('\\', '/')
            rel = rel[:-5] if rel.endswith('.json') else rel   # drop .json
            if any(seg in rel.split('/') for seg in ('node_modules', 'target', 'dist')):
                continue   # vendored / build output: not project source, and absent
                           # from the git-archive baseline tree (population symmetry)
            scanned += 1
            if strip and rel.startswith(strip):
                rel = rel[len(strip):]
            for sp in (d.get('spaces') or []):
                walk(sp, rel, fns)
    cy = [f['cyclomatic'] for f in fns]
    cg = [f['cognitive'] for f in fns]
    over = [f for f in fns if f['cognitive'] > 15]
    # PINNED to the baseline record's shape: cognitive-led. `top` rows are
    # [fn, file, cognitive, cyclomatic] sorted by cognitive desc; `max` is max cognitive.
    # (Recovered from the baseline record + verified at the raw metric: dispatch is
    #  cyclomatic 47 / cognitive 16 and the record stores it as 16.0, 47.0.)
    fns_by_cy = sorted(fns, key=lambda f: (-f['cognitive'], -f['cyclomatic'], f['fn']))
    mx = fns_by_cy[0] if fns_by_cy else None
    res = {
        'label': label,
        'population': 'kind==function spaces in *.rs.json under crates/ (rust-code-analysis)',
        'ceiling_rule': 'cognitive > 15',
        'rs_files_scanned': scanned,
        'functions': len(fns),
        'cyclomatic_p50': nearest_rank(cy, 50), 'cyclomatic_p90': nearest_rank(cy, 90),
        'cognitive_p50': nearest_rank(cg, 50), 'cognitive_p90': nearest_rank(cg, 90),
        'over_ceiling': len(over),
        'max': {'fn': mx['fn'], 'file': mx['file'], 'val': mx['cognitive']} if mx else None,
        'top': [[f['fn'], f['file'], f['cognitive'], f['cyclomatic']] for f in fns_by_cy[:10]],
        'over_ceiling_list': sorted(
            [[f['fn'], f['file'], f['cyclomatic'], f['cognitive']] for f in over],
            key=lambda r: -r[3]),
    }
    json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
    print(json.dumps({k: v for k, v in res.items()
                      if k not in ('top', 'over_ceiling_list')}, indent=None))
    print(' top-10 by cognitive [cg, cy]:')
    for r in res['top']:
        print(f'   cg={r[2]:5} cy={r[3]:5}  {r[0]}  ({r[1]})')


main()
