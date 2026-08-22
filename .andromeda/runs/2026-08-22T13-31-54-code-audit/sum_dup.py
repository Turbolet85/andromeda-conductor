"""A1 duplication summarizer (pinned): from a jscpd report JSON.
Shape = {pct, duplicated_lines, total_lines, clones, top[10], split src/test/mixed}.
Usage: python sum_dup.py <jscpd-report.json> <out.json> <label> [strip_prefix]
"""
import json, sys, re

SLASHES = re.compile(r'[\\/]+')


def is_test(path):
    return 'tests' in SLASHES.split(path)


def rel(p, strip):
    p = p.replace('\\', '/')
    if strip and p.startswith(strip):
        p = p[len(strip):]
    i = p.find('crates/')
    return p[i:] if i >= 0 else p


def main():
    src, out, label = sys.argv[1], sys.argv[2], sys.argv[3]
    strip = sys.argv[4] if len(sys.argv) > 4 else ''
    d = json.load(open(src, encoding='utf-8'))
    tot = d['statistics']['total']
    pairs = []
    for c in d.get('duplicates', []):
        a = rel(c['firstFile']['name'], strip)
        b = rel(c['secondFile']['name'], strip)
        pairs.append([a, b, c.get('lines', 0)])
    pairs.sort(key=lambda p: -p[2])
    split = {'both_test': {'pairs': 0, 'lines': 0},
             'both_src': {'pairs': 0, 'lines': 0},
             'mixed': {'pairs': 0, 'lines': 0}}
    for a, b, n in pairs:
        ta, tb = is_test(a), is_test(b)
        k = 'both_test' if (ta and tb) else 'both_src' if not (ta or tb) else 'mixed'
        split[k]['pairs'] += 1
        split[k]['lines'] += n
    res = {
        'label': label,
        'population': 'jscpd --format rust over crates/ (Rust only)',
        'pct': round(tot['percentage'], 2),
        'duplicated_lines': tot['duplicatedLines'],
        'total_lines': tot['lines'],
        'clones': tot['clones'],
        'sources': tot['sources'],
        'top': pairs[:10],
        'split': split,
    }
    json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
    print(json.dumps({k: v for k, v in res.items() if k != 'top'}, indent=1))
    for a, b, n in pairs[:10]:
        print(f'   {n:4}  {a}  <->  {b}')


main()
