"""A3 sizes summarizer (pinned): per-file Rust code lines from a tokei JSON.
Percentiles are nearest-rank on the sorted per-file values.
Usage: python sum_sizes.py <tokei.json> <out.json> <label> [strip_prefix]
"""
import json, sys, math


def nearest_rank(vals, pct):
    if not vals:
        return None
    s = sorted(vals)
    k = max(1, math.ceil(pct / 100 * len(s)))
    return s[k - 1]


def main():
    src, out, label = sys.argv[1], sys.argv[2], sys.argv[3]
    strip = sys.argv[4] if len(sys.argv) > 4 else ''
    d = json.load(open(src, encoding='utf-8'))
    rust = d.get('Rust') or {}
    files = []
    for rep in rust.get('reports', []):
        name = rep['name'].replace('\\', '/')
        if strip and name.startswith(strip):
            name = name[len(strip):]
        files.append((name, rep['stats']['code']))
    vals = [c for _, c in files]
    files.sort(key=lambda t: -t[1])
    res = {
        'label': label,
        'population': 'Rust .rs files under crates/ (tokei, gitignore-respecting)',
        'files': len(files),
        'loc': sum(vals),
        'file_p50': nearest_rank(vals, 50),
        'file_p90': nearest_rank(vals, 90),
        'file_max': max(vals) if vals else None,
        'over_800': sum(1 for v in vals if v > 800),
        'top': [[f, c] for f, c in files[:10]],
    }
    json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
    print(json.dumps({k: v for k, v in res.items() if k != 'top'}, indent=None))
    for f, c in files[:10]:
        print(f'   {c:5}  {f}')


main()
