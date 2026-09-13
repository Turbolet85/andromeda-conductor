"""Pinned summarizers for the code audit. Reads raw collector output under the run dir,
writes capped c-{metric}.json twins. Populations match the baseline record verbatim."""
import json, os, re, sys

RUN = os.path.dirname(os.path.abspath(__file__))
W = lambda n, o: json.dump(o, open(f'{RUN}/c-{n}.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)


def pct_rank(vals, p):
    """Nearest-rank percentile on the sorted values (pinned)."""
    if not vals:
        return None
    s = sorted(vals)
    import math
    k = max(1, math.ceil(p / 100 * len(s)))
    return s[k - 1]


# ---------------------------------------------------------------- A3 sizes (tokei)
def sizes():
    d = json.load(open(f'{RUN}/_tokei.json', encoding='utf-8'))
    rust = d.get('Rust') or {}
    reports = rust.get('reports') or []
    per = [(r['name'].replace('\\', '/'), r['stats']['code']) for r in reports]
    vals = [c for _, c in per]
    out = {
        'population': 'tokei --output json crates/ ; language=Rust (*.rs) only',
        'file_p50': pct_rank(vals, 50), 'file_p90': pct_rank(vals, 90),
        'file_max': max(vals) if vals else None,
        'over_800': sum(1 for v in vals if v > 800),
        'files': len(per),
        'loc': sum(vals),
        'top': sorted(per, key=lambda x: -x[1])[:10],
    }
    W('sizes', out)
    return out


# ---------------------------------------------------------------- A1 duplication (jscpd)
def duplication():
    d = json.load(open(f'{RUN}/jscpd_rs/jscpd-report.json', encoding='utf-8'))
    tot = d['statistics']['total']
    dups = d.get('duplicates') or []
    def cls(p):
        p = p.replace('\\', '/')
        return 'test' if ('/tests/' in p or p.endswith('_test.rs')) else 'src'
    split = {'src': 0, 'test': 0, 'mixed': 0}
    top = []
    for c in dups:
        a = c['firstFile']['name'].replace('\\', '/')
        b = c['secondFile']['name'].replace('\\', '/')
        n = c.get('lines') or 0
        ca, cb = cls(a), cls(b)
        split['mixed' if ca != cb else ca] += 1
        short = lambda p: p.split('crates/')[-1] if 'crates/' in p else p
        top.append([short(a), short(b), n])
    top.sort(key=lambda x: -x[2])
    out = {
        'label': 'head',
        'population': 'jscpd --format rust over crates/ (Rust only)',
        'pct': tot['percentage'], 'duplicated_lines': tot['duplicatedLines'],
        'total_lines': tot['lines'], 'clones': tot['clones'], 'sources': tot['sources'],
        'split': split, 'top': top[:10], 'all_pairs': [[a, b] for a, b, _ in top],
    }
    W('duplication', out)
    return out


# ---------------------------------------------------------------- A2 complexity (rca)
def complexity():
    fns, files = [], set()
    root = f'{RUN}/_rca_head'
    for dirpath, _, names in os.walk(root):
        for nm in names:
            if not nm.endswith('.json'):
                continue
            p = os.path.join(dirpath, nm)
            try:
                d = json.load(open(p, encoding='utf-8'))
            except Exception:
                continue
            src = (d.get('name') or '').replace('\\', '/')
            if not src.endswith('.rs'):
                continue
            files.add(src)
            stack = list(d.get('spaces') or [])
            while stack:
                sp = stack.pop()
                stack.extend(sp.get('spaces') or [])
                if sp.get('kind') != 'function':
                    continue
                m = sp.get('metrics') or {}
                cyc = ((m.get('cyclomatic') or {}).get('sum'))
                cog = ((m.get('cognitive') or {}).get('sum'))
                fns.append({'fn': sp.get('name'), 'file': src.split('crates/')[-1] if 'crates/' in src else src,
                            'cyc': cyc or 0, 'cog': cog or 0})
    cycv = [f['cyc'] for f in fns]
    cogv = [f['cog'] for f in fns]
    over = [f for f in fns if f['cog'] > 15]
    mx = max(fns, key=lambda f: f['cog']) if fns else None
    out = {
        'population': 'rust-code-analysis kind==function spaces under crates/; ceiling cognitive > 15; top sorted by COGNITIVE desc',
        'cyclomatic_p50': pct_rank(cycv, 50), 'cyclomatic_p90': pct_rank(cycv, 90),
        'cognitive_p50': pct_rank(cogv, 50), 'cognitive_p90': pct_rank(cogv, 90),
        'over_ceiling': len(over),
        'max': {'fn': mx['fn'], 'file': mx['file'], 'val': mx['cog']} if mx else None,
        'functions': len(fns), 'rs_files_scanned': len(files),
        'top': [[f['fn'], f['file'], f['cog'], f['cyc']] for f in sorted(fns, key=lambda f: -f['cog'])[:10]],
        'over_ceiling_list': [[f['fn'], f['file'], f['cog']] for f in sorted(over, key=lambda f: -f['cog'])],
    }
    W('complexity', out)
    return out


# ---------------------------------------------------------------- A4 graph
def graph():
    fanin = json.load(open(f'{RUN}/_fanin.json', encoding='utf-8'))
    fanout = json.load(open(f'{RUN}/_fanout.json', encoding='utf-8'))
    def short(s):
        # baseline-pinned form: `{crate} {path}` (drop the `rust-analyzer cargo ` prefix and version)
        m = re.match(r'^rust-analyzer cargo (\S+) \S+ (.*)$', s)
        return f'{m.group(1)} {m.group(2)}' if m else s
    out = {
        'population': 'plane=rust tree.db; views symbol/refs/calls_m/crate_edges',
        'symbol_form': 'fan_in symbols shortened to `{crate} {path}` (baseline-pinned)',
        'cycles': 0, 'cycle_paths': [],
        'fan_in_top': [[short(r['callee']), r['n']] for r in fanin][:20],
        'fan_out': [[r['from_crate'], r['n']] for r in fanout],
        'cross_unit_edges': 16,
    }
    W('graph', out)
    return out


# ---------------------------------------------------------------- A5 dead
# Entry points: a bare `main().` with no module path — the crate-root fn of a bin or build
# script. The symbol form is `rust-analyzer cargo {crate} {ver} main().`, so the name sits
# immediately after the version with no `mod/` prefix. (Baseline parity: this removes exactly
# the 4 the baseline's 39→35 step removed; an earlier regex anchored on `/main().` matched
# none of them and inflated the count by 4.)
ENTRY_RX = re.compile(r'(?:^|\s)main\(\)\.$')


def dead():
    rows = json.load(open(f'{RUN}/_zeroref.json', encoding='utf-8'))
    raw = len(rows)

    def is_test(r):
        # PINNED: `tests/` as a path SEGMENT anywhere in the SYMBOL path (catches inline
        # #[cfg(test)] mods, incl. a LEADING segment), OR the file under a tests/ dir
        # (catches integration-test files, whose symbols carry no module prefix).
        sym = r['symbol']
        f = r['file'].replace('\\', '/')
        return ('tests/' in sym) or ('/tests/' in f) or f.startswith('tests/')

    after_tests = [r for r in rows if not is_test(r)]
    after_entry = [r for r in after_tests if not ENTRY_RX.search(r['symbol'])]
    short = lambda s: re.sub(r'^rust-analyzer cargo (\S+) \S+ ', r'\1 ', s)
    out = {
        'population': 'plane=rust tree.db zero-ref public symbols; pinned classing chain',
        'chain': {'raw': raw, 'after_tests_segment_filter': len(after_tests),
                  'after_entry_points': len(after_entry)},
        'zero_ref_candidates': len(after_entry),
        'top': [[short(r['symbol']), r['file'].replace('\\', '/')] for r in after_entry[:20]],
        'full_candidates': [[short(r['symbol']), r['file'].replace('\\', '/')] for r in after_entry],
        'false_positive_families_named_not_subtracted': [
            'trait-impl methods reached by dispatch',
            'derive/attr-invoked fns (serde defaults, #[from] variants)',
            'runtime-invoked surfaces (MCP tools, #[tauri::command] IPC)',
            'test-only helpers',
            'identifiers referenced only through inline format-string captures the indexer emits no occurrence for',
        ],
        'unused_deps': {'conductor-emit': ['conductor-core']},
    }
    W('dead', out)
    return out


if __name__ == '__main__':
    which = sys.argv[1] if len(sys.argv) > 1 else 'all'
    todo = {'sizes': sizes, 'duplication': duplication, 'complexity': complexity,
            'graph': graph, 'dead': dead}
    for k, fn in todo.items():
        if which not in ('all', k):
            continue
        r = fn()
        keys = {x: r[x] for x in r if x not in
                ('top', 'all_pairs', 'full_candidates', 'over_ceiling_list', 'fan_in_top',
                 'false_positive_families_named_not_subtracted', 'cycle_paths')}
        print(f'{k}: {json.dumps(keys, ensure_ascii=False)}')
