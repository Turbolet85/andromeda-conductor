"""A4 graph + A5 dead-symbol collectors — pinned per the Epoch-4 record's `recipes`.

Dead-symbol recipe (verbatim from the ledger, reused so `dead-growth` fires on real
growth and not on filter drift):
  WORKSPACE-WIDE, one query over all symbols, never per-crate.
  raw zero-ref pub symbols
    -> exclude `tests` path SEGMENT in the SYMBOL path UNION the FILE path
    -> subtract ENTRY POINTS only (main(). / /bin/)
    -> residual = reportable candidates.
  The other FP families (trait-impl-dispatch, derive-attr-invoked, runtime-invoked,
  test-only-helper) are NAMED, not subtracted.
"""
import json, subprocess, os, sys, re

RD = '.andromeda/runs/2026-09-02T15-49-17-code-audit'


def q(sql):
    r = subprocess.run(
        [sys.executable, 'scripts/code-graph.py', 'query', RD, 'code-audit', sql, 'rust'],
        capture_output=True, text=True, encoding='utf-8', errors='replace')
    out = r.stdout.strip()
    i = out.find('[')
    if i < 0:
        raise SystemExit(f'no JSON in output:\n{out[:800]}\n{r.stderr[:800]}')
    return json.loads(out[i:])


CYCLE_SQL = ("WITH RECURSIVE walk(start, cur, path, depth) AS ("
             "SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges "
             "UNION ALL SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1 "
             "FROM crate_edges e JOIN walk w ON e.from_crate = w.cur "
             "WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges) "
             "AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start)) "
             "SELECT DISTINCT start, path FROM walk WHERE cur = start;")

cycles = q(CYCLE_SQL)
fan_in = q("SELECT callee, count(DISTINCT caller) AS n FROM calls_m "
           "GROUP BY callee ORDER BY n DESC LIMIT 20;")
fan_out = q("SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges "
            "GROUP BY from_crate ORDER BY n DESC;")
edges = q("SELECT count(*) AS n FROM crate_edges;")

SYM_SHORT = re.compile(r'^rust-analyzer\s+cargo\s+(\S+)\s+\S+\s+(.*)$')


def short_sym(s):
    """Baseline's pinned fan-in symbol form: `{crate} {path}`.

    The Epoch-4 record stores the SHORTENED form; comparing raw SCIP strings
    against it makes every row read as a top-N entrant (measured: all 20 did).
    """
    m = SYM_SHORT.match(s or '')
    return f'{m.group(1)} {m.group(2)}' if m else s


graph = {
    'population': 'plane=rust tree.db; views symbol/refs/calls_m/crate_edges',
    'symbol_form': 'fan_in symbols shortened to `{crate} {path}` (baseline-pinned)',
    'cycles': len(cycles),
    'cycle_paths': [c.get('path') for c in cycles],
    'fan_in_top': [[short_sym(r['callee']), r['n']] for r in fan_in],
    'fan_out': [[r['from_crate'], r['n']] for r in fan_out],
    'cross_unit_edges': edges[0]['n'],
}
json.dump(graph, open(os.path.join(RD, 'c-graph.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

# ---- A5 dead symbols (pinned chain) ----
raw = q("SELECT s.symbol, s.file FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol "
        "WHERE r.callee IS NULL;")
n_raw = len(raw)


SCIP_PREFIX = re.compile(r'^rust-analyzer\s+cargo\s+\S+\s+\S+\s+')


def has_tests_segment(p, is_symbol=False):
    """`tests` as a PATH SEGMENT anywhere in the path.

    For a SYMBOL the SCIP prefix (`rust-analyzer cargo {crate} {version} `) must be
    stripped first, or it fuses into the first segment and a leading `tests/` — the
    inline `#[cfg(test)]` marker — is missed entirely (measured: 21 such symbols in
    conductor-run/src/lib.rs survived the naive check and inflated the candidate count).
    """
    p = (p or '').replace('\\', '/')
    if is_symbol:
        p = SCIP_PREFIX.sub('', p)
    return any(seg == 'tests' for seg in p.split('/'))


step1 = [r for r in raw
         if not (has_tests_segment(r.get('symbol'), True)
                 or has_tests_segment(r.get('file')))]
n_step1 = len(step1)

ENTRY = re.compile(r'main\(\)\.|/bin/')
step2 = [r for r in step1 if not ENTRY.search(r.get('symbol') or '')]
n_step2 = len(step2)

dead = {
    'population': 'workspace-wide zero-ref pub symbols, plane=rust',
    'chain': {'raw': n_raw, 'after_tests_segment_filter': n_step1,
              'after_entry_points': n_step2},
    'zero_ref_candidates': n_step2,
    'top': [[r['symbol'], r['file']] for r in step2[:20]],
    'all_candidates': [[r['symbol'], r['file']] for r in step2],
    'false_positive_families_named_not_subtracted': [
        'trait-impl methods reached by dispatch',
        'derive/attr-invoked fns (serde defaults, #[from] variants)',
        'runtime-invoked surfaces (MCP tools, #[tauri::command] IPC)',
        'test-only helpers outside a tests/ path segment',
    ],
}
json.dump(dead, open(os.path.join(RD, 'c-dead-symbols.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

print('GRAPH  cycles=%d cross_unit_edges=%d fan_out=%s'
      % (graph['cycles'], graph['cross_unit_edges'], graph['fan_out']))
print('       fan_in_top5=%s' % [[s.split()[-1], n] for s, n in graph['fan_in_top'][:5]])
print('DEAD   chain %d -> %d -> %d  (candidates=%d)'
      % (n_raw, n_step1, n_step2, n_step2))
