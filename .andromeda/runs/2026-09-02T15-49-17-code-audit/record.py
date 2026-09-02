"""Phase 3 — assemble the ledger record from the c-*.json twins.

Writes {run_dir}/record.json. The append is a separate, explicit step (the pinned
UTF-8 single-line recipe in audit-pass.md) so assembly can be re-run safely.
"""
import json, os, subprocess, sys

RD = '.andromeda/runs/2026-09-02T15-49-17-code-audit'
EPOCH = 'Epoch 5 — Verification surfaces'
BASELINE_SHA = 'b8f3332df9f12c679dd50fd967f5aaa18b84f657'


def load(n):
    p = os.path.join(RD, f'c-{n}.json')
    return json.load(open(p, encoding='utf-8')) if os.path.exists(p) else None


sha = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True,
                     text=True).stdout.strip()
ts = subprocess.run(['date', '-u', '+%Y-%m-%dT%H:%M:%SZ'], capture_output=True,
                    text=True).stdout.strip()

sizes = load('sizes')
dup = load('duplication')
cx = load('complexity')
graph = load('graph')
dead = load('dead-symbols')
churn = load('churn')
hot = load('hotspots')
mut = load('mutation') or {}
cov = load('coverage') or {'line': None, 'branch': None}
machete = load('dead-deps') or {'unused_deps': {}}

skips = [
    {'metric': 'dead-code-web', 'reason': 'tool-missing',
     'note': 'knip absent; recipe: npm i -D knip in crates/conductor-tauri/ui'},
    {'metric': 'mutation-web', 'reason': 'tool-missing',
     'note': 'StrykerJS absent; recipe: npm i -D @stryker-mutator/core'},
    # The reason enum (tool-missing | stack-absent | declined | budget-exhausted |
    # no-baseline) has no value for this outcome: the tier RAN, generated its plan,
    # and aborted on the unmutated baseline. Filing it under any existing value would
    # be false, so a sixth value is used and the gap is disclosed in proposals.md.
    {'metric': 'mutation-conductor-tauri', 'reason': 'baseline-test-failure',
     'note': ('43 mutants planned, 0 tested. cargo-mutants exit 4: "cargo test failed '
              'in an unmutated tree". Cause: commands.rs:516 assert_cmd cargo_bin('
              '"conductor") resolves via assert_cmd 2.2.2 legacy_cargo_bin -> '
              '<target_dir>/conductor.exe, which no declared dependency guarantees '
              '(conductor-tauri does not depend on conductor-cli). Green under '
              '--workspace and on this host under -p (a stale artifact from 16:11); '
              'absent in a fresh tree. SCHEMA GAP: the skips reason enum needs a '
              'sixth value.'),
     'enum_gap': True},
]
for u in ('conductor-core', 'conductor-cli', 'conductor-report',
          'conductor-timeline', 'conductor-emit', 'conductor-faults'):
    skips.append({'metric': f'mutation-{u}', 'reason': 'declined',
                  'note': 'out of scope: not touched in b8f3332..HEAD'})

record = {
    'ts': ts, 'epoch': EPOCH, 'mode': 'trend',
    'sha': sha, 'baseline_sha': BASELINE_SHA, 'span': 1,
    'ancestry_broken': False,
    'tool_versions': {
        'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25',
        'cargo-machete': '0.9.2', 'cargo-mutants': '27.1.0',
        'cargo-llvm-cov': '0.8.5', 'cargo-nextest': '0.9.133',
        'code-graph': 'tree.db via scripts/code-graph.py',
        'lizard': '1.24.0 (available, NOT collected this run — see recipes.lizard)',
    },
    'totals': {'loc': sizes['totals']['loc'], 'files': sizes['totals']['files'],
               'units': 9},
    'duplication': {k: dup[k] for k in ('label', 'population', 'pct',
                                        'duplicated_lines', 'total_lines',
                                        'clones', 'sources', 'top', 'split')},
    'complexity': {'population': cx['population'],
                   'cyclomatic_p50': cx['cyclomatic_p50'],
                   'cyclomatic_p90': cx['cyclomatic_p90'],
                   'cognitive_p50': cx['cognitive_p50'],
                   'cognitive_p90': cx['cognitive_p90'],
                   'over_ceiling': cx['over_ceiling'],
                   'max': cx['max'], 'top': cx['top'],
                   'functions': cx['functions']},
    'sizes': {'population': sizes['population'], 'file_p50': sizes['file_p50'],
              'file_p90': sizes['file_p90'], 'file_max': sizes['file_max'],
              'over_800': sizes['over_800'], 'top': sizes['top']},
    'graph': graph,
    'dead': {'unused_deps': machete['unused_deps'],
             'zero_ref_candidates': dead['zero_ref_candidates'],
             'chain': dead['chain'],
             'top': dead['top'],
             'false_positive_families_named_not_subtracted':
                 dead['false_positive_families_named_not_subtracted']},
    'coverage': {'line': cov.get('line'), 'branch': cov.get('branch')},
    'churn': {'pct': churn['pct'], 'files_churned': churn['files_churned'],
              'files_touched': churn['files_touched'],
              'total_adds': churn['total_adds'],
              'churned_adds': churn['churned_adds'],
              'population': churn['population']},
    'hotspots': hot['top'],
    'mutation': {'scoped_units': mut.get('scoped_units', []),
                 'unit_states': mut.get('unit_states', {}),
                 'scores': mut.get('scores', {}),
                 'counts': mut.get('counts', {}),
                 'score_formula': mut.get('score_formula'),
                 'survivors': mut.get('survivors', []),
                 'timeouts': mut.get('timeouts', [])},
    'commands': {
        'sizes': 'tokei --output json crates/',
        'duplication': ('jscpd crates --format rust --reporters json '
                        '--output {run_dir}/jscpd_rs --silent'),
        'complexity': ('mkdir -p {run_dir}/_rca_head && rust-code-analysis-cli '
                       '--metrics -O json -o {run_dir}/_rca_head -p crates'),
        'dead_deps': 'cargo machete',
        'dead_symbols': ('python scripts/code-graph.py query {run_dir} code-audit '
                         '"SELECT s.symbol, s.file FROM symbol s LEFT JOIN refs r '
                         'ON r.callee = s.symbol WHERE r.callee IS NULL;" rust'),
        'graph': ('python scripts/code-graph.py query {run_dir} code-audit '
                  '"<cycles | fan-in LIMIT 20 | fan-out | count(*) FROM crate_edges>" rust'),
        'coverage': ('cargo llvm-cov nextest --workspace --lcov '
                     '--output-path {run_dir}/_lcov.info'),
        'churn': ("git log --numstat --format='commit %H' b8f3332..HEAD -- "
                  "'crates/**/*.rs' 'crates/**/*.ts' 'crates/**/*.tsx'"),
        'mutation': ('cargo mutants -p {unit} --test-tool=nextest --jobs 2 '
                     '--output {run_dir}/mutants-{unit}'),
    },
    'recipes': {
        'sizes': 'tokei over crates/; Rust *.rs only; nearest-rank p50/p90 on per-file code lines',
        'duplication': 'jscpd crates --format rust (Rust-only)',
        'complexity': ('rust-code-analysis kind==function spaces under crates/; ceiling '
                       'cognitive > 15; top sorted by COGNITIVE desc, rows '
                       '[fn, file, cognitive, cyclomatic]; max = max cognitive'),
        'dead': ('WORKSPACE-WIDE, one query over all symbols. raw zero-ref pub symbols '
                 '-> exclude `tests` path SEGMENT in the SYMBOL path (AFTER stripping the '
                 'SCIP prefix `rust-analyzer cargo {crate} {version} ` — otherwise the '
                 'prefix fuses into the first segment and inline #[cfg(test)] symbols, '
                 'whose path is a LEADING `tests/`, survive) UNION the FILE path '
                 '-> subtract ENTRY POINTS only (main(). / /bin/) -> residual = candidates. '
                 'HEAD chain 772 -> 36 -> 32.'),
        'graph': ('fan_in symbols SHORTENED to `{crate} {path}` (strip '
                  '`rust-analyzer cargo ` and the version token) — the baseline stores '
                  'this form; comparing raw SCIP strings makes every row a false entrant'),
        'churn': ('git log --numstat baseline..HEAD over crates/ *.rs|*.ts|*.tsx; adds in '
                  "a file's 2nd..nth touching commit count as churn"),
        'hotspots': ('commits x per-file max cognitive over the RUST rca output; a TS/TSX '
                     'file scores 0 (no Rust cognitive, and the tokei KLOC fallback is '
                     'Rust-only) — same limitation as the baseline, so the trend is '
                     'comparable, but wdio.conf.ts (5 commits, the epoch top) scores 0'),
        'lizard': ('lizard 1.24.0 IS on PATH via `python -m lizard` (the pip script shim '
                   'is not) — the Epoch-4 record recorded complexity-web as tool-missing, '
                   'which may have been a probe artifact. NOT collected this run: adding a '
                   'TS population mid-trend would break comparability with both prior '
                   'records. A separate web-complexity series can start at any boundary.'),
    },
    'skips': skips,
}

json.dump(record, open(os.path.join(RD, 'record.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)
print('record.json assembled')
print('  totals    ', record['totals'])
print('  coverage  ', record['coverage'])
print('  mutation  ', record['mutation']['scores'], record['mutation']['unit_states'])
print('  skips     ', len(skips))
