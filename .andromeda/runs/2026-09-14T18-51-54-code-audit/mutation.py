"""Tier C driver. Per-unit, capped, incremental. Reads a tally ONLY when the invocation is
complete by cargo-mutants' own markers (collectors.md C1): outcomes.json's top-level end_time set
AND total_mutants == len(mutants.json). Writes c-mutation-{unit}.json only for a complete unit,
so a stopped run resumes by artifact presence."""
import json, os, subprocess, sys, time

RD = 'D:/dev/projects/conductor/.andromeda/runs/2026-09-14T18-51-54-code-audit'
ROOT = 'D:/dev/projects/conductor'
CAP = int(os.environ.get('UNIT_CAP', '900'))  # per-unit wall-clock cap, seconds

# (unit, shard) — shard mirrors the baseline for core (504 mutants, 1/4 pinned for comparability)
UNITS = [
    ('conductor-report', None),
    ('conductor-cli', None),
    ('conductor-run', None),
    ('conductor-verify', None),
    ('conductor-core', '1/4'),
    ('conductor-emit', '1/4'),
]


def firing(unit, shard):
    cmd = ['cargo', 'mutants', '-p', unit, '--test-tool=nextest', '--jobs', '2',
           '--output', f'{RD}/mutants-{unit}']
    if shard:
        cmd += ['--shard', shard]
    return cmd


def complete(out):
    """Completion by the tool's own markers. Returns (state, outcomes, planned)."""
    oj, mj = f'{out}/mutants.out/outcomes.json', f'{out}/mutants.out/mutants.json'
    if not os.path.exists(oj):
        return 'no-outcomes', None, None
    try:
        o = json.load(open(oj, encoding='utf-8'))
    except Exception:
        return 'partial-unparseable', None, None
    planned = None
    if os.path.exists(mj):
        try:
            planned = len(json.load(open(mj, encoding='utf-8')))
        except Exception:
            planned = None
    if not o.get('end_time'):
        return 'incomplete-no-end-time', o, planned
    tm = o.get('total_mutants')
    if planned is not None and tm != planned:
        return f'incomplete-count {tm}/{planned}', o, planned
    return 'complete', o, planned


def tally(o):
    c = {'mutants': o.get('total_mutants') or 0, 'caught': 0, 'missed': 0, 'timeout': 0, 'unviable': 0}
    survivors = []
    for e in (o.get('outcomes') or []):
        sc = e.get('scenario')
        if sc == 'Baseline' or (isinstance(sc, dict) and 'Baseline' in sc):
            continue
        s = (e.get('summary') or '').lower()
        if s == 'caughtmutant' or s == 'caught':
            c['caught'] += 1
        elif s == 'missedmutant' or s == 'missed':
            c['missed'] += 1
            m = (sc or {}).get('Mutant') if isinstance(sc, dict) else None
            if m:
                survivors.append([f"{m.get('file')}:{m.get('line')}:{m.get('col')}", m.get('replacement') or m.get('genre')])
        elif 'timeout' in s:
            c['timeout'] += 1
        elif 'unviable' in s:
            c['unviable'] += 1
    return c, survivors


results = {}
for unit, shard in UNITS:
    art = f'{RD}/c-mutation-{unit}.json'
    if os.path.exists(art):
        print(f'[{unit}] artifact present — resuming past it', flush=True)
        continue
    out = f'{RD}/mutants-{unit}'
    cmd = firing(unit, shard)
    print(f'[{unit}] START shard={shard} cap={CAP}s :: {" ".join(cmd)}', flush=True)
    t0 = time.time()
    try:
        p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True,
                           encoding='utf-8', errors='replace', timeout=CAP)
        rc, killed = p.returncode, False
        tailtxt = (p.stdout or '')[-1500:] + (p.stderr or '')[-1500:]
    except subprocess.TimeoutExpired as e:
        rc, killed = None, True
        tailtxt = (str(e.stdout or '')[-800:] + str(e.stderr or '')[-800:])
    el = round(time.time() - t0, 1)
    state, o, planned = complete(out)
    rec = {'unit': unit, 'shard': shard, 'elapsed_s': el, 'exit': rc, 'killed_at_cap': killed,
           'completion_state': state, 'command': ' '.join(cmd),
           'planned_from_mutants_json': planned}
    if state == 'complete':
        c, surv = tally(o)
        denom = c['caught'] + c['missed']
        rec['counts'] = c
        rec['score'] = round(100 * c['caught'] / denom, 2) if denom else None
        rec['score_formula'] = 'caught/(caught+missed)  [timeout and unviable excluded]'
        rec['survivors'] = surv
        rec['unit_state'] = (f"complete ({c['mutants']}/{planned} tested)"
                             + (f" of the unit — shard {shard}" if shard else ""))
        if c['mutants'] == 0:
            rec['unit_state'] = 'NO-OP: Found 0 mutants — mis-scoped filter, never a pass'
        json.dump(rec, open(art, 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
        print(f"[{unit}] COMPLETE {el}s score={rec['score']} counts={c}", flush=True)
    else:
        oc0 = ((o or {}).get('outcomes') or [{}])[0].get('summary') if o else None
        if o and (o.get('total_mutants') == 0) and oc0 not in ('Success', 'success'):
            rec['classified'] = 'baseline-test-failure'
        else:
            rec['classified'] = 'budget-exhausted'
        rec['outcomes0_summary'] = oc0
        rec['tail'] = tailtxt[-900:]
        json.dump(rec, open(f'{RD}/x-mutation-{unit}.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
        print(f"[{unit}] {rec['classified'].upper()} {el}s state={state}", flush=True)
    results[unit] = rec
print('DRIVER DONE', flush=True)
