"""Phase 4 — diff current vs baseline and classify against the threshold table.

Two baselines are carried per metric where they differ:
  recorded    — the value stored in the ledger record at 8cba57d
  recomputed  — the same metric re-measured at 8cba57d under THIS run's declared
                population (from the extracted baseline tree)
A metric whose recipe was recovered and verified uses `recomputed` as the judging
basis; where the two agree, they are the same number.
"""
import json, os

RD = '.andromeda/runs/2026-08-22T13-31-54-code-audit'


def j(p):
    return json.load(open(os.path.join(RD, p), encoding='utf-8'))


cur_sz, cur_cx, cur_dup = j('c-sizes.json'), j('c-complexity.json'), j('c-duplication.json')
cur_dead, cur_cov, cur_ch = j('c-dead.json'), j('c-coverage.json'), j('c-churn.json')
cur_g = j('c-graph.json')
b_sz = j('_sizes_baseline_recomputed.json')
b_cx = j('_complexity_baseline_recomputed.json')
b_dup = j('_duplication_baseline_recomputed.json')

REC = {  # the ledger record at 8cba57d, verbatim
    'loc': 16543, 'files': 118, 'dup_pct': 3.68, 'dup_clones': 79,
    'cog_p50': 0.0, 'cog_p90': 1.0, 'cyc_p50': 1.0, 'cyc_p90': 4.0, 'over_ceiling': 6,
    'file_p50': 88, 'file_p90': 347, 'file_max': 898, 'over_800': 2,
    'cycles': 0, 'cross_unit_edges': 16, 'zero_ref': 31, 'cov_line': 92.26,
    'mut': {'conductor-run': 58.82, 'conductor-verify': 72.97},
}

rows = []


def add(metric, recorded, recomputed, current, basis, check, fires, note=''):
    rows.append({'metric': metric, 'recorded_baseline': recorded,
                 'recomputed_baseline': recomputed, 'current': current,
                 'judging_basis': basis, 'check': check, 'fires': fires, 'note': note})


# --- graph ---
add('graph.cycles', REC['cycles'], REC['cycles'], cur_g['cycles'], 'identical',
    'new-cycle', cur_g['cycles'] > REC['cycles'], 'zero at both boundaries')
add('graph.cross_unit_edges', REC['cross_unit_edges'], REC['cross_unit_edges'],
    cur_g['cross_unit_edges'], 'identical', '—', False, 'unchanged; fan-out set identical')

# --- duplication (recipe recovered: jscpd --format rust; clones reproduce exactly) ---
dup_delta_pt = round(cur_dup['pct'] - b_dup['pct'], 2)
add('duplication.pct', REC['dup_pct'], b_dup['pct'], cur_dup['pct'], 'recomputed',
    'duplication-up', (dup_delta_pt >= 0.5 and cur_dup['pct'] >= b_dup['pct'] * 1.15),
    f'{dup_delta_pt:+} pt — DOWN')
add('duplication.clones', REC['dup_clones'], b_dup['clones'], cur_dup['clones'],
    'recomputed (exact match on clones)', '—', False,
    'both_test 35->40, both_src 37->36, mixed 7->8 — growth is entirely test-side')

# --- complexity (recipe reproduces the record exactly) ---
oc = cur_cx['over_ceiling']
add('complexity.over_ceiling', REC['over_ceiling'], b_cx['over_ceiling'], oc, 'exact',
    'complexity-creep', (oc >= b_cx['over_ceiling'] + 3 and oc >= b_cx['over_ceiling'] * 1.25),
    'flat at 6')
for k, rk in (('cyclomatic_p50', 'cyc_p50'), ('cyclomatic_p90', 'cyc_p90'),
              ('cognitive_p50', 'cog_p50'), ('cognitive_p90', 'cog_p90')):
    add('complexity.' + k, REC[rk], b_cx[k], cur_cx[k], 'exact', '—',
        cur_cx[k] != b_cx[k], 'flat' if cur_cx[k] == b_cx[k] else 'moved')
add('complexity.max', f"{REC['over_ceiling']}|shape_is_realizable 31",
    f"{b_cx['max']['fn']} {b_cx['max']['val']}",
    f"{cur_cx['max']['fn']} {cur_cx['max']['val']}", 'exact', 'top-N entrants', True,
    'serve_stub (test stub) cognitive 27->34 takes the max slot from shape_is_realizable 31')

# --- sizes (top/max/over_800 reproduce; percentiles do NOT — population differs) ---
add('sizes.file_max', REC['file_max'], b_sz['file_max'], cur_sz['file_max'], 'exact',
    '—', False, 'conductor-run/src/lib.rs 898 -> 1012')
add('sizes.over_800', REC['over_800'], b_sz['over_800'], cur_sz['over_800'], 'exact',
    '—', False, 'flat at 2, but BOTH members now exceed 1000')
add('sizes.file_p50', REC['file_p50'], b_sz['file_p50'], cur_sz['file_p50'],
    'recomputed (recorded population unrecoverable)', '—', False, '101 -> 104')
add('sizes.file_p90', REC['file_p90'], b_sz['file_p90'], cur_sz['file_p90'],
    'recomputed (recorded population unrecoverable)', '—', False, '359 -> 360')
add('totals.loc', REC['loc'], b_sz['loc'], cur_sz['loc'],
    'recomputed (recorded population unrecoverable)', '—', False, '15651 -> 17518 (+1867)')
add('totals.files', REC['files'], b_sz['files'], cur_sz['files'],
    'recomputed (recorded population unrecoverable)', '—', False,
    '104 -> 111 (+7, all test files; git confirms 7 adds / 0 deletes)')

# --- dead (recipe recovered and validated to 31 exactly) ---
zr = cur_dead['zero_ref_candidates']
add('dead.zero_ref_candidates', REC['zero_ref'], 31, zr, 'recomputed == recorded (validated)',
    'dead-growth', zr >= 31 + 5,
    '+2: both are PauseResolver::kind() trait impls (a named FP family), not dead code')
add('dead.unused_deps', 2, 2, len([d for c in cur_dead['unused_deps_by_crate'].values()
                                   for d in c]), 'identical', '—', False,
    'conductor-cli:tracing resolved; conductor-emit:conductor-core remains')

# --- coverage ---
add('coverage.line', REC['cov_line'], REC['cov_line'], cur_cov['line'], 'identical',
    'coverage-drop', cur_cov['line'] <= REC['cov_line'] - 2,
    f"{round(cur_cov['line'] - REC['cov_line'], 2):+} pt")

# --- churn / hotspots (informational: ledger held 1 record when this ran) ---
add('churn.pct', None, None, cur_ch['pct'], 'no-baseline', 'churn (<2 records)', False,
    'informational by rule')
add('hotspots', '[] (no-baseline)', '[] (no-baseline)',
    cur_ch['hotspots'][0] if cur_ch['hotspots'] else None, 'no-baseline',
    'top-N entrants', True, 'first hotspot table; all entries are entrants')

fires = [r for r in rows if r['fires'] and r['check'] not in ('—',)]
json.dump({'rows': rows, 'firing': fires},
          open(os.path.join(RD, 'c-judgment.json'), 'w', encoding='utf-8'), indent=1)

print(f"{'metric':34} {'rec':>12} {'recomp':>12} {'cur':>12}  check")
for r in rows:
    f = 'FIRES' if (r['fires'] and r['check'] != '—') else ''
    print(f"{r['metric']:34} {str(r['recorded_baseline'])[:12]:>12} "
          f"{str(r['recomputed_baseline'])[:12]:>12} {str(r['current'])[:12]:>12}  "
          f"{r['check']:18} {f}")
print()
print('THRESHOLD PROPOSALS (excluding informational checks):',
      [r['metric'] for r in rows if r['fires'] and r['check'] not in ('—', 'top-N entrants',
                                                                     'churn (<2 records)')])
