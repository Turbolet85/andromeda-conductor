"""Phase 3+4: assemble record.json from the c-*.json twins, then diff + judge against the baseline chain (by sha)."""
import glob, json, os, re, sys
from datetime import datetime, timezone

RD = sys.argv[1]
RDN = RD.rstrip('/')
C = lambda n: json.load(open(os.path.join(RD, f'c-{n}.json'), encoding='utf-8'))
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')

HEAD = '88de1805c3e253277f2fdff4a259d7904896dc2d'
BASE = 'e799b9e0a41e1a39302f4a99ccb36829f5f1b0b1'
BOUNDARY = '07c8f113a4bbc501543b5d3d3792cff3eec982a5'
EPOCH = 'Epoch 5 — Polish & ship'

recs, bad = [], 0
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try: recs.append(json.loads(line))
    except ValueError: bad += 1
# baseline = LAST record whose sha == BASE among records not this run's own (this run's sha is HEAD)
prior = [r for r in recs if r.get('sha') != HEAD or r.get('epoch') != EPOCH]
baseline = [r for r in prior if r.get('sha') == BASE][-1]
b0 = [r for r in prior if r.get('sha') == baseline.get('baseline_sha')][-1]

sz, cx, dp, gr, dd, dw, cw, cv, ch, hs = (C('sizes'), C('complexity'), C('duplication'), C('graph'), C('dead'), C('dead-web'),
                                          C('complexity-web'), C('coverage'), C('churn'), C('hotspots'))
dead_deps = C('dead-deps')

# ---- mutation
mut_files = sorted(glob.glob(os.path.join(RD, 'c-mutation-*.json')))
units, unit_states, scores, counts, survivors, not_measured, timeouts, skips_mut = [], {}, {}, {}, [], [], [], []
for f in mut_files:
    m = json.load(open(f, encoding='utf-8'))
    u = m['unit']; units.append(u)
    if m['state'] == 'baseline-test-failure':
        unit_states[u] = 'baseline-test-failure'; skips_mut.append({'metric': f'mutation-{u}', 'reason': 'baseline-test-failure'}); continue
    unit_states[u] = m['unit_state']
    counts[u] = m['counts']
    survivors += m['survivors']; not_measured += m['not_measured']; timeouts += [[u] + t for t in m['timeouts']]
    if m['state'] == 'unviable-dominant':
        skips_mut.append({'metric': f'mutation-{u}', 'reason': 'unviable-dominant', 'note': f"counts {m['counts']}"})
    else:
        scores[u] = m['score']
    if m.get('shard'):
        tot = {'conductor-core': 559, 'conductor-emit': 456}.get(u)
        skips_mut.append({'metric': f'mutation-{u}', 'reason': 'budget-exhausted',
                          'note': f"ran --shard {m['shard']} only ({m['counts']['mutants']}/{m['counts']['mutants']} of the shard; {tot} in the unit); the other 3 shards are untested this run"})
log = open(os.path.join(RD, '_mutants.log'), encoding='utf-8').read() if os.path.isfile(os.path.join(RD, '_mutants.log')) else ''
for u in ['conductor-faults', 'conductor-timeline', 'conductor-tauri', 'conductor-report', 'conductor-cli', 'conductor-run', 'conductor-verify', 'conductor-core', 'conductor-emit']:
    if u not in units:
        units.append(u); unit_states[u] = 'budget-exhausted'
        skips_mut.append({'metric': f'mutation-{u}', 'reason': 'budget-exhausted', 'note': 'invocation incomplete at its 15-minute cap'})

R = f'.andromeda/runs/{os.path.basename(RDN)}'
rec = {
    'ts': datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'), 'epoch': EPOCH, 'mode': 'trend',
    'sha': HEAD, 'baseline_sha': BASE, 'span': 2, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': BOUNDARY, 'commits': 2, 'files': [],
                       'note': 'HEAD carries two 0-pending wrap commits past the Epoch 5 boundary (b4bfe7e route adaptation, 88de180 U35 registry migration); no source-path file changed. Operator-confirmed trend mode (the founder ruled the audit live, relayed by the overseer).'},
    'host': {'target': 'x86_64-unknown-linux-gnu', 'os': 'linux',
             'note': 'FIRST record measured on the Linux dev host; all 7 prior records were measured on the Windows host (x86_64-pc-windows-msvc). Every metric that executes or compiles code (coverage, mutation incl. cfg-excluded mutants, rca/tokei are host-neutral) is read with this host change beside it.'},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'cargo-machete': '0.9.2',
                      'cargo-mutants': '27.1.0', 'cargo-llvm-cov': '0.9.1', 'cargo-nextest': '0.9.146', 'code-graph': 'd0425fb1',
                      'knip': '6.34.0', 'rustc': '1.95.0', 'lizard': '1.24.0'},
    'totals': sz['totals'],
    'duplication': {k: dp[k] for k in ('population', 'pct', 'duplicated_lines', 'total_lines', 'clones', 'sources', 'split', 'top')},
    'complexity': {k: cx[k] for k in ('population', 'cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max', 'functions', 'rs_files_scanned', 'top')},
    'complexity_web': {k: cw[k] for k in ('population', 'functions', 'cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'top')},
    'sizes': {k: sz[k] for k in ('population', 'file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': gr,
    'dead': {'population': dd['population'], 'unused_deps': dead_deps['unused_deps'], 'zero_ref_candidates': dd['zero_ref_candidates'],
             'chain': dd['chain'], 'top': dd['top']},
    'dead_web': {'tool': 'knip 6.34.0', 'counts': dw['counts'], 'total': dw['total'], 'files_with_issues': dw['files_with_issues']},
    'coverage': {k: cv[k] for k in ('population', 'line', 'branch', 'lines_found', 'lines_hit')},
    'churn': {k: ch[k] for k in ('population', 'pct', 'files_churned', 'files_touched', 'total_adds', 'churned_adds')},
    'hotspots': hs['top'],
    'mutation': {'scoped_units': units, 'unit_states': unit_states, 'scores': scores, 'counts': counts,
                 'score_formula': 'caught/(caught+missed)', 'survivors': survivors,
                 'host': 'x86_64-unknown-linux-gnu', 'not_measured': not_measured, 'timeouts': timeouts,
                 'runner_portability': 'conductor-core under cargo-mutants copy mode (no .git in the temp copy): unmutated baseline FAILED - secret_scan_gate::the_workspace_holds_no_secret_shaped_string panics because `git ls-files` exits 128 (the gate has no subject); the gate arrived 2026-09-24 (this epoch). Re-run with --copy-vcs true: 140/140. Evidence: evidence-core-nocopyvcs.json + _mutants-conductor-core-nocopyvcs.log', 'project_union_verdict': 'test-plan registers cargo-mutants (named by pointer, never fetched or merged)'},
    'commands': {
        'sizes': 'tokei --output json crates/   (summarizer: language Rust, node_modules/target excluded)',
        'duplication': f'jscpd crates --format rust --reporters json --output {R}/jscpd_rs --silent',
        'complexity': f'rust-code-analysis-cli --metrics -O json -o {R}/_rca_head -p crates   (summarizer filters to crates/**/*.rs outside node_modules/target; the walk also visits the ui tree)',
        'complexity_web': 'lizard crates/conductor-tauri/ui/src --csv   (auto-detect .ts/.tsx; cyclomatic only)',
        'dead_deps': 'cargo machete',
        'dead_symbols': f'python scripts/code-graph.py query {R} code-audit "SELECT s.symbol, s.file FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL;" rust   (classing: tests/ exclusion = UNION of symbol-path (SCIP prefix stripped) AND file-path segments, then main() entry points)',
        'dead_web': 'npx --no-install knip --reporter json   (cwd crates/conductor-tauri/ui; exit 0, no findings)',
        'graph': f'python scripts/code-graph.py query {R} code-audit "<cycles | fan-in LIMIT 20 | fan-out | count(*) FROM crate_edges>" rust',
        'coverage': f'cargo llvm-cov nextest --workspace --lcov --output-path {R}/_lcov.info',
        'churn': f"git log --reverse --numstat --format='commit %H' {BASE[:8]}..HEAD -- 'crates/**/*.rs' 'crates/**/*.ts' 'crates/**/*.tsx'",
        'hotspots': 'commits(churn window) x max cognitive per file over the FULL rca population (fallback 0 for non-Rust)',
        'mutation': f'cargo mutants -p {{unit}} --test-tool=nextest --jobs 4 --output {R}/mutants-{{unit}}   (timeout 900 per unit)',
        'mutation_conductor-core': f'cargo mutants -p conductor-core --test-tool=nextest --jobs 4 --copy-vcs true --output {R}/mutants-conductor-core --shard 1/4   (a first invocation WITHOUT --copy-vcs into {R}/mutants-conductor-core-nocopyvcs failed its unmutated baseline: see mutation.runner_portability)',
        'mutation_conductor-emit': f'cargo mutants -p conductor-emit --test-tool=nextest --jobs 4 --output {R}/mutants-conductor-emit --shard 1/4',
        'head_overshoot': f"git log --format=%H -1 -S'2026-10-04-host-portable-tauri-ipc-tests · complete' -- .andromeda/master-route.md  |  git rev-list --count {BOUNDARY}..HEAD  |  git diff --numstat {BOUNDARY}..HEAD -- 'crates/**/*.rs' 'crates/**/*.ts' 'crates/**/*.tsx'",
    },
    'corrections': [],
    'skips': [{'metric': 'mutation-web', 'reason': 'tool-missing', 'note': 'StrykerJS absent; recipe: npm i -D @stryker-mutator/core in crates/conductor-tauri/ui'}] + skips_mut,
}
# per-unit survivor asserts (audit-pass.md Caps)
for f in mut_files:
    m = json.load(open(f, encoding='utf-8'))
    if m['state'] == 'baseline-test-failure': continue
    assert len(m['survivors']) == m['counts']['missed'] and len({tuple(s) for s in m['survivors']}) == len(m['survivors']), m['unit']
    assert len(m['not_measured']) == m['counts']['not_measured'], m['unit']
json.dump(rec, open(os.path.join(RD, 'record.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

# ---- Phase 4: diff + judge
TB = {'duplication': 'jscpd', 'coverage': 'cargo-llvm-cov', 'complexity': 'rust-code-analysis', 'sizes': 'tokei', 'dead': 'code-graph', 'mutation': 'cargo-mutants'}
def tb(metric, r0, r1):
    t = TB[metric]; a, b = (r0.get('tool_versions') or {}).get(t), (r1.get('tool_versions') or {}).get(t)
    lead = lambda v: (re.match(r'[0-9A-Za-z.\-]+', v or '') or [None])[0]
    return None if lead(a) is None else (lead(a) != lead(b))
get = {'duplication.pct': lambda r: r['duplication'].get('pct'), 'complexity.over_ceiling': lambda r: r['complexity'].get('over_ceiling'),
       'dead.zero_ref_candidates': lambda r: r['dead'].get('zero_ref_candidates'), 'sizes.file_max': lambda r: r['sizes'].get('file_max'),
       'sizes.over_800': lambda r: r['sizes'].get('over_800'), 'coverage.line': lambda r: r['coverage'].get('line')}
lower_worse = {'coverage.line'}
mono = {}
for k, g in get.items():
    v0, v1, v2 = g(b0), g(baseline), g(rec)
    metric = k.split('.')[0]
    brk = tb(metric, baseline, rec)
    if None in (v0, v1, v2): mono[k] = {'values': [v0, v1, v2], 'evaluable': False}; continue
    w = (lambda a, b: b < a) if k in lower_worse else (lambda a, b: b > a)
    mono[k] = {'values': [v0, v1, v2], 'worse_d1': w(v0, v1), 'worse_d2': w(v1, v2), 'trend_break_d2': brk,
               'fires': w(v0, v1) and w(v1, v2) and not brk}
deltas = {k: [get[k](baseline), get[k](rec)] for k in get}
deltas['duplication.clones'] = [baseline['duplication']['clones'], rec['duplication']['clones']]
deltas['duplication.duplicated_lines'] = [baseline['duplication']['duplicated_lines'], rec['duplication']['duplicated_lines']]
deltas['duplication.total_lines'] = [baseline['duplication'].get('total_lines'), rec['duplication']['total_lines']]
deltas['totals.loc'] = [baseline['totals']['loc'], rec['totals']['loc']]
deltas['totals.files'] = [baseline['totals']['files'], rec['totals']['files']]
deltas['graph.cycles'] = [baseline['graph']['cycles'], rec['graph']['cycles']]
deltas['graph.cross_unit_edges'] = [baseline['graph']['cross_unit_edges'], rec['graph']['cross_unit_edges']]
deltas['complexity.functions'] = [baseline['complexity'].get('functions'), rec['complexity']['functions']]
deltas['dead_web.total'] = [baseline.get('dead_web', {}).get('total'), rec['dead_web']['total']]
deltas['mutation.scores'] = {u: [baseline['mutation']['scores'].get(u), rec['mutation']['scores'].get(u)] for u in units}
old_top = {(a, b) for a, b, _ in baseline['duplication']['top']}
new_top_entrants = [t for t in rec['duplication']['top'] if (t[0], t[1]) not in old_top]
old_hs = {p for p, _ in baseline['hotspots']}
old_cx = {(f, fl) for f, fl, *_ in baseline['complexity']['top']}
old_sz = {p for p, _ in baseline['sizes']['top']}
old_dead = {s for s, _ in baseline['dead']['top']}
old_surv = {(s.rsplit(':', 1)[0] if s.count(':') == 2 else s, t) for s, t in baseline['mutation']['survivors']}
cur_surv_fl = [(s.rsplit(':', 1)[0], t) for s, t in rec['mutation']['survivors']]
judge = {'baseline': {'sha': BASE, 'epoch': baseline['epoch'], 'baseline_sha': baseline['baseline_sha'], 'head_overshoot_commits': baseline['head_overshoot'].get('commits')},
         'span': 2, 'unparseable_ledger_lines': bad, 'deltas': deltas, 'monotonic': mono,
         'trend_breaks': {m: tb(m, baseline, rec) for m in TB},
         'entrants': {'duplication_top': new_top_entrants,
                      'hotspots': [h for h in rec['hotspots'] if h[0] not in old_hs],
                      'complexity_top': [c for c in rec['complexity']['top'] if (c[0], c[1]) not in old_cx],
                      'sizes_top': [s for s in rec['sizes']['top'] if s[0] not in old_sz],
                      'dead_top': [d for d in rec['dead']['top'] if d[0] not in old_dead]},
         'survivors_new_vs_baseline_fileline': [list(x) for x in cur_surv_fl if x not in old_surv],
         'survivors_gone_vs_baseline_fileline': [list(x) for x in old_surv if x not in set(cur_surv_fl)]}
json.dump(judge, open(os.path.join(RD, 'c-diff.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
print(json.dumps({'mono': {k: v.get('fires') for k, v in mono.items()}, 'tb': judge['trend_breaks'], 'scores': deltas['mutation.scores']}))
