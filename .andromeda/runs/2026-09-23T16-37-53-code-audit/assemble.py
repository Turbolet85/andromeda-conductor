"""Assemble this run's ledger record -> {run_dir}/record.json (audit-pass.md §Schema)."""
import json, os, subprocess, sys
R = sys.argv[1]
RR = R.replace('\\', '/')
L = lambda m: json.load(open(os.path.join(R, 'c-%s.json' % m), encoding='utf-8'))
sh = lambda *a: subprocess.run(list(a), capture_output=True, text=True, encoding='utf-8').stdout.strip()
HEAD = sh('git', 'rev-parse', 'HEAD')
BASE = '6861eb63439875520599180c5a726564e0f9884a'
BOUND = 'e75fcb9dc19f9fa3ae97965bc4ec1cef452f43c5'
ns = sh('git', 'diff', '--numstat', BOUND + '..HEAD', '--', 'crates/**/*.rs', 'crates/**/*.ts', 'crates/**/*.tsx')
ofiles = [[p, int(a), int(d)] for a, d, p in (l.split('\t') for l in ns.splitlines()) if a != '-']
sizes, dup, cx, dead, cov, gr, ch, hs = (L(m) for m in ('sizes', 'duplication', 'complexity', 'dead', 'coverage', 'graph', 'churn', 'hotspots'))
units = ['conductor-timeline', 'conductor-cli', 'conductor-run', 'conductor-core', 'conductor-emit', 'conductor-tauri']
mu = {u: L('mutation-' + u) for u in units}
for k in ('all_pairs',): dup.pop(k)
for k in ('per_file_max_cognitive', 'over_ceiling_list'): cx.pop(k)
dead_web = dead.pop('dead_web'); dead.pop('all_candidates')
ch.pop('commits_per_file')
totals = sizes.pop('totals')
mcmd = 'cargo mutants -p {unit} --test-tool=nextest --jobs 2 --output %s/mutants-{unit}' % RR
rec = {
 "ts": sh('date', '-u', '+%FT%TZ'), "epoch": "Epoch 3 — The a11y capability's terminal", "mode": "trend",
 "sha": HEAD, "baseline_sha": BASE, "span": 2, "ancestry_broken": False,
 "head_overshoot": {"boundary_sha": BOUND, "commits": int(sh('git', 'rev-list', '--count', BOUND + '..HEAD')), "files": ofiles,
   "note": "HEAD carries Epoch 4's two complete chunks (2026-09-18-real-model-leg-posture-and-grading-rule, 2026-09-22-interpretation-proven-live) plus one route-adaptation commit past the Epoch 3 boundary; operator-confirmed trend mode. Every metric below INCLUDES this delta; the next record attributes it here and never re-diffs it."},
 "tool_versions": {"jscpd": "5.0.16", "tokei": "14.0.0", "rust-code-analysis": "0.0.25", "cargo-machete": "0.9.2",
   "cargo-mutants": "27.1.0", "cargo-llvm-cov": "0.8.5", "cargo-nextest": "0.9.133",
   "code-graph": sh('git', 'rev-parse', 'HEAD:scripts/code-graph.py')[:8], "knip": "6.34.0", "rustc": "1.95.0"},
 "totals": totals, "duplication": dup, "complexity": cx, "sizes": sizes, "graph": gr, "dead": dead, "dead_web": dead_web,
 "coverage": cov, "churn": ch, "hotspots": hs['top'],
 "mutation": {"scoped_units": units,
   "unit_states": {u: mu[u]['unit_state'] for u in units},
   "scores": {u: mu[u]['score'] for u in units},
   "counts": {u: mu[u]['counts'] for u in units},
   "score_formula": "caught/(caught+missed)",
   "survivors": [s for u in units for s in mu[u]['survivors']],
   "timeouts": [t for u in units for t in mu[u]['timeouts']],
   "note": "conductor-run ran 16m against the 15m per-unit cap; it was COMPLETE by the tool's markers (end_time set, 123/123) when read, so it is scored, the overrun disclosed. conductor-core/emit ran --shard 1/4 (the pinned form); a shard of a changed mutant list is not the baseline's shard set."},
 "commands": {
   "sizes": "tokei --output json crates/",
   "duplication": "jscpd crates --format rust --reporters json --output %s/jscpd_rs --silent" % RR,
   "complexity": "rust-code-analysis-cli --metrics -O json -o %s/_rca_head -p crates   (summarizer filters to *.rs; the walk also visits the ui tree)" % RR,
   "dead_deps": "cargo machete",
   "dead_symbols": "python scripts/code-graph.py query %s code-audit \"SELECT s.symbol, s.file FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL;\" rust   (classing: tests/ exclusion = UNION of symbol-path AND file-path segments, then main() entry points)" % RR,
   "dead_web": "npx --no-install knip --reporter json   (cwd crates/conductor-tauri/ui; exit 1 = findings present, knip convention)",
   "graph": "python scripts/code-graph.py query %s code-audit \"<cycles | fan-in LIMIT 20 | fan-out | count(*) FROM crate_edges>\" rust" % RR,
   "coverage": "cargo llvm-cov nextest --workspace --lcov --output-path %s/_lcov.info" % RR,
   "churn": "git log --reverse --numstat --format='commit %%H' %s..HEAD -- 'crates/**/*.rs' 'crates/**/*.ts' 'crates/**/*.tsx'" % BASE[:8],
   "hotspots": "commits(churn window) x max cognitive per file over the FULL rca population (fallback 0 for non-Rust)",
   "mutation": mcmd,
   "mutation_conductor-core": mcmd.replace('{unit}', 'conductor-core') + ' --shard 1/4',
   "mutation_conductor-emit": mcmd.replace('{unit}', 'conductor-emit') + ' --shard 1/4',
   "head_overshoot": "git log --format=%%H -1 -S'2026-09-17-keyboard-and-focus-order-coverage-ownership · complete' -- .andromeda/master-route.md  |  git rev-list --count %s..HEAD  |  git diff --numstat %s..HEAD -- 'crates/**/*.rs' 'crates/**/*.ts' 'crates/**/*.tsx'" % (BOUND, BOUND)},
 "corrections": [],
 "skips": [
   {"metric": "mutation-web", "reason": "tool-missing", "note": "StrykerJS absent; recipe: npm i -D @stryker-mutator/core in crates/conductor-tauri/ui"},
   {"metric": "complexity-web", "reason": "declined", "note": "lizard not on PATH this run (python -m lizard was used as the probe form before); not collected, matching every prior boundary, to keep the Rust-only complexity series comparable"},
   {"metric": "mutation-conductor-core", "reason": "budget-exhausted", "note": "ran --shard 1/4 only (140/140 of the shard; 559 in the unit); the other 3 shards are untested this run"},
   {"metric": "mutation-conductor-emit", "reason": "budget-exhausted", "note": "ran --shard 1/4 only (113/113 of the shard; 452 in the unit); the other 3 shards are untested this run"}]}
json.dump(rec, open(os.path.join(R, 'record.json'), 'w', encoding='utf-8', newline=''), ensure_ascii=False, indent=1)
print('record ok', HEAD[:8], len(ofiles), 'overshoot files', rec['head_overshoot']['commits'], 'commits')
