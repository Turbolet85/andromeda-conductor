"""Tier B (pinned, collectors.md B1/B2): churn over baseline_sha..HEAD on source paths; hotspots = commits x max cognitive."""
import json, os, subprocess, sys

RD, BASE = sys.argv[1], sys.argv[2]
cmd = ['git', 'log', '--reverse', '--numstat', '--format=commit %H', f'{BASE}..HEAD', '--',
       'crates/**/*.rs', 'crates/**/*.ts', 'crates/**/*.tsx']
out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
seen, commits, adds_all, churned = {}, {}, 0, 0
n_commits = 0
for line in out.splitlines():
    if line.startswith('commit '): n_commits += 1; continue
    parts = line.split('\t')
    if len(parts) != 3 or parts[0] == '-': continue  # skip binary rows
    a, path = int(parts[0]), parts[2]
    if '=>' in path:  # rename: count to the new path
        if '{' in path:
            pre, rest = path.split('{', 1); mid, post = rest.split('}', 1)
            path = pre + mid.split(' => ')[1] + post
        else:
            path = path.split(' => ')[1]
        path = path.replace('//', '/')
    if '/node_modules/' in path: continue
    adds_all += a
    commits[path] = commits.get(path, 0) + 1
    if commits[path] > 1: churned += a
files_churned = sum(1 for c in commits.values() if c > 1)
json.dump({'population': f'git log --numstat {BASE[:8]}..HEAD -- crates/**/*.rs crates/**/*.ts crates/**/*.tsx',
           'commits_in_window': n_commits, 'pct': round(100 * churned / adds_all, 2) if adds_all else None,
           'files_churned': files_churned, 'files_touched': len(commits), 'total_adds': adds_all, 'churned_adds': churned,
           'top_touch_counts': sorted(([p, c] for p, c in commits.items()), key=lambda x: (-x[1], x[0]))[:10]},
          open(os.path.join(RD, 'c-churn.json'), 'w', encoding='utf-8'), indent=1)
maxcog = json.load(open(os.path.join(RD, '_file_maxcog.json')))
hs = sorted(([p, c * maxcog.get(p, 0)] for p, c in commits.items()), key=lambda x: (-x[1], x[0]))
json.dump({'formula': 'commits(churn window) x max cognitive per file over the FULL rca population (fallback 0 for non-Rust)',
           'top': [[p, float(s)] for p, s in hs[:10]]},
          open(os.path.join(RD, 'c-hotspots.json'), 'w', encoding='utf-8'), indent=1)
print(n_commits, len(commits), files_churned, adds_all, churned, hs[:10])
