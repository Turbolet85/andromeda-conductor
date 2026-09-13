"""Mutation tier summarizer. Reads each unit's nested mutants.out/, writes c-mutation-{unit}.json
incrementally (so a stopped run resumes by artifact presence), then a combined c-mutation.json.

Takes ONLY the tallies + the repo-relative missed.txt lines: outcomes.json's `outcomes[].argv`
carries absolute host toolchain paths, which must never reach a committed artifact.
Score formula is the baseline's, verbatim: caught/(caught+missed), timeout and unviable excluded.
"""
import json, os, sys

RUN = os.path.dirname(os.path.abspath(__file__))
UNITS = ['conductor-tauri', 'conductor-cli', 'conductor-run', 'conductor-verify', 'conductor-core']
FORMULA = 'caught/(caught+missed)  [timeout and unviable excluded]'


def unit(u):
    out = f'{RUN}/mutants-{u}/mutants.out'
    oc = f'{out}/outcomes.json'
    if not os.path.exists(oc):
        return None
    d = json.load(open(oc, encoding='utf-8'))
    # COMPLETENESS GUARD. cargo-mutants writes outcomes.json incrementally, so a mid-run read
    # yields a real-looking but partial tally. Measured this run: conductor-cli read 54 of 112
    # planned with end_time None and scored 100.0 against a 58.59 baseline — a +41pt artifact
    # that would have entered the ledger as a permanent baseline. A unit counts as complete only
    # when end_time is set AND the tested total equals the planned mutant count.
    planned = None
    mj = f'{out}/mutants.json'
    if os.path.exists(mj):
        try:
            planned = len(json.load(open(mj, encoding='utf-8')))
        except Exception:
            planned = None
    if d.get('end_time') is None or (planned is not None and d.get('total_mutants') != planned):
        return {'_partial': True, 'unit': u, 'tested': d.get('total_mutants'), 'planned': planned,
                'end_time': d.get('end_time')}
    caught, missed = d.get('caught', 0), d.get('missed', 0)
    denom = caught + missed
    survivors = []
    mt = f'{out}/missed.txt'
    if os.path.exists(mt):
        for line in open(mt, encoding='utf-8'):
            line = line.strip().replace('\\', '/')
            if not line:
                continue
            # "path:line:col: mutation description" -> ["path:line", "description"]
            parts = line.split(': ', 1)
            survivors.append([parts[0], parts[1] if len(parts) > 1 else ''])
    r = {
        'unit': u,
        'counts': {'mutants': d.get('total_mutants', 0), 'caught': caught, 'missed': missed,
                   'timeout': d.get('timeout', 0), 'unviable': d.get('unviable', 0)},
        'score': round(100 * caught / denom, 2) if denom else None,
        'score_formula': FORMULA,
        'survivors': survivors,
        'wall_clock': {'start': d.get('start_time'), 'end': d.get('end_time')},
        'tool': d.get('cargo_mutants_version'),
    }
    json.dump(r, open(f'{RUN}/c-mutation-{u}.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    return r


if __name__ == '__main__':
    res, done = {}, []
    for u in UNITS:
        r = unit(u)
        if r is None:
            print(f'{u}: NOT YET (no outcomes.json)')
            continue
        if r.get('_partial'):
            print(f"{u}: IN PROGRESS — {r['tested']}/{r['planned']} tested, end_time={r['end_time']} "
                  f"(partial tally NOT recorded)")
            continue
        done.append(u)
        res[u] = r
        c = r['counts']
        print(f"{u:<20} score={r['score']:<7} mutants={c['mutants']:<4} caught={c['caught']:<4} "
              f"missed={c['missed']:<3} timeout={c['timeout']} unviable={c['unviable']}")
    combined = {
        'scoped_units': done,
        'unit_states': {u: f"complete ({res[u]['counts']['mutants']}/{res[u]['counts']['mutants']} tested)"
                        + (' of 504 in the unit — shard 1/4' if u == 'conductor-core' else '')
                        for u in done},
        'scores': {u: res[u]['score'] for u in done},
        'counts': {u: res[u]['counts'] for u in done},
        'score_formula': FORMULA,
        'survivors': [[u] + s for u in done for s in res[u]['survivors']],
    }
    json.dump(combined, open(f'{RUN}/c-mutation.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    print(f"\ncombined: {len(done)}/{len(UNITS)} units · {len(combined['survivors'])} survivors")
