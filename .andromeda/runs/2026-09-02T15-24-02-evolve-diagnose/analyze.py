"""Evolve diagnosis analysis — Epoch 5 (Verification surfaces).

Emits q-health.json / q-typed.json / q-untyped.json / q-chains.json / q-level.json
into the run dir. Read-only on the ledger; writes only inside the run dir.
"""
import json, collections, re, os, sys

LEDGER = '.andromeda/friction-log.ndjson'
RUN = '.andromeda/runs/2026-09-02T15-24-02-evolve-diagnose'
EPOCH = 'Epoch 5 — Verification surfaces'

UNIVERSAL = {
    'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
    'contract.structural-blind-spot', 'contract.token-proxy-check',
    'tooling.output-cap-overflow', 'contract.skill-reference-drift',
}


def fold_epoch(e):
    return None if e is None else re.sub(r'\s+[-–—]\s+', ' — ', e).strip()


def fold_skill(s):
    if not s:
        return s
    return s[len('andromeda-'):] if s.startswith('andromeda-') else s


# ---------- load + normalize ----------
recs, skips = [], 0
for line in open(LEDGER, encoding='utf-8'):
    if not line.strip():
        continue
    try:
        recs.append(json.loads(line))
    except ValueError:
        skips += 1

for r in recs:
    r['_epoch'] = fold_epoch(r.get('epoch'))
    r['_skill'] = fold_skill(r.get('skill'))
    p = r.get('problem')
    r['_problems'] = [] if p is None else (p if isinstance(p, list) else [p])

# ---------- retraction exclusions ----------
by_id = {r['id']: r for r in recs if r.get('id')}
excl_ids, excl_probs = set(), set()
for r in recs:
    rl = r.get('retracts')
    if not rl:
        continue
    for t in (rl if isinstance(rl, list) else [rl]):
        if not isinstance(t, dict):
            continue
        tid, scope = t.get('id'), t.get('scope')
        tgt = by_id.get(tid)
        if tid is None or tgt is None or scope not in ('record', 'problem'):
            continue
        if scope == 'record':
            if tgt.get('kind') != 'step':
                excl_ids.add(tid)
        else:
            excl_probs.add((tid, t.get('index')))

# apply: drop retracted friction records; drop targeted problem-facts
filtered = []
for r in recs:
    if r.get('id') in excl_ids and r.get('kind') == 'friction':
        continue
    if r.get('kind') == 'step' and r.get('_problems'):
        rid = r.get('id')
        if (rid, None) in excl_probs:
            r['_problems'] = []
        else:
            keep = [f for i, f in enumerate(r['_problems']) if (rid, i) not in excl_probs]
            r['_problems'] = keep
    filtered.append(r)

E = [r for r in filtered if r['_epoch'] == EPOCH]
steps = [r for r in E if r.get('kind') == 'step']
fric = [r for r in E if r.get('kind') == 'friction']

# ---------- Stage 0: mechanism health ----------
EXPECTED = {'phase': 5, 'implement': 3, 'wrap-session': 5}
step_names = collections.defaultdict(set)
for r in steps:
    step_names[r['_skill']].add(r.get('step'))

cov = {}
for ch in sorted({r.get('chunk') for r in steps if r.get('chunk')}):
    per = collections.Counter()
    for r in steps:
        if r.get('chunk') == ch:
            per[r['_skill']] += 1
    cov[ch] = {'phase': per.get('phase', 0), 'implement': per.get('implement', 0),
               'wrap-session': per.get('wrap-session', 0),
               'new-session': per.get('new-session', 0),
               'other': {k: v for k, v in per.items()
                         if k not in ('phase', 'implement', 'wrap-session', 'new-session')},
               'steps': sorted({r.get('step') for r in steps
                                if r.get('chunk') == ch})}

nullchunk_steps = [{'skill': r['_skill'], 'step': r.get('step'), 'ts': r.get('ts')}
                   for r in steps if not r.get('chunk')]

untyped_by_step = collections.Counter()
total_by_step = collections.Counter()
for r in fric:
    k = f"{r['_skill']}/{r.get('step')}"
    total_by_step[k] += 1
    if r.get('untyped') or r.get('type') is None:
        untyped_by_step[k] += 1

prob_fill = sum(1 for r in steps if r['_problems'])
id_fill = sum(1 for r in E if r.get('id'))

health = {
    'epoch': EPOCH,
    'records': len(E), 'step': len(steps), 'friction': len(fric),
    'unparseable_whole_ledger': skips,
    'chunks': sorted(cov.keys()),
    'coverage': cov,
    'expected_per_chunk': EXPECTED,
    'null_chunk_step_records': nullchunk_steps,
    'untyped_rate_overall': [sum(untyped_by_step.values()), len(fric)],
    'untyped_by_step': {k: [untyped_by_step[k], total_by_step[k]]
                        for k in sorted(total_by_step, key=lambda x: -untyped_by_step[x])
                        if untyped_by_step[k]},
    'problem_fact_fill': [prob_fill, len(steps)],
    'id_fill': [id_fill, len(E)],
    'skills_steps': {k: sorted(v) for k, v in sorted(step_names.items())},
    'retraction_exclusions_applied': {'friction_ids': sorted(excl_ids),
                                      'problem_targets': sorted(map(list, excl_probs), key=str)},
}
json.dump(health, open(os.path.join(RUN, 'q-health.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

# ---------- Stage 1: typed patterns ----------
def weight_of(rs):
    w = 0
    for r in rs:
        i = r.get('impact') or {}
        w += (1 + i.get('iterations', 0) + i.get('retries', 0)
              + i.get('reformulations', 0) + 2 * i.get('dialogue_rounds', 0)
              + 3 * i.get('halted', 0) + 3 * i.get('soft_exit', 0))
    return w


def impact_sum(rs):
    tot = collections.Counter()
    for r in rs:
        for k, v in (r.get('impact') or {}).items():
            if isinstance(v, (int, float)):
                tot[k] += v
    return dict(tot)


step_runs = collections.Counter()
for r in steps:
    step_runs[(r['_skill'], r.get('step'))] += 1

groups = collections.defaultdict(list)
for r in fric:
    t = r.get('type')
    if t is None:
        continue
    groups[(r['_skill'], r.get('step'), t)].append(r)

typed = []
for (sk, st, t), rs in groups.items():
    imp = impact_sum(rs)
    typed.append({
        'key': f'{sk}/{st}/{t}', 'skill': sk, 'step': st, 'type': t,
        'n': len(rs), 'weight': weight_of(rs),
        'step_runs': step_runs.get((sk, st), 0),
        'rate': round(len(rs) / step_runs[(sk, st)], 3) if step_runs.get((sk, st)) else None,
        'chunks': sorted({r.get('chunk') for r in rs if r.get('chunk')}),
        'impact': imp,
        'halt_or_softexit': imp.get('halted', 0) + imp.get('soft_exit', 0),
        'cases': [{'chunk': r.get('chunk'), 'id': r.get('id'), 'ts': r.get('ts'),
                   'what': r.get('what'), 'impact': r.get('impact'),
                   'evidence': r.get('evidence'), 'artifacts': r.get('artifacts')} for r in rs],
    })
typed.sort(key=lambda g: (-g['weight'], -g['n']))

# universal types grouped by type ALONE across steps
uni_groups = collections.defaultdict(list)
for r in fric:
    if r.get('type') in UNIVERSAL:
        uni_groups[r['type']].append(r)
universal = []
for t, rs in uni_groups.items():
    imp = impact_sum(rs)
    universal.append({
        'key': f'UNIVERSAL/{t}', 'type': t, 'n': len(rs), 'weight': weight_of(rs),
        'steps': sorted({f"{r['_skill']}/{r.get('step')}" for r in rs}),
        'chunks': sorted({r.get('chunk') for r in rs if r.get('chunk')}),
        'impact': imp, 'halt_or_softexit': imp.get('halted', 0) + imp.get('soft_exit', 0),
        'cases': [{'chunk': r.get('chunk'), 'id': r.get('id'), 'ts': r.get('ts'),
                   'skill_step': f"{r['_skill']}/{r.get('step')}",
                   'what': r.get('what'), 'impact': r.get('impact'),
                   'evidence': r.get('evidence'), 'artifacts': r.get('artifacts')} for r in rs],
    })
universal.sort(key=lambda g: (-g['weight'], -g['n']))


def above(g):
    return g['n'] >= 3 or (g['n'] >= 2 and g['halt_or_softexit'] > 0)


json.dump({'per_skill_step_type': typed, 'universal_by_type': universal,
           'step_runs': {f'{k[0]}/{k[1]}': v for k, v in sorted(step_runs.items())},
           'above_threshold': [g['key'] for g in typed if above(g)]
                              + [g['key'] for g in universal if above(g)]},
          open(os.path.join(RUN, 'q-typed.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

# ---------- Stage 2: untyped ----------
un = [{'chunk': r.get('chunk'), 'id': r.get('id'), 'ts': r.get('ts'),
       'skill_step': f"{r['_skill']}/{r.get('step')}", 'what': r.get('what'),
       'impact': r.get('impact'), 'artifacts': r.get('artifacts'),
       'evidence': r.get('evidence')}
      for r in fric if r.get('untyped') or r.get('type') is None]
# cross-epoch: previous epoch untyped for recurrence
PREV = 'Epoch 4 — Lifecycle & delegated timing'
un_prev = [{'chunk': r.get('chunk'), 'skill_step': f"{fold_skill(r.get('skill'))}/{r.get('step')}",
            'what': r.get('what')}
           for r in filtered
           if r['_epoch'] == PREV and r.get('kind') == 'friction'
           and (r.get('untyped') or r.get('type') is None)]
json.dump({'epoch5_untyped': un, 'epoch4_untyped_for_recurrence': un_prev},
          open(os.path.join(RUN, 'q-untyped.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

# ---------- Stage 3: chains ----------
# anchors: input.* frictions, and step records with a degraded consumed[] verdict
anchors = []
for r in fric:
    if (r.get('type') or '').startswith('input.'):
        anchors.append({'kind': 'friction', 'chunk': r.get('chunk'), 'id': r.get('id'),
                        'skill_step': f"{r['_skill']}/{r.get('step')}", 'type': r.get('type'),
                        'what': r.get('what'), 'artifact': None})
for r in steps:
    for c in (r.get('consumed') or []):
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append({'kind': 'step-consumed', 'chunk': r.get('chunk'), 'id': r.get('id'),
                            'skill_step': f"{r['_skill']}/{r.get('step')}",
                            'artifact': c.get('artifact'), 'quality': c.get('quality'),
                            'note': c.get('note')})

# producers index: (chunk, artifact) -> step records producing it, in file order
prod = collections.defaultdict(list)
for idx, r in enumerate(steps):
    for p in (r.get('produced') or []):
        prod[(r.get('chunk'), p.get('artifact'))].append(
            {'order': idx, 'skill_step': f"{r['_skill']}/{r.get('step')}", 'id': r.get('id'),
             'outcome': r.get('outcome'), 'signals': p.get('signals'), 'note': p.get('note')})

order_of = {r.get('id'): i for i, r in enumerate(steps)}
chains = []
for a in anchors:
    if not a.get('artifact'):
        continue
    cands = prod.get((a['chunk'], a['artifact']), [])
    a_ord = order_of.get(a['id'], 10 ** 9)
    ups = [c for c in cands if c['order'] < a_ord]
    if ups:
        p = ups[-1]
        chains.append({'chunk': a['chunk'], 'artifact': a['artifact'],
                       'producer': p['skill_step'], 'producer_outcome': p['outcome'],
                       'producer_signals': p['signals'], 'producer_note': p['note'],
                       'producer_id': p['id'],
                       'consumer': a['skill_step'], 'consumer_id': a['id'],
                       'quality': a.get('quality'), 'consumer_note': a.get('note')})

shape = collections.Counter((c['producer'], c['artifact'], c['consumer']) for c in chains)
json.dump({'anchors': anchors, 'chains': chains,
           'shapes_ge2_chunks': [{'shape': list(k), 'n': v} for k, v in shape.most_common() if v >= 2],
           'all_shapes': [{'shape': list(k), 'n': v} for k, v in shape.most_common()]},
          open(os.path.join(RUN, 'q-chains.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

# ---------- Stage 4: level ----------
facts = []
for r in steps:
    for i, f in enumerate(r['_problems']):
        if not isinstance(f, dict):
            continue
        facts.append({'chunk': r.get('chunk'), 'id': r.get('id'), 'index': i,
                      'skill_step': f"{r['_skill']}/{r.get('step')}",
                      'nature': f.get('nature'), 'solution': f.get('solution'),
                      'note': f.get('note'), 'outcome': r.get('outcome')})

prev_facts = []
for r in filtered:
    if r['_epoch'] == PREV and r.get('kind') == 'step':
        for i, f in enumerate(r['_problems']):
            if isinstance(f, dict):
                prev_facts.append({'chunk': r.get('chunk'),
                                   'skill_step': f"{fold_skill(r.get('skill'))}/{r.get('step')}",
                                   'nature': f.get('nature'), 'solution': f.get('solution'),
                                   'note': f.get('note')})

by_sol = collections.Counter(f['solution'] for f in facts)
degraded = [{'chunk': r.get('chunk'), 'skill_step': f"{r['_skill']}/{r.get('step')}",
             'id': r.get('id'), 'outcome': r.get('outcome'),
             'counts': r.get('counts')}
            for r in steps if r.get('outcome') in ('ok-degraded', 'halted-resolved',
                                                   'soft-exit', 'aborted')]
prev_degraded = collections.Counter(
    r.get('outcome') for r in filtered
    if r['_epoch'] == PREV and r.get('kind') == 'step'
    and r.get('outcome') in ('ok-degraded', 'halted-resolved', 'soft-exit', 'aborted'))

json.dump({'facts': facts, 'by_solution': dict(by_sol),
           'prev_epoch_facts': prev_facts,
           'degraded_outcomes': degraded,
           'prev_epoch_degraded_counts': dict(prev_degraded)},
          open(os.path.join(RUN, 'q-level.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

# ---------- console summary ----------
print('HEALTH: records', len(E), 'step', len(steps), 'friction', len(fric),
      'unparseable', skips)
print('chunks', len(cov))
for ch, v in cov.items():
    print(' ', ch, 'phase', v['phase'], 'implement', v['implement'],
          'wrap', v['wrap-session'], 'new-session', v['new-session'],
          ('other ' + str(v['other'])) if v['other'] else '')
print('null-chunk step records:', len(nullchunk_steps))
print('untyped', sum(untyped_by_step.values()), '/', len(fric))
print('problem-fact fill', prob_fill, '/', len(steps), ' id fill', id_fill, '/', len(E))
print('TYPED groups', len(typed), 'above', sum(1 for g in typed if above(g)),
      '| UNIVERSAL groups', len(universal), 'above', sum(1 for g in universal if above(g)))
print('UNTYPED', len(un), '| CHAINS', len(chains), 'shapes>=2',
      sum(1 for k, v in shape.items() if v >= 2))
print('LEVEL facts', len(facts), dict(by_sol), '| degraded outcomes', len(degraded))
