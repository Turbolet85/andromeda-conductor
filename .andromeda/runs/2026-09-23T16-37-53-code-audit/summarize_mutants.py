"""Per-unit cargo-mutants summarizer (collectors.md C1). Run from the repo root:
python -X utf8 {run_dir}/summarize_mutants.py {run_dir} {unit} [shard-label]
Writes {run_dir}/c-mutation-{unit}.json ONLY when the invocation is complete by the tool's own markers."""
import json, os, re, sys

R, unit = sys.argv[1], sys.argv[2]
shard = sys.argv[3] if len(sys.argv) > 3 else None
base = os.path.join(R, 'mutants-' + unit, 'mutants.out')
oc = json.load(open(os.path.join(base, 'outcomes.json'), encoding='utf-8'))
mj = os.path.join(base, 'mutants.json')
planned = len(json.load(open(mj, encoding='utf-8'))) if os.path.exists(mj) else None
total = oc.get('total_mutants')
if not oc.get('end_time'):
    sys.exit('INCOMPLETE: end_time unset')
if planned is not None and total != planned:
    sys.exit('INCOMPLETE: total_mutants %s != len(mutants.json) %s' % (total, planned))
baseline_ok = oc['outcomes'][0].get('summary') == 'Success' if oc.get('outcomes') else None
c = {k: oc.get(k, 0) for k in ('caught', 'missed', 'timeout', 'unviable')}
score = round(100.0 * c['caught'] / (c['caught'] + c['missed']), 2) if (c['caught'] + c['missed']) else None
surv = []
mt = os.path.join(base, 'missed.txt')
if os.path.exists(mt):
    for line in open(mt, encoding='utf-8'):
        line = line.strip()
        if not line: continue
        m = re.match(r'^(.+?):(\d+):\d+: (.*)$', line)
        surv.append([('%s:%s' % (m.group(1), m.group(2))).replace('\\', '/'), m.group(3)] if m else [line, ''])
timeouts = []
tt = os.path.join(base, 'timeout.txt')
if os.path.exists(tt):
    timeouts = [l.strip().replace('\\', '/') for l in open(tt, encoding='utf-8') if l.strip()]
state = 'complete (%s/%s tested)' % (total, planned if planned is not None else '?')
if shard: state += ' of the unit — shard %s' % shard
if not baseline_ok and not total:
    state = 'baseline-test-failure'
out = {"unit": unit, "unit_state": state, "baseline_success": baseline_ok, "shard": shard,
       "counts": {"mutants": total, **c}, "score": score, "score_formula": "caught/(caught+missed)",
       "unit_states_sources": {"total_mutants": total, "len_mutants_json": planned},
       "survivors": surv, "timeouts": timeouts}
with open(os.path.join(R, 'c-mutation-%s.json' % unit), 'w', encoding='utf-8', newline='') as f:
    json.dump(out, f, ensure_ascii=False, indent=1)
print(unit, state, 'score', score, c)
