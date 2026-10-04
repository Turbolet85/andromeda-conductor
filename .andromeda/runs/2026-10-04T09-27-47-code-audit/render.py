"""Phase 5: render proposals.md from record.json + c-diff.json + c-*.json; evidence-table row counts asserted against independent sources."""
import json, os, sys

RD = sys.argv[1]
J = lambda n: json.load(open(os.path.join(RD, n), encoding='utf-8'))
rec, dif, dup, sz, cx, cw, ch = J('record.json'), J('c-diff.json'), J('c-duplication.json'), J('c-sizes.json'), J('c-complexity.json'), J('c-complexity-web.json'), J('c-churn.json')
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')]
base = [r for r in recs[:-1] if r['sha'] == rec['baseline_sha']][-1]
b0 = [r for r in recs[:-1] if r['sha'] == base['baseline_sha']][-1]
m = rec['mutation']
out = []
w = out.append


def table(header, rows, n, key):
    assert len(rows) == n, (header, len(rows), n)
    assert len({key(r) for r in rows}) == len(rows), ('duplicate key', header)
    w('| ' + ' | '.join(header) + ' |'); w('|' + '---|' * len(header))
    for r in rows: w('| ' + ' | '.join(str(c) for c in r) + ' |')


w(f"# Code Audit — conductor · {rec['epoch']} · {rec['ts']}")
w(f"mode trend · HEAD {rec['sha'][:8]} · baseline {rec['baseline_sha'][:8]} ({base['epoch']}) · span {rec['span']}")
w('')
w(f"- **Overshoot (this run):** {rec['head_overshoot']['commits']} commits past the Epoch 5 boundary `{rec['head_overshoot']['boundary_sha'][:8]}` "
  "(two 0-pending wraps), **0 source files** in the delta.")
w(f"- **Overshoot (baseline):** the baseline record carried {base['head_overshoot']['commits']} commits / {len(base['head_overshoot']['files'])} source files "
  "past the Epoch 3 boundary (Epoch 4's first two chunks + one route adaptation). That delta is attributed to the baseline's record and not re-diffed here; "
  "this run's window metrics start at the baseline sha.")
w('- **Span 2:** the Epoch 4 boundary was not recorded (the founder ruled this run at HEAD over a worktree). Every single-epoch threshold is demoted to Informational with the span named. `new-cycle` and `monotonic` still run.')
w('- **Host change:** this is the FIRST ledger record measured on the Linux dev host (x86_64-unknown-linux-gnu). All 7 prior records were measured on the Windows host. '
  'rca, tokei, jscpd and the graph are host-neutral reads of the source. Coverage and mutation execute code: read their deltas with the host beside them.')
w('- **Trend-breaks:** duplication (jscpd 5.0.16 → 5.4.0) and coverage (cargo-llvm-cov 0.8.5 → 0.9.1). Their thresholds are suppressed and their deltas are labeled `trend-break — tool upgrade`.')
w('')
w('## Proposals')
w('')
# M1 file_max
mono = dif['monotonic']
fm, o8 = mono['sizes.file_max']['values'], mono['sizes.over_800']['values']
w(f"### M1 — monotonic · sizes.file_max — {fm[1]} → {fm[2]} (+{fm[2]-fm[1]})")
w(f"**Movement:** {fm[0]} ({b0['epoch']}) → {fm[1]} ({base['epoch']}) → {fm[2]} ({rec['epoch']}). It worsened at both of the last two diffs, with tokei 14.0.0 unchanged.")
harvest_dup = [t for t in dup['top_with_starts'] if t[0] == t[1] == 'conductor-run/tests/real_model_harvest.rs']
w(f"**Evidence:** the max file is `crates/conductor-run/tests/real_model_harvest.rs` at {sz['top'][0][1]} code lines (680 at the baseline; window numstat +1793/−74). "
  f"The next largest file is `{sz['top'][1][0]}` at {sz['top'][1][1]}. The same file holds {len(harvest_dup)} of the duplication top-10 rows as self-clones: "
  + ', '.join(f"{t[2]} L (lines {t[3]}/{t[4]})" for t in harvest_dup)
  + ". It is hotspot #4 (score 32.0 = 4 window commits × max cognitive 8). Four chunk pre-CI commits touched it in the window: 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin, 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir, 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix and 2026-10-02-captured-fingerprint-values-elided (the real-model series' grading and digest-pin work).")
w('**Suspected shape:** the real-model harvest test grows one grading arm per series and per assertion. Its repeated grading blocks show as self-clones, and the file roughly tripled in one epoch.')
w('**Proposal:** split `real_model_harvest.rs` by series or assertion family, and lift the repeated grading fixture (the self-clone family at lines 1916–2422) into the existing `tests/real_model_common` module.')
w('')
w(f"### M2 — monotonic · sizes.over_800 — {o8[1]} → {o8[2]} (+{o8[2]-o8[1]})")
w(f"**Movement:** {o8[0]} → {o8[1]} → {o8[2]} over the same three records. It worsened at both diffs, with tokei unchanged.")
over = [[p, c] for p, c in sz['top'] if c > 800]
base_top = dict(base['sizes']['top'])
rows = [[p, c, base_top.get(p, '—'), 'entrant' if base_top.get(p, 0) <= 800 else 'standing'] for p, c in over]
w('**Evidence:** every file over 800 code lines (n = `sizes.over_800`):')
table(['file', 'code lines', 'baseline', ''], rows, rec['sizes']['over_800'], lambda r: r[0])
w('**Suspected shape:** both entrants are test harvest files. `delegated_timing_harvest.rs` grew +558/−36 in the window, across three chunks: 2026-09-29-hue-shift-budget-graded-hard, 2026-10-02-p-075-assert-round-against-pulse and 2026-10-03-p-075-re-round-on-incident-events. The two standing files are source files and moved by a handful of lines.')
w('**Proposal:** structure the two harvest files per scenario or assertion family, as for M1. The standing `scenario.rs` and `execute.rs` hold steady, so they need no direction this boundary.')
w('')
w('## Informational')
w('')
w('- **Runner portability — `conductor-core` is unmeasurable by cargo-mutants copy mode (new this epoch).** The first core invocation failed its unmutated baseline. '
  '`secret_scan_gate::the_workspace_holds_no_secret_shaped_string` panics when `git ls-files` exits 128 ("the gate has no subject"), because the temp copy carries no `.git`. '
  'The gate arrived on 2026-09-24 in this epoch (`2026-09-24-secret-scanning-ci-gate`), after the baseline\'s core run. Re-run with `--copy-vcs true`, the unit scored 140/140, so the score is measured, but the default form no longer works for this unit. '
  'Evidence: `evidence-core-nocopyvcs.json`, `_mutants-conductor-core-nocopyvcs.log`. A direction for the founder: either the gate tolerates a VCS-less tree by naming it a skip, or the project pins `--copy-vcs true` in its own mutation form (test-plan names the tool).')
dl = rec['duplication']; bd = base['duplication']
w(f"- **duplication (trend-break — tool upgrade, span 2):** pct {bd['pct']:.2f}% → {dl['pct']:.2f}% (+{dl['pct']-bd['pct']:.2f}pt, +{100*(dl['pct']/bd['pct']-1):.0f}% rel). "
  f"That would satisfy `duplication-up`, but it is suppressed by the jscpd 5.0.16 → 5.4.0 break and the span. Clones {bd['clones']} → {dl['clones']}, duplicated lines {bd['duplicated_lines']} → {dl['duplicated_lines']}; "
  f"population total_lines {bd['total_lines']} → {dl['total_lines']} (+{100*(dl['total_lines']/bd['total_lines']-1):.1f}%). "
  f"Split: src {bd['split']['src']['pairs']}/{bd['split']['src']['lines']} → {dl['split']['src']['pairs']}/{dl['split']['src']['lines']} · test {bd['split']['test']['pairs']}/{bd['split']['test']['lines']} → {dl['split']['test']['pairs']}/{dl['split']['test']['lines']} · mixed {bd['split']['mixed']['pairs']}/{bd['split']['mixed']['lines']} → {dl['split']['mixed']['pairs']}/{dl['split']['mixed']['lines']} (pairs/lines). "
  "The growth is almost entirely in tests (+25 pairs, +277 lines); src gained 1 pair and fell 9 lines. The top standing pair is `conductor-core/src/scenario.rs` self-clone, 17 L, standing since the Epoch 3 record. The tool-version share of the movement is unmeasured: jscpd 5.0.16 was not re-run.")
ent = dif['entrants']
w('- **Duplication top entrants (5):** ' + '; '.join(f"`{a}` ↔ `{b}` {n} L" for a, b, n in ent['duplication_top']) + '.')
w(f"- **coverage (trend-break — tool upgrade + host change):** line {base['coverage']['line']} → {rec['coverage']['line']} (lines found {base['coverage']['lines_found']} → {rec['coverage']['lines_found']}, hit {base['coverage']['lines_hit']} → {rec['coverage']['lines_hit']}); 1197/1197 tests passed. Branch is not reported (null, as before).")
w('- **Mutation — new survivors this epoch** (excluding units first scored here and line-drifted standing survivors): '
  '`crates/conductor-emit/src/identity.rs:49:15` `replace ^= with |= in xor_in_place` and `replace ^= with &= in xor_in_place` (the per-run span identity code, `2026-10-01-per-run-span-identity-in-the-real-model-harness`); '
  '`crates/conductor-run/src/canary.rs:203:14` `replace > with >= in emit_canary_storms`, `canary.rs:206:61` `replace * with +` and `replace * with /`. '
  'The baseline\'s `execute.rs:122/165` survivors stand at `:126/:169` (line drift), not gone.')
sc = dif['deltas']['mutation.scores']
w('- **Mutation scores (span 2, so `mutation-drop` is informational; no unit moved ≥ 10pt):** ' + ' · '.join(f"{u} {a if a is not None else '—'} → {b}" for u, (a, b) in sorted(sc.items())) +
  '. faults, report and verify were not in the baseline scope: this is their first score. core and emit are shard 1/4, as at the baseline.')
w('- **Standing timeouts, unowned:** conductor-emit holds 11 timeouts, all in `exception.rs` `skip_absolute_path` / `skip_line_number_suffix` (lines 321–348). The baseline also had 11. They are invisible to the score formula and carry across boundaries.')
w('- **Pre-schema baseline mutation:** the baseline record has no `host` and no `not_measured`, so its host-excluded share is UNKNOWN, never 0. It was NOT recomputed: the cover evaluates against the host that built, the baseline was measured on Windows, and a faithful recompute needs a Windows host at `e799b9e0`. No `corrections[]` fill is made, and the score comparisons above carry this caveat.')
w(f"- **Hotspot entrants ({len(ent['hotspots'])}):** " + '; '.join(f"`{p}` {s}" for p, s in ent['hotspots']) + '.')
w(f"- **Complexity top entrant:** `analyse` in `conductor-core/tests/workflow_env_gate.rs` (cognitive 26), the workflow env-context gate (`2026-09-24-secret-scanning-ci-gate`).")
w(f"- **Sizes top entrant:** `{ent['sizes_top'][0][0]}` ({ent['sizes_top'][0][1]}).")
w('- **Dead-code top rotation:** ' + '; '.join(f"`{s}`" for s, _ in ent['dead_top']) + ' entered the top-20. The candidate count is unchanged (33 → 33, chain 1129 → 36 → 33).')
w(f"- **churn (span 2):** {base['churn']['pct']}% → {ch['pct']}% ({ch['churned_adds']}/{ch['total_adds']} adds; {ch['files_churned']} of {ch['files_touched']} files touched more than once across {ch['commits_in_window']} commits). The most-touched files: " +
  '; '.join(f"`{p}` ×{c}" for p, c in ch['top_touch_counts'][:5]) + '.')
w(f"- **Web dead code:** knip {base['dead_web']['total']} → {rec['dead_web']['total']} findings (exit 0, clean).")
w(f"- **Web complexity (first collection, lizard 1.24.0, cyclomatic only):** {cw['functions']} functions, p50 {cw['cyclomatic_p50']} / p90 {cw['cyclomatic_p90']}; 1 over 15: `App` in `ui/src/App.tsx` (24). There is no baseline, so this starts a series.")
w('')
w('## Below threshold — no action')
w('')
w(f"- complexity.over_ceiling {base['complexity']['over_ceiling']} → {rec['complexity']['over_ceiling']} (+2; `complexity-creep` needs +3 and +25%). The new members are `analyse` (26) and `print_pulse_witnesses` (14 → 18, `conductor-run/tests/real_model_live.rs`). p50/p90 are unchanged (cyc 1/4, cog 0/1); functions {base['complexity']['functions']} → {rec['complexity']['functions']}; max `serve_stub` 35 → 36.")
w(f"- dead.zero_ref_candidates 33 → 33. Unused deps: none (cargo-machete).")
w(f"- graph: cycles 0 → 0, cross-unit edges 16 → 16, fan-out unchanged. Fan-in top: `ReportState#` 110 → 122, `Verdict#` 92 → 101, `crate/` 154 → 158.")
w(f"- sizes p50 {base['sizes']['file_p50']} → {rec['sizes']['file_p50']}, p90 {base['sizes']['file_p90']} → {rec['sizes']['file_p90']}. The populations (totals.loc {base['totals']['loc']} → {rec['totals']['loc']}, files {base['totals']['files']} → {rec['totals']['files']}) are never movements.")
w('- monotonic, not fired: duplication.pct (trend-break at the second diff), coverage.line (trend-break; not worse), complexity.over_ceiling (flat at the first diff), dead.zero_ref_candidates (flat).')
w('- count-under-ratio: not applicable. clones and duplicated_lines rose with pct rising too, not held or fallen.')
w('')
w('## Skips')
w('')
for s in rec['skips']:
    w(f"- {s['metric']} — {s['reason']}" + (f" ({s['note']})" if s.get('note') else ''))
open(os.path.join(RD, 'proposals.md'), 'w', encoding='utf-8').write('\n'.join(out) + '\n')
print('rendered', len(out), 'lines')
