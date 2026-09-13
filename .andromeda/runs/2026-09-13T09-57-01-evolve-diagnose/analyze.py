"""Evolve diagnosis analysis — Stages 0-4 for one epoch. Read-only on the ledger; writes q-*.json into the run dir."""
import json, re, collections, os, sys

RUN = os.path.dirname(os.path.abspath(__file__))
LEDGER = '.andromeda/friction-log.ndjson'
TARGET = 'Epoch 6b — Polish & ship'

UNIVERSAL = {
    'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
    'contract.structural-blind-spot', 'contract.token-proxy-check',
    'tooling.output-cap-overflow', 'contract.skill-reference-drift',
    'contract.grammar-irregularity',
}
EXPECTED = {'phase': 5, 'implement': 3, 'wrap-session': 5, 'new-session': 1}


def norm_epoch(s):
    s = re.sub(r'^\s*#+\s*', '', (s or '')).strip()
    return s.replace('–', '-').replace('—', '-')


def norm_skill(s):
    return re.sub(r'^andromeda-', '', s or '')


def load():
    recs, skips = [], 0
    for line in open(LEDGER, encoding='utf-8'):
        if not line.strip():
            continue
        try:
            recs.append(json.loads(line))
        except ValueError:
            skips += 1
    return recs, skips


def exclusions(recs):
    by_id = {r['id']: r for r in recs if r.get('id')}
    ids, probs = set(), set()
    for r in recs:
        for t in (r.get('retracts') or []):
            if not isinstance(t, dict):
                continue
            tid, scope = t.get('id'), t.get('scope')
            tgt = by_id.get(tid)
            if tid is None or tgt is None or scope not in ('record', 'problem'):
                continue
            if scope == 'record':
                if tgt.get('kind') != 'step':
                    ids.add(tid)
            else:
                probs.add((tid, t.get('index')))
    return ids, probs


def problem_facts(r, excl_probs):
    """Normalized problem list for a step record, minus retracted targets."""
    p = r.get('problem')
    facts = [] if p is None else (p if isinstance(p, list) else [p])
    rid = r.get('id')
    if (rid, None) in excl_probs:
        return []
    return [f for i, f in enumerate(facts) if (rid, i) not in excl_probs]


def weight(r):
    im = r.get('impact') or {}
    g = lambda k: im.get(k, 0) or 0
    return (1 + g('iterations') + g('retries') + g('reformulations')
            + 2 * g('dialogue_rounds') + 3 * g('halted') + 3 * g('soft_exit'))


def impact_sum(rows):
    keys = ['iterations', 'retries', 'reformulations', 'dialogue_rounds',
            'extra_reads', 'halted', 'soft_exit', 'deferred']
    out = {}
    for k in keys:
        v = sum((r.get('impact') or {}).get(k, 0) or 0 for r in rows)
        if v:
            out[k] = v
    return out


def main():
    recs, skips = load()
    excl_ids, excl_probs = exclusions(recs)
    for r in recs:
        r['_skill'] = norm_skill(r.get('skill'))
        r['_epoch'] = norm_epoch(r.get('epoch'))
    tgt = norm_epoch(TARGET)
    keep = [r for r in recs if r['_epoch'] == tgt and r.get('id') not in excl_ids]
    steps = [r for r in keep if r.get('kind') == 'step']
    fric = [r for r in keep if r.get('kind') == 'friction']
    dropped = [r.get('id') for r in recs if r['_epoch'] == tgt and r.get('id') in excl_ids]

    # ---------- Stage 0: mechanism health ----------
    per_chunk = collections.defaultdict(lambda: collections.Counter())
    for r in steps:
        per_chunk[r.get('chunk')][(r['_skill'], r.get('step'))] += 1
    chunk_cov = {}
    for ch, c in per_chunk.items():
        by_skill = collections.Counter()
        for (sk, st), n in c.items():
            by_skill[sk] += 1
        chunk_cov[str(ch)] = {
            'steps_by_skill': dict(by_skill),
            'expected': {k: EXPECTED[k] for k in by_skill if k in EXPECTED},
            'gaps': {k: EXPECTED[k] - by_skill[k] for k in by_skill
                     if k in EXPECTED and by_skill[k] < EXPECTED[k]},
            'step_ids': sorted({f"{sk}/{st}" for (sk, st) in c}),
        }
    untyped_by_step = collections.Counter()
    typed_by_step = collections.Counter()
    for r in fric:
        k = f"{r['_skill']}/{r.get('step')}"
        if r.get('untyped') or not r.get('type'):
            untyped_by_step[k] += 1
        else:
            typed_by_step[k] += 1
    prob_fill = sum(1 for r in steps if problem_facts(r, excl_probs))
    id_fill = sum(1 for r in keep if r.get('id'))
    outcomes = collections.Counter(r.get('outcome') for r in steps)
    health = {
        'epoch': TARGET, 'records': len(keep), 'step': len(steps), 'friction': len(fric),
        'unparseable_whole_ledger': skips,
        'retracted_dropped_in_epoch': dropped,
        'chunks': len([c for c in per_chunk if c]),
        'chunk_coverage': chunk_cov,
        'untyped_by_step': dict(untyped_by_step.most_common()),
        'typed_by_step': dict(typed_by_step.most_common()),
        'untyped_rate_overall': round(sum(untyped_by_step.values()) / max(1, len(fric)), 3),
        'problem_fact_fill': f"{prob_fill}/{len(steps)}",
        'id_fill': f"{id_fill}/{len(keep)}",
        'outcomes': dict(outcomes),
    }
    json.dump(health, open(f'{RUN}/q-health.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

    # ---------- Stage 1: typed patterns ----------
    denom = collections.Counter(f"{r['_skill']}/{r.get('step')}" for r in steps)
    groups = collections.defaultdict(list)
    for r in fric:
        t = r.get('type')
        if not t:
            continue
        if t in UNIVERSAL or t.startswith('recall.'):
            groups[('TYPE-ALONE', t)].append(r)
        else:
            groups[(f"{r['_skill']}/{r.get('step')}", t)].append(r)
    typed = []
    for (scope, t), rows in groups.items():
        n = len(rows)
        imp = impact_sum(rows)
        halts = imp.get('halted', 0) + imp.get('soft_exit', 0)
        typed.append({
            'key': f"{scope} :: {t}", 'scope': scope, 'type': t, 'n': n,
            'weight': sum(weight(r) for r in rows),
            'impact': imp,
            'halt_bearing': halts,
            'rate': (round(n / denom[scope], 2) if scope in denom else None),
            'chunks': sorted({str(r.get('chunk')) for r in rows}),
            'above_threshold': bool(n >= 3 or (n >= 2 and halts > 0)),
            'cases': [{'id': r.get('id'), 'chunk': r.get('chunk'),
                       'skill_step': f"{r['_skill']}/{r.get('step')}",
                       'what': r.get('what'), 'impact': r.get('impact'),
                       'artifacts': r.get('artifacts'), 'evidence': r.get('evidence')}
                      for r in rows],
        })
    typed.sort(key=lambda g: (-g['weight'], -g['n']))
    json.dump({'denominators': dict(denom), 'groups': typed},
              open(f'{RUN}/q-typed.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

    # ---------- Stage 2: untyped ----------
    un = [{'id': r.get('id'), 'chunk': r.get('chunk'),
           'skill_step': f"{r['_skill']}/{r.get('step')}", 'what': r.get('what'),
           'impact': r.get('impact'), 'artifacts': r.get('artifacts'), 'evidence': r.get('evidence')}
          for r in fric if (r.get('untyped') or not r.get('type'))]
    json.dump(un, open(f'{RUN}/q-untyped.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

    # ---------- Stage 3: chains ----------
    order = {id(r): i for i, r in enumerate(keep)}
    chains = []
    by_chunk_steps = collections.defaultdict(list)
    for r in steps:
        by_chunk_steps[r.get('chunk')].append(r)
    for ch, srecs in by_chunk_steps.items():
        srecs = sorted(srecs, key=lambda r: order[id(r)])
        for i, cons in enumerate(srecs):
            for c in (cons.get('consumed') or []):
                if c.get('quality') not in ('thin', 'wrong', 'missing'):
                    continue
                art = c.get('artifact')
                for prod in srecs[:i]:
                    for p in (prod.get('produced') or []):
                        if p.get('artifact') != art:
                            continue
                        chains.append({
                            'chunk': ch, 'artifact': art,
                            'producer': f"{prod['_skill']}/{prod.get('step')}",
                            'producer_outcome': prod.get('outcome'),
                            'producer_signals': p.get('signals'),
                            'producer_note': p.get('note'),
                            'consumer': f"{cons['_skill']}/{cons.get('step')}",
                            'quality': c.get('quality'), 'consumer_note': c.get('note'),
                            'producer_id': prod.get('id'), 'consumer_id': cons.get('id'),
                        })
    shape = collections.defaultdict(list)
    for c in chains:
        shape[(c['producer'], c['artifact'], c['consumer'])].append(c)
    shapes = [{'shape': ' -> '.join(k), 'n_chunks': len({c['chunk'] for c in v}),
               'n': len(v), 'quality': sorted({c['quality'] for c in v}),
               'chunks': sorted({str(c['chunk']) for c in v}), 'cases': v}
              for k, v in shape.items()]
    shapes.sort(key=lambda s: (-s['n_chunks'], -s['n']))
    # input.* anchored frictions
    input_fric = [{'id': r.get('id'), 'chunk': r.get('chunk'),
                   'skill_step': f"{r['_skill']}/{r.get('step')}", 'type': r.get('type'),
                   'what': r.get('what')} for r in fric if (r.get('type') or '').startswith('input.')]
    json.dump({'shapes': shapes, 'input_frictions': input_fric},
              open(f'{RUN}/q-chains.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

    # ---------- Stage 4: level ----------
    facts = []
    for r in steps:
        for i, f in enumerate(problem_facts(r, excl_probs)):
            if not isinstance(f, dict):
                continue
            facts.append({'id': r.get('id'), 'index': i, 'chunk': r.get('chunk'),
                          'skill_step': f"{r['_skill']}/{r.get('step')}",
                          'nature': f.get('nature'), 'solution': f.get('solution'),
                          'note': f.get('note')})
    by_sol = collections.Counter(f['solution'] for f in facts)
    by_nat = collections.Counter(f['nature'] for f in facts)
    degraded = [{'id': r.get('id'), 'chunk': r.get('chunk'),
                 'skill_step': f"{r['_skill']}/{r.get('step')}", 'outcome': r.get('outcome')}
                for r in steps if r.get('outcome') in ('ok-degraded', 'halted-resolved', 'soft-exit', 'aborted')]
    json.dump({'facts': facts, 'by_solution': dict(by_sol), 'by_nature': dict(by_nat),
               'non_ok_outcomes': degraded},
              open(f'{RUN}/q-level.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

    # ---------- console ----------
    print(f"epoch={TARGET} records={len(keep)} step={len(steps)} friction={len(fric)} "
          f"chunks={health['chunks']} dropped_retracted={len(dropped)}")
    print(f"untyped {sum(untyped_by_step.values())}/{len(fric)} "
          f"({health['untyped_rate_overall']}) · problem-fill {health['problem_fact_fill']} "
          f"· id-fill {health['id_fill']} · outcomes {dict(outcomes)}")
    above = [g for g in typed if g['above_threshold']]
    print(f"\nSTAGE1: {len(typed)} groups · {len(above)} above threshold")
    for g in above:
        print(f"  n={g['n']:<3} w={g['weight']:<4} halt={g['halt_bearing']} "
              f"chunks={len(g['chunks'])} {g['key']}")
    print(f"\nSTAGE2: {len(un)} untyped records")
    print(f"\nSTAGE3: {len(shapes)} distinct shapes; >=2 chunks: "
          f"{len([s for s in shapes if s['n_chunks'] >= 2])} · input.* frictions {len(input_fric)}")
    for s in shapes[:12]:
        print(f"  chunks={s['n_chunks']} n={s['n']} {s['quality']} {s['shape']}")
    print(f"\nSTAGE4: {len(facts)} problem-facts · by_solution {dict(by_sol)} · by_nature {dict(by_nat)}")
    print(f"  non-ok step outcomes: {len(degraded)} {collections.Counter(d['outcome'] for d in degraded)}")


main()
