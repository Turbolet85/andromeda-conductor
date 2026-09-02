"""C1 mutation summarizer — score formula pinned to the Epoch-4 record.

score = caught / (caught + missed)   [timeouts and unviable EXCLUDED from the
denominator — verified against the baseline: conductor-run 48/(48+20)=70.59 and
conductor-verify 97/(97+12)=88.99 both reproduce exactly].

Survivor list kept FULL (small, expensive to regenerate). Per-unit incremental:
a unit with no outcomes.json yet is reported as not-run, never as zero.
"""
import json, os, glob

RD = '.andromeda/runs/2026-09-02T15-49-17-code-audit'
UNITS = ['conductor-run', 'conductor-verify', 'conductor-tauri']

scores, counts, survivors, timeouts, states = {}, {}, [], [], {}

for unit in UNITS:
    oc = os.path.join(RD, f'mutants-{unit}', 'mutants.out', 'outcomes.json')
    done = os.path.join(RD, f'done-{unit}')
    if not os.path.exists(oc):
        states[unit] = 'not-run'
        continue
    d = json.load(open(oc, encoding='utf-8'))
    caught = d.get('caught', 0)
    missed = d.get('missed', 0)
    tmo = d.get('timeout', 0)
    unv = d.get('unviable', 0)
    tested = caught + missed + tmo + unv

    # `total_mutants` in outcomes.json is a ROLLING field that tracks progress, not
    # the plan — reading it as the denominator makes a half-finished run look complete
    # (measured: it read 70/70 while the plan held 118). The plan is mutants.json.
    plan = os.path.join(RD, f'mutants-{unit}', 'mutants.out', 'mutants.json')
    total = len(json.load(open(plan, encoding='utf-8'))) if os.path.exists(plan) \
        else d.get('total_mutants', 0)

    complete = os.path.exists(done) and tested >= total and total > 0
    states[unit] = 'complete' if complete else f'partial ({tested}/{total} tested)'
    counts[unit] = {'mutants': total, 'caught': caught, 'missed': missed,
                    'timeout': tmo, 'unviable': unv}
    scores[unit] = round(100.0 * caught / (caught + missed), 2) if (caught + missed) else None
    for o in d.get('outcomes', []):
        s = o.get('summary')
        sc = o.get('scenario')
        # `scenario` is the string "Baseline" for the unmutated run and an object
        # {"Mutant": {...}} for a real mutant — guard, or the baseline row throws.
        if not isinstance(sc, dict):
            continue
        m = sc.get('Mutant') or {}
        name = m.get('name') or ''
        # name is "file:line:col: <mutation>" — split into location + mutation
        parts = name.split(': ', 1)
        loc = parts[0] if parts else name
        mut = parts[1] if len(parts) > 1 else ''
        if s == 'MissedMutant':
            survivors.append([loc, mut])
        elif s == 'Timeout':
            timeouts.append([loc, mut])

out = {
    'scoped_units': UNITS,
    'unit_states': states,
    'scores': scores,
    'counts': counts,
    'score_formula': 'caught/(caught+missed)  [timeout and unviable excluded]',
    'command': ('cargo mutants -p {unit} --test-tool=nextest --jobs 2 '
                '--output {run_dir}/mutants-{unit}'),
    'survivors': survivors,
    'timeouts': timeouts,
}
json.dump(out, open(os.path.join(RD, 'c-mutation.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

for u in UNITS:
    print(f'{u:20s} {states[u]:28s} score={scores.get(u)} counts={counts.get(u)}')
print(f'survivors total={len(survivors)}  timeouts={len(timeouts)}')
