"""Add the FULL clone-pair list to c-duplication.json (so the next diff can name entrants — the capped top-10 cannot),
and print the pairs that involve a file touched in the audited span (the only ones that can be new this epoch)."""
import json, os, sys
RUN = sys.argv[1]
rep = json.load(open(os.path.join(RUN, 'jscpd_rs2', 'jscpd-report.json'), encoding='utf-8'))
def short(p):
    p = p.replace('\\', '/'); i = p.find('crates/')
    p = p[i:] if i >= 0 else p
    return p[len('crates/'):] if p.startswith('crates/') else p
pairs = sorted(([short(d['firstFile']['name']), short(d['secondFile']['name']), d.get('lines', 0),
                 d['firstFile'].get('startLoc', {}).get('line'), d['secondFile'].get('startLoc', {}).get('line')] for d in rep.get('duplicates', [])),
               key=lambda x: (-x[2], x[0], x[1]))
c = json.load(open(os.path.join(RUN, 'c-duplication.json'), encoding='utf-8'))
c['all_pairs'] = [p[:3] for p in pairs]
c['all_pairs_with_start_lines'] = pairs
touched = {short('crates/' + k[len('crates/'):]) if k.startswith('crates/') else k for k in json.load(open(os.path.join(RUN, 'c-churn.json'), encoding='utf-8'))['per_file_commits']}
inv = [p for p in pairs if p[0] in touched or p[1] in touched]
c['pairs_involving_files_touched_in_span'] = inv
json.dump(c, open(os.path.join(RUN, 'c-duplication.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
print('pairs', len(pairs), '| involving touched files:', len(inv))
for p in inv: print('  ', p)
