"""C1 mutation summarizer (pinned): per unit {mutants, caught, missed, timeout,
unviable, score}; survivor list kept FULL as [file:line, mutation].

score = caught / (caught + missed)  — unviable and timeout excluded from the
denominator, matching the baseline's arithmetic (verify 27/37 = 72.97).
Usage: python sum_mutation.py <out.json> <unit>...
"""
import json, os, re, sys

RD = '.andromeda/runs/2026-08-22T13-31-54-code-audit'
LINE = re.compile(r'^(.*?):\s*(.*)$')


def read_lines(p):
    if not os.path.exists(p):
        return []
    return [l.rstrip('\n') for l in open(p, encoding='utf-8', errors='replace') if l.strip()]


def parse(entry):
    """'crates/x/src/y.rs:12:5: replace foo -> bar with Default::default()'"""
    parts = entry.split(': ', 1)
    if len(parts) == 2:
        return [parts[0], parts[1]]
    return [entry, '']


def main():
    out = sys.argv[1]
    units = sys.argv[2:]
    res = {'scoped_units': [], 'scores': {}, 'counts': {}, 'survivors': [],
           'survivors_by_unit': {}, 'timeouts': {}, 'unviable': {},
           'command': ('cargo mutants -p {unit} --test-tool=nextest --jobs 2 '
                       '--output {run_dir}/mutants-{unit}'),
           'score_formula': 'caught / (caught + missed); unviable and timeout excluded'}
    for u in units:
        d = os.path.join(RD, f'mutants-{u}', 'mutants.out')
        if not os.path.isdir(d):
            continue
        caught = read_lines(os.path.join(d, 'caught.txt'))
        missed = read_lines(os.path.join(d, 'missed.txt'))
        tmo = read_lines(os.path.join(d, 'timeout.txt'))
        unv = read_lines(os.path.join(d, 'unviable.txt'))
        denom = len(caught) + len(missed)
        res['scoped_units'].append(u)
        res['scores'][u] = round(100.0 * len(caught) / denom, 2) if denom else None
        res['counts'][u] = {'mutants': denom + len(tmo) + len(unv), 'caught': len(caught),
                            'missed': len(missed), 'timeout': len(tmo),
                            'unviable': len(unv)}
        surv = [parse(m) for m in missed]
        res['survivors_by_unit'][u] = surv
        res['survivors'].extend(surv)
        res['timeouts'][u] = [parse(t) for t in tmo]
        res['unviable'][u] = len(unv)
    json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
    for u in res['scoped_units']:
        c = res['counts'][u]
        print(f"{u:20} score={res['scores'][u]:6}  caught={c['caught']:3} "
              f"missed={c['missed']:3} timeout={c['timeout']:2} unviable={c['unviable']:2}")
    print(f"total survivors: {len(res['survivors'])}")


main()
