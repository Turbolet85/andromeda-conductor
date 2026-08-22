"""Render assertion (proposal-template.md): an evidence table's row count must EQUAL
its stated n — asserted programmatically, never by eye."""
import re, sys

P = '.andromeda/runs/2026-08-22T13-31-54-code-audit/proposals.md'
lines = open(P, encoding='utf-8').read().split('\n')


def table_rows(start):
    """Count body rows of the first markdown table at/after `start`."""
    i, seen_sep, n = start, False, 0
    while i < len(lines):
        l = lines[i].strip()
        if l.startswith('|'):
            if re.match(r'^\|[\s:\-|]+\|$', l):
                seen_sep = True
            elif seen_sep:
                n += 1
        elif seen_sep and not l.startswith('|') and l:
            break
        elif seen_sep and not l:
            break
        i += 1
    return n


checks = []
for idx, l in enumerate(lines):
    m = re.search(r'\*\*Evidence:\*\*\s*all (\d+)', l)
    if m:
        checks.append((l.strip()[:60], int(m.group(1)), table_rows(idx)))
    if re.search(r'\*\*Evidence:\*\*\s*both units', l):
        checks.append((l.strip()[:60], 2, table_rows(idx)))
    if re.search(r'\*\*Evidence:\*\*\s*both, complete', l):
        checks.append((l.strip()[:60], 2, table_rows(idx)))
    if 'Accepted-deliberate class — 6 survivors' in l:
        checks.append(('accepted-deliberate (6)', 6, table_rows(idx)))

ok = True
for label, stated, actual in checks:
    status = 'OK ' if stated == actual else 'MISMATCH'
    if stated != actual:
        ok = False
    print(f'  [{status}] stated n={stated} rows={actual}  :: {label}')
print()
print('ALL EVIDENCE TABLES CONSISTENT' if ok else 'RENDER ASSERTION FAILED')
sys.exit(0 if ok else 1)
