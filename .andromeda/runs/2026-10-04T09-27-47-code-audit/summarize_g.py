"""Graph + dead-symbol summarizers (pinned: audit-pass.md canonical SQL; collectors.md A4/A5 classing recipe)."""
import json, os, re, sys

RD = sys.argv[1]
L = lambda n: json.load(open(os.path.join(RD, f'_g_{n}.out'), encoding='utf-8'))
PFX = re.compile(r'^rust-analyzer cargo (\S+) \S+ ')
short = lambda s: PFX.sub(lambda m: m.group(1) + ' ', s)

cycles = L('cycles')
fanin = [[short(r['callee']), r['n']] for r in L('fanin')][:20]
fanout = [[r['from_crate'], r['n']] for r in L('fanout')]
edges = list(L('edges')[0].values())[0]
json.dump({'population': 'plane=rust tree.db; views symbol/refs/calls_m/crate_edges',
           'symbol_form': 'fan_in symbols shortened to `{crate} {path}` (baseline-pinned)',
           'cycles': len(cycles), 'cycle_paths': [r['path'] for r in cycles], 'fan_in_top': fanin,
           'fan_out': fanout, 'cross_unit_edges': edges},
          open(os.path.join(RD, 'c-graph.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

# dead: raw zero-ref -> tests/ segment UNION (symbol path after prefix strip, file path) -> entry points (main)
raw = L('dead')
seg = lambda p: p.startswith('tests/') or '/tests/' in p
after_tests = [r for r in raw if not seg(PFX.sub('', r['symbol'])) and not seg(r['file'])]
after_entry = [r for r in after_tests if not re.search(r'(^|/)main\(\)\.$', PFX.sub('', r['symbol']))]
fp = {'trait-impl (impl#[T][Trait]…)': 0, 'error variant (…Error#Variant#)': 0, 'const/static': 0, 'other': 0}
for r in after_entry:
    s = PFX.sub('', r['symbol'])
    if re.search(r'impl#\[[^\]]*\]\[', s): fp['trait-impl (impl#[T][Trait]…)'] += 1
    elif re.search(r'Error#\w+#$', s): fp['error variant (…Error#Variant#)'] += 1
    elif re.search(r'/[A-Z0-9_]+\.$', s): fp['const/static'] += 1
    else: fp['other'] += 1
json.dump({'population': 'plane=rust tree.db zero-ref public symbols; pinned classing chain',
           'chain': {'raw': len(raw), 'after_tests_segment_filter': len(after_tests), 'after_entry_points': len(after_entry)},
           'zero_ref_candidates': len(after_entry),
           'fp_family_tally_of_candidates': fp,
           'candidates_full': [[short(r['symbol']), r['file']] for r in after_entry],
           'top': [[short(r['symbol']), r['file']] for r in after_entry][:20]},
          open(os.path.join(RD, 'c-dead.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
print(len(cycles), edges, len(raw), len(after_tests), len(after_entry), fp)
