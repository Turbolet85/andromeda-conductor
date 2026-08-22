"""Per-file max cognitive / cyclomatic over the FULL rca output (not the capped summary).
Feeds B2 hotspots so the weight basis is 'cognitive' for every analysed file.
Usage: python perfile_cx.py <rca_dir> <out.json> [strip_prefix]
"""
import json, os, sys


def walk(sp, acc):
    if sp.get('kind') == 'function':
        m = sp.get('metrics') or {}
        acc.append(((m.get('cyclomatic') or {}).get('sum') or 0.0,
                    (m.get('cognitive') or {}).get('sum') or 0.0))
    for c in (sp.get('spaces') or []):
        walk(c, acc)


def main():
    root, out = sys.argv[1], sys.argv[2]
    strip = sys.argv[3] if len(sys.argv) > 3 else ''
    per = {}
    for dirpath, _d, files in os.walk(root):
        for name in files:
            if not name.endswith('.rs.json'):
                continue
            full = os.path.join(dirpath, name)
            try:
                d = json.load(open(full, encoding='utf-8'))
            except ValueError:
                continue
            rel = os.path.relpath(full, root).replace('\\', '/')[:-5]
            if any(seg in rel.split('/') for seg in ('node_modules', 'target', 'dist')):
                continue
            if strip and rel.startswith(strip):
                rel = rel[len(strip):]
            acc = []
            for sp in (d.get('spaces') or []):
                walk(sp, acc)
            per[rel] = {'max_cyclomatic': max((a for a, _ in acc), default=0.0),
                        'max_cognitive': max((b for _, b in acc), default=0.0),
                        'functions': len(acc)}
    json.dump(per, open(out, 'w', encoding='utf-8'), indent=1)
    print('files:', len(per))


main()
