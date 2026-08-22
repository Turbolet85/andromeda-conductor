"""Second validation arm: does `after test exclusion, minus entry points` reproduce 31?"""
import json, os, re

RD = '.andromeda/runs/2026-08-22T13-31-54-code-audit'
BASE = os.path.join(RD, '_baseline_tree')
SYM = re.compile(r'^rust-analyzer cargo (\S+) (\S+) ')
SLASHES = re.compile(r'[\\/]+')


def seg(p):
    return 'tests' in SLASHES.split(p)


def desc(s):
    m = SYM.match(s)
    return s[m.end():] if m else s


def leaf(d):
    parts = [p for p in re.split(r'[/#\]\[()]+', d.rstrip('.'))
             if p and not p.startswith('impl')]
    return parts[-1] if parts else d


def is_entry(d, f):
    return bool(re.match(r'^main\(\)\.$', d)) or '/bin/' in f


rows = json.load(open(os.path.join(RD, '_zeroref.json'), encoding='utf-8'))
kept = []
for r in rows:
    d, f = desc(r['symbol']), r['file'].replace('\\', '/')
    if seg(d) or seg(f):
        continue
    kept.append({'d': d, 'f': f})

ep = [k for k in kept if is_entry(k['d'], k['f'])]
rest = [k for k in kept if not is_entry(k['d'], k['f'])]

print('entry-points at HEAD (%d):' % len(ep))
for k in ep:
    print(f"   {k['d'][:55]:55} {k['f']}   at-baseline={os.path.exists(os.path.join(BASE, k['f']))}")

new = []
for k in rest:
    bf = os.path.join(BASE, k['f'])
    ident = leaf(k['d'])
    if not os.path.exists(bf):
        new.append([k['d'], k['f']])
        continue
    txt = open(bf, encoding='utf-8', errors='replace').read()
    if not re.search(r'\b' + re.escape(ident) + r'\b', txt):
        new.append([k['d'], k['f']])

implied = len(rest) - len(new)
print()
print('HEAD  after test-exclusion             :', len(kept))
print('HEAD  minus entry-points  (reportable) :', len(rest))
print('implied BASELINE (existed, no entry-pt):', implied)
print('RECORDED baseline                      : 31')
print('MATCH:', implied == 31)
print()
print('new since baseline in this set (%d):' % len(new))
for d, f in new:
    print('   ', d[:70], '||', f)

json.dump({'head_after_test_exclusion': len(kept),
           'head_minus_entry_points': len(rest),
           'implied_baseline': implied, 'recorded_baseline': 31,
           'match': implied == 31, 'new_members': new,
           'entry_points_excluded': [[k['d'], k['f']] for k in ep]},
          open(os.path.join(RD, 'c-dead-validation.json'), 'w', encoding='utf-8'), indent=1)
