"""Assemble the ledger record from the c-*.json twins (audit-pass.md §Schema).

Adds two fields beyond the pinned schema, at the operator's direction for trend run #2:
  `recipes`  — the classing/population recipe per metric, so a later run reproduces the
               population from the LEDGER alone instead of a prior run dir (which the
               skill may not read).
  `commands` — the firing form actually run, per metric (the reproducibility contract
               collectors.md names but the schema had no slot for).
"""
import json, os, glob, sys

RD = '.andromeda/runs/2026-08-22T13-31-54-code-audit'
HEAD = 'b8f3332df9f12c679dd50fd967f5aaa18b84f657'
BASE = '8cba57dc5291da32bfefbe1eb0ccfae3ea390e09'


def j(p):
    return json.load(open(os.path.join(RD, p), encoding='utf-8'))


sizes = j('c-sizes.json')
cx = j('c-complexity.json')
dead = j('c-dead.json')
cov = j('c-coverage.json')
churn = j('c-churn.json')
dup = j('c-duplication.json')
graph = j('c-graph.json')

mutation = {'scoped_units': [], 'scores': {}, 'survivors': []}
mut_path = os.path.join(RD, 'c-mutation.json')
if os.path.exists(mut_path):
    mutation = j('c-mutation.json')

rec = {
    'ts': '2026-08-22T13:31:54Z',
    'epoch': 'Epoch 4 — Lifecycle & delegated timing',
    'mode': 'trend',
    'sha': HEAD,
    'baseline_sha': BASE,
    'span': 1,
    'ancestry_broken': False,
    'tool_versions': {
        'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25',
        'cargo-machete': '0.9.2', 'cargo-mutants': '27.1.0',
        'cargo-llvm-cov': '0.8.5', 'cargo-nextest': '0.9.133',
        'code-graph': 'tree.db via scripts/code-graph.py',
    },
    'totals': {'loc': sizes['loc'], 'files': sizes['files'], 'units': 9},
    'duplication': dup,
    'complexity': {
        'cyclomatic_p50': cx['cyclomatic_p50'], 'cyclomatic_p90': cx['cyclomatic_p90'],
        'cognitive_p50': cx['cognitive_p50'], 'cognitive_p90': cx['cognitive_p90'],
        'over_ceiling': cx['over_ceiling'], 'max': cx['max'], 'top': cx['top'],
    },
    'sizes': {'file_p50': sizes['file_p50'], 'file_p90': sizes['file_p90'],
              'file_max': sizes['file_max'], 'over_800': sizes['over_800'],
              'top': sizes['top']},
    'graph': graph,
    'dead': {'unused_deps': dead['unused_deps_by_crate'],
             'zero_ref_candidates': dead['zero_ref_candidates'],
             'top': dead['top'][:10]},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': churn['pct'], 'files_churned': churn['files_churned']},
    'hotspots': churn['hotspots'],
    'mutation': mutation,
    'skips': [
        {'metric': 'complexity-web', 'reason': 'tool-missing'},
        {'metric': 'dead-code-web', 'reason': 'tool-missing'},
        {'metric': 'mutation-web', 'reason': 'tool-missing'},
        {'metric': 'mutation-conductor-core', 'reason': 'declined'},
        {'metric': 'mutation-conductor-cli', 'reason': 'declined'},
        {'metric': 'mutation-conductor-tauri', 'reason': 'declined'},
        {'metric': 'mutation-conductor-report', 'reason': 'declined'},
        {'metric': 'mutation-conductor-timeline', 'reason': 'declined'},
    ],
    # --- beyond the pinned schema, at operator direction (trend run #2) ---
    'recipes': {
        'population_note': ('the baseline record (8cba57d) carries no population/command '
                            'fields; its totals and size PERCENTILES could not be '
                            'reproduced (recorded 16543 loc / 118 files; the Rust-only '
                            'population at that commit is 15651 / 104, and no language '
                            'combination yields 16543/118). Its duplication, complexity '
                            'and dead-code recipes WERE recovered and reproduce exactly.'),
        'sizes': 'tokei over crates/; Rust *.rs only; nearest-rank p50/p90 on per-file code lines',
        'duplication': 'jscpd crates --format rust (Rust-only; reproduces the baseline 79 clones)',
        'complexity': ('rust-code-analysis kind==function spaces under crates/; ceiling '
                       'cognitive > 15; top sorted by COGNITIVE desc, rows '
                       '[fn, file, cognitive, cyclomatic]; max = max cognitive '
                       '(reproduces the baseline exactly)'),
        'dead': ('WORKSPACE-WIDE, one query over all symbols, never per-crate. '
                 'raw zero-ref pub symbols -> exclude `tests` path SEGMENT in the SYMBOL '
                 'path UNION the FILE path -> subtract ENTRY POINTS only (main(). / /bin/) '
                 '-> residual = reportable candidates. The other FP families '
                 '(trait-impl-dispatch, derive-attr-invoked, runtime-invoked, '
                 'test-only-helper) are NAMED, not subtracted. Validated: reproduces the '
                 'baseline 31 exactly (HEAD chain 745 -> 36 -> 33).'),
        'churn': ('git log --numstat baseline..HEAD over crates/ *.rs|*.ts|*.tsx; adds in a '
                  "file's 2nd..nth touching commit count as churn"),
        'hotspots': 'commits x per-file max cognitive over the FULL rca output',
    },
    'commands': {
        'sizes': 'tokei --output json crates/',
        'duplication': 'jscpd crates --format rust --reporters json --output {run_dir}/jscpd_rs --silent',
        'complexity': 'rust-code-analysis-cli --metrics -O json -o {run_dir}/_rca_head -p crates',
        'dead_deps': 'cargo machete',
        'dead_symbols': ('python scripts/code-graph.py query {run_dir} code-audit "SELECT '
                         's.symbol, s.file FROM symbol s LEFT JOIN refs r ON r.callee = '
                         's.symbol WHERE r.callee IS NULL;" rust'),
        'coverage': 'cargo llvm-cov nextest --workspace --lcov --output-path {run_dir}/_lcov.info',
        'churn': ("git log --numstat --format='commit %H' 8cba57d..HEAD -- 'crates/**/*.rs' "
                  "'crates/**/*.ts' 'crates/**/*.tsx'"),
        'mutation': ('cargo mutants -p {unit} --test-tool=nextest --jobs 2 --output '
                     '{run_dir}/mutants-{unit}'),
    },
    'carried_follow_ups': [
        {'id': 'mutation-timeouts', 'since': 'boundary #1 (8cba57d)', 'owner': None,
         'note': '2 pre-existing cargo-mutants timeouts; unchanged, no owner assigned'},
        {'id': 'command-field-omission', 'since': 'boundary #1 (8cba57d)', 'owner': None,
         'note': ("the baseline recorded the DEFAULT mutants form while the run needed "
                  "--test-tool=nextest, so replaying the recorded triple aborts. This "
                  "record adds a `commands` field carrying the firing form.")},
    ],
    'accepted_deliberate': {
        'class': 'declares survivors',
        'count': 6,
        'ratified': '2026-08-21',
        'rule': ('.claude/rules/testing.md:19 — every named survivor ends killed OR '
                 'classified accepted-deliberate against a cited rule; the score is a '
                 'consequence, never the acceptance'),
        'note': ('env is read at the caller and passed in as a typed value, so killing '
                 'these would require unsafe env writes in a shared-process test module'),
    },
    'known_residue': {
        'path': '.andromeda/runs/_wrap_tmp',
        'tracked_files': 7,
        'introduced': '01c6dac (2026-08-15)',
        'note': 'committed residue, not this run\'s damage; surfaced, deliberately not cleaned',
    },
}

json.dump(rec, open(os.path.join(RD, 'record.json'), 'w', encoding='utf-8'),
          ensure_ascii=False, indent=1)
print('record.json written; top-level keys:', len(rec))
print(json.dumps({k: rec[k] for k in ('ts', 'epoch', 'mode', 'sha', 'baseline_sha',
                                      'span', 'totals', 'coverage', 'churn')}, indent=1))
