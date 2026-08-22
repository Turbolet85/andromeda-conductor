"""Validate the hypothesis that the baseline's zero_ref_candidates=31 is the
`after_test_exclusion` step of the pinned recipe (FP families NAMED, not subtracted).

Test: take the HEAD after-test-exclusion set, and for each member decide whether its
defining symbol already existed at the baseline commit (by searching the extracted
baseline tree for the leaf identifier in the same file). If exactly (HEAD - 31) of them
are NEW, the hypothesis holds arithmetically.
"""
import json, os, re, sys

RD = '.andromeda/runs/2026-08-22T13-31-54-code-audit'
BASE = os.path.join(RD, '_baseline_tree')
SYM_PREFIX = re.compile(r'^rust-analyzer cargo (\S+) (\S+) ')


def has_tests_segment(p):
    return 'tests' in p.replace('\\', '/').split('/')


def descriptor(s):
    m = SYM_PREFIX.match(s)
    return s[m.end():] if m else s


def leaf(desc):
    """Last meaningful identifier of a SCIP descriptor."""
    d = desc.rstrip('.')
    for sep in (']', '/', '#'):
        pass
    parts = re.split(r'[/#\]\[()]+', d)
    parts = [p for p in parts if p and not p.startswith('impl')]
    return parts[-1] if parts else d


def main():
    rows = json.load(open(os.path.join(RD, '_zeroref.json'), encoding='utf-8'))
    kept = []
    for r in rows:
        d, f = descriptor(r['symbol']), r['file']
        if has_tests_segment(d) or has_tests_segment(f):
            continue
        kept.append({'symbol': r['symbol'], 'desc': d, 'file': f.replace('\\', '/')})

    existed, new, unknown = [], [], []
    for k in kept:
        bf = os.path.join(BASE, k['file'])
        ident = leaf(k['desc'])
        if not os.path.exists(bf):
            new.append((k['desc'], k['file'], 'file-absent-at-baseline'))
            continue
        try:
            txt = open(bf, encoding='utf-8', errors='replace').read()
        except OSError:
            unknown.append((k['desc'], k['file'], 'unreadable'))
            continue
        if re.search(r'\b' + re.escape(ident) + r'\b', txt):
            existed.append((k['desc'], k['file'], ident))
        else:
            new.append((k['desc'], k['file'], ident))

    out = {
        'head_after_test_exclusion': len(kept),
        'existed_at_baseline': len(existed),
        'new_since_baseline': len(new),
        'unknown': len(unknown),
        'implied_baseline_count': len(existed),
        'recorded_baseline_count': 31,
        'hypothesis_holds': len(existed) == 31,
        'new_members': [[d, f, i] for d, f, i in new],
        'existed_members': [[d, f] for d, f, _ in existed],
    }
    json.dump(out, open(os.path.join(RD, 'c-dead-validation.json'), 'w', encoding='utf-8'),
              indent=1)
    print(json.dumps({k: v for k, v in out.items()
                      if k not in ('new_members', 'existed_members')}, indent=1))
    print('\nNEW since baseline (%d):' % len(new))
    for d, f, i in new:
        print(f'   {d[:78]:78} {f}')


main()
