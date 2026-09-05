"""Render-time assertion: every evidence table's row count EQUALS the n its heading states (never by eye).
A heading declares its n as `<!-- rows: N -->` on the line before the table; the check counts the table's data rows."""
import re, sys
doc = open(sys.argv[1], encoding='utf-8').read().splitlines()
ok = True; checked = 0
for i, line in enumerate(doc):
    m = re.match(r'^<!-- rows: (\d+) -->$', line.strip())
    if not m: continue
    n = int(m.group(1)); j = i + 1
    while j < len(doc) and not doc[j].startswith('|'): j += 1
    rows = 0; j += 2  # header + separator
    while j < len(doc) and doc[j].startswith('|'): rows += 1; j += 1
    checked += 1
    status = 'OK ' if rows == n else 'MISMATCH'
    if rows != n: ok = False
    print(f'{status} line {i+1}: stated {n}, enumerated {rows}')
print(f'{checked} tables checked; ' + ('all consistent' if ok else 'INCONSISTENT'))
sys.exit(0 if ok else 1)
