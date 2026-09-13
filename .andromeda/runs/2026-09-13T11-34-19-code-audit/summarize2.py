"""Summarizers for coverage (lcov), churn (numstat), hotspots, and web dead code (knip)."""
import json, os, re, collections

RUN = os.path.dirname(os.path.abspath(__file__))
W = lambda n, o: json.dump(o, open(f'{RUN}/c-{n}.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)


def coverage():
    """LCOV totals: LF/LH give lines, BRF/BRH branches. Population = whatever the run covered."""
    lf = lh = brf = brh = 0
    for line in open(f'{RUN}/_lcov.info', encoding='utf-8'):
        line = line.strip()
        if line.startswith('LF:'):
            lf += int(line[3:])
        elif line.startswith('LH:'):
            lh += int(line[3:])
        elif line.startswith('BRF:'):
            brf += int(line[4:])
        elif line.startswith('BRH:'):
            brh += int(line[4:])
    out = {
        'population': 'cargo llvm-cov nextest --workspace --lcov; totals summed over all records',
        'line': round(100 * lh / lf, 2) if lf else None,
        'branch': round(100 * brh / brf, 2) if brf else None,
        'lines_found': lf, 'lines_hit': lh, 'branches_found': brf,
    }
    W('coverage', out)
    return out


def churn():
    """B1 pinned: per file, adds in its 2nd..nth touching commits are churn.
    numstat parsing: skip binary '-' rows; a rename line counts to the NEW path."""
    per_file_commits = collections.defaultdict(list)  # path -> [adds per commit, in order]
    cur = None
    for raw in open(f'{RUN}/_churn.txt', encoding='utf-8'):
        line = raw.rstrip('\n')
        if line.startswith('commit '):
            cur = line[7:]
            continue
        if not line.strip():
            continue
        parts = line.split('\t')
        if len(parts) < 3:
            continue
        a, d, path = parts[0], parts[1], parts[2]
        if a == '-' or d == '-':
            continue  # binary
        m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', path)
        if m:  # brace-form rename -> new path
            path = (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
        elif ' => ' in path:
            path = path.split(' => ')[-1]
        per_file_commits[path.replace('\\', '/')].append(int(a))
    # git log is newest-first; reverse each file's list so index 0 is its first touch
    total_adds = churned_adds = 0
    churned_files = []
    commits_per_file = {}
    for p, adds in per_file_commits.items():
        adds = list(reversed(adds))
        commits_per_file[p] = len(adds)
        total_adds += sum(adds)
        if len(adds) > 1:
            c = sum(adds[1:])
            churned_adds += c
            if c:
                churned_files.append([p, len(adds), c])
    churned_files.sort(key=lambda x: -x[2])
    out = {
        'population': "git log --numstat 59d5b7c8..HEAD -- crates/**/*.rs crates/**/*.ts crates/**/*.tsx",
        'pct': round(100 * churned_adds / total_adds, 2) if total_adds else None,
        'files_churned': sum(1 for _, a in per_file_commits.items() if len(a) > 1),
        'files_touched': len(per_file_commits),
        'total_adds': total_adds, 'churned_adds': churned_adds,
        'top_churned': churned_files[:10],
    }
    W('churn', out)
    W('_commits_per_file', commits_per_file)
    return out


def hotspots():
    """B2 pinned: score = commits x max cognitive in file (fallback cyclomatic, then KLOC)."""
    commits = json.load(open(f'{RUN}/c-_commits_per_file.json', encoding='utf-8'))
    cx = json.load(open(f'{RUN}/c-complexity.json', encoding='utf-8'))
    # per-file max cognitive from the complexity top list is capped; recompute from over_ceiling
    # + top is insufficient, so use sizes as the documented fallback where absent.
    sz = {f: n for f, n in json.load(open(f'{RUN}/c-sizes.json', encoding='utf-8'))['top']}
    maxcog = {}
    for fn, f, cog, cyc in cx['top']:
        maxcog[f] = max(maxcog.get(f, 0), cog)
    for fn, f, cog in cx['over_ceiling_list']:
        maxcog[f] = max(maxcog.get(f, 0), cog)
    rows = []
    for p, n in commits.items():
        short = p.split('crates/')[-1] if 'crates/' in p else p
        basis = 'cognitive'
        val = maxcog.get(short)
        if val is None:
            val = max(1, sz.get(p, 0) // 1000) or 1
            basis = 'kloc-fallback'
        rows.append([short, n * val, n, val, basis])
    rows.sort(key=lambda r: -r[1])
    out = {'population': 'commits(B1 window) x max cognitive in file; fallback KLOC',
           'top': [[r[0], r[1]] for r in rows[:10]],
           'detail': rows[:10]}
    W('hotspots', out)
    return out


def knip_web():
    d = json.load(open(f'{RUN}/_knip.json', encoding='utf-8'))
    issues = d.get('issues') or []
    buckets = collections.Counter()
    detail = collections.defaultdict(list)
    for it in issues:
        f = it.get('file')
        for k, v in it.items():
            if k == 'file' or not isinstance(v, list) or not v:
                continue
            buckets[k] += len(v)
            for e in v:
                nm = e.get('name') if isinstance(e, dict) else str(e)
                detail[k].append(f'{f}:{nm}')
    out = {
        'population': 'npx knip --reporter json in crates/conductor-tauri/ui (no knip config present)',
        'tool': 'knip 6.34.0', 'exit': 1,
        'counts': dict(buckets), 'total': sum(buckets.values()),
        'files_with_issues': len(issues),
        'detail': {k: v[:20] for k, v in detail.items()},
        'CAVEAT': ('This epoch MEASURED knip at 20 findings / 20 false positives on its first run '
                   '(2026-09-07-dependency-polish): WebdriverIO discovers specs through wdio.conf.ts '
                   "and loads devDependencies through its own plugin resolution, neither of which is an "
                   'import edge knip can see. Counts are reported as CANDIDATES against an UNCONFIGURED '
                   'tool, never as dead code. No baseline exists (tool-missing at 59d5b7c8).'),
    }
    W('deadweb', out)
    return out


if __name__ == '__main__':
    for fn in (coverage, churn, hotspots, knip_web):
        r = fn()
        print(f'{fn.__name__}: ' + json.dumps(
            {k: v for k, v in r.items() if k not in ('top', 'detail', 'top_churned', 'CAVEAT')},
            ensure_ascii=False))
