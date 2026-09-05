"""Assemble the ledger record (record.json) from the c-*.json twins + baseline, and compute the diff/judge (q-diff.json)."""
import json, os, sys, glob, subprocess, datetime, re
RUN = sys.argv[1]
EPOCH = 'Epoch 6a — Verification follow-ups'
recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try: recs.append(json.loads(line))
    except ValueError: pass
base, prior = recs[-1], (recs[-2] if len(recs) > 1 else None)
L = lambda n: json.load(open(os.path.join(RUN, f'c-{n}.json'), encoding='utf-8'))
sizes_t, dup, cx, graph, dead, cov, churn, hot = L('sizes'), L('duplication'), L('complexity'), L('graph'), L('dead'), L('coverage'), L('churn'), L('hotspots')
sha = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
tool_versions = {'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'cargo-machete': '0.9.2', 'cargo-mutants': '27.1.0',
                 'cargo-llvm-cov': '0.8.5', 'cargo-nextest': '0.9.133', 'code-graph': 'tree.db via scripts/code-graph.py',
                 'lizard': '1.24.0 (available, NOT collected this run — see recipes.lizard)', 'rustc': '1.95.0 (59807616e 2026-04-14)'}
# ---------- mutation ----------
mut_files = sorted(glob.glob(os.path.join(RUN, 'c-mutation-*.json')))
units = {}
for f in mut_files:
    m = json.load(open(f, encoding='utf-8')); units[m['unit']] = m
scoped = ['conductor-tauri', 'conductor-run', 'conductor-verify', 'conductor-cli', 'conductor-core']
mutation = {'scoped_units': scoped,
            'unit_states': {u: (units[u]['state'] + (f" ({units[u]['tested']}/{units[u]['planned_in_scope']} tested" + (f" of {units[u]['planned_full_unit']} in the unit — shard 1/4" if u == 'conductor-core' else '') + ')') if u in units else 'not-run') for u in scoped},
            'scores': {u: (units[u]['score'] if u in units else None) for u in scoped},
            'counts': {u: (units[u]['counts'] if u in units else {'mutants': None, 'caught': 0, 'missed': 0, 'timeout': 0, 'unviable': 0}) for u in scoped},
            'planned_full_unit': {u: (units[u]['planned_full_unit'] if u in units else None) for u in scoped},
            'score_formula': 'caught/(caught+missed)  [timeout and unviable excluded]',
            'survivors': [s for u in scoped if u in units for s in units[u]['survivors']],
            'timeouts': [t for u in scoped if u in units for t in units[u]['timeouts']]}
# ---------- skips ----------
skips = [{'metric': 'dead-code-web', 'reason': 'tool-missing', 'note': 'knip absent (not in package.json, not in node_modules); recipe: npm i -D knip in crates/conductor-tauri/ui'},
         {'metric': 'mutation-web', 'reason': 'tool-missing', 'note': 'StrykerJS absent; recipe: npm i -D @stryker-mutator/core in crates/conductor-tauri/ui'},
         {'metric': 'complexity-web', 'reason': 'declined', 'note': 'lizard 1.24.0 IS available (python -m lizard); not collected to keep the Rust-only complexity series comparable with the three prior records — a separate web series can start at any boundary'}]
for u in ['conductor-timeline', 'conductor-emit', 'conductor-faults', 'conductor-report']:
    skips.append({'metric': f'mutation-{u}', 'reason': 'declined', 'note': f'out of scope: not touched in {base["sha"][:7]}..HEAD'})
core = units.get('conductor-core')
if core and core['planned_full_unit'] and core['tested'] < core['planned_full_unit']:
    skips.append({'metric': 'mutation-conductor-core', 'reason': 'budget-exhausted',
                  'note': f"504 mutants in the unit; the operator-ratified 15-min per-unit cap was honored by running --shard 1/4 ({core['planned_in_scope']} mutants, {core['tested']} tested) rather than killing cargo's child tree at the cap; the remaining 3 shards are untested this run",
                  'tested': core['tested'], 'planned_full_unit': core['planned_full_unit']})
for u in scoped:
    if u not in units: skips.append({'metric': f'mutation-{u}', 'reason': 'budget-exhausted', 'note': 'runner did not reach this unit'})
commands = {
 'sizes': 'tokei --output json crates/',
 'duplication': 'jscpd crates --format rust --reporters json --output {run_dir}/jscpd_rs --silent',
 'complexity': 'mkdir -p {run_dir}/_rca_head && rust-code-analysis-cli --metrics -O json -o {run_dir}/_rca_head -p crates   (summarizer filters to *.rs; the walk also visits the ui tree)',
 'dead_deps': 'cargo machete',
 'dead_symbols': 'python scripts/code-graph.py query {run_dir} code-audit "SELECT s.symbol, s.file FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL;" rust',
 'graph': 'python scripts/code-graph.py query {run_dir} code-audit "<cycles | fan-in LIMIT 20 | fan-out | count(*) FROM crate_edges>" rust',
 'coverage': cov['command'],
 'churn': f"git log --numstat --format='commit %H' {base['sha'][:7]}..HEAD -- 'crates/**/*.rs' 'crates/**/*.ts' 'crates/**/*.tsx'",
 'mutation': 'cargo mutants -p {unit} --test-tool=nextest --jobs 2 --output {run_dir}/mutants-{unit}',
 'mutation_conductor-core': 'cargo mutants -p conductor-core --test-tool=nextest --jobs 2 --shard 1/4 --output {run_dir}/mutants-conductor-core'}
recipes = dict(base.get('recipes', {}))
recipes['dead'] = recipes.get('dead', '') + ' | this run: FULL candidate list kept in c-dead.json (all_candidates) so the next diff can name entrants, which the capped top-20 could not'
record = {'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'), 'epoch': EPOCH, 'mode': 'trend', 'sha': sha,
          'baseline_sha': base['sha'], 'span': 1, 'ancestry_broken': False, 'tool_versions': tool_versions,
          'totals': sizes_t['totals'],
          'duplication': {k: v for k, v in dup.items() if k not in ('statistics_total_keys', 'all_pairs_with_start_lines', 'pairs_involving_files_touched_in_span')},
          'complexity': {k: v for k, v in cx.items() if k != 'over_ceiling_list'},
          'sizes': sizes_t['sizes'], 'graph': graph,
          'dead': {k: v for k, v in dead.items() if k != 'all_candidates'} | {'all_candidates': dead['all_candidates']},
          'coverage': {'line': cov['line'], 'branch': cov['branch']},
          'churn': {k: v for k, v in churn.items() if k != 'per_file_commits'},
          'hotspots': hot['top'], 'mutation': mutation, 'commands': commands, 'recipes': recipes, 'corrections': [], 'skips': skips}
json.dump(record, open(os.path.join(RUN, 'record.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
# ---------- diff / judge ----------
def g(r, *ks):
    d = r
    for k in ks:
        d = d.get(k) if isinstance(d, dict) else None
    return d
scalars = {  # name: (path, worse_direction) — health scalars only; totals (loc/files) are context and are reported, never judged
 'dup_pct': (('duplication', 'pct'), 'up'), 'clones': (('duplication', 'clones'), 'up'),
 'over_ceiling': (('complexity', 'over_ceiling'), 'up'), 'file_max': (('sizes', 'file_max'), 'up'), 'over_800': (('sizes', 'over_800'), 'up'),
 'cycles': (('graph', 'cycles'), 'up'), 'cross_unit_edges': (('graph', 'cross_unit_edges'), 'up'),
 'dead_candidates': (('dead', 'zero_ref_candidates'), 'up'), 'coverage_line': (('coverage', 'line'), 'down'), 'churn_pct': (('churn', 'pct'), 'up')}
rows = []
for name, (path, worse) in scalars.items():
    cur, b, p = g(record, *path), g(base, *path), (g(prior, *path) if prior else None)
    d1 = None if (b is None or p is None) else (b - p)
    d2 = None if (cur is None or b is None) else (cur - b)
    def worsened(d): return d is not None and ((d > 0) if worse == 'up' else (d < 0))
    rows.append({'metric': name, 'prior': p, 'baseline': b, 'current': cur, 'delta': d2, 'prior_delta': d1,
                 'worsened_now': worsened(d2), 'worsened_prior': worsened(d1), 'monotonic': worsened(d1) and worsened(d2)})
checks = []
def fire(check, cls, detail): checks.append({'check': check, 'class': cls, 'detail': detail})
cyc = g(record, 'graph', 'cycles'); fire('new-cycle', 'proposal' if cyc > g(base, 'graph', 'cycles') else 'below', f"cycles {g(base,'graph','cycles')} -> {cyc}")
dp, db = g(record, 'duplication', 'pct'), g(base, 'duplication', 'pct')
fire('duplication-up', 'proposal' if (dp >= db + 0.5 and dp >= db * 1.15) else 'below', f"dup pct {db} -> {round(dp,2)} ({round(dp-db,2):+} pt)")
oc, ob = g(record, 'complexity', 'over_ceiling'), g(base, 'complexity', 'over_ceiling')
fire('complexity-creep', 'proposal' if (oc >= ob + 3 and oc >= ob * 1.25) else 'below', f"over_ceiling {ob} -> {oc}")
dz, dzb = g(record, 'dead', 'zero_ref_candidates'), g(base, 'dead', 'zero_ref_candidates')
fire('dead-growth', 'proposal' if dz >= dzb + 5 else 'below', f"zero-ref candidates {dzb} -> {dz} ({dz-dzb:+})")
cl, cb = g(record, 'coverage', 'line'), g(base, 'coverage', 'line')
fire('coverage-drop', 'proposal' if cl <= cb - 2 else 'below', f"line coverage {cb} -> {cl} ({round(cl-cb,2):+} pt)")
for u in scoped:
    s, sb = g(record, 'mutation', 'scores', u), g(base, 'mutation', 'scores', u)
    if s is None or sb is None: fire(f'mutation-drop:{u}', 'informational', f"score {sb} -> {s} (no prior score or not run: no trend judgment)")
    else: fire(f'mutation-drop:{u}', 'proposal' if s <= sb - 10 else 'below', f"score {sb} -> {s} ({round(s-sb,2):+} pt)")
mono = [r for r in rows if r['monotonic']]
for r in mono: fire('monotonic', 'proposal', f"{r['metric']}: {r['prior']} -> {r['baseline']} -> {r['current']} (worsened at both diffs)")
if not mono: fire('monotonic', 'below', 'no tracked scalar worsened at both of the last two diffs')
tv_base = base.get('tool_versions', {})
breaks = [k for k in tool_versions if k in tv_base and tv_base[k] != tool_versions[k] and k != 'lizard']
fire('trend-break', 'informational' if breaks else 'below', f"tool version changes vs baseline: {breaks or 'none'}")
json.dump({'scalars': rows, 'checks': checks,
           'entrants': {'sizes_top': [x for x in record['sizes']['top'] if x[0] not in {y[0] for y in base['sizes']['top']}],
                        'complexity_top': [x for x in record['complexity']['top'] if x[0] not in {y[0] for y in base['complexity']['top']}],
                        'duplication_top': dup['new_in_top_vs_baseline'], 'fan_in_top': graph['fan_in_entrants'], 'hotspots_top': hot['entrants_vs_baseline'],
                        'dead_top20_new_vs_baseline_top20': [x for x in dead['all_candidates'] if x[0] not in {y[0] for y in base['dead']['top']}]}},
          open(os.path.join(RUN, 'q-diff.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
print('record.json written; sha', sha[:7], 'ts', record['ts'])
for r in rows: print(f"  {r['metric']:18} {r['prior']!s:>8} -> {r['baseline']!s:>8} -> {r['current']!s:>8}   delta {r['delta']!s:>8}   monotonic={r['monotonic']}")
for c in checks: print(f"  [{c['class']:13}] {c['check']}: {c['detail']}")
print('  survivors total', len(mutation['survivors']), 'timeouts', len(mutation['timeouts']), 'skips', len(skips))
