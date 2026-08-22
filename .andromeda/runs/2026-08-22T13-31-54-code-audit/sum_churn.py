"""B1 churn + B2 hotspots (pinned).

B1: per file, adds in its 2nd..nth touching commit count as churn;
    churn% = churned adds / all adds. Binary '-' rows skipped; a rename
    line counts to the NEW path.
B2: score = commits x max cognitive in file (fallback max cyclomatic; fallback KLOC).
Usage: python sum_churn.py <numstat.txt> <c-complexity.json> <c-sizes.json> <out.json>
"""
import json, re, sys, collections

SRC = re.compile(r'\.(rs|ts|tsx)$')
RENAME_BRACE = re.compile(r'^(.*)\{(.*) => (.*)\}(.*)$')


def norm_rename(path):
    """git numstat rename forms: 'a/{b => c}/d' or 'old => new'."""
    m = RENAME_BRACE.match(path)
    if m:
        pre, _old, new, post = m.groups()
        return (pre + new + post).replace('//', '/')
    if ' => ' in path:
        return path.split(' => ')[-1]
    return path


def main():
    numstat, cxf, szf, out = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    commits = []          # list of {file: adds}
    cur = None
    for line in open(numstat, encoding='utf-8', errors='replace'):
        line = line.rstrip('\n')
        if line.startswith('commit '):
            cur = {}
            commits.append(cur)
            continue
        if not line.strip() or cur is None:
            continue
        parts = line.split('\t')
        if len(parts) != 3:
            continue
        adds, dels, path = parts
        if adds == '-' or dels == '-':      # binary
            continue
        path = norm_rename(path)
        if not SRC.search(path):
            continue
        cur[path] = cur.get(path, 0) + int(adds)

    # commits are newest-first from git log; reverse to chronological
    commits.reverse()
    seen = collections.Counter()
    total_adds = 0
    churned_adds = 0
    per_file_commits = collections.Counter()
    per_file_adds = collections.Counter()
    for c in commits:
        for path, adds in c.items():
            total_adds += adds
            per_file_adds[path] += adds
            per_file_commits[path] += 1
            if seen[path] >= 1:             # 2nd..nth touch
                churned_adds += adds
            seen[path] += 1

    cx = json.load(open(cxf, encoding='utf-8'))
    cog = collections.defaultdict(float)
    cyc = collections.defaultdict(float)
    for fn, f, c_cog, c_cyc in cx['top'] + cx['over_ceiling_list']:
        pass
    # build per-file maxima from the full function set is not in the summary;
    # use over_ceiling_list + top (the only per-function rows the summary keeps)
    for row in cx['top']:
        fn, f, c_cog, c_cyc = row
        cog[f] = max(cog[f], c_cog)
        cyc[f] = max(cyc[f], c_cyc)
    for row in cx['over_ceiling_list']:
        fn, f, c_cyc, c_cog = row
        cog[f] = max(cog[f], c_cog)
        cyc[f] = max(cyc[f], c_cyc)
    sz = {f: c for f, c in json.load(open(szf, encoding='utf-8'))['top']}

    hotspots = []
    for path, n in per_file_commits.items():
        weight = cog.get(path) or cyc.get(path) or (sz.get(path, 0) / 1000.0)
        basis = ('cognitive' if cog.get(path) else
                 'cyclomatic' if cyc.get(path) else 'kloc')
        hotspots.append([path, round(n * weight, 2), n, basis])
    hotspots.sort(key=lambda r: -r[1])

    res = {
        'commits_scanned': len(commits),
        'files_touched': len(per_file_commits),
        'total_adds': total_adds,
        'churned_adds': churned_adds,
        'pct': round(100.0 * churned_adds / total_adds, 2) if total_adds else None,
        'files_churned': sum(1 for _p, n in per_file_commits.items() if n > 1),
        'top_touched': [[p, n, per_file_adds[p]]
                        for p, n in per_file_commits.most_common(10)],
        'hotspots': [[r[0], r[1]] for r in hotspots[:10]],
        'hotspots_detail': hotspots[:10],
        'note': ('hotspot weight uses per-file max cognitive where the complexity '
                 'summary carries that file; otherwise cyclomatic, otherwise KLOC — '
                 'so files outside the complexity top/over-ceiling lists score on KLOC'),
    }
    json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
    print(json.dumps({k: v for k, v in res.items()
                      if k not in ('top_touched', 'hotspots', 'hotspots_detail', 'note')},
                     indent=1))
    print(' top touched [file, commits, adds]:')
    for r in res['top_touched']:
        print(f'   {r[1]:3} commits  {r[2]:5} adds  {r[0]}')
    print(' hotspots [file, score, commits, basis]:')
    for r in res['hotspots_detail']:
        print(f'   {r[1]:9}  ({r[2]} commits x {r[3]})  {r[0]}')


main()
