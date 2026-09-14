"""Re-derive each unit's survivor list from cargo-mutants' own missed.txt (the canonical
`{path}:{line}:{col}: {mutation}` form the project's mutation-gate also parses), and assert the
row count equals the unit's counts.missed read from outcomes.json — two independent sources."""
import json, glob, os, re

RD = 'D:/dev/projects/conductor/.andromeda/runs/2026-09-14T18-51-54-code-audit'
LINE = re.compile(r'^(?P<site>.+?:\d+:\d+): (?P<mut>.+)$')

for f in sorted(glob.glob(f'{RD}/c-mutation-*.json')):
    d = json.load(open(f, encoding='utf-8'))
    u = d['unit']
    mt = f'{RD}/mutants-{u}/mutants.out/missed.txt'
    rows = []
    if os.path.exists(mt):
        for line in open(mt, encoding='utf-8'):
            line = line.rstrip('\n')
            if not line.strip():
                continue
            m = LINE.match(line)
            if not m:
                raise SystemExit(f'UNPARSED missed.txt line in {u}: {line!r}')
            rows.append([m.group('site'), m.group('mut')])
    expected = d['counts']['missed']
    assert len(rows) == expected, f'{u}: missed.txt rows {len(rows)} != counts.missed {expected}'
    d['survivors'] = rows
    d['survivors_source'] = 'mutants.out/missed.txt, row count asserted == counts.missed from outcomes.json'
    json.dump(d, open(f, 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    print(f'  {u:20s} survivors={len(rows):>3} (== counts.missed {expected})')
print('all units re-derived and cross-checked')
