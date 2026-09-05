"""Pinned C1 summarizer: per unit, read cargo-mutants' mutants.out lists and write c-mutation-{unit}.json.
score = caught/(caught+missed) [timeout and unviable excluded] — the baseline's formula. Survivors and timeouts kept FULL."""
import json, os, sys, re, glob
RUN = sys.argv[1]
PLANNED_FULL = {'conductor-core': 504, 'conductor-verify': 148, 'conductor-run': 118, 'conductor-cli': 117, 'conductor-tauri': 43}
def read_list(p):
    if not os.path.exists(p): return []
    return [l.rstrip('\n') for l in open(p, encoding='utf-8', errors='replace') if l.strip()]
def split_entry(line):
    m = re.match(r'^(.*?:\d+:\d+): (.*)$', line)
    return [m.group(1).replace('\\', '/'), m.group(2)] if m else [line, '']
for d in sorted(glob.glob(os.path.join(RUN, 'mutants-*'))):
    unit = os.path.basename(d)[len('mutants-'):]
    mo = os.path.join(d, 'mutants.out')
    if not os.path.isdir(mo): continue
    caught = read_list(os.path.join(mo, 'caught.txt')); missed = read_list(os.path.join(mo, 'missed.txt'))
    timeout = read_list(os.path.join(mo, 'timeout.txt')); unviable = read_list(os.path.join(mo, 'unviable.txt'))
    try: planned = len(json.load(open(os.path.join(mo, 'mutants.json'), encoding='utf-8')))
    except Exception: planned = None
    done = os.path.join(RUN, f'_done-{unit}')
    done_txt = open(done, encoding='utf-8').read().strip() if os.path.exists(done) else None
    tested = len(caught) + len(missed) + len(timeout) + len(unviable)
    score = round(100.0 * len(caught) / (len(caught) + len(missed)), 2) if (caught or missed) else None
    log = os.path.join(RUN, f'_mutants-{unit}.log')
    tail = read_list(log)[-3:] if os.path.exists(log) else []
    state = 'complete' if done_txt and planned is not None and tested >= planned else ('partial' if done_txt else 'running')
    rec = {'unit': unit, 'state': state, 'done_marker': done_txt, 'planned_in_scope': planned, 'planned_full_unit': PLANNED_FULL.get(unit),
           'counts': {'mutants': planned, 'caught': len(caught), 'missed': len(missed), 'timeout': len(timeout), 'unviable': len(unviable)},
           'tested': tested, 'score': score, 'score_formula': 'caught/(caught+missed)  [timeout and unviable excluded]',
           'survivors': [split_entry(l) for l in missed], 'timeouts': [split_entry(l) for l in timeout],
           'command': f'cargo mutants -p {unit} --test-tool=nextest --jobs 2' + (' --shard 1/4' if unit == 'conductor-core' else '') + f' --output {{run_dir}}/mutants-{unit}',
           'log_tail': tail}
    json.dump(rec, open(os.path.join(RUN, f'c-mutation-{unit}.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    print(f"{unit}: {state} planned={planned} tested={tested} caught={len(caught)} missed={len(missed)} timeout={len(timeout)} unviable={len(unviable)} score={score} | {done_txt}")
