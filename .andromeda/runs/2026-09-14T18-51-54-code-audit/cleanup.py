"""Size discipline: delete raw tool output AFTER summarization. Deletes are confined to THIS
run_dir (the only sanctioned delete scope); every path is re-checked to live under it before removal."""
import os, shutil, json

RD = os.path.abspath('D:/dev/projects/conductor/.andromeda/runs/2026-09-14T18-51-54-code-audit')

KEEP_PREFIX = ('c-',)
KEEP_EXACT = {'record.json', 'judge.json', 'proposals.md', 'tree-query-code-audit.json',
              '_mutation_driver.log',  # the tier's per-unit timing/completion narration, ~2 KB
              'summarize.py', 'mutation.py', 'assemble.py', 'fix_survivors.py',
              'render_audit.py', 'cleanup.py'}


def under_rd(p):
    return os.path.abspath(p).startswith(RD + os.sep)


before = sum(os.path.getsize(os.path.join(r, f))
             for r, _, fs in os.walk(RD) for f in fs)
removed = []
for name in sorted(os.listdir(RD)):
    if name in KEEP_EXACT or name.startswith(KEEP_PREFIX):
        continue
    p = os.path.join(RD, name)
    if not under_rd(p):
        raise SystemExit(f'REFUSED — outside run dir: {p}')
    if os.path.isdir(p):
        n = sum(len(fs) for _, _, fs in os.walk(p))
        shutil.rmtree(p)
        removed.append((name + '/', n))
    else:
        os.remove(p)
        removed.append((name, 1))
after = sum(os.path.getsize(os.path.join(r, f))
            for r, _, fs in os.walk(RD) for f in fs)
for n, c in removed:
    print(f'  removed {n}  ({c} file{"s" if c != 1 else ""})')
print(f'\nrun dir {before/1048576:.1f} MB -> {after/1024:.0f} KB')
print('kept:', sorted(os.listdir(RD)))
