"""Size discipline: delete raw tool output and transients. Deletes are confined to
THIS run dir — the only sanctioned delete scope. Kept: c-*.json evidence twins, the
recomputed-baseline twins cited by proposals.md, record.json, proposals.md, summarizers.
"""
import os, shutil, sys

RD = os.path.abspath('.andromeda/runs/2026-08-22T13-31-54-code-audit')
assert os.path.isdir(RD), RD

DIRS = ['_baseline_tree', '_rca_head', '_rca_base', 'jscpd', 'jscpd_baseline',
        'jscpd_baseline_rs', 'jscpd_rs', 'mutants-conductor-verify', 'mutants-conductor-run']
FILES = ['_lcov.info', '_tokei.json', '_tokei_baseline.json', '_tokei_baseline_root.json',
         '_zeroref.json', '_zeroref_raw.txt', '_numstat.txt', '_perfile_cx.json',
         '_fanin.txt', '_ledger_dryrun.ndjson']


def under_run_dir(p):
    return os.path.abspath(p).startswith(RD + os.sep)


def size(p):
    if os.path.isfile(p):
        return os.path.getsize(p)
    t = 0
    for dp, _d, fs in os.walk(p):
        for f in fs:
            try:
                t += os.path.getsize(os.path.join(dp, f))
            except OSError:
                pass
    return t


freed = 0
for name in DIRS + FILES:
    p = os.path.join(RD, name)
    if not os.path.exists(p):
        continue
    if not under_run_dir(p):                      # refuse anything outside {run_dir}
        print('REFUSED (outside run dir):', p)
        sys.exit(1)
    freed += size(p)
    if os.path.isdir(p):
        shutil.rmtree(p, ignore_errors=True)
    else:
        os.remove(p)
    print('removed', name)

print(f'\nfreed ~{freed / 1_048_576:.1f} MB')
rest = sorted(os.listdir(RD))
print('kept (%d):' % len(rest))
for r in rest:
    print('  ', r, f'({size(os.path.join(RD, r)) / 1024:.0f} KB)')
