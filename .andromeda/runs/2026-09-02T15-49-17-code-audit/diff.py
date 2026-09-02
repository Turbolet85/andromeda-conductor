"""Phase 4 — diff current vs baseline, apply the threshold table.

Thresholds (audit-pass.md, fire only at span=1 which holds here):
  new-cycle          graph.cycles > baseline                       -> proposal always
  duplication-up     pct >= base + 0.5pt AND >= +15% relative      -> proposal
  complexity-creep   over_ceiling >= base + 3 AND >= +25%          -> proposal
  dead-growth        zero_ref_candidates >= base + 5               -> proposal
  coverage-drop      line <= base - 2pts                           -> proposal
  mutation-drop      a unit's score <= its last score - 10pts      -> proposal
  monotonic          worsened at BOTH of the last two diffs        -> proposal
  top-N entrants / span>1 / churn(<2 records) / trend-break        -> informational
"""
import json, os

RD = '.andromeda/runs/2026-09-02T15-49-17-code-audit'


def load(n):
    p = os.path.join(RD, f'c-{n}.json')
    return json.load(open(p, encoding='utf-8')) if os.path.exists(p) else None


recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')
        if l.strip()]
b = recs[-1]          # Epoch 4
b0 = recs[0]          # Epoch 3 (for the monotonic check)

cur = {n: load(n) for n in ('sizes', 'duplication', 'complexity', 'graph',
                            'dead-symbols', 'churn', 'hotspots', 'mutation',
                            'coverage')}

findings = {'proposals': [], 'informational': [], 'below': []}


def rec(bucket, check, metric, was, now, note=''):
    findings[bucket].append({'check': check, 'metric': metric, 'was': was,
                             'now': now, 'note': note})


rows = []


def cmp_row(label, was, now, better='lower'):
    if was is None or now is None:
        d = None
    else:
        d = round(now - was, 4)
    rows.append([label, was, now, d, better])
    return d


# ---- totals / sizes ----
cmp_row('totals.loc', b['totals']['loc'], cur['sizes']['totals']['loc'], 'n/a')
cmp_row('totals.files', b['totals']['files'], cur['sizes']['totals']['files'], 'n/a')
d_p50 = cmp_row('sizes.file_p50', b['sizes']['file_p50'], cur['sizes']['file_p50'])
d_p90 = cmp_row('sizes.file_p90', b['sizes']['file_p90'], cur['sizes']['file_p90'])
d_max = cmp_row('sizes.file_max', b['sizes']['file_max'], cur['sizes']['file_max'])
d_o8 = cmp_row('sizes.over_800', b['sizes']['over_800'], cur['sizes']['over_800'])

# ---- duplication ----
d_pct = cmp_row('duplication.pct', b['duplication']['pct'], cur['duplication']['pct'])
cmp_row('duplication.clones', b['duplication']['clones'], cur['duplication']['clones'])
cmp_row('duplication.duplicated_lines', b['duplication']['duplicated_lines'],
        cur['duplication']['duplicated_lines'])
bp, np_ = b['duplication']['pct'], cur['duplication']['pct']
if np_ >= bp + 0.5 and np_ >= bp * 1.15:
    rec('proposals', 'duplication-up', 'duplication.pct', bp, np_)
else:
    rec('below', 'duplication-up', 'duplication.pct', bp, np_,
        'below threshold (needs >= +0.5pt AND >= +15% relative)')

# ---- complexity ----
d_oc = cmp_row('complexity.over_ceiling', b['complexity']['over_ceiling'],
               cur['complexity']['over_ceiling'])
cmp_row('complexity.cognitive_p90', b['complexity']['cognitive_p90'],
        cur['complexity']['cognitive_p90'])
cmp_row('complexity.cyclomatic_p90', b['complexity']['cyclomatic_p90'],
        cur['complexity']['cyclomatic_p90'])
cmp_row('complexity.max', b['complexity']['max']['val'], cur['complexity']['max']['val'])
bo, no = b['complexity']['over_ceiling'], cur['complexity']['over_ceiling']
if no >= bo + 3 and no >= bo * 1.25:
    rec('proposals', 'complexity-creep', 'complexity.over_ceiling', bo, no)
else:
    rec('below', 'complexity-creep', 'complexity.over_ceiling', bo, no,
        'below threshold (needs >= +3 AND >= +25%)')

# ---- graph ----
d_cy = cmp_row('graph.cycles', b['graph']['cycles'], cur['graph']['cycles'])
cmp_row('graph.cross_unit_edges', b['graph']['cross_unit_edges'],
        cur['graph']['cross_unit_edges'])
if cur['graph']['cycles'] > b['graph']['cycles']:
    rec('proposals', 'new-cycle', 'graph.cycles', b['graph']['cycles'],
        cur['graph']['cycles'], str(cur['graph']['cycle_paths']))
else:
    rec('below', 'new-cycle', 'graph.cycles', b['graph']['cycles'],
        cur['graph']['cycles'], 'no cycle')

# ---- dead ----
bd = b['dead']['zero_ref_candidates']
nd = cur['dead-symbols']['zero_ref_candidates']
d_dead = cmp_row('dead.zero_ref_candidates', bd, nd)
if nd >= bd + 5:
    rec('proposals', 'dead-growth', 'dead.zero_ref_candidates', bd, nd)
else:
    rec('below', 'dead-growth', 'dead.zero_ref_candidates', bd, nd,
        'below threshold (needs >= +5)')

# ---- coverage ----
if cur['coverage']:
    bc = b['coverage']['line']
    nc = cur['coverage']['line']
    cmp_row('coverage.line', bc, nc, 'higher')
    if nc is not None and bc is not None and nc <= bc - 2:
        rec('proposals', 'coverage-drop', 'coverage.line', bc, nc)
    else:
        rec('below', 'coverage-drop', 'coverage.line', bc, nc,
            'below threshold (needs <= -2pts)')

# ---- churn (informational: the ledger now holds 2 records, so it is comparable) ----
cmp_row('churn.pct', b['churn']['pct'], cur['churn']['pct'])
cmp_row('churn.files_churned', b['churn']['files_churned'], cur['churn']['files_churned'])
rec('informational', 'churn', 'churn.pct', b['churn']['pct'], cur['churn']['pct'],
    'no threshold defined in the table; reported as movement')

# ---- mutation ----
if cur['mutation']:
    states = cur['mutation'].get('unit_states', {})
    for unit, sc in (cur['mutation']['scores'] or {}).items():
        prior = (b['mutation']['scores'] or {}).get(unit)
        cmp_row(f'mutation.{unit}', prior, sc, 'higher')
        if sc is None:
            # No score because the tier ABORTED, not because the unit is new — a
            # null here is evidence of a skip, never a zero and never a baseline.
            rec('proposals', 'mutation-tier-aborted', f'mutation.{unit}', prior, None,
                f"unit state: {states.get(unit)}; see skips[] for the cause")
        elif prior is None:
            rec('informational', 'mutation-new-unit', f'mutation.{unit}', None, sc,
                'no prior score — fresh sub-baseline')
        elif sc <= prior - 10:
            rec('proposals', 'mutation-drop', f'mutation.{unit}', prior, sc)
        else:
            rec('below', 'mutation-drop', f'mutation.{unit}', prior, sc,
                'below threshold (needs <= -10pts)')

# ---- monotonic (worsened at BOTH of the last two diffs) ----
mono = []
pairs = [('duplication.pct', b0['duplication']['pct'], b['duplication']['pct'],
          cur['duplication']['pct'], 'lower'),
         ('complexity.over_ceiling', b0['complexity']['over_ceiling'],
          b['complexity']['over_ceiling'], cur['complexity']['over_ceiling'], 'lower'),
         ('dead.zero_ref_candidates', b0['dead']['zero_ref_candidates'],
          b['dead']['zero_ref_candidates'], nd, 'lower'),
         ('sizes.file_max', b0['sizes']['file_max'], b['sizes']['file_max'],
          cur['sizes']['file_max'], 'lower'),
         ('sizes.over_800', b0['sizes']['over_800'], b['sizes']['over_800'],
          cur['sizes']['over_800'], 'lower')]
for name, v0, v1, v2, better in pairs:
    if None in (v0, v1, v2):
        continue
    worse1 = v1 > v0 if better == 'lower' else v1 < v0
    worse2 = v2 > v1 if better == 'lower' else v2 < v1
    if worse1 and worse2:
        mono.append([name, v0, v1, v2])
        rec('proposals', 'monotonic', name, f'{v0} -> {v1}', v2,
            'worsened at both of the last two diffs')

# ---- top-N entrants (informational) ----
def entrants(cur_top, base_top, key=lambda x: x[0]):
    base = {key(x) for x in base_top}
    return [x for x in cur_top if key(x) not in base]


ent = {
    'sizes': entrants(cur['sizes']['top'], b['sizes']['top']),
    'complexity': entrants(cur['complexity']['top'], b['complexity']['top']),
    'hotspots': entrants(cur['hotspots']['top'], b['hotspots']),
    'duplication': entrants(cur['duplication']['top'], b['duplication']['top'],
                            key=lambda x: (x[0], x[1])),
    'fan_in': entrants(cur['graph']['fan_in_top'], b['graph']['fan_in_top']),
}
for k, v in ent.items():
    if v:
        rec('informational', 'top-N-entrant', k, None, v)

out = {'span': 1, 'baseline_sha': b['sha'], 'baseline_epoch': b['epoch'],
       'rows': rows, 'entrants': ent, 'monotonic': mono, **findings}
json.dump(out, open(os.path.join(RD, 'q-diff.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

print(f"{'metric':34s} {'E4':>10s} {'E5':>10s} {'delta':>10s}")
for label, was, now, d, better in rows:
    print(f'{label:34s} {str(was):>10s} {str(now):>10s} {str(d):>10s}')
print()
for bucket in ('proposals', 'informational', 'below'):
    print(f'--- {bucket.upper()} ({len(findings[bucket])}) ---')
    for f in findings[bucket]:
        print(f"  {f['check']:20s} {f['metric']:34s} {f['was']} -> "
              f"{str(f['now'])[:110]}  {f['note'][:70]}")
