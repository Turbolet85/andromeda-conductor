"""Relay-directed in-place redaction (conductor-wrap-srcause-2026-09-30 §1): host paths OUTSIDE the repo in three
phase-run logs become <host-path>. A path is a drive + separators + segments; a NON-final segment may hold single
spaces (`Program Files`), the final one may not, so trailing prose is never swallowed. The repo-root path (hygiene's
excluded class) is kept. Each file is written as bytes and re-scanned."""
import pathlib, re, sys
BS = chr(92)
root = pathlib.Path('D:/dev/projects/conductor')
d = root / '.andromeda/runs/2026-09-30T12-47-30-phase'
sep = '[' + re.escape(BS) + '/]+'
word = r'[A-Za-z0-9_(][A-Za-z0-9_.+~()-]*'
seg_sp = word + '(?: ' + word + ')*'
drive = re.compile(r'(?<![A-Za-z0-9])[A-Za-z]:' + sep + '(?:' + seg_sp + sep + ')*' + word)
repo = re.compile(r'[Dd]:' + sep + 'dev' + sep + 'projects' + sep + 'conductor(?![A-Za-z0-9_.-])')
dry = '--dry-run' in sys.argv
for name in ('baseline.log', 'baseline-2.log', 'gate-dryrun.log'):
    p = d / name
    text = p.read_bytes().decode('utf-8')
    hits = []
    def sub(m):
        s = m.group(0)
        if repo.match(s):
            return s
        hits.append(s)
        return '<host-path>'
    new = drive.sub(sub, text)
    print(name, 'replaced', len(hits), [h.replace(BS, '/').split('/')[0] + '/…' + str(len(h)) for h in hits])
    if not dry:
        p.write_bytes(new.encode('utf-8'))
        back = p.read_bytes().decode('utf-8')
        left = [m.group(0) for m in drive.finditer(back) if not repo.match(m.group(0))]
        print('   left', len(left), 'lines', back.count('\n'), 'crlf', back.count('\r'))
