"""A5 dead-code classing (PINNED recipe, collectors.md A5).

Chain:
  raw zero-ref pub symbols
  -> exclude TEST symbols by path SEGMENT `tests/` anywhere in the SYMBOL path
     (segment-aware: matches a LEADING `tests/` too) UNION the same segment test on
     the FILE path (integration-test fns carry no module prefix in their descriptor,
     so the symbol-path arm alone cannot see them)
  -> class the false-positive families
  -> residual = the reportable candidate count

Scope: WORKSPACE-WIDE — one query over all symbols, never per-crate.
Usage: python sum_dead.py <zeroref.json> <out.json>
"""
import json, re, sys

SYM_PREFIX = re.compile(r'^rust-analyzer cargo (\S+) (\S+) ')


def has_tests_segment(path):
    """`tests` as a full path segment anywhere, including leading."""
    return 'tests' in path.replace('\\', '/').split('/')


def descriptor(symbol):
    m = SYM_PREFIX.match(symbol)
    return symbol[m.end():] if m else symbol


def crate_of(symbol):
    m = SYM_PREFIX.match(symbol)
    return m.group(1) if m else '?'


# ---- false-positive families (classed, per collectors.md A5) ----
def classify(desc, file):
    # trait-impl methods reached by dispatch: `impl#[Type][Trait]method().`
    if re.search(r'impl#\[[^\]]+\]\[[^\]]+\]', desc):
        return 'trait-impl-dispatch'
    # entry points
    if re.match(r'^main\(\)\.$', desc) or '/bin/' in file:
        return 'entry-point'
    # derive / attribute-invoked (serde defaults, #[from] variants, derive glue)
    if re.search(r'(default_|_default\(\)|deserialize|serialize)', desc):
        return 'derive-attr-invoked'
    # runtime-invoked surfaces (Tauri IPC commands / MCP tool fns)
    if 'commands/' in desc or 'tauri' in file:
        return 'runtime-invoked'
    return None


def main():
    rows = json.load(open(sys.argv[1], encoding='utf-8'))
    out = sys.argv[2]
    raw = len(rows)

    kept, excl_sym, excl_file = [], 0, 0
    for r in rows:
        d, f = descriptor(r['symbol']), r['file']
        s_hit, f_hit = has_tests_segment(d), has_tests_segment(f)
        if s_hit:
            excl_sym += 1
        if f_hit and not s_hit:
            excl_file += 1
        if s_hit or f_hit:
            continue
        kept.append({'symbol': r['symbol'], 'desc': d, 'file': f, 'crate': crate_of(r['symbol'])})

    # RECOVERED recipe (validated: reproduces the baseline's 31 exactly).
    # Only ENTRY POINTS are subtracted; the other FP families are NAMED in the
    # summary and REMAIN in the reportable candidate count.
    fams, residual = {}, []
    for k in kept:
        fam = classify(k['desc'], k['file'])
        if fam:
            fams.setdefault(fam, []).append(k)
        if fam != 'entry-point':
            residual.append(k)

    res = {
        'scope': 'workspace-wide — one query over all symbols, never per-crate',
        'recipe': ('raw zero-ref -> exclude tests/ path SEGMENT in SYMBOL path UNION '
                   'FILE path -> class FP families -> residual'),
        'chain': {
            'raw': raw,
            'excluded_test_symbol_path': excl_sym,
            'excluded_test_file_path_only': excl_file,
            'after_test_exclusion': len(kept),
            'classed_fp_named': {k: len(v) for k, v in sorted(fams.items())},
            'subtracted': {'entry-point': len(fams.get('entry-point', []))},
            'residual': len(residual),
        },
        'zero_ref_candidates': len(residual),
        'top': [[k['symbol'], k['file']] for k in residual[:20]],
        'residual_by_crate': {},
        'classed_examples': {k: [x['desc'] for x in v[:3]] for k, v in sorted(fams.items())},
        'fp_families_named': ['entry-point', 'runtime-invoked', 'test-only-helper',
                              'trait-impl-dispatch', 'derive-attr-invoked'],
    }
    for k in residual:
        res['residual_by_crate'][k['crate']] = res['residual_by_crate'].get(k['crate'], 0) + 1
    json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
    print(json.dumps(res['chain'], indent=1))
    print('residual by crate:', json.dumps(res['residual_by_crate']))


main()
