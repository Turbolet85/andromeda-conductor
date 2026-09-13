"""Assemble the ledger record from the c-*.json twins. Writes {run_dir}/record.json only —
the append is a separate pinned command."""
import json, os, subprocess, datetime

RUN = os.path.dirname(os.path.abspath(__file__))
RUNREL = '.andromeda/runs/2026-09-13T11-34-19-code-audit'
J = lambda n: json.load(open(f'{RUN}/c-{n}.json', encoding='utf-8'))

SHA = '0f780c62651f1aaa9cd95b9316159f16288c3f6d'
BASE = '59d5b7c84bb3d6df2420790d111aebe19ed1bed7'
EPOCH = 'Epoch 6b — Polish & ship'

sz, dup, cx, gr, dead, cov, ch, hs, mut = (J('sizes'), J('duplication'), J('complexity'), J('graph'),
                                           J('dead'), J('coverage'), J('churn'), J('hotspots'),
                                           J('mutation'))
deadweb = J('deadweb')

rec = {
    'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
    'epoch': EPOCH,
    'mode': 'trend',
    'sha': SHA,
    'baseline_sha': BASE,
    'span': 1,
    'ancestry_broken': False,
    'head_overshoot': {
        'boundary_sha': 'b54e6ec',
        'note': ('HEAD is 3 chunks past the Epoch 6b boundary (the first three conductor-0.3.0 '
                 'Epoch-1 chunks). Source delta boundary..HEAD is ONE file: '
                 'crates/conductor-report/tests/matrix_ledger_gate.rs +61/-6 (a test file). '
                 'Every metric here therefore includes that file; nothing else in the overshoot '
                 'is a source path.'),
    },
    'tool_versions': {
        'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25',
        'cargo-machete': '0.9.2', 'cargo-mutants': '27.1.0', 'cargo-llvm-cov': '0.8.5',
        'cargo-nextest': '0.9.133', 'knip': '6.34.0',
        'code-graph': 'tree.db via scripts/code-graph.py',
        'lizard': '1.24.0 (available, NOT collected this run — see skips.complexity-web)',
        'rustc': '1.95.0 (59807616e 2026-04-14)',
    },
    'totals': {'loc': sz['loc'], 'files': sz['files'], 'units': 9},
    'duplication': {k: dup[k] for k in ('label', 'population', 'pct', 'duplicated_lines',
                                        'total_lines', 'clones', 'sources', 'split', 'top')},
    'complexity': {k: cx[k] for k in ('population', 'cyclomatic_p50', 'cyclomatic_p90',
                                      'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max',
                                      'functions', 'rs_files_scanned', 'top')},
    'sizes': {k: sz[k] for k in ('population', 'file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': {k: gr[k] for k in ('population', 'symbol_form', 'cycles', 'cycle_paths',
                                 'fan_in_top', 'fan_out', 'cross_unit_edges')},
    'dead': {k: dead[k] for k in ('population', 'unused_deps', 'zero_ref_candidates', 'chain',
                                  'top', 'false_positive_families_named_not_subtracted')},
    'dead_web': {'tool': 'knip 6.34.0', 'counts': deadweb['counts'], 'total': deadweb['total'],
                 'files_with_issues': deadweb['files_with_issues'],
                 'population': deadweb['population'], 'CAVEAT': deadweb['CAVEAT'],
                 'series': 'NEW at this boundary (tool-missing at the baseline) — no trend'},
    'coverage': {'line': cov['line'], 'branch': cov['branch'],
                 'lines_found': cov['lines_found'], 'lines_hit': cov['lines_hit'],
                 'population': cov['population']},
    'churn': {k: ch[k] for k in ('population', 'pct', 'files_churned', 'files_touched',
                                 'total_adds', 'churned_adds')},
    'hotspots': hs['top'],
    'mutation': mut,
    'commands': {
        'sizes': 'tokei --output json crates/',
        'duplication': f'jscpd crates --format rust --reporters json --output {RUNREL}/jscpd_rs --silent',
        'complexity': f'mkdir -p {RUNREL}/_rca_head && rust-code-analysis-cli --metrics -O json -o {RUNREL}/_rca_head -p crates   (summarizer filters to *.rs; the walk also visits the ui tree)',
        'dead_deps': 'cargo machete',
        'dead_symbols': f'python scripts/code-graph.py query {RUNREL} code-audit "SELECT s.symbol, s.file FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL;" rust',
        'dead_web': 'npx --no-install knip --reporter json   (cwd crates/conductor-tauri/ui; exit 1 = findings present, knip convention)',
        'graph': f'python scripts/code-graph.py query {RUNREL} code-audit "<cycles | fan-in LIMIT 20 | fan-out | count(*) FROM crate_edges>" rust',
        'coverage': f'cargo llvm-cov nextest --workspace --lcov --output-path {RUNREL}/_lcov.info',
        'churn': f"git log --numstat --format='commit %H' {BASE[:8]}..HEAD -- 'crates/**/*.rs' 'crates/**/*.ts' 'crates/**/*.tsx'",
        'hotspots': 'commits(churn window) x max cognitive per file over the FULL rca population (fallback cyclomatic, then 0 for non-Rust)',
        'mutation': f'cargo mutants -p {{unit}} --test-tool=nextest --jobs 2 --output {RUNREL}/mutants-{{unit}}',
        'mutation_conductor-core': f'cargo mutants -p conductor-core --test-tool=nextest --jobs 2 --shard 1/4 --output {RUNREL}/mutants-conductor-core',
    },
    'corrections': [],
    'skips': [
        {'metric': 'mutation-web', 'reason': 'tool-missing',
         'note': 'StrykerJS absent; recipe: npm i -D @stryker-mutator/core in crates/conductor-tauri/ui'},
        {'metric': 'complexity-web', 'reason': 'declined',
         'note': 'lizard 1.24.0 IS available (python -m lizard); not collected, matching the baseline\'s '
                 'deliberate decline, to keep the Rust-only complexity series comparable across all five '
                 'records — a separate web series can start at any boundary'},
        {'metric': 'mutation-conductor-emit', 'reason': 'declined',
         'note': 'touched in this window (458 mutants) but out of the operator-chosen baseline-parity scope'},
        {'metric': 'mutation-conductor-timeline', 'reason': 'declined',
         'note': 'touched in this window (38 mutants) but out of the operator-chosen baseline-parity scope'},
        {'metric': 'mutation-conductor-report', 'reason': 'declined',
         'note': 'touched in this window (89 mutants) but out of the operator-chosen baseline-parity scope; '
                 'note this is where the epoch\'s final chunk landed — first measurement still owed'},
        {'metric': 'mutation-conductor-faults', 'reason': 'declined',
         'note': 'touched in this window (28 mutants) but out of the operator-chosen baseline-parity scope'},
        {'metric': 'mutation-conductor-core', 'reason': 'budget-exhausted',
         'note': '504 mutants in the unit; ran --shard 1/4 as the baseline did, so the two are comparable; '
                 'the remaining 3 shards are untested this run',
         'tested': None, 'planned_full_unit': 504},
    ],
}
# fill the core shard's tested count from the measured result
core = (mut.get('counts') or {}).get('conductor-core')
if core:
    for s in rec['skips']:
        if s['metric'] == 'mutation-conductor-core':
            s['tested'] = core['mutants']

json.dump(rec, open(f'{RUN}/record.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
print('record.json written ·', len(json.dumps(rec, ensure_ascii=False)), 'bytes as one line')
print('units scored:', rec['mutation'].get('scores'))
print('skips:', len(rec['skips']))
