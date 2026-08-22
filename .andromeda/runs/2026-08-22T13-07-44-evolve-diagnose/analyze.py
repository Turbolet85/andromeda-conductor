"""Evolve diagnosis analysis — Epoch 4 (Lifecycle & delegated timing).
Read-only over the ledger; writes only into this run dir."""
import json, collections, re, sys

LEDGER = '.andromeda/friction-log.ndjson'
RUN = '.andromeda/runs/2026-08-22T13-07-44-evolve-diagnose/'
TARGET = 'Epoch 4 - Lifecycle & delegated timing'   # folded form

EXPECTED = {'phase': 5, 'implement': 3, 'wrap-session': 5, 'new-session': 1}


def fold_epoch(e):
    if e is None:
        return None
    return re.sub(r'\s[-–—]\s', ' - ', e).strip()


def fold_skill(s):
    if s is None:
        return None
    return re.sub(r'^andromeda-', '', s)


def load():
    recs, skips = [], 0
    for i, line in enumerate(open(LEDGER, encoding='utf-8')):
        if not line.strip():
            continue
        try:
            r = json.loads(line)
        except ValueError:
            skips += 1
            continue
        r['_line'] = i
        r['_epoch'] = fold_epoch(r.get('epoch'))
        r['_skill'] = fold_skill(r.get('skill'))
        recs.append(r)
    return recs, skips


def exclusions(recs):
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
    return excl_ids, excl_probs


def norm_problem(r, excl_probs):
    p = r.get('problem')
    if not p:
        return []
    facts = p if isinstance(p, list) else [p]
    rid = r.get('id')
    if (rid, None) in excl_probs:
        return []
    out = []
    for i, f in enumerate(facts):
        if (rid, i) in excl_probs:
            continue
        out.append(f)
    return out


WK = dict(iterations=1, retries=1, reformulations=1, dialogue_rounds=2, halted=3, soft_exit=3)


def weight(rows):
    w = 0
    for r in rows:
        w += 1
        im = r.get('impact') or {}
        for k, m in WK.items():
            w += m * (im.get(k) or 0)
    return w


def main():
    recs, skips = load()
    excl_ids, excl_probs = exclusions(recs)

    epoch = [r for r in recs if r['_epoch'] == TARGET and r.get('id') not in excl_ids]
    steps = [r for r in epoch if r.get('kind') == 'step']
    fric = [r for r in epoch if r.get('kind') == 'friction']

    # ---------- Stage 0: health ----------
    chunks = sorted({r.get('chunk') for r in steps if r.get('chunk')})
    per_chunk = {}
    for c in chunks:
        cs = [r for r in steps if r.get('chunk') == c]
        bysk = collections.Counter(r['_skill'] for r in cs)
        per_chunk[c] = {
            'step_records': len(cs),
            'by_skill': dict(bysk),
            'expected': EXPECTED,
            'gaps': {k: EXPECTED[k] - bysk.get(k, 0) for k in EXPECTED
                     if k != 'new-session' and bysk.get(k, 0) < EXPECTED[k]},
            'extra': {k: bysk[k] - EXPECTED[k] for k in bysk
                      if k in EXPECTED and bysk[k] > EXPECTED[k]},
            'steps_fired': sorted({f"{r['_skill']}/{r['step']}" for r in cs}),
        }
    nullchunk = [r for r in steps if not r.get('chunk')]
    untyped = [r for r in fric if r.get('untyped')]
    ut_by_step = collections.Counter(f"{r['_skill']}/{r['step']}" for r in untyped)
    st_runs = collections.Counter(f"{r['_skill']}/{r['step']}" for r in steps)
    fr_by_step = collections.Counter(f"{r['_skill']}/{r['step']}" for r in fric)
    with_facts = [r for r in steps if norm_problem(r, excl_probs)]
    allfacts = [f for r in steps for f in norm_problem(r, excl_probs)]
    outcomes = collections.Counter(r.get('outcome') for r in steps)
    idfill = sum(1 for r in epoch if r.get('id'))
    health = {
        'epoch': TARGET,
        'records': len(epoch), 'step': len(steps), 'friction': len(fric),
        'unparseable_whole_ledger': skips,
        'chunks': chunks, 'per_chunk': per_chunk,
        'null_chunk_step_records': [{'id': r.get('id'), 'skill': r['_skill'],
                                     'step': r.get('step')} for r in nullchunk],
        'untyped_total': len(untyped),
        'untyped_rate_overall': round(len(untyped) / len(fric), 3) if fric else 0,
        'untyped_by_step': {k: [v, ut_by_step[k] and fr_by_step[k]] for k, v in ut_by_step.items()},
        'friction_by_step': dict(fr_by_step),
        'step_runs': dict(st_runs),
        'problem_fact_fill': f"{len(with_facts)}/{len(steps)}",
        'problem_facts_total': len(allfacts),
        'problem_fact_solutions': dict(collections.Counter(f.get('solution') for f in allfacts)),
        'problem_fact_natures': dict(collections.Counter(f.get('nature') for f in allfacts)),
        'outcomes': dict(outcomes),
        'id_fill': f"{idfill}/{len(epoch)}",
        'excluded_by_retraction': {'friction_ids': sorted(excl_ids),
                                   'problem_blocks': sorted(map(list, excl_probs))},
    }
    json.dump(health, open(RUN + 'q-health.json', 'w', encoding='utf-8'), indent=1)

    # ---------- Stage 1: typed ----------
    UNIVERSAL = {'tooling.host-shell', 'contract.narrow-basis-claim',
                 'contract.premise-falsified', 'contract.structural-blind-spot'}
    groups = collections.defaultdict(list)
    for r in fric:
        t = r.get('type') or 'UNTYPED'
        groups[(r['_skill'], r['step'], t)].append(r)
    typed_rows = []
    for (sk, st, t), rows in groups.items():
        if t == 'UNTYPED':
            continue
        im = collections.Counter()
        for r in rows:
            for k, v in (r.get('impact') or {}).items():
                im[k] += v
        n = len(rows)
        halty = sum((r.get('impact') or {}).get('halted', 0) +
                    (r.get('impact') or {}).get('soft_exit', 0) for r in rows)
        typed_rows.append({
            'key': f"{sk}/{st}/{t}", 'skill': sk, 'step': st, 'type': t, 'n': n,
            'weight': weight(rows),
            'rate': round(n / st_runs[f"{sk}/{st}"], 2) if st_runs[f"{sk}/{st}"] else None,
            'step_runs': st_runs[f"{sk}/{st}"],
            'chunks': sorted({r.get('chunk') for r in rows if r.get('chunk')}),
            'impact': dict(im),
            'above_threshold': n >= 3 or (n >= 2 and halty > 0),
            'ids': [r.get('id') for r in rows],
            'whats': [r.get('what') for r in rows],
        })
    typed_rows.sort(key=lambda d: (-d['weight'], -d['n']))

    # universal types grouped by type ALONE across steps
    uni_rows = []
    for t in sorted(UNIVERSAL):
        rows = [r for r in fric if r.get('type') == t]
        if not rows:
            continue
        halty = sum((r.get('impact') or {}).get('halted', 0) +
                    (r.get('impact') or {}).get('soft_exit', 0) for r in rows)
        uni_rows.append({
            'type': t, 'n': len(rows), 'weight': weight(rows),
            'steps': sorted({f"{r['_skill']}/{r['step']}" for r in rows}),
            'chunks': sorted({r.get('chunk') for r in rows if r.get('chunk')}),
            'above_threshold': len(rows) >= 3 or (len(rows) >= 2 and halty > 0),
            'ids': [r.get('id') for r in rows],
            'whats': [r.get('what') for r in rows],
        })
    uni_rows.sort(key=lambda d: -d['weight'])
    json.dump({'per_step_type': typed_rows, 'universal_by_type': uni_rows},
              open(RUN + 'q-typed.json', 'w', encoding='utf-8'), indent=1)

    # ---------- Stage 2 raw: untyped dump ----------
    ut = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['_skill'],
           'step': r.get('step'), 'what': r.get('what'), 'impact': r.get('impact'),
           'artifacts': r.get('artifacts'), 'evidence': r.get('evidence')} for r in untyped]
    json.dump(ut, open(RUN + 'q-untyped.json', 'w', encoding='utf-8'), indent=1)

    # ---------- Stage 3: chains ----------
    # anchors: input.* frictions, and step records with thin/wrong/missing consumed verdicts
    anchors = []
    for r in fric:
        if (r.get('type') or '').startswith('input.'):
            anchors.append({'kind': 'friction', 'id': r.get('id'), 'chunk': r.get('chunk'),
                            'consumer': f"{r['_skill']}/{r['step']}", 'artifact': None,
                            'what': r.get('what'), 'artifacts': r.get('artifacts')})
    for r in steps:
        for c in (r.get('consumed') or []):
            if c.get('quality') in ('thin', 'wrong', 'missing'):
                anchors.append({'kind': 'step-consumed', 'id': r.get('id'),
                                'chunk': r.get('chunk'),
                                'consumer': f"{r['_skill']}/{r['step']}",
                                'artifact': c.get('artifact'), 'quality': c.get('quality'),
                                'note': c.get('note')})
    # lineage join within the same chunk
    chain_rows = []
    for a in anchors:
        if not a.get('artifact') or not a.get('chunk'):
            continue
        prods = [r for r in steps
                 if r.get('chunk') == a['chunk']
                 and r['_line'] < next((x['_line'] for x in steps if x.get('id') == a['id']),
                                       10 ** 9)
                 and any(p.get('artifact') == a['artifact'] for p in (r.get('produced') or []))]
        for p in prods:
            sig = [s for pr in (p.get('produced') or [])
                   if pr.get('artifact') == a['artifact'] for s in (pr.get('signals') or [])]
            pfric = [f for f in fric if f.get('chunk') == p.get('chunk')
                     and f['_skill'] == p['_skill'] and f.get('step') == p.get('step')]
            chain_rows.append({
                'chunk': a['chunk'], 'artifact': a['artifact'],
                'producer': f"{p['_skill']}/{p['step']}", 'producer_id': p.get('id'),
                'producer_outcome': p.get('outcome'), 'producer_signals': sig,
                'producer_frictions': [f.get('type') or 'UNTYPED' for f in pfric],
                'consumer': a['consumer'], 'consumer_id': a['id'],
                'quality': a.get('quality'), 'note': a.get('note'),
            })
    shape = collections.defaultdict(list)
    for c in chain_rows:
        shape[(c['producer'], c['artifact'], c['consumer'])].append(c)
    shapes = [{'producer': k[0], 'artifact': k[1], 'consumer': k[2],
               'chunks': sorted({c['chunk'] for c in v}), 'n': len(v), 'cases': v}
              for k, v in shape.items()]
    shapes.sort(key=lambda d: -len(d['chunks']))
    json.dump({'anchors': anchors, 'chains': chain_rows, 'shapes': shapes},
              open(RUN + 'q-chains.json', 'w', encoding='utf-8'), indent=1)

    # ---------- Stage 4 raw: problem facts + prior-epoch lookback ----------
    facts = []
    for r in steps:
        for i, f in enumerate(norm_problem(r, excl_probs)):
            facts.append({'id': r.get('id'), 'chunk': r.get('chunk'),
                          'step': f"{r['_skill']}/{r.get('step')}", 'index': i,
                          'nature': f.get('nature'), 'solution': f.get('solution'),
                          'note': f.get('note')})
    prior = []
    for r in recs:
        if r.get('kind') == 'step' and r['_epoch'] and r['_epoch'] != TARGET:
            for i, f in enumerate(norm_problem(r, excl_probs)):
                prior.append({'epoch': r['_epoch'], 'chunk': r.get('chunk'),
                              'step': f"{r['_skill']}/{r.get('step')}",
                              'nature': f.get('nature'), 'solution': f.get('solution'),
                              'note': f.get('note')})
    json.dump({'epoch_facts': facts, 'prior_epoch_facts': prior},
              open(RUN + 'q-level-raw.json', 'w', encoding='utf-8'), indent=1)

    # ---------- console summary ----------
    print('=== HEALTH ===')
    print('records', len(epoch), 'step', len(steps), 'friction', len(fric),
          'unparseable', skips)
    print('chunks', len(chunks))
    for c, d in per_chunk.items():
        print(f"  {c[:58]:58} steps={d['step_records']:3} gaps={d['gaps']} extra={d['extra']}")
    print('null-chunk step records:', len(nullchunk),
          [f"{r['_skill']}/{r.get('step')}" for r in nullchunk])
    print('untyped', len(untyped), 'of', len(fric),
          f"= {round(100*len(untyped)/len(fric),1)}%")
    print('untyped by step:', dict(ut_by_step))
    print('friction by step:', dict(fr_by_step))
    print('step runs:', dict(st_runs))
    print('problem-fact fill', health['problem_fact_fill'], 'facts', len(allfacts),
          health['problem_fact_solutions'], health['problem_fact_natures'])
    print('outcomes', dict(outcomes), 'id fill', health['id_fill'])
    print()
    print('=== TYPED (per skill/step/type) ===')
    for d in typed_rows:
        flag = 'YES' if d['above_threshold'] else '   '
        print(f"  [{flag}] n={d['n']:2} w={d['weight']:3} rate={d['rate']} "
              f"({d['step_runs']} runs) {d['key']}  chunks={len(d['chunks'])}")
    print()
    print('=== UNIVERSAL (by type alone) ===')
    for d in uni_rows:
        flag = 'YES' if d['above_threshold'] else '   '
        print(f"  [{flag}] n={d['n']:2} w={d['weight']:3} {d['type']} "
              f"steps={d['steps']} chunks={len(d['chunks'])}")
    print()
    print('=== CHAIN SHAPES (>=2 chunks flagged) ===')
    for s in shapes:
        print(f"  chunks={len(s['chunks']):2} n={s['n']:2} "
              f"{s['producer']} -[{s['artifact']}]-> {s['consumer']}")
    print('anchors total', len(anchors),
          '(friction', sum(1 for a in anchors if a['kind'] == 'friction'),
          '/ consumed-verdict', sum(1 for a in anchors if a['kind'] == 'step-consumed'), ')')
    print()
    print('=== PROBLEM FACTS ===', len(facts), 'in epoch |', len(prior), 'prior-epoch')


main()
